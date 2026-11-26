"""The catalogue-import stage.

The stage reads the RDF export, a read-only copy of the Zotero database and the files in Zotero
storage. It writes three research artifacts through the store:

- ``export-snapshot``: the fingerprint and counts of the RDF export.
- ``source-manifest``: one entry per catalogue record, with a readable summary.
- ``alias-table``: each export-scoped ID mapped to its source identity.

The stage links each RDF record to a database item by evidence, in this order: the ``#item_N``
number, the parent of its attachment records, then a unique match on item type, title and ISBN.
It never selects a primary PDF. It lists every ambiguity with its evidence instead.
"""

from __future__ import annotations

import hashlib
import json
import re
import unicodedata
from collections import Counter
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Literal

import pymupdf  # pyright: ignore[reportMissingTypeStubs]

from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact, canonical_json
from urg.version import code_version
from urg.zotero_db import (
    LINK_MODES,
    NON_RECORD_TYPES,
    DbAttachment,
    DbItem,
    DbSnapshot,
    database_copy,
    read_snapshot,
    resolve_attachment_file,
    sha256_file,
)
from urg.zotero_rdf import (
    RdfExport,
    RdfRecord,
    collection_paths,
    item_number,
    read_export,
    records_in_top_level_collection,
)

STAGE = "catalogue-import"
SCHEMA_VERSION = 1
EXPORT_SNAPSHOT = "export-snapshot"
SOURCE_MANIFEST = "source-manifest"
ALIAS_TABLE = "alias-table"
YEAR = re.compile(r"\b(\d{4})\b")
ISBN_TOKEN = re.compile(r"^[0-9Xx-]{10,17}$")

Json = dict[str, Any]
LinkMethod = Literal["item-number", "attachment-parent", "field-match"]


@dataclass(frozen=True)
class ImportConfig:
    """The configuration that every dependency record of the stage names."""

    library_id: int = 1
    computing_collection: str = "Computing"
    expected_records: int = 2723
    expected_books: int = 1280
    expected_computing_books: int = 841

    def as_json(self) -> Json:
        return {
            "library_id": self.library_id,
            "computing_collection": self.computing_collection,
            "expected_audit_figures": {
                "records": self.expected_records,
                "books": self.expected_books,
                "computing_books": self.expected_computing_books,
            },
        }


@dataclass(frozen=True)
class ImportResult:
    export_snapshot: StoredArtifact
    source_manifest: StoredArtifact
    alias_table: StoredArtifact
    reused: bool


def normalise_text(value: str | None) -> str:
    """Return text in NFC, case-folded, with runs of whitespace collapsed."""
    if not value:
        return ""
    return " ".join(unicodedata.normalize("NFC", value).casefold().split())


def normalise_isbns(value: str | None) -> list[str]:
    if not value:
        return []
    return sorted(
        {
            token.replace("-", "").upper()
            for token in value.replace(",", " ").split()
            if ISBN_TOKEN.match(token)
        }
    )


def year_of(value: str | None) -> str | None:
    if not value:
        return None
    match = YEAR.search(value)
    return match.group(1) if match else None


# --- Dependency records -------------------------------------------------------------------


def storage_fingerprint(snapshot: DbSnapshot, zotero_dir: Path, library_id: int) -> str:
    """Return the SHA-256 of the path, size and modification time of each attachment file.

    The fingerprint covers every attachment file of the library, so a new, removed or changed
    file invalidates the manifest. It reads no file content.
    """
    members: list[list[Any]] = []
    for attachment in snapshot.attachments.values():
        if attachment.library_id != library_id or attachment.deleted:
            continue
        location = resolve_attachment_file(zotero_dir, attachment)
        if location is None:
            continue
        try:
            stat = location.stat()
            members.append([attachment.key, str(location), stat.st_size, stat.st_mtime_ns])
        except OSError:
            members.append([attachment.key, str(location), "missing"])
    members.sort(key=lambda member: (member[0], member[1]))
    return hashlib.sha256(canonical_json(members)).hexdigest()


def dependency_records(
    rdf_path: Path,
    rdf_sha256: str,
    zotero_dir: Path,
    database_sha256: str,
    storage_sha256: str,
    config: ImportConfig,
) -> tuple[DependencyRecord, DependencyRecord, DependencyRecord]:
    """Return the dependency records of the three research artifacts, before any is built."""
    rdf_input = InputIdentity(
        role="rdf-export", kind="file", locator=str(rdf_path), identity=rdf_sha256
    )
    database_input = InputIdentity(
        role="zotero-database",
        kind="file",
        locator=str(zotero_dir / "zotero.sqlite"),
        identity=database_sha256,
    )
    storage_input = InputIdentity(
        role="zotero-storage",
        kind="file-set",
        locator=str(zotero_dir / "storage"),
        identity=storage_sha256,
    )
    common: Json = {
        "stage": STAGE,
        "schema_version": SCHEMA_VERSION,
        "code_version": code_version(__name__),
    }
    snapshot = DependencyRecord(
        **common, artifact=EXPORT_SNAPSHOT, configuration={}, inputs=[rdf_input]
    )
    manifest = DependencyRecord(
        **common,
        artifact=SOURCE_MANIFEST,
        configuration=config.as_json(),
        inputs=[
            InputIdentity(
                role="export-snapshot",
                kind="artifact",
                locator=f"{EXPORT_SNAPSHOT}/{snapshot.key()}",
                identity=snapshot.key(),
            ),
            rdf_input,
            database_input,
            storage_input,
        ],
    )
    aliases = DependencyRecord(
        **common,
        artifact=ALIAS_TABLE,
        configuration={"library_id": config.library_id},
        inputs=[
            InputIdentity(
                role="export-snapshot",
                kind="artifact",
                locator=f"{EXPORT_SNAPSHOT}/{snapshot.key()}",
                identity=snapshot.key(),
            ),
            InputIdentity(
                role="source-manifest",
                kind="artifact",
                locator=f"{SOURCE_MANIFEST}/{manifest.key()}",
                identity=manifest.key(),
            ),
        ],
    )
    return snapshot, manifest, aliases


# --- Linking ------------------------------------------------------------------------------


@dataclass
class Link:
    db_item_id: int | None = None
    method: LinkMethod | None = None
    evidence: Json = field(default_factory=dict[str, Any])
    disagreements: list[Json] = field(default_factory=list[Json])


def _is_record_item(item: DbItem | None, library_id: int) -> bool:
    return (
        item is not None
        and item.library_id == library_id
        and item.item_type not in NON_RECORD_TYPES
        and not item.deleted
    )


def _rdf_attachments_in_db(
    record: RdfRecord, snapshot: DbSnapshot, library_id: int
) -> list[DbAttachment]:
    found: list[DbAttachment] = []
    for ref in record.attachment_refs:
        number = item_number(ref)
        attachment = snapshot.attachments.get(number) if number is not None else None
        if attachment is not None and attachment.library_id == library_id:
            found.append(attachment)
    return found


def link_records(export: RdfExport, snapshot: DbSnapshot, library_id: int) -> dict[str, Link]:
    """Link each RDF record to at most one database item, with the evidence for the link."""
    links: dict[str, Link] = {}
    claimed: dict[int, list[str]] = {}
    for record in export.records:
        link = Link()
        links[record.about] = link
        number = item_number(record.about)
        item = snapshot.items.get(number) if number is not None else None
        parents = sorted(
            {
                attachment.parent_item_id
                for attachment in _rdf_attachments_in_db(record, snapshot, library_id)
                if attachment.parent_item_id is not None
            }
        )
        if (
            _is_record_item(item, library_id)
            and item is not None
            and item.item_type == record.item_type
        ):
            link.db_item_id = item.item_id
            link.method = "item-number"
            link.evidence = {"item_number": number}
            other_parents = [parent for parent in parents if parent != item.item_id]
            if other_parents:
                link.disagreements.append(
                    {
                        "field": "attachment-parent",
                        "rdf": record.about,
                        "database": other_parents,
                        "note": "An RDF attachment has another parent in the database.",
                    }
                )
        elif len(parents) == 1 and _is_record_item(snapshot.items.get(parents[0]), library_id):
            link.db_item_id = parents[0]
            link.method = "attachment-parent"
            link.evidence = {
                "attachment_refs": list(record.attachment_refs),
                "parent_item_id": parents[0],
            }
        elif len(parents) > 1:
            link.disagreements.append(
                {
                    "field": "attachment-parent",
                    "rdf": list(record.attachment_refs),
                    "database": parents,
                    "note": "The RDF attachments have several parents in the database.",
                }
            )
        if link.db_item_id is not None:
            claimed.setdefault(link.db_item_id, []).append(record.about)

    # A record without a number or attachments links only by a unique field match.
    candidates: dict[tuple[str, str], list[DbItem]] = {}
    for item in snapshot.regular_items(library_id):
        if not item.deleted and item.item_id not in claimed:
            candidates.setdefault((item.item_type, normalise_text(item.title)), []).append(item)
    proposals: dict[str, tuple[DbItem, list[str]]] = {}
    for record in export.records:
        link = links[record.about]
        if link.db_item_id is not None or link.disagreements:
            continue
        matches = candidates.get((record.item_type, normalise_text(record.title)), [])
        matched_fields = ["item_type", "title"]
        if record.isbns and len(matches) > 1:
            by_isbn = [
                item
                for item in matches
                if set(record.isbns) & set(normalise_isbns(item.fields.get("ISBN")))
            ]
            if by_isbn:
                matches = by_isbn
                matched_fields.append("ISBN")
        if len(matches) == 1:
            proposals[record.about] = (matches[0], matched_fields)
        elif matches:
            link.evidence = {
                "matched_fields": matched_fields,
                "candidate_item_keys": sorted(item.key for item in matches),
            }

    # A field match is one-to-one: an item that several records match links to none of them.
    rivals: dict[int, list[str]] = {}
    for about, (item, _fields) in proposals.items():
        rivals.setdefault(item.item_id, []).append(about)
    for about, (item, matched_fields) in proposals.items():
        link = links[about]
        if len(rivals[item.item_id]) > 1:
            link.evidence = {
                "matched_fields": matched_fields,
                "candidate_item_keys": [item.key],
                "rival_records": sorted(rivals[item.item_id]),
                "note": "Several RDF records match this database item.",
            }
            continue
        link.db_item_id = item.item_id
        link.method = "field-match"
        link.evidence = {"matched_fields": matched_fields, "item_key": item.key}
        claimed.setdefault(item.item_id, []).append(about)

    for item_id, abouts in claimed.items():
        if len(abouts) > 1:
            for about in abouts:
                links[about].disagreements.append(
                    {
                        "field": "source-identity",
                        "rdf": sorted(abouts),
                        "database": item_id,
                        "note": "Several RDF records link to one database item.",
                    }
                )
    return links


def item_number_check(
    about: str, rdf_type: str, rdf_title: str, snapshot: DbSnapshot, library_id: int
) -> Json:
    """Check that an RDF ``#item_N`` names the database item N, and return the result."""
    number = item_number(about)
    if number is None:
        return {"status": "not-applicable", "reason": "The export ID is not of the form #item_N."}
    item = snapshot.items.get(number)
    if item is None:
        return {"status": "absent", "item_number": number}
    reasons: list[str] = []
    if item.library_id != library_id:
        reasons.append(f"item {number} is in library {item.library_id}")
    if item.deleted:
        reasons.append(f"item {number} is in the trash")
    if item.item_type != rdf_type:
        reasons.append(f"item type {item.item_type} differs from RDF type {rdf_type}")
    if normalise_text(item.title) != normalise_text(rdf_title):
        reasons.append("the title differs")
    return {
        "status": "disagrees" if reasons else "agrees",
        "item_number": number,
        "item_key": item.key,
        "reasons": reasons,
    }


# --- Document files -----------------------------------------------------------------------


class FileInspector:
    """Hash and open each document file once, however many records it backs."""

    def __init__(self) -> None:
        self._cache: dict[Path, Json] = {}
        pymupdf.TOOLS.mupdf_display_errors(False)  # pyright: ignore

    def inspect(self, location: Path, is_pdf: bool) -> Json:
        cached = self._cache.get(location)
        if cached is not None:
            return cached
        result: Json = {"exists": location.is_file()}
        if result["exists"]:
            result["size"] = location.stat().st_size
            result["sha256"] = sha256_file(location)
            if is_pdf:
                result.update(_pdf_readability(location))
        self._cache[location] = result
        return result


def _pdf_readability(location: Path) -> Json:
    try:
        with pymupdf.open(location) as document:  # pyright: ignore
            page_count = int(document.page_count)  # pyright: ignore
            needs_password = bool(document.needs_pass)  # pyright: ignore
    except Exception as e:  # PyMuPDF raises several exception types for damaged files.
        return {"readable": False, "page_count": None, "error": f"{type(e).__name__}: {e}"}
    readable = page_count > 0 and not needs_password
    return {
        "readable": readable,
        "page_count": page_count,
        "needs_password": needs_password,
        "error": None if readable else "The PDF reports no pages or needs a password.",
    }


def _is_pdf(content_type: str | None, path: str | None) -> bool:
    return content_type == "application/pdf" or (path or "").lower().endswith(".pdf")


def _mapping(value: Json | None) -> Json:
    return value or {}


def _file_of(attachment: Json) -> Json:
    """Return the file facts of an attachment entry, or an empty mapping."""
    return _mapping(attachment["file"])


# --- Manifest entries ---------------------------------------------------------------------


def _db_details(item: DbItem) -> Json:
    fields = item.fields
    return {
        "item_type": item.item_type,
        "title": item.title,
        "creators": [
            {"role": c.role, "last_name": c.last_name, "first_name": c.first_name}
            for c in item.creators
        ],
        "date": fields.get("date"),
        "year": year_of(fields.get("date")),
        "edition": fields.get("edition"),
        "publisher": fields.get("publisher"),
        "place": fields.get("place"),
        "isbns": normalise_isbns(fields.get("ISBN")),
        "doi": fields.get("DOI"),
        "url": fields.get("url"),
        "num_pages": fields.get("numPages"),
    }


def _rdf_details(record: RdfRecord) -> Json:
    return {
        "item_type": record.item_type,
        "title": record.title,
        "creators": [
            {"role": c.role, "last_name": c.surname, "first_name": c.given_name}
            for c in record.creators
        ],
        "date": record.date,
        "year": year_of(record.date),
        "edition": record.edition,
        "publisher": record.publisher,
        "place": record.place,
        "isbns": sorted(set(record.isbns)),
        "url": record.url,
        "num_pages": record.num_pages,
    }


def _field_disagreements(rdf: Json, db: Json) -> list[Json]:
    found: list[Json] = []
    for name in ("item_type", "title", "edition", "year", "publisher"):
        rdf_value, db_value = rdf.get(name), db.get(name)
        if normalise_text(rdf_value) != normalise_text(db_value):
            found.append({"field": name, "rdf": rdf_value, "database": db_value})
    if set(rdf["isbns"]) != set(db["isbns"]):
        found.append({"field": "isbns", "rdf": rdf["isbns"], "database": db["isbns"]})
    return found


def _attachment_entry(
    attachment: DbAttachment | None,
    rdf_ref: str | None,
    export: RdfExport,
    snapshot: DbSnapshot,
    zotero_dir: Path,
    library_id: int,
    inspector: FileInspector,
    in_db_children: bool,
) -> Json:
    rdf_attachment = export.attachments.get(rdf_ref) if rdf_ref else None
    entry: Json = {
        "export_id": rdf_ref,
        "in_rdf": rdf_attachment is not None,
        "in_database_children": in_db_children,
        "rdf_title": rdf_attachment.title if rdf_attachment else None,
        "rdf_mime_type": rdf_attachment.mime_type if rdf_attachment else None,
    }
    if rdf_ref is not None:
        entry["item_number_check"] = item_number_check(
            rdf_ref,
            "attachment",
            rdf_attachment.title if rdf_attachment else "",
            snapshot,
            library_id,
        )
    if attachment is None:
        entry.update(
            {
                "db_item_id": None,
                "item_key": None,
                "is_pdf": _is_pdf(rdf_attachment.mime_type if rdf_attachment else None, None),
                "file": None,
            }
        )
        return entry
    is_pdf = _is_pdf(attachment.content_type, attachment.path)
    location = resolve_attachment_file(zotero_dir, attachment)
    entry.update(
        {
            "db_item_id": attachment.item_id,
            "item_key": attachment.key,
            "title": attachment.title,
            "link_mode": (
                LINK_MODES.get(attachment.link_mode, f"unknown-{attachment.link_mode}")
                if attachment.link_mode is not None
                else None
            ),
            "content_type": attachment.content_type,
            "stored_path": attachment.path,
            "is_pdf": is_pdf,
            "file": (
                {"location": str(location), **inspector.inspect(location, is_pdf)}
                if location is not None
                else None
            ),
        }
    )
    return entry


def _entry_attachments(
    record: RdfRecord,
    link: Link,
    export: RdfExport,
    snapshot: DbSnapshot,
    zotero_dir: Path,
    library_id: int,
    inspector: FileInspector,
) -> list[Json]:
    """Return the union of the RDF attachment records and the database child attachments."""
    children = (
        {
            a.item_id: a
            for a in snapshot.attachments.values()
            if a.parent_item_id == link.db_item_id and not a.deleted
        }
        if link.db_item_id is not None
        else {}
    )
    entries: list[Json] = []
    seen: set[int] = set()
    for ref in record.attachment_refs:
        number = item_number(ref)
        attachment = snapshot.attachments.get(number) if number is not None else None
        if attachment is not None and attachment.library_id != library_id:
            attachment = None
        if attachment is not None:
            seen.add(attachment.item_id)
        entries.append(
            _attachment_entry(
                attachment,
                ref,
                export,
                snapshot,
                zotero_dir,
                library_id,
                inspector,
                attachment is not None and attachment.item_id in children,
            )
        )
    for item_id in sorted(children):
        if item_id not in seen:
            entries.append(
                _attachment_entry(
                    children[item_id],
                    None,
                    export,
                    snapshot,
                    zotero_dir,
                    library_id,
                    inspector,
                    True,
                )
            )
    return entries


def _ambiguities(entry: Json, link: Link) -> list[Json]:
    found: list[Json] = []
    attachments: list[Json] = entry["attachments"]
    pdfs = [a for a in attachments if a["is_pdf"]]
    if len(pdfs) > 1:
        found.append(
            {
                "kind": "several-pdfs",
                "evidence": [
                    {
                        "item_key": a.get("item_key"),
                        "title": a.get("title") or a.get("rdf_title"),
                        "sha256": _file_of(a).get("sha256"),
                        "page_count": _file_of(a).get("page_count"),
                    }
                    for a in pdfs
                ],
            }
        )
    if not pdfs:
        found.append(
            {
                "kind": "no-pdf",
                "evidence": [
                    {"item_key": a.get("item_key"), "content_type": a.get("content_type")}
                    for a in attachments
                ],
            }
        )
    for a in attachments:
        file_info: Json | None = a["file"]
        if a["db_item_id"] is None:
            found.append(
                {
                    "kind": "rdf-db-disagreement",
                    "evidence": {
                        "field": "attachment",
                        "rdf": a["export_id"],
                        "database": None,
                        "note": "The RDF attachment record has no database attachment.",
                    },
                }
            )
        elif not a["in_rdf"]:
            found.append(
                {
                    "kind": "rdf-db-disagreement",
                    "evidence": {
                        "field": "attachment",
                        "rdf": None,
                        "database": a["item_key"],
                        "note": "The database attachment is absent from the RDF export.",
                    },
                }
            )
        elif not a["in_database_children"] and link.db_item_id is not None:
            found.append(
                {
                    "kind": "rdf-db-disagreement",
                    "evidence": {
                        "field": "attachment-parent",
                        "rdf": a["export_id"],
                        "database": a["item_key"],
                        "note": "The database attachment belongs to another parent item.",
                    },
                }
            )
        check = a.get("item_number_check")
        if check is not None and check["status"] in ("disagrees", "absent"):
            found.append({"kind": "item-number-mismatch", "evidence": check})
        if file_info is not None and not file_info["exists"]:
            found.append(
                {
                    "kind": "missing-file",
                    "evidence": {"item_key": a["item_key"], "location": file_info["location"]},
                }
            )
        if file_info is not None and file_info.get("readable") is False:
            found.append(
                {
                    "kind": "unreadable-pdf",
                    "evidence": {
                        "item_key": a["item_key"],
                        "location": file_info["location"],
                        "error": file_info.get("error"),
                    },
                }
            )
    check = entry["item_number_check"]
    if check["status"] in ("disagrees", "absent"):
        found.append({"kind": "item-number-mismatch", "evidence": check})
    for disagreement in link.disagreements:
        found.append({"kind": "rdf-db-disagreement", "evidence": disagreement})
    if entry["database_details"] is not None:
        for disagreement in _field_disagreements(entry["rdf_details"], entry["database_details"]):
            found.append({"kind": "rdf-db-disagreement", "evidence": disagreement})
    if link.method == "field-match":
        found.append({"kind": "field-match", "evidence": link.evidence})
    if link.db_item_id is None:
        found.append({"kind": "no-source-identity", "evidence": link.evidence})
    return found


def build_entries(
    export: RdfExport,
    snapshot: DbSnapshot,
    zotero_dir: Path,
    rdf_sha256: str,
    config: ImportConfig,
) -> list[Json]:
    library_id = config.library_id
    library = snapshot.libraries.get(library_id)
    links = link_records(export, snapshot, library_id)
    paths = collection_paths(export)
    computing = records_in_top_level_collection(export, config.computing_collection)
    inspector = FileInspector()
    entries: list[Json] = []
    for record in export.records:
        link = links[record.about]
        item = snapshot.items[link.db_item_id] if link.db_item_id is not None else None
        entries.append(
            {
                "export_id": record.about,
                "export_snapshot_sha256": rdf_sha256,
                "rdf_identifiers": list(record.identifiers),
                "source_identity": (
                    {
                        "library_id": library_id,
                        "library_type": library.library_type if library else None,
                        "item_key": item.key,
                    }
                    if item is not None
                    else None
                ),
                "db_item_id": link.db_item_id,
                "link": {"method": link.method, "evidence": link.evidence},
                "item_number_check": item_number_check(
                    record.about, record.item_type, record.title, snapshot, library_id
                ),
                "details": _db_details(item) if item is not None else _rdf_details(record),
                "details_source": "database" if item is not None else "rdf",
                "database_details": _db_details(item) if item is not None else None,
                "rdf_details": _rdf_details(record),
                "collections": paths.get(record.about, []),
                "computing": record.about in computing,
                "workbook_mapping": {
                    "status": "workbook-unavailable",
                    "r_id": None,
                    "note": "The audit workbook is not available. No R-ID is assigned.",
                },
                "attachments": _entry_attachments(
                    record, link, export, snapshot, zotero_dir, library_id, inspector
                ),
            }
        )

    # A document file that backs several records is an ambiguity of each of those records.
    owners: dict[str, set[str]] = {}
    for entry in entries:
        for attachment in entry["attachments"]:
            sha = _file_of(attachment).get("sha256")
            if sha:
                owners.setdefault(sha, set()).add(entry["export_id"])
    for entry in entries:
        link = links[entry["export_id"]]
        ambiguities = _ambiguities(entry, link)
        for attachment in entry["attachments"]:
            sha = _file_of(attachment).get("sha256")
            others = sorted(owners.get(sha, set()) - {entry["export_id"]}) if sha else []
            if others:
                ambiguities.append(
                    {
                        "kind": "shared-document-file",
                        "evidence": {
                            "item_key": attachment["item_key"],
                            "sha256": sha,
                            "other_export_ids": others,
                        },
                    }
                )
        entry["ambiguities"] = ambiguities
        entry["availability"] = _availability(entry["attachments"])
        entry["reconciliation_status"] = (
            "unlinked"
            if entry["db_item_id"] is None
            else "needs-review"
            if ambiguities
            else "reconciled"
        )
    return entries


def _availability(attachments: list[Json]) -> Json:
    pdfs = [a for a in attachments if a["is_pdf"]]
    present = [a for a in pdfs if _file_of(a).get("exists")]
    readable = [a for a in present if _file_of(a).get("readable")]
    return {
        "pdf_records": len(pdfs),
        "pdf_files_present": len(present),
        "pdf_files_readable": len(readable),
    }


# --- Audit --------------------------------------------------------------------------------


def audit(
    export: RdfExport, snapshot: DbSnapshot, entries: list[Json], config: ImportConfig
) -> Json:
    """Check the audit figures against the inputs and explain each discrepancy."""
    books = [entry for entry in entries if entry["rdf_details"]["item_type"] == "book"]
    computing_books = [entry for entry in books if entry["computing"]]
    db_counts: dict[str, Any] = {}
    for library_id, library in sorted(snapshot.libraries.items()):
        items = [i for i in snapshot.regular_items(library_id) if not i.deleted]
        db_counts[str(library_id)] = {
            "library_type": library.library_type,
            "records": len(items),
            "books": sum(1 for i in items if i.item_type == "book"),
            "records_in_trash": sum(1 for i in snapshot.regular_items(library_id) if i.deleted),
        }
    facts: Json = {
        "rdf_records_by_type": dict(sorted(Counter(r.item_type for r in export.records).items())),
        "database_by_library": db_counts,
    }
    figures = [
        ("records", config.expected_records, len(entries)),
        ("books", config.expected_books, len(books)),
        ("computing_books", config.expected_computing_books, len(computing_books)),
    ]
    library = db_counts.get(str(config.library_id), {})
    checks: list[Json] = []
    for name, expected, observed in figures:
        check: Json = {"figure": name, "expected": expected, "observed": observed}
        check["status"] = "agrees" if expected == observed else "discrepancy"
        if expected != observed:
            check["explanation"] = _explain(name, expected, observed, library)
        checks.append(check)
    pdf_counts = Counter(min(entry["availability"]["pdf_records"], 2) for entry in books)
    return {
        "checks": checks,
        "facts": facts,
        "book_pdf_records": {
            "none": pdf_counts.get(0, 0),
            "one": pdf_counts.get(1, 0),
            "several": pdf_counts.get(2, 0),
        },
    }


def _explain(name: str, expected: int, observed: int, library: Json) -> str:
    database_figure = {"records": library.get("records"), "books": library.get("books")}.get(name)
    parts = [f"The inputs give {observed}, not {expected}, a difference of {observed - expected}."]
    if database_figure is not None:
        parts.append(f"The database library gives {database_figure} outside the trash.")
        if database_figure == observed:
            parts.append("The RDF export and the database agree, so the audit figure is stale.")
        elif database_figure == expected:
            parts.append("The database agrees with the audit, so the RDF export is incomplete.")
    return " ".join(parts)


# --- Summary ------------------------------------------------------------------------------


def summary_markdown(manifest: Json) -> str:
    entries: list[Json] = manifest["entries"]
    lines = [
        "# Source manifest",
        "",
        f"- RDF export SHA-256: `{manifest['export_snapshot']['sha256']}`",
        f"- Database copy SHA-256: `{manifest['database']['sha256']}`",
        f"- Records: {len(entries)}",
        "",
        "## Audit figures",
        "",
        "| Figure | Expected | Observed | Status |",
        "| --- | ---: | ---: | --- |",
    ]
    for check in manifest["audit"]["checks"]:
        lines.append(
            f"| {check['figure']} | {check['expected']} | {check['observed']} | {check['status']} |"
        )
    for check in manifest["audit"]["checks"]:
        if "explanation" in check:
            lines.append("")
            lines.append(f"{check['figure']}: {check['explanation']}")
    pdfs = manifest["audit"]["book_pdf_records"]
    lines += [
        "",
        f"Books with one PDF: {pdfs['one']}. With no PDF: {pdfs['none']}. "
        f"With several PDFs: {pdfs['several']}.",
        "",
        "## Reconciliation status",
        "",
        "| Status | Records | Books |",
        "| --- | ---: | ---: |",
    ]
    statuses = Counter(e["reconciliation_status"] for e in entries)
    book_statuses = Counter(
        e["reconciliation_status"] for e in entries if e["rdf_details"]["item_type"] == "book"
    )
    for status in ("reconciled", "needs-review", "unlinked"):
        lines.append(f"| {status} | {statuses.get(status, 0)} | {book_statuses.get(status, 0)} |")
    lines += ["", "## Ambiguities", "", "| Kind | Records | Books |", "| --- | ---: | ---: |"]
    kinds: Counter[str] = Counter()
    book_kinds: Counter[str] = Counter()
    for entry in entries:
        entry_kinds = {a["kind"] for a in entry["ambiguities"]}
        kinds.update(entry_kinds)
        if entry["rdf_details"]["item_type"] == "book":
            book_kinds.update(entry_kinds)
    for kind in sorted(kinds):
        lines.append(f"| {kind} | {kinds[kind]} | {book_kinds.get(kind, 0)} |")
    lines += [
        "",
        "## Books that need review",
        "",
        "Each line names the item key, the title and the kinds of ambiguity. "
        "The file `manifest.json` holds the evidence and the records of other types.",
        "",
    ]
    for entry in entries:
        if (
            entry["rdf_details"]["item_type"] != "book"
            or entry["reconciliation_status"] == "reconciled"
        ):
            continue
        key = _mapping(entry["source_identity"]).get("item_key") or entry["export_id"]
        entry_kinds = ", ".join(sorted({a["kind"] for a in entry["ambiguities"]}))
        title = entry["details"]["title"].replace("|", "\\|")
        lines.append(f"- `{key}` {title}: {entry_kinds}")
    return "\n".join(lines) + "\n"


# --- The stage ----------------------------------------------------------------------------


def _json_bytes(value: Json) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def export_snapshot_payload(rdf_path: Path, rdf_sha256: str, export: RdfExport) -> Json:
    return {
        "schema_version": SCHEMA_VERSION,
        "file_name": rdf_path.name,
        "size": rdf_path.stat().st_size,
        "sha256": rdf_sha256,
        "counts": {
            "records": len(export.records),
            "attachments": len(export.attachments),
            "collections": len(export.collections),
            "records_by_type": dict(sorted(Counter(r.item_type for r in export.records).items())),
        },
    }


def alias_payload(manifest: Json, snapshot: DbSnapshot, library_id: int) -> Json:
    aliases: list[Json] = []
    library = snapshot.libraries.get(library_id)
    for entry in manifest["entries"]:
        aliases.append(
            {
                "export_id": entry["export_id"],
                "kind": "record",
                "source_identity": entry["source_identity"],
                "link_method": entry["link"]["method"],
            }
        )
        for attachment in entry["attachments"]:
            if attachment["export_id"] is None:
                continue
            aliases.append(
                {
                    "export_id": attachment["export_id"],
                    "kind": "attachment",
                    "source_identity": (
                        {
                            "library_id": library_id,
                            "library_type": library.library_type if library else None,
                            "item_key": attachment["item_key"],
                        }
                        if attachment["item_key"]
                        else None
                    ),
                    "link_method": "item-number" if attachment["item_key"] else None,
                }
            )
    return {
        "schema_version": SCHEMA_VERSION,
        "export_snapshot_sha256": manifest["export_snapshot"]["sha256"],
        "database_sha256": manifest["database"]["sha256"],
        "aliases": aliases,
    }


def run_import(
    rdf_path: Path,
    zotero_dir: Path,
    store: ArtifactStore,
    config: ImportConfig | None = None,
) -> ImportResult:
    """Run the stage, reusing the stored research artifacts when every dependency is unchanged."""
    config = config or ImportConfig()
    rdf_sha256 = sha256_file(rdf_path)
    with database_copy(zotero_dir) as (copy, database_sha256):
        snapshot = read_snapshot(copy, database_sha256)
    storage_sha256 = storage_fingerprint(snapshot, zotero_dir, config.library_id)
    snapshot_dep, manifest_dep, alias_dep = dependency_records(
        rdf_path, rdf_sha256, zotero_dir, database_sha256, storage_sha256, config
    )
    found = [store.reuse(dep) for dep in (snapshot_dep, manifest_dep, alias_dep)]
    if all(artifact is not None for artifact in found):
        snapshot_artifact, manifest_artifact, alias_artifact = found
        assert snapshot_artifact and manifest_artifact and alias_artifact
        return ImportResult(snapshot_artifact, manifest_artifact, alias_artifact, reused=True)

    export = read_export(rdf_path)
    snapshot_json = export_snapshot_payload(rdf_path, rdf_sha256, export)
    entries = build_entries(export, snapshot, zotero_dir, rdf_sha256, config)
    library = snapshot.libraries.get(config.library_id)
    manifest: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "export_snapshot": {"sha256": rdf_sha256, "artifact_key": snapshot_dep.key()},
        "database": {
            "sha256": database_sha256,
            "library_id": config.library_id,
            "library_type": library.library_type if library else None,
        },
        "storage_fingerprint": storage_sha256,
        "audit": audit(export, snapshot, entries, config),
        "entries": entries,
    }
    snapshot_artifact = store.write(snapshot_dep, {"snapshot.json": _json_bytes(snapshot_json)})
    manifest_artifact = store.write(
        manifest_dep,
        {
            "manifest.json": _json_bytes(manifest),
            "summary.md": summary_markdown(manifest).encode(),
        },
    )
    alias_artifact = store.write(
        alias_dep,
        {"aliases.json": _json_bytes(alias_payload(manifest, snapshot, config.library_id))},
    )
    return ImportResult(snapshot_artifact, manifest_artifact, alias_artifact, reused=False)
