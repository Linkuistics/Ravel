"""Tests of the baseline and weighted reports over the fixture discovery run.

The fixture discovery run has two books. Each book lists five observations: two sections
and three interpretations. Pattern p1 cites episode ep1 of each book. Pattern p2 cites
section s002 of each book. Every interpretation lies in section s001.
"""

from __future__ import annotations

import json
import math
from html import unescape
from pathlib import Path
from typing import Any

import pytest
from typer.testing import CliRunner

from tests.test_pattern_discovery import BOOKS, World
from urg.cli import app
from urg.model import RecordedModel
from urg.report import ReportError
from urg.store import StoredArtifact
from urg.weighted_report import (
    BASELINE_PAGE,
    COMPARISON_PAGE,
    SCORES,
    WEIGHTED_PAGE,
    WeightedRun,
    build_weighted_reports,
)
from urg.weighting import (
    Corpus,
    Observation,
    PatternCitations,
    WeightingError,
    baseline_scores,
)

Json = dict[str, Any]
A, B = BOOKS


# --- Helpers --------------------------------------------------------------------------


def write_weights(
    path: Path,
    books: dict[str, float] | None = None,
    sections: dict[str, float] | None = None,
    status: str = "illustrative",
) -> Path:
    path.write_text(
        json.dumps(
            {
                "version": 1,
                "status": status,
                "statement": "Weights for a test.",
                "books": {k: {"weight": v, "reason": "test"} for k, v in (books or {}).items()},
                "sections": {
                    k: {"weight": v, "reason": "test"} for k, v in (sections or {}).items()
                },
            }
        )
    )
    return path


def write_relevance(path: Path, books: dict[str, str] | None = None) -> Path:
    path.write_text(
        json.dumps(
            {
                "version": 1,
                "status": "illustrative",
                "statement": "Relevance for a test.",
                "books": {k: {"level": v, "reason": "test"} for k, v in (books or {}).items()},
            }
        )
    )
    return path


@pytest.fixture
def discovered(tmp_path: Path) -> tuple[World, StoredArtifact]:
    world = World(tmp_path)
    run = world.discover(RecordedModel([world.answer()]))
    return world, run.discovery


def build(
    world: World, discovery: StoredArtifact, weights: Path, relevance: Path | None = None
) -> WeightedRun:
    relevance = relevance or write_relevance(world.root / "relevance.json")
    return build_weighted_reports(world.store, discovery, weights, relevance)


def scores_of(artifact: StoredArtifact) -> Json:
    return json.loads(artifact.read_bytes(SCORES))


def by_local_id(discovery: StoredArtifact, scores: Json) -> dict[str, Json]:
    record = json.loads(discovery.read_bytes("discovery.json"))
    return {p["local_id"]: scores["scores"]["patterns"][p["id"]] for p in record["patterns"]}


def synthetic(lengths: dict[str, int], supporting: dict[str, int]) -> tuple[Corpus, list[str]]:
    """Build a corpus where book b lists lengths[b] observations and supporting[b] support P."""
    observations: dict[str, Observation] = {}
    cited: list[str] = []
    for book, n in lengths.items():
        for i in range(n):
            o = Observation(f"{book}:o{i}", "interpretation", book, "", (f"{book}:s001",))
            observations[o.id] = o
            if i < supporting[book]:
                cited.append(o.id)
    corpus = Corpus(
        books=tuple(lengths),
        observations=observations,
        parents={f"{b}:s001": None for b in lengths},
        defects={},
        chapter_defects={},
        section_titles={},
    )
    return corpus, cited


# --- Equal-book baseline --------------------------------------------------------------


def test_a_longer_chapter_gives_its_book_no_more_influence() -> None:
    short, short_cited = synthetic({"S": 4, "L": 4}, {"S": 1, "L": 2})
    long, long_cited = synthetic({"S": 4, "L": 40}, {"S": 1, "L": 20})
    before = baseline_scores(short, [PatternCitations("P", tuple(short_cited))])
    after = baseline_scores(long, [PatternCitations("P", tuple(long_cited))])
    assert before.shares == after.shares == {"S": 0.5, "L": 0.5}
    assert math.isclose(before.patterns["P"].prevalence, 0.375)
    assert math.isclose(after.patterns["P"].prevalence, 0.375)


def test_a_pattern_of_the_long_book_alone_weighs_the_same_as_one_of_the_short_book() -> None:
    corpus, _ = synthetic({"S": 3, "L": 30}, {"S": 0, "L": 0})
    short = PatternCitations("short", tuple(f"S:o{i}" for i in range(3)))
    long = PatternCitations("long", tuple(f"L:o{i}" for i in range(30)))
    scores = baseline_scores(corpus, [short, long])
    assert math.isclose(scores.patterns["short"].prevalence, 0.5)
    assert math.isclose(scores.patterns["long"].prevalence, 0.5)
    assert scores.ranks() == {"long": 1, "short": 1}


def test_the_baseline_report_gives_each_book_an_equal_share(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    run = build(world, discovery, write_weights(world.root / "w.json", {A: 3.0}))
    scores = scores_of(run.baseline)
    assert scores["scores"]["shares"] == {A: 0.5, B: 0.5}
    assert scores["scores"]["denominators"] == {A: 5.0, B: 5.0}
    patterns = by_local_id(discovery, scores)
    assert math.isclose(patterns["p1"]["prevalence"], 0.2)
    assert math.isclose(patterns["p2"]["prevalence"], 0.2)
    assert patterns["p1"]["uncounted"] == []


# --- Preference weights ---------------------------------------------------------------


def test_preference_weights_follow_the_documented_formula(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", {A: 3.0, B: 1.0}, {f"{B}:s002": 0.0})
    scores = scores_of(build(world, discovery, weights).weighted)
    assert scores["scores"]["shares"] == {A: 0.75, B: 0.25}
    assert scores["scores"]["denominators"] == {A: 5.0, B: 4.0}
    patterns = by_local_id(discovery, scores)
    assert math.isclose(patterns["p1"]["prevalence"], 0.75 * 1 / 5 + 0.25 * 1 / 4)
    assert math.isclose(patterns["p2"]["prevalence"], 0.75 * 1 / 5)
    assert math.isclose(patterns["p2"]["book_coverage"], 0.75)
    assert patterns["p1"]["rank"] == 1


def test_a_section_weight_applies_to_its_descendants(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", sections={f"{A}:s001": 0.0})
    scores = scores_of(build(world, discovery, weights).weighted)
    assert scores["scores"]["excluded"] == {
        A: "no listed observation of this book has a positive weight"
    }
    assert scores["scores"]["shares"] == {A: 0.0, B: 1.0}


def test_a_zero_book_weight_excludes_the_book_and_renormalises(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    scores = scores_of(
        build(world, discovery, write_weights(world.root / "w.json", {A: 0.0})).weighted
    )
    assert scores["scores"]["excluded"] == {A: "the book weight is zero"}
    assert scores["scores"]["shares"] == {A: 0.0, B: 1.0}


def test_weights_that_exclude_every_book_are_refused(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    with pytest.raises(WeightingError):
        build(world, discovery, write_weights(world.root / "w.json", {A: 0.0, B: 0.0}))


def test_entries_for_unanalysed_material_are_listed_as_ignored(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", {"ZZZZ9999": 2.0}, {f"{A}:s999": 0.5})
    scores = scores_of(build(world, discovery, weights).weighted)
    ignored = scores["ignored_weight_entries"]
    assert any("ZZZZ9999" in entry for entry in ignored)
    assert any(f"{A}:s999" in entry for entry in ignored)


def test_an_invalid_weight_file_is_refused(discovered: tuple[World, StoredArtifact]) -> None:
    world, discovery = discovered
    path = world.root / "w.json"
    path.write_text(
        json.dumps(
            {
                "version": 1,
                "status": "illustrative",
                "statement": "x",
                "books": {A: {"weight": -1, "reason": "no"}},
            }
        )
    )
    with pytest.raises(ReportError):
        build(world, discovery, path)


# --- Separate inputs ------------------------------------------------------------------


def test_relevance_never_changes_a_number(discovered: tuple[World, StoredArtifact]) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", {A: 2.0})
    low = build(world, discovery, weights, write_relevance(world.root / "r1.json", {A: "low"}))
    high = build(world, discovery, weights, write_relevance(world.root / "r2.json", {A: "high"}))
    assert low.weighted.record.key != high.weighted.record.key
    assert scores_of(low.weighted)["scores"] == scores_of(high.weighted)["scores"]
    assert scores_of(low.baseline)["scores"] == scores_of(high.baseline)["scores"]


def test_each_input_has_its_own_column(discovered: tuple[World, StoredArtifact]) -> None:
    world, discovery = discovered
    run = build(world, discovery, write_weights(world.root / "w.json"))
    for artifact, page in ((run.baseline, BASELINE_PAGE), (run.weighted, WEIGHTED_PAGE)):
        html = artifact.read_bytes(page).decode()
        for column in (
            "<th>Prevalence</th>",
            "<th>Relevance assessments of supporting instances</th>",
            "<th>Extraction uncertainty</th>",
            "<th>Interpretation uncertainty</th>",
        ):
            assert column in html
        assert "confidence" not in html.lower()
        assert "quality" not in html.lower()


# --- Labels ---------------------------------------------------------------------------


def test_illustrative_weights_are_labelled_as_not_the_humans_preferences(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    run = build(world, discovery, write_weights(world.root / "w.json", {A: 2.0}))
    for page in (WEIGHTED_PAGE, COMPARISON_PAGE):
        text = unescape(run.weighted.read_bytes(page).decode())
        assert "are not the human's preferences" in text
    manifest = scores_of(run.weighted)["preference_weights"]
    assert manifest["status"] == "illustrative"
    assert manifest["described_as"] == "illustrative weights, not the human's preferences"


def test_human_weights_carry_no_illustrative_label(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", {A: 2.0}, status="human")
    run = build(world, discovery, weights)
    html = run.weighted.read_bytes(WEIGHTED_PAGE).decode()
    assert "Illustrative preference weights" not in html
    assert "Preference weights supplied by the human" in html


def test_each_report_says_it_reweights_and_names_the_discovery_run(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    run = build(world, discovery, write_weights(world.root / "w.json"))
    pages = [
        (run.baseline, BASELINE_PAGE),
        (run.weighted, WEIGHTED_PAGE),
        (run.weighted, COMPARISON_PAGE),
    ]
    for artifact, page in pages:
        html = artifact.read_bytes(page).decode()
        assert "This report reweights fixed patterns." in html
        assert discovery.record.key in html
    for artifact in (run.baseline, run.weighted):
        manifest = scores_of(artifact)
        assert manifest["kind"] == "reweighting"
        assert manifest["discovery_artifact"] == discovery.record.key


# --- Reuse ----------------------------------------------------------------------------


def test_a_weight_change_reuses_every_upstream_artifact(
    discovered: tuple[World, StoredArtifact],
) -> None:
    world, discovery = discovered
    relevance = write_relevance(world.root / "relevance.json")
    first = build(world, discovery, write_weights(world.root / "w1.json", {A: 2.0}), relevance)
    second = build(world, discovery, write_weights(world.root / "w2.json", {A: 0.5}), relevance)
    assert not first.baseline_reused and not first.weighted_reused
    assert second.baseline_reused and not second.weighted_reused
    assert second.baseline.record.key == first.baseline.record.key
    assert second.weighted.record.key != first.weighted.record.key
    assert second.upstream == first.upstream

    def roles(artifact: StoredArtifact) -> dict[str, set[str]]:
        found: dict[str, set[str]] = {}
        for i in artifact.record.dependency.inputs:
            found.setdefault(i.role, set()).add(i.identity)
        return found

    one, two = roles(first.weighted), roles(second.weighted)
    for role in ("pattern-discovery", "chapter-extraction", "episode-analysis", "baseline-report"):
        assert one[role] == two[role]
    assert one["pattern-discovery"] == {discovery.record.key}
    assert one["baseline-report"] == {first.baseline.record.key}
    assert one["preference-weights"] != two["preference-weights"]
    assert "preference-weights" not in roles(first.baseline)


def test_the_same_weights_reuse_both_reports(discovered: tuple[World, StoredArtifact]) -> None:
    world, discovery = discovered
    weights = write_weights(world.root / "w.json", {A: 2.0})
    first = build(world, discovery, weights)
    again = build(world, discovery, weights)
    assert again.baseline_reused and again.weighted_reused
    assert again.weighted.record.key == first.weighted.record.key


def test_the_command_builds_then_reuses(discovered: tuple[World, StoredArtifact]) -> None:
    world, _ = discovered
    args = [
        "weighted-report",
        "--weights", str(write_weights(world.root / "w.json", {A: 2.0})),
        "--relevance", str(write_relevance(world.root / "relevance.json")),
        "--data-dir", str(world.root / "data"),
    ]  # fmt: skip
    runner = CliRunner()
    first = runner.invoke(app, args)
    assert first.exit_code == 0, first.output
    assert "built baseline report" in first.output
    assert "built weighted report" in first.output
    assert "reused 5 upstream research artifacts" in first.output
    second = runner.invoke(app, args)
    assert second.exit_code == 0, second.output
    assert "reused baseline report" in second.output
    assert "reused weighted report" in second.output
