"""Record corrections to interpretations as new interpretation versions.

A correction is a JSON file that a human or a reviewer writes. It names one interpretation
and the version that it corrects, the change, the rationale and the author. The stage
applies every correction again on each run, in file-name order, to the analyses of the
latest episode-analysis run. It never changes evidence or an earlier version.

A book with corrections gets a new ``episode-analysis`` artifact that holds the current
version of each interpretation and every superseded version. A book without corrections
keeps its original artifact. The run report has the result list of an episode-analysis
run, so that pattern discovery reads it in the same way.
"""

from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Literal, Self

from pydantic import BaseModel, ConfigDict, Field, ValidationError, model_validator

from urg.analysis import ANALYSIS, KINDS, Uncertainty, evidence_index, render_summary
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.analysis import RUN_REPORT as ANALYSIS_RUN
from urg.analysis import SUMMARY_FILE as ANALYSIS_SUMMARY
from urg.errors import UrgError
from urg.extraction import EXTRACTION
from urg.extraction import RECORD_FILE as EXTRACTION_FILE
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.version import code_version

Json = dict[str, Any]

STAGE = "corrections"
RUN_REPORT = "analysis-corrections-run"
SCHEMA_VERSION = 1
REPORT_FILE = "report.json"
SUMMARY_FILE = "summary.md"
CORRECTED = "corrected"
DEFAULT_DIRECTORY = Path("corrections")
REFERENCES: dict[str, tuple[str, ...]] = {
    "move": ("episode",),
    "relationship": ("source", "target"),
}


class CorrectionError(UrgError):
    """The corrections stage cannot run."""


class Refused(CorrectionError):
    """One correction cannot apply. The stage records the reason and goes on."""


class Author(BaseModel):
    model_config = ConfigDict(extra="forbid")

    kind: Literal["human", "reviewer"]
    name: str = Field(min_length=1)


class Target(BaseModel):
    model_config = ConfigDict(extra="forbid")

    id: str = Field(min_length=1)
    version: int = Field(ge=1)


class Revision(BaseModel):
    """The fields that a correction replaces. A field that the file omits stays as it is."""

    model_config = ConfigDict(extra="forbid")

    label: str | None = None
    description: str | None = Field(default=None, min_length=1)
    evidence: list[str] | None = Field(default=None, min_length=1)
    uncertainty: Uncertainty | None = None
    episode: str | None = None
    source: str | None = None
    target: str | None = None


class Reading(BaseModel):
    """An alternative reading of the target interpretation."""

    model_config = ConfigDict(extra="forbid")

    label: str | None = None
    description: str = Field(min_length=1)
    evidence: list[str] = Field(min_length=1)
    uncertainty: Uncertainty


class Correction(BaseModel):
    model_config = ConfigDict(extra="forbid")

    id: str = Field(pattern=r"^[a-z0-9][a-z0-9-]*$")
    author: Author
    origin: str = Field(min_length=1)
    rationale: str = Field(min_length=1)
    target: Target
    operation: Literal["revise", "add-alternative-reading"]
    revise: Revision | None = None
    reading: Reading | None = None

    @model_validator(mode="after")
    def _check_operation(self) -> Self:
        if self.operation == "revise":
            if self.revise is None or self.reading is not None:
                raise ValueError("a 'revise' correction needs 'revise' and no 'reading'")
            if not self.revise.model_fields_set:
                raise ValueError("'revise' must name at least one field")
        elif self.reading is None or self.revise is not None:
            raise ValueError("an 'add-alternative-reading' correction needs 'reading' only")
        return self


@dataclass(frozen=True)
class CorrectionFile:
    path: Path
    sha256: str
    correction: Correction | None
    error: str | None

    def identity(self) -> InputIdentity:
        return InputIdentity(
            role="correction", kind="file", locator=self.path.as_posix(), identity=self.sha256
        )


def load_corrections(directory: Path) -> list[CorrectionFile]:
    """Read every correction file of the directory, in file-name order.

    A missing directory holds no corrections. A file that does not validate stays in the
    list with its error, so that the run report can name it.
    """
    if not directory.exists():
        return []
    if not directory.is_dir():
        raise CorrectionError(f"{directory} is not a directory")
    files: list[CorrectionFile] = []
    for path in sorted(directory.glob("*.json")):
        data = path.read_bytes()
        digest = hashlib.sha256(data).hexdigest()
        try:
            files.append(CorrectionFile(path, digest, Correction.model_validate_json(data), None))
        except ValidationError as e:
            files.append(CorrectionFile(path, digest, None, f"invalid correction file: {e}"))
    return files


@dataclass
class Book:
    """The analysis of one book, and its interpretations as the corrections change them."""

    item_key: str
    base: StoredArtifact
    record: Json
    evidence: dict[str, Json]
    current: dict[str, Json]
    history: list[Json] = field(default_factory=list[Json])
    applied: list[str] = field(default_factory=list[str])
    files: list[CorrectionFile] = field(default_factory=list[CorrectionFile])


def _load_books(store: ArtifactStore, report: Json) -> dict[str, Book]:
    books: dict[str, Book] = {}
    for result in report["results"]:
        if result["status"] != "analysed":
            continue
        base = store.load(ANALYSIS, result["analysis_key"])
        record: Json = json.loads(base.read_bytes(ANALYSIS_FILE))
        extraction = store.load(EXTRACTION, record["source"]["extraction_artifact"])
        books[result["item_key"]] = Book(
            item_key=result["item_key"],
            base=base,
            record=record,
            evidence=evidence_index(json.loads(extraction.read_bytes(EXTRACTION_FILE))),
            current={i["id"]: copy.deepcopy(i) for i in record["interpretations"]},
        )
    return books


def _reference(book: Book, name: str, value: str) -> Json:
    """Resolve a reference the way the analysis stage does."""
    if value in book.current:
        kind = book.current[value]["kind"]
        if name == "episode" and kind != "episode":
            raise Refused(f"'episode' names {value}, which is a {kind}")
        return {"layer": "interpretation", "id": value}
    if name in ("source", "target") and value in book.evidence:
        return {"layer": book.evidence[value]["layer"], "id": value}
    raise Refused(f"'{name}' names {value}, which is neither an interpretation nor evidence")


def _evidence(book: Book, ids: list[str]) -> list[Json]:
    missing = [e for e in ids if e not in book.evidence]
    if missing:
        raise Refused(f"cites evidence that the extraction does not contain: {missing}")
    return [book.evidence[e] for e in dict.fromkeys(ids)]


def _version_block(correction: Correction, revises: int | None) -> Json:
    return {
        "id": correction.id,
        "origin": correction.origin,
        "rationale": correction.rationale,
        "revises_version": revises,
    }


def _revise(book: Book, current: Json, correction: Correction, revision: Revision) -> Json:
    allowed = REFERENCES.get(current["kind"], ())
    changed = revision.model_fields_set
    for name in ("episode", "source", "target"):
        if name in changed and name not in allowed:
            raise Refused(f"a {current['kind']} has no '{name}' reference")
        if name in changed and getattr(revision, name) is None:
            raise Refused(f"'{name}' cannot be removed from a {current['kind']}")
    new = copy.deepcopy(current)
    if "label" in changed:
        new["label"] = revision.label
    if revision.description is not None:
        new["description"] = revision.description
    if revision.evidence is not None:
        new["evidence"] = _evidence(book, revision.evidence)
    if revision.uncertainty is not None:
        new["uncertainty"] = revision.uncertainty.model_dump()
    for name in ("episode", "source", "target"):
        value = getattr(revision, name)
        if value is not None:
            if value == current["id"]:
                raise Refused(f"'{name}' names the interpretation itself")
            new[name] = _reference(book, name, value)
    new["version"] = current["version"] + 1
    new["author"] = correction.author.model_dump()
    new["review_status"] = CORRECTED
    new["correction"] = _version_block(correction, current["version"])
    return new


def _reading(book: Book, target: Json, correction: Correction, reading: Reading) -> Json:
    return {
        "id": f"{book.item_key}:{correction.id}:i001",
        "version": 1,
        "kind": "alternative-reading",
        "label": reading.label,
        "description": reading.description,
        "evidence": _evidence(book, reading.evidence),
        "uncertainty": reading.uncertainty.model_dump(),
        "author": correction.author.model_dump(),
        "review_status": CORRECTED,
        "local_id": correction.id,
        "alternative_to": {"layer": "interpretation", "id": target["id"]},
        "correction": _version_block(correction, None),
    }


def _apply(books: dict[str, Book], owners: dict[str, str], correction: Correction) -> Json:
    """Apply one correction, and return the interpretation version that it creates."""
    item_key = owners.get(correction.target.id)
    if item_key is None:
        raise Refused(f"no analysed book holds interpretation {correction.target.id}")
    book = books[item_key]
    current = book.current[correction.target.id]
    if current["version"] != correction.target.version:
        raise Refused(
            f"target version {correction.target.version} of {correction.target.id} is not "
            f"the current version {current['version']}"
        )
    if correction.revise is not None:
        new = _revise(book, current, correction, correction.revise)
        book.history.append(current | {"superseded_by": correction.id})
    else:
        assert correction.reading is not None
        new = _reading(book, current, correction, correction.reading)
        if new["id"] in book.current:
            raise Refused(f"interpretation {new['id']} exists already")
        owners[new["id"]] = item_key
    book.current[new["id"]] = new
    book.applied.append(correction.id)
    return new


def _corrected_record(book: Book) -> Json:
    record = copy.deepcopy(book.record)
    record["interpretations"] = list(book.current.values())
    counts = {kind: 0 for kind in KINDS}
    for interpretation in record["interpretations"]:
        counts[interpretation["kind"]] += 1
    record["counts"] = counts | {"rejected": len(record["rejected"])}
    record["revision"] = {
        "base_analysis_artifact": book.base.record.key,
        "corrections": book.applied,
    }
    record["history"] = book.history
    return record


def render_corrected_summary(record: Json) -> str:
    """Render the analysis summary, then the corrections and the superseded versions."""
    corrected = [i for i in record["interpretations"] if i["review_status"] == CORRECTED]
    text = render_summary(record).replace(
        "All interpretations are proposed.",
        f"{len(corrected)} interpretation versions come from corrections. "
        "A model proposed the others.",
    )
    lines = ["", "## Corrections", ""]
    for item in corrected:
        block = item["correction"]
        lines.append(
            f"- `{item['id']}` version {item['version']}, correction `{block['id']}` by "
            f"{item['author']['name']} ({block['origin']}): {block['rationale']}"
        )
    lines += ["", "## Superseded versions", ""]
    for item in record["history"]:
        evidence = ", ".join(e["id"] for e in item["evidence"])
        lines.append(
            f"- `{item['id']}` version {item['version']}, superseded by "
            f"`{item['superseded_by']}`: {item['label']}. {item['description']} "
            f"Evidence: {evidence}."
        )
    return text + "\n".join(lines) + "\n"


def _artifact_identity(role: str, name: str, artifact: StoredArtifact) -> InputIdentity:
    key = artifact.record.key
    return InputIdentity(role=role, kind="artifact", locator=f"{name}/{key}", identity=key)


def corrected_dependency(book: Book) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=ANALYSIS,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"corrections": book.applied},
        inputs=[
            _artifact_identity("base-analysis", ANALYSIS, book.base),
            *(f.identity() for f in book.files),
        ],
    )


def run_dependency(
    base_run: StoredArtifact, files: list[CorrectionFile], outcomes: list[Json]
) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=RUN_REPORT,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={
            "outcomes": [{"file": o["file"], "status": o["status"]} for o in outcomes],
        },
        inputs=[
            _artifact_identity("episode-analysis-run", ANALYSIS_RUN, base_run),
            *(f.identity() for f in files),
        ],
    )


@dataclass(frozen=True)
class CorrectedBook:
    item_key: str
    analysis: StoredArtifact
    reused: bool


@dataclass(frozen=True)
class CorrectionRun:
    run: StoredArtifact
    reused: bool
    outcomes: list[Json]
    corrected: list[CorrectedBook]

    @property
    def refused(self) -> list[Json]:
        return [o for o in self.outcomes if o["status"] == "refused"]


def _outcome(file: CorrectionFile) -> Json:
    correction = file.correction
    return {
        "file": file.path.as_posix(),
        "sha256": file.sha256,
        "id": correction.id if correction else None,
        "operation": correction.operation if correction else None,
        "target": correction.target.model_dump() if correction else None,
        "author": correction.author.model_dump() if correction else None,
        "status": "refused",
        "reason": file.error,
        "item_key": None,
        "interpretation": None,
    }


def render_run_summary(report: Json) -> str:
    lines = [
        "# Corrections",
        "",
        f"Base episode-analysis run `{report['base_run']}`.",
        "",
        "| File | Correction | Target | Status | Result or reason |",
        "| --- | --- | --- | --- | --- |",
    ]
    for o in report["corrections"]:
        target = f"`{o['target']['id']}` v{o['target']['version']}" if o["target"] else ""
        result = (
            f"`{o['interpretation']['id']}` v{o['interpretation']['version']}"
            if o["status"] == "applied"
            else (o["reason"] or "").replace("\n", " ").replace("|", "/")
        )
        lines.append(f"| {o['file']} | {o['id'] or ''} | {target} | {o['status']} | {result} |")
    lines += ["", "| Book | Analysis | Base analysis |", "| --- | --- | --- |"]
    for r in report["results"]:
        if r["status"] == "analysed":
            keys = f"`{r['analysis_key']}` | `{r['base_analysis_key']}`"
            lines.append(f"| {r['item_key']} | {keys} |")
    return "\n".join(lines) + "\n"


def run_corrections(store: ArtifactStore, directory: Path = DEFAULT_DIRECTORY) -> CorrectionRun:
    """Apply the correction files to the latest episode-analysis run.

    A refused correction does not stop the others. The run report records its reason.
    """
    base_key = store.latest_key(ANALYSIS_RUN)
    if base_key is None:
        raise CorrectionError("no episode-analysis run is in the store. Run episode-analysis first")
    base_run = store.load(ANALYSIS_RUN, base_key)
    base_report: Json = json.loads(base_run.read_bytes("report.json"))
    books = _load_books(store, base_report)
    owners = {i: book.item_key for book in books.values() for i in book.current}
    files = load_corrections(directory)

    outcomes: list[Json] = []
    seen: set[str] = set()
    for file in files:
        outcome = _outcome(file)
        outcomes.append(outcome)
        correction = file.correction
        if correction is None:
            continue
        try:
            if correction.id in seen:
                raise Refused(f"correction ID {correction.id} occurs in an earlier file")
            seen.add(correction.id)
            new = _apply(books, owners, correction)
        except Refused as e:
            outcome["reason"] = str(e)
            continue
        book = books[owners[correction.target.id]]
        book.files.append(file)
        outcome |= {
            "status": "applied",
            "item_key": book.item_key,
            "interpretation": {"id": new["id"], "version": new["version"]},
        }

    corrected: list[CorrectedBook] = []
    keys: dict[str, str] = {}
    for book in books.values():
        if not book.applied:
            continue
        dependency = corrected_dependency(book)
        found = store.reuse(dependency)
        if found is None:
            record = _corrected_record(book)
            found = store.write(
                dependency,
                {
                    ANALYSIS_FILE: (json.dumps(record, indent=2) + "\n").encode(),
                    ANALYSIS_SUMMARY: render_corrected_summary(record).encode(),
                },
            )
            corrected.append(CorrectedBook(book.item_key, found, False))
        else:
            corrected.append(CorrectedBook(book.item_key, found, True))
        keys[book.item_key] = found.record.key

    results: list[Json] = []
    for result in base_report["results"]:
        entry = copy.deepcopy(result)
        if result["status"] == "analysed":
            book = books[result["item_key"]]
            entry["base_analysis_key"] = result["analysis_key"]
            entry["analysis_key"] = keys.get(result["item_key"], result["analysis_key"])
            entry["corrected"] = result["item_key"] in keys
            entry["counts"] = _corrected_record(book)["counts"]
        results.append(entry)
    report: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "base_run": base_key,
        "corrections": outcomes,
        "results": results,
    }
    dependency = run_dependency(base_run, files, outcomes)
    found = store.reuse(dependency)
    if found is not None:
        return CorrectionRun(found, True, outcomes, corrected)
    stored = store.write(
        dependency,
        {
            REPORT_FILE: (json.dumps(report, indent=2) + "\n").encode(),
            SUMMARY_FILE: render_run_summary(report).encode(),
        },
    )
    return CorrectionRun(stored, False, outcomes, corrected)
