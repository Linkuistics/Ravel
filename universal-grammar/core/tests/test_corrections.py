"""Tests for the corrections stage and the change summary."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest
from typer.testing import CliRunner

from tests.test_pattern_discovery import BOOKS, World, line_id, write_versioned
from urg.analysis import ANALYSIS, run_analysis
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.changes import ChangeSummaryError, render_summary, summarise_changes
from urg.cli import app
from urg.corrections import run_corrections
from urg.discovery import DiscoveryError, run_discovery
from urg.model import RecordedModel
from urg.report import INTERPRETATIONS, build_report

Json = dict[str, Any]
A, B = BOOKS


def answer_with_relationship(key: str) -> str:
    interpretations: list[Json] = [
        {
            "local_id": "ep1",
            "kind": "episode",
            "label": "example before name",
            "description": "A loop is shown, then named.",
            "evidence": [f"{key}:e0002", f"{key}:e0003", f"{key}:e0004"],
            "uncertainty": {"level": "medium", "reason": "The order implies the purpose."},
        },
        {
            "local_id": "ep2",
            "kind": "episode",
            "label": "errors",
            "description": "The errors close the topic.",
            "evidence": [f"{key}:e0005", f"{key}:e0006"],
            "uncertainty": {"level": "low", "reason": "A heading names them."},
        },
        {
            "local_id": "mv1",
            "kind": "move",
            "episode": "ep1",
            "label": "states the problem",
            "description": "The input is one string.",
            "evidence": [f"{key}:e0002"],
            "uncertainty": {"level": "low", "reason": "Stated directly."},
        },
        {
            "local_id": "rel1",
            "kind": "relationship",
            "source": "ep1",
            "target": "ep2",
            "label": "motivates",
            "description": "The example motivates the errors.",
            "evidence": [f"{key}:e0005"],
            "uncertainty": {"level": "medium", "reason": "Implied by order."},
        },
        {
            "local_id": "obs1",
            "kind": "observation",
            "label": None,
            "description": "The name comes after the code.",
            "evidence": [line_id(key, 4)],
            "uncertainty": {"level": "low", "reason": "Visible in the order."},
        },
    ]
    return json.dumps({"interpretations": interpretations})


class Corrections:
    """A world with a corrections directory."""

    def __init__(self, root: Path) -> None:
        self.world = World(root, answer_with_relationship)
        self.store = self.world.store
        self.directory = root / "corrections"
        self.directory.mkdir()

    def id(self, key: str, local_id: str) -> str:
        return self.world.interpretation(key, local_id)

    def write(self, name: str, interpretation: str, version: int = 1, **change: Any) -> Path:
        correction: Json = {
            "id": name,
            "author": {"kind": "human", "name": "Tester"},
            "origin": "close reading",
            "rationale": f"Rationale of {name}.",
            "target": {"id": interpretation, "version": version},
        }
        if "reading" in change:
            correction |= {"operation": "add-alternative-reading", "reading": change["reading"]}
        else:
            correction |= {"operation": "revise", "revise": change}
        path = self.directory / f"{name}.json"
        path.write_text(json.dumps(correction))
        return path

    def move_boundary(self) -> Path:
        """Move the end of episode 1 of book A one element later."""
        evidence = [f"{A}:e0002", f"{A}:e0003", f"{A}:e0004", f"{A}:e0005"]
        uncertainty = {"level": "high", "reason": "The next element can belong to either."}
        return self.write(
            "c01-boundary", self.id(A, "ep1"), evidence=evidence, uncertainty=uncertainty
        )

    def analysis(self, key: str) -> Json:
        return json.loads(self.store.load(ANALYSIS, key).read_bytes(ANALYSIS_FILE))

    def interpretation(self, key: str, item_key: str, local_id: str) -> Json:
        wanted = self.id(item_key, local_id)
        return next(i for i in self.analysis(key)["interpretations"] if i["id"] == wanted)

    def keys(self, name: str) -> set[str]:
        directory = self.store.root / "artifacts" / name
        return {p.name for p in directory.iterdir() if p.name != "latest"}


@pytest.fixture
def corrections(tmp_path: Path) -> Corrections:
    return Corrections(tmp_path)


def results(run: Any) -> dict[str, Json]:
    report = json.loads(run.run.read_bytes("report.json"))
    return {r["item_key"]: r for r in report["results"]}


def test_a_boundary_correction_creates_a_new_version(corrections: Corrections) -> None:
    corrections.move_boundary()
    run = run_corrections(corrections.store, corrections.directory)

    assert run.refused == []
    by_book = results(run)
    assert by_book[A]["corrected"]
    assert by_book[A]["analysis_key"] != by_book[A]["base_analysis_key"]
    assert not by_book[B]["corrected"]
    assert by_book[B]["analysis_key"] == by_book[B]["base_analysis_key"]

    new = corrections.interpretation(by_book[A]["analysis_key"], A, "ep1")
    assert new["version"] == 2
    assert new["review_status"] == "corrected"
    assert [e["id"] for e in new["evidence"]][-1] == f"{A}:e0005"
    assert new["correction"]["id"] == "c01-boundary"
    assert new["correction"]["revises_version"] == 1
    assert new["author"] == {"kind": "human", "name": "Tester"}

    record = corrections.analysis(by_book[A]["analysis_key"])
    [old] = record["history"]
    assert (old["version"], old["superseded_by"]) == (1, "c01-boundary")
    assert record["revision"]["base_analysis_artifact"] == by_book[A]["base_analysis_key"]


def test_the_original_version_stays_readable(corrections: Corrections) -> None:
    corrections.move_boundary()
    run = run_corrections(corrections.store, corrections.directory)
    base = results(run)[A]["base_analysis_key"]

    original = corrections.interpretation(base, A, "ep1")
    assert original["version"] == 1
    assert original["review_status"] == "proposed"
    assert len(original["evidence"]) == 3


def test_a_correction_can_change_a_label_a_relationship_and_add_a_reading(
    corrections: Corrections,
) -> None:
    corrections.write("c01-label", corrections.id(B, "ep2"), label="error handling")
    corrections.write(
        "c02-relationship",
        corrections.id(A, "rel1"),
        label="prepares",
        target=f"{A}:e0006",
    )
    corrections.write(
        "c03-reading",
        corrections.id(B, "ep1"),
        reading={
            "label": "worked example",
            "description": "The loop is a worked example, not a motivation.",
            "evidence": [f"{B}:e0003"],
            "uncertainty": {"level": "high", "reason": "The text does not say."},
        },
    )
    run = run_corrections(corrections.store, corrections.directory)

    assert run.refused == []
    by_book = results(run)
    label = corrections.interpretation(by_book[B]["analysis_key"], B, "ep2")
    assert (label["label"], label["version"]) == ("error handling", 2)

    relationship = corrections.interpretation(by_book[A]["analysis_key"], A, "rel1")
    assert relationship["label"] == "prepares"
    assert relationship["target"] == {"layer": "structure-element", "id": f"{A}:e0006"}
    assert relationship["source"]["id"] == corrections.id(A, "ep1")

    record = corrections.analysis(by_book[B]["analysis_key"])
    reading = next(i for i in record["interpretations"] if i["kind"] == "alternative-reading")
    assert reading["id"] == f"{B}:c03-reading:i001"
    assert reading["alternative_to"] == {"layer": "interpretation", "id": corrections.id(B, "ep1")}
    assert reading["review_status"] == "corrected"
    assert record["counts"]["alternative-reading"] == 1


def test_a_stale_target_version_is_refused(corrections: Corrections) -> None:
    target = corrections.id(A, "ep1")
    corrections.write("c01-first", target, label="first")
    corrections.write("c02-stale", target, version=1, label="stale")
    corrections.write("c03-next", target, version=2, label="next")
    run = run_corrections(corrections.store, corrections.directory)

    [refused] = run.refused
    assert refused["id"] == "c02-stale"
    assert "target version 1" in refused["reason"]
    assert "current version 2" in refused["reason"]
    current = corrections.interpretation(results(run)[A]["analysis_key"], A, "ep1")
    assert (current["label"], current["version"]) == ("next", 3)

    result = CliRunner().invoke(
        app,
        [
            "corrections",
            "--corrections-dir",
            str(corrections.directory),
            "--data-dir",
            str(corrections.store.root),
        ],
    )
    assert result.exit_code == 1


def test_an_invalid_or_unknown_correction_is_refused(corrections: Corrections) -> None:
    (corrections.directory / "c01-broken.json").write_text("{not json")
    corrections.write("c02-unknown", f"{A}:nothing:i001", label="x")
    corrections.write("c03-evidence", corrections.id(A, "ep1"), evidence=[f"{A}:e9999"])
    corrections.write("c04-reference", corrections.id(A, "ep1"), episode=corrections.id(A, "ep2"))
    run = run_corrections(corrections.store, corrections.directory)

    reasons = {o["file"].rsplit("/", 1)[-1]: o["reason"] for o in run.refused}
    assert reasons["c01-broken.json"].startswith("invalid correction file")
    assert "no analysed book" in reasons["c02-unknown.json"]
    assert "e9999" in reasons["c03-evidence.json"]
    assert "has no 'episode' reference" in reasons["c04-reference.json"]
    assert run.corrected == []


def test_a_move_can_only_belong_to_an_episode(corrections: Corrections) -> None:
    move = corrections.id(A, "mv1")
    corrections.write("c01-relationship", move, episode=corrections.id(A, "rel1"))
    corrections.write("c02-observation", move, episode=corrections.id(A, "obs1"))
    corrections.write("c03-episode", move, episode=corrections.id(A, "ep2"))
    run = run_corrections(corrections.store, corrections.directory)

    reasons = {o["id"]: o["reason"] for o in run.refused}
    assert reasons == {
        "c01-relationship": f"'episode' names {corrections.id(A, 'rel1')}, which is a relationship",
        "c02-observation": f"'episode' names {corrections.id(A, 'obs1')}, which is a observation",
    }
    current = corrections.interpretation(results(run)[A]["analysis_key"], A, "mv1")
    assert current["episode"] == {"layer": "interpretation", "id": corrections.id(A, "ep2")}
    assert current["version"] == 2


def test_a_rerun_with_the_same_corrections_reuses_every_artifact(
    corrections: Corrections,
) -> None:
    corrections.move_boundary()
    first = run_corrections(corrections.store, corrections.directory)
    second = run_corrections(corrections.store, corrections.directory)

    assert second.reused
    assert second.run.record.key == first.run.record.key
    assert [c.reused for c in second.corrected] == [True]


def test_a_correction_regenerates_only_the_dependent_results(corrections: Corrections) -> None:
    world = corrections.world
    before = world.discover(RecordedModel([world.answer()]))
    extractions = corrections.keys("chapter-extraction")
    responses = corrections.keys("episode-analysis-response")

    corrections.move_boundary()
    run = run_corrections(corrections.store, corrections.directory)
    answer = json.loads(world.answer())
    p1 = answer["patterns"][0]
    p1["unresolved"] = []
    p1["supporting"].append({"observation": corrections.id(A, "mv1"), "note": "In ep1."})
    after = world.discover(RecordedModel([json.dumps(answer)]))

    # Neither extraction nor episode analysis ran again.
    assert corrections.keys("chapter-extraction") == extractions
    assert corrections.keys("episode-analysis-response") == responses
    assert not after.response_reused
    assert after.discovery.record.key != before.discovery.record.key
    sources = {s["item_key"]: s for s in after.record["sources"]}
    assert sources[A]["analysis_artifact"] == results(run)[A]["analysis_key"]
    assert sources[B]["analysis_artifact"] == results(run)[B]["base_analysis_key"]

    summary = summarise_changes(corrections.store).record
    status = {row["artifact"]: row["status"] for row in summary["artifacts"]}
    assert status == {
        f"chapter-extraction {A}": "reused",
        f"episode-analysis-response {A}": "reused",
        f"episode-analysis {A}": "changed",
        f"chapter-extraction {B}": "reused",
        f"episode-analysis-response {B}": "reused",
        f"episode-analysis {B}": "reused",
        "pattern-discovery-response": "changed",
        "pattern-discovery": "changed",
        "pattern-report": "changed",
    }
    assert summary["before"]["discovery"] == before.discovery.record.key
    assert INTERPRETATIONS in summary["report_files"]["changed"]
    assert summary["report_files"]["reused"]

    [corrected] = summary["interpretations"]
    assert corrected["id"] == corrections.id(A, "ep1")
    assert {"field": "evidence", "added": [f"{A}:e0005"], "removed": []} in corrected["changes"]
    assert corrections.id(A, "mv1") in summary["affected"]
    assert corrections.id(A, "rel1") in summary["affected"]
    assert corrections.id(B, "ep1") not in summary["affected"]

    [changed] = summary["patterns"]["matched_changed"]
    causes = {d.get("added") or d.get("removed"): d["cause"] for d in changed["differences"]}
    assert causes == {corrections.id(A, "mv1"): "correction", corrections.id(B, "obs1"): "rerun"}
    assert len(summary["patterns"]["matched_unchanged"]) == 1
    assert summary["causes"]["correction"] >= 1
    assert summary["causes"]["rerun"] >= 1

    again = summarise_changes(corrections.store)
    assert again.reused


def test_the_report_shows_the_version_and_the_superseded_version(
    corrections: Corrections,
) -> None:
    world = corrections.world
    corrections.move_boundary()
    run_corrections(corrections.store, corrections.directory)
    discovered = world.discover(RecordedModel([world.answer()]))
    report = build_report(corrections.store, discovered.discovery).report
    page = report.read_bytes(INTERPRETATIONS).decode()

    assert "Version 2." in page
    assert "c01-boundary" in page
    assert "Rationale of c01-boundary." in page
    assert "Version 1" in page


def test_discovery_refuses_a_corrections_run_for_an_older_analysis(
    corrections: Corrections,
) -> None:
    world = corrections.world
    corrections.move_boundary()
    run_corrections(corrections.store, corrections.directory)
    instructions = world.root / "analysis.md"
    instructions.write_text("---\nversion: 2\n---\nFind episodes again.\n")
    model = RecordedModel([answer_with_relationship(key) for key in BOOKS])
    run_analysis(corrections.store, instructions, model)

    with pytest.raises(DiscoveryError, match="Run corrections again"):
        world.discover(RecordedModel([world.answer()]))


def test_the_change_summary_needs_a_run_before_the_corrections(
    corrections: Corrections,
) -> None:
    world = corrections.world
    corrections.move_boundary()
    run_corrections(corrections.store, corrections.directory)
    world.discover(RecordedModel([world.answer()]))

    with pytest.raises(ChangeSummaryError, match="pass --before"):
        summarise_changes(corrections.store)


def summary_over(corrections: Corrections, answer: Json | None = None) -> Json:
    """Discover before and after the written corrections, then summarise the changes."""
    world = corrections.world
    world.discover(RecordedModel([world.answer()]))
    run = run_corrections(corrections.store, corrections.directory)
    assert run.refused == []
    after = answer if answer is not None else json.loads(world.answer())
    world.discover(RecordedModel([json.dumps(after)]))
    return summarise_changes(corrections.store).record


def test_the_summary_spans_every_correction_since_the_base_analysis(
    corrections: Corrections,
) -> None:
    corrections.move_boundary()
    corrections.write("c02-label", corrections.id(A, "ep1"), version=2, label="renamed")
    summary = summary_over(corrections)

    [entry] = summary["interpretations"]
    assert (entry["before_version"], entry["after_version"]) == (1, 3)
    assert [c["id"] for c in entry["corrections"]] == ["c01-boundary", "c02-label"]
    assert [c["revises_version"] for c in entry["corrections"]] == [1, 2]
    assert entry["corrections"][0]["author"] == {"kind": "human", "name": "Tester"}
    assert entry["corrections"][0]["rationale"] == "Rationale of c01-boundary."
    fields = {c["field"]: c for c in entry["changes"]}
    assert fields["evidence"] == {"field": "evidence", "added": [f"{A}:e0005"], "removed": []}
    assert fields["label"]["after"] == "renamed"
    assert f"{A}:e0005" in summary["affected"]
    assert corrections.id(A, "mv1") in summary["affected"]

    page = render_summary(summary)
    assert "version 1 to 3" in page
    assert "correction `c01-boundary` by Tester (close reading)" in page
    assert "correction `c02-label` by Tester (close reading)" in page


def test_a_revised_alternative_reading_keeps_its_creation(corrections: Corrections) -> None:
    corrections.write(
        "c01-reading",
        corrections.id(B, "ep1"),
        reading={
            "label": "worked example",
            "description": "The loop is a worked example.",
            "evidence": [f"{B}:e0003"],
            "uncertainty": {"level": "high", "reason": "The text does not say."},
        },
    )
    reading = f"{B}:c01-reading:i001"
    corrections.write("c02-revise", reading, label="exercise")
    summary = summary_over(corrections)

    [entry] = summary["interpretations"]
    assert entry["id"] == reading
    assert (entry["before_version"], entry["after_version"]) == (None, 2)
    assert [c["id"] for c in entry["corrections"]] == ["c01-reading", "c02-revise"]
    fields = {c["field"]: c for c in entry["changes"]}
    assert fields["alternative_to"]["after"]["id"] == corrections.id(B, "ep1")
    assert fields["label"] == {"field": "label", "before": None, "after": "exercise"}
    assert fields["evidence"]["added"] == [f"{B}:e0003"]
    assert corrections.id(B, "ep1") in summary["affected"]
    assert "version none (added) to 2" in render_summary(summary)


def test_the_summary_compares_representation_and_notes(corrections: Corrections) -> None:
    corrections.move_boundary()
    answer = json.loads(corrections.world.answer())
    p1, p2, _ = answer["patterns"]
    p1["representation"] = [
        {"feature": "cross-reference", "justification": "The name points back."}
    ]
    p1["supporting"][0]["note"] = "Loop, then name, then more."
    p2["supporting"][0]["note"] = "Errors at the end of Scanning."
    summary = summary_over(corrections, answer)

    changed = {p["name"]: p["differences"] for p in summary["patterns"]["matched_changed"]}
    assert changed["Example before name"] == [
        {"role": "supporting", "note": corrections.id(A, "ep1"), "cause": "either"},
        {"field": "representation", "cause": "either"},
    ]
    assert changed["Errors under the main section"] == [
        {"role": "supporting", "note": f"{A}:s002", "cause": "rerun"},
    ]
    page = render_summary(summary)
    assert "representation field" in page
    assert f"supporting note {A}:s002" in page


@pytest.mark.parametrize("change", ["instructions", "model", "analyses"])
def test_a_named_run_before_must_match_the_later_run(corrections: Corrections, change: str) -> None:
    world = corrections.world
    instructions = world.instructions
    model = RecordedModel([world.answer()])
    if change == "instructions":
        instructions = write_versioned(world.root / "other.md", "Find other patterns.")
    if change == "model":
        model = RecordedModel([world.answer()], identity={"harness": "recorded", "model": "x"})
    corrections.move_boundary()
    if change != "analyses":
        run_corrections(corrections.store, corrections.directory)
    before = run_discovery(corrections.store, world.settings, instructions, model)
    if change == "analyses":
        run_corrections(corrections.store, corrections.directory)
        before = world.discover(RecordedModel([world.answer()], identity={"harness": "x"}))
    world.discover(RecordedModel([world.answer()]))

    reason = {
        "instructions": "Its instructions differ",
        "model": "Its model differ",
        "analyses": "does not read the uncorrected analyses",
    }[change]
    with pytest.raises(ChangeSummaryError, match=reason):
        summarise_changes(corrections.store, before_key=before.discovery.record.key)
