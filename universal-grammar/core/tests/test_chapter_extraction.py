"""Tests of the chapter-extraction stage over a generated five-page book."""

from __future__ import annotations

import json
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import pymupdf  # pyright: ignore[reportMissingTypeStubs]
import pytest

from urg import chapter_layout as layout
from urg import extraction
from urg.extraction import (
    RECORD_FILE,
    ExtractionConfig,
    ExtractionError,
    ExtractionRun,
    latest_extraction,
    render_anchor,
    run_extraction,
)
from urg.store import ArtifactStore, DependencyRecord, InputIdentity
from urg.zotero_db import sha256_file

Json = dict[str, Any]

HIDDEN = "hidden words"


def _write_book(path: Path) -> None:
    """Write a book whose chapter 2 runs from page index 1 to 3, with labels from 10."""
    doc: Any = pymupdf.open()
    for index in range(5):
        page = doc.new_page(width=400, height=600)
        page.insert_text((190, 585), str(10 + index), fontsize=9, fontname="helv")
        if index == 0:
            page.insert_text((40, 80), "1 Preface", fontsize=18, fontname="helv")
            page.insert_text((40, 120), "The preface ends here.", fontsize=10, fontname="helv")
        elif index == 1:
            page.insert_text((40, 80), "2 Parsing", fontsize=18, fontname="helv")
            page.insert_text(
                (40, 120), "A parser turns tokens into a tree.", fontsize=10, fontname="helv"
            )
            page.insert_text((40, 160), "2.1 Tokens", fontsize=13, fontname="helv")
            page.insert_text(
                (40, 190), "The tree in Figure 1 shows the result.", fontsize=10, fontname="helv"
            )
        elif index == 2:
            shape = page.new_shape()
            shape.draw_rect(pymupdf.Rect(100, 100, 300, 220))
            shape.draw_line((100, 100), (300, 220))
            shape.draw_line((300, 100), (100, 220))
            shape.finish(color=(0, 0, 0), width=1)
            shape.commit()
            page.insert_text((100, 245), "Figure 1. A parse tree.", fontsize=9, fontname="helv")
            page.insert_text((40, 300), "Each node holds one token.", fontsize=10, fontname="helv")
        elif index == 3:
            page.insert_text((40, 80), "The parser is short:", fontsize=10, fontname="helv")
            page.insert_text((60, 110), "def parse(tokens):", fontsize=9, fontname="cour")
            page.insert_text((60, 122), "    return tree(tokens)", fontsize=9, fontname="cour")
            page.insert_text((40, 160), "That ends the chapter.", fontsize=10, fontname="helv")
            page.insert_text((40, 300), HIDDEN, fontsize=10, fontname="helv", color=(1, 1, 1))
        else:
            page.insert_text((40, 80), "3 Evaluation", fontsize=18, fontname="helv")
            page.insert_text(
                (40, 120), "An evaluator walks the tree.", fontsize=10, fontname="helv"
            )
    doc.set_toc(
        [[1, "1 Preface", 1], [1, "2 Parsing", 2], [2, "2.1 Tokens", 2], [1, "3 Evaluation", 5]]
    )
    doc.set_page_labels([{"startpage": 0, "prefix": "", "style": "D", "firstpagenum": 10}])
    doc.save(path)
    doc.close()


@dataclass(frozen=True)
class Library:
    selection: Path
    store: ArtifactStore
    pdf: Path


def _entry(item_key: str, attachment: str, sha: str) -> Json:
    return {
        "source_identity": {"library_id": 1, "library_type": "user", "item_key": item_key},
        "title": f"Book {item_key}",
        "year": 2020,
        "attachment_item_key": attachment,
        "document_file_sha256": sha,
        "page_count": 5,
        "approach": "fixture",
        "chapter": {
            "number": 2,
            "title": "Parsing",
            "pdf_page_index_first": 1,
            "pdf_page_index_last": 3,
            "printed_page_label_first": "11",
            "printed_page_label_last": "13",
            "boundary_evidence": {},
        },
    }


def _write_manifest(store: ArtifactStore, entries: list[Json], code: str = "fixture") -> str:
    """Store a source manifest as one catalogue code version and return its key."""
    manifest = store.write(
        DependencyRecord(
            stage="catalogue-manifest",
            artifact="source-manifest",
            schema_version=1,
            code_version=code,
            configuration={},
            inputs=[
                InputIdentity(role="fixture", kind="file", locator="fixture", identity="0" * 64)
            ],
        ),
        {"manifest.json": json.dumps({"entries": entries}).encode()},
    )
    return manifest.record.key


def _library(root: Path, books: list[str], wrong_sha: set[str] | None = None) -> Library:
    root.mkdir(parents=True, exist_ok=True)
    pdf = root / "book.pdf"
    _write_book(pdf)
    sha = sha256_file(pdf)
    wrong_sha = wrong_sha or set()
    entries = [
        {
            "source_identity": {"library_id": 1, "library_type": "user", "item_key": book},
            "attachments": [
                {"item_key": f"A{book}", "file": {"location": str(pdf), "sha256": sha}}
            ],
        }
        for book in books
    ]
    store = ArtifactStore(root / "data")
    manifest_key = _write_manifest(store, entries)
    selection = {
        "schema_version": 1,
        "stage": "extraction-probe",
        "source_manifest_sha256": manifest_key,
        "page_index_base": 0,
        "selected": [
            _entry(book, f"A{book}", "f" * 64 if book in wrong_sha else sha) for book in books
        ],
    }
    path = root / "chapter-selection.json"
    path.write_text(json.dumps(selection))
    return Library(path, store, pdf)


def _record(run: ExtractionRun, item_key: str) -> Json:
    outcome = next(o for o in run.outcomes if o.item_key == item_key)
    assert outcome.artifact is not None, outcome.error
    return json.loads(outcome.artifact.read_bytes(RECORD_FILE))


@pytest.fixture
def library(tmp_path: Path) -> Library:
    return _library(tmp_path / "lib", ["BOOK1"])


def test_each_anchor_resolves_to_its_text_on_its_page(library: Library) -> None:
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    assert record["evidence"]
    assert record["checks"]["anchor_resolution"]["passed"]
    with layout.open_pdf(library.pdf) as pdf:
        for item in record["evidence"]:
            assert 1 <= item["pdf_page_index"] <= 3
            text = layout.clipped_text(pdf[item["pdf_page_index"]], tuple(item["bbox"]))
            assert layout.norm(item["text"]) in layout.norm(text)


def test_page_index_and_printed_label_are_separate_fields(library: Library) -> None:
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    first = next(i for i in record["evidence"] if i["pdf_page_index"] == 1)
    assert first["printed_page_label"] == "11"
    assert first["printed_page_label_method"] == "page-labels-dictionary"
    assert record["chapter"]["printed_page_label_first"]["value"] == "11"
    assert record["chapter"]["printed_page_label_last"]["value"] == "13"


def test_ids_are_stable_across_reruns(tmp_path: Path) -> None:
    first = _record(run_extraction(*_args(_library(tmp_path / "a", ["BOOK1"]))), "BOOK1")
    second = _record(run_extraction(*_args(_library(tmp_path / "b", ["BOOK1"]))), "BOOK1")
    assert [i["id"] for i in first["evidence"]] == [i["id"] for i in second["evidence"]]
    elements = first["structure"]["elements"]
    assert [e["id"] for e in elements] == [e["id"] for e in second["structure"]["elements"]]
    assert all(i["id"].startswith("BOOK1:p000") for i in first["evidence"])


def _args(library: Library) -> tuple[Path, ArtifactStore]:
    return library.selection, library.store


def test_boundary_and_representative_checks_are_recorded(library: Library) -> None:
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    boundary = record["checks"]["boundary"]
    assert boundary["passed"], boundary["assertions"]
    assert boundary["rendered_pages"] == [0, 1, 3, 4]
    purposes = {p["purpose"]: p for p in record["checks"]["representative_pages"]}
    assert set(purposes) == {"first page", "last page", "code page", "figure page"}
    assert purposes["figure page"]["pdf_page_index"] == 2
    assert purposes["code page"]["pdf_page_index"] == 3
    for name in record["renders"]:
        assert name.startswith("render-p")


def test_structure_holds_figure_caption_and_code(library: Library) -> None:
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    kinds = [e["kind"] for e in record["structure"]["elements"]]
    assert {"heading", "paragraph", "code_block", "figure", "caption"} <= set(kinds)
    relations = {r["kind"] for r in record["relationships"]}
    assert "captions-figure" in relations
    assert record["book_outline"]["chapter_entry"]["title"] == "2 Parsing"
    assert record["coverage"]["scope"] == "one-chapter"


def test_invisible_text_found_by_the_render_check_is_a_defect(library: Library) -> None:
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    hidden = [i["id"] for i in record["evidence"] if i["text"].strip() == HIDDEN]
    defects = [d for d in record["defects"] if d["kind"] == "invisible-text"]
    assert hidden and defects
    assert hidden[0] in defects[0]["evidence"]
    code_page = next(
        p for p in record["checks"]["representative_pages"] if p["purpose"] == "code page"
    )
    assert not code_page["passed"]
    mismatch = next(d for d in record["defects"] if d["id"] == code_page["defect"])
    assert mismatch["kind"] == "render-mismatch"
    assert hidden[0] in mismatch["evidence"]


def test_a_failure_only_at_the_render_resolution_is_a_defect(
    library: Library, monkeypatch: pytest.MonkeyPatch
) -> None:
    """The coverage check passes at 72 dpi, and the representative check fails at 110 dpi."""
    original = extraction._ink_check  # pyright: ignore[reportPrivateUsage]
    flagged: list[str] = []

    def ink_check(document: Any, page: Any, dpi: int, settings: Any) -> Json:
        result = original(document, page, dpi, settings)
        if page.index == 1 and dpi == settings.render_dpi:
            flagged.append(page.lines[0].id)
            result["text_lines_without_ink"] = [page.lines[0].id]
        return result

    monkeypatch.setattr(extraction, "_ink_check", ink_check)
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    assert flagged
    on_page = [d["kind"] for d in record["defects"] if d["pdf_page_index"] == 1]
    assert on_page == ["render-mismatch"]
    first = next(
        p for p in record["checks"]["representative_pages"] if p["purpose"] == "first page"
    )
    assert not first["passed"]
    defect = next(d for d in record["defects"] if d["id"] == first["defect"])
    assert defect["kind"] == "render-mismatch"
    assert defect["evidence"] == flagged


def test_an_unresolved_anchor_is_a_defect(
    library: Library, monkeypatch: pytest.MonkeyPatch
) -> None:
    original = extraction._anchor_check  # pyright: ignore[reportPrivateUsage]

    def anchor_check(document: Any, evidence: list[Json]) -> Json:
        result = original(document, evidence)
        result["unresolved"] = [evidence[0]["id"]]
        result["passed"] = False
        return result

    monkeypatch.setattr(extraction, "_anchor_check", anchor_check)
    record = _record(run_extraction(library.selection, library.store), "BOOK1")
    anchors = record["checks"]["anchor_resolution"]
    defects = [d for d in record["defects"] if d["id"] in anchors["defects"]]
    assert [d["kind"] for d in defects] == ["unresolved-anchor"]
    assert defects[0]["evidence"] == [record["evidence"][0]["id"]]
    assert defects[0]["pdf_page_index"] == record["evidence"][0]["pdf_page_index"]


def test_rerun_reuses_and_an_override_invalidates_only_that_book(tmp_path: Path) -> None:
    library = _library(tmp_path / "lib", ["BOOK1", "BOOK2"])
    first = run_extraction(library.selection, library.store)
    assert [o.reused for o in first.outcomes] == [False, False]
    again = run_extraction(library.selection, library.store)
    assert [o.reused for o in again.outcomes] == [True, True]
    config = ExtractionConfig(book_overrides={"BOOK2": {"render_dpi": 90}})
    changed = run_extraction(library.selection, library.store, config)
    assert [o.reused for o in changed.outcomes] == [True, False]
    keys = [o.artifact.record.key for o in changed.outcomes if o.artifact]
    assert keys[0] == first.outcomes[0].artifact.record.key  # pyright: ignore[reportOptionalMemberAccess]
    assert keys[1] != first.outcomes[1].artifact.record.key  # pyright: ignore[reportOptionalMemberAccess]


def test_a_failed_book_does_not_stop_the_others(tmp_path: Path) -> None:
    library = _library(tmp_path / "lib", ["BAD", "BOOK1"], wrong_sha={"BAD"})
    run = run_extraction(library.selection, library.store)
    bad, good = run.outcomes
    assert bad.error is not None and "SHA-256" in bad.error
    assert good.error is None and good.artifact is not None
    report = json.loads(run.report.read_bytes("report.json"))
    assert [r["status"] for r in report["results"]] == ["failed", "extracted"]


def test_the_latest_manifest_serves_a_selection_that_names_an_absent_one(
    library: Library,
) -> None:
    """A clean store holds only the manifest of the current catalogue code."""
    selection = json.loads(library.selection.read_text())
    selection["source_manifest_sha256"] = "e" * 64
    library.selection.write_text(json.dumps(selection))
    run = run_extraction(library.selection, library.store)
    assert not run.failed
    latest = library.store.latest_key("source-manifest")
    inputs = {i.role: i.identity for i in run.report.record.dependency.inputs}
    assert inputs["source-manifest"] == latest


def test_the_latest_manifest_wins_when_the_pinned_one_is_also_stored(
    library: Library, tmp_path: Path
) -> None:
    """Two catalogue code versions: the pin names the older manifest, which is present."""
    selection = json.loads(library.selection.read_text())
    pinned = selection["source_manifest_sha256"]
    moved = tmp_path / "moved" / "book.pdf"
    moved.parent.mkdir()
    shutil.copy(library.pdf, moved)
    library.pdf.unlink()
    sha = sha256_file(moved)
    entries = [
        {
            "source_identity": {"library_id": 1, "library_type": "user", "item_key": "BOOK1"},
            "attachments": [
                {"item_key": "ABOOK1", "file": {"location": str(moved), "sha256": sha}}
            ],
        }
    ]
    latest = _write_manifest(library.store, entries, code="fixture-2")
    assert latest != pinned
    library.store.load("source-manifest", pinned)  # the pinned manifest is stored
    run = run_extraction(library.selection, library.store)
    assert not run.failed, run.failed[0].error
    inputs = {i.role: i.identity for i in run.report.record.dependency.inputs}
    assert inputs["source-manifest"] == latest


def test_an_equal_item_key_in_another_library_does_not_serve_the_selection(
    library: Library,
) -> None:
    """Library and item key together name a record, so matching bytes do not suffice."""
    manifest = json.loads(
        library.store.load(
            "source-manifest", library.store.latest_key("source-manifest") or ""
        ).read_bytes("manifest.json")
    )
    manifest["entries"][0]["source_identity"]["library_id"] = 2
    _write_manifest(library.store, manifest["entries"], code="fixture-2")
    run = run_extraction(library.selection, library.store)
    assert run.failed and "is not in the source manifest" in (run.failed[0].error or "")


def test_extraction_without_a_manifest_names_the_stage_to_run(tmp_path: Path) -> None:
    library = _library(tmp_path / "lib", ["BOOK1"])
    with pytest.raises(ExtractionError, match="Run catalogue-manifest first"):
        run_extraction(library.selection, ArtifactStore(tmp_path / "empty"))


def test_a_missing_document_is_recorded_for_its_book(library: Library) -> None:
    shutil.move(library.pdf, library.pdf.with_suffix(".moved"))
    run = run_extraction(library.selection, library.store)
    assert run.failed and "does not exist" in (run.failed[0].error or "")


def test_an_anchor_renders_its_page_region(library: Library) -> None:
    run_extraction(library.selection, library.store)
    artifact = latest_extraction(library.store, "BOOK1")
    record = json.loads(artifact.read_bytes(RECORD_FILE))
    png = render_anchor(artifact, record["evidence"][0]["id"])
    assert png.startswith(b"\x89PNG")
    figure = next(e for e in record["structure"]["elements"] if e["kind"] == "figure")
    assert render_anchor(artifact, figure["id"]).startswith(b"\x89PNG")


def test_a_reused_extraction_renders_after_its_document_moves(library: Library) -> None:
    run_extraction(library.selection, library.store)
    moved = library.pdf.with_name("moved.pdf")
    shutil.move(library.pdf, moved)
    manifest = json.loads(
        library.store.load(
            "source-manifest", library.store.latest_key("source-manifest") or ""
        ).read_bytes("manifest.json")
    )
    for entry in manifest["entries"]:
        for attachment in entry["attachments"]:
            attachment["file"]["location"] = str(moved)
    library.store.write(
        DependencyRecord(
            stage="catalogue-manifest",
            artifact="source-manifest",
            schema_version=1,
            code_version="fixture",
            configuration={"moved": True},
            inputs=[
                InputIdentity(role="fixture", kind="file", locator="fixture", identity="1" * 64)
            ],
        ),
        {"manifest.json": json.dumps(manifest).encode()},
    )
    again = run_extraction(library.selection, library.store)
    assert [o.reused for o in again.outcomes] == [True]
    artifact = latest_extraction(library.store, "BOOK1")
    record = json.loads(artifact.read_bytes(RECORD_FILE))
    assert record["source"]["document"]["location"] == str(library.pdf)
    evidence_id = record["evidence"][0]["id"]
    with pytest.raises(ExtractionError, match="missing or changed"):
        render_anchor(artifact, evidence_id)
    png = render_anchor(artifact, evidence_id, store=library.store)
    assert png.startswith(b"\x89PNG")


def test_a_rule_near_the_foot_is_found_and_a_running_foot_is_furniture(tmp_path: Path) -> None:
    path = tmp_path / "rules.pdf"
    doc: Any = pymupdf.open()
    page = doc.new_page(width=400, height=600)
    page.insert_text(
        (40, 80), "The iteration is defined by one rule:", fontsize=10, fontname="helv"
    )
    shape = page.new_shape()
    shape.draw_line((180, 548), (220, 548))  # The bar of an axiom, inside the bottom band.
    shape.draw_line((40, 572), (360, 572))  # The rule over the running foot.
    shape.finish(color=(0, 0, 0), width=0.4)
    shape.commit()
    page.insert_text((184, 560), "s to s", fontsize=10, fontname="helv")
    page.insert_text((320, 556), "(5.1a)", fontsize=10, fontname="helv")
    page.insert_text((40, 584), "2.1 Tokens | 12", fontsize=8, fontname="helv")
    doc.set_page_labels([{"startpage": 0, "prefix": "", "style": "D", "firstpagenum": 12}])
    doc.save(path)
    doc.close()
    settings = layout.Settings()
    with layout.open_pdf(path) as pdf:
        data = layout.read_page(pdf[0], 0, "T:", settings)
        layout.assign_labels([data], {0: layout.page_label(pdf[0])})
        layout.mark_furniture([data], settings.margin_band)
        layout.find_rules(data, settings)
    foot = next(ln for ln in data.lines if ln.text.startswith("2.1"))
    assert foot.kind == "furniture"
    assert len(data.rules) == 1
    texts = {ln.id: ln.text for ln in data.lines}
    assert [texts[i] for i in data.rules[0]["conclusion"]] == ["s to s"]
    assert [texts[i] for i in data.rules[0]["label"]] == ["(5.1a)"]
