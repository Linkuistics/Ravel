"""Read a Zotero RDF export.

The export holds catalogue records, attachment records and collections as top-level children
of ``rdf:RDF``. Every identifier in it is export-scoped: ``rdf:about`` mixes URLs, ISBN URNs
and local ``#item_N`` numbers, and the export holds no attachment file paths.
"""

from __future__ import annotations

import re
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from pathlib import Path

from urg.errors import UrgError

NS = {
    "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    "z": "http://www.zotero.org/namespaces/export#",
    "dc": "http://purl.org/dc/elements/1.1/",
    "dcterms": "http://purl.org/dc/terms/",
    "bib": "http://purl.org/net/biblio#",
    "foaf": "http://xmlns.com/foaf/0.1/",
    "link": "http://purl.org/rss/1.0/modules/link/",
    "prism": "http://prismstandard.org/namespaces/1.2/basic/",
    "vcard": "http://nwalsh.com/rdf/vCard#",
}
ABOUT = f"{{{NS['rdf']}}}about"
RESOURCE = f"{{{NS['rdf']}}}resource"
ITEM_N = re.compile(r"^#item_(\d+)$")
# Zotero exports the ISBN of these item types on their container, not on the record.
CONTAINED_ITEM_TYPES = frozenset(
    {"bookSection", "conferencePaper", "encyclopediaArticle", "dictionaryEntry"}
)
ISBN_TOKEN = re.compile(r"^[0-9Xx-]{10,17}$")


class RdfError(UrgError):
    """The RDF export cannot be read."""


@dataclass(frozen=True)
class RdfCreator:
    role: str
    surname: str
    given_name: str


@dataclass(frozen=True)
class RdfAttachment:
    about: str
    title: str
    mime_type: str | None
    link_mode: str | None
    url: str | None


@dataclass(frozen=True)
class RdfRecord:
    about: str
    item_type: str
    title: str
    creators: tuple[RdfCreator, ...]
    date: str | None
    publisher: str | None
    place: str | None
    edition: str | None
    isbns: tuple[str, ...]
    identifiers: tuple[str, ...]
    url: str | None
    num_pages: str | None
    attachment_refs: tuple[str, ...]


@dataclass(frozen=True)
class RdfCollection:
    about: str
    title: str
    parts: tuple[str, ...]


@dataclass
class RdfExport:
    records: list[RdfRecord] = field(default_factory=list[RdfRecord])
    attachments: dict[str, RdfAttachment] = field(default_factory=dict[str, RdfAttachment])
    collections: dict[str, RdfCollection] = field(default_factory=dict[str, RdfCollection])


def item_number(about: str) -> int | None:
    """Return N for an export-scoped identifier ``#item_N``, else ``None``."""
    match = ITEM_N.match(about)
    return int(match.group(1)) if match else None


def _text(element: ET.Element, path: str) -> str | None:
    found = element.find(path, NS)
    if found is None or found.text is None:
        return None
    text = found.text.strip()
    return text or None


def _creators(element: ET.Element) -> tuple[RdfCreator, ...]:
    creators: list[RdfCreator] = []
    for tag in ("authors", "editors", "contributors", "translators", "seriesEditors"):
        for person in element.findall(f"bib:{tag}/rdf:Seq/rdf:li/foaf:Person", NS):
            creators.append(
                RdfCreator(
                    role=tag,
                    surname=_text(person, "foaf:surname") or "",
                    given_name=_text(person, "foaf:givenName") or "",
                )
            )
    return tuple(creators)


def _isbns(identifiers: list[str]) -> tuple[str, ...]:
    isbns: list[str] = []
    for identifier in identifiers:
        if not identifier.upper().startswith("ISBN"):
            continue
        for token in identifier[4:].split():
            if ISBN_TOKEN.match(token):
                isbns.append(token.replace("-", "").upper())
    return tuple(isbns)


def _record(element: ET.Element, item_type: str) -> RdfRecord:
    identifiers = [
        (node.text or "").strip()
        for node in element.findall("dc:identifier", NS)
        if (node.text or "").strip()
    ]
    url = None
    for uri in element.findall("dc:identifier/dcterms:URI/rdf:value", NS):
        if uri.text and uri.text.strip():
            url = uri.text.strip()
            break
    # A book's container is a series, whose identifiers do not belong to the book.
    container_identifiers = (
        [
            (node.text or "").strip()
            for node in element.findall("dcterms:isPartOf/*/dc:identifier", NS)
            if (node.text or "").strip()
        ]
        if item_type in CONTAINED_ITEM_TYPES
        else []
    )
    publisher = element.find("dc:publisher/foaf:Organization", NS)
    return RdfRecord(
        about=element.get(ABOUT, ""),
        item_type=item_type,
        title=_text(element, "dc:title") or "",
        creators=_creators(element),
        date=_text(element, "dc:date"),
        publisher=_text(publisher, "foaf:name") if publisher is not None else None,
        place=(
            _text(publisher, "vcard:adr/vcard:Address/vcard:locality")
            if publisher is not None
            else None
        ),
        edition=_text(element, "prism:edition"),
        isbns=tuple(dict.fromkeys(_isbns(identifiers + container_identifiers))),
        identifiers=tuple(identifiers),
        url=url,
        num_pages=_text(element, "z:numPages"),
        attachment_refs=tuple(
            ref for node in element.findall("link:link", NS) if (ref := node.get(RESOURCE))
        ),
    )


def read_export(path: Path) -> RdfExport:
    """Parse an export into catalogue records, attachment records and collections."""
    try:
        root = ET.parse(path).getroot()
    except (ET.ParseError, OSError) as e:
        raise RdfError(f"cannot parse RDF export {path}: {e}") from e
    export = RdfExport()
    for element in root:
        about = element.get(ABOUT)
        if about is None:
            continue
        if element.tag == f"{{{NS['z']}}}Collection":
            export.collections[about] = RdfCollection(
                about=about,
                title=_text(element, "dc:title") or "",
                parts=tuple(
                    ref
                    for node in element.findall("dcterms:hasPart", NS)
                    if (ref := node.get(RESOURCE))
                ),
            )
            continue
        item_type = _text(element, "z:itemType")
        if item_type is None:
            continue
        if item_type == "attachment":
            export.attachments[about] = RdfAttachment(
                about=about,
                title=_text(element, "dc:title") or "",
                mime_type=_text(element, "link:type"),
                link_mode=_text(element, "z:linkMode"),
                url=_text(element, "dc:identifier/dcterms:URI/rdf:value"),
            )
        elif item_type not in ("note", "annotation"):
            export.records.append(_record(element, item_type))
    return export


def collection_paths(export: RdfExport) -> dict[str, list[str]]:
    """Map each record identifier to the slash-joined paths of the collections that hold it."""
    children: set[str] = set()
    for collection in export.collections.values():
        children.update(part for part in collection.parts if part in export.collections)
    paths: dict[str, list[str]] = {}

    def walk(about: str, prefix: str, seen: frozenset[str]) -> None:
        collection = export.collections[about]
        path = f"{prefix}/{collection.title}" if prefix else collection.title
        for part in collection.parts:
            if part in export.collections:
                if part not in seen:
                    walk(part, path, seen | {part})
            else:
                paths.setdefault(part, []).append(path)

    for about in sorted(export.collections):
        if about not in children:
            walk(about, "", frozenset({about}))
    return {about: sorted(set(found)) for about, found in paths.items()}


def records_in_top_level_collection(export: RdfExport, title: str) -> set[str]:
    """Return the records in a named top-level collection or any of its subcollections."""
    paths = collection_paths(export)
    return {
        about
        for about, found in paths.items()
        if any(path == title or path.startswith(f"{title}/") for path in found)
    }
