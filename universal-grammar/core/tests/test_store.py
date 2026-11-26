"""Tests of the research-artifact store."""

from __future__ import annotations

from pathlib import Path

import pytest

from urg.store import (
    ArtifactStore,
    DependencyRecord,
    IncompleteDependencyRecord,
    InputIdentity,
    InterruptedWrite,
    StoreError,
    validate_dependency,
)

SHA = "0" * 64


def _dependency(configuration: dict[str, int] | None = None) -> DependencyRecord:
    return DependencyRecord(
        stage="test",
        artifact="example",
        schema_version=1,
        code_version="urg test",
        configuration=dict(configuration or {}),
        inputs=[InputIdentity(role="source", kind="file", locator="in.txt", identity=SHA)],
    )


def test_write_then_find_returns_the_artifact(tmp_path: Path) -> None:
    store = ArtifactStore(tmp_path)
    written = store.write(_dependency(), {"out.txt": b"hello"})
    found = store.find(_dependency())
    assert found is not None
    assert found.directory == written.directory
    assert found.read_bytes("out.txt") == b"hello"
    assert store.latest_key("example") == written.record.key


def test_another_configuration_is_another_artifact(tmp_path: Path) -> None:
    store = ArtifactStore(tmp_path)
    store.write(_dependency(), {"out.txt": b"a"})
    assert store.find(_dependency({"n": 2})) is None


@pytest.mark.parametrize(
    "record",
    [
        {"stage": "test"},
        {
            "stage": "test",
            "artifact": "example",
            "schema_version": 1,
            "code_version": "urg test",
            "configuration": {},
            "inputs": [],
        },
        {
            "stage": "test",
            "artifact": "example",
            "schema_version": 1,
            "code_version": "urg test",
            "configuration": {},
            "inputs": [{"role": "source", "kind": "file", "locator": "in.txt", "identity": "x"}],
        },
    ],
)
def test_an_incomplete_dependency_record_is_rejected(record: dict[str, object]) -> None:
    with pytest.raises(IncompleteDependencyRecord):
        validate_dependency(record)  # pyright: ignore[reportArgumentType]


def test_a_missing_record_file_is_an_interrupted_write(tmp_path: Path) -> None:
    store = ArtifactStore(tmp_path)
    written = store.write(_dependency(), {"out.txt": b"hello"})
    (written.directory / "record.json").unlink()
    with pytest.raises(InterruptedWrite):
        store.load("example", written.record.key)
    assert store.find(_dependency()) is None


def test_a_damaged_payload_is_an_interrupted_write(tmp_path: Path) -> None:
    store = ArtifactStore(tmp_path)
    written = store.write(_dependency(), {"out.txt": b"hello"})
    written.path("out.txt").write_bytes(b"jello")
    with pytest.raises(InterruptedWrite):
        store.load("example", written.record.key)


def test_writing_again_replaces_a_damaged_artifact(tmp_path: Path) -> None:
    store = ArtifactStore(tmp_path)
    written = store.write(_dependency(), {"out.txt": b"hello"})
    (written.directory / "record.json").unlink()
    rewritten = store.write(_dependency(), {"out.txt": b"hello"})
    assert rewritten.read_bytes("out.txt") == b"hello"
    assert store.find(_dependency()) is not None


@pytest.mark.parametrize("name", ["record.json", "a/b", ".hidden"])
def test_reserved_payload_names_are_rejected(tmp_path: Path, name: str) -> None:
    with pytest.raises(StoreError):
        ArtifactStore(tmp_path).write(_dependency(), {name: b"x"})


def test_an_empty_payload_is_rejected(tmp_path: Path) -> None:
    with pytest.raises(StoreError):
        ArtifactStore(tmp_path).write(_dependency(), {})
