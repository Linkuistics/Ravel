"""The pattern report: a static HTML view of one discovery run.

The report has three pages, one for each layer, and keeps the layers apart:

- ``index.html`` shows the inferred patterns, the discovery settings, the rejected
  patterns and the observations that no pattern supports.
- ``interpretations.html`` shows the candidate episodes, moves and relationships.
- ``structure.html`` shows the observed document structure and the cited lines. Each
  cited element or line shows its printed page label, its page index and a render.

A link always goes from a derived layer to the layer below it, or across one layer.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from html import escape
from typing import Any

from urg.analysis import ANALYSIS
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.discovery import DISCOVERY, STRUCTURE_LAYERS
from urg.discovery import RECORD_FILE as DISCOVERY_FILE
from urg.errors import UrgError
from urg.extraction import EXTRACTION, render_anchor
from urg.extraction import RECORD_FILE as EXTRACTION_FILE
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.version import code_version

Json = dict[str, Any]

STAGE = "pattern-report"
REPORT = "pattern-report"
SCHEMA_VERSION = 1
INDEX = "index.html"
INTERPRETATIONS = "interpretations.html"
STRUCTURE = "structure.html"
MANIFEST = "report.json"
DEFAULT_DPI = 110

STYLE = """
body { font-family: system-ui, sans-serif; max-width: 72rem; margin: 1rem auto;
  padding: 0 1rem; line-height: 1.45; color: #1d1d1f; background: #fff; }
nav { margin-bottom: 1rem; }
nav a { margin-right: 1rem; }
section.item { border-top: 1px solid #ddd; padding: 0.5rem 0; }
.meta { color: #555; font-size: 0.9rem; }
.layer { font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em; color: #666; }
code, pre { background: #f4f4f6; }
pre { padding: 0.5rem; overflow-x: auto; white-space: pre-wrap; }
img.render { max-width: 100%; border: 1px solid #ccc; display: block; margin: 0.3rem 0; }
table { border-collapse: collapse; }
td, th { border: 1px solid #ddd; padding: 0.2rem 0.4rem; text-align: left;
  vertical-align: top; }
"""

LAYER_PAGES = {
    "pattern": INDEX,
    "interpretation": INTERPRETATIONS,
    "structure-section": STRUCTURE,
    "structure-element": STRUCTURE,
    "evidence-line": STRUCTURE,
}


class ReportError(UrgError):
    """The report stage cannot build the report."""


def anchor(observation_id: str) -> str:
    """Return the HTML anchor of an ID. IDs contain ':', which a file name cannot hold."""
    return re.sub(r"[^A-Za-z0-9._-]", "_", observation_id)


def render_name(observation_id: str) -> str:
    return f"render-{anchor(observation_id)}.png"


def href(layer: str, observation_id: str, here: str) -> str:
    page = LAYER_PAGES.get(layer)
    if page is None:
        raise ReportError(f"no report page shows the layer {layer}")
    return f"#{anchor(observation_id)}" if page == here else f"{page}#{anchor(observation_id)}"


def link(layer: str, observation_id: str, here: str) -> str:
    target = href(layer, observation_id, here)
    return f'<a href="{target}"><code>{escape(observation_id)}</code></a>'


def page(title: str, here: str, body: list[str]) -> bytes:
    nav = " ".join(
        f'<a href="{name}">{escape(label)}</a>' if name != here else f"<b>{escape(label)}</b>"
        for name, label in (
            (INDEX, "Patterns"),
            (INTERPRETATIONS, "Interpretations"),
            (STRUCTURE, "Document structure"),
        )
    )
    html = [
        "<!doctype html>",
        '<html lang="en"><head><meta charset="utf-8">',
        '<meta name="viewport" content="width=device-width, initial-scale=1">',
        f"<title>{escape(title)}</title><style>{STYLE}</style></head><body>",
        f"<nav>{nav}</nav>",
        *body,
        "</body></html>",
        "",
    ]
    return "\n".join(html).encode()


# --- Inputs ---------------------------------------------------------------------------


@dataclass(frozen=True)
class Book:
    item_key: str
    entry: Json
    extraction: StoredArtifact
    extraction_record: Json
    analysis_record: Json


def load_books(store: ArtifactStore, record: Json) -> list[Book]:
    books: list[Book] = []
    for entry in record["sources"]:
        extraction = store.load(EXTRACTION, entry["extraction_artifact"])
        analysis = store.load(ANALYSIS, entry["analysis_artifact"])
        books.append(
            Book(
                item_key=entry["item_key"],
                entry=entry,
                extraction=extraction,
                extraction_record=json.loads(extraction.read_bytes(EXTRACTION_FILE)),
                analysis_record=json.loads(analysis.read_bytes(ANALYSIS_FILE)),
            )
        )
    return books


@dataclass
class Targets:
    """The structure IDs that a link names, by book, so that the structure page shows them."""

    lines: dict[str, set[str]] = field(default_factory=dict[str, set[str]])
    rendered: dict[str, set[str]] = field(default_factory=dict[str, set[str]])

    def need_line(self, item_key: str, line_id: str) -> None:
        self.lines.setdefault(item_key, set()).add(line_id)

    def need_render(self, item_key: str, observation_id: str) -> None:
        self.rendered.setdefault(item_key, set()).add(observation_id)


def collect_targets(record: Json, books: list[Book]) -> Targets:
    """Find the lines that a link names and the elements and lines that the report renders.

    The report renders each element or line that a pattern cites. It also renders the
    evidence of each interpretation that a pattern cites, and the heading of each cited
    section.
    """
    targets = Targets()
    sections = {
        s["id"]: s for b in books for s in b.extraction_record["structure"].get("sections", [])
    }
    interpretations = {i["id"]: i for b in books for i in b.analysis_record["interpretations"]}
    for book in books:
        for interpretation in book.analysis_record["interpretations"]:
            for evidence in interpretation["evidence"]:
                if evidence["layer"] == "evidence-line":
                    targets.need_line(book.item_key, evidence["id"])
            for ref in _refs(interpretation):
                if ref["layer"] == "evidence-line":
                    targets.need_line(book.item_key, ref["id"])
    for pattern in record["patterns"]:
        for role in ("supporting", "exceptions", "unresolved"):
            for citation in pattern[role]:
                observation = citation["observation"]
                item_key = citation["item_key"]
                if observation["layer"] == "evidence-line":
                    targets.need_line(item_key, observation["id"])
                if observation["layer"] in ("structure-element", "evidence-line"):
                    targets.need_render(item_key, observation["id"])
                elif observation["layer"] == "structure-section":
                    heading = sections[observation["id"]].get("heading")
                    if heading:
                        targets.need_render(item_key, heading)
                elif observation["layer"] == "interpretation":
                    for evidence in interpretations[observation["id"]]["evidence"]:
                        targets.need_render(item_key, evidence["id"])
    for entry in record["unfitted"]:
        if entry["layer"] == "evidence-line":
            targets.need_line(entry["item_key"], entry["id"])
    return targets


def _refs(interpretation: Json) -> list[Json]:
    return [
        {"name": name, **interpretation[name]}
        for name in ("episode", "source", "target", "alternative_to")
        if isinstance(interpretation.get(name), dict)
    ]


def render_targets(
    store: ArtifactStore, books: list[Book], targets: Targets, dpi: int
) -> tuple[dict[str, bytes], Json]:
    """Render each target. A failed render is recorded and leaves the report without it."""
    files: dict[str, bytes] = {}
    outcomes: Json = {}
    for book in books:
        for observation_id in sorted(targets.rendered.get(book.item_key, set())):
            try:
                png = render_anchor(book.extraction, observation_id, dpi, store=store)
            except UrgError as error:
                outcomes[observation_id] = {"status": "failed", "error": str(error)}
                continue
            name = render_name(observation_id)
            files[name] = png
            outcomes[observation_id] = {"status": "rendered", "file": name}
    return files, outcomes


# --- Pages ----------------------------------------------------------------------------


def _uncertainty(value: Json) -> str:
    return f"Uncertainty: {escape(value['level'])}. {escape(value['reason'])}"


def _citation_list(pattern: Json, role: str, title: str) -> list[str]:
    citations = pattern[role]
    if not citations:
        return [f"<p class='meta'>{escape(title)}: none.</p>"]
    items = [
        f"<li><span class='layer'>{escape(c['observation']['layer'])}</span> "
        f"{link(c['observation']['layer'], c['observation']['id'], INDEX)} "
        f"({escape(c['item_key'])}): {escape(c['note'])}</li>"
        for c in citations
    ]
    return [f"<h4>{escape(title)} ({len(citations)})</h4>", "<ul>", *items, "</ul>"]


def _json_block(value: Any) -> str:
    return f"<pre>{escape(json.dumps(value, indent=2, ensure_ascii=False))}</pre>"


def index_page(record: Json) -> bytes:
    run = record["discovery_run"]
    body = [
        "<h1>Candidate patterns</h1>",
        f"<p>{escape(record['review_note'])}</p>",
        "<p>This page shows the inferred patterns. "
        f"<a href='{INTERPRETATIONS}'>Interpretations</a> shows the candidate episodes, moves "
        "and relationships. "
        f"<a href='{STRUCTURE}'>Document structure</a> shows the observed structure. "
        "These preference weights changed the discovery itself. "
        "They do not reweight a stored result.</p>",
        "<h2>Sources</h2>",
        "<table><tr><th>Book</th><th>Title</th><th>Chapter</th><th>Preference weight</th></tr>",
    ]
    for source in record["sources"]:
        chapter = source["chapter"]
        body.append(
            f"<tr><td>{escape(source['item_key'])}</td><td>{escape(source['title'])}</td>"
            f"<td>{escape(str(chapter['number']))}: {escape(chapter['title'])}</td>"
            f"<td>{source['preference_weight']:g}</td></tr>"
        )
    body.append("</table>")
    body += [f"<h2>Patterns ({len(record['patterns'])})</h2>"]
    for pattern in record["patterns"]:
        body += [
            f"<section class='item' id='{anchor(pattern['id'])}'>",
            f"<h3>{escape(pattern['name'])}</h3>",
            f"<p class='meta'><code>{escape(pattern['id'])}</code>. "
            f"Basis: {escape(pattern['basis'])}. Books: {escape(', '.join(pattern['books']))}. "
            f"Review status: {escape(pattern['review_status'])}.</p>",
            f"<p>{escape(pattern['description'])}</p>",
            f"<p class='meta'>{_uncertainty(pattern['uncertainty'])}</p>",
        ]
        if pattern["rules"]:
            rules = "\n".join(pattern["rules"])
            body += ["<h4>Grammar fragment</h4>", f"<pre>{escape(rules)}</pre>"]
        if pattern["representation"]:
            body += ["<h4>Representation needs</h4>", "<ul>"]
            body += [
                f"<li><b>{escape(r['feature'])}</b>: {escape(r['justification'])}</li>"
                for r in pattern["representation"]
            ]
            body.append("</ul>")
        body += _citation_list(pattern, "supporting", "Supporting instances")
        body += _citation_list(pattern, "exceptions", "Exceptions")
        body += _citation_list(pattern, "unresolved", "Unresolved cases")
        body.append("</section>")
    unsupported = len(record["unfitted"])
    body += [f"<h2 id='unfitted'>Observations that no pattern supports ({unsupported})</h2>"]
    body += ["<ul>"]
    for entry in record["unfitted"]:
        cited = ", ".join(
            f"{c['role']} of {link('pattern', c['pattern'], INDEX)}" for c in entry["cited_as"]
        )
        label = entry.get("title") or entry.get("label") or ""
        body.append(
            f"<li><span class='layer'>{escape(entry['layer'])}</span> "
            f"{link(entry['layer'], entry['id'], INDEX)} {escape(label)}"
            + (f" (cited as {cited})" if cited else "")
            + "</li>"
        )
    body.append("</ul>")
    body += [f"<h2>Rejected patterns ({len(record['rejected'])})</h2>", "<ul>"]
    body += [
        f"<li>{escape(r['name'])} (<code>{escape(r['local_id'])}</code>): "
        f"{escape('; '.join(r['reasons']))}</li>"
        for r in record["rejected"]
    ]
    body.append("</ul>")
    body += [
        "<h2>Discovery run</h2>",
        f"<p>Method: {escape(run['method']['name'])}. {escape(run['method']['description'])}</p>",
        f"<p>Fit: {escape(run['method']['fit'])}</p>",
        f"<p>Response artifact: <code>{escape(run['response_artifact'])}</code>, attempt "
        f"{run['attempt']}. Instructions: <code>{escape(run['instructions']['path'])}</code>, "
        f"version {run['instructions']['version']}.</p>",
        "<h3>Model</h3>",
        _json_block(run["model"]),
        "<h3>Settings</h3>",
        _json_block(run["settings"]),
        "<h3>Preference weights</h3>",
        _json_block(run["preference_weights"]),
    ]
    return page("Candidate patterns", INDEX, body)


def _cited_by(record: Json) -> dict[str, list[Json]]:
    result: dict[str, list[Json]] = {}
    for pattern in record["patterns"]:
        for role in ("supporting", "exceptions", "unresolved"):
            for citation in pattern[role]:
                result.setdefault(citation["observation"]["id"], []).append(
                    {"pattern": pattern["id"], "role": role}
                )
    return result


def _backlinks(observation_id: str, cited_by: dict[str, list[Json]], here: str) -> str:
    entries = cited_by.get(observation_id, [])
    if not entries:
        return ""
    links = ", ".join(f"{c['role']} of {link('pattern', c['pattern'], here)}" for c in entries)
    return f"<p class='meta'>Cited as {links}.</p>"


def interpretations_page(record: Json, books: list[Book]) -> bytes:
    cited_by = _cited_by(record)
    scope = set(
        record["discovery_run"]["settings"]["content"]["observation_scope"]["interpretation_kinds"]
    )
    body = [
        "<h1>Candidate episodes, moves and relationships</h1>",
        "<p>A model proposed each interpretation, unless the interpretation names a "
        "correction. Each interpretation cites the document structure.</p>",
    ]
    for book in books:
        body.append(f"<h2>{escape(book.item_key)}: {escape(book.entry['title'])}</h2>")
        superseded: dict[str, list[Json]] = {}
        for old in book.analysis_record.get("history", []):
            superseded.setdefault(old["id"], []).append(old)
        for interpretation in book.analysis_record["interpretations"]:
            in_scope = interpretation["kind"] in scope
            refs = "; ".join(
                f"{escape(r['name'])}: {link(r['layer'], r['id'], INTERPRETATIONS)}"
                for r in _refs(interpretation)
            )
            evidence = ", ".join(
                f"{link(e['layer'], e['id'], INTERPRETATIONS)} (page "
                f"{escape(str(e['printed_page_label']))}, index {e['pdf_page_index']})"
                for e in interpretation["evidence"]
            )
            body += [
                f"<section class='item' id='{anchor(interpretation['id'])}'>",
                f"<p><span class='layer'>{escape(interpretation['kind'])}</span> "
                f"<b>{escape(interpretation['label'] or '')}</b> "
                f"<code>{escape(interpretation['id'])}</code>"
                + ("" if in_scope else " (outside the discovery scope)")
                + "</p>",
                f"<p>{escape(interpretation['description'])}</p>",
                f"<p class='meta'>{refs}</p>" if refs else "",
                f"<p class='meta'>Evidence: {evidence}</p>",
                f"<p class='meta'>{_uncertainty(interpretation['uncertainty'])} "
                f"Version {interpretation['version']}. "
                f"Review status: {escape(interpretation['review_status'])}.</p>",
                _correction(interpretation),
                *(_superseded(old) for old in superseded.get(interpretation["id"], [])),
                _backlinks(interpretation["id"], cited_by, INTERPRETATIONS),
                "</section>",
            ]
    return page("Interpretations", INTERPRETATIONS, body)


def _correction(interpretation: Json) -> str:
    block = interpretation.get("correction")
    if block is None:
        return ""
    author = interpretation["author"]
    return (
        f"<p class='meta'>Correction <code>{escape(block['id'])}</code> by "
        f"{escape(author['name'])} ({escape(author['kind'])}, {escape(block['origin'])}): "
        f"{escape(block['rationale'])}</p>"
    )


def _superseded(old: Json) -> str:
    evidence = ", ".join(escape(e["id"]) for e in old["evidence"])
    return (
        f"<details class='meta'><summary>Version {old['version']}, superseded by "
        f"<code>{escape(old['superseded_by'])}</code></summary>"
        f"<p><b>{escape(old['label'] or '')}</b>: {escape(old['description'])}</p>"
        f"<p>Evidence: {evidence}. Review status: {escape(old['review_status'])}.</p>"
        "</details>"
    )


def _location(item: Json) -> str:
    return (
        f"page {escape(str(item.get('printed_page_label')))}, "
        f"index {escape(str(item.get('pdf_page_index')))}"
    )


def _render(observation_id: str, renders: Json) -> str:
    outcome = renders.get(observation_id)
    if outcome is None:
        return ""
    if outcome["status"] == "rendered":
        return (
            f"<img class='render' src='{outcome['file']}' "
            f"alt='Render of {escape(observation_id)}' loading='lazy'>"
        )
    return f"<p class='meta'>No render: {escape(outcome['error'])}</p>"


def structure_page(record: Json, books: list[Book], targets: Targets, renders: Json) -> bytes:
    cited_by = _cited_by(record)
    body = [
        "<h1>Observed document structure</h1>",
        "<p>The extraction stage observed this structure. No model proposed it.</p>",
    ]
    for book in books:
        structure = book.extraction_record["structure"]
        elements = {e["id"]: e for e in structure["elements"]}
        body.append(f"<h2>{escape(book.item_key)}: {escape(book.entry['title'])}</h2>")
        body.append("<h3>Sections and elements</h3>")
        placed: set[str] = set()
        for section in structure.get("sections", []):
            heading = elements.get(section.get("heading") or "")
            body += [
                f"<section class='item' id='{anchor(section['id'])}'>",
                f"<h4><span class='layer'>section</span> {escape(section['title'])} "
                f"<code>{escape(section['id'])}</code></h4>",
                f"<p class='meta'>Level {section['level']}. "
                f"{_location(heading or section)}."
                + (
                    f" Parent: {link('structure-section', section['parent'], STRUCTURE)}."
                    if section.get("parent")
                    else ""
                )
                + "</p>",
                _backlinks(section["id"], cited_by, STRUCTURE),
            ]
            for element_id in section["elements"]:
                if element_id in elements:
                    body.append(_element(elements[element_id], cited_by, renders))
                    placed.add(element_id)
            body.append("</section>")
        loose = [e for e in structure["elements"] if e["id"] not in placed]
        if loose:
            body.append("<h4>Elements outside any section</h4>")
            body += [_element(e, cited_by, renders) for e in loose]
        lines = {e["id"]: e for e in book.extraction_record["evidence"]}
        wanted = sorted(targets.lines.get(book.item_key, set()))
        body.append(f"<h3>Cited lines ({len(wanted)})</h3>")
        for line_id in wanted:
            line = lines.get(line_id)
            if line is None:
                raise ReportError(f"the extraction of {book.item_key} has no line {line_id}")
            body += [
                f"<section class='item' id='{anchor(line_id)}'>",
                f"<p><span class='layer'>line</span> <code>{escape(line_id)}</code> "
                f"<span class='meta'>{_location(line)}</span></p>",
                f"<pre>{escape(line.get('text', ''))}</pre>",
                _render(line_id, renders),
                _backlinks(line_id, cited_by, STRUCTURE),
                "</section>",
            ]
    return page("Document structure", STRUCTURE, body)


def _element(element: Json, cited_by: dict[str, list[Json]], renders: Json) -> str:
    return "\n".join(
        [
            f"<div class='item' id='{anchor(element['id'])}'>",
            f"<p><span class='layer'>{escape(element['kind'])}</span> "
            f"<code>{escape(element['id'])}</code> "
            f"<span class='meta'>{_location(element)}</span></p>",
            f"<pre>{escape(str(element.get('text') or ''))}</pre>",
            _render(element["id"], renders),
            _backlinks(element["id"], cited_by, STRUCTURE),
            "</div>",
        ]
    )


# --- Stage ----------------------------------------------------------------------------


def report_dependency(discovery: StoredArtifact, books: list[Book], dpi: int) -> DependencyRecord:
    def artifact(role: str, name: str, key: str) -> InputIdentity:
        return InputIdentity(role=role, kind="artifact", locator=f"{name}/{key}", identity=key)

    return DependencyRecord(
        stage=STAGE,
        artifact=REPORT,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"dpi": dpi},
        inputs=[
            artifact("pattern-discovery", DISCOVERY, discovery.record.key),
            *(artifact("chapter-extraction", EXTRACTION, b.extraction.record.key) for b in books),
        ],
    )


@dataclass(frozen=True)
class ReportRun:
    report: StoredArtifact
    reused: bool
    manifest: Json


def build_report(
    store: ArtifactStore, discovery: StoredArtifact, dpi: int = DEFAULT_DPI
) -> ReportRun:
    """Build the report of one discovery run, or reuse the stored report."""
    record: Json = json.loads(discovery.read_bytes(DISCOVERY_FILE))
    books = load_books(store, record)
    dependency = report_dependency(discovery, books, dpi)
    found = store.reuse(dependency)
    if found is not None:
        return ReportRun(found, True, json.loads(found.read_bytes(MANIFEST)))
    targets = collect_targets(record, books)
    files, renders = render_targets(store, books, targets, dpi)
    manifest: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "discovery_artifact": discovery.record.key,
        "pages": {
            "patterns": INDEX,
            "interpretations": INTERPRETATIONS,
            "document-structure": STRUCTURE,
        },
        "layers": {layer: LAYER_PAGES[layer] for layer in ("pattern", "interpretation")}
        | {layer: STRUCTURE for layer in STRUCTURE_LAYERS},
        "renders": renders,
    }
    files |= {
        INDEX: index_page(record),
        INTERPRETATIONS: interpretations_page(record, books),
        STRUCTURE: structure_page(record, books, targets, renders),
        MANIFEST: (json.dumps(manifest, indent=2) + "\n").encode(),
    }
    return ReportRun(store.write(dependency, files), False, manifest)


def latest_discovery(store: ArtifactStore, key: str | None = None) -> StoredArtifact:
    """Return the named discovery run, or the newest stored one."""
    key = key or store.latest_key(DISCOVERY)
    if key is None:
        raise ReportError("no pattern-discovery run is in the store. Run pattern-discovery first")
    return store.load(DISCOVERY, key)
