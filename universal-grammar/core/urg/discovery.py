"""Pattern discovery: candidate patterns and grammar fragments from the observations.

The stage reads the latest episode-analysis run. It renders every analysed chapter as an
observation inventory: the sections of the observed document structure and the proposed
interpretations. One model call proposes patterns over the whole inventory. The stage then
checks each pattern against the inventory and stores the result as one discovery run.

A call that fails at the model boundary is kept as ``pattern-discovery-failed-response``
with its request, its output and the failure. A failed call is never reused as an answer.

A discovery run records its settings, its method and its model, so that a later run can
repeat discovery under other preference weights. A pattern is a derived result. It never
replaces the observations that it cites.
"""

from __future__ import annotations

import hashlib
import json
import re
from collections.abc import Iterable
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

from urg.analysis import ANALYSIS, FENCE, FRONT_MATTER, Kind, Uncertainty
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.analysis import RUN_REPORT as ANALYSIS_RUN
from urg.corrections import RUN_REPORT as CORRECTIONS_RUN
from urg.errors import UrgError
from urg.extraction import EXTRACTION
from urg.extraction import RECORD_FILE as EXTRACTION_FILE
from urg.model import FailedOutput, ModelBoundary, ModelError, ModelRequest
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact, canonical_json
from urg.version import code_version

Json = dict[str, Any]

STAGE = "pattern-discovery"
DISCOVERY = "pattern-discovery"
RESPONSE = "pattern-discovery-response"
FAILED_RESPONSE = "pattern-discovery-failed-response"
SCHEMA_VERSION = 1
RECORD_FILE = "discovery.json"
SUMMARY_FILE = "summary.md"
REQUEST_FILE = "request.json"
RAW_RESPONSE_FILE = "raw-response.json"
METADATA_FILE = "metadata.json"
STDOUT_FILE = "stdout"
STDERR_FILE = "stderr"
FAILURE_FILE = "failure.json"
DEFAULT_SETTINGS = Path("docs/discovery/settings.json")
DEFAULT_INSTRUCTIONS = Path("docs/discovery/instructions.md")
DEFAULT_MODEL_CONFIG = Path("docs/discovery/model.json")
PROPOSED = "proposed"

METHOD = {
    "name": "model-proposal-checked-against-inventory",
    "description": (
        "One model call reads the observation inventory of every analysed chapter and "
        "proposes patterns. The stage accepts a pattern only when each cited ID is in the "
        "inventory, no observation repeats in the pattern, the pattern has the minimum "
        "number of supporting instances and each rule has the form 'Name -> Part Part? "
        "Part* Part+'. The stage keeps a rejected pattern with its reasons."
    ),
    "inventory": (
        "The sections of the observed document structure, the interpretations in the "
        "observation scope, and the elements and lines that those interpretations cite."
    ),
    "fit": (
        "An observation fits when an accepted pattern cites it as a supporting instance. A "
        "section also fits when an accepted pattern cites an ancestor section as a "
        "supporting instance. The unfitted list shows the coverage of the proposal. An "
        "observation that the model did not cite stays in the list, also when it fits a "
        "pattern."
    ),
}

# A refinement rule: one name, an arrow and one or more parts on one line. A part is a
# name with an optional modifier: ``?`` optional, ``*`` zero or more, ``+`` one or more.
_NAME = r"[A-Za-z][\w-]*"
_PART = rf"{_NAME}[?*+]?"
RULE = re.compile(rf"[ \t]*{_NAME}[ \t]*->[ \t]*{_PART}(?:[ \t]+{_PART})*[ \t]*")

STRUCTURE_LAYERS = ("structure-section", "structure-element", "evidence-line")


class DiscoveryError(UrgError):
    """The discovery stage cannot run, or the model answer is unusable."""


class MalformedDiscoveryResponse(DiscoveryError):
    """The stored model answer is not a valid discovery answer."""


class FailedDiscoveryCall(DiscoveryError):
    """The model call failed at the boundary. ``kept`` holds its output in the store."""

    def __init__(self, message: str, kept: StoredArtifact) -> None:
        super().__init__(message)
        self.kept = kept


# --- Settings -------------------------------------------------------------------------


class PreferenceWeights(BaseModel):
    """How much the human cares about each book as evidence. A weight is not evidence."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    default: float = Field(gt=0)
    books: dict[str, Annotated[float, Field(gt=0)]] = Field(default_factory=dict[str, float])


class ObservationScope(BaseModel):
    """The observations that the inventory lists."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    interpretation_kinds: list[Kind] = Field(min_length=1)
    structure: list[Literal["section"]]


class DiscoverySettings(BaseModel):
    """The settings file of a discovery run."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    version: int = Field(ge=1)
    minimum_supporting_instances: int = Field(ge=2)
    preference_weights: PreferenceWeights
    observation_scope: ObservationScope


@dataclass(frozen=True)
class Settings:
    path: Path
    value: DiscoverySettings

    def content(self) -> Json:
        return self.value.model_dump(mode="json")

    def identity(self) -> InputIdentity:
        """Identify the settings by their content, so that layout changes do not count."""
        return InputIdentity(
            role="discovery-settings",
            kind="file",
            locator=self.path.as_posix(),
            identity=hashlib.sha256(canonical_json(self.content())).hexdigest(),
        )


def load_settings(path: Path) -> Settings:
    try:
        value = DiscoverySettings.model_validate_json(path.read_bytes())
    except (OSError, ValidationError) as error:
        raise DiscoveryError(f"cannot read the discovery settings {path}: {error}") from error
    return Settings(path=path, value=value)


@dataclass(frozen=True)
class Instructions:
    path: Path
    version: int
    text: str
    sha256: str

    def identity(self) -> InputIdentity:
        return InputIdentity(
            role="discovery-instructions",
            kind="file",
            locator=self.path.as_posix(),
            identity=self.sha256,
        )


def load_instructions(path: Path) -> Instructions:
    """Read the instructions file. Its front matter must give an integer ``version``."""
    try:
        data = path.read_bytes()
    except OSError as error:
        raise DiscoveryError(f"cannot read the discovery instructions {path}: {error}") from error
    match = FRONT_MATTER.match(data.decode())
    if match is None:
        raise DiscoveryError(f"{path} has no front matter between two '---' lines")
    fields = dict(line.split(":", 1) for line in match["head"].splitlines() if ":" in line)
    try:
        version = int(fields["version"].strip())
    except KeyError, ValueError:
        raise DiscoveryError(f"{path} front matter gives no integer version") from None
    return Instructions(
        path=path,
        version=version,
        text=match["body"].strip() + "\n",
        sha256=hashlib.sha256(data).hexdigest(),
    )


# --- Sources and inventory ------------------------------------------------------------


@dataclass(frozen=True)
class Source:
    """One analysed chapter: its analysis and the extraction that the analysis read."""

    item_key: str
    analysis: StoredArtifact
    extraction: StoredArtifact
    analysis_record: Json
    extraction_record: Json

    def identities(self) -> list[InputIdentity]:
        return [
            _artifact_identity("episode-analysis", ANALYSIS, self.analysis),
            _artifact_identity("chapter-extraction", EXTRACTION, self.extraction),
        ]


def _artifact_identity(role: str, name: str, artifact: StoredArtifact) -> InputIdentity:
    key = artifact.record.key
    return InputIdentity(role=role, kind="artifact", locator=f"{name}/{key}", identity=key)


def analysis_results(store: ArtifactStore) -> Json:
    """Return the result list that discovery reads.

    The latest corrections run supplies it when one exists. That run must apply to the
    latest episode-analysis run, so that discovery never reads stale corrections.
    """
    key = store.latest_key(ANALYSIS_RUN)
    if key is None:
        raise DiscoveryError("no episode-analysis run is in the store. Run episode-analysis first")
    corrections = store.latest_key(CORRECTIONS_RUN)
    if corrections is None:
        return json.loads(store.load(ANALYSIS_RUN, key).read_bytes("report.json"))
    report: Json = json.loads(store.load(CORRECTIONS_RUN, corrections).read_bytes("report.json"))
    if report["base_run"] != key:
        raise DiscoveryError(
            f"the latest corrections run applies to episode-analysis run {report['base_run']}, "
            f"not to the latest run {key}. Run corrections again"
        )
    return report


def analysed_sources(store: ArtifactStore) -> list[Source]:
    """Return the analysed chapters that discovery reads, ordered by key."""
    report = analysis_results(store)
    sources: list[Source] = []
    for result in report["results"]:
        if result["status"] != "analysed":
            continue
        analysis = store.load(ANALYSIS, result["analysis_key"])
        analysis_record: Json = json.loads(analysis.read_bytes(ANALYSIS_FILE))
        extraction = store.load(EXTRACTION, analysis_record["source"]["extraction_artifact"])
        sources.append(
            Source(
                item_key=result["item_key"],
                analysis=analysis,
                extraction=extraction,
                analysis_record=analysis_record,
                extraction_record=json.loads(extraction.read_bytes(EXTRACTION_FILE)),
            )
        )
    if not sources:
        raise DiscoveryError("the latest analysis run has no analysed chapter")
    return sorted(sources, key=lambda s: s.item_key)


@dataclass
class Inventory:
    """The observations that a pattern can cite, by ID.

    ``listed`` holds the IDs that the prompt lists one by one. The list of observations
    that no pattern supports covers exactly these IDs.
    """

    entries: dict[str, Json] = field(default_factory=dict[str, Json])
    listed: list[str] = field(default_factory=list[str])
    parents: dict[str, str | None] = field(default_factory=dict[str, str | None])

    def add(self, entry: Json, listed: bool) -> None:
        if entry["id"] not in self.entries:
            self.entries[entry["id"]] = entry
            if listed:
                self.listed.append(entry["id"])
        elif listed and entry["id"] not in self.listed:
            self.listed.append(entry["id"])

    def ancestors(self, section_id: str) -> Iterable[str]:
        parent = self.parents.get(section_id)
        while parent is not None:
            yield parent
            parent = self.parents.get(parent)


def _in_scope(settings: DiscoverySettings, interpretation: Json) -> bool:
    return interpretation["kind"] in settings.observation_scope.interpretation_kinds


def build_inventory(sources: list[Source], settings: DiscoverySettings) -> Inventory:
    inventory = Inventory()
    for source in sources:
        structure = source.extraction_record["structure"]
        elements = {e["id"]: e for e in structure["elements"]}
        if "section" in settings.observation_scope.structure:
            for section in structure.get("sections", []):
                heading = elements.get(section.get("heading") or "")
                inventory.parents[section["id"]] = section.get("parent")
                inventory.add(
                    {
                        "id": section["id"],
                        "layer": "structure-section",
                        "item_key": source.item_key,
                        "title": section["title"],
                        "level": section["level"],
                        "pdf_page_index": section["pdf_page_index"],
                        "printed_page_label": heading["printed_page_label"] if heading else None,
                    },
                    listed=True,
                )
        for interpretation in source.analysis_record["interpretations"]:
            if not _in_scope(settings, interpretation):
                continue
            inventory.add(
                {
                    "id": interpretation["id"],
                    "layer": "interpretation",
                    "item_key": source.item_key,
                    "kind": interpretation["kind"],
                    "label": interpretation["label"],
                },
                listed=True,
            )
            for evidence in interpretation["evidence"]:
                entry = {
                    "id": evidence["id"],
                    "layer": evidence["layer"],
                    "item_key": source.item_key,
                    "pdf_page_index": evidence["pdf_page_index"],
                    "printed_page_label": evidence["printed_page_label"],
                }
                if "kind" in evidence:
                    entry["kind"] = evidence["kind"]
                inventory.add(entry, listed=False)
    return inventory


# --- Prompt ---------------------------------------------------------------------------


def resolved_weights(sources: list[Source], settings: DiscoverySettings) -> dict[str, float]:
    weights = settings.preference_weights
    unknown = sorted(set(weights.books) - {s.item_key for s in sources})
    if unknown:
        raise DiscoveryError(
            f"the settings give preference weights for books outside the analysis run: "
            f"{', '.join(unknown)}"
        )
    return {s.item_key: weights.books.get(s.item_key, weights.default) for s in sources}


def _kind_runs(kinds: list[str]) -> str:
    runs: list[tuple[str, int]] = []
    for kind in kinds:
        if runs and runs[-1][0] == kind:
            runs[-1] = (kind, runs[-1][1] + 1)
        else:
            runs.append((kind, 1))
    return ", ".join(kind if count == 1 else f"{kind}×{count}" for kind, count in runs)


def _references(interpretation: Json) -> str:
    parts = [
        f"{name}: {interpretation[name]['id']}"
        for name in ("episode", "source", "target", "alternative_to")
        if isinstance(interpretation.get(name), dict)
    ]
    parts.append("evidence: " + ", ".join(e["id"] for e in interpretation["evidence"]))
    return "; ".join(parts)


def render_prompt(
    sources: list[Source], settings: DiscoverySettings, weights: dict[str, float]
) -> str:
    """Render the observation inventory in the form that the instructions describe."""
    lines = [
        f"Minimum number of supporting instances for a pattern: "
        f"{settings.minimum_supporting_instances}",
        "",
    ]
    for source in sources:
        record = source.extraction_record
        chapter = record["chapter"]
        elements = {e["id"]: e for e in record["structure"]["elements"]}
        lines += [
            f"# Book {source.item_key}: {record['source']['title']} "
            f"({record['source'].get('year') or 'no year'})",
            f"Chapter {chapter['number']}: {chapter['title']}",
            f"Preference weight: {weights[source.item_key]:g}",
            "",
        ]
        if "section" in settings.observation_scope.structure:
            lines += ["## Document structure", ""]
            for section in record["structure"].get("sections", []):
                heading = elements.get(section.get("heading") or "")
                label = heading["printed_page_label"] if heading else "unknown"
                kinds = [elements[e]["kind"] for e in section["elements"] if e in elements]
                lines.append(
                    f"[{section['id']}] level {section['level']}, page {label}: "
                    f"{section['title']} | {_kind_runs(kinds)}"
                )
            lines.append("")
        lines += ["## Interpretations", ""]
        for interpretation in source.analysis_record["interpretations"]:
            if not _in_scope(settings, interpretation):
                continue
            label = interpretation["label"] or ""
            lines.append(
                f'[{interpretation["id"]}] {interpretation["kind"]} "{label}": '
                f"{interpretation['description']} {{{_references(interpretation)}}}"
            )
        lines.append("")
    return "\n".join(lines)


def build_request(instructions: Instructions, prompt: str) -> ModelRequest:
    return ModelRequest(system=instructions.text, prompt=prompt)


# --- Model answer ---------------------------------------------------------------------

LocalId = Annotated[str, StringConstraints(pattern=r"^[A-Za-z0-9._-]+$")]
Feature = Literal["hierarchy", "cross-reference", "attribute", "refinement-rule"]
FEATURES: tuple[Feature, ...] = ("hierarchy", "cross-reference", "attribute", "refinement-rule")
ROLES = ("supporting", "exceptions", "unresolved")


class Citation(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)

    observation: str = Field(min_length=1)
    note: str = Field(min_length=1)


class RepresentationNeed(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)

    feature: Feature
    justification: str = Field(min_length=1)


class ProposedPattern(BaseModel):
    """One pattern as the model proposes it, before the inventory check."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    local_id: LocalId
    name: str = Field(min_length=1)
    description: str = Field(min_length=1)
    # The minimum is a per-pattern check, so an empty list rejects only this pattern.
    supporting: list[Citation]
    exceptions: list[Citation] = Field(default_factory=list[Citation])
    unresolved: list[Citation] = Field(default_factory=list[Citation])
    rules: list[str] = Field(default_factory=list[str])
    representation: list[RepresentationNeed] = Field(default_factory=list[RepresentationNeed])
    uncertainty: Uncertainty


class DiscoveryAnswer(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)

    patterns: list[ProposedPattern]

    @model_validator(mode="after")
    def _check_unique(self) -> Self:
        seen: set[str] = set()
        for pattern in self.patterns:
            if pattern.local_id in seen:
                raise ValueError(f"the local id {pattern.local_id} occurs twice")
            seen.add(pattern.local_id)
        return self


def parse_answer(text: str) -> DiscoveryAnswer:
    """Parse the model answer, or raise ``MalformedDiscoveryResponse``.

    The stage accepts one surrounding Markdown fence, like the analysis stage.
    """
    body = text.strip()
    fenced = FENCE.match(body)
    if fenced is not None:
        body = fenced["body"].strip()
    try:
        return DiscoveryAnswer.model_validate_json(body)
    except ValidationError as error:
        problems = "; ".join(
            f"{'.'.join(str(p) for p in e['loc']) or 'answer'}: {e['msg']}"
            for e in error.errors()[:10]
        )
        raise MalformedDiscoveryResponse(f"the model answer is malformed: {problems}") from None


# --- Inventory check ------------------------------------------------------------------


def _problems(pattern: ProposedPattern, inventory: Inventory, minimum: int) -> list[str]:
    problems: list[str] = []
    seen: set[str] = set()
    for role in ROLES:
        for citation in getattr(pattern, role):
            if citation.observation not in inventory.entries:
                problems.append(f"{role}: {citation.observation} is not in the inventory")
            if citation.observation in seen:
                problems.append(f"{role}: {citation.observation} is cited twice")
            seen.add(citation.observation)
    if len(pattern.supporting) < minimum:
        problems.append(
            f"supporting: {len(pattern.supporting)} instances, fewer than the minimum {minimum}"
        )
    for rule in pattern.rules:
        if not RULE.fullmatch(rule):
            problems.append(
                f"rules: {rule!r} does not have the form 'Name -> Part Part? Part* Part+'"
            )
    return problems


def _cite(citation: Citation, inventory: Inventory) -> Json:
    entry = inventory.entries[citation.observation]
    return {
        "observation": {"layer": entry["layer"], "id": entry["id"]},
        "item_key": entry["item_key"],
        "note": citation.note,
    }


def _basis(supporting: list[Json]) -> str:
    layers = {c["observation"]["layer"] for c in supporting}
    structural = any(layer in STRUCTURE_LAYERS for layer in layers)
    interpretive = "interpretation" in layers
    if structural and interpretive:
        return "document-structure-and-interpretation"
    return "document-structure" if structural else "interpretation"


@dataclass(frozen=True)
class Checked:
    patterns: list[Json]
    rejected: list[Json]


def check_answer(
    answer: DiscoveryAnswer, inventory: Inventory, minimum: int, prefix: str, author: Json
) -> Checked:
    """Accept each pattern whose citations resolve, and keep the others with reasons.

    The stage numbers the accepted patterns in answer order, so the IDs are stable for one
    stored response.
    """
    patterns: list[Json] = []
    rejected: list[Json] = []
    for proposal in answer.patterns:
        problems = _problems(proposal, inventory, minimum)
        if problems:
            rejected.append(
                {
                    "local_id": proposal.local_id,
                    "name": proposal.name,
                    "reasons": problems,
                    "proposal": proposal.model_dump(mode="json"),
                }
            )
            continue
        cited = {role: [_cite(c, inventory) for c in getattr(proposal, role)] for role in ROLES}
        books = sorted({c["item_key"] for role in ROLES for c in cited[role]})
        patterns.append(
            {
                "id": f"{prefix}:p{len(patterns) + 1:03d}",
                "version": 1,
                "local_id": proposal.local_id,
                "name": proposal.name,
                "description": proposal.description,
                "basis": _basis(cited["supporting"]),
                "books": books,
                **cited,
                "rules": proposal.rules,
                "representation": [r.model_dump(mode="json") for r in proposal.representation],
                "uncertainty": proposal.uncertainty.model_dump(mode="json"),
                "author": author,
                "review_status": PROPOSED,
            }
        )
    return Checked(patterns=patterns, rejected=rejected)


def unfitted(patterns: list[Json], inventory: Inventory) -> list[Json]:
    """Return each listed observation that no pattern cites as a supporting instance.

    A section also fits when a pattern supports one of its ancestor sections. Each entry
    names the patterns that cite the observation in another role.
    """
    supported = {c["observation"]["id"] for p in patterns for c in p["supporting"]}
    other: dict[str, list[Json]] = {}
    for pattern in patterns:
        for role in ("exceptions", "unresolved"):
            for citation in pattern[role]:
                other.setdefault(citation["observation"]["id"], []).append(
                    {"pattern": pattern["id"], "role": role}
                )
    result: list[Json] = []
    for observation_id in inventory.listed:
        if observation_id in supported:
            continue
        entry = inventory.entries[observation_id]
        if entry["layer"] == "structure-section" and any(
            a in supported for a in inventory.ancestors(observation_id)
        ):
            continue
        result.append({**entry, "cited_as": other.get(observation_id, [])})
    return result


# --- Store ----------------------------------------------------------------------------


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _source_identities(sources: list[Source]) -> list[InputIdentity]:
    return [identity for source in sources for identity in source.identities()]


def response_dependency(
    settings: Settings,
    instructions: Instructions,
    sources: list[Source],
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
        inputs=[settings.identity(), instructions.identity(), *_source_identities(sources)],
    )


def obtain_response(
    store: ArtifactStore,
    settings: Settings,
    instructions: Instructions,
    sources: list[Source],
    model: ModelBoundary,
    request: ModelRequest,
    new_attempt: bool,
) -> tuple[StoredArtifact, bool]:
    """Return the newest stored response for this request, or call the model.

    The second value is true when a stored response was reused.
    """

    def dependency(attempt: int) -> DependencyRecord:
        return response_dependency(settings, instructions, sources, model, request, attempt)

    newest: StoredArtifact | None = None
    attempt = 1
    while (found := store.find(dependency(attempt))) is not None:
        newest = found
        attempt += 1
    if newest is not None and not new_attempt:
        # Move the latest pointer, as every other reused artifact does.
        return store.reuse(newest.record.dependency) or newest, True
    try:
        response = model.call(request)
    except ModelError as error:
        if error.output is None:
            raise
        kept = keep_failed_output(store, dependency(attempt), request, str(error), error.output)
        raise FailedDiscoveryCall(
            f"{error}. The output is kept as {FAILED_RESPONSE}/{kept.record.key}", kept
        ) from error
    stored = store.write(
        dependency(attempt),
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
    report: Json = {**output.metadata, "status": "failed", "error": message}
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
    raw: Json = json.loads(response.read_bytes(RAW_RESPONSE_FILE))
    text = raw.get("result")
    if not isinstance(text, str):
        raise MalformedDiscoveryResponse("the stored raw response has no 'result' text")
    return text


def discovery_dependency(
    settings: Settings,
    instructions: Instructions,
    sources: list[Source],
    weights: dict[str, float],
    response: StoredArtifact,
) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=DISCOVERY,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={
            "settings": settings.content(),
            "preference_weights": weights,
            "instructions_version": instructions.version,
            "model": response.record.dependency.configuration["model"],
        },
        inputs=[
            settings.identity(),
            instructions.identity(),
            *_source_identities(sources),
            _artifact_identity("model-response", RESPONSE, response),
        ],
    )


def _source_entry(source: Source, weight: float) -> Json:
    record = source.extraction_record
    return {
        "item_key": source.item_key,
        "title": record["source"]["title"],
        "year": record["source"].get("year"),
        "chapter": {"number": record["chapter"]["number"], "title": record["chapter"]["title"]},
        "analysis_artifact": source.analysis.record.key,
        "extraction_artifact": source.extraction.record.key,
        "preference_weight": weight,
    }


def _counts(inventory: Inventory, checked: Checked, unfit: list[Json]) -> Json:
    layers: dict[str, int] = {}
    for observation_id in inventory.listed:
        layer = inventory.entries[observation_id]["layer"]
        layers[layer] = layers.get(layer, 0) + 1
    return {
        "listed_observations": layers,
        "patterns": len(checked.patterns),
        "rejected": len(checked.rejected),
        "unfitted": len(unfit),
    }


def build_record(
    settings: Settings,
    instructions: Instructions,
    sources: list[Source],
    weights: dict[str, float],
    response: StoredArtifact,
    answer: DiscoveryAnswer,
) -> Json:
    model = response.record.dependency.configuration["model"]
    attempt = response.record.dependency.configuration["attempt"]
    inventory = build_inventory(sources, settings.value)
    author = {"kind": "model", **model, "instructions_version": instructions.version}
    checked = check_answer(
        answer,
        inventory,
        settings.value.minimum_supporting_instances,
        f"pattern:{response.record.key[:12]}",
        author,
    )
    unfit = unfitted(checked.patterns, inventory)
    return {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "discovery_run": {
            "method": METHOD,
            "settings": {
                "path": settings.path.as_posix(),
                "sha256": settings.identity().identity,
                "content": settings.content(),
            },
            "preference_weights": weights,
            "instructions": {
                "path": instructions.path.as_posix(),
                "version": instructions.version,
                "sha256": instructions.sha256,
            },
            "model": model,
            "response_artifact": response.record.key,
            "attempt": attempt,
        },
        "sources": [_source_entry(s, weights[s.item_key]) for s in sources],
        "review_note": (
            "A model proposed every pattern. No human reviewed them. Each pattern is a "
            "candidate and never replaces the observations that it cites."
        ),
        "counts": _counts(inventory, checked, unfit),
        "inventory": {i: inventory.entries[i] for i in sorted(inventory.entries)},
        "patterns": checked.patterns,
        "rejected": checked.rejected,
        "unfitted": unfit,
    }


def render_summary(record: Json) -> str:
    run = record["discovery_run"]
    lines = [
        "# Pattern discovery",
        "",
        f"Model: {run['model'].get('model')} (attempt {run['attempt']}). "
        f"Instructions version {run['instructions']['version']}. "
        f"Settings version {run['settings']['content']['version']}.",
        "",
        record["review_note"],
        "",
        "| Pattern | Basis | Books | Supporting | Exceptions | Unresolved |",
        "|---|---|---|---|---|---|",
    ]
    for p in record["patterns"]:
        lines.append(
            f"| {p['name']} | {p['basis']} | {', '.join(p['books'])} | {len(p['supporting'])} "
            f"| {len(p['exceptions'])} | {len(p['unresolved'])} |"
        )
    counts = record["counts"]
    lines += [
        "",
        f"Rejected patterns: {counts['rejected']}. "
        f"Observations that no pattern supports: {counts['unfitted']}.",
        "",
    ]
    return "\n".join(lines)


@dataclass(frozen=True)
class DiscoveryRun:
    discovery: StoredArtifact
    response_reused: bool
    discovery_reused: bool
    record: Json


def run_discovery(
    store: ArtifactStore,
    settings_path: Path,
    instructions_path: Path,
    model: ModelBoundary,
    new_attempt: bool = False,
) -> DiscoveryRun:
    """Discover patterns over the latest analysis run. Reuse a stored discovery run.

    With unchanged observations, settings, instructions and model, the stage reuses the
    stored response and the stored discovery run. ``new_attempt`` calls the model again.
    """
    settings = load_settings(settings_path)
    instructions = load_instructions(instructions_path)
    sources = analysed_sources(store)
    weights = resolved_weights(sources, settings.value)
    request = build_request(instructions, render_prompt(sources, settings.value, weights))
    response, response_reused = obtain_response(
        store, settings, instructions, sources, model, request, new_attempt
    )
    dependency = discovery_dependency(settings, instructions, sources, weights, response)
    found = store.reuse(dependency)
    if found is not None:
        record: Json = json.loads(found.read_bytes(RECORD_FILE))
        return DiscoveryRun(found, response_reused, True, record)
    try:
        answer = parse_answer(raw_answer_text(response))
    except MalformedDiscoveryResponse as error:
        raise MalformedDiscoveryResponse(
            f"{error}. The response is kept as {RESPONSE}/{response.record.key}. "
            f"Rerun with --new-attempt to call the model again"
        ) from None
    record = build_record(settings, instructions, sources, weights, response, answer)
    stored = store.write(
        dependency,
        {RECORD_FILE: _json_bytes(record), SUMMARY_FILE: render_summary(record).encode()},
    )
    return DiscoveryRun(stored, response_reused, False, record)
