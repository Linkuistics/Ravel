#!/usr/bin/env python3
"""Count the always-on hook declarations of each Claude Code installation.

The pack has two Claude Code plugins. The writing plugin holds the skills and
declares no hook. The plugin technical-communication-hooks holds the hooks and
no skill. The writing plugin names the hooks plugin in its `dependencies`. No
route installs the writing plugin now, and the count keeps the dependency
from adding a second declaration of any hook.

This check reads the manifests and hooks files as Claude Code reads them. For
each plugin, it takes the default `hooks/hooks.json` and each entry of the
manifest's `hooks` field. It follows the `dependencies` of each plugin. It then
counts the declarations of `tc-hook` for three installations: the writing
plugin alone, the hooks plugin alone, and both. Each installation must give
exactly one declaration for each expected hook.

Grove's docs/specs/pack-membership.md, the row "A second plugin in one
repository", asks for this count.

The rules that this check models come from the Claude Code plugin reference,
https://code.claude.com/docs/en/plugins-reference:
- "hooks ... Loaded together with hooks/hooks.json" (the manifest merges).
- "Bare names resolve against this plugin's own marketplace" (dependencies).
- "A plugin with SKILL.md at its root, no skills/, and no skills key loads as
  a single skill". The hooks plugin's root holds SKILL.md, so its manifest
  must carry a `skills` key, and that key must list nothing.

Usage:
  hook-count.py PACK_ROOT --plugin grove=DIR
  hook-count.py PACK_ROOT --plugin grove=DIR --controls

PACK_ROOT is the repository root. Each --plugin names a plugin outside the
pack that a dependency can name. With --controls, the check also runs against
mutated scratch copies of the pack and requires each copy to fail.
"""

from __future__ import annotations

import argparse
import json
import shutil
import sys
import tempfile
from collections import Counter
from pathlib import Path

WRITING = "writing"
HOOKS = "technical-communication-hooks"
PLUGIN_ROOT_VARIABLE = "${CLAUDE_PLUGIN_ROOT}"

# One entry for each hook: (event, matcher, the arguments of tc-hook).
EXPECTED: frozenset[tuple[str, str, tuple[str, ...]]] = frozenset(
    {
        ("SessionStart", "", ("claude-code", "session-start")),
        ("SubagentStart", "", ("claude-code", "subagent-start")),
        ("UserPromptSubmit", "", ("claude-code", "prompt")),
        ("PostToolUse", "Write|Edit|NotebookEdit", ("claude-code", "write")),
        ("Stop", "", ("claude-code", "reply")),
        ("SubagentStop", "", ("claude-code", "reply")),
    }
)

INSTALLATIONS: tuple[tuple[str, ...], ...] = ((WRITING,), (HOOKS,), (WRITING, HOOKS))

Key = tuple[str, str, tuple[str, ...]]


class CheckError(Exception):
    """A finding that fails the check."""


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CheckError(f"{path}: cannot read JSON: {error}") from error


def manifest(root: Path) -> dict:
    data = load_json(root / ".claude-plugin" / "plugin.json")
    if not isinstance(data, dict):
        raise CheckError(f"{root}: the manifest is not a JSON object")
    return data


def pack_plugins(pack_root: Path) -> dict[str, Path]:
    """Map the name of each Claude Code plugin in the pack to its root."""
    plugins: dict[str, Path] = {}
    for path in sorted((pack_root / "plugins").rglob(".claude-plugin/plugin.json")):
        root = path.parent.parent
        name = manifest(root).get("name")
        if not isinstance(name, str):
            raise CheckError(f"{path}: the manifest has no name")
        if name in plugins:
            raise CheckError(f"two plugins of the pack have the name {name}")
        plugins[name] = root
    return plugins


def dependency_name(entry: object, owner: str) -> str:
    if isinstance(entry, str):
        return entry.split("@", 1)[0]
    if isinstance(entry, dict) and isinstance(entry.get("name"), str):
        return entry["name"]
    raise CheckError(f"{owner}: a dependency entry has no name: {entry!r}")


def closure(requested: tuple[str, ...], plugins: dict[str, Path]) -> list[str]:
    """Return the requested plugins and every dependency, each once."""
    seen: list[str] = []
    pending = list(requested)
    while pending:
        name = pending.pop(0)
        if name in seen:
            continue
        if name not in plugins:
            raise CheckError(f"the dependency {name} names no known plugin")
        seen.append(name)
        for entry in manifest(plugins[name]).get("dependencies", []):
            pending.append(dependency_name(entry, name))
    return seen


def hook_maps(root: Path) -> list[tuple[str, dict]]:
    """Return each event map that Claude Code loads for one plugin."""
    maps: list[tuple[str, dict]] = []

    def from_file(path: Path) -> None:
        data = load_json(path)
        if not isinstance(data, dict) or not isinstance(data.get("hooks"), dict):
            raise CheckError(f"{path}: a hooks file needs a top-level hooks object")
        maps.append((str(path), data["hooks"]))

    default = root / "hooks" / "hooks.json"
    if default.is_file():
        from_file(default)

    declared = manifest(root).get("hooks")
    entries = declared if isinstance(declared, list) else [declared] if declared else []
    for entry in entries:
        if isinstance(entry, str):
            from_file(root / entry)
        elif isinstance(entry, dict):
            maps.append((f"{root} manifest", entry))
        else:
            raise CheckError(f"{root}: a hooks entry is neither a path nor an object")
    return maps


def declarations(name: str, root: Path) -> list[Key]:
    """Return the tc-hook declarations of one plugin, as keys."""
    found: list[Key] = []
    for origin, events in hook_maps(root):
        for event, groups in events.items():
            for group in groups:
                matcher = group.get("matcher", "")
                for handler in group.get("hooks", []):
                    argv = [handler.get("command", ""), *handler.get("args", [])]
                    argv = [part.replace(PLUGIN_ROOT_VARIABLE, str(root)) for part in argv]
                    for index, part in enumerate(argv):
                        if Path(part).name == "tc-hook":
                            script = Path(part)
                            if not script.is_file():
                                raise CheckError(
                                    f"{name}: {origin} runs {script}, which does not exist"
                                )
                            found.append((event, matcher, tuple(argv[index + 1 :])))
                            break
    return found


def check_layout(plugins: dict[str, Path]) -> None:
    hooks_root = plugins[HOOKS]
    if (hooks_root / "skills").exists():
        raise CheckError(f"{HOOKS}: the plugin has a skills/ directory")
    hooks_manifest = manifest(hooks_root)
    if hooks_manifest.get("skills", None) != []:
        raise CheckError(
            f"{HOOKS}: the manifest needs \"skills\": [], or Claude Code loads "
            "the root SKILL.md as a skill"
        )

    writing_root = plugins[WRITING]
    if manifest(writing_root).get("hooks"):
        raise CheckError(f"{WRITING}: the manifest declares hooks")
    if (writing_root / "hooks" / "hooks.json").exists():
        raise CheckError(f"{WRITING}: the plugin has a hooks/hooks.json")
    names = [
        dependency_name(entry, WRITING)
        for entry in manifest(writing_root).get("dependencies", [])
    ]
    if HOOKS not in names:
        raise CheckError(f"{WRITING}: the dependencies do not name {HOOKS}")

    # The same root is the Codex hooks plugin. Codex 0.166.0 prefers
    # .codex-plugin/plugin.json to .claude-plugin/plugin.json, and it reads the
    # default hooks/hooks.json only when that manifest has no hooks field:
    # https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core-plugins/src/loader.rs
    # (load_plugin_hooks) and codex-rs/exec-server-protocol/src/protocol.rs
    # (DISCOVERABLE_PLUGIN_MANIFEST_PATHS). Without the field, Codex would run
    # the Claude Code hooks file.
    codex = load_json(hooks_root / ".codex-plugin" / "plugin.json")
    if not isinstance(codex, dict) or codex.get("hooks") != "./harnesses/codex/hooks.json":
        raise CheckError(
            "the Codex manifest needs \"hooks\": \"./harnesses/codex/hooks.json\", "
            "or Codex reads the Claude Code hooks/hooks.json"
        )


def run(pack_root: Path, external: dict[str, Path], quiet: bool = False) -> None:
    plugins = pack_plugins(pack_root)
    for name, root in external.items():
        plugins.setdefault(name, root)
    for name in (WRITING, HOOKS):
        if name not in plugins:
            raise CheckError(f"the pack has no plugin named {name}")

    for requested in INSTALLATIONS:
        installed = closure(requested, plugins)
        counts: Counter[Key] = Counter()
        for name in installed:
            counts.update(declarations(name, plugins[name]))
        label = " + ".join(requested)
        problems = [
            f"{event} {matcher or '*'} {' '.join(args)}: {counts[(event, matcher, args)]}"
            for event, matcher, args in sorted(EXPECTED | set(counts))
            if counts[(event, matcher, args)] != (1 if (event, matcher, args) in EXPECTED else 0)
        ]
        if problems:
            raise CheckError(
                f"installation {label} (plugins {', '.join(installed)}) gives these "
                "declaration counts, and each must be 1: " + "; ".join(problems)
            )
        if not quiet:
            print(
                f"{label}: plugins {', '.join(installed)}; "
                f"{sum(counts.values())} declarations, one for each of {len(EXPECTED)} hooks"
            )

    # The layout rules come after the count, so a second declaration fails the
    # count first.
    check_layout(plugins)


def mutate_manifest(path: Path, change) -> None:
    data = json.loads(path.read_text(encoding="utf-8"))
    change(data)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def controls(pack_root: Path, external: dict[str, Path]) -> None:
    writing = Path("plugins/writing")
    hooks = writing / "craft/technical-communication"

    def manifest_hooks(copy: Path) -> None:
        mutate_manifest(
            copy / writing / ".claude-plugin/plugin.json",
            lambda data: data.__setitem__(
                "hooks",
                {
                    "Stop": [
                        {
                            "hooks": [
                                {
                                    "type": "command",
                                    "command": "/bin/sh",
                                    "args": [
                                        "${CLAUDE_PLUGIN_ROOT}/craft/technical-communication/scripts/tc-hook",
                                        "claude-code",
                                        "reply",
                                    ],
                                }
                            ]
                        }
                    ]
                },
            ),
        )

    def default_hooks_file(copy: Path) -> None:
        source = (copy / hooks / "hooks/hooks.json").read_text(encoding="utf-8")
        target = copy / writing / "hooks/hooks.json"
        target.parent.mkdir()
        target.write_text(
            source.replace(
                "${CLAUDE_PLUGIN_ROOT}/scripts/",
                "${CLAUDE_PLUGIN_ROOT}/craft/technical-communication/scripts/",
            ),
            encoding="utf-8",
        )

    def second_handler(copy: Path) -> None:
        path = copy / hooks / "hooks/hooks.json"
        data = json.loads(path.read_text(encoding="utf-8"))
        group = data["hooks"]["Stop"][0]
        group["hooks"].append(dict(group["hooks"][0]))
        path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

    def lost_event(copy: Path) -> None:
        path = copy / hooks / "hooks/hooks.json"
        data = json.loads(path.read_text(encoding="utf-8"))
        del data["hooks"]["SessionStart"]
        path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

    def no_dependency(copy: Path) -> None:
        mutate_manifest(
            copy / writing / ".claude-plugin/plugin.json",
            lambda data: data.__setitem__(
                "dependencies", [d for d in data["dependencies"] if d != HOOKS]
            ),
        )

    def no_skills_key(copy: Path) -> None:
        mutate_manifest(
            copy / hooks / ".claude-plugin/plugin.json",
            lambda data: data.pop("skills"),
        )

    def no_codex_hooks_field(copy: Path) -> None:
        mutate_manifest(
            copy / hooks / ".codex-plugin/plugin.json",
            lambda data: data.pop("hooks"),
        )

    def missing_script(copy: Path) -> None:
        (copy / hooks / "scripts/tc-hook").unlink()

    # Each case names a fragment of the error that it must cause, so a control
    # cannot pass by failing for another reason.
    cases = [
        ("the writing manifest also declares a Stop hook", manifest_hooks, "Stop * claude-code reply: 2"),
        ("the writing plugin also has a default hooks file", default_hooks_file, "SessionStart * claude-code session-start: 2"),
        ("the Stop hook has a second handler", second_handler, "Stop * claude-code reply: 2"),
        ("the hooks file loses SessionStart", lost_event, "SessionStart * claude-code session-start: 0"),
        ("the writing plugin drops the dependency", no_dependency, "installation writing (plugins writing, grove)"),
        ("the hooks plugin loses tc-hook", missing_script, "which does not exist"),
        ("the hooks manifest drops its skills key", no_skills_key, "the root SKILL.md"),
        ("the Codex manifest drops its hooks field", no_codex_hooks_field, "Codex reads the Claude Code"),
    ]
    for label, change, fragment in cases:
        with tempfile.TemporaryDirectory(prefix="hook-count-") as scratch:
            copy = Path(scratch)
            shutil.copytree(pack_root / "plugins", copy / "plugins")
            change(copy)
            try:
                run(copy, external, quiet=True)
            except CheckError as error:
                if fragment not in str(error):
                    raise CheckError(
                        f"control {label} failed for another reason: {error}"
                    ) from error
                print(f"control failed as required: {label}: {error}")
                continue
            raise CheckError(f"control passed, so the check misses it: {label}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("pack_root", type=Path)
    parser.add_argument("--plugin", action="append", default=[], metavar="NAME=DIR")
    parser.add_argument("--controls", action="store_true")
    options = parser.parse_args()

    external: dict[str, Path] = {}
    for item in options.plugin:
        name, _, directory = item.partition("=")
        if not name or not directory:
            parser.error(f"--plugin needs NAME=DIR, not {item}")
        external[name] = Path(directory).resolve()

    try:
        run(options.pack_root.resolve(), external)
        if options.controls:
            controls(options.pack_root.resolve(), external)
    except CheckError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
