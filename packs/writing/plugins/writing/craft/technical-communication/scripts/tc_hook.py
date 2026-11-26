#!/usr/bin/env python3
"""The hook core of the technical-communication skill.

Run as `tc-hook HARNESS EVENT`, with the harness's payload on standard input.
The events are session-start, subagent-start, prompt, write and reply, and
instruction, which only Pi uses. The core does the work and an adapter for each harness reads that harness's payload and
writes that harness's output, so a harness entry stays a thin, stable file.

The core checks the off file first. Every path ends in exit 0, and the core
prints at most one JSON object, and only when it adds context. A reply event
prints nothing, because any output from a stop hook continues the turn.
"""

import importlib.util
import json
import os
import re
import sys

# The hooks run in place from an installed source, such as a Homebrew keg, and
# a file written there changes the source's digest. The import of ste-lint.py
# below would otherwise write its bytecode into scripts/__pycache__. tc-hook
# passes -B too; this setting covers a run without the guard.
sys.dont_write_bytecode = True

SCRIPTS = os.path.dirname(os.path.realpath(__file__))
SKILL_DIR = os.path.dirname(SCRIPTS)
ALWAYS_ON = os.path.join(SKILL_DIR, "always-on")

GATE_MESSAGE_LIMIT = 1500
GATE_FINDING_LIMIT = 10
GATE_FILE_LIMIT = 1_000_000  # bytes; a larger file is not worth the delay
TRANSCRIPT_TAIL = 1_000_000  # bytes of a transcript read to find the model

PROSE_SUFFIXES = (".md", ".markdown", ".txt")
TEMP_ROOTS = ("/tmp", "/private/tmp", "/var/folders", "/private/var/folders")
SKIPPED_DIRECTORIES = {".git", ".jj", "LICENSES", "vendor", "third_party",
                       "node_modules"}
LICENCE_PREFIXES = ("LICENSE", "LICENCE", "COPYING", "NOTICE")
HISTORIC_TASK = re.compile(r"^\d+-(DONE|ABANDONED)-")
GENERATED = ("@generated", "DO NOT EDIT")


def load_checker():
    """Load ste-lint.py by path, so the core runs the checker's own functions."""
    spec = importlib.util.spec_from_file_location(
        "ste_lint", os.path.join(SCRIPTS, "ste-lint.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# --- the adapter for each harness ---------------------------------------------

class Adapter:
    """What the harnesses agree on: session_id, agent_id, source, cwd, the
    last_assistant_message of a stop event, and an additionalContext nested
    under hookSpecificOutput. An adapter overrides what differs.

    Each harness names its own payload fields in its subclass docstring.
    """

    name = ""
    EVENT_NAMES = {"session-start": "SessionStart",
                   "subagent-start": "SubagentStart",
                   "prompt": "UserPromptSubmit",
                   "write": "PostToolUse"}

    def session(self, payload):
        return payload.get("session_id")

    def agent(self, payload):
        # present only inside a subagent
        return payload.get("agent_id") or None

    def source(self, event, payload):
        if event == "subagent-start":
            return "subagent"
        return payload.get("source")

    def model(self, payload):
        return None

    def reply(self, payload):
        text = payload.get("last_assistant_message")
        return text if isinstance(text, str) else ""

    def cwd(self, payload):
        return payload.get("cwd") or os.getcwd()

    def written_paths(self, payload):
        return []

    def output(self, event, context):
        return {"hookSpecificOutput": {"hookEventName": self.EVENT_NAMES[event],
                                       "additionalContext": context}}


class ClaudeCode(Adapter):
    """Claude Code's payloads and output.

    The payload fields are those of Claude Code 2.1.302
    (docs/research/harness-hooks-a.md, section 1). A top-level
    additionalContext is dropped by Claude Code, so the output always nests it
    under hookSpecificOutput and names the event.
    """

    name = "claude-code"

    def model(self, payload):
        """The model of the latest assistant turn in the transcript, or None.

        No hook payload names the model in 2.1.302, but each assistant entry of
        the transcript does. A subagent has a transcript of its own. A tail of
        the file is enough, and a failure to read it is not an error.
        """
        path = payload.get("agent_transcript_path") or payload.get("transcript_path")
        try:
            with open(path, "rb") as fh:
                fh.seek(0, os.SEEK_END)
                fh.seek(max(0, fh.tell() - TRANSCRIPT_TAIL))
                tail = fh.read().decode("utf-8", errors="replace")
            for line in reversed(tail.splitlines()):
                if '"assistant"' not in line:
                    continue
                try:
                    entry = json.loads(line)
                except ValueError:
                    continue  # the tail may begin in the middle of a line
                message = entry.get("message")
                model = message.get("model") if isinstance(message, dict) else None
                if entry.get("type") == "assistant" and model and model != "<synthetic>":
                    return model
        except (OSError, TypeError, AttributeError):
            pass
        return None

    def written_paths(self, payload):
        tool_input = payload.get("tool_input")
        if not isinstance(tool_input, dict):
            return []
        path = tool_input.get("file_path") or tool_input.get("notebook_path")
        return [path] if isinstance(path, str) and path else []


class Codex(Adapter):
    """Codex's payloads and output.

    The payload fields are those of Codex 0.163.0
    (docs/research/harness-hooks-a.md, section 2). Codex rejects a Stop
    output it does not know, and the core prints nothing for a reply, so only
    the context events need a shape: the same hookSpecificOutput object as
    Claude Code. A subagent shares its parent's session_id.
    """

    name = "codex"
    # The patch headers that name a file the patch leaves behind. A line of an
    # added file's content starts with "+", so content cannot imitate a header.
    PATCH_HEADER = re.compile(r"^\*\*\* (?:Add File|Update File|Move to): (.+)$")

    def model(self, payload):
        model = payload.get("model")
        return model if isinstance(model, str) and model else None

    def written_paths(self, payload):
        """The files that an apply_patch call wrote, from the patch's headers.

        Codex gives the gate no list of changed paths, only the patch text.
        """
        tool_input = payload.get("tool_input")
        patch = tool_input.get("command") if isinstance(tool_input, dict) else None
        if not isinstance(patch, str):
            return []
        paths = []
        for line in patch.splitlines():
            match = self.PATCH_HEADER.match(line)
            if match and match.group(1).strip() not in paths:
                paths.append(match.group(1).strip())
        return paths


class Pi(Adapter):
    """Pi's payloads and output.

    Pi has no hook payloads. The extension (harnesses/pi/technical-communication.ts)
    builds one from Pi 0.84.2's events (docs/research/harness-hooks-a.md,
    section 3): session_id, cwd, source (a session_start reason), model and
    last_assistant_message (from agent_end), and tool_input.path (from a write or
    edit result). The output is {"additionalContext": text}, which the extension
    places itself, so the core needs no event name in it.
    """

    name = "pi"

    def model(self, payload):
        model = payload.get("model")
        return model if isinstance(model, str) and model else None

    def written_paths(self, payload):
        """The path of a write or edit call, as Pi's own tools read it."""
        tool_input = payload.get("tool_input")
        path = tool_input.get("path") if isinstance(tool_input, dict) else None
        if not isinstance(path, str) or not path:
            return []
        # Pi's tools drop a leading "@" and expand "~" before they resolve a path.
        return [os.path.expanduser(path[1:] if path.startswith("@") else path)]

    def output(self, event, context):
        return {"additionalContext": context}


ADAPTERS = {adapter.name: adapter for adapter in (ClaudeCode(), Codex(), Pi())}


# --- the layers ---------------------------------------------------------------

def read_text_file(path):
    with open(path, "rb") as fh:
        return fh.read().decode("utf-8")


def under(path, root):
    return path == root or path.startswith(root.rstrip(os.sep) + os.sep)


def is_temporary(path):
    roots = list(TEMP_ROOTS)
    if os.environ.get("TMPDIR"):
        roots.append(os.environ["TMPDIR"])
    roots += [os.path.realpath(root) for root in roots]
    candidates = {path, os.path.realpath(path)}
    return any(under(candidate, root) for candidate in candidates for root in roots)


def never_gated(path, text):
    """True for a file in a class that the gate skips and conversion never touches."""
    if not path.lower().endswith(PROSE_SUFFIXES):
        return True
    if is_temporary(path):
        return True
    parts = path.split(os.sep)
    name = parts[-1]
    if SKIPPED_DIRECTORIES.intersection(parts[:-1]):
        return True
    if name.upper().startswith(LICENCE_PREFIXES):
        return True
    if ".grove" in parts[:-1] and HISTORIC_TASK.match(name):
        return True
    head = "\n".join(text.splitlines()[:5])
    return any(mark in head for mark in GENERATED)


def display_path(path, cwd):
    relative = os.path.relpath(path, cwd)
    return path if relative.startswith("..") else relative


def gate_message(entries):
    """Build the gate's message from (file, hard findings) pairs, within the limit.

    Returns None when no file has a hard finding.
    """
    entries = [(name, found) for name, found in entries if found]
    if not entries:
        return None
    closing = ("The skill expects these findings fixed, or a justified departure "
               "marked, before you finish. If this document was not converted "
               "before your edit, convert it first, as its own change.")
    total = sum(len(found) for _, found in entries)
    listed = min(GATE_FINDING_LIMIT, total)
    while True:
        lines = ["technical-communication check of the files you wrote:"]
        remaining = listed
        for name, found in entries:
            lines.append(f"{name}: {len(found)} hard findings")
            for finding in found[:remaining]:
                lines.append(f"  {finding['line']}:{finding['col']} "
                             f"{finding['rule']}: {finding['message']}")
            remaining = max(0, remaining - len(found))
        if total > listed:
            lines.append(f"{total - listed} more findings are not listed.")
        lines.append(closing)
        message = "\n".join(lines)
        if len(message) <= GATE_MESSAGE_LIMIT or listed == 0:
            return message[:GATE_MESSAGE_LIMIT]
        listed -= 1


def gate(checker, paths, cwd):
    entries = []
    for path in paths:
        path = os.path.normpath(path if os.path.isabs(path) else os.path.join(cwd, path))
        try:
            if os.path.getsize(path) > GATE_FILE_LIMIT:
                continue
            text = read_text_file(path)
        except (OSError, UnicodeDecodeError):
            continue
        if never_gated(path, text):
            continue
        result = checker.analyse(text, path, "auto", reply=False)
        hard = [f for f in result["findings"] if f["level"] == checker.HARD]
        entries.append((display_path(path, cwd), hard))
    return gate_message(entries)


def always_on_text(name):
    with open(os.path.join(ALWAYS_ON, name), encoding="utf-8") as fh:
        return fh.read().strip()


def handle(adapter, event, payload, checker):
    """Serve one event. Returns the context to send, or None."""
    session = adapter.session(payload)
    agent = adapter.agent(payload)
    if event in ("session-start", "subagent-start"):
        text = always_on_text("instruction.md")
        checker.log_start(adapter.name, session, agent,
                          adapter.source(event, payload))
        return text
    if event == "instruction":
        # Pi sends the instruction with every prompt, so this event only reads it.
        # The start record belongs to the session-start event.
        return always_on_text("instruction.md")
    if event == "prompt":
        return always_on_text("reminder.md")
    if event == "write":
        return gate(checker, adapter.written_paths(payload), adapter.cwd(payload))
    if event == "reply":
        checker.log_reply(adapter.reply(payload), adapter.name, session, agent,
                          adapter.model(payload))
        return None
    return None


def main(argv):
    if len(argv) != 3 or argv[1] not in ADAPTERS:
        return 0
    adapter = ADAPTERS[argv[1]]
    event = argv[2]
    checker = load_checker()
    if os.path.exists(os.path.join(checker.state_dir(), "off")):
        return 0
    raw = sys.stdin.buffer.read().decode("utf-8", errors="replace")
    payload = json.loads(raw) if raw.strip() else {}
    if not isinstance(payload, dict):
        return 0
    context = handle(adapter, event, payload, checker)
    if context:
        # one complete object, written once; ASCII escapes survive any locale
        data = json.dumps(adapter.output(event, context)) + "\n"
        sys.stdout.buffer.write(data.encode("ascii"))
        sys.stdout.buffer.flush()
    return 0


if __name__ == "__main__":
    try:
        main(sys.argv)
    except BaseException:  # noqa: BLE001 - a hook failure never reaches the model
        pass
    sys.exit(0)
