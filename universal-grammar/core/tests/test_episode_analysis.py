"""Tests of the episode-analysis stage. The model boundary replays recorded answers."""

from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import pytest

from urg.analysis import (
    ANALYSIS,
    FAILED_RESPONSE,
    FAILURE_FILE,
    PROPOSED,
    RAW_RESPONSE_FILE,
    RECORD_FILE,
    REQUEST_FILE,
    RESPONSE,
    STDERR_FILE,
    STDOUT_FILE,
    MalformedResponse,
    load_instructions,
    parse_answer,
    render_prompt,
    run_analysis,
)
from urg.model import (
    ClaudeCliModel,
    ModelConfig,
    ModelError,
    ModelRequest,
    ModelResponse,
    RecordedModel,
    parse_claude_result,
)
from urg.store import ArtifactStore, DependencyRecord, InputIdentity
from urg.version import module_closure

Json = dict[str, Any]
KEY = "ABCD1234"


def element(n: int, kind: str, text: str, page: int) -> Json:
    return {
        "id": f"{KEY}:e{n:04d}",
        "kind": kind,
        "pdf_page_index": page,
        "printed_page_label": str(page + 1),
        "bbox": [0, 0, 1, 1],
        "evidence": [f"{KEY}:p{page:04d}-b000-l{n:02d}"],
        "text": text,
        "section": f"{KEY}:s001",
    }


def extraction_record() -> Json:
    elements = [
        element(1, "heading", "Scanning", 0),
        element(2, "paragraph", "The input is one long string.", 0),
        element(3, "code_block", "for c in text: emit(c)", 0),
        element(4, "paragraph", "This loop is a scanner.", 1),
    ]
    elements[0]["level"] = 1
    lines = [
        {
            "id": e["evidence"][0],
            "pdf_page_index": e["pdf_page_index"],
            "printed_page_label": e["printed_page_label"],
            "text": e["text"],
        }
        for e in elements
    ]
    return {
        "source": {"source_identity": {"item_key": KEY}, "title": "A Book", "year": "2020"},
        "chapter": {"number": "1", "title": "Scanning"},
        "evidence": lines,
        "structure": {"sections": [], "elements": elements},
    }


def seed_store(tmp_path: Path) -> ArtifactStore:
    """Store one extraction and an extraction run report that names it."""
    store = ArtifactStore(tmp_path / "data")
    extraction = store.write(
        DependencyRecord(
            stage="chapter-extraction",
            artifact="chapter-extraction",
            schema_version=1,
            code_version="fixture",
            configuration={},
            inputs=[
                InputIdentity(role="document", kind="file", locator="book.pdf", identity="0" * 64)
            ],
        ),
        {"extraction.json": json.dumps(extraction_record()).encode()},
    )
    report = {
        "results": [
            {
                "item_key": KEY,
                "title": "A Book",
                "status": "extracted",
                "artifact_key": extraction.record.key,
                "error": None,
            }
        ]
    }
    store.write(
        DependencyRecord(
            stage="chapter-extraction",
            artifact="chapter-extraction-run",
            schema_version=1,
            code_version="fixture",
            configuration={},
            inputs=[
                InputIdentity(role="selection", kind="file", locator="s.json", identity="1" * 64)
            ],
        ),
        {"report.json": json.dumps(report).encode()},
    )
    return store


def write_instructions(
    tmp_path: Path, version: int = 1, body: str = "Analyse the chapter."
) -> Path:
    path = tmp_path / "instructions.md"
    path.write_text(f"---\nversion: {version}\n---\n{body}\n")
    return path


def answer(**overrides: Any) -> str:
    interpretations: list[Json] = [
        {
            "local_id": "ep1",
            "kind": "episode",
            "label": "worked example before name",
            "description": "A loop is shown, then named.",
            "evidence": [f"{KEY}:e0002", f"{KEY}:e0003", f"{KEY}:e0004"],
            "uncertainty": {"level": "medium", "reason": "The order implies the purpose."},
        },
        {
            "local_id": "mv1",
            "kind": "move",
            "episode": "ep1",
            "label": "states the problem",
            "description": "The input is one string.",
            "evidence": [f"{KEY}:e0002"],
            "uncertainty": {"level": "low", "reason": "Stated directly."},
        },
        {
            "local_id": "rel1",
            "kind": "relationship",
            "source": "mv1",
            "target": f"{KEY}:e0003",
            "label": "motivates",
            "description": "The problem motivates the loop.",
            "evidence": [f"{KEY}:e0003"],
            "uncertainty": {"level": "medium", "reason": "Implied by order."},
        },
        {
            "local_id": "obs1",
            "kind": "observation",
            "label": None,
            "description": "The code comes before its name.",
            "evidence": [f"{KEY}:p0001-b000-l04"],
            "uncertainty": {"level": "low", "reason": "Visible in the order."},
        },
    ]
    for item in interpretations:
        item.update(overrides.get(item["local_id"], {}))
    return json.dumps({"interpretations": interpretations})


def analysis_of(store: ArtifactStore, run_outcome: Any) -> Json:
    assert run_outcome.analysis is not None, run_outcome.error
    return json.loads(run_outcome.analysis.read_bytes(RECORD_FILE))


def test_every_citation_resolves_to_evidence(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    model = RecordedModel([answer()])
    run = run_analysis(store, write_instructions(tmp_path), model)
    record = analysis_of(store, run.outcomes[0])
    extraction = extraction_record()
    known = {e["id"] for e in extraction["evidence"]} | {
        e["id"] for e in extraction["structure"]["elements"]
    }
    assert record["counts"] == {
        "episode": 1,
        "move": 1,
        "relationship": 1,
        "alternative-reading": 0,
        "observation": 1,
        "rejected": 0,
    }
    for item in record["interpretations"]:
        assert item["evidence"]
        assert all(e["id"] in known for e in item["evidence"])
        assert item["review_status"] == PROPOSED
        assert item["version"] == 1
        assert item["author"]["kind"] == "model"
        assert item["uncertainty"]["level"] in ("low", "medium", "high")
    by_local = {i["local_id"]: i for i in record["interpretations"]}
    assert by_local["mv1"]["episode"] == {"layer": "interpretation", "id": by_local["ep1"]["id"]}
    assert by_local["rel1"]["target"] == {"layer": "structure-element", "id": f"{KEY}:e0003"}
    assert by_local["obs1"]["evidence"][0]["layer"] == "evidence-line"


def test_interpretation_citing_missing_evidence_is_rejected(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    bad = answer(ep1={"evidence": [f"{KEY}:e0002", f"{KEY}:e0999"]})
    run = run_analysis(store, write_instructions(tmp_path), RecordedModel([bad]))
    record = analysis_of(store, run.outcomes[0])
    rejected = {r["local_id"]: r for r in record["rejected"]}
    assert "e0999" in rejected["ep1"]["reasons"][0]
    # The move names the rejected episode, and the relationship names the move.
    assert set(rejected) == {"ep1", "mv1", "rel1"}
    assert [i["local_id"] for i in record["interpretations"]] == ["obs1"]


def test_malformed_response_is_rejected_and_kept(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    run = run_analysis(
        store, write_instructions(tmp_path), RecordedModel(["Here are the episodes: none"])
    )
    outcome = run.outcomes[0]
    assert run.failed
    assert outcome.analysis is None
    assert outcome.error is not None and "malformed" in outcome.error
    assert outcome.response is not None
    raw = json.loads(outcome.response.read_bytes(RAW_RESPONSE_FILE))
    assert raw["result"] == "Here are the episodes: none"
    assert run.report is not None
    report: Json = json.loads(run.report.read_bytes("report.json"))
    assert report["results"][0]["status"] == "rejected"


@pytest.mark.parametrize(
    "text",
    [
        "{}",
        '{"interpretations": []}',
        answer(mv1={"episode": None}),
        answer(ep1={"label": ""}),
        answer(ep1={"uncertainty": {"level": "certain", "reason": "x"}}),
        answer(obs1={"local_id": "ep1"}),
        answer(ep1={"weight": 0.9}),
    ],
)
def test_parse_answer_rejects_malformed_answers(text: str) -> None:
    with pytest.raises(MalformedResponse):
        parse_answer(text)


def test_parse_answer_accepts_one_fence() -> None:
    assert len(parse_answer(f"```json\n{answer()}\n```").interpretations) == 4


def test_rerun_reuses_the_stored_response(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    instructions = write_instructions(tmp_path)
    first = run_analysis(store, instructions, RecordedModel([answer()]))
    model = RecordedModel([])
    second = run_analysis(store, instructions, model)
    assert model.requests == []
    assert second.outcomes[0].response_reused and second.outcomes[0].analysis_reused
    assert second.outcomes[0].analysis is not None and first.outcomes[0].analysis is not None
    assert second.outcomes[0].analysis.record.key == first.outcomes[0].analysis.record.key
    third = run_analysis(store, instructions, RecordedModel([answer()]), new_attempt=True)
    assert not third.outcomes[0].response_reused
    assert third.outcomes[0].response is not None and first.outcomes[0].response is not None
    assert third.outcomes[0].response.record.key != first.outcomes[0].response.record.key


def test_instruction_change_invalidates_analysis_but_not_extraction(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    extraction_key = store.latest_key("chapter-extraction")
    first = run_analysis(store, write_instructions(tmp_path, 1), RecordedModel([answer()]))
    model = RecordedModel([answer()])
    second = run_analysis(store, write_instructions(tmp_path, 2, "Analyse it again."), model)
    assert len(model.requests) == 1
    assert model.requests[0].system == "Analyse it again.\n"
    a, b = first.outcomes[0].analysis, second.outcomes[0].analysis
    assert a is not None and b is not None and a.record.key != b.record.key
    assert json.loads(b.read_bytes(RECORD_FILE))["instructions"]["version"] == 2
    assert store.latest_key("chapter-extraction") == extraction_key
    inputs = {i.role: i for i in b.record.dependency.inputs}
    assert inputs["chapter-extraction"].identity == extraction_key
    # The extraction stage does not depend on the analysis code.
    assert "urg.analysis" not in module_closure("urg.extraction")
    assert "urg.model" not in module_closure("urg.extraction")


def test_dependency_record_names_instructions_and_model(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    instructions = write_instructions(tmp_path)
    run = run_analysis(
        store, instructions, RecordedModel([answer()], identity={"harness": "x", "model": "m"})
    )
    analysis = run.outcomes[0].analysis
    assert analysis is not None
    dependency = analysis.record.dependency
    assert dependency.artifact == ANALYSIS
    assert dependency.configuration == {
        "instructions_version": 1,
        "model": {"harness": "x", "model": "m"},
    }
    roles = {i.role: i for i in dependency.inputs}
    assert roles["analysis-instructions"].identity == load_instructions(instructions).sha256
    assert roles["model-response"].locator.startswith(f"{RESPONSE}/")


def test_prompt_lists_every_element_with_its_id(tmp_path: Path) -> None:
    prompt = render_prompt(extraction_record())
    for n in range(1, 5):
        assert f"[{KEY}:e{n:04d}]" in prompt
    assert "heading, page 1, level 1" in prompt


def test_instructions_need_a_version(tmp_path: Path) -> None:
    path = tmp_path / "bad.md"
    path.write_text("no front matter\n")
    with pytest.raises(Exception, match="front matter"):
        load_instructions(path)


def test_claude_argv_gives_the_model_no_tools() -> None:
    model = ClaudeCliModel(
        ModelConfig(harness="claude-cli", model="claude-opus-5-5", effort="high")
    )
    request = ModelRequest(system="S", prompt="P")
    argv = model.argv(request)
    assert argv[:4] == ["claude", "-p", "--output-format", "json"]
    assert argv[argv.index("--tools") + 1] == ""
    assert "--safe-mode" in argv and "--no-session-persistence" in argv
    assert argv[argv.index("--system-prompt") + 1] == request.system
    assert argv[-2:] == ["--effort", "high"]


def test_claude_result_parsing() -> None:
    ok = json.dumps(
        {"is_error": False, "result": "{}", "modelUsage": {"m": {"provider": "firstParty"}}}
    )
    response = parse_claude_result(ok.encode(), b"", 0)
    assert response.text == "{}" and response.metadata["providers"] == ["firstParty"]
    failed = json.dumps({"is_error": True, "subtype": "success", "result": "Invalid API key"})
    with pytest.raises(ModelError, match="Invalid API key"):
        parse_claude_result(failed.encode(), b"", 1)
    with pytest.raises(ModelError, match="no JSON"):
        parse_claude_result(b"not json", b"boom", 1)


@pytest.mark.parametrize(
    ("stdout", "returncode", "reason"),
    [
        (b"not json", 1, "no-json"),
        (b"[1, 2]", 0, "not-an-object"),
        (b'{"is_error": true, "result": "Invalid API key"}', 1, "harness-error"),
        (b'{"is_error": false}', 0, "harness-error"),
    ],
)
def test_failed_envelope_keeps_its_output(stdout: bytes, returncode: int, reason: str) -> None:
    with pytest.raises(ModelError) as caught:
        parse_claude_result(stdout, b"stderr text", returncode)
    output = caught.value.output
    assert output is not None
    assert output.stdout == stdout and output.stderr == b"stderr text"
    assert output.metadata == {"reason": reason, "returncode": returncode}


def cli_model(monkeypatch: pytest.MonkeyPatch, run: Any) -> ClaudeCliModel:
    """A CLI boundary whose subprocess is replaced. It makes no model call."""
    model = ClaudeCliModel(ModelConfig(harness="claude-cli", model="m", timeout_seconds=5))

    def harness_version(self: ClaudeCliModel) -> str:
        return "2.1.322"

    monkeypatch.setattr(ClaudeCliModel, "_harness_version", harness_version)
    monkeypatch.setattr(subprocess, "run", run)
    return model


def test_timeout_keeps_the_partial_output(monkeypatch: pytest.MonkeyPatch) -> None:
    def run(argv: list[str], **kwargs: Any) -> Any:
        raise subprocess.TimeoutExpired(argv, 5, output=b'{"type": "res', stderr=b"killed")

    model = cli_model(monkeypatch, run)
    with pytest.raises(ModelError, match="timed out") as caught:
        model.call(ModelRequest(system="S", prompt="P"))
    output = caught.value.output
    assert output is not None
    assert output.stdout == b'{"type": "res' and output.stderr == b"killed"
    assert output.metadata == {
        "reason": "timeout",
        "timeout_seconds": 5,
        "harness_version": "2.1.322",
    }


def test_wrong_provider_keeps_the_output(monkeypatch: pytest.MonkeyPatch) -> None:
    stdout = json.dumps(
        {"is_error": False, "result": "{}", "modelUsage": {"m": {"provider": "bedrock"}}}
    ).encode()

    def run(argv: list[str], **kwargs: Any) -> Any:
        return subprocess.CompletedProcess(argv, 0, stdout=stdout, stderr=b"")

    model = cli_model(monkeypatch, run)
    with pytest.raises(ModelError, match="not the subscription") as caught:
        model.call(ModelRequest(system="S", prompt="P"))
    output = caught.value.output
    assert output is not None and output.stdout == stdout
    assert output.metadata["reason"] == "provider"
    assert output.metadata["providers"] == ["bedrock"]
    assert output.metadata["harness_version"] == "2.1.322"


@dataclass
class EnvelopeModel:
    """A boundary that parses a recorded CLI envelope, as ``ClaudeCliModel`` does."""

    stdout: bytes
    stderr: bytes
    returncode: int
    calls: int = 0

    def describe(self) -> Json:
        return {"harness": "recorded", "model": "recorded"}

    def call(self, request: ModelRequest) -> ModelResponse:
        self.calls += 1
        return parse_claude_result(self.stdout, self.stderr, self.returncode)


def test_failed_call_is_kept_and_never_reused(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    instructions = write_instructions(tmp_path)
    failing = EnvelopeModel(b"Error: overloaded", b"retry later", 1)
    first = run_analysis(store, instructions, failing)
    outcome = first.outcomes[0]
    assert first.failed and outcome.response is None and outcome.analysis is None
    kept = outcome.failed_response
    assert kept is not None and kept.record.dependency.artifact == FAILED_RESPONSE
    assert kept.read_bytes(STDOUT_FILE) == b"Error: overloaded"
    assert kept.read_bytes(STDERR_FILE) == b"retry later"
    request: Json = json.loads(kept.read_bytes(REQUEST_FILE))
    assert set(request) == {"system", "prompt"}
    failure: Json = json.loads(kept.read_bytes(FAILURE_FILE))
    assert failure["reason"] == "no-json" and failure["returncode"] == 1
    assert outcome.error is not None and kept.record.key in outcome.error
    assert first.report is not None
    result: Json = json.loads(first.report.read_bytes("report.json"))["results"][0]
    assert result["status"] == "failed" and result["response_key"] is None
    assert result["failed_response_key"] == kept.record.key

    # A second failure is kept beside the first. Neither is reused as an answer.
    second = run_analysis(store, instructions, failing)
    again = second.outcomes[0].failed_response
    assert failing.calls == 2
    assert again is not None and again.record.key != kept.record.key
    assert store.load(FAILED_RESPONSE, kept.record.key).read_bytes(STDOUT_FILE)

    model = RecordedModel([answer()], identity=failing.describe())
    third = run_analysis(store, instructions, model)
    assert len(model.requests) == 1
    assert third.outcomes[0].analysis is not None
    assert third.outcomes[0].failed_response is None


def test_call_that_never_ran_keeps_nothing(tmp_path: Path) -> None:
    store = seed_store(tmp_path)
    run = run_analysis(store, write_instructions(tmp_path), RecordedModel([]))
    outcome = run.outcomes[0]
    assert outcome.error is not None and "no answer left" in outcome.error
    assert outcome.failed_response is None
    assert store.latest_key(FAILED_RESPONSE) is None
