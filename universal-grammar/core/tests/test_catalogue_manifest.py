"""Tests of the catalogue-import stage against the fixture library."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import pytest

from urg.manifest import ImportConfig, ImportResult, run_import
from urg.store import ArtifactStore
from urg.zotero_db import ZoteroDbError, sha256_file
from urg.zotero_rdf import read_export

from .fixture import Fixture, build_fixture, key

Json = dict[str, Any]
MATCHING = ImportConfig(expected_records=7, expected_books=6, expected_computing_books=2)


@pytest.fixture
def library(tmp_path: Path) -> Fixture:
    return build_fixture(tmp_path / "library")


def _run(library: Fixture, tmp_path: Path, config: ImportConfig = MATCHING) -> ImportResult:
    return run_import(library.rdf, library.zotero_dir, ArtifactStore(tmp_path / "data"), config)


def _manifest(result: ImportResult) -> Json:
    return json.loads(result.source_manifest.read_bytes("manifest.json"))


def _entries(result: ImportResult) -> dict[str, Json]:
    return {entry["export_id"]: entry for entry in _manifest(result)["entries"]}


def _kinds(entry: Json) -> set[str]:
    return {ambiguity["kind"] for ambiguity in entry["ambiguities"]}


def test_each_record_has_one_entry_and_excluded_items_are_absent(
    library: Fixture, tmp_path: Path
) -> None:
    entries = _entries(_run(library, tmp_path))
    assert set(entries) == {
        "urn:isbn:9780262510875",
        "#item_20",
        "http://example.org/one",
        "http://example.org/two",
        "http://example.org/fieldmatch",
        "#item_60",
        "#item_90",
    }
    keys = {entry["source_identity"]["item_key"] for entry in entries.values()}
    assert key(70) not in keys
    assert key(80) not in keys


def test_book_with_several_pdfs_lists_each_pdf_and_picks_none(
    library: Fixture, tmp_path: Path
) -> None:
    result = _run(library, tmp_path)
    entry = _entries(result)["urn:isbn:9780262510875"]
    assert entry["link"]["method"] == "attachment-parent"
    assert entry["source_identity"] == {
        "library_id": 1,
        "library_type": "user",
        "item_key": key(10),
    }
    assert entry["collections"] == ["Computing/Lisp"]
    assert entry["computing"] is True
    assert entry["workbook_mapping"]["status"] == "workbook-unavailable"
    assert _kinds(entry) == {"several-pdfs"}
    (several,) = [a for a in entry["ambiguities"] if a["kind"] == "several-pdfs"]
    assert {e["item_key"] for e in several["evidence"]} == {key(11), key(12)}
    assert {e["page_count"] for e in several["evidence"]} == {1, 2}
    assert entry["reconciliation_status"] == "needs-review"
    assert "primary" not in result.source_manifest.read_bytes("manifest.json").decode()


def test_missing_file_is_recorded(library: Fixture, tmp_path: Path) -> None:
    entry = _entries(_run(library, tmp_path))["#item_20"]
    assert entry["link"]["method"] == "item-number"
    assert entry["item_number_check"]["status"] == "agrees"
    assert _kinds(entry) == {"missing-file"}
    assert entry["availability"] == {
        "pdf_records": 1,
        "pdf_files_present": 0,
        "pdf_files_readable": 0,
    }


def test_one_file_backing_two_records_is_recorded_on_both(library: Fixture, tmp_path: Path) -> None:
    entries = _entries(_run(library, tmp_path))
    one, two = entries["http://example.org/one"], entries["http://example.org/two"]
    assert _kinds(one) == _kinds(two) == {"shared-document-file"}
    (shared,) = [a for a in one["ambiguities"] if a["kind"] == "shared-document-file"]
    assert shared["evidence"]["other_export_ids"] == ["http://example.org/two"]
    assert shared["evidence"]["sha256"] == sha256_file(library.shared_pdf)
    assert two["attachments"][0]["link_mode"] == "linked_file"


def test_field_match_records_its_evidence_and_the_disagreement(
    library: Fixture, tmp_path: Path
) -> None:
    entry = _entries(_run(library, tmp_path))["http://example.org/fieldmatch"]
    assert entry["link"]["method"] == "field-match"
    assert entry["link"]["evidence"]["matched_fields"] == ["item_type", "title"]
    assert entry["source_identity"]["item_key"] == key(50)
    assert _kinds(entry) == {"field-match", "no-pdf", "rdf-db-disagreement"}
    disagreements = [
        a["evidence"] for a in entry["ambiguities"] if a["kind"] == "rdf-db-disagreement"
    ]
    assert disagreements == [{"field": "edition", "rdf": "1st", "database": "First"}]


def test_item_number_check_reports_a_title_mismatch(library: Fixture, tmp_path: Path) -> None:
    entry = _entries(_run(library, tmp_path))["#item_60"]
    check = entry["item_number_check"]
    assert check["status"] == "disagrees"
    assert check["reasons"] == ["the title differs"]
    assert _kinds(entry) == {"item-number-mismatch", "no-pdf", "rdf-db-disagreement"}


def test_unreadable_pdf_is_recorded(library: Fixture, tmp_path: Path) -> None:
    entry = _entries(_run(library, tmp_path))["#item_90"]
    assert _kinds(entry) == {"unreadable-pdf"}
    file_info = entry["attachments"][0]["file"]
    assert file_info["exists"] is True
    assert file_info["readable"] is False
    assert file_info["error"]


def test_audit_explains_a_discrepancy(library: Fixture, tmp_path: Path) -> None:
    audit = _manifest(_run(library, tmp_path, ImportConfig()))["audit"]
    records = next(check for check in audit["checks"] if check["figure"] == "records")
    assert records["status"] == "discrepancy"
    assert records["observed"] == 7
    assert "audit figure is stale" in records["explanation"]
    assert audit["book_pdf_records"] == {"none": 1, "one": 4, "several": 1}


def test_audit_agrees_with_matching_figures(library: Fixture, tmp_path: Path) -> None:
    audit = _manifest(_run(library, tmp_path))["audit"]
    assert [check["status"] for check in audit["checks"]] == ["agrees"] * 3


def test_dependency_records_name_every_input(library: Fixture, tmp_path: Path) -> None:
    result = _run(library, tmp_path)
    dependency = result.source_manifest.record.dependency
    roles = {item.role: item for item in dependency.inputs}
    assert set(roles) == {"export-snapshot", "rdf-export", "zotero-database", "zotero-storage"}
    assert roles["rdf-export"].identity == sha256_file(library.rdf)
    assert roles["export-snapshot"].identity == result.export_snapshot.record.key
    assert roles["zotero-storage"].kind == "file-set"
    assert dependency.code_version.startswith("urg ")
    assert dependency.configuration["expected_audit_figures"]["records"] == 7
    alias_inputs = {item.role for item in result.alias_table.record.dependency.inputs}
    assert alias_inputs == {"export-snapshot", "source-manifest"}


def test_alias_table_maps_export_ids_to_source_identities(library: Fixture, tmp_path: Path) -> None:
    result = _run(library, tmp_path)
    aliases = json.loads(result.alias_table.read_bytes("aliases.json"))
    by_id = {alias["export_id"]: alias for alias in aliases["aliases"]}
    assert by_id["#item_11"]["source_identity"]["item_key"] == key(11)
    assert by_id["urn:isbn:9780262510875"]["source_identity"]["item_key"] == key(10)
    assert aliases["export_snapshot_sha256"] == sha256_file(library.rdf)


def test_unchanged_inputs_reuse_the_stored_artifacts(library: Fixture, tmp_path: Path) -> None:
    first = _run(library, tmp_path)
    second = _run(library, tmp_path)
    assert not first.reused
    assert second.reused
    assert second.source_manifest.directory == first.source_manifest.directory


def test_a_changed_storage_file_rebuilds_the_manifest(library: Fixture, tmp_path: Path) -> None:
    first = _run(library, tmp_path)
    stat = library.shared_pdf.stat()
    os.utime(library.shared_pdf, ns=(stat.st_atime_ns, stat.st_mtime_ns + 1_000_000_000))
    second = _run(library, tmp_path)
    assert not second.reused
    assert second.source_manifest.record.key != first.source_manifest.record.key
    assert second.export_snapshot.record.key == first.export_snapshot.record.key


def test_the_database_is_not_changed(library: Fixture, tmp_path: Path) -> None:
    database = library.zotero_dir / "zotero.sqlite"
    before = sha256_file(database)
    _run(library, tmp_path)
    assert sha256_file(database) == before


def test_a_non_empty_write_ahead_log_is_refused(library: Fixture, tmp_path: Path) -> None:
    (library.zotero_dir / "zotero.sqlite-wal").write_bytes(b"pending")
    with pytest.raises(ZoteroDbError):
        _run(library, tmp_path)


def test_summary_names_the_books_that_need_review(library: Fixture, tmp_path: Path) -> None:
    summary = _run(library, tmp_path).source_manifest.read_bytes("summary.md").decode()
    assert "## Books that need review" in summary
    assert f"`{key(10)}` Structure and Interpretation of Computer Programs: several-pdfs" in summary


def test_container_isbn_belongs_to_the_record(tmp_path: Path) -> None:
    rdf = tmp_path / "paper.rdf"
    rdf.write_text(
        '<?xml version="1.0"?>\n'
        '<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"'
        ' xmlns:z="http://www.zotero.org/namespaces/export#"'
        ' xmlns:dc="http://purl.org/dc/elements/1.1/"'
        ' xmlns:dcterms="http://purl.org/dc/terms/"'
        ' xmlns:bib="http://purl.org/net/biblio#">\n'
        '<rdf:Description rdf:about="urn:isbn:978-1-4503-2873-9">'
        "<z:itemType>conferencePaper</z:itemType>"
        "<dcterms:isPartOf><bib:Journal>"
        "<dc:identifier>ISBN 978-1-4503-2873-9</dc:identifier>"
        "</bib:Journal></dcterms:isPartOf>"
        "<dc:title>A Paper</dc:title></rdf:Description>\n"
        "</rdf:RDF>\n"
    )
    (record,) = read_export(rdf).records
    assert record.isbns == ("9781450328739",)


def test_a_field_match_links_one_record_to_one_item(library: Fixture, tmp_path: Path) -> None:
    text = library.rdf.read_text()
    copy = (
        '<bib:Book rdf:about="http://example.org/fieldmatch-copy"><z:itemType>book</z:itemType>'
        "<dc:title>Field Match Book</dc:title></bib:Book>\n"
    )
    library.rdf.write_text(text.replace("</rdf:RDF>", copy + "</rdf:RDF>"))
    entries = _entries(_run(library, tmp_path))
    rivals = ["http://example.org/fieldmatch", "http://example.org/fieldmatch-copy"]
    for about in rivals:
        entry = entries[about]
        assert entry["source_identity"] is None
        assert entry["reconciliation_status"] == "unlinked"
        (unlinked,) = [a for a in entry["ambiguities"] if a["kind"] == "no-source-identity"]
        assert unlinked["evidence"]["candidate_item_keys"] == [key(50)]
        assert unlinked["evidence"]["rival_records"] == rivals
    keys = [
        entry["source_identity"]["item_key"]
        for entry in entries.values()
        if entry["source_identity"] is not None
    ]
    assert len(keys) == len(set(keys))


def test_a_book_does_not_take_the_isbn_of_its_series(tmp_path: Path) -> None:
    rdf = tmp_path / "book.rdf"
    rdf.write_text(
        '<?xml version="1.0"?>\n'
        '<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"'
        ' xmlns:z="http://www.zotero.org/namespaces/export#"'
        ' xmlns:dc="http://purl.org/dc/elements/1.1/"'
        ' xmlns:dcterms="http://purl.org/dc/terms/"'
        ' xmlns:bib="http://purl.org/net/biblio#">\n'
        '<bib:Book rdf:about="#item_1"><z:itemType>book</z:itemType>'
        "<dcterms:isPartOf><bib:Series>"
        "<dc:identifier>ISBN 978-1-4503-2873-9</dc:identifier>"
        "</bib:Series></dcterms:isPartOf>"
        "<dc:title>A Book</dc:title></bib:Book>\n"
        "</rdf:RDF>\n"
    )
    (record,) = read_export(rdf).records
    assert record.isbns == ()
