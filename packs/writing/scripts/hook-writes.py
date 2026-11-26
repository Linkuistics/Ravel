#!/usr/bin/env python3
"""Check that no technical-communication hook writes inside the source.

The hooks run in place from an installed source. A Homebrew keg holds the
`writing` source, and Ravel digests every file of it. A hook that writes a
file there, such as Python bytecode or a log, changes the source's
`resourceDigest`, and the owner switch then refuses the source.

This check copies the source twice into a scratch folder. It makes one copy
read-only and leaves the other writable. Then it runs every declared hook
command against each copy:

- each command of `hooks/hooks.json`, with `${CLAUDE_PLUGIN_ROOT}` set to the
  copy's skill directory, as Claude Code runs it;
- each command of `harnesses/codex/hooks.json` through `/bin/sh -c`, with
  `PLUGIN_ROOT` set to the same directory, as Codex runs it;
- `tc-hook pi EVENT` for each core event of the Pi extension, and a real Pi
  that loads the extension from the copy, when Pi is installed.

Each command receives a payload that makes its event do its work. The write
event names a document with a hard finding, and the reply event carries
prose for the reply log. Each command must exit 0, the context events must
add their text, and the gate must report the finding. Both copies must give
the same outputs. After all the runs, the writable copy must have the same
tree digest as before: the same paths, types, modes, contents and link text.

The scratch folder lies in the repository's ignored `.cache/`, because the
gate skips every file below a temporary root such as `/var/folders`. HOME and
XDG_STATE_HOME name folders inside it, so the reply log goes there. Pi keeps
the caller's HOME, because mise starts it, and only its reply log moves.

Usage:
  hook-writes.py PACK_ROOT
  hook-writes.py PACK_ROOT --controls

With --controls, the check also runs against two mutated copies and requires
each to fail: one without the bytecode fix, and one whose core appends a log
line inside its own folder.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path

SOURCE = Path("plugins/writing")
SKILL = Path("craft/technical-communication")
CLAUDE_HOOKS = SKILL / "hooks/hooks.json"
CODEX_HOOKS = SKILL / "harnesses/codex/hooks.json"
PI_EXTENSION = SKILL / "harnesses/pi/technical-communication.ts"
GUARD = SKILL / "scripts/tc-hook"
CORE = SKILL / "scripts/tc_hook.py"

# The bytecode fix, as the mutation control removes it.
GUARD_FLAG = '"$python" -I -B '
GUARD_WITHOUT_FLAG = '"$python" -I '
CORE_SETTING = "sys.dont_write_bytecode = True\n"

DOCUMENT = "notes.md"
# One semicolon is a hard finding of the checker.
DOCUMENT_TEXT = "The gate reads this line; it has a semicolon.\n"
REPLY = ("The worker restarts after each failure. "
         "The log names the cause of the failure.")
GATE_MARK = "technical-communication check of the files you wrote"
COMMAND_TIMEOUT = 60  # seconds; the declared timeouts are 5 to 15
PI_INPUT_SECONDS = 3  # Pi does not wait for the extension's child process


class CheckError(Exception):
    """A finding that fails the check."""


def tree_digest(root: Path) -> dict[str, str]:
    """Map each path below root to a digest of its type, mode and content."""
    entries: dict[str, str] = {}
    for path in sorted(root.rglob("*")):
        info = path.lstat()
        mode = stat.S_IMODE(info.st_mode)
        if stat.S_ISLNK(info.st_mode):
            entry = f"link {mode:o} {os.readlink(path)}"
        elif stat.S_ISDIR(info.st_mode):
            entry = f"dir {mode:o}"
        else:
            entry = f"file {mode:o} {hashlib.sha256(path.read_bytes()).hexdigest()}"
        entries[str(path.relative_to(root))] = entry
    return entries


def tree_changes(before: dict[str, str], after: dict[str, str]) -> list[str]:
    changes = [f"added {path}" for path in sorted(after.keys() - before.keys())]
    changes += [f"removed {path}" for path in sorted(before.keys() - after.keys())]
    changes += [f"changed {path}" for path in sorted(before.keys() & after.keys())
                if before[path] != after[path]]
    return changes


def set_writable(root: Path, writable: bool) -> None:
    for path in [root, *root.rglob("*")]:
        if path.is_symlink():
            continue
        mode = stat.S_IMODE(path.stat().st_mode)
        path.chmod(mode | stat.S_IWUSR if writable else mode & ~0o222)


def load_hooks(path: Path) -> dict:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CheckError(f"{path}: cannot read JSON: {error}") from error
    if not isinstance(data, dict) or not isinstance(data.get("hooks"), dict):
        raise CheckError(f"{path}: a hooks file needs a top-level hooks object")
    return data["hooks"]


def declared_hooks(path: Path) -> list[tuple[str, dict]]:
    """Each (event name, hook entry) of one hooks file."""
    found = []
    for event, groups in load_hooks(path).items():
        for group in groups:
            for hook in group.get("hooks", []):
                found.append((event, hook))
    if not found:
        raise CheckError(f"{path}: declares no hook")
    return found


def hook_event(words: list[str], where: str) -> tuple[str, str]:
    """The harness and core event that follow tc-hook in a command."""
    for index, word in enumerate(words):
        if word.rstrip('"').endswith("tc-hook") and index + 2 < len(words):
            return words[index + 1], words[index + 2]
    raise CheckError(f"{where}: the command does not run tc-hook HARNESS EVENT")


class Commands:
    """The hook commands of one copy of the source, as the harnesses run them."""

    def __init__(self, source: Path) -> None:
        self.source = source
        self.skill = source / SKILL

    def all(self) -> list[tuple[str, str, str, list[str], dict[str, str]]]:
        """Each command as (label, harness, event, argv, extra environment)."""
        commands = []
        for event_name, hook in declared_hooks(self.source / CLAUDE_HOOKS):
            argv = [hook["command"], *hook.get("args", [])]
            argv = [word.replace("${CLAUDE_PLUGIN_ROOT}", str(self.skill)) for word in argv]
            harness, event = hook_event(argv, f"{CLAUDE_HOOKS} {event_name}")
            commands.append((f"claude {event_name}", harness, event, argv,
                             {"CLAUDE_PLUGIN_ROOT": str(self.skill)}))
        for event_name, hook in declared_hooks(self.source / CODEX_HOOKS):
            command = hook["command"]
            harness, event = hook_event(command.split(), f"{CODEX_HOOKS} {event_name}")
            commands.append((f"codex {event_name}", harness, event,
                             ["/bin/sh", "-c", command], {"PLUGIN_ROOT": str(self.skill)}))
        text = (self.source / PI_EXTENSION).read_text(encoding="utf-8")
        pi_events = sorted(set(re.findall(r'\bcore\(\s*"([a-z-]+)"', text)))
        if not pi_events:
            raise CheckError(f"{PI_EXTENSION}: names no core event")
        for event in pi_events:
            commands.append((f"pi {event}", "pi", event,
                             ["/bin/sh", str(self.skill / "scripts/tc-hook"), "pi", event], {}))
        return commands


def payload(harness: str, event: str, work: Path) -> dict:
    document = str(work / DOCUMENT)
    data: dict = {"session_id": f"{harness}-{event}", "cwd": str(work), "source": "startup"}
    if event == "write":
        if harness == "claude-code":
            data["tool_input"] = {"file_path": document}
        elif harness == "codex":
            data["tool_input"] = {"command": f"*** Begin Patch\n*** Update File: {document}\n"}
        else:
            data["tool_input"] = {"path": document}
    if event == "reply":
        data["last_assistant_message"] = REPLY
    return data


def expected_mark(event: str) -> str | None:
    """Text that the output of an event must hold, or None for no output."""
    if event in ("session-start", "subagent-start", "instruction"):
        return "technical-communication"
    if event == "prompt":
        return "additionalContext"
    if event == "write":
        return GATE_MARK
    return None


class Scratch:
    """HOME, the state folder and the work folder of one run."""

    def __init__(self, root: Path, name: str) -> None:
        self.root = root / name
        self.home = self.root / "home"
        self.state = self.root / "state"
        self.work = self.root / "work"
        for folder in (self.home, self.state, self.work):
            folder.mkdir(parents=True)
        (self.work / DOCUMENT).write_text(DOCUMENT_TEXT, encoding="utf-8")

    def environment(self, extra: dict[str, str]) -> dict[str, str]:
        env = {"HOME": str(self.home), "XDG_STATE_HOME": str(self.state),
               "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
               "LANG": os.environ.get("LANG", "en_GB.UTF-8")}
        env.update(extra)
        return env


def run_commands(source: Path, scratch: Scratch) -> dict[str, str]:
    """Run each hook command against one copy. Returns each label's output."""
    outputs = {}
    for label, harness, event, argv, extra in Commands(source).all():
        data = json.dumps(payload(harness, event, scratch.work))
        try:
            result = subprocess.run(argv, input=data, capture_output=True, text=True,
                                    cwd=scratch.work, env=scratch.environment(extra),
                                    timeout=COMMAND_TIMEOUT, check=False)
        except subprocess.TimeoutExpired as error:
            raise CheckError(f"{label}: did not finish in {COMMAND_TIMEOUT} s") from error
        if result.returncode != 0:
            raise CheckError(f"{label}: exit {result.returncode}")
        mark = expected_mark(event)
        if mark is None and result.stdout.strip():
            raise CheckError(f"{label}: printed output for an event that adds nothing")
        if mark is not None and mark not in result.stdout:
            raise CheckError(f"{label}: the output lacks '{mark}': {result.stdout[:200]!r}")
        outputs[label] = result.stdout
    sessions = scratch.state / "technical-communication" / "sessions"
    logs = "".join(path.read_text(encoding="utf-8") for path in sorted(sessions.glob("*.jsonl")))
    if '"kind":"start"' not in logs or '"kind":"reply"' not in logs:
        raise CheckError("the reply log holds no start record or no reply record")
    return outputs


def pi_environment(scratch: Scratch) -> dict[str, str]:
    """The caller's environment with the reply log in scratch.

    Pi starts through mise, and mise refuses its own configuration under
    another HOME or XDG_STATE_HOME. Pi therefore keeps both, as
    `pi-extension-load.test.sh` does, and only the log variable moves.
    """
    env = dict(os.environ)
    env["TECHNICAL_COMMUNICATION_STATE_DIR"] = str(scratch.state / "technical-communication")
    return env


def run_pi(source: Path, scratch: Scratch) -> str:
    """Load the Pi extension from the copy in a real Pi, without a model."""
    if shutil.which("pi") is None:
        return "skip   pi is not installed; the Pi extension was not loaded"
    extension = source / PI_EXTENSION
    process = subprocess.Popen(
        ["pi", "--mode", "rpc", "--no-session", "-ne", "-e", str(extension)],
        stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
        cwd=scratch.work, env=pi_environment(scratch))
    time.sleep(PI_INPUT_SECONDS)
    try:
        _, error = process.communicate(input=b"", timeout=COMMAND_TIMEOUT)
    except subprocess.TimeoutExpired as timeout:
        process.kill()
        process.communicate()
        raise CheckError("pi: did not leave after its input ended") from timeout
    if error.strip():
        raise CheckError(f"pi: printed on standard error: {error[:300]!r}")
    logs = "".join(path.read_text(encoding="utf-8") for path in
                   sorted((scratch.state / "technical-communication" / "sessions").glob("pi--*")))
    if '"harness":"pi"' not in logs:
        raise CheckError("pi: the extension wrote no start record")
    return "pass   pi loaded the extension from the writable copy"


def check(source: Path, scratch_root: Path) -> list[str]:
    """Run every hook against a read-only and a writable copy of source."""
    notes = []
    writable = scratch_root / "writable"
    read_only = scratch_root / "read-only"
    shutil.copytree(source, writable, symlinks=True)
    shutil.copytree(source, read_only, symlinks=True)
    for copy in (writable, read_only):
        shutil.rmtree(copy / SKILL / "scripts" / "__pycache__", ignore_errors=True)
    set_writable(read_only, False)
    try:
        before = tree_digest(writable)
        written = run_commands(writable, Scratch(scratch_root, "run-writable"))
        notes.append(run_pi(writable, Scratch(scratch_root, "run-pi")))
        changes = tree_changes(before, tree_digest(writable))
        if changes:
            raise CheckError("the hooks wrote inside the writable copy: " + ", ".join(changes))
        protected = run_commands(read_only, Scratch(scratch_root, "run-read-only"))
        for label, output in written.items():
            if protected[label] != output.replace(str(writable), str(read_only)):
                raise CheckError(f"{label}: the read-only copy gave other output")
    finally:
        set_writable(read_only, True)
    notes.append(f"pass   {len(written)} hook commands left the writable copy unchanged "
                 "and gave the same output from the read-only copy")
    return notes


def mutate(source: Path, target: Path, name: str) -> None:
    """Copy source to target, then apply one mutation that must fail the check."""
    shutil.copytree(source, target, symlinks=True)
    guard = target / GUARD
    core = target / CORE
    if name == "no-bytecode-fix":
        guard_text = guard.read_text(encoding="utf-8")
        core_text = core.read_text(encoding="utf-8")
        if GUARD_FLAG not in guard_text or CORE_SETTING not in core_text:
            raise CheckError("the control is stale: the bytecode fix is not where it expects")
        guard.write_text(guard_text.replace(GUARD_FLAG, GUARD_WITHOUT_FLAG), encoding="utf-8")
        core.write_text(core_text.replace(CORE_SETTING, ""), encoding="utf-8")
    elif name == "log-in-root":
        core_text = core.read_text(encoding="utf-8")
        hook = "    raw = sys.stdin.buffer.read()"
        if hook not in core_text:
            raise CheckError("the control is stale: main reads no payload where it expects")
        line = '    open(os.path.join(SCRIPTS, "hook.log"), "a").write("event\\n")\n'
        core.write_text(core_text.replace(hook, line + hook, 1), encoding="utf-8")
    else:
        raise CheckError(f"no mutation {name}")


def controls(source: Path, scratch_root: Path) -> list[str]:
    notes = []
    for name in ("no-bytecode-fix", "log-in-root"):
        mutant = scratch_root / f"mutant-{name}"
        mutate(source, mutant / "source", name)
        try:
            check(mutant / "source", mutant)
        except CheckError as error:
            if "wrote inside the writable copy" not in str(error):
                raise CheckError(f"control {name} failed for another reason: {error}") from error
            notes.append(f"pass   control {name} fails: {str(error)[:160]}")
            continue
        raise CheckError(f"control {name} passed, so the check cannot see that write")
    return notes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n", 1)[0])
    parser.add_argument("pack_root", type=Path)
    parser.add_argument("--controls", action="store_true",
                        help="also require two mutated copies to fail")
    args = parser.parse_args()
    pack_root = args.pack_root.resolve()
    source = pack_root / SOURCE
    cache = pack_root / ".cache"
    cache.mkdir(exist_ok=True)
    scratch_root = Path(tempfile.mkdtemp(prefix="hook-writes-", dir=cache))
    try:
        notes = check(source, scratch_root / "check")
        if args.controls:
            notes += controls(source, scratch_root / "controls")
    except CheckError as error:
        print(f"hook-writes: FAILED — {error}", file=sys.stderr)
        return 1
    finally:
        set_writable(scratch_root, True)
        shutil.rmtree(scratch_root)
    for note in notes:
        print(note)
    print("hook-writes: no hook writes inside the source")
    return 0


if __name__ == "__main__":
    sys.exit(main())
