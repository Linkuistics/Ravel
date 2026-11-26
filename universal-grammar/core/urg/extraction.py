"""The chapter-extraction stage.

The stage reads the chapter-selection record and resolves each selected book's PDF
through the source manifest. It writes one chapter-extraction research artifact per
book. Each artifact holds the chapter's evidence lines with their anchors, the
document structure, the relationships, the defect list, the checks against
rendered pages and the renders themselves.

One book's failure does not stop the others. A run-report artifact records which
books succeeded, which were reused and which failed.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import asdict, dataclass, field, fields, replace
from pathlib import Path
from typing import Any

import pymupdf  # pyright: ignore[reportMissingTypeStubs]

from urg import chapter_layout as layout
from urg.chapter_layout import Box, Json, PageData, Settings
from urg.errors import UrgError
from urg.store import (
    ArtifactStore,
    DependencyRecord,
    InputIdentity,
    StoredArtifact,
    StoreError,
    canonical_json,
)
from urg.version import code_version
from urg.zotero_db import sha256_file

STAGE = "chapter-extraction"
SCHEMA_VERSION = 1
EXTRACTION = "chapter-extraction"
RUN_REPORT = "chapter-extraction-run"
SOURCE_MANIFEST = "source-manifest"
RECORD_FILE = "extraction.json"
SUMMARY_FILE = "summary.md"

KNOWN_LIMITATIONS = [
    "The record covers one chapter. It makes no claim about the rest of the book.",
    "Text comes from the PDF text layer. No OCR runs, so no OCR confidence exists.",
    "A paragraph is one text block of the text layer. A block can join or split paragraphs.",
    "Reading order is the stored block order. The defect list names pages where it differs "
    "from the geometric order.",
    "Code is found by monospace fonts. Code set in a proportional font reads as a paragraph.",
    "A figure is an image or a cluster of vector drawings above a size threshold. Text-heavy "
    "shaded boxes are recorded as boxes, not figures.",
    "Heading levels come from font sizes inside this chapter only.",
    "Inference rules are found from thin bars with a centred line below. Premise "
    "structure inside a line, such as fractions and subscripts, is not recovered.",
    "Code indentation is estimated from line offsets and the median monospace character width.",
]


class ExtractionError(UrgError):
    """A chapter could not be extracted."""


@dataclass(frozen=True)
class ExtractionConfig:
    """The stage configuration: shared settings and per-book overrides."""

    settings: Settings = field(default_factory=Settings)
    book_overrides: dict[str, dict[str, Any]] = field(default_factory=dict[str, dict[str, Any]])

    def for_book(self, item_key: str) -> Settings:
        overrides = self.book_overrides.get(item_key, {})
        known = {f.name for f in fields(Settings)}
        unknown = sorted(set(overrides) - known)
        if unknown:
            raise ExtractionError(f"unknown settings for {item_key}: {', '.join(unknown)}")
        return replace(self.settings, **overrides)


def load_config(path: Path) -> ExtractionConfig:
    """Read a JSON configuration with optional ``settings`` and ``book_overrides`` keys."""
    try:
        data: Json = json.loads(path.read_text())
        settings = Settings(**data.get("settings", {}))
    except (OSError, ValueError, TypeError) as error:
        raise ExtractionError(
            f"cannot read the extraction configuration {path}: {error}"
        ) from error
    unknown = sorted(set(data) - {"settings", "book_overrides"})
    if unknown:
        raise ExtractionError(f"{path}: unknown keys {', '.join(unknown)}")
    config = ExtractionConfig(settings, dict(data.get("book_overrides", {})))
    for item_key in config.book_overrides:
        config.for_book(item_key)
    return config


@dataclass(frozen=True)
class BookOutcome:
    item_key: str
    title: str
    artifact: StoredArtifact | None
    reused: bool
    error: str | None


@dataclass(frozen=True)
class ExtractionRun:
    outcomes: list[BookOutcome]
    report: StoredArtifact

    @property
    def failed(self) -> list[BookOutcome]:
        return [o for o in self.outcomes if o.error is not None]


def _json_bytes(value: Json) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


# Inputs -----------------------------------------------------------------------------


def load_selection(path: Path) -> Json:
    try:
        selection: Json = json.loads(path.read_text())
    except (OSError, ValueError) as error:
        raise ExtractionError(f"cannot read the chapter selection {path}: {error}") from error
    if not isinstance(selection.get("selected"), list) or not selection.get(
        "source_manifest_sha256"
    ):
        raise ExtractionError(f"{path} is not a chapter-selection record")
    return selection


def load_manifest(store: ArtifactStore) -> tuple[str, Json]:
    """Return the key and the content of the latest source manifest in the store.

    The stage does not use the manifest key that the chapter selection names. That key
    records the manifest that the probe read. A change to the catalogue-manifest code gives
    a new manifest key, and a clean store holds only the new one. The stage identifies each
    book by its source identity and its file SHA-256, so any manifest that resolves both
    serves.
    """
    key = store.latest_key(SOURCE_MANIFEST)
    if key is None:
        raise ExtractionError("no source manifest is in the store. Run catalogue-manifest first")
    try:
        artifact = store.load(SOURCE_MANIFEST, key)
        manifest: Json = json.loads(artifact.read_bytes("manifest.json"))
    except (StoreError, OSError, ValueError) as error:
        raise ExtractionError(
            f"the latest source manifest {key} cannot be read: {error}"
        ) from error
    return key, manifest


def resolve_document(manifest: Json, entry: Json) -> Path:
    """Return the PDF that the source manifest records for a selected book.

    A record matches only when its whole source identity equals the
    selection's: library and item key together name a Zotero record, so an
    equal item key in another library is a different record.
    """
    selected: Json = entry["source_identity"]
    item_key = selected["item_key"]
    attachment_key = entry["attachment_item_key"]
    entries: list[Json] = manifest["entries"]
    for record in entries:
        if record.get("source_identity") != selected:
            continue
        for attachment in record.get("attachments", []):
            if attachment.get("item_key") != attachment_key:
                continue
            file: Json = attachment.get("file") or {}
            if file.get("sha256") != entry["document_file_sha256"]:
                raise ExtractionError(
                    f"{item_key}: the manifest records SHA-256 {file.get('sha256')} for "
                    f"{attachment_key}, but the selection names {entry['document_file_sha256']}"
                )
            return Path(file["location"])
    raise ExtractionError(f"{item_key}: attachment {attachment_key} is not in the source manifest")


def book_dependency(entry: Json, settings: Settings) -> DependencyRecord:
    """Return the dependency record of one book's extraction.

    The inputs name the document by attachment key and content hash, so a moved
    file with the same content keeps its stored extraction.
    """
    identity = entry["source_identity"]
    return DependencyRecord(
        stage=STAGE,
        artifact=EXTRACTION,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={
            "settings": asdict(settings),
            "text_flags": layout.TEXT_FLAGS,
            "source_identity": identity,
            "chapter": {k: v for k, v in entry["chapter"].items() if k != "boundary_evidence"},
        },
        inputs=[
            InputIdentity(
                role="document-file",
                kind="file",
                locator=f"zotero-attachment:{identity['library_id']}/{entry['attachment_item_key']}",
                identity=entry["document_file_sha256"],
            ),
            InputIdentity(
                role="chapter-selection-entry",
                kind="file",
                locator=f"chapter-selection#{identity['item_key']}",
                identity=_sha256(canonical_json(entry)),
            ),
        ],
    )


# Extraction -----------------------------------------------------------------------------


def _outline(document: Any, entry: Json) -> tuple[Json, list[Json]]:
    """Place the chapter in the book outline. Return the outline record and its defects."""
    toc: list[list[Any]] = document.get_toc(simple=True)
    chapter = entry["chapter"]
    first = chapter["pdf_page_index_first"]
    stored: list[Json] = [
        {
            "stored_position": i,
            "level": int(level),
            "title": str(title),
            "target_pdf_page_index": int(page) - 1,
        }
        for i, (level, title, page, *_rest) in enumerate(toc)
    ]
    defects: list[Json] = []
    ordered = sorted(stored, key=lambda e: (e["target_pdf_page_index"], e["stored_position"]))
    if [e["stored_position"] for e in ordered] != list(range(len(stored))):
        out_of_order = sum(
            1
            for a, b in zip(stored, stored[1:], strict=False)
            if b["target_pdf_page_index"] < a["target_pdf_page_index"]
        )
        defects.append(
            {
                "kind": "outline-order",
                "pdf_page_index": None,
                "detail": f"The stored outline is not in page order: {out_of_order} entries point "
                "earlier than the entry before them. The outline is read in page order.",
                "evidence": [],
            }
        )
    wanted = layout.title_key(chapter["title"])
    match = next(
        (
            e
            for e in ordered
            if e["target_pdf_page_index"] == first and wanted in layout.title_key(e["title"])
        ),
        None,
    )
    record: Json = {"entries_in_outline": len(stored), "chapter_entry": match}
    if match is None:
        defects.append(
            {
                "kind": "outline-missing-chapter",
                "pdf_page_index": first,
                "detail": f"No outline entry titled like {chapter['title']!r} points at "
                f"page index {first}.",
                "evidence": [],
            }
        )
        record.update(siblings=[], children=[], next_entry=None)
        return record, defects
    level = match["level"]
    deeper_titles = {layout.norm(e["title"]) for e in stored if e["level"] > level}
    duplicates = [
        e
        for e in ordered
        if e["level"] <= level and e is not match and layout.norm(e["title"]) in deeper_titles
    ]
    if duplicates:
        defects.append(
            {
                "kind": "outline-duplicate-entries",
                "pdf_page_index": None,
                "detail": f"{len(duplicates)} entries at the chapter's level repeat a "
                "deeper entry, "
                "for example " + repr(duplicates[0]["title"]) + ". They are skipped when "
                "the next chapter is found.",
                "evidence": [],
            }
        )
    record["skipped_duplicate_entries"] = duplicates
    siblings = [
        dict(e, selected=e is match) for e in ordered if e["level"] == level and e not in duplicates
    ]
    position = ordered.index(match)
    later = [e for e in ordered[position + 1 :] if e["level"] <= level and e not in duplicates]
    next_entry = later[0] if later else None
    end = next_entry["target_pdf_page_index"] if next_entry else 10**9
    children = [
        e
        for e in ordered[position + 1 :]
        if e["level"] > level and e["target_pdf_page_index"] < end
    ]
    record.update(siblings=siblings, children=children, next_entry=next_entry)
    return record, defects


def _evidence(page: PageData) -> list[Json]:
    return [
        {
            "id": line.id,
            "pdf_page_index": page.index,
            "printed_page_label": page.label["value"],
            "printed_page_label_method": page.label["method"],
            "bbox": line.bbox,
            "text": line.text,
            "kind": line.kind,
            "block": line.block,
            "line": line.number,
            "font_size": line.size,
            "monospace_fraction": line.monospace,
            "bold_fraction": line.bold,
            "fonts": line.fonts,
        }
        for line in page.lines
    ]


def _page_record(page: PageData, in_chapter: bool) -> Json:
    stored, geometric = layout.reading_order(page)
    return {
        "pdf_page_index": page.index,
        "in_chapter": in_chapter,
        "printed_page_label": page.label,
        "width": round(page.width, 2),
        "height": round(page.height, 2),
        "stored_block_order": stored,
        "geometric_block_order": geometric,
        "boxes": page.boxes,
        "image_regions": page.images,
        "rule_bars": len(page.bars),
    }


def _contains_heading(page: PageData, title: str) -> str | None:
    """Return how a page shows a chapter title: in a heading, in body text, or not at all."""
    wanted = layout.title_key(title)
    headings = layout.norm(" ".join(ln.text for ln in page.lines if ln.kind == "heading"))
    if wanted and wanted in headings:
        return "heading"
    text = layout.norm(" ".join(ln.text for ln in page.lines if ln.kind != "furniture"))
    return "page-text" if wanted and wanted in text else None


def _boundary_check(
    entry: Json, pages: dict[int, PageData], outline: Json, page_count: int
) -> tuple[Json, list[Json]]:
    chapter = entry["chapter"]
    first, last = chapter["pdf_page_index_first"], chapter["pdf_page_index_last"]
    assertions: list[Json] = []

    def add(name: str, passed: bool, observed: Any) -> None:
        assertions.append({"name": name, "passed": passed, "observed": observed})

    entry_match = outline.get("chapter_entry")
    add(
        "outline-entry-targets-first-page",
        entry_match is not None,
        entry_match
        and {"title": entry_match["title"], "target": entry_match["target_pdf_page_index"]},
    )
    found = _contains_heading(pages[first], chapter["title"])
    add("first-page-opens-chapter", found == "heading", found)
    if first - 1 in pages:
        earlier = _contains_heading(pages[first - 1], chapter["title"])
        add("previous-page-does-not-open-chapter", earlier != "heading", earlier)
    add(
        "last-page-has-content",
        any(ln.kind not in ("furniture", "") for ln in pages[last].lines)
        or bool(pages[last].figures),
        sum(1 for ln in pages[last].lines if ln.kind != "furniture"),
    )
    next_entry = outline.get("next_entry")
    if last + 1 < page_count and next_entry is not None:
        add(
            "next-entry-targets-next-page",
            next_entry["target_pdf_page_index"] == last + 1,
            {"title": next_entry["title"], "target": next_entry["target_pdf_page_index"]},
        )
        opened = _contains_heading(pages[last + 1], next_entry["title"])
        add("next-page-opens-next-chapter", opened == "heading", opened)
    labels = {"first": pages[first].label, "last": pages[last].label}
    add(
        "printed-labels-match-selection",
        labels["first"]["value"] == chapter["printed_page_label_first"]
        and labels["last"]["value"] == chapter["printed_page_label_last"],
        {k: {"value": v["value"], "method": v["method"]} for k, v in labels.items()},
    )
    defects: list[Json] = [
        {
            "kind": "chapter-boundary",
            "pdf_page_index": first if "next" not in a["name"] else last,
            "detail": f"Boundary check {a['name']} failed: observed {a['observed']!r}.",
            "evidence": [],
        }
        for a in assertions
        if not a["passed"]
    ]
    rendered = [i for i in (first - 1, first, last, last + 1) if i in pages]
    check: Json = {
        "pdf_page_index_first": first,
        "pdf_page_index_last": last,
        "rendered_pages": rendered,
        "assertions": assertions,
        "passed": all(a["passed"] for a in assertions),
    }
    return check, defects


def _representative_pages(chapter_pages: list[PageData]) -> list[tuple[str, PageData | None]]:
    def code_lines(page: PageData) -> int:
        return sum(1 for ln in page.lines if ln.kind == "code")

    def figure_area(page: PageData) -> float:
        return sum(layout.area(f["bbox"]) for f in page.figures)

    # Prefer an inner page, so the four checks cover four pages where the chapter allows it.
    inner = chapter_pages[1:-1] or chapter_pages
    code_page = max(inner, key=code_lines)
    if not code_lines(code_page):
        code_page = max(chapter_pages, key=code_lines)
    figure_page = max(inner, key=figure_area)
    if not figure_area(figure_page):
        figure_page = max(chapter_pages, key=figure_area)
    return [
        ("first page", chapter_pages[0]),
        ("last page", chapter_pages[-1]),
        ("code page", code_page if code_lines(code_page) else None),
        ("figure page", figure_page if figure_area(figure_page) else None),
    ]


def _anchor_check(document: Any, evidence: list[Json]) -> Json:
    failures: list[str] = []
    for item in evidence:
        page = document[item["pdf_page_index"]]
        clipped = layout.norm(layout.clipped_text(page, tuple(item["bbox"])))
        if layout.norm(item["text"]) not in clipped:
            failures.append(item["id"])
    return {
        "method": "read the text inside each anchor box back from its page and compare it",
        "evidence_items": len(evidence),
        "resolved": len(evidence) - len(failures),
        "unresolved": failures,
        "passed": not failures,
    }


def _ink_check(document: Any, page: PageData, dpi: int, settings: Settings) -> Json:
    """Compare a page render with the extraction, then recheck lines without contrast."""
    pdf_page = document[page.index]
    result = layout.compare_ink(
        page, layout.ink_map(pdf_page, dpi, settings.ink_threshold), settings
    )
    if result["text_lines_without_ink"]:
        layout.recheck_invisible(pdf_page, page, result, settings)
    return result


def _render_mismatch(purpose: str, index: int, ink: Json, dpi: int) -> Json:
    """Return the defect of a representative page whose render disagrees with the text layer.

    The coverage check at the check resolution can find the same lines or regions.
    The defect stays separate, because it records the result of a different check.
    """
    lines = ink["text_lines_without_ink"]
    regions = ink["uncovered_regions"]
    return {
        "kind": "render-mismatch",
        "pdf_page_index": index,
        "regions": [region["bbox"] for region in regions],
        "detail": f"At {dpi} dpi the render of the {purpose} disagrees with the extraction. "
        f"{len(lines)} text lines cover no drawn ink. "
        f"{len(regions)} ink regions have no text line, image or drawing.",
        "evidence": list(lines),
    }


def _unresolved_anchor_defects(anchors: Json, evidence: list[Json]) -> list[Json]:
    """Return one defect for each evidence item whose box does not hold its stored text."""
    pages = {item["id"]: item["pdf_page_index"] for item in evidence}
    return [
        {
            "kind": "unresolved-anchor",
            "pdf_page_index": pages[item_id],
            "detail": "The text inside the anchor box does not match the stored text.",
            "evidence": [item_id],
        }
        for item_id in anchors["unresolved"]
    ]


def _render_name(index: int) -> str:
    return f"render-p{index:04d}.png"


def extract_chapter(path: Path, entry: Json, settings: Settings) -> tuple[Json, dict[str, bytes]]:
    """Extract one selected chapter. Return the record and the payload files."""
    chapter = entry["chapter"]
    item_key = entry["source_identity"]["item_key"]
    prefix = f"{item_key}:"
    first, last = chapter["pdf_page_index_first"], chapter["pdf_page_index_last"]
    with layout.open_pdf(path) as document:
        page_count = int(document.page_count)
        if page_count != entry["page_count"]:
            raise ExtractionError(
                f"{item_key}: the PDF has {page_count} pages, "
                f"the selection says {entry['page_count']}"
            )
        if not 0 <= first <= last < page_count:
            raise ExtractionError(f"{item_key}: page range {first}-{last} is outside the PDF")
        outline, defects = _outline(document, entry)
        indexes = list(range(max(0, first - 1), min(page_count, last + 2)))
        pages = {i: layout.read_page(document[i], i, prefix, settings) for i in indexes}
        ordered = [pages[i] for i in indexes]
        layout.assign_labels(ordered, {i: layout.page_label(document[i]) for i in indexes})
        layout.mark_furniture(ordered, settings.margin_band)
        chapter_pages = [pages[i] for i in range(first, last + 1)]
        body = layout.body_size(chapter_pages)
        for page in ordered:
            layout.find_rules(page, settings)
            layout.find_figures(page, settings)
            layout.classify_lines(page, body, settings)
        elements = layout.build_elements(chapter_pages, prefix)
        sections = layout.build_sections(elements, body, settings, prefix)
        relationships, orphans = layout.build_relationships(elements, settings)
        evidence = [item for page in chapter_pages for item in _evidence(page)]

        for page in chapter_pages:
            label = page.label
            if label["dictionary_error"]:
                defects.append(
                    {
                        "kind": "page-label",
                        "pdf_page_index": page.index,
                        "detail": f"Reading /PageLabels failed: {label['dictionary_error']}. "
                        f"The label comes from {label['method']}.",
                        "evidence": [],
                    }
                )
            elif label["method"] == "inferred-from-neighbour":
                defects.append(
                    {
                        "kind": "page-label",
                        "pdf_page_index": page.index,
                        "detail": label["detail"],
                        "evidence": [],
                    }
                )
            elif label["folio_in_text"] and label["value"] != label["folio_in_text"]:
                defects.append(
                    {
                        "kind": "page-label",
                        "pdf_page_index": page.index,
                        "detail": f"/PageLabels says {label['value']!r} but the folio says "
                        f"{label['folio_in_text']!r}.",
                        "evidence": [],
                    }
                )
            stored, geometric = layout.reading_order(page)
            if stored != geometric:
                defects.append(
                    {
                        "kind": "reading-order",
                        "pdf_page_index": page.index,
                        "detail": f"Stored block order {stored} differs from "
                        f"geometric order {geometric}.",
                        "evidence": [
                            ln.id
                            for ln in page.lines
                            if ln.kind != "furniture"
                            and stored.index(ln.block) != geometric.index(ln.block)
                        ],
                    }
                )
            defects.extend(layout.text_layer_defects(page))
            if not page.lines:
                defects.append(
                    {
                        "kind": "no-text-layer",
                        "pdf_page_index": page.index,
                        "detail": "The page has no text in its text layer.",
                        "evidence": [],
                    }
                )
        for caption in orphans:
            defects.append(
                {
                    "kind": "missing-figure",
                    "pdf_page_index": caption["pdf_page_index"],
                    "detail": f"Caption {caption.get('label')!r} has no figure region near it.",
                    "evidence": caption["evidence"],
                }
            )

        coverage: list[Json] = []
        for page in chapter_pages:
            ink = _ink_check(document, page, settings.check_dpi, settings)
            coverage.append({"pdf_page_index": page.index, **ink})
            for region in ink["uncovered_regions"]:
                defects.append(
                    {
                        "kind": "missing-figure",
                        "pdf_page_index": page.index,
                        "bbox": region["bbox"],
                        "detail": "Drawn ink here is not covered by any text line, "
                        "image or drawing. "
                        "It can be a figure or text the extractor missed.",
                        "evidence": [],
                    }
                )
            if ink["text_lines_without_ink"]:
                defects.append(
                    {
                        "kind": "invisible-text",
                        "pdf_page_index": page.index,
                        "detail": "Text-layer lines cover no drawn ink.",
                        "evidence": ink["text_lines_without_ink"],
                    }
                )

        boundary, boundary_defects = _boundary_check(entry, pages, outline, page_count)
        defects.extend(boundary_defects)
        renders: dict[str, bytes] = {}
        render_index: Json = {}

        def render(index: int, purpose: str) -> str:
            name = _render_name(index)
            if name not in renders:
                renders[name] = layout.render_png(document[index], None, settings.render_dpi)
                render_index[name] = {
                    "pdf_page_index": index,
                    "dpi": settings.render_dpi,
                    "purposes": [],
                }
            render_index[name]["purposes"].append(purpose)
            return name

        for index in boundary["rendered_pages"]:
            render(index, "chapter boundary")
        representative: list[Json] = []
        # Each failed check links to its defects. The IDs exist only after numbering.
        linked: list[tuple[Json, str, list[Json]]] = []
        for purpose, page in _representative_pages(chapter_pages):
            if page is None:
                representative.append(
                    {
                        "purpose": purpose,
                        "pdf_page_index": None,
                        "passed": True,
                        "note": f"The chapter has no {purpose.split()[0]} page.",
                    }
                )
                continue
            ink = _ink_check(document, page, settings.render_dpi, settings)
            passed = not ink["text_lines_without_ink"] and not ink["uncovered_regions"]
            check_entry: Json = {
                "purpose": purpose,
                "pdf_page_index": page.index,
                "printed_page_label": page.label["value"],
                "render": render(page.index, purpose),
                "comparison": ink,
                "passed": passed,
            }
            representative.append(check_entry)
            if not passed:
                defect = _render_mismatch(purpose, page.index, ink, settings.render_dpi)
                defects.append(defect)
                linked.append((check_entry, "defect", [defect]))
        anchors = _anchor_check(document, evidence)
        anchor_defects = _unresolved_anchor_defects(anchors, evidence)
        defects.extend(anchor_defects)
        linked.append((anchors, "defects", anchor_defects))

    for number, defect in enumerate(defects, start=1):
        defect["id"] = f"{prefix}d{number:03d}"
        if defect.get("pdf_page_index") is not None and "printed_page_label" not in defect:
            defect["printed_page_label"] = pages[defect["pdf_page_index"]].label["value"]
    for check_entry, name, found in linked:
        ids = [defect["id"] for defect in found]
        check_entry[name] = ids[0] if name == "defect" else ids
    record: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "coverage": {
            "scope": "one-chapter",
            "statement": f"This record covers chapter {chapter['number']} only, page indexes "
            f"{first} to {last}. It makes no claim about the rest of the book.",
        },
        "source": {
            "source_identity": entry["source_identity"],
            "title": entry["title"],
            "year": entry.get("year"),
            "attachment_item_key": entry["attachment_item_key"],
            "document": {
                "sha256": entry["document_file_sha256"],
                "location": str(path),
                "page_count": page_count,
            },
        },
        "chapter": {
            "number": chapter["number"],
            "title": chapter["title"],
            "pdf_page_index_first": first,
            "pdf_page_index_last": last,
            "printed_page_label_first": pages[first].label,
            "printed_page_label_last": pages[last].label,
            "selection_labels": {
                "first": chapter["printed_page_label_first"],
                "last": chapter["printed_page_label_last"],
            },
            "body_font_size": body,
        },
        "book_outline": outline,
        "pages": [_page_record(pages[i], first <= i <= last) for i in indexes],
        "evidence": evidence,
        "structure": {"sections": sections, "elements": elements},
        "relationships": relationships,
        "defects": defects,
        "checks": {
            "boundary": boundary,
            "representative_pages": representative,
            "anchor_resolution": anchors,
            "ink_coverage": coverage,
        },
        "renders": render_index,
        "method": {
            "extractor": "PyMuPDF",
            "pymupdf_version": str(pymupdf.VersionBind),  # pyright: ignore
            "mupdf_version": str(pymupdf.VersionFitz),  # pyright: ignore
            "text_flags": layout.TEXT_FLAGS,
            "code_version": code_version(__name__),
            "settings": asdict(settings),
            "page_index_base": 0,
            "coordinates": "PDF points, unrotated page space, origin at the top left",
            "known_limitations": KNOWN_LIMITATIONS,
        },
    }
    payload = {RECORD_FILE: _json_bytes(record), SUMMARY_FILE: summary_markdown(record).encode()}
    payload.update(renders)
    return record, payload


# Summary -----------------------------------------------------------------------------


def summary_markdown(record: Json) -> str:
    chapter = record["chapter"]
    source = record["source"]
    checks = record["checks"]
    counts: dict[str, int] = {}
    for element in record["structure"]["elements"]:
        counts[element["kind"]] = counts.get(element["kind"], 0) + 1
    defect_counts: dict[str, int] = {}
    for defect in record["defects"]:
        defect_counts[defect["kind"]] = defect_counts.get(defect["kind"], 0) + 1
    lines = [
        f"# Chapter extraction: {source['title']} ({source['year']})",
        "",
        record["coverage"]["statement"],
        "",
        f"- Chapter {chapter['number']}: {chapter['title']}",
        f"- Page indexes {chapter['pdf_page_index_first']} to {chapter['pdf_page_index_last']}, "
        f"printed labels {chapter['printed_page_label_first']['value']} to "
        f"{chapter['printed_page_label_last']['value']}",
        f"- Evidence lines: {len(record['evidence'])}",
        f"- Sections: {len(record['structure']['sections'])}",
        "- Elements: " + ", ".join(f"{k} {v}" for k, v in sorted(counts.items())),
        f"- Relationships: {len(record['relationships'])}",
        "",
        "## Pages",
        "",
        "| Page index | Printed label | Label method |",
        "|---|---|---|",
    ]
    for page in record["pages"]:
        if page["in_chapter"]:
            label = page["printed_page_label"]
            lines.append(f"| {page['pdf_page_index']} | {label['value']} | {label['method']} |")
    lines += ["", "## Checks", ""]
    boundary = checks["boundary"]
    lines.append(f"- Boundary check: {'passed' if boundary['passed'] else 'FAILED'}")
    for assertion in boundary["assertions"]:
        mark = "passed" if assertion["passed"] else "FAILED"
        lines.append(f"  - {assertion['name']}: {mark}")
    for page in checks["representative_pages"]:
        if page["pdf_page_index"] is None:
            lines.append(f"- {page['purpose'].capitalize()}: {page['note']}")
            continue
        comparison = page["comparison"]
        lines.append(
            f"- {page['purpose'].capitalize()} (index {page['pdf_page_index']}): "
            f"{'passed' if page['passed'] else 'FAILED'}, "
            f"{comparison['text_lines_with_ink']} of {comparison['text_lines']} lines on ink, "
            f"{len(comparison['uncovered_regions'])} uncovered ink regions, render {page['render']}"
        )
    anchors = checks["anchor_resolution"]
    lines.append(
        f"- Anchor resolution: {anchors['resolved']} of {anchors['evidence_items']} resolved"
    )
    lines += ["", "## Defects", ""]
    if not defect_counts:
        lines.append("The extractor found no defects.")
    for kind, count in sorted(defect_counts.items()):
        lines.append(f"- {kind}: {count}")
    lines += ["", "## Known limitations", ""]
    lines += [f"- {item}" for item in record["method"]["known_limitations"]]
    return "\n".join(lines) + "\n"


# Run -----------------------------------------------------------------------------


def _extract_book(
    store: ArtifactStore, manifest: Json, entry: Json, settings: Settings
) -> tuple[StoredArtifact, bool]:
    path = resolve_document(manifest, entry)
    if not path.is_file():
        raise ExtractionError(f"the document file {path} does not exist")
    actual = sha256_file(path)
    if actual != entry["document_file_sha256"]:
        raise ExtractionError(
            f"{path} has SHA-256 {actual}, the selection names {entry['document_file_sha256']}"
        )
    dependency = book_dependency(entry, settings)
    found = store.reuse(dependency)
    if found is not None:
        return found, True
    _record, payload = extract_chapter(path, entry, settings)
    return store.write(dependency, payload), False


def run_extraction(
    selection_path: Path,
    store: ArtifactStore,
    config: ExtractionConfig | None = None,
    only: set[str] | None = None,
) -> ExtractionRun:
    """Extract every selected chapter. Record each book's success or failure."""
    config = config or ExtractionConfig()
    selection = load_selection(selection_path)
    manifest_key, manifest = load_manifest(store)
    outcomes: list[BookOutcome] = []
    for entry in selection["selected"]:
        item_key = entry["source_identity"]["item_key"]
        if only is not None and item_key not in only:
            continue
        try:
            artifact, reused = _extract_book(store, manifest, entry, config.for_book(item_key))
            outcomes.append(BookOutcome(item_key, entry["title"], artifact, reused, None))
        except Exception as error:  # noqa: BLE001 - one book's failure must not stop the others
            message = f"{type(error).__name__}: {error}"
            outcomes.append(BookOutcome(item_key, entry["title"], None, False, message))
    report = _write_report(store, selection_path, manifest_key, config, outcomes)
    return ExtractionRun(outcomes, report)


def _write_report(
    store: ArtifactStore,
    selection_path: Path,
    manifest_key: str,
    config: ExtractionConfig,
    outcomes: list[BookOutcome],
) -> StoredArtifact:
    results: list[Json] = [
        {
            "item_key": o.item_key,
            "title": o.title,
            "status": "failed" if o.error else "extracted",
            "artifact_key": o.artifact.record.key if o.artifact else None,
            "error": o.error,
        }
        for o in outcomes
    ]
    dependency = DependencyRecord(
        stage=STAGE,
        artifact=RUN_REPORT,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={
            "settings": asdict(config.settings),
            "book_overrides": config.book_overrides,
            "results": results,
        },
        inputs=[
            InputIdentity(
                role="chapter-selection",
                kind="file",
                locator=selection_path.name,
                identity=sha256_file(selection_path),
            ),
            InputIdentity(
                role="source-manifest",
                kind="artifact",
                locator=f"{SOURCE_MANIFEST}/{manifest_key}",
                identity=manifest_key,
            ),
        ],
    )
    found = store.reuse(dependency)
    if found is not None:
        return found
    report: Json = {"schema_version": SCHEMA_VERSION, "stage": STAGE, "results": results}
    lines = ["# Chapter extraction run", ""]
    for result in results:
        if result["error"]:
            lines.append(f"- {result['item_key']} ({result['title']}): failed. {result['error']}")
        else:
            lines.append(
                f"- {result['item_key']} ({result['title']}): artifact {result['artifact_key']}"
            )
    return store.write(
        dependency,
        {"report.json": _json_bytes(report), SUMMARY_FILE: ("\n".join(lines) + "\n").encode()},
    )


# Anchors -----------------------------------------------------------------------------


def find_evidence(record: Json, evidence_id: str) -> Json:
    for item in record["evidence"]:
        if item["id"] == evidence_id:
            return item
    for element in record["structure"]["elements"]:
        if element["id"] == evidence_id:
            return element
    raise ExtractionError(f"no evidence item or element {evidence_id} in this extraction")


def _holds(path: Path, sha256: str) -> bool:
    return path.is_file() and sha256_file(path) == sha256


def locate_document(record: Json, store: ArtifactStore | None = None) -> Path:
    """Return a file that holds the document of an extraction record.

    The record keeps the location where the extraction read the file. A run reuses the
    record after the file moves, because the artifact key names the content and not the
    location. When the recorded location does not hold the same content, the latest source
    manifest in the store gives the current location.
    """
    source = record["source"]
    document = source["document"]
    recorded = Path(document["location"])
    if _holds(recorded, document["sha256"]):
        return recorded
    if store is None:
        raise ExtractionError(f"{recorded} is missing or changed since the extraction ran")
    _key, manifest = load_manifest(store)
    entry = {
        "source_identity": source["source_identity"],
        "attachment_item_key": source["attachment_item_key"],
        "document_file_sha256": document["sha256"],
    }
    current = resolve_document(manifest, entry)
    if not _holds(current, document["sha256"]):
        raise ExtractionError(
            f"neither the recorded location {recorded} nor the manifest location {current} "
            f"holds the document with SHA-256 {document['sha256']}"
        )
    return current


def render_anchor(
    artifact: StoredArtifact,
    evidence_id: str,
    dpi: int = 150,
    margin: float = 4.0,
    store: ArtifactStore | None = None,
) -> bytes:
    """Render the page region of one evidence item or element as a PNG.

    With a store, the render finds a moved document through the latest source manifest.
    """
    record: Json = json.loads(artifact.read_bytes(RECORD_FILE))
    item = find_evidence(record, evidence_id)
    path = locate_document(record, store)
    box: Box = layout.inflate(tuple(item["bbox"]), margin)
    with layout.open_pdf(path) as pdf:
        return layout.render_png(pdf[item["pdf_page_index"]], box, dpi)


def latest_extraction(store: ArtifactStore, item_key: str) -> StoredArtifact:
    """Return the newest extraction of a book that the latest run report names."""
    key = store.latest_key(RUN_REPORT)
    if key is None:
        raise ExtractionError("no chapter-extraction run is in the store")
    report: Json = json.loads(store.load(RUN_REPORT, key).read_bytes("report.json"))
    for result in report["results"]:
        if result["item_key"] == item_key and result["artifact_key"]:
            return store.load(EXTRACTION, result["artifact_key"])
    raise ExtractionError(f"the latest chapter-extraction run has no extraction of {item_key}")
