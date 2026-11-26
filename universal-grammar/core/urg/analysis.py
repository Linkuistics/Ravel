"""Episode analysis: candidate interpretations of each extracted chapter.

The stage sends one chapter to the model boundary and asks for explanatory episodes, moves,
relationships, alternative readings and observations. Each answer is checked against the
chapter: every cited evidence id must exist in the extraction, and every reference must name
another accepted interpretation. The stage keeps three kinds of artifact:

- ``episode-analysis-response`` keeps the request and the raw output of the model harness.
  A rerun reuses the newest response for the same request, so a rerun makes no model call.
  A call that fails at the model boundary is kept as ``episode-analysis-failed-response``
  instead. A failed call is never reused as an answer, so a rerun calls the model again.
- ``episode-analysis`` keeps the accepted interpretations and the rejected proposals of one
  chapter. Every accepted interpretation has review status "proposed".
- ``episode-analysis-run`` reports the outcome of one run over the selected chapters.

The dependency records name the analysis instructions, by version and digest, and the model
configuration. They do not guarantee identical model output on a rerun, because the model
is not deterministic. A rerun reuses the stored response instead of calling the model again.
"""

from __future__ import annotations

import hashlib
import json
import re
from collections.abc import Collection
from dataclasses import dataclass, field
from pathlib import Path
from typing import Annotated, Any, Literal, Self

from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    StringConstraints,
    ValidationError,
    model_validator,
)

from urg.errors import UrgError
from urg.extraction import EXTRACTION
from urg.extraction import RECORD_FILE as EXTRACTION_FILE
from urg.extraction import RUN_REPORT as EXTRACTION_RUN
from urg.model import FailedOutput, ModelBoundary, ModelError, ModelRequest
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.version import code_version

Json = dict[str, Any]

STAGE = "episode-analysis"
ANALYSIS = "episode-analysis"
RESPONSE = "episode-analysis-response"
FAILED_RESPONSE = "episode-analysis-failed-response"
RUN_REPORT = "episode-analysis-run"
SCHEMA_VERSION = 1
RECORD_FILE = "analysis.json"
SUMMARY_FILE = "summary.md"
REQUEST_FILE = "request.json"
RAW_RESPONSE_FILE = "raw-response.json"
METADATA_FILE = "metadata.json"
STDOUT_FILE = "stdout"
STDERR_FILE = "stderr"
FAILURE_FILE = "failure.json"
DEFAULT_INSTRUCTIONS = Path("docs/analysis/instructions.md")
DEFAULT_MODEL_CONFIG = Path("docs/analysis/model.json")
PROPOSED = "proposed"

FRONT_MATTER = re.compile(r"\A---\n(?P<head>.*?)\n---\n(?P<body>.*)\Z", re.DOTALL)
FENCE = re.compile(r"\A```[A-Za-z]*\n(?P<body>.*)\n```\Z", re.DOTALL)


class AnalysisError(UrgError):
    """The analysis stage cannot go on."""


class MalformedResponse(AnalysisError):
    """The model answer is not a valid interpretation list. The stage rejects all of it."""


class FailedCall(AnalysisError):
    """The model call failed at the boundary. ``kept`` holds its output in the store."""

    def __init__(self, message: str, kept: StoredArtifact) -> None:
        super().__init__(message)
        self.kept = kept


# --- Instructions ---------------------------------------------------------------------


@dataclass(frozen=True)
class Instructions:
    """A versioned analysis-instructions file."""

    path: Path
    version: int
    text: str
    sha256: str

    def identity(self) -> InputIdentity:
        return InputIdentity(
            role="analysis-instructions",
            kind="file",
            locator=self.path.as_posix(),
            identity=self.sha256,
        )


def load_instructions(path: Path) -> Instructions:
    """Read the instructions file. Its front matter must give an integer ``version``."""
    try:
        data = path.read_bytes()
    except OSError as error:
        raise AnalysisError(f"cannot read the analysis instructions {path}: {error}") from error
    match = FRONT_MATTER.match(data.decode())
    if match is None:
        raise AnalysisError(f"{path} has no front matter between two '---' lines")
    fields = dict(line.split(":", 1) for line in match["head"].splitlines() if ":" in line)
    try:
        version = int(fields["version"].strip())
    except KeyError, ValueError:
        raise AnalysisError(f"{path} front matter gives no integer version") from None
    return Instructions(
        path=path,
        version=version,
        text=match["body"].strip() + "\n",
        sha256=hashlib.sha256(data).hexdigest(),
    )


# --- Prompt ---------------------------------------------------------------------------


def render_prompt(extraction: Json) -> str:
    """Render one chapter as the list of elements that the instructions describe."""
    source = extraction["source"]
    chapter = extraction["chapter"]
    lines = [
        f"Book: {source['title']} ({source.get('year') or 'no year'})",
        f"Chapter {chapter['number']}: {chapter['title']}",
        "",
        "Elements:",
        "",
    ]
    for element in extraction["structure"]["elements"]:
        header = f"[{element['id']}] {element['kind']}, page {element['printed_page_label']}"
        if element["kind"] == "heading" and element.get("level") is not None:
            header += f", level {element['level']}"
        lines += [header, element.get("text") or "(no text)", ""]
    return "\n".join(lines)


def build_request(instructions: Instructions, extraction: Json) -> ModelRequest:
    return ModelRequest(system=instructions.text, prompt=render_prompt(extraction))


# --- Model answer ---------------------------------------------------------------------

LocalId = Annotated[str, StringConstraints(pattern=r"^[A-Za-z0-9._-]+$")]
Kind = Literal["episode", "move", "relationship", "alternative-reading", "observation"]
KINDS: tuple[Kind, ...] = ("episode", "move", "relationship", "alternative-reading", "observation")
REFERENCE_FIELDS = ("episode", "source", "target", "alternative_to")
REQUIRED_REFERENCES: dict[str, tuple[str, ...]] = {
    "episode": (),
    "move": ("episode",),
    "relationship": ("source", "target"),
    "alternative-reading": ("alternative_to",),
    "observation": (),
}


class Uncertainty(BaseModel):
    """Interpretation uncertainty: a level and the reason for it."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    level: Literal["low", "medium", "high"]
    reason: str = Field(min_length=1)


class Proposal(BaseModel):
    """One interpretation as the model proposes it, before the evidence check."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    local_id: LocalId
    kind: Kind
    label: str | None = None
    description: str = Field(min_length=1)
    evidence: list[str] = Field(min_length=1)
    uncertainty: Uncertainty
    episode: str | None = None
    source: str | None = None
    target: str | None = None
    alternative_to: str | None = None

    @model_validator(mode="after")
    def _check_kind(self) -> Self:
        if self.kind != "observation" and not (self.label and self.label.strip()):
            raise ValueError(f"{self.local_id}: a {self.kind} needs a label")
        required = REQUIRED_REFERENCES[self.kind]
        for name in REFERENCE_FIELDS:
            value = getattr(self, name)
            if name in required and not value:
                raise ValueError(f"{self.local_id}: a {self.kind} needs '{name}'")
            if name not in required and value is not None:
                raise ValueError(f"{self.local_id}: a {self.kind} cannot give '{name}'")
        return self

    def references(self) -> dict[str, str]:
        return {name: getattr(self, name) for name in REQUIRED_REFERENCES[self.kind]}


class Answer(BaseModel):
    """The whole model answer for one chapter."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    interpretations: list[Proposal] = Field(min_length=1)

    @model_validator(mode="after")
    def _check_unique(self) -> Self:
        seen: set[str] = set()
        for proposal in self.interpretations:
            if proposal.local_id in seen:
                raise ValueError(f"the local id {proposal.local_id} occurs twice")
            seen.add(proposal.local_id)
        return self


def parse_answer(text: str) -> Answer:
    """Parse the model answer, or raise ``MalformedResponse``.

    The stage accepts one surrounding Markdown fence, because models add one against
    instructions. It accepts nothing else around the JSON object.
    """
    body = text.strip()
    fenced = FENCE.match(body)
    if fenced is not None:
        body = fenced["body"].strip()
    try:
        return Answer.model_validate_json(body)
    except ValidationError as error:
        problems = "; ".join(
            f"{'.'.join(str(p) for p in e['loc']) or 'answer'}: {e['msg']}"
            for e in error.errors()[:10]
        )
        raise MalformedResponse(f"the model answer is malformed: {problems}") from None


# --- Evidence check -------------------------------------------------------------------


@dataclass(frozen=True)
class Checked:
    """The result of checking one answer against one extraction."""

    interpretations: list[Json]
    rejected: list[Json]


def evidence_index(extraction: Json) -> dict[str, Json]:
    """Map every citable id of an extraction to its layer and position."""
    index: dict[str, Json] = {}
    for item in extraction["evidence"]:
        index[item["id"]] = {
            "id": item["id"],
            "layer": "evidence-line",
            "pdf_page_index": item["pdf_page_index"],
            "printed_page_label": item["printed_page_label"],
        }
    for element in extraction["structure"]["elements"]:
        index[element["id"]] = {
            "id": element["id"],
            "layer": "structure-element",
            "kind": element["kind"],
            "pdf_page_index": element["pdf_page_index"],
            "printed_page_label": element["printed_page_label"],
        }
    return index


def _first_problems(answer: Answer, index: dict[str, Json]) -> dict[str, list[str]]:
    """Return the problems of each proposal that do not depend on other rejections."""
    proposals = {p.local_id: p for p in answer.interpretations}
    problems: dict[str, list[str]] = {}
    for proposal in answer.interpretations:
        found: list[str] = []
        missing = [e for e in proposal.evidence if e not in index]
        if missing:
            found.append(f"cites evidence that the extraction does not contain: {missing}")
        for name, value in proposal.references().items():
            target = proposals.get(value)
            if target is None and not (name in ("source", "target") and value in index):
                found.append(
                    f"'{name}' names {value}, which is neither an interpretation nor evidence"
                )
            elif target is not None and name == "episode" and target.kind != "episode":
                found.append(f"'episode' names {value}, which is a {target.kind}")
            elif value == proposal.local_id:
                found.append(f"'{name}' names the interpretation itself")
        if found:
            problems[proposal.local_id] = found
    return problems


def check_answer(answer: Answer, extraction: Json, prefix: str, author: Json) -> Checked:
    """Resolve citations and references, and build the interpretation records.

    A proposal that cites missing evidence, or names a missing or rejected interpretation,
    is rejected. The other proposals stay accepted.
    """
    index = evidence_index(extraction)
    problems = _first_problems(answer, index)
    proposals = {p.local_id: p for p in answer.interpretations}
    changed = True
    while changed:
        changed = False
        for proposal in answer.interpretations:
            if proposal.local_id in problems:
                continue
            for name, value in proposal.references().items():
                if value in problems:
                    problems[proposal.local_id] = [
                        f"'{name}' names the rejected interpretation {value}"
                    ]
                    changed = True
                    break
    accepted = [p for p in answer.interpretations if p.local_id not in problems]
    ids = {p.local_id: f"{prefix}:i{n:03d}" for n, p in enumerate(accepted, start=1)}
    records: list[Json] = []
    for proposal in accepted:
        record: Json = {
            "id": ids[proposal.local_id],
            "version": 1,
            "kind": proposal.kind,
            "label": proposal.label,
            "description": proposal.description,
            "evidence": [index[e] for e in dict.fromkeys(proposal.evidence)],
            "uncertainty": proposal.uncertainty.model_dump(),
            "author": author,
            "review_status": PROPOSED,
            "local_id": proposal.local_id,
        }
        for name, value in proposal.references().items():
            if value in proposals:
                record[name] = {"layer": "interpretation", "id": ids[value]}
            else:
                record[name] = {"layer": index[value]["layer"], "id": value}
        records.append(record)
    rejected = [
        {
            "local_id": p.local_id,
            "kind": p.kind,
            "reasons": problems[p.local_id],
            "proposal": p.model_dump(),
        }
        for p in answer.interpretations
        if p.local_id in problems
    ]
    return Checked(interpretations=records, rejected=rejected)


# --- Stage ----------------------------------------------------------------------------


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _extraction_identity(extraction: StoredArtifact) -> InputIdentity:
    key = extraction.record.key
    return InputIdentity(
        role="chapter-extraction", kind="artifact", locator=f"{EXTRACTION}/{key}", identity=key
    )


def response_dependency(
    instructions: Instructions,
    extraction: StoredArtifact,
    model: ModelBoundary,
    request: ModelRequest,
    attempt: int,
) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=RESPONSE,
        schema_version=SCHEMA_VERSION,
        code_version=code_version("urg.model"),
        configuration={
            "model": model.describe(),
            "request_sha256": _sha256(_json_bytes(request.as_json())),
            "attempt": attempt,
        },
        inputs=[instructions.identity(), _extraction_identity(extraction)],
    )


def obtain_response(
    store: ArtifactStore,
    instructions: Instructions,
    extraction: StoredArtifact,
    model: ModelBoundary,
    new_attempt: bool,
) -> tuple[StoredArtifact, bool]:
    """Return the newest stored response for this request, or call the model.

    With ``new_attempt``, the stage calls the model again and stores the answer as the next
    attempt. The second value is true when a stored response was reused.
    """
    record = json.loads(extraction.read_bytes(EXTRACTION_FILE))
    request = build_request(instructions, record)
    newest: StoredArtifact | None = None
    attempt = 1
    while (
        found := store.find(response_dependency(instructions, extraction, model, request, attempt))
    ) is not None:
        newest = found
        attempt += 1
    if newest is not None and not new_attempt:
        # Move the latest pointer, as every other reused artifact does.
        return store.reuse(newest.record.dependency) or newest, True
    dependency = response_dependency(instructions, extraction, model, request, attempt)
    try:
        response = model.call(request)
    except ModelError as error:
        if error.output is None:
            raise
        kept = keep_failed_output(store, dependency, request, str(error), error.output)
        raise FailedCall(
            f"{error}. The output is kept as {FAILED_RESPONSE}/{kept.record.key}", kept
        ) from error
    stored = store.write(
        dependency,
        {
            REQUEST_FILE: _json_bytes(request.as_json()),
            RAW_RESPONSE_FILE: response.raw,
            METADATA_FILE: _json_bytes(response.metadata),
        },
    )
    return stored, False


def failed_response_dependency(response: DependencyRecord, failure: int) -> DependencyRecord:
    """Return the dependency record of the ``failure``-th failed call for one response.

    The record has its own artifact name, so ``obtain_response`` never finds a failed call
    when it looks for a stored answer.
    """
    return DependencyRecord(
        stage=response.stage,
        artifact=FAILED_RESPONSE,
        schema_version=response.schema_version,
        code_version=response.code_version,
        configuration={**response.configuration, "failure": failure},
        inputs=response.inputs,
    )


def keep_failed_output(
    store: ArtifactStore,
    response: DependencyRecord,
    request: ModelRequest,
    message: str,
    output: FailedOutput,
) -> StoredArtifact:
    """Store the request and the output of a failed call as the next failure of a response."""
    failure = 1
    while store.find(failed_response_dependency(response, failure)) is not None:
        failure += 1
    report: Json = {"error": message, **output.metadata}
    return store.write(
        failed_response_dependency(response, failure),
        {
            REQUEST_FILE: _json_bytes(request.as_json()),
            STDOUT_FILE: output.stdout,
            STDERR_FILE: output.stderr,
            FAILURE_FILE: _json_bytes(report),
        },
    )


def raw_answer_text(response: StoredArtifact) -> str:
    """Return the answer of the model from the stored raw harness output."""
    raw: Json = json.loads(response.read_bytes(RAW_RESPONSE_FILE))
    text = raw.get("result")
    if not isinstance(text, str):
        raise MalformedResponse("the stored raw response has no 'result' text")
    return text


def analysis_dependency(
    instructions: Instructions, extraction: StoredArtifact, response: StoredArtifact
) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=ANALYSIS,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={
            "instructions_version": instructions.version,
            "model": response.record.dependency.configuration["model"],
        },
        inputs=[
            instructions.identity(),
            _extraction_identity(extraction),
            InputIdentity(
                role="model-response",
                kind="artifact",
                locator=f"{RESPONSE}/{response.record.key}",
                identity=response.record.key,
            ),
        ],
    )


def analyse_chapter(
    store: ArtifactStore,
    instructions: Instructions,
    extraction: StoredArtifact,
    response: StoredArtifact,
) -> tuple[StoredArtifact, bool]:
    """Check the stored response and store the analysis. Reuse a stored analysis."""
    dependency = analysis_dependency(instructions, extraction, response)
    found = store.reuse(dependency)
    if found is not None:
        return found, True
    record = json.loads(extraction.read_bytes(EXTRACTION_FILE))
    answer = parse_answer(raw_answer_text(response))
    model = response.record.dependency.configuration["model"]
    author = {"kind": "model", **model, "instructions_version": instructions.version}
    item_key = record["source"]["source_identity"]["item_key"]
    checked = check_answer(answer, record, f"{item_key}:{response.record.key[:12]}", author)
    analysis: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "source": {
            "item_key": item_key,
            "title": record["source"]["title"],
            "year": record["source"].get("year"),
            "chapter": {"number": record["chapter"]["number"], "title": record["chapter"]["title"]},
            "extraction_artifact": extraction.record.key,
        },
        "instructions": {
            "path": instructions.path.as_posix(),
            "version": instructions.version,
            "sha256": instructions.sha256,
        },
        "model": model,
        "response_artifact": response.record.key,
        "review_note": (
            "Every interpretation has review status 'proposed'. Review and correction "
            "record new versions. They do not edit this artifact."
        ),
        "counts": {
            kind: sum(1 for i in checked.interpretations if i["kind"] == kind) for kind in KINDS
        }
        | {"rejected": len(checked.rejected)},
        "interpretations": checked.interpretations,
        "rejected": checked.rejected,
    }
    stored = store.write(
        dependency,
        {RECORD_FILE: _json_bytes(analysis), SUMMARY_FILE: render_summary(analysis).encode()},
    )
    return stored, False


def render_summary(analysis: Json) -> str:
    """Render a short Markdown summary of one analysis."""
    source = analysis["source"]
    lines = [
        f"# Episode analysis: {source['title']}, chapter {source['chapter']['number']}",
        "",
        f"Instructions version {analysis['instructions']['version']}. "
        f"Model {analysis['model'].get('model')}. All interpretations are proposed.",
        "",
        "| Kind | Count |",
        "| --- | --- |",
    ]
    lines += [f"| {kind} | {count} |" for kind, count in analysis["counts"].items()]
    lines += ["", "## Episodes", ""]
    for item in analysis["interpretations"]:
        if item["kind"] != "episode":
            continue
        pages = [e["printed_page_label"] for e in item["evidence"]]
        span = pages[0] if pages[0] == pages[-1] else f"{pages[0]}–{pages[-1]}"
        lines.append(
            f"- `{item['id']}` **{item['label']}** (pages {span}, uncertainty "
            f"{item['uncertainty']['level']}): {item['description']}"
        )
    if analysis["rejected"]:
        lines += ["", "## Rejected proposals", ""]
        lines += [
            f"- `{r['local_id']}` ({r['kind']}): {'; '.join(r['reasons'])}"
            for r in analysis["rejected"]
        ]
    return "\n".join(lines) + "\n"


@dataclass(frozen=True)
class ChapterOutcome:
    item_key: str
    title: str
    response: StoredArtifact | None = None
    response_reused: bool = False
    analysis: StoredArtifact | None = None
    analysis_reused: bool = False
    error: str | None = None
    failed_response: StoredArtifact | None = None


@dataclass(frozen=True)
class AnalysisRun:
    outcomes: list[ChapterOutcome] = field(default_factory=list[ChapterOutcome])
    report: StoredArtifact | None = None

    @property
    def failed(self) -> bool:
        return any(o.error for o in self.outcomes)


def extracted_chapters(store: ArtifactStore) -> tuple[StoredArtifact, list[Json]]:
    """Return the latest extraction run report and its extracted chapters."""
    key = store.latest_key(EXTRACTION_RUN)
    if key is None:
        raise AnalysisError(
            "no chapter-extraction run is in the store. Run chapter-extraction first"
        )
    report = store.load(EXTRACTION_RUN, key)
    results: list[Json] = json.loads(report.read_bytes("report.json"))["results"]
    return report, [r for r in results if r["status"] == "extracted"]


def _analyse_one(
    store: ArtifactStore,
    instructions: Instructions,
    model: ModelBoundary,
    chapter: Json,
    new_attempt: bool,
) -> ChapterOutcome:
    base = {"item_key": chapter["item_key"], "title": chapter["title"]}
    try:
        extraction = store.load(EXTRACTION, chapter["artifact_key"])
        response, response_reused = obtain_response(
            store, instructions, extraction, model, new_attempt
        )
    except FailedCall as error:
        return ChapterOutcome(**base, error=str(error), failed_response=error.kept)
    except UrgError as error:  # ModelError and StoreError are UrgError subclasses
        return ChapterOutcome(**base, error=str(error))
    try:
        analysis, analysis_reused = analyse_chapter(store, instructions, extraction, response)
    except UrgError as error:
        return ChapterOutcome(
            **base, response=response, response_reused=response_reused, error=str(error)
        )
    return ChapterOutcome(
        **base,
        response=response,
        response_reused=response_reused,
        analysis=analysis,
        analysis_reused=analysis_reused,
    )


def _status(outcome: ChapterOutcome) -> str:
    if outcome.analysis is not None:
        return "analysed"
    return "rejected" if outcome.response is not None else "failed"


def _write_report(
    store: ArtifactStore,
    instructions: Instructions,
    extraction_run: StoredArtifact,
    outcomes: list[ChapterOutcome],
) -> StoredArtifact:
    results: list[Json] = []
    for o in outcomes:
        counts = json.loads(o.analysis.read_bytes(RECORD_FILE))["counts"] if o.analysis else None
        results.append(
            {
                "item_key": o.item_key,
                "title": o.title,
                "status": _status(o),
                "response_key": o.response.record.key if o.response else None,
                "failed_response_key": (
                    o.failed_response.record.key if o.failed_response else None
                ),
                "analysis_key": o.analysis.record.key if o.analysis else None,
                "counts": counts,
                "error": o.error,
            }
        )
    dependency = DependencyRecord(
        stage=STAGE,
        artifact=RUN_REPORT,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"instructions_version": instructions.version, "results": results},
        inputs=[
            instructions.identity(),
            InputIdentity(
                role="chapter-extraction-run",
                kind="artifact",
                locator=f"{EXTRACTION_RUN}/{extraction_run.record.key}",
                identity=extraction_run.record.key,
            ),
        ],
    )
    found = store.reuse(dependency)
    if found is not None:
        return found
    lines = [
        "# Episode analysis run",
        "",
        "| Book | Status | Interpretations | Rejected |",
        "| --- | --- | --- | --- |",
    ]
    for r in results:
        tally: dict[str, int] = r["counts"] or {}
        accepted = sum(v for k, v in tally.items() if k != "rejected")
        lines.append(f"| {r['title']} | {r['status']} | {accepted} | {tally.get('rejected', '')} |")
    errors = [f"- {r['title']}: {r['error']}" for r in results if r["error"]]
    if errors:
        lines += ["", "## Errors", "", *errors]
    report = {"schema_version": SCHEMA_VERSION, "stage": STAGE, "results": results}
    return store.write(
        dependency,
        {"report.json": _json_bytes(report), SUMMARY_FILE: ("\n".join(lines) + "\n").encode()},
    )


def run_analysis(
    store: ArtifactStore,
    instructions_path: Path,
    model: ModelBoundary,
    only: Collection[str] = (),
    new_attempt: bool = False,
) -> AnalysisRun:
    """Analyse every chapter of the latest extraction run, or only the named books."""
    instructions = load_instructions(instructions_path)
    extraction_run, chapters = extracted_chapters(store)
    if only:
        unknown = set(only) - {c["item_key"] for c in chapters}
        if unknown:
            raise AnalysisError(
                f"the latest extraction run has no extracted chapter for {sorted(unknown)}"
            )
        chapters = [c for c in chapters if c["item_key"] in only]
    outcomes = [_analyse_one(store, instructions, model, c, new_attempt) for c in chapters]
    report = _write_report(store, instructions, extraction_run, outcomes)
    return AnalysisRun(outcomes=outcomes, report=report)
