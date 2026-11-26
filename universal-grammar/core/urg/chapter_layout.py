"""Page analysis for the chapter-extraction stage.

The functions here read the pages of one chapter through PyMuPDF. They return
evidence lines, page records, structure elements, relationships and defects as
plain JSON values. The stage module `urg.extraction` runs them and stores the
result.
"""

from __future__ import annotations

import re
import statistics
import unicodedata
from collections import Counter
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import pymupdf  # pyright: ignore[reportMissingTypeStubs]

Json = dict[str, Any]
Box = tuple[float, float, float, float]

# The "dict" defaults without image bytes: image blocks appear only with TEXT_PRESERVE_IMAGES.
# https://pymupdf.readthedocs.io/en/latest/textpage.html
TEXT_FLAGS: int = pymupdf.TEXTFLAGS_DICT & ~pymupdf.TEXT_PRESERVE_IMAGES  # pyright: ignore
# Span flags: bit 3 is monospaced, bit 4 is bold.
# https://pymupdf.readthedocs.io/en/latest/textpage.html
MONOSPACE_FLAG = 8
BOLD_FLAG = 16
MONOSPACE_FONT = re.compile(r"mono|courier|code|cmtt|consol|menlo|inconsolata|typewriter", re.I)
FOLIO = re.compile(r"\d{1,4}|[ivxlcdm]{1,7}", re.I)
CAPTION = re.compile(
    r"(Figure|Fig\.|Table|Example|Listing)\s+([0-9A-Z]+(?:[-.][0-9]+)*)[.:]?(\s|$)"
)
CHAPTER_PREFIX = re.compile(r"^(chapter\s+)?\d+[.:]?\s+", re.I)
ROMAN_TOKEN = re.compile(r"\b[ivxlcdm]{1,7}\b")
TERMINAL = re.compile(r"[.!?:;)\"”’]$")

LINE_KIND_TO_ELEMENT = {
    "body": "paragraph",
    "heading": "heading",
    "code": "code_block",
    "caption": "caption",
}


@dataclass(frozen=True)
class Settings:
    """The tunable values of the extraction, one set per book."""

    margin_band: float = 0.1
    heading_size_ratio: float = 1.12
    code_monospace_fraction: float = 0.8
    figure_min_area: float = 0.015
    drawing_cluster_gap: float = 4.0
    box_text_coverage: float = 0.35
    rule_gap: float = 14.0
    caption_gap: float = 60.0
    ink_threshold: int = 160
    cell_size: float = 8.0
    uncovered_min_cells: int = 6
    min_contrast: int = 60
    check_dpi: int = 72
    render_dpi: int = 110


@dataclass
class Line:
    """One text line of the text layer, with its anchor on the page."""

    id: str
    page: int
    block: int
    number: int
    bbox: Box
    text: str
    span_texts: list[str]
    size: float
    monospace: float
    bold: float
    fonts: list[str]
    char_width: float
    kind: str = ""
    span_boxes: list[Box] = field(default_factory=list[Box])


@dataclass
class PageData:
    """What the extractor read from one page."""

    index: int
    width: float
    height: float
    lines: list[Line]
    block_boxes: dict[int, Box]
    images: list[Box]
    drawings: list[Box]
    bars: list[Box]
    folio: str | None
    frames: list[Box] = field(default_factory=list[Box])
    label: Json = field(default_factory=dict[str, Any])
    figures: list[Json] = field(default_factory=list[Json])
    boxes: list[Json] = field(default_factory=list[Json])
    rules: list[Json] = field(default_factory=list[Json])


def open_pdf(path: Path) -> Any:
    pymupdf.TOOLS.mupdf_display_errors(False)  # pyright: ignore
    return pymupdf.open(path)  # pyright: ignore


def to_rect(box: Box) -> Any:
    return pymupdf.Rect(*box)  # pyright: ignore


def norm(text: str) -> str:
    """Return text in NFKC, case-folded, with collapsed white space and no soft hyphens."""
    text = unicodedata.normalize("NFKC", text).replace("­", "")
    return " ".join(text.casefold().split())


def title_key(title: str) -> str:
    """Return a normalised title without a leading chapter number."""
    return CHAPTER_PREFIX.sub("", norm(title)).strip()


def rounded(box: Any) -> Box:
    x0, y0, x1, y1 = (float(v) for v in box)
    return (round(x0, 2), round(y0, 2), round(x1, 2), round(y1, 2))


def area(box: Box) -> float:
    return max(0.0, box[2] - box[0]) * max(0.0, box[3] - box[1])


def union(a: Box, b: Box) -> Box:
    return (min(a[0], b[0]), min(a[1], b[1]), max(a[2], b[2]), max(a[3], b[3]))


def inflate(box: Box, by: float) -> Box:
    return (box[0] - by, box[1] - by, box[2] + by, box[3] + by)


def intersects(a: Box, b: Box) -> bool:
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]


def centre_inside(inner: Box, outer: Box) -> bool:
    cx, cy = (inner[0] + inner[2]) / 2, (inner[1] + inner[3]) / 2
    return outer[0] <= cx <= outer[2] and outer[1] <= cy <= outer[3]


def bounding(boxes: list[Box]) -> Box:
    result = boxes[0]
    for box in boxes[1:]:
        result = union(result, box)
    return result


# Reading pages -----------------------------------------------------------------------------


def _is_monospace(span: Json) -> bool:
    return bool(span["flags"] & MONOSPACE_FLAG) or bool(MONOSPACE_FONT.search(span["font"]))


def _line_from_spans(prefix: str, index: int, block: int, number: int, line: Json) -> Line | None:
    spans: list[Json] = line["spans"]
    text = "".join(span["text"] for span in spans)
    if not text.strip():
        return None
    weights = [len(span["text"].strip()) for span in spans]
    total = sum(weights) or 1
    mono = sum(w for w, s in zip(weights, spans, strict=True) if _is_monospace(s)) / total
    bold = sum(w for w, s in zip(weights, spans, strict=True) if s["flags"] & BOLD_FLAG) / total
    size = sum(w * float(s["size"]) for w, s in zip(weights, spans, strict=True)) / total
    bbox = rounded(line["bbox"])
    width = bbox[2] - bbox[0]
    return Line(
        id=f"{prefix}p{index:04d}-b{block:03d}-l{number:02d}",
        page=index,
        block=block,
        number=number,
        bbox=bbox,
        text=text,
        span_texts=[span["text"] for span in spans],
        size=round(size, 2),
        monospace=round(mono, 3),
        bold=round(bold, 3),
        fonts=sorted({span["font"] for span in spans}),
        char_width=round(width / len(text), 3) if text else 0.0,
        span_boxes=[rounded(span["bbox"]) for span in spans],
    )


def _bars(drawings: list[Json], width: float, height: float, band: float) -> list[Box]:
    """Return thin horizontal strokes that could be inference-rule bars."""
    bars: list[Box] = []
    for drawing in drawings:
        if (drawing.get("width") or 0.0) > 1.5:
            continue  # A thick stroke is a decorative rule, not an inference bar.
        for item in drawing["items"]:
            box: Box | None = None
            if item[0] == "l" and abs(item[1].y - item[2].y) < 0.6:
                box = rounded(
                    (
                        min(item[1].x, item[2].x),
                        item[1].y - 0.3,
                        max(item[1].x, item[2].x),
                        item[1].y + 0.3,
                    )
                )
            elif item[0] == "re" and item[1].height < 1.4:
                box = rounded(item[1])
            if box is None:
                continue
            length = box[2] - box[0]
            if not 12 <= length <= 0.7 * width:
                continue
            # The top band holds running-head rules. The bottom band is kept, because a
            # rule can sit near the foot of a page. A footnote rule there fails the
            # conclusion test in find_rules, since the note under it is wider than the bar.
            if box[3] < height * band:
                continue
            bars.append(box)
    return bars


def read_page(page: Any, index: int, prefix: str, settings: Settings) -> PageData:
    width, height = float(page.rect.width), float(page.rect.height)
    data: Json = page.get_text("dict", flags=TEXT_FLAGS)
    lines: list[Line] = []
    block_boxes: dict[int, Box] = {}
    for block in data["blocks"]:
        if block["type"] != 0:
            continue
        number = int(block["number"])
        block_boxes[number] = rounded(block["bbox"])
        for line_number, line in enumerate(block["lines"]):
            parsed = _line_from_spans(prefix, index, number, line_number, line)
            if parsed is not None:
                lines.append(parsed)
    images: list[Box] = []
    page_area = width * height
    for info in page.get_image_info():
        box = rounded(info["bbox"])
        if page_area * 0.002 <= area(box) <= page_area * 0.9:
            images.append(box)
    drawings: list[Json] = page.get_drawings()
    bars = _bars(drawings, width, height, settings.margin_band)
    drawing_boxes: list[Box] = []
    frames: list[Box] = []
    for drawing in drawings:
        # The rect of a stroked path excludes the stroke, so a thick rule has zero height.
        box = rounded(inflate(tuple(drawing["rect"]), (drawing.get("width") or 0.0) / 2))
        if area(box) < page_area * 0.8 and (box[2] > box[0] or box[3] > box[1]):
            drawing_boxes.append(box)
            rectangles = all(item[0] in ("re", "qu") for item in drawing["items"])
            background = drawing.get("fill") is not None and drawing.get("color") is None
            if rectangles or background:
                frames.append(box)
    page_data = PageData(
        index,
        width,
        height,
        lines,
        block_boxes,
        images,
        drawing_boxes,
        bars,
        folio=None,
        frames=frames,
    )
    page_data.folio = _folio(page_data, settings.margin_band)
    return page_data


def _in_band(line: Line, page: PageData, band: float) -> bool:
    return line.bbox[3] <= page.height * band or line.bbox[1] >= page.height * (1 - band)


def _folio(page: PageData, band: float) -> str | None:
    for line in page.lines:
        if not _in_band(line, page, band):
            continue
        for text in line.span_texts:
            if FOLIO.fullmatch(text.strip()):
                return text.strip()
    return None


def page_label(page: Any) -> tuple[str | None, str | None]:
    """Return the /PageLabels label of a page, or the error that reading it raised."""
    try:
        return str(page.get_label()), None
    except Exception as error:  # noqa: BLE001 - malformed label trees raise assorted errors
        return None, f"{type(error).__name__}: {error}"


# Page labels -----------------------------------------------------------------------------


def assign_labels(
    pages: list[PageData], dictionary: dict[int, tuple[str | None, str | None]]
) -> None:
    """Give each page a printed label and record how the label was found.

    The label never comes from the page index alone. A page without a /PageLabels
    label and without a folio gets a label inferred from a neighbour's folio, and the
    record names that neighbour.
    """
    for page in pages:
        label, error = dictionary.get(page.index, (None, None))
        record: Json = {
            "value": None,
            "method": "unknown",
            "detail": "",
            "dictionary_label": label or None,
            "dictionary_error": error,
            "folio_in_text": page.folio,
        }
        if label:
            record["value"] = label
            record["method"] = "page-labels-dictionary"
            if page.folio is None:
                record["detail"] = "No folio in the page text confirms the label."
            elif page.folio == label:
                record["detail"] = "The folio in the page text agrees."
            else:
                record["detail"] = f"The folio in the page text is {page.folio!r}."
        elif page.folio is not None:
            record["value"] = page.folio
            record["method"] = "folio-in-page-text"
            record["detail"] = "Read from a page-number span in the header or footer band."
        page.label = record
    numbered = [p for p in pages if p.label["value"] and str(p.label["value"]).isdigit()]
    for page in pages:
        if page.label["value"] is not None or not numbered:
            continue
        nearest = min(numbered, key=lambda p: abs(p.index - page.index))
        offset = page.index - nearest.index
        page.label["value"] = str(int(str(nearest.label["value"])) + offset)
        page.label["method"] = "inferred-from-neighbour"
        page.label["detail"] = (
            f"Page index {nearest.index} has label {nearest.label['value']} "
            f"({nearest.label['method']}), so this page is {offset:+d} from it."
        )


# Classification -----------------------------------------------------------------------------


def mark_furniture(pages: list[PageData], band: float) -> None:
    """Mark running heads and folios, which repeat in the margin bands."""

    def key(line: Line) -> str:
        return ROMAN_TOKEN.sub("", re.sub(r"\d+", "", norm(line.text))).strip()

    seen: Counter[str] = Counter()
    for page in pages:
        keys = {key(line) for line in page.lines if _in_band(line, page, band)}
        seen.update(k for k in keys if k)
    for page in pages:
        for line in page.lines:
            if not _in_band(line, page, band):
                continue
            stripped = line.text.strip()
            tokens = stripped.split()
            label = page.label.get("value") if page.label else None
            # A running head or foot carries the page's own printed label at one end.
            carries_label = bool(label) and bool(tokens) and label in (tokens[0], tokens[-1])
            if FOLIO.fullmatch(stripped) or carries_label or (key(line) and seen[key(line)] >= 2):
                line.kind = "furniture"


def body_size(pages: list[PageData]) -> float:
    sizes: Counter[float] = Counter()
    for page in pages:
        for line in page.lines:
            if line.kind != "furniture" and line.monospace < 0.5:
                sizes[round(line.size * 2) / 2] += len(line.text.strip())
    return sizes.most_common(1)[0][0] if sizes else 10.0


def find_rules(page: PageData, settings: Settings) -> None:
    """Group the lines around a thin bar into an inference rule.

    Three or more bars with the same ends are table rules. They form a table box.
    """
    extents: dict[tuple[int, int], list[Box]] = {}
    for bar in page.bars:
        extents.setdefault((round(bar[0]), round(bar[2])), []).append(bar)
    table_bars = [b for group in extents.values() if len(group) >= 3 for b in group]
    if table_bars:
        for box in _clusters(table_bars, settings.rule_gap * 2):
            page.boxes.append(
                {
                    "bbox": box,
                    "kind": "table",
                    "text_coverage": None,
                    "reason": "three or more rules with the same ends",
                }
            )
    for bar in page.bars:
        if bar in table_bars:
            continue
        length = bar[2] - bar[0]
        reach = (bar[0] - 10, bar[2] + 10)
        candidates = [ln for ln in page.lines if ln.kind == ""]

        def overlaps(ln: Line, reach: tuple[float, float] = reach) -> bool:
            return ln.bbox[0] < reach[1] and ln.bbox[2] > reach[0]

        conclusion = [
            ln
            for ln in candidates
            if overlaps(ln) and bar[3] - 1 <= ln.bbox[1] <= bar[3] + settings.rule_gap
        ]
        spread = [_extent_before(ln, bar[2] + 5) for ln in conclusion]
        if not conclusion or any(e[2] - e[0] > length + 20 for e in spread):
            continue  # A conclusion is no wider than its bar. A paragraph under a rule is.
        centre = sum((e[0] + e[2]) / 2 for e in spread) / len(spread)
        if abs(centre - (bar[0] + bar[2]) / 2) > 0.25 * length + 6:
            continue
        premises = [
            ln
            for ln in candidates
            if overlaps(ln) and bar[1] - settings.rule_gap <= ln.bbox[3] <= bar[1] + 1
        ]
        labels = [
            ln
            for ln in candidates
            if ln not in conclusion
            and ln not in premises
            and ln.bbox[0] >= bar[2] + 5
            and bar[1] - 10 <= (ln.bbox[1] + ln.bbox[3]) / 2 <= bar[3] + 12
        ]
        for ln in conclusion + premises + labels:
            ln.kind = "rule-text"
        # The text layer can join a conclusion and its label into one line.
        inline_labels = [
            {"line": ln.id, "text": text}
            for ln in conclusion + premises
            if (text := _text_from(ln, bar[2] + 5).strip())
        ]
        page.rules.append(
            {
                "bar": bar,
                "premises": [ln.id for ln in premises],
                "conclusion": [ln.id for ln in conclusion],
                "label": [ln.id for ln in labels],
                "label_in_line": inline_labels,
                "bbox": bounding([bar] + [ln.bbox for ln in conclusion + premises + labels]),
            }
        )


def _extent_before(line: Line, x: float) -> Box:
    """Return the box of the spans of a line that start left of ``x``."""
    boxes = [b for b in line.span_boxes if b[0] < x] or [line.bbox]
    return bounding(boxes)


def _text_from(line: Line, x: float) -> str:
    """Return the text of the spans of a line that start at or right of ``x``."""
    if len(line.span_boxes) != len(line.span_texts):
        return ""
    pairs = zip(line.span_boxes, line.span_texts, strict=True)
    return "".join(t for b, t in pairs if b[0] >= x)


def _clusters(boxes: list[Box], gap: float) -> list[Box]:
    order = sorted(range(len(boxes)), key=lambda i: boxes[i][1])
    parent = list(range(len(boxes)))

    def find(i: int) -> int:
        while parent[i] != i:
            parent[i] = parent[parent[i]]
            i = parent[i]
        return i

    for position, i in enumerate(order):
        grown = inflate(boxes[i], gap)
        for j in order[position + 1 :]:
            if boxes[j][1] > grown[3]:
                break
            if intersects(grown, boxes[j]):
                parent[find(j)] = find(i)
    groups: dict[int, Box] = {}
    for i, box in enumerate(boxes):
        root = find(i)
        groups[root] = union(groups[root], box) if root in groups else box
    return list(groups.values())


def find_figures(page: PageData, settings: Settings) -> None:
    """Find image and vector-drawing regions, and set shaded text boxes apart from figures."""
    page_area = page.width * page.height
    bar_set = set(page.bars)
    strokes = [b for b in page.drawings if b not in bar_set]
    regions: list[tuple[Box, str]] = []
    for box in _clusters(strokes, settings.drawing_cluster_gap):
        if min(box[2] - box[0], box[3] - box[1]) < 3:
            continue
        if area(box) >= settings.figure_min_area * page_area:
            regions.append((box, "vector"))
    for box in page.images:
        if area(box) >= settings.figure_min_area * page_area / 3:
            regions.append((box, "image"))
    merged: list[tuple[Box, set[str]]] = []
    for box, kind in sorted(regions):
        for i, (other, kinds) in enumerate(merged):
            if intersects(box, other):
                merged[i] = (union(box, other), kinds | {kind})
                break
        else:
            merged.append((box, {kind}))
    for box, kinds in merged:
        inside = [ln for ln in page.lines if ln.kind == "" and centre_inside(ln.bbox, box)]
        coverage = sum(area(ln.bbox) for ln in inside) / max(area(box), 1.0)
        mono = sum(ln.monospace for ln in inside) / len(inside) if inside else 0.0
        members = [d for d in strokes if intersects(d, box)]
        only_frames = bool(members) and all(d in page.frames for d in members)
        texty = len(inside) >= 3 and (mono >= 0.7 or coverage >= settings.box_text_coverage)
        if "image" not in kinds and inside and (only_frames or texty):
            page.boxes.append(
                {
                    "bbox": box,
                    "kind": "code-box" if mono >= 0.7 else "text-box",
                    "text_coverage": round(coverage, 3),
                    "reason": "only filled or stroked rectangles around text"
                    if only_frames
                    else "text covers much of the region",
                }
            )
            continue
        for ln in inside:
            ln.kind = "figure-text"
        page.figures.append(
            {
                "bbox": box,
                "figure_kind": "mixed" if len(kinds) > 1 else next(iter(kinds)),
                "label_evidence": [ln.id for ln in inside],
            }
        )


def classify_lines(page: PageData, body: float, settings: Settings) -> None:
    blocks: dict[int, list[Line]] = {}
    for line in page.lines:
        blocks.setdefault(line.block, []).append(line)
    for block_lines in blocks.values():
        open_lines = [ln for ln in block_lines if ln.kind == ""]
        if not open_lines:
            continue
        caption = CAPTION.match(open_lines[0].text.strip()) is not None
        all_bold = all(ln.bold >= 0.9 for ln in open_lines)
        for line in open_lines:
            stripped = line.text.strip()
            if line.monospace >= settings.code_monospace_fraction:
                line.kind = "code"
            elif caption:
                line.kind = "caption"
            elif line.size >= body * settings.heading_size_ratio:
                line.kind = "heading"
            elif (
                all_bold
                and len(open_lines) <= 2
                and len(stripped) <= 80
                and line.size >= body - 0.5
            ):
                line.kind = "heading"
            else:
                line.kind = "body"


# Elements -----------------------------------------------------------------------------


def _code_text(lines: list[Line]) -> tuple[str, list[Json]]:
    widths = [ln.char_width for ln in lines if len(ln.text.strip()) >= 4]
    char_width = statistics.median(widths) if widths else 0.0
    left = min(ln.bbox[0] for ln in lines)
    rows: list[str] = []
    detail: list[Json] = []
    for ln in lines:
        indent = round((ln.bbox[0] - left) / char_width) if char_width > 0 else 0
        has_spaces = ln.text.startswith(" ")
        rows.append(ln.text if has_spaces else " " * indent + ln.text)
        detail.append({"evidence": ln.id, "indent_columns": 0 if has_spaces else indent})
    return "\n".join(rows), detail


def _same_run(kind: str, previous: list[Line], line: Line) -> bool:
    last = previous[-1]
    if line.block == last.block:
        return True
    gap = line.bbox[1] - last.bbox[3]
    if kind == "code_block":
        return -1 <= gap <= 1.2 * max(last.size, line.size)
    if kind == "heading":
        return -1 <= gap <= 1.5 * max(last.size, line.size) and abs(line.size - last.size) < 0.6
    return False


def build_elements(pages: list[PageData], prefix: str) -> list[Json]:
    """Return the chapter's structure elements in reading order, without section links."""
    elements: list[Json] = []
    for page in pages:
        page_elements: list[Json] = []
        runs: list[tuple[str, list[Line]]] = []
        for line in page.lines:
            kind = LINE_KIND_TO_ELEMENT.get(line.kind)
            if kind is None:
                continue
            if runs and runs[-1][0] == kind and _same_run(kind, runs[-1][1], line):
                runs[-1][1].append(line)
            else:
                runs.append((kind, [line]))
        for kind, lines in runs:
            element: Json = {
                "kind": kind,
                "pdf_page_index": page.index,
                "printed_page_label": page.label["value"],
                "bbox": bounding([ln.bbox for ln in lines]),
                "evidence": [ln.id for ln in lines],
            }
            if kind == "code_block":
                element["text"], element["lines"] = _code_text(lines)
            elif kind == "paragraph":
                element["text"] = " ".join(ln.text.strip() for ln in lines)
            else:
                element["text"] = " ".join(ln.text.strip() for ln in lines)
                if kind == "heading":
                    element["font_size"] = max(ln.size for ln in lines)
                    element["bold"] = all(ln.bold >= 0.9 for ln in lines)
            page_elements.append(element)
        placed: list[Json] = []
        for figure in page.figures:
            placed.append(
                {
                    "kind": "figure",
                    "pdf_page_index": page.index,
                    "printed_page_label": page.label["value"],
                    "bbox": figure["bbox"],
                    "figure_kind": figure["figure_kind"],
                    "evidence": figure["label_evidence"],
                }
            )
        for rule in page.rules:
            placed.append(
                {
                    "kind": "inference_rule",
                    "pdf_page_index": page.index,
                    "printed_page_label": page.label["value"],
                    "bbox": rule["bbox"],
                    "bar": rule["bar"],
                    "premises": rule["premises"],
                    "conclusion": rule["conclusion"],
                    "label": rule["label"],
                    "label_in_line": rule["label_in_line"],
                    "evidence": rule["premises"] + rule["conclusion"] + rule["label"],
                }
            )
        for item in sorted(placed, key=lambda e: (e["bbox"][1], e["bbox"][0])):
            position = next(
                (
                    i
                    for i, e in enumerate(page_elements)
                    if e["kind"] not in ("figure", "inference_rule")
                    and e["bbox"][1] >= item["bbox"][1]
                ),
                len(page_elements),
            )
            page_elements.insert(position, item)
        elements.extend(page_elements)
    for number, element in enumerate(elements, start=1):
        element["id"] = f"{prefix}e{number:04d}"
    return elements


def build_sections(
    elements: list[Json], body: float, settings: Settings, prefix: str
) -> list[Json]:
    """Give headings levels by font size, then link each element to its section."""
    sizes = sorted(
        {
            round(e["font_size"] * 2) / 2
            for e in elements
            if e["kind"] == "heading" and e["font_size"] >= body * settings.heading_size_ratio
        },
        reverse=True,
    )
    for element in elements:
        if element["kind"] != "heading":
            continue
        size = round(element["font_size"] * 2) / 2
        element["level"] = sizes.index(size) + 1 if size in sizes else len(sizes) + 1
    sections: list[Json] = []
    stack: list[Json] = []
    for element in elements:
        if element["kind"] == "heading":
            while stack and stack[-1]["level"] >= element["level"]:
                stack.pop()
            section: Json = {
                "id": f"{prefix}s{len(sections) + 1:03d}",
                "title": element["text"],
                "level": element["level"],
                "heading": element["id"],
                "parent": stack[-1]["id"] if stack else None,
                "pdf_page_index": element["pdf_page_index"],
                "elements": [],
            }
            sections.append(section)
            stack.append(section)
        element["section"] = stack[-1]["id"] if stack else None
        if stack:
            stack[-1]["elements"].append(element["id"])
    return sections


# Relationships -----------------------------------------------------------------------------


def build_relationships(elements: list[Json], settings: Settings) -> tuple[list[Json], list[Json]]:
    """Return relationships and the captions that found no figure."""
    relationships: list[Json] = []
    orphan_captions: list[Json] = []
    figures = [e for e in elements if e["kind"] == "figure"]
    taken: set[str] = set()
    labels: dict[str, str] = {}
    for index, caption in enumerate(elements):
        if caption["kind"] != "caption":
            continue
        match = CAPTION.match(caption["text"])
        label = f"{match.group(1)} {match.group(2)}" if match else caption["text"][:20]
        caption["label"] = label
        if label.startswith(("Example", "Listing")):
            target = next(
                (e for e in elements[index + 1 : index + 3] if e["kind"] == "code_block"), None
            )
            if target is not None:
                labels[norm(label)] = target["id"]
                relationships.append(
                    _relation(
                        "captions-listing", caption, target, "the next code block after the caption"
                    )
                )
                continue
        best: tuple[float, Json] | None = None
        for figure in figures:
            if figure["pdf_page_index"] != caption["pdf_page_index"] or figure["id"] in taken:
                continue
            below = caption["bbox"][1] - figure["bbox"][3]
            above = figure["bbox"][1] - caption["bbox"][3]
            gap = min(g for g in (below, above) if g >= -5) if max(below, above) >= -5 else None
            if gap is not None and gap <= settings.caption_gap and (best is None or gap < best[0]):
                best = (gap, figure)
        if best is None:
            orphan_captions.append(caption)
            continue
        taken.add(best[1]["id"])
        labels[norm(label)] = best[1]["id"]
        relationships.append(
            _relation(
                "captions-figure",
                caption,
                best[1],
                f"nearest figure on the page, {best[0]:.1f} pt away",
            )
        )
    by_id = {e["id"]: e for e in elements}
    for index, element in enumerate(elements):
        if element["kind"] != "paragraph":
            continue
        text = norm(element["text"])
        for label, target in labels.items():
            if re.search(rf"\b{re.escape(label)}\b", text):
                relationships.append(
                    _relation("cites", element, by_id[target], f"the paragraph names {label!r}")
                )
        following = elements[index + 1] if index + 1 < len(elements) else None
        if (
            following
            and following["kind"] == "code_block"
            and element["text"].rstrip().endswith(":")
        ):
            relationships.append(
                _relation(
                    "introduces-listing",
                    element,
                    following,
                    "the paragraph ends with a colon before the code block",
                )
            )
    paragraphs = [e for e in elements if e["kind"] == "paragraph"]
    for first, second in zip(paragraphs, paragraphs[1:], strict=False):
        if second["pdf_page_index"] != first["pdf_page_index"] + 1:
            continue
        last_on_page = [
            e
            for e in elements
            if e["pdf_page_index"] == first["pdf_page_index"] and e["kind"] == "paragraph"
        ][-1]
        first_on_next = next(
            e
            for e in elements
            if e["pdf_page_index"] == second["pdf_page_index"] and e["kind"] == "paragraph"
        )
        if first is not last_on_page or second is not first_on_next:
            continue
        starts_lower = second["text"][:1].islower()
        if not TERMINAL.search(first["text"].rstrip()) and starts_lower:
            relationships.append(
                _relation(
                    "continues-on-next-page",
                    first,
                    second,
                    "no terminal punctuation, next page starts in lower case",
                )
            )
    return relationships, orphan_captions


def _relation(kind: str, source: Json, target: Json, method: str) -> Json:
    return {"kind": kind, "source": source["id"], "target": target["id"], "method": method}


# Ink comparison -----------------------------------------------------------------------------


@dataclass(frozen=True)
class InkMap:
    """A grey-scale render of a page, used to compare drawn ink with the evidence."""

    width: int
    height: int
    stride: int
    samples: bytes
    scale: float
    table: bytes

    def _rows(self, box: Box) -> list[bytes]:
        x0 = max(0, int(box[0] * self.scale))
        x1 = min(self.width, int(box[2] * self.scale + 0.999))
        y0 = max(0, int(box[1] * self.scale))
        y1 = min(self.height, int(box[3] * self.scale + 0.999))
        if x1 <= x0:
            return []
        return [self.samples[y * self.stride + x0 : y * self.stride + x1] for y in range(y0, y1)]

    def ink(self, box: Box) -> int:
        """Count the pixels darker than the threshold."""
        return sum(sum(row.translate(self.table)) for row in self._rows(box))

    def contrast(self, box: Box) -> int:
        """Return the grey-level range, which also sees light text on a dark fill."""
        rows = self._rows(box)
        if not rows:
            return 0
        return max(max(row) for row in rows) - min(min(row) for row in rows)


def ink_map(page: Any, dpi: int, threshold: int) -> InkMap:
    # get_pixmap(dpi=...) exists since v1.19.2 and ignores the matrix when set.
    # https://pymupdf.readthedocs.io/en/latest/page.html
    pixmap: Any = page.get_pixmap(dpi=dpi, colorspace=pymupdf.csGRAY, alpha=False)  # pyright: ignore
    table = bytes(1 if value < threshold else 0 for value in range(256))
    return InkMap(
        int(pixmap.width),
        int(pixmap.height),
        int(pixmap.stride),
        bytes(pixmap.samples),
        dpi / 72,
        table,
    )


def compare_ink(page: PageData, ink: InkMap, settings: Settings) -> Json:
    """Compare the drawn page with what the extractor found on it.

    Every text line must cover visible contrast. Ink that no line, image, drawing or
    figure covers is grouped into regions, because such regions show content that
    the text layer and the figure finder both missed.
    """
    visible = [
        ln for ln in page.lines if ink.contrast(inflate(ln.bbox, 0.5)) >= settings.min_contrast
    ]
    invisible = [ln.id for ln in page.lines if ln not in visible]
    covered = [inflate(ln.bbox, 1.5) for ln in page.lines]
    covered += [inflate(b, 1.5) for b in page.images + page.drawings + page.bars]
    covered += [inflate(f["bbox"], 1.5) for f in page.figures]
    cell = settings.cell_size
    columns, rows = int(page.width / cell) + 1, int(page.height / cell) + 1
    uncovered: set[tuple[int, int]] = set()
    for row in range(rows):
        for column in range(columns):
            box = (column * cell, row * cell, (column + 1) * cell, (row + 1) * cell)
            if ink.ink(box) >= 2 and not any(intersects(box, c) for c in covered):
                uncovered.add((column, row))
    regions: list[Json] = []
    remaining = set(uncovered)
    while remaining:
        start = remaining.pop()
        group = [start]
        frontier = [start]
        while frontier:
            column, row = frontier.pop()
            for dc in (-1, 0, 1):
                for dr in (-1, 0, 1):
                    neighbour = (column + dc, row + dr)
                    if neighbour in remaining:
                        remaining.remove(neighbour)
                        group.append(neighbour)
                        frontier.append(neighbour)
        if len(group) >= settings.uncovered_min_cells:
            box = bounding([(c * cell, r * cell, (c + 1) * cell, (r + 1) * cell) for c, r in group])
            regions.append({"bbox": rounded(box), "cells": len(group)})
    regions.sort(key=lambda r: (r["bbox"][1], r["bbox"][0]))
    return {
        "dpi": round(ink.scale * 72),
        "text_lines": len(page.lines),
        "text_lines_with_ink": len(visible),
        "text_lines_without_ink": invisible,
        "uncovered_ink_cells": len(uncovered),
        "uncovered_regions": regions,
    }


def recheck_invisible(
    pdf_page: Any, page: PageData, result: Json, settings: Settings, dpi: int = 288
) -> None:
    """Check each line without contrast again on a fine render of that line alone.

    Small, light glyphs lose their contrast in a coarse render. A line that shows
    contrast at the fine resolution is visible text and leaves the defect list.
    """
    lines = {ln.id: ln for ln in page.lines}
    still: list[str] = []
    for line_id in result["text_lines_without_ink"]:
        box = inflate(lines[line_id].bbox, 0.5)
        pix = pdf_page.get_pixmap(
            dpi=dpi,
            clip=to_rect(box),
            colorspace=pymupdf.csGRAY,  # pyright: ignore
            alpha=False,
        )
        samples = bytes(pix.samples)
        if not samples or max(samples) - min(samples) < settings.min_contrast:
            still.append(line_id)
    result["rechecked_at_dpi"] = dpi
    result["text_lines_with_ink"] += len(result["text_lines_without_ink"]) - len(still)
    result["text_lines_without_ink"] = still


def render_png(page: Any, box: Box | None, dpi: int) -> bytes:
    clip = to_rect(box) if box is not None else None
    pixmap: Any = page.get_pixmap(dpi=dpi, clip=clip, alpha=False)
    return bytes(pixmap.tobytes("png"))


def clipped_text(page: Any, box: Box) -> str:
    # A character is kept only when its box lies wholly inside the clip, so pad the box.
    # https://pymupdf.readthedocs.io/en/latest/page.html
    return str(page.get_text("text", clip=to_rect(inflate(box, 1.0)), flags=TEXT_FLAGS))


# Defects -----------------------------------------------------------------------------


def reading_order(page: PageData) -> tuple[list[int], list[int]]:
    """Return the stored block order and the geometric (top-left first) block order."""
    blocks = sorted({ln.block for ln in page.lines if ln.kind != "furniture"})
    geometric = sorted(
        blocks, key=lambda b: (round(page.block_boxes[b][1] / 3), page.block_boxes[b][0])
    )
    return blocks, geometric


def text_layer_defects(page: PageData) -> list[Json]:
    """Find code points that show an uncertain text layer on a page."""
    uncertain: dict[str, list[str]] = {}
    ligature: dict[str, list[str]] = {}
    for line in page.lines:
        for char in line.text:
            category = unicodedata.category(char)
            point = f"U+{ord(char):04X}"
            if char == "�" or category in ("Cc", "Co", "Cs") and char not in "\t":
                uncertain.setdefault(point, []).append(line.id)
            elif 0xFB00 <= ord(char) <= 0xFB06:
                ligature.setdefault(point, []).append(line.id)
    defects: list[Json] = []
    if uncertain:
        defects.append(
            {
                "kind": "text-layer-uncertainty",
                "pdf_page_index": page.index,
                "detail": "Control, private-use or replacement code points in the text layer: "
                + ", ".join(f"{p} x{len(ids)}" for p, ids in sorted(uncertain.items())),
                "evidence": sorted({i for ids in uncertain.values() for i in ids}),
            }
        )
    if ligature:
        defects.append(
            {
                "kind": "ligature-code-point",
                "pdf_page_index": page.index,
                "detail": "Ligature code points stay unexpanded in the text: "
                + ", ".join(f"{p} x{len(ids)}" for p, ids in sorted(ligature.items())),
                "evidence": sorted({i for ids in ligature.values() for i in ids}),
            }
        )
    return defects
