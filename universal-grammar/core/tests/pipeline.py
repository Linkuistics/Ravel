"""The whole pipeline on fixtures, run through the stage commands.

The fixture library is the catalogue fixture of ``tests/fixture.py``. Two of its books get a
five-page chapter book as their PDF. The model boundary is ``ScriptedModel``. It builds each
answer from the ids in the request, so the answers fit any extraction and need no recording.

``python -m tests.pipeline kill POINT TARGET -- ARGS`` runs one stage command in a child
process and kills that process with SIGKILL at one point of a store write. The resumability
tests use it to interrupt a stage for real.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import signal
import sys
from collections.abc import Callable, Generator
from contextlib import contextmanager
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from typer.testing import CliRunner

from tests.fixture import build_fixture, key
from tests.test_chapter_extraction import _entry, _write_book  # pyright: ignore[reportPrivateUsage]
from urg import cli, store
from urg.model import ModelConfig, ModelRequest, ModelResponse
from urg.store import ArtifactStore
from urg.zotero_db import sha256_file

Json = dict[str, Any]
REPO = Path(__file__).resolve().parent.parent

#: The two fixture books that get a chapter, as (item key, attachment key, storage file).
BOOKS = ((key(10), key(11), "sicp.pdf"), (key(90), key(91), "broken.pdf"))
A, B = (book[0] for book in BOOKS)

ELEMENT = re.compile(r"^\[(?P<id>[^\]]+)\] (?P<kind>[a-z_]+), page ", re.MULTILINE)
INTERPRETATION = re.compile(r'^\[(?P<id>[^\]]+)\] (?P<kind>[a-z-]+) "', re.MULTILINE)


# --- Model boundary -------------------------------------------------------------------


@dataclass
class ScriptedModel:
    """A model boundary that derives each answer from the ids in the request.

    An analysis request gets one episode, one move and one observation over the first
    elements of the chapter. A discovery request gets one pattern over the episodes and one
    over the moves, each supported by every book. ``calls`` counts the calls by stage.
    """

    calls: list[str] = field(default_factory=list[str])
    config: Json = field(default_factory=dict[str, Any])

    def describe(self) -> Json:
        return {"harness": "scripted", **self.config}

    def call(self, request: ModelRequest) -> ModelResponse:
        if "\nElements:\n" in request.prompt:
            self.calls.append("episode-analysis")
            text = analysis_answer(request.prompt)
        else:
            self.calls.append("pattern-discovery")
            text = discovery_answer(request.prompt)
        raw = json.dumps({"type": "result", "is_error": False, "result": text}).encode()
        return ModelResponse(raw=raw, text=text, metadata={"scripted": True})


def analysis_answer(prompt: str) -> str:
    ids = [m["id"] for m in ELEMENT.finditer(prompt)]
    first, second, third = ids[0], ids[1], ids[2]
    uncertainty = {"level": "medium", "reason": "A fixture answer."}
    return json.dumps(
        {
            "interpretations": [
                {
                    "local_id": "ep1",
                    "kind": "episode",
                    "label": "opening",
                    "description": "The chapter opens with its topic.",
                    "evidence": [first, second, third],
                    "uncertainty": uncertainty,
                },
                {
                    "local_id": "mv1",
                    "kind": "move",
                    "episode": "ep1",
                    "label": "names the topic",
                    "description": "The heading names the topic.",
                    "evidence": [first],
                    "uncertainty": uncertainty,
                },
                {
                    "local_id": "obs1",
                    "kind": "observation",
                    "label": None,
                    "description": "The text follows the heading.",
                    "evidence": [second],
                    "uncertainty": uncertainty,
                },
            ]
        }
    )


def discovery_answer(prompt: str) -> str:
    by_kind: dict[str, list[str]] = {}
    for match in INTERPRETATION.finditer(prompt):
        by_kind.setdefault(match["kind"], []).append(match["id"])
    uncertainty = {"level": "high", "reason": "A fixture answer."}

    def pattern(local_id: str, kind: str) -> Json:
        return {
            "local_id": local_id,
            "name": f"Every chapter has its {kind}",
            "description": f"Each book shows the {kind}.",
            "supporting": [{"observation": i, "note": "Fixture."} for i in by_kind[kind]],
            "uncertainty": uncertainty,
        }

    return json.dumps({"patterns": [pattern("p1", "episode"), pattern("p2", "move")]})


# --- Fixture pipeline -----------------------------------------------------------------


def write_weights(path: Path, weight_of_a: float) -> Path:
    path.write_text(
        json.dumps(
            {
                "version": 1,
                "status": "illustrative",
                "statement": "Weights for the resumability fixture.",
                "books": {A: {"weight": weight_of_a, "reason": "fixture"}},
                "sections": {},
            }
        )
    )
    return path


def write_relevance(path: Path) -> Path:
    path.write_text(
        json.dumps(
            {
                "version": 1,
                "status": "illustrative",
                "statement": "Relevance for the resumability fixture.",
                "books": {},
            }
        )
    )
    return path


@dataclass
class Pipeline:
    """The fixture inputs of every stage, and the stage commands over them."""

    root: Path
    calls: list[str] = field(default_factory=list[str])

    def __post_init__(self) -> None:
        library = build_fixture(self.root / "library")
        self.rdf = library.rdf
        self.zotero_dir = library.zotero_dir
        for _, attachment, name in BOOKS:
            _write_book(self.zotero_dir / "storage" / attachment / name)
        self.data = self.root / "data"
        self.store = ArtifactStore(self.data)
        self.selection = self.root / "chapter-selection.json"
        self.analysis_instructions = self.copy("docs/analysis/instructions.md")
        self.discovery_instructions = self.copy("docs/discovery/instructions.md")
        self.settings = self.copy("docs/discovery/settings.json")
        self.model_config = self.copy("docs/analysis/model.json")
        self.corrections = self.root / "corrections"
        self.corrections.mkdir()
        self.weights = write_weights(self.root / "weights.json", 2.0)
        self.relevance = write_relevance(self.root / "relevance.json")

    def copy(self, relative: str) -> Path:
        target = self.root / "inputs" / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(REPO / relative, target)
        return target

    def write_selection(self) -> None:
        """Select chapter 2 of both books from the newest source manifest."""
        manifest = self.store.latest_key("source-manifest")
        assert manifest is not None
        selected: list[Json] = []
        for item_key, attachment, name in BOOKS:
            sha = sha256_file(self.zotero_dir / "storage" / attachment / name)
            selected.append(_entry(item_key, attachment, sha))
        selection = {
            "schema_version": 1,
            "stage": "extraction-probe",
            "source_manifest_sha256": manifest,
            "page_index_base": 0,
            "selected": selected,
        }
        self.selection.write_text(json.dumps(selection))

    def commands(self) -> list[list[str]]:
        """Return each stage command in the order of the pipeline."""
        data = ["--data-dir", str(self.data)]
        commands = [
            ["catalogue-manifest", "--rdf", str(self.rdf), "--zotero-dir", str(self.zotero_dir)],
            ["chapter-extraction", "--selection", str(self.selection)],
            [
                "episode-analysis",
                "--instructions", str(self.analysis_instructions),
                "--model-config", str(self.model_config),
            ],
            ["corrections", "--corrections-dir", str(self.corrections)],
            [
                "pattern-discovery",
                "--settings", str(self.settings),
                "--instructions", str(self.discovery_instructions),
                "--model-config", str(self.model_config),
            ],
            ["weighted-report", "--weights", str(self.weights), "--relevance", str(self.relevance)],
        ]  # fmt: skip
        return [command + data for command in commands]

    def stage(self, command: list[str]) -> str:
        """Run one stage command in this process with the scripted model."""
        original = cli.model_from_config
        cli.model_from_config = self._model
        try:
            result = CliRunner().invoke(cli.app, command)
        finally:
            cli.model_from_config = original
        assert result.exit_code == 0, f"{command[0]} failed:\n{result.output}"
        if command[0] == "catalogue-manifest":
            self.write_selection()
        return result.output

    def _model(self, config: ModelConfig) -> ScriptedModel:
        # The configuration is part of the identity, so a change to it is a change of model.
        return ScriptedModel(self.calls, config.model_dump(mode="json"))

    def run(self) -> dict[str, str]:
        """Run every stage command in order and return the output of each."""
        return {command[0]: self.stage(command) for command in self.commands()}

    def artifacts(self) -> dict[tuple[str, str], str]:
        """Map each complete artifact in the store to the time its record was written."""
        found: dict[tuple[str, str], str] = {}
        for directory in sorted((self.data / "artifacts").glob("*/*")):
            if directory.is_dir() and (directory / store.RECORD_FILE).is_file():
                record = json.loads((directory / store.RECORD_FILE).read_bytes())
                found[(directory.parent.name, directory.name)] = record["written_at"]
        return found


# --- Killing a stage ------------------------------------------------------------------

#: Where the child process kills itself during a store write.
#: ``mid-payload``: after half of the payload file TARGET is on disk.
#: ``before-rename``: after the staged artifact TARGET is complete, before it is renamed.
#: ``before-latest``: after the artifact TARGET is in place, before its latest pointer moves.
KILL_POINTS = ("mid-payload", "before-rename", "before-latest")


def _die() -> None:
    os.kill(os.getpid(), signal.SIGKILL)


@contextmanager
def kill_at(point: str, target: str) -> Generator[None]:
    """Patch the store so that this process kills itself at POINT of writing TARGET.

    TARGET is a payload file name for ``mid-payload`` and an artifact name otherwise. A
    suffix ``@N`` kills at the Nth match, so that the earlier matches complete.
    """
    target, _, nth = target.partition("@")
    remaining = [int(nth or "1")]

    def matches(name: str) -> bool:
        if name != target:
            return False
        remaining[0] -= 1
        return remaining[0] == 0

    write_durable: Callable[[Path, bytes], None] = store._write_durable  # pyright: ignore[reportPrivateUsage]
    rename = os.rename
    set_latest = store.ArtifactStore._set_latest  # pyright: ignore[reportPrivateUsage]

    def partial(path: Path, data: bytes) -> None:
        staged = path.parent.parent.name == "staging"
        if point == "mid-payload" and staged and matches(path.name):
            with path.open("wb") as handle:
                handle.write(data[: len(data) // 2])
                handle.flush()
                os.fsync(handle.fileno())
            _die()
        write_durable(path, data)

    def renaming(source: Any, destination: Any) -> None:
        if point == "before-rename" and matches(Path(destination).parent.name):
            _die()
        rename(source, destination)

    def pointing(self: ArtifactStore, name: str, artifact_key: str) -> None:
        if point == "before-latest" and matches(name):
            _die()
        set_latest(self, name, artifact_key)

    store._write_durable = partial  # pyright: ignore[reportPrivateUsage]
    store.os.rename = renaming  # pyright: ignore[reportAttributeAccessIssue]
    store.ArtifactStore._set_latest = pointing  # pyright: ignore[reportPrivateUsage, reportAttributeAccessIssue]
    try:
        yield
    finally:
        store._write_durable = write_durable  # pyright: ignore[reportPrivateUsage]
        store.os.rename = rename  # pyright: ignore[reportAttributeAccessIssue]
        store.ArtifactStore._set_latest = set_latest  # pyright: ignore[reportPrivateUsage]


def main(argv: list[str]) -> None:
    """Run ``kill POINT TARGET -- ARGS``: one stage command that dies at POINT."""
    if len(argv) < 4 or argv[0] != "kill" or argv[1] not in KILL_POINTS or argv[3] != "--":
        raise SystemExit("usage: python -m tests.pipeline kill POINT TARGET -- ARGS")
    calls: list[str] = []

    def scripted(config: ModelConfig) -> ScriptedModel:
        return ScriptedModel(calls, config.model_dump(mode="json"))

    cli.model_from_config = scripted  # pyright: ignore[reportAttributeAccessIssue]
    with kill_at(argv[1], argv[2]):
        cli.app(argv[4:])


if __name__ == "__main__":
    main(sys.argv[1:])
