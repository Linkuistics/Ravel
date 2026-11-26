"""The baseline and weighted pattern reports: reweighting the fixed patterns of one discovery run.

The stage writes two research artifacts:

- ``baseline-report`` scores the patterns under the equal-book baseline. It depends on the
  discovery run, the chapter extractions and analyses of that run and the relevance
  assessments. It does not depend on the preference weights.
- ``weighted-report`` scores the same patterns under supplied preference weights and
  compares the two scorings. It depends on the baseline report and the weight file.

A change to the preference weights reuses the extractions, the analyses, the discovery run
and the baseline report. Neither artifact calls the model.
"""

from __future__ import annotations

import hashlib
import json
from collections import Counter
from collections.abc import Sequence
from dataclasses import dataclass
from html import escape
from pathlib import Path
from typing import Any

from pydantic import BaseModel, ValidationError

from urg.analysis import ANALYSIS
from urg.discovery import DISCOVERY
from urg.discovery import RECORD_FILE as DISCOVERY_FILE
from urg.extraction import EXTRACTION
from urg.report import STYLE, Book, ReportError, anchor, load_books
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.version import code_version
from urg.weighting import (
    Corpus,
    Material,
    PatternCitations,
    RelevanceConfig,
    Scores,
    WeightConfig,
    baseline_scores,
    build_corpus,
    extraction_uncertainty,
    ignored_entries,
    interpretation_uncertainty_levels,
    relevance_levels,
    weighted_scores,
)

Json = dict[str, Any]

STAGE = "weighted-report"
BASELINE = "baseline-report"
WEIGHTED = "weighted-report"
SCHEMA_VERSION = 1
BASELINE_PAGE = "baseline.html"
WEIGHTED_PAGE = "weighted.html"
COMPARISON_PAGE = "comparison.html"
SCORES = "scores.json"
FORMULA_DOC = "docs/stages/weighted-report.md"
DEFAULT_WEIGHTS = Path("docs/weights/illustrative-weights.json")
DEFAULT_RELEVANCE = Path("docs/weights/illustrative-relevance.json")

LEVEL_ORDER = ("high", "medium", "low", "mixed", "not assessed")

EXTRA_STYLE = """
.notice { border-left: 4px solid #b26b00; padding: 0.3rem 0.8rem; background: #fff7e6; }
.illustrative { border-left-color: #b00020; background: #fdecee; }
td.num { text-align: right; font-variant-numeric: tabular-nums; }
"""


# --- Inputs ---------------------------------------------------------------------------


def _load_config[M: BaseModel](path: Path, model: type[M], what: str) -> tuple[M, bytes]:
    try:
        data = path.read_bytes()
    except OSError as e:
        raise ReportError(f"cannot read the {what} file {path}: {e.strerror}") from e
    try:
        return model.model_validate_json(data), data
    except ValidationError as e:
        raise ReportError(f"the {what} file {path} is not valid: {e}") from e


def load_weights(path: Path) -> tuple[WeightConfig, bytes]:
    return _load_config(path, WeightConfig, "preference weight")


def load_relevance(path: Path) -> tuple[RelevanceConfig, bytes]:
    return _load_config(path, RelevanceConfig, "relevance assessment")


def _file_identity(role: str, path: Path, data: bytes) -> InputIdentity:
    return InputIdentity(
        role=role, kind="file", locator=path.as_posix(), identity=hashlib.sha256(data).hexdigest()
    )


def _artifact_identity(role: str, name: str, key: str) -> InputIdentity:
    return InputIdentity(role=role, kind="artifact", locator=f"{name}/{key}", identity=key)


def _upstream(discovery: StoredArtifact, books: Sequence[Book]) -> list[InputIdentity]:
    return [
        _artifact_identity("pattern-discovery", DISCOVERY, discovery.record.key),
        *(
            identity
            for b in books
            for identity in (
                _artifact_identity(
                    "chapter-extraction", EXTRACTION, b.entry["extraction_artifact"]
                ),
                _artifact_identity("episode-analysis", ANALYSIS, b.entry["analysis_artifact"]),
            )
        ),
    ]


# --- Pages ----------------------------------------------------------------------------


def _number(value: float) -> str:
    return f"{value:.4f}"


def _counts(counter: Counter[str], order: Sequence[str] = LEVEL_ORDER) -> str:
    if not counter:
        return "none"
    keys = [k for k in order if k in counter] + sorted(k for k in counter if k not in order)
    return ", ".join(f"{escape(k)} {counter[k]}" for k in keys)


def _page(title: str, here: str, nav: Sequence[tuple[str, str]], body: list[str]) -> bytes:
    links = " ".join(
        f'<a href="{escape(target)}">{escape(label)}</a>'
        if target != here
        else f"<b>{escape(label)}</b>"
        for target, label in nav
    )
    html = [
        "<!doctype html>",
        '<html lang="en"><head><meta charset="utf-8">',
        '<meta name="viewport" content="width=device-width, initial-scale=1">',
        f"<title>{escape(title)}</title><style>{STYLE}{EXTRA_STYLE}</style></head><body>",
        f"<nav>{links}</nav>",
        *body,
        "</body></html>",
        "",
    ]
    return "\n".join(html).encode()


def reweighting_notice(record: Json, discovery_key: str) -> str:
    """State that the report reweights fixed patterns and name the discovery run."""
    weights = record["discovery_run"].get("preference_weights", {})
    listed = ", ".join(f"{escape(k)} {v}" for k, v in sorted(weights.items())) or "none recorded"
    return (
        "<p class='notice'><b>This report reweights fixed patterns.</b> "
        f"Discovery run <code>{escape(discovery_key)}</code> found these patterns under its "
        f"own recorded settings and preference weights ({listed}). This report recomputes "
        "the prevalence of those patterns only. It is not a new discovery run, and it does not "
        "show which patterns a discovery run under other weights would find.</p>"
    )


def _book_table(corpus: Corpus, record: Json, scores: Scores, shown_weights: bool) -> list[str]:
    titles = {s["item_key"]: s.get("title", "") for s in record["sources"]}
    head = "<tr><th>Book</th><th>Title</th>"
    head += "<th>Book weight</th>" if shown_weights else ""
    head += "<th>Share s_b</th><th>Listed observations</th><th>Denominator</th><th>Note</th></tr>"
    rows = [head]
    for b in corpus.books:
        listed = sum(1 for o in corpus.observations.values() if o.item_key == b)
        weight = f"<td class='num'>{scores.book_weights[b]:g}</td>" if shown_weights else ""
        rows.append(
            f"<tr><td><code>{escape(b)}</code></td><td>{escape(titles.get(b, ''))}</td>{weight}"
            f"<td class='num'>{_number(scores.shares[b])}</td><td class='num'>{listed}</td>"
            f"<td class='num'>{scores.denominators[b]:g}</td>"
            f"<td>{escape(scores.excluded.get(b, ''))}</td></tr>"
        )
    return ["<h2>Books</h2>", "<table>", *rows, "</table>"]


def _pattern_rows(
    corpus: Corpus,
    record: Json,
    scores: Scores,
    relevance: RelevanceConfig,
) -> list[str]:
    ranks = scores.ranks()
    head = (
        "<tr><th>Rank</th><th>Pattern</th><th>Prevalence</th><th>Exception prevalence</th>"
        "<th>Book coverage</th><th>Supporting by book</th>"
        "<th>Relevance assessments of supporting instances</th>"
        "<th>Extraction uncertainty</th><th>Interpretation uncertainty</th></tr>"
    )
    rows = [head]
    patterns = sorted(record["patterns"], key=lambda p: (ranks[p["id"]], p["id"]))
    for pattern in patterns:
        score = scores.patterns[pattern["id"]]
        supporting = [c["observation"]["id"] for c in pattern["supporting"]]
        by_book = "<br>".join(
            f"{escape(t.item_key)}: {t.supporting_weight:g} / {t.denominator:g}"
            for t in score.terms
            if t.supporting_weight > 0
        )
        touched, defects = extraction_uncertainty(corpus, supporting)
        chapter = sorted({d for b in pattern["books"] for d in corpus.chapter_defects.get(b, ())})
        extraction = (
            f"{touched} of {len(supporting)} supporting instances rest on evidence with "
            f"defects ({len(defects)} defects)"
        )
        if chapter:
            extraction += f". Chapter-level defects: {escape(', '.join(chapter))}"
        levels, alternatives = interpretation_uncertainty_levels(corpus, supporting)
        interpretation = (
            f"Pattern: {escape(pattern['uncertainty']['level'])}. "
            f"Supporting interpretations: {_counts(levels)}. "
            f"Alternative readings of them: {alternatives}"
        )
        uncounted = (
            f"<br><span class='meta'>Not counted: {escape(', '.join(score.uncounted))}</span>"
            if score.uncounted
            else ""
        )
        rows.append(
            f"<tr id='{anchor(pattern['id'])}'><td class='num'>{ranks[pattern['id']]}</td>"
            f"<td><code>{escape(pattern['id'])}</code><br>{escape(pattern['name'])}{uncounted}</td>"
            f"<td class='num'>{_number(score.prevalence)}</td>"
            f"<td class='num'>{_number(score.exception_prevalence)}</td>"
            f"<td class='num'>{_number(score.book_coverage)}</td>"
            f"<td>{by_book}</td>"
            f"<td>{_counts(relevance_levels(corpus, supporting, relevance))}</td>"
            f"<td>{extraction}</td><td>{interpretation}</td></tr>"
        )
    return ["<h2>Patterns</h2>", "<table>", *rows, "</table>"]


def _columns_note() -> str:
    return (
        "<p class='meta'>The four inputs stay in separate columns. Only preference weights "
        "change the prevalence. A relevance level, an extraction defect or an interpretation "
        "uncertainty never enters a number. Prevalence is the share-weighted mean, over books, "
        "of the weighted fraction of listed observations that support the pattern. "
        f"<code>{FORMULA_DOC}</code> states the formula, the normalisation, the denominators "
        "and the handling of missing material.</p>"
    )


def _relevance_section(relevance: RelevanceConfig, ignored: list[str]) -> list[str]:
    css = "notice illustrative" if relevance.status == "illustrative" else "notice"
    label = (
        "Illustrative relevance assessments"
        if relevance.status == "illustrative"
        else "Relevance assessments supplied by the human"
    )
    body = [
        "<h2>Relevance assessments</h2>",
        f"<p class='{css}'><b>{label}.</b> {escape(relevance.statement)}</p>",
        "<p class='meta'>An unassessed book or section counts as <i>not assessed</i>. An "
        "interpretation whose evidence spans sections with different levels counts as "
        "<i>mixed</i>.</p>",
    ]
    if ignored:
        body += [
            "<p>Ignored entries:</p>",
            "<ul>",
            *(f"<li>{escape(i)}</li>" for i in ignored),
            "</ul>",
        ]
    return body


def baseline_page(
    record: Json,
    discovery_key: str,
    corpus: Corpus,
    scores: Scores,
    relevance: RelevanceConfig,
    relevance_ignored: list[str],
) -> bytes:
    body = [
        "<h1>Equal-book baseline</h1>",
        reweighting_notice(record, discovery_key),
        "<p>Each analysed book has the same total influence, whatever the length of its "
        "analysed chapter. Every book weight and every observation weight is 1.</p>",
        _columns_note(),
        *_book_table(corpus, record, scores, shown_weights=False),
        *_pattern_rows(corpus, record, scores, relevance),
        *_relevance_section(relevance, relevance_ignored),
    ]
    return _page("Equal-book baseline", BASELINE_PAGE, [(BASELINE_PAGE, "Baseline")], body)


def _weights_section(config: WeightConfig, corpus: Corpus, ignored: list[str]) -> list[str]:
    if config.status == "illustrative":
        banner = (
            "<p class='notice illustrative'><b>Illustrative preference weights.</b> These "
            f"weights are not the human's preferences. {escape(config.statement)}</p>"
        )
    else:
        banner = (
            "<p class='notice'><b>Preference weights supplied by the human.</b> "
            f"{escape(config.statement)}</p>"
        )
    rows = ["<tr><th>Entry</th><th>Weight</th><th>Reason</th></tr>"]
    rows.append(
        f"<tr><td>default book weight</td>"
        f"<td class='num'>{config.default_book_weight:g}</td><td></td></tr>"
    )
    rows += [
        f"<tr><td>book <code>{escape(k)}</code></td><td class='num'>{e.weight:g}</td>"
        f"<td>{escape(e.reason)}</td></tr>"
        for k, e in sorted(config.books.items())
    ]
    rows.append(
        f"<tr><td>default section weight</td>"
        f"<td class='num'>{config.default_section_weight:g}</td><td></td></tr>"
    )
    rows += [
        f"<tr><td>section <code>{escape(k)}</code> {escape(corpus.section_titles.get(k, ''))}</td>"
        f"<td class='num'>{e.weight:g}</td><td>{escape(e.reason)}</td></tr>"
        for k, e in sorted(config.sections.items())
    ]
    body = [banner, "<h2>Preference weights</h2>", "<table>", *rows, "</table>"]
    if ignored:
        body += [
            "<p>Ignored entries:</p>",
            "<ul>",
            *(f"<li>{escape(i)}</li>" for i in ignored),
            "</ul>",
        ]
    return body


def _nav(baseline_key: str) -> list[tuple[str, str]]:
    return [
        (WEIGHTED_PAGE, "Weighted"),
        (COMPARISON_PAGE, "Comparison"),
        (f"../../{BASELINE}/{baseline_key}/{BASELINE_PAGE}", "Baseline"),
    ]


def weighted_page(
    record: Json,
    discovery_key: str,
    baseline_key: str,
    corpus: Corpus,
    scores: Scores,
    config: WeightConfig,
    ignored: list[str],
    relevance: RelevanceConfig,
) -> bytes:
    title = (
        "Weighted report (illustrative weights)"
        if config.status == "illustrative"
        else "Weighted report"
    )
    body = [
        f"<h1>{escape(title)}</h1>",
        reweighting_notice(record, discovery_key),
        *_weights_section(config, corpus, ignored),
        _columns_note(),
        *_book_table(corpus, record, scores, shown_weights=True),
        *_pattern_rows(corpus, record, scores, relevance),
        *_relevance_section(relevance, []),
    ]
    return _page(title, WEIGHTED_PAGE, _nav(baseline_key), body)


def comparison_page(
    record: Json,
    discovery_key: str,
    baseline_key: str,
    baseline: Scores,
    weighted: Scores,
    config: WeightConfig,
) -> bytes:
    base_ranks, weighted_ranks = baseline.ranks(), weighted.ranks()
    names = {p["id"]: p["name"] for p in record["patterns"]}
    rows = [
        "<tr><th>Pattern</th><th>Baseline prevalence</th><th>Weighted prevalence</th>"
        "<th>Difference</th><th>Baseline rank</th><th>Weighted rank</th>"
        "<th>Baseline book coverage</th><th>Weighted book coverage</th></tr>"
    ]
    for pattern_id in sorted(names, key=lambda p: (weighted_ranks[p], p)):
        b, w = baseline.patterns[pattern_id], weighted.patterns[pattern_id]
        rows.append(
            f"<tr><td><code>{escape(pattern_id)}</code><br>{escape(names[pattern_id])}</td>"
            f"<td class='num'>{_number(b.prevalence)}</td>"
            f"<td class='num'>{_number(w.prevalence)}</td>"
            f"<td class='num'>{w.prevalence - b.prevalence:+.4f}</td>"
            f"<td class='num'>{base_ranks[pattern_id]}</td>"
            f"<td class='num'>{weighted_ranks[pattern_id]}</td>"
            f"<td class='num'>{_number(b.book_coverage)}</td>"
            f"<td class='num'>{_number(w.book_coverage)}</td></tr>"
        )
    shares = ["<tr><th>Book</th><th>Baseline share</th><th>Weighted share</th></tr>"] + [
        f"<tr><td><code>{escape(k)}</code></td><td class='num'>{_number(baseline.shares[k])}</td>"
        f"<td class='num'>{_number(weighted.shares[k])}</td></tr>"
        for k in baseline.shares
    ]
    label = (
        "the illustrative preference weights, which are not the human's preferences"
        if config.status == "illustrative"
        else "the preference weights supplied by the human"
    )
    body = [
        "<h1>Baseline and weighted comparison</h1>",
        reweighting_notice(record, discovery_key),
        f"<p>The comparison sets the equal-book baseline against {escape(label)}.</p>",
        "<h2>Book shares</h2>",
        "<table>",
        *shares,
        "</table>",
        "<h2>Patterns</h2>",
        "<table>",
        *rows,
        "</table>",
    ]
    return _page("Baseline and weighted comparison", COMPARISON_PAGE, _nav(baseline_key), body)


def scores_json(scores: Scores) -> Json:
    ranks = scores.ranks()
    return {
        "book_weights": scores.book_weights,
        "shares": scores.shares,
        "denominators": scores.denominators,
        "excluded": scores.excluded,
        "patterns": {
            pid: {
                "prevalence": s.prevalence,
                "exception_prevalence": s.exception_prevalence,
                "book_coverage": s.book_coverage,
                "rank": ranks[pid],
                "uncounted": list(s.uncounted),
                "terms": [
                    {
                        "item_key": t.item_key,
                        "share": t.share,
                        "supporting_weight": t.supporting_weight,
                        "exception_weight": t.exception_weight,
                        "denominator": t.denominator,
                        "fraction": t.fraction,
                    }
                    for t in s.terms
                ],
            }
            for pid, s in scores.patterns.items()
        },
    }


def _dump(value: Json) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


# --- Stage ----------------------------------------------------------------------------


@dataclass(frozen=True)
class WeightedRun:
    baseline: StoredArtifact
    baseline_reused: bool
    weighted: StoredArtifact
    weighted_reused: bool
    upstream: list[InputIdentity]


def build_weighted_reports(
    store: ArtifactStore,
    discovery: StoredArtifact,
    weights_path: Path,
    relevance_path: Path,
) -> WeightedRun:
    """Build the baseline and weighted reports of one discovery run, or reuse stored ones."""
    weights, weights_bytes = load_weights(weights_path)
    relevance, relevance_bytes = load_relevance(relevance_path)
    record: Json = json.loads(discovery.read_bytes(DISCOVERY_FILE))
    books = load_books(store, record)
    corpus = build_corpus(
        record, [Material(b.item_key, b.extraction_record, b.analysis_record) for b in books]
    )
    patterns = [PatternCitations.of(p) for p in record["patterns"]]
    upstream = _upstream(discovery, books)
    relevance_input = _file_identity("relevance-assessments", relevance_path, relevance_bytes)
    relevance_ignored = ignored_entries(corpus, relevance.books, relevance.sections)

    baseline_dependency = DependencyRecord(
        stage=STAGE,
        artifact=BASELINE,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"aggregation": "equal-book-baseline"},
        inputs=[*upstream, relevance_input],
    )
    baseline = baseline_scores(corpus, patterns)
    found = store.reuse(baseline_dependency)
    baseline_reused = found is not None
    if found is None:
        manifest = _manifest(BASELINE, discovery.record.key, relevance, None) | {
            "pages": {"baseline": BASELINE_PAGE},
            "ignored_relevance_entries": relevance_ignored,
        }
        found = store.write(
            baseline_dependency,
            {
                BASELINE_PAGE: baseline_page(
                    record, discovery.record.key, corpus, baseline, relevance, relevance_ignored
                ),
                SCORES: _dump(manifest | {"scores": scores_json(baseline)}),
            },
        )
    baseline_artifact = found

    weighted_dependency = DependencyRecord(
        stage=STAGE,
        artifact=WEIGHTED,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"aggregation": "preference-weights"},
        inputs=[
            _artifact_identity("baseline-report", BASELINE, baseline_artifact.record.key),
            _file_identity("preference-weights", weights_path, weights_bytes),
            *upstream,
            relevance_input,
        ],
    )
    found = store.reuse(weighted_dependency)
    weighted_reused = found is not None
    if found is None:
        scores = weighted_scores(corpus, patterns, weights)
        ignored = ignored_entries(corpus, weights.books, weights.sections)
        manifest = _manifest(WEIGHTED, discovery.record.key, relevance, weights) | {
            "baseline_artifact": baseline_artifact.record.key,
            "pages": {"weighted": WEIGHTED_PAGE, "comparison": COMPARISON_PAGE},
            "ignored_weight_entries": ignored,
        }
        found = store.write(
            weighted_dependency,
            {
                WEIGHTED_PAGE: weighted_page(
                    record,
                    discovery.record.key,
                    baseline_artifact.record.key,
                    corpus,
                    scores,
                    weights,
                    ignored,
                    relevance,
                ),
                COMPARISON_PAGE: comparison_page(
                    record,
                    discovery.record.key,
                    baseline_artifact.record.key,
                    baseline,
                    scores,
                    weights,
                ),
                SCORES: _dump(manifest | {"scores": scores_json(scores)}),
            },
        )
    return WeightedRun(baseline_artifact, baseline_reused, found, weighted_reused, upstream)


def _manifest(
    artifact: str, discovery_key: str, relevance: RelevanceConfig, weights: WeightConfig | None
) -> Json:
    manifest: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "artifact": artifact,
        "kind": "reweighting",
        "limitation": (
            "This report reweights the fixed patterns of one discovery run. It is not a "
            "discovery run and does not show which patterns other weights would find."
        ),
        "discovery_artifact": discovery_key,
        "formula": FORMULA_DOC,
        "relevance_assessments": {"status": relevance.status, "statement": relevance.statement},
    }
    if weights is not None:
        manifest["preference_weights"] = {
            "status": weights.status,
            "statement": weights.statement,
            "described_as": (
                "illustrative weights, not the human's preferences"
                if weights.status == "illustrative"
                else "the human's preference weights"
            ),
        }
    return manifest
