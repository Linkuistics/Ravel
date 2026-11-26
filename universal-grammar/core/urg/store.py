"""The research-artifact store.

Every stage writes its research artifacts through this store. Each research artifact has exactly
one dependency record. The key of the research artifact is the SHA-256 of that record in
canonical JSON, so a rerun with the same inputs, code, configuration and schema finds the same
key. Reuse compares dependency records and the recorded file digests, never file existence.

Layout under the store root:

    artifacts/<name>/<key>/record.json   the dependency record and the payload digests
    artifacts/<name>/<key>/<payload>     the payload files
    artifacts/<name>/latest              the key of the newest research artifact of that name
    staging/<random>/                    writes in progress

A write fills a staging directory, writes ``record.json`` last and then renames the directory
into place. An interrupted write therefore never appears under ``artifacts/``. A reader checks
the record and every payload digest, so a damaged research artifact fails to load.
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import uuid
from datetime import UTC, datetime
from pathlib import Path
from typing import Annotated, Any, Literal

from pydantic import BaseModel, ConfigDict, Field, StringConstraints, ValidationError

from urg.errors import UrgError

NonEmpty = Annotated[str, StringConstraints(min_length=1, strip_whitespace=True)]
Sha256 = Annotated[str, StringConstraints(pattern=r"^[0-9a-f]{64}$")]
JsonValue = Any

RECORD_FILE = "record.json"


class StoreError(UrgError):
    """The store refused a write or a read."""


class IncompleteDependencyRecord(StoreError):
    """A dependency record lacks a required field."""


class InterruptedWrite(StoreError):
    """A research artifact is partial or damaged."""


class InputIdentity(BaseModel):
    """The identity of one input behind a research artifact.

    ``kind`` says how ``identity`` was computed:

    - ``file``: ``identity`` is the SHA-256 of the file content.
    - ``file-set``: ``identity`` is the SHA-256 of the sorted list of relative path, size and
      modification time of each member. A content change that keeps size and time escapes it.
    - ``artifact``: ``identity`` is the key of another research artifact in the store.
    """

    model_config = ConfigDict(extra="forbid", frozen=True)

    role: NonEmpty
    kind: Literal["file", "file-set", "artifact"]
    locator: NonEmpty
    identity: Sha256


class DependencyRecord(BaseModel):
    """The identities and versions of every input behind one research artifact."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    stage: NonEmpty
    artifact: NonEmpty
    schema_version: Annotated[int, Field(ge=1)]
    code_version: NonEmpty
    configuration: dict[str, JsonValue]
    inputs: Annotated[list[InputIdentity], Field(min_length=1)]

    def key(self) -> str:
        """Return the SHA-256 of the record in canonical JSON."""
        return hashlib.sha256(canonical_json(self.model_dump(mode="json"))).hexdigest()


class PayloadDigest(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)

    sha256: Sha256
    size: Annotated[int, Field(ge=0)]


class ArtifactRecord(BaseModel):
    """The content of ``record.json``."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    key: Sha256
    dependency: DependencyRecord
    payload: Annotated[dict[NonEmpty, PayloadDigest], Field(min_length=1)]
    written_at: datetime


class StoredArtifact(BaseModel):
    """A research artifact that passed every check on read."""

    model_config = ConfigDict(frozen=True)

    record: ArtifactRecord
    directory: Path

    def path(self, payload_name: str) -> Path:
        if payload_name not in self.record.payload:
            raise StoreError(f"research artifact has no payload file {payload_name!r}")
        return self.directory / payload_name

    def read_bytes(self, payload_name: str) -> bytes:
        return self.path(payload_name).read_bytes()


def canonical_json(value: JsonValue) -> bytes:
    """Return JSON with sorted keys and no insignificant whitespace."""
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def validate_dependency(dependency: DependencyRecord | dict[str, JsonValue]) -> DependencyRecord:
    """Return a validated dependency record, or raise ``IncompleteDependencyRecord``."""
    raw = (
        dependency.model_dump(mode="json")
        if isinstance(dependency, DependencyRecord)
        else dependency
    )
    try:
        return DependencyRecord.model_validate(raw)
    except ValidationError as e:
        raise IncompleteDependencyRecord(f"incomplete dependency record: {e}") from e


def _fsync_dir(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def _write_durable(path: Path, data: bytes) -> None:
    with path.open("wb") as handle:
        handle.write(data)
        handle.flush()
        os.fsync(handle.fileno())


class ArtifactStore:
    """A directory of research artifacts keyed by their dependency records."""

    def __init__(self, root: Path) -> None:
        self.root = root

    @property
    def _artifacts(self) -> Path:
        return self.root / "artifacts"

    @property
    def _staging(self) -> Path:
        return self.root / "staging"

    def directory_for(self, name: str, key: str) -> Path:
        return self._artifacts / name / key

    def write(
        self,
        dependency: DependencyRecord | dict[str, JsonValue],
        payload: dict[str, bytes],
    ) -> StoredArtifact:
        """Write a research artifact atomically and return it as read back."""
        record_dependency = validate_dependency(dependency)
        if not payload:
            raise StoreError("a research artifact needs at least one payload file")
        for payload_name in payload:
            if payload_name == RECORD_FILE or "/" in payload_name or payload_name.startswith("."):
                raise StoreError(f"payload file name {payload_name!r} is not allowed")

        key = record_dependency.key()
        name = record_dependency.artifact
        final = self.directory_for(name, key)
        if final.exists():
            try:
                existing = self.load(name, key)
            except InterruptedWrite:
                self.replace_damaged(record_dependency)
            else:
                if existing.record.dependency != record_dependency:
                    raise StoreError(f"key collision for {name}/{key}")
                self._set_latest(name, key)
                return existing

        self._staging.mkdir(parents=True, exist_ok=True)
        staging = self._staging / uuid.uuid4().hex
        staging.mkdir()
        try:
            digests: dict[str, PayloadDigest] = {}
            for payload_name, data in payload.items():
                _write_durable(staging / payload_name, data)
                digests[payload_name] = PayloadDigest(
                    sha256=hashlib.sha256(data).hexdigest(), size=len(data)
                )
            record = ArtifactRecord(
                key=key,
                dependency=record_dependency,
                payload=digests,
                written_at=datetime.now(UTC),
            )
            _write_durable(staging / RECORD_FILE, canonical_json(record.model_dump(mode="json")))
            _fsync_dir(staging)
            final.parent.mkdir(parents=True, exist_ok=True)
            os.rename(staging, final)
            _fsync_dir(final.parent)
        except BaseException:
            shutil.rmtree(staging, ignore_errors=True)
            raise
        self._set_latest(name, key)
        return self.load(name, key)

    def _set_latest(self, name: str, key: str) -> None:
        pointer = self._artifacts / name / "latest"
        temporary = pointer.with_name(f".latest.{uuid.uuid4().hex}")
        _write_durable(temporary, key.encode())
        os.replace(temporary, pointer)

    def latest_key(self, name: str) -> str | None:
        pointer = self._artifacts / name / "latest"
        if not pointer.is_file():
            return None
        return pointer.read_text().strip()

    def load(self, name: str, key: str) -> StoredArtifact:
        """Read a research artifact and check its record and every payload digest."""
        directory = self.directory_for(name, key)
        record_path = directory / RECORD_FILE
        if not record_path.is_file():
            raise InterruptedWrite(f"{name}/{key} has no {RECORD_FILE}")
        try:
            record = ArtifactRecord.model_validate_json(record_path.read_bytes())
        except ValidationError as e:
            raise InterruptedWrite(f"{name}/{key} has an invalid {RECORD_FILE}: {e}") from e
        if record.key != key or record.dependency.key() != key:
            raise InterruptedWrite(f"{name}/{key} records a different key")
        if record.dependency.artifact != name:
            raise InterruptedWrite(f"{name}/{key} records artifact {record.dependency.artifact}")
        for payload_name, digest in record.payload.items():
            payload_path = directory / payload_name
            if not payload_path.is_file():
                raise InterruptedWrite(f"{name}/{key} lacks payload file {payload_name}")
            data = payload_path.read_bytes()
            if len(data) != digest.size or hashlib.sha256(data).hexdigest() != digest.sha256:
                raise InterruptedWrite(f"{name}/{key} payload file {payload_name} is damaged")
        return StoredArtifact(record=record, directory=directory)

    def find(self, dependency: DependencyRecord) -> StoredArtifact | None:
        """Return the stored research artifact for this dependency record, if it is intact.

        A damaged research artifact is not reused. The caller then writes it again.
        """
        record_dependency = validate_dependency(dependency)
        directory = self.directory_for(record_dependency.artifact, record_dependency.key())
        if not directory.exists():
            return None
        try:
            stored = self.load(record_dependency.artifact, record_dependency.key())
        except InterruptedWrite:
            return None
        if stored.record.dependency != record_dependency:
            return None
        return stored

    def reuse(self, dependency: DependencyRecord) -> StoredArtifact | None:
        """Return the intact artifact for this dependency record, and make it the latest.

        A stage that reuses an artifact calls this, not ``find``. A later stage reads its
        upstream run through the ``latest`` pointer. A reused artifact must therefore move the
        pointer, as a new artifact does. Otherwise a kill between the rename and the pointer
        update, or a return to earlier inputs, leaves the pointer on another artifact.
        """
        found = self.find(dependency)
        if found is not None:
            self._set_latest(found.record.dependency.artifact, found.record.key)
        return found

    def replace_damaged(self, dependency: DependencyRecord) -> None:
        """Move a damaged research artifact aside so that a new write can take its place."""
        directory = self.directory_for(dependency.artifact, dependency.key())
        if directory.exists():
            aside = self._staging / f"damaged-{uuid.uuid4().hex}"
            self._staging.mkdir(parents=True, exist_ok=True)
            os.rename(directory, aside)
