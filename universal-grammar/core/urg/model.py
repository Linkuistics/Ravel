"""The model boundary.

Every model call of URG passes through one ``ModelBoundary``. A stage builds a
``ModelRequest`` and receives a ``ModelResponse`` that keeps the raw output of the model
harness. Tests replace the real harness with ``RecordedModel``, which replays stored
responses and makes no call.

``ClaudeCliModel`` runs the Claude Code CLI in print mode. The binding follows the
Writegood sibling project (``writegood/harness/claude.py`` and ``environ.py``):

- ``--tools ""`` gives the model no tools, so it answers from the prompt alone.
- ``--safe-mode`` disables CLAUDE.md files, skills, plugins, hooks and MCP servers.
- ``--no-session-persistence`` keeps the call out of the session history.
- The child environment is built from an allowlist. Variables that redirect the
  credential path to metered access, such as ``ANTHROPIC_API_KEY``, are removed.
- The working directory is ``/``, so no project file reaches the model.

The flags were checked against ``claude --help`` of Claude Code 2.1.322. The JSON result
fields (``result``, ``is_error``, ``usage``, ``modelUsage``) follow the CLI reference at
https://code.claude.com/docs/en/cli-reference and the Writegood parser, which was
established by running the CLI. ``is_error`` is the failure signal: on an authentication
failure the ``subtype`` field can still say ``success``.

A dependency record names the model configuration. It cannot guarantee identical output
on a rerun, because the model is not deterministic and the provider can change it.
"""

from __future__ import annotations

import json
import os
import subprocess
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field, replace
from pathlib import Path
from typing import Any, Literal, Protocol, cast

from pydantic import BaseModel, ConfigDict, Field, ValidationError

from urg.errors import UrgError

Json = dict[str, Any]

#: Names copied from the parent environment. ``LC_*`` is a prefix match.
BASE_ENV = ("PATH", "HOME", "USER", "LOGNAME", "TMPDIR", "LANG", "LC_*", "CLAUDE_CONFIG_DIR")
#: Names that would move the call from the subscription to another credential path.
DENIED_ENV = (
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_AUTH_TOKEN",
    "ANTHROPIC_BASE_URL",
    "CLAUDE_CODE_USE_BEDROCK",
    "CLAUDE_CODE_USE_VERTEX",
    "CLAUDE_CODE_USE_FOUNDRY",
)
PINNED_ENV = {"TZ": "UTC"}
NEUTRAL_CWD = "/"
EXHAUSTION_MARKERS = ("usage limit", "rate limit", "quota", "credit", "subscription limit")


@dataclass(frozen=True)
class FailedOutput:
    """What a failed model call produced before the boundary rejected it.

    ``stdout`` and ``stderr`` are kept byte for byte. A timeout keeps the partial output
    that the harness wrote before the kill. ``metadata`` names the failure, such as the exit
    code, the timeout or the providers that answered.
    """

    stdout: bytes
    stderr: bytes
    metadata: Json = field(default_factory=dict[str, Any])


class ModelError(UrgError):
    """A model call failed, or the model configuration is invalid.

    ``output`` holds what the harness produced when the call itself ran. It is ``None`` when
    no call ran, for example when the configuration is invalid or the executable is missing.
    """

    def __init__(self, message: str, output: FailedOutput | None = None) -> None:
        super().__init__(message)
        self.output = output


class ModelConfig(BaseModel):
    """The model configuration file of a stage."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    harness: Literal["claude-cli"]
    model: str = Field(min_length=1)
    effort: Literal["low", "medium", "high", "xhigh", "max"] | None = None
    timeout_seconds: int = Field(default=1800, ge=1)


def load_model_config(path: Path) -> ModelConfig:
    """Read a JSON model configuration."""
    try:
        return ModelConfig.model_validate_json(path.read_bytes())
    except (OSError, ValidationError) as error:
        raise ModelError(f"cannot read the model configuration {path}: {error}") from error


@dataclass(frozen=True)
class ModelRequest:
    """One model call: the system instructions and the user prompt."""

    system: str
    prompt: str

    def as_json(self) -> Json:
        return {"system": self.system, "prompt": self.prompt}


@dataclass(frozen=True)
class ModelResponse:
    """The answer of one model call.

    ``raw`` is the complete output of the harness, kept byte for byte. ``text`` is the answer
    of the model, taken from ``raw``. ``metadata`` holds the harness report, such as the
    model that answered and the token usage.
    """

    raw: bytes
    text: str
    metadata: Json = field(default_factory=dict[str, Any])


class ModelBoundary(Protocol):
    """The one place where URG calls a model."""

    def describe(self) -> Json:
        """Return the identity of the model and harness for the dependency record."""
        ...

    def call(self, request: ModelRequest) -> ModelResponse:
        """Send one request and return the response, or raise ``ModelError``."""
        ...


def child_environment(parent: Mapping[str, str]) -> dict[str, str]:
    """Build the child environment from the allowlist, then apply the deny list and the pins."""
    exact = {name for name in BASE_ENV if not name.endswith("*")}
    prefixes = tuple(name[:-1] for name in BASE_ENV if name.endswith("*"))
    env = {k: v for k, v in parent.items() if k in exact or k.startswith(prefixes)}
    for name in DENIED_ENV:
        env.pop(name, None)
    env.update(PINNED_ENV)
    return env


def parse_claude_result(stdout: bytes, stderr: bytes, returncode: int) -> ModelResponse:
    """Turn the JSON result of ``claude -p --output-format json`` into a response."""
    detail = stderr.decode(errors="replace").strip()

    def failed(reason: str) -> FailedOutput:
        return FailedOutput(stdout, stderr, {"reason": reason, "returncode": returncode})

    try:
        parsed: object = json.loads(stdout)
    except ValueError:
        raise ModelError(
            f"claude -p produced no JSON (exit {returncode}): {detail or stdout[:500]!r}",
            failed("no-json"),
        ) from None
    if not isinstance(parsed, dict):
        raise ModelError(
            f"claude -p produced JSON that is not an object (exit {returncode})",
            failed("not-an-object"),
        )
    result = cast(Json, parsed)
    text = result.get("result")
    if result.get("is_error") or returncode != 0 or not isinstance(text, str):
        message = str(text or detail or result.get("subtype") or "no detail")
        if any(marker in message.lower() for marker in EXHAUSTION_MARKERS):
            message = f"usage exhausted: {message}"
        raise ModelError(
            f"claude -p failed (exit {returncode}): {message[:2000]}", failed("harness-error")
        )
    raw_usage: object = result.get("modelUsage")
    model_usage = cast(Json, raw_usage) if isinstance(raw_usage, dict) else {}
    providers: set[str] = set()
    for entry in model_usage.values():
        if isinstance(entry, dict):
            providers.add(str(cast(Json, entry).get("provider")))
    metadata: Json = {
        "models": sorted(model_usage),
        "providers": sorted(providers),
        "usage": result.get("usage"),
        "num_turns": result.get("num_turns"),
        "duration_ms": result.get("duration_ms"),
        "total_cost_usd": result.get("total_cost_usd"),
    }
    return ModelResponse(raw=stdout, text=text, metadata=metadata)


@dataclass(frozen=True)
class ClaudeCliModel:
    """A model boundary that runs the Claude Code CLI in print mode."""

    config: ModelConfig
    executable: str = "claude"

    def argv(self, request: ModelRequest) -> list[str]:
        argv = [
            self.executable,
            "-p",
            "--output-format",
            "json",
            "--tools",
            "",
            "--safe-mode",
            "--no-session-persistence",
            "--model",
            self.config.model,
            "--system-prompt",
            request.system,
        ]
        if self.config.effort is not None:
            argv += ["--effort", self.config.effort]
        return argv

    def describe(self) -> Json:
        # The harness version is left out of the identity, so that an update of the CLI does
        # not force new model calls. Each response records the version in its metadata.
        return {
            "harness": self.config.harness,
            "model": self.config.model,
            "effort": self.config.effort,
            "flags": ["--tools", "", "--safe-mode", "--no-session-persistence"],
        }

    def _harness_version(self) -> str:
        try:
            completed = subprocess.run(
                [self.executable, "--version"],
                capture_output=True,
                stdin=subprocess.DEVNULL,
                env=child_environment(os.environ),
                cwd=NEUTRAL_CWD,
                timeout=60,
                check=False,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            raise ModelError(f"cannot run {self.executable} --version: {error}") from error
        if completed.returncode != 0:
            raise ModelError(f"{self.executable} --version failed: {completed.stderr!r}")
        return completed.stdout.decode().strip()

    def call(self, request: ModelRequest) -> ModelResponse:
        harness_version = self._harness_version()
        try:
            completed = subprocess.run(
                self.argv(request),
                input=request.prompt.encode(),
                capture_output=True,
                env=child_environment(os.environ),
                cwd=NEUTRAL_CWD,
                timeout=self.config.timeout_seconds,
                check=False,
            )
        except subprocess.TimeoutExpired as error:
            # ``run`` kills the child and attaches the output that it wrote before the kill.
            partial = FailedOutput(
                _as_bytes(error.stdout),
                _as_bytes(error.stderr),
                {"reason": "timeout", "timeout_seconds": error.timeout},
            )
            raise ModelError(
                f"claude -p timed out after {error.timeout} seconds",
                _with_harness_version(partial, harness_version),
            ) from error
        except OSError as error:
            raise ModelError(f"cannot run {self.executable}: {error}") from error
        try:
            response = parse_claude_result(completed.stdout, completed.stderr, completed.returncode)
        except ModelError as error:
            if error.output is None:
                raise
            raise ModelError(
                str(error), _with_harness_version(error.output, harness_version)
            ) from error
        providers = response.metadata["providers"]
        if providers and providers != ["firstParty"]:
            wrong = FailedOutput(
                completed.stdout,
                completed.stderr,
                {**response.metadata, "reason": "provider", "returncode": completed.returncode},
            )
            raise ModelError(
                f"claude -p answered through {providers}, not the subscription",
                _with_harness_version(wrong, harness_version),
            )
        response.metadata["harness_version"] = harness_version
        return response


def _as_bytes(output: bytes | str | None) -> bytes:
    if output is None:
        return b""
    return output.encode() if isinstance(output, str) else output


def _with_harness_version(output: FailedOutput, harness_version: str) -> FailedOutput:
    return replace(output, metadata={**output.metadata, "harness_version": harness_version})


@dataclass
class RecordedModel:
    """A model boundary that replays recorded answers. It makes no model call.

    Each call takes the next answer. ``requests`` keeps every request for inspection.
    """

    answers: Sequence[str]
    identity: Json = field(default_factory=lambda: {"harness": "recorded", "model": "recorded"})
    requests: list[ModelRequest] = field(default_factory=list[ModelRequest])

    def describe(self) -> Json:
        return dict(self.identity)

    def call(self, request: ModelRequest) -> ModelResponse:
        if len(self.requests) >= len(self.answers):
            raise ModelError("the recorded model has no answer left")
        text = self.answers[len(self.requests)]
        self.requests.append(request)
        raw = json.dumps({"type": "result", "is_error": False, "result": text}).encode()
        return ModelResponse(raw=raw, text=text, metadata={"recorded": True})


def model_from_config(config: ModelConfig) -> ModelBoundary:
    """Return the model boundary that a configuration names."""
    match config.harness:
        case "claude-cli":
            return ClaudeCliModel(config)
