"""Tests of pattern discovery and the pattern report over fixture interpretations.

The fixture holds two books. Each book has a short PDF, an extraction record with two
nested sections, and an analysis that a recorded model answered.
"""

from __future__ import annotations

import json
import re
from collections.abc import Callable
from dataclasses import dataclass
from html.parser import HTMLParser
from pathlib import Path
from typing import Any

import pymupdf  # pyright: ignore[reportMissingTypeStubs]
import pytest
from typer.testing import CliRunner

from urg.analysis import ANALYSIS, run_analysis
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.cli import app
from urg.discovery import (
    FAILED_RESPONSE,
    FAILURE_FILE,
    RECORD_FILE,
    REQUEST_FILE,
    RESPONSE,
    STDERR_FILE,
    STDOUT_FILE,
    FailedDiscoveryCall,
    MalformedDiscoveryResponse,
    run_discovery,
)
from urg.model import (
    FailedOutput,
    ModelBoundary,
    ModelError,
    ModelRequest,
    ModelResponse,
    RecordedModel,
    parse_claude_result,
)
from urg.report import INDEX, INTERPRETATIONS, MANIFEST, STRUCTURE, build_report
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.zotero_db import sha256_file

Json = dict[str, Any]

BOOKS = ("ABCD1234", "EFGH5678")
ELEMENTS: list[tuple[str, str, int]] = [
    ("heading", "Scanning", 0),
    ("paragraph", "The input is one string.", 0),
    ("code_block", "for c in s:", 0),
    ("paragraph", "This loop is a scanner.", 1),
    ("heading", "Errors", 1),
    ("paragraph", "A bad character stops it.", 1),
]


def bbox(n: int) -> list[float]:
    return [72.0, 60.0 + 20 * n, 400.0, 76.0 + 20 * n]


def line_id(key: str, n: int) -> str:
    return f"{key}:p{ELEMENTS[n - 1][2]:04d}-b000-l{n:02d}"


def write_pdf(path: Path) -> None:
    document = pymupdf.open()
    for page_index in range(2):
        page = document.new_page()  # pyright: ignore[reportUnknownMemberType]
        for n, (_, text, page_of) in enumerate(ELEMENTS, start=1):
            if page_of == page_index:
                page.insert_text((72, 72 + 20 * n), text)  # pyright: ignore[reportUnknownMemberType]
    document.save(path)  # pyright: ignore[reportUnknownMemberType]


def extraction_record(key: str, pdf: Path) -> Json:
    elements: list[Json] = []
    for n, (kind, text, page) in enumerate(ELEMENTS, start=1):
        element: Json = {
            "id": f"{key}:e{n:04d}",
            "kind": kind,
            "pdf_page_index": page,
            "printed_page_label": str(page + 1),
            "bbox": bbox(n),
            "evidence": [line_id(key, n)],
            "text": text,
        }
        if kind == "heading":
            element["level"] = 1 if n == 1 else 2
        elements.append(element)
    lines = [
        {
            "id": line_id(key, n),
            "pdf_page_index": page,
            "printed_page_label": str(page + 1),
            "bbox": bbox(n),
            "text": text,
        }
        for n, (_, text, page) in enumerate(ELEMENTS, start=1)
    ]
    sections = [
        {
            "id": f"{key}:s001",
            "title": "Scanning",
            "level": 1,
            "heading": f"{key}:e0001",
            "parent": None,
            "pdf_page_index": 0,
            "elements": [f"{key}:e{n:04d}" for n in range(1, 5)],
        },
        {
            "id": f"{key}:s002",
            "title": "Errors",
            "level": 2,
            "heading": f"{key}:e0005",
            "parent": f"{key}:s001",
            "pdf_page_index": 1,
            "elements": [f"{key}:e0005", f"{key}:e0006"],
        },
    ]
    return {
        "source": {
            "source_identity": {"item_key": key},
            "title": f"Book {key}",
            "year": "2020",
            "document": {"location": str(pdf), "sha256": sha256_file(pdf), "page_count": 2},
        },
        "chapter": {"number": "1", "title": "Scanners"},
        "evidence": lines,
        "structure": {"sections": sections, "elements": elements},
    }


def fixture_dependency(artifact: str, locator: str) -> DependencyRecord:
    return DependencyRecord(
        stage="chapter-extraction",
        artifact=artifact,
        schema_version=1,
        code_version="fixture",
        configuration={},
        inputs=[InputIdentity(role="input", kind="file", locator=locator, identity="0" * 64)],
    )


def analysis_answer(key: str) -> str:
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
            "local_id": "mv1",
            "kind": "move",
            "episode": "ep1",
            "label": "states the problem",
            "description": "The input is one string.",
            "evidence": [f"{key}:e0002"],
            "uncertainty": {"level": "low", "reason": "Stated directly."},
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


def write_versioned(path: Path, body: str, version: int = 1) -> Path:
    path.write_text(f"---\nversion: {version}\n---\n{body}\n")
    return path


def write_settings(path: Path, weights: dict[str, float] | None = None) -> Path:
    settings: Json = {
        "version": 1,
        "minimum_supporting_instances": 2,
        "preference_weights": {"default": 1.0, "books": weights or {}},
        "observation_scope": {
            "interpretation_kinds": [
                "episode",
                "move",
                "relationship",
                "alternative-reading",
                "observation",
            ],
            "structure": ["section"],
        },
    }
    path.write_text(json.dumps(settings))
    return path


class World:
    """A store with two analysed books, and the files that a discovery run reads."""

    def __init__(self, root: Path, analysis: Callable[[str], str] | None = None) -> None:
        self.root = root
        self.store = ArtifactStore(root / "data")
        results: list[Json] = []
        for key in BOOKS:
            pdf = root / f"{key}.pdf"
            write_pdf(pdf)
            extraction = self.store.write(
                fixture_dependency("chapter-extraction", f"{key}.pdf"),
                {"extraction.json": json.dumps(extraction_record(key, pdf)).encode()},
            )
            results.append(
                {
                    "item_key": key,
                    "title": f"Book {key}",
                    "status": "extracted",
                    "artifact_key": extraction.record.key,
                    "error": None,
                }
            )
        self.store.write(
            fixture_dependency("chapter-extraction-run", "selection.json"),
            {"report.json": json.dumps({"results": results}).encode()},
        )
        analysis_instructions = write_versioned(root / "analysis.md", "Find episodes.")
        model = RecordedModel([(analysis or analysis_answer)(key) for key in BOOKS])
        run = run_analysis(self.store, analysis_instructions, model)
        assert not run.failed
        self.ids: dict[tuple[str, str], str] = {}
        for outcome in run.outcomes:
            assert outcome.analysis is not None
            record = json.loads(outcome.analysis.read_bytes(ANALYSIS_FILE))
            for interpretation in record["interpretations"]:
                self.ids[(outcome.item_key, interpretation["local_id"])] = interpretation["id"]
        self.settings = write_settings(root / "settings.json")
        self.instructions = write_versioned(root / "discovery.md", "Find patterns.")

    def interpretation(self, key: str, local_id: str) -> str:
        return self.ids[(key, local_id)]

    def answer(self) -> str:
        a, b = BOOKS
        patterns: list[Json] = [
            {
                "local_id": "p1",
                "name": "Example before name",
                "description": "The chapter shows code, then names what it does.",
                "supporting": [
                    {"observation": self.interpretation(a, "ep1"), "note": "Loop, then name."},
                    {"observation": self.interpretation(b, "ep1"), "note": "Same order."},
                ],
                "exceptions": [],
                "unresolved": [
                    {"observation": self.interpretation(b, "obs1"), "note": "It may be a name."}
                ],
                "rules": ["Explanation -> Problem Example Name"],
                "representation": [
                    {"feature": "hierarchy", "justification": "An episode contains moves."}
                ],
                "uncertainty": {"level": "medium", "reason": "Two books only."},
            },
            {
                "local_id": "p2",
                "name": "Errors under the main section",
                "description": "A level-2 section about errors closes the topic.",
                "supporting": [
                    {"observation": f"{a}:s002", "note": "Errors under Scanning."},
                    {"observation": f"{b}:s002", "note": "Errors under Scanning."},
                ],
                "uncertainty": {"level": "high", "reason": "The fixture is small."},
            },
            {
                "local_id": "p3",
                "name": "Lonely move",
                "description": "One move only.",
                "supporting": [{"observation": self.interpretation(a, "mv1"), "note": "One."}],
                "uncertainty": {"level": "high", "reason": "One instance."},
            },
        ]
        return json.dumps({"patterns": patterns})

    def discover(self, model: ModelBoundary, **options: Any) -> Any:
        return run_discovery(self.store, self.settings, self.instructions, model, **options)


@pytest.fixture
def world(tmp_path: Path) -> World:
    return World(tmp_path)


def record_of(artifact: StoredArtifact) -> Json:
    return json.loads(artifact.read_bytes(RECORD_FILE))


def test_each_pattern_instance_resolves_to_an_observation(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    record = record_of(run.discovery)
    analyses = {
        i["id"]: i
        for s in record["sources"]
        for i in json.loads(
            world.store.load(ANALYSIS, s["analysis_artifact"]).read_bytes(ANALYSIS_FILE)
        )["interpretations"]
    }
    sections = {f"{key}:s00{n}" for key in BOOKS for n in (1, 2)}
    assert [p["local_id"] for p in record["patterns"]] == ["p1", "p2"]
    for pattern in record["patterns"]:
        for role in ("supporting", "exceptions", "unresolved"):
            for citation in pattern[role]:
                observation = citation["observation"]
                entry = record["inventory"][observation["id"]]
                assert entry["layer"] == observation["layer"]
                assert entry["item_key"] == citation["item_key"]
                if observation["layer"] == "interpretation":
                    assert observation["id"] in analyses
                else:
                    assert observation["id"] in sections
    p1, p2 = record["patterns"]
    assert p1["basis"] == "interpretation"
    assert p2["basis"] == "document-structure"
    assert p1["books"] == list(BOOKS)


def test_patterns_below_the_minimum_are_rejected_with_reasons(world: World) -> None:
    record = world.discover(RecordedModel([world.answer()])).record
    [rejected] = record["rejected"]
    assert rejected["local_id"] == "p3"
    assert any("fewer than the minimum 2" in reason for reason in rejected["reasons"])


def test_observations_that_fit_no_pattern_are_listed(world: World) -> None:
    record = world.discover(RecordedModel([world.answer()])).record
    a, b = BOOKS
    unfitted = {entry["id"]: entry for entry in record["unfitted"]}
    assert world.interpretation(a, "mv1") in unfitted
    assert world.interpretation(a, "obs1") in unfitted
    assert world.interpretation(a, "ep1") not in unfitted
    assert f"{a}:s001" in unfitted
    assert f"{a}:s002" not in unfitted
    assert unfitted[world.interpretation(b, "obs1")]["cited_as"] == [
        {"pattern": record["patterns"][0]["id"], "role": "unresolved"}
    ]
    assert record["counts"]["unfitted"] == len(record["unfitted"])


def test_discovery_records_its_settings_method_and_model(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    discovery_run = run.record["discovery_run"]
    assert discovery_run["settings"]["content"] == json.loads(world.settings.read_text())
    assert discovery_run["settings"]["sha256"]
    assert discovery_run["preference_weights"] == {key: 1.0 for key in BOOKS}
    assert discovery_run["method"]["name"]
    assert discovery_run["model"] == {"harness": "recorded", "model": "recorded"}
    assert discovery_run["instructions"]["version"] == 1
    roles = {i.role for i in run.discovery.record.dependency.inputs}
    assert {"discovery-settings", "discovery-instructions", "model-response"} <= roles


def test_a_rerun_with_unchanged_inputs_reuses_the_discovery_run(world: World) -> None:
    first = world.discover(RecordedModel([world.answer()]))
    second = world.discover(RecordedModel([]))
    assert second.response_reused and second.discovery_reused
    assert second.discovery.record.key == first.discovery.record.key
    built = build_report(world.store, first.discovery)
    again = build_report(world.store, second.discovery)
    assert not built.reused and again.reused
    assert again.report.record.key == built.report.record.key


def test_changed_preference_weights_start_a_new_discovery_run(world: World) -> None:
    first = world.discover(RecordedModel([world.answer()]))
    write_settings(world.settings, {BOOKS[0]: 2.0})
    model = RecordedModel([world.answer()])
    second = world.discover(model)
    assert not second.response_reused and not second.discovery_reused
    assert second.discovery.record.key != first.discovery.record.key
    assert second.record["discovery_run"]["preference_weights"][BOOKS[0]] == 2.0
    assert "Preference weight: 2" in model.requests[0].prompt


def test_weights_for_books_outside_the_analysis_run_fail(world: World) -> None:
    write_settings(world.settings, {"ZZZZ9999": 2.0})
    with pytest.raises(Exception, match="outside the analysis run"):
        world.discover(RecordedModel([world.answer()]))


def test_a_malformed_answer_keeps_the_response(world: World) -> None:
    with pytest.raises(MalformedDiscoveryResponse, match="--new-attempt"):
        world.discover(RecordedModel(["not json"]))
    assert world.store.latest_key(RESPONSE) is not None
    run = world.discover(RecordedModel([world.answer()]), new_attempt=True)
    assert run.record["discovery_run"]["attempt"] == 2


def edited_answer(world: World, edit: Callable[[list[Json]], None]) -> str:
    patterns: list[Json] = json.loads(world.answer())["patterns"]
    edit(patterns)
    return json.dumps({"patterns": patterns})


# The rules of the first real discovery run. Each one must stay a valid rule.
FIRST_RUN_RULES = [
    "Recipe -> Problem Solution Discussion Variations? SeeAlso",
    "Solution -> Intro Variant+",
    "Variant -> Requirement Regex FlavorNote",
    "Motivation -> FailingCase Principle? Fix",
    "Development -> Initial Intervening* Revision",
    "Chapter -> Opening Body Return",
    "Chapter -> Preview Section+",
    "FormalDefinition -> Prose? Rules Gloss",
    "Theorem -> Statement ProofSketch",
    "WorkedExample -> Point? Trace Analysis",
    "DesignChoice -> Options Rejection* Adoption",
    "Chapter -> Body Challenges? DesignNote? Notes?",
    "Progression -> Simplest Complication+ Generalisation?",
    "Formulation -> Definition EquivalenceTheorem",
]


def test_rules_with_each_modifier_are_accepted(world: World) -> None:
    rules = [*FIRST_RUN_RULES, "  Step_2 -> first-part Part_b? C* D+  "]

    def edit(patterns: list[Json]) -> None:
        patterns[0]["rules"] = rules

    record = world.discover(RecordedModel([edited_answer(world, edit)])).record
    assert record["patterns"][0]["rules"] == rules


@pytest.mark.parametrize(
    "rule",
    [
        "Recipe -> ???",
        "Recipe -> Part -> Other",
        "Recipe -> Part\nanything at all",
        "Recipe -> Part Other!",
        "Recipe -> Part++",
        "Recipe -> (Part)",
        "Recipe ->",
        "-> Part",
        "1Recipe -> Part",
    ],
)
def test_a_malformed_rule_rejects_its_pattern(world: World, rule: str) -> None:
    def edit(patterns: list[Json]) -> None:
        patterns[1]["rules"] = ["Topic -> Body Errors", rule]

    record = world.discover(RecordedModel([edited_answer(world, edit)])).record
    assert [p["local_id"] for p in record["patterns"]] == ["p1"]
    rejected = {r["local_id"]: r for r in record["rejected"]}
    assert rejected["p2"]["reasons"] == [
        f"rules: {rule!r} does not have the form 'Name -> Part Part? Part* Part+'"
    ]


def test_a_pattern_without_support_is_rejected_alone(world: World) -> None:
    def edit(patterns: list[Json]) -> None:
        patterns[1]["supporting"] = []

    record = world.discover(RecordedModel([edited_answer(world, edit)])).record
    assert [p["local_id"] for p in record["patterns"]] == ["p1"]
    rejected = {r["local_id"]: r for r in record["rejected"]}
    assert set(rejected) == {"p2", "p3"}
    assert rejected["p2"]["reasons"] == ["supporting: 0 instances, fewer than the minimum 2"]


def test_a_pattern_without_a_supporting_field_is_malformed(world: World) -> None:
    def edit(patterns: list[Json]) -> None:
        del patterns[1]["supporting"]

    with pytest.raises(MalformedDiscoveryResponse, match="supporting"):
        world.discover(RecordedModel([edited_answer(world, edit)]))


def test_only_supporting_citations_make_an_observation_fit(world: World) -> None:
    record = world.discover(RecordedModel([world.answer()])).record
    unfitted = {entry["id"]: entry for entry in record["unfitted"]}
    [p1] = [p for p in record["patterns"] if p["local_id"] == "p1"]
    cited = unfitted[world.interpretation(BOOKS[1], "obs1")]
    assert cited["cited_as"] == [{"pattern": p1["id"], "role": "unresolved"}]


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


def test_a_failed_call_is_kept_and_never_reused(world: World) -> None:
    failing = EnvelopeModel(b"Error: overloaded", b"retry later", 1)
    with pytest.raises(FailedDiscoveryCall) as caught:
        world.discover(failing)
    kept = caught.value.kept
    assert kept.record.dependency.artifact == FAILED_RESPONSE
    assert kept.record.key in str(caught.value)
    assert kept.read_bytes(STDOUT_FILE) == b"Error: overloaded"
    assert kept.read_bytes(STDERR_FILE) == b"retry later"
    assert set(json.loads(kept.read_bytes(REQUEST_FILE))) == {"system", "prompt"}
    failure: Json = json.loads(kept.read_bytes(FAILURE_FILE))
    assert failure["status"] == "failed"
    assert failure["reason"] == "no-json" and failure["returncode"] == 1
    assert world.store.latest_key(RESPONSE) is None

    # A second failure is kept beside the first. Neither is reused as an answer.
    with pytest.raises(FailedDiscoveryCall) as again:
        world.discover(failing)
    assert failing.calls == 2
    assert again.value.kept.record.key != kept.record.key
    assert world.store.load(FAILED_RESPONSE, kept.record.key).read_bytes(STDOUT_FILE)

    model = RecordedModel([world.answer()], identity=failing.describe())
    run = world.discover(model)
    assert len(model.requests) == 1
    assert run.record["discovery_run"]["attempt"] == 1


@dataclass
class TimedOutModel:
    """A boundary that times out after partial output, as ``ClaudeCliModel`` reports it."""

    def describe(self) -> Json:
        return {"harness": "recorded", "model": "recorded"}

    def call(self, request: ModelRequest) -> ModelResponse:
        output = FailedOutput(b'{"type":"res', b"", {"reason": "timeout", "timeout_seconds": 5})
        raise ModelError("the model call timed out after 5 seconds", output)


def test_partial_output_of_a_timed_out_call_is_kept(world: World) -> None:
    with pytest.raises(FailedDiscoveryCall, match="timed out") as caught:
        world.discover(TimedOutModel())
    kept = caught.value.kept
    assert kept.read_bytes(STDOUT_FILE) == b'{"type":"res'
    failure: Json = json.loads(kept.read_bytes(FAILURE_FILE))
    assert failure["reason"] == "timeout"
    assert failure["error"] == "the model call timed out after 5 seconds"


def test_a_call_that_never_ran_keeps_nothing(world: World) -> None:
    with pytest.raises(ModelError, match="no answer left"):
        world.discover(RecordedModel([]))
    assert world.store.latest_key(FAILED_RESPONSE) is None
    assert world.store.latest_key(RESPONSE) is None


class Links(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.ids: set[str] = set()
        self.hrefs: list[str] = []
        self.images: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if values.get("id"):
            self.ids.add(values["id"] or "")
        if tag == "a" and values.get("href"):
            self.hrefs.append(values["href"] or "")
        if tag == "img" and values.get("src"):
            self.images.append(values["src"] or "")


def parse_pages(directory: Path) -> dict[str, Links]:
    pages: dict[str, Links] = {}
    for name in (INDEX, INTERPRETATIONS, STRUCTURE):
        parser = Links()
        parser.feed((directory / name).read_text())
        pages[name] = parser
    return pages


def test_each_report_link_resolves(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    built = build_report(world.store, run.discovery)
    directory = built.report.directory
    pages = parse_pages(directory)
    checked = 0
    for name, parser in pages.items():
        for target in parser.hrefs:
            page, _, fragment = target.partition("#")
            page = page or name
            assert page in pages, f"{name} links to a missing page {target}"
            if fragment:
                assert fragment in pages[page].ids, f"{name} links to a missing anchor {target}"
            checked += 1
        for image in parser.images:
            assert (directory / image).is_file(), f"{name} shows a missing image {image}"
    assert checked > 20
    assert pages[STRUCTURE].images, "the structure page shows no render"


def test_the_report_keeps_the_three_layers_apart(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    record = run.record
    pages = parse_pages(build_report(world.store, run.discovery).report.directory)

    def anchors(ids: Any) -> set[str]:
        return {re.sub(r"[^A-Za-z0-9._-]", "_", i) for i in ids}

    patterns = anchors(p["id"] for p in record["patterns"])
    interpretations = anchors(world.ids.values())
    structure = anchors(i for i, e in record["inventory"].items() if e["layer"] != "interpretation")
    assert pages[INDEX].ids == patterns | {"unfitted"}
    assert pages[INTERPRETATIONS].ids == interpretations
    assert structure <= pages[STRUCTURE].ids
    assert not pages[STRUCTURE].ids & (patterns | interpretations)


def test_each_unfitted_observation_appears_in_the_report(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    built = build_report(world.store, run.discovery)
    html = built.report.read_bytes(INDEX).decode()
    unfitted = html.split("id='unfitted'")[1]
    for entry in run.record["unfitted"]:
        assert entry["id"] in unfitted


def test_a_failed_render_is_recorded_and_the_report_still_builds(world: World) -> None:
    run = world.discover(RecordedModel([world.answer()]))
    (world.root / f"{BOOKS[0]}.pdf").unlink()
    built = build_report(world.store, run.discovery)
    renders = json.loads(built.report.read_bytes(MANIFEST))["renders"]
    failed = {i: r for i, r in renders.items() if r["status"] == "failed"}
    assert failed and all(i.startswith(BOOKS[0]) for i in failed)
    assert all(r["status"] == "rendered" for i, r in renders.items() if i.startswith(BOOKS[1]))
    assert "No render:" in built.report.read_bytes(STRUCTURE).decode()


def test_the_report_command_fails_without_a_discovery_run(tmp_path: Path) -> None:
    result = CliRunner().invoke(app, ["pattern-report", "--data-dir", str(tmp_path / "data")])
    assert result.exit_code == 1
    assert "Run pattern-discovery first" in result.output
