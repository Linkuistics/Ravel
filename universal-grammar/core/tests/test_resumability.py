"""Resumability of the whole pipeline, run through the stage commands on fixtures.

Each test supports one claim of the resumability leaf. ``docs/resumability.md`` maps each
claim to its test and to its section of the real-data transcript.

A test compares the store before and after a rerun through the ``written_at`` time of each
record. An unchanged time shows that the rerun reused the artifact. A new key shows new work.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from collections import Counter
from collections.abc import Callable
from pathlib import Path
from typing import Any

import pytest

from tests.pipeline import REPO, A, B, Pipeline, write_weights
from urg import analysis, corrections, discovery, extraction, manifest, report, weighted_report
from urg.store import RECORD_FILE, ArtifactStore, InterruptedWrite, StoredArtifact

Json = dict[str, Any]
Artifacts = dict[tuple[str, str], str]

#: The artifacts of a complete fixture run, by name, with the number of each.
COMPLETE_RUN = {
    "export-snapshot": 1,
    "source-manifest": 1,
    "alias-table": 1,
    "chapter-extraction": 2,
    "chapter-extraction-run": 1,
    "episode-analysis-response": 2,
    "episode-analysis": 2,
    "episode-analysis-run": 1,
    "analysis-corrections-run": 1,
    "pattern-discovery-response": 1,
    "pattern-discovery": 1,
    "pattern-report": 1,
    "baseline-report": 1,
    "weighted-report": 1,
}

#: The artifacts downstream of the analyses, which a new analysis makes new.
AFTER_ANALYSIS = [
    "episode-analysis-run",
    "analysis-corrections-run",
    "pattern-discovery-response",
    "pattern-discovery",
    "pattern-report",
    "baseline-report",
    "weighted-report",
]
AFTER_DISCOVERY_INPUTS = AFTER_ANALYSIS[2:]
NEW_ANALYSIS = {
    "episode-analysis-response": 2,
    "episode-analysis": 2,
    **dict.fromkeys(AFTER_ANALYSIS, 1),
}


def new_since(before: Artifacts, after: Artifacts) -> Counter[str]:
    """Count the artifacts, by name, that are in AFTER and not in BEFORE."""
    return Counter(name for (name, artifact_key) in after if (name, artifact_key) not in before)


def records(pipeline: Pipeline, name: str) -> list[StoredArtifact]:
    store = pipeline.store
    return [store.load(n, k) for (n, k) in pipeline.artifacts() if n == name]


def roles(artifact: StoredArtifact) -> Counter[str]:
    return Counter(i.role for i in artifact.record.dependency.inputs)


def interpretation_id(pipeline: Pipeline, item_key: str, local: int) -> str:
    for artifact in records(pipeline, "episode-analysis"):
        record = json.loads(artifact.read_bytes(analysis.RECORD_FILE))
        for interpretation in record["interpretations"]:
            if interpretation["id"].startswith(f"{item_key}:"):
                if interpretation["id"].endswith(f":i{local:03d}"):
                    return interpretation["id"]
    raise AssertionError(f"no interpretation {local} for {item_key}")


def write_correction(pipeline: Pipeline, target: str) -> Path:
    path = pipeline.corrections / "c001-fixture.json"
    correction = {
        "id": "c001-fixture",
        "author": {"kind": "human", "name": "Fixture reviewer"},
        "origin": "the resumability test",
        "rationale": "The episode also introduces the topic.",
        "target": {"id": target, "version": 1},
        "operation": "revise",
        "revise": {"description": "The chapter opens with its topic and introduces it."},
    }
    path.write_text(json.dumps(correction))
    return path


CODE_VERSION = re.compile(r"urg \S+ source-sha256:[0-9a-f]{64}")


@pytest.fixture
def complete(tmp_path: Path) -> Pipeline:
    """A fixture pipeline after one complete run."""
    pipeline = Pipeline(tmp_path)
    pipeline.run()
    assert Counter(name for name, _ in pipeline.artifacts()) == COMPLETE_RUN
    return pipeline


# --- Claim 1: each dependency record names each input that applies to it --------------


def test_every_dependency_record_names_its_inputs(complete: Pipeline) -> None:
    write_correction(complete, interpretation_id(complete, A, 1))
    complete.run()

    for name, artifact_key in complete.artifacts():
        dependency = complete.store.load(name, artifact_key).record.dependency
        assert dependency.schema_version >= 1, name
        # The code version names the package and the hash of the stage's module closure, so
        # that a code change invalidates the outputs of the stages whose modules it changes.
        assert CODE_VERSION.fullmatch(dependency.code_version), (name, dependency.code_version)

    # Source identity: the export, the database and the storage files, then each PDF.
    (source_manifest,) = records(complete, "source-manifest")
    assert roles(source_manifest) >= Counter(
        {"rdf-export": 1, "zotero-database": 1, "zotero-storage": 1}
    )
    assert source_manifest.record.dependency.schema_version == manifest.SCHEMA_VERSION

    # Extraction code and configuration, with the source identity of the PDF.
    for chapter in records(complete, "chapter-extraction"):
        dependency = chapter.record.dependency
        assert roles(chapter) == {"document-file": 1, "chapter-selection-entry": 1}
        assert {"settings", "text_flags", "source_identity", "chapter"} <= set(
            dependency.configuration
        )
        assert dependency.schema_version == extraction.SCHEMA_VERSION

    # Analysis instructions and model configuration.
    proposed = [
        a
        for a in records(complete, "episode-analysis")
        if a.record.dependency.stage == analysis.STAGE
    ]
    assert len(proposed) == 2
    for artifact in proposed:
        dependency = artifact.record.dependency
        assert roles(artifact) == {
            "analysis-instructions": 1,
            "chapter-extraction": 1,
            "model-response": 1,
        }
        assert dependency.configuration["model"]["effort"] == "high"
        assert dependency.configuration["instructions_version"] == 1
        assert dependency.schema_version == analysis.SCHEMA_VERSION

    # Run reports: each one names the configuration of its stage and the run that it reads.
    for run in records(complete, extraction.RUN_REPORT):
        assert roles(run) == {"chapter-selection": 1, "source-manifest": 1}
    for run in records(complete, analysis.RUN_REPORT):
        assert roles(run) == {"analysis-instructions": 1, "chapter-extraction-run": 1}
    runs = records(complete, corrections.RUN_REPORT)
    assert {roles(run)["correction"] for run in runs} == {0, 1}
    for run in runs:
        assert roles(run) - Counter({"correction": 1}) == {"episode-analysis-run": 1}

    # Interpretation versions and corrections: the corrected analysis names its base analysis
    # and the correction file, and records the corrections that it applied.
    (corrected,) = [
        a
        for a in records(complete, "episode-analysis")
        if a.record.dependency.stage == corrections.STAGE
    ]
    assert roles(corrected) == {"base-analysis": 1, "correction": 1}
    assert corrected.record.dependency.configuration["corrections"]
    assert corrected.record.dependency.schema_version == corrections.SCHEMA_VERSION

    # Discovery settings, with the analyses that discovery reads.
    newest = complete.store.latest_key(discovery.DISCOVERY)
    found = complete.store.load(discovery.DISCOVERY, newest or "")
    assert roles(found) == {
        "discovery-settings": 1,
        "discovery-instructions": 1,
        "episode-analysis": 2,
        "chapter-extraction": 2,
        "model-response": 1,
    }
    analysed = {i.identity for i in found.record.dependency.inputs if i.role == "episode-analysis"}
    assert corrected.record.key in analysed
    assert {"settings", "preference_weights", "model"} <= set(found.record.dependency.configuration)
    assert found.record.dependency.schema_version == discovery.SCHEMA_VERSION

    # Weight configuration.
    for weighted in records(complete, "weighted-report"):
        assert roles(weighted)["preference-weights"] == 1
        assert weighted.record.dependency.schema_version == weighted_report.SCHEMA_VERSION
    for pattern_report in records(complete, "pattern-report"):
        assert pattern_report.record.dependency.schema_version == report.SCHEMA_VERSION


def edit_analysis_instructions(pipeline: Pipeline) -> None:
    path = pipeline.analysis_instructions
    path.write_text(path.read_text() + "\nA sentence that changes the instructions.\n")


def edit_model_configuration(pipeline: Pipeline) -> None:
    path = pipeline.model_config
    path.write_text(json.dumps(json.loads(path.read_text()) | {"effort": "medium"}))


def edit_discovery_settings(pipeline: Pipeline) -> None:
    path = pipeline.settings
    settings = json.loads(path.read_text())
    settings["preference_weights"]["books"] = {A: 2.0}
    path.write_text(json.dumps(settings))


def edit_weights(pipeline: Pipeline) -> None:
    write_weights(pipeline.weights, 3.0)


@pytest.mark.parametrize(
    ("edit", "expected", "model_calls"),
    [
        pytest.param(
            edit_analysis_instructions,
            NEW_ANALYSIS,
            ["episode-analysis", "episode-analysis", "pattern-discovery"],
            id="analysis-instructions",
        ),
        pytest.param(
            edit_model_configuration,
            NEW_ANALYSIS,
            ["episode-analysis", "episode-analysis", "pattern-discovery"],
            id="model-configuration",
        ),
        pytest.param(
            edit_discovery_settings,
            dict.fromkeys(AFTER_DISCOVERY_INPUTS, 1),
            ["pattern-discovery"],
            id="discovery-settings",
        ),
        pytest.param(edit_weights, {"weighted-report": 1}, [], id="weights"),
    ],
)
def test_a_change_to_one_input_redoes_only_its_dependants(
    complete: Pipeline,
    edit: Callable[[Pipeline], None],
    expected: dict[str, int],
    model_calls: list[str],
) -> None:
    before = complete.artifacts()
    calls = len(complete.calls)
    edit(complete)
    complete.run()
    assert new_since(before, complete.artifacts()) == expected
    assert complete.calls[calls:] == model_calls


# --- Claim 2: a restart after a complete run repeats no valid work --------------------


def test_a_restart_after_a_complete_run_repeats_no_work(complete: Pipeline) -> None:
    before = complete.artifacts()
    calls = len(complete.calls)
    outputs = complete.run()
    assert complete.artifacts() == before
    assert complete.calls[calls:] == []
    assert "new response" not in outputs["episode-analysis"]
    assert "new response" not in outputs["pattern-discovery"]


def test_a_return_to_earlier_inputs_reuses_their_results_downstream(complete: Pipeline) -> None:
    first = complete.artifacts()
    latest = {name: complete.store.latest_key(name) for name, _ in first}
    original = complete.analysis_instructions.read_text()
    edit_analysis_instructions(complete)
    complete.run()
    second = complete.artifacts()
    complete.analysis_instructions.write_text(original)
    calls = len(complete.calls)

    complete.run()

    assert complete.artifacts() == second
    assert complete.calls[calls:] == []
    # Each later stage reads its upstream run through the latest pointer. The pointers must
    # name the reused results of the original inputs, not the results of the edited ones.
    assert {name: complete.store.latest_key(name) for name, _ in first} == latest


# --- Claim 3: a stage killed during a write leaves an output that the store rejects -----


def run_killed(pipeline: Pipeline, stage: str, point: str, target: str) -> None:
    """Run the stage command in a child process that dies with SIGKILL at POINT."""
    (command,) = [c for c in pipeline.commands() if c[0] == stage]
    child = subprocess.run(
        [sys.executable, "-m", "tests.pipeline", "kill", point, target, "--", *command],
        cwd=REPO,
        capture_output=True,
        check=False,
    )
    assert child.returncode == -9, child.stderr.decode()


@pytest.mark.parametrize(
    ("stage", "point", "target", "model_calls"),
    [
        pytest.param(
            "episode-analysis",
            "mid-payload",
            f"{analysis.RECORD_FILE}@2",
            [],
            id="analysis-mid-payload",
        ),
        pytest.param(
            "chapter-extraction",
            "before-rename",
            "chapter-extraction@2",
            [],
            id="extraction-before-rename",
        ),
        pytest.param(
            "chapter-extraction",
            "before-latest",
            "chapter-extraction-run",
            [],
            id="extraction-run-before-latest",
        ),
        pytest.param(
            "pattern-discovery",
            "mid-payload",
            discovery.RECORD_FILE,
            [],
            id="discovery-mid-payload",
        ),
    ],
)
def test_a_killed_stage_leaves_a_rejected_output_and_a_rerun_completes_it(
    tmp_path: Path, stage: str, point: str, target: str, model_calls: list[str]
) -> None:
    pipeline = Pipeline(tmp_path)
    commands = pipeline.commands()
    position = [c[0] for c in commands].index(stage)
    for command in commands[:position]:
        pipeline.stage(command)

    run_killed(pipeline, stage, point, target)

    killed = pipeline.artifacts()
    staging = pipeline.data / "staging"
    if point == "before-latest":
        # The artifact is complete and in place, but no later stage can see it yet.
        assert pipeline.store.latest_key(target) is None
        assert any(name == target for name, _ in killed)
    else:
        # The partial output stays in staging, which the store never reads. A kill during a
        # payload leaves no record. A kill before the rename leaves a whole copy in staging.
        (partial,) = [d for d in staging.iterdir() if not d.name.startswith("damaged-")]
        assert (partial / RECORD_FILE).exists() == (point == "before-rename")
        written = Counter(name for name, _ in killed)
        artifact = target.partition("@")[0] if point == "before-rename" else stage
        assert written[artifact] < COMPLETE_RUN[artifact]

    calls = len(pipeline.calls)
    for command in commands[position:]:
        pipeline.stage(command)

    after = pipeline.artifacts()
    assert Counter(name for name, _ in after) == COMPLETE_RUN
    # The completed work of the killed run is reused, not repeated.
    assert all(after[k] == killed[k] for k in killed)
    if stage != "pattern-discovery" or point != "mid-payload":
        assert pipeline.calls[calls:].count(stage) == len(model_calls)
    else:
        # The response was stored before the kill, so the rerun does not call the model.
        assert "pattern-discovery" not in pipeline.calls[calls:]


# --- Claim 4: the store rejects an output that is incomplete or corrupted ----------------


def truncate_payload(directory: Path) -> None:
    path = directory / analysis.RECORD_FILE
    path.write_bytes(path.read_bytes()[:-10])


def flip_a_byte(directory: Path) -> None:
    path = directory / analysis.RECORD_FILE
    data = bytearray(path.read_bytes())
    data[len(data) // 2] ^= 0x01
    path.write_bytes(bytes(data))


def delete_payload(directory: Path) -> None:
    (directory / analysis.SUMMARY_FILE).unlink()


def delete_record(directory: Path) -> None:
    (directory / RECORD_FILE).unlink()


def garble_record(directory: Path) -> None:
    path = directory / RECORD_FILE
    path.write_bytes(path.read_bytes()[:40])


def change_recorded_dependency(directory: Path) -> None:
    path = directory / RECORD_FILE
    record = json.loads(path.read_bytes())
    record["dependency"]["configuration"]["instructions_version"] = 99
    path.write_text(json.dumps(record))


@pytest.mark.parametrize(
    "damage",
    [
        truncate_payload,
        flip_a_byte,
        delete_payload,
        delete_record,
        garble_record,
        change_recorded_dependency,
    ],
)
def test_the_store_rejects_a_damaged_output_and_a_rerun_rewrites_only_it(
    complete: Pipeline, damage: Callable[[Path], None]
) -> None:
    before = complete.artifacts()
    (victim,) = [
        a
        for a in records(complete, "episode-analysis")
        if any(i.role == "chapter-extraction" for i in a.record.dependency.inputs)
        and json.loads(a.read_bytes(analysis.RECORD_FILE))["source"]["item_key"] == B
    ]
    name, artifact_key = analysis.ANALYSIS, victim.record.key
    damage(complete.store.directory_for(name, artifact_key))

    with pytest.raises(InterruptedWrite):
        complete.store.load(name, artifact_key)
    assert complete.store.find(victim.record.dependency) is None

    calls = len(complete.calls)
    complete.run()

    after = complete.artifacts()
    assert set(after) == set(before)
    assert [k for k in before if after[k] != before[k]] == [(name, artifact_key)]
    assert complete.calls[calls:] == []
    complete.store.load(name, artifact_key)


# --- Claim 5: a correction invalidates its dependants and nothing else ------------------


def test_a_correction_invalidates_its_dependants_and_nothing_else(complete: Pipeline) -> None:
    before = complete.artifacts()
    calls = len(complete.calls)
    write_correction(complete, interpretation_id(complete, A, 1))

    outputs = complete.run()

    assert new_since(before, complete.artifacts()) == {
        "episode-analysis": 1,
        "analysis-corrections-run": 1,
        "pattern-discovery-response": 1,
        "pattern-discovery": 1,
        "pattern-report": 1,
        "baseline-report": 1,
        "weighted-report": 1,
    }
    assert all(complete.artifacts()[k] == before[k] for k in before)
    assert complete.calls[calls:] == ["pattern-discovery"]
    assert f"corrected {A}" in outputs["corrections"] or A in outputs["corrections"]

    # The corrected analysis is of book A. Discovery reads it with the original analysis of B.
    newest = complete.store.latest_key(discovery.DISCOVERY)
    found = complete.store.load(discovery.DISCOVERY, newest or "")
    analysed = {i.identity for i in found.record.dependency.inputs if i.role == "episode-analysis"}
    original = {k for (n, k) in before if n == "episode-analysis"}
    (kept,) = analysed & original
    assert (
        json.loads(complete.store.load("episode-analysis", kept).read_bytes(analysis.RECORD_FILE))[
            "source"
        ]["item_key"]
        == B
    )


# --- Claim 6: a change to the weights reuses extraction, analysis and discovery ----------


def test_a_weight_change_reuses_extraction_analysis_and_discovery(complete: Pipeline) -> None:
    before = complete.artifacts()
    calls = len(complete.calls)
    edit_weights(complete)

    outputs = complete.run()

    assert new_since(before, complete.artifacts()) == {"weighted-report": 1}
    assert complete.calls[calls:] == []
    assert "reused baseline report" in outputs["weighted-report"]
    assert "built weighted report" in outputs["weighted-report"]


def test_the_store_used_by_the_pipeline_is_the_one_the_test_reads(complete: Pipeline) -> None:
    assert isinstance(complete.store, ArtifactStore)
    assert complete.store.latest_key(weighted_report.WEIGHTED) is not None
