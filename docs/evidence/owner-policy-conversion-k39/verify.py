"""One-off verification of the owner's already installed Rhai conversion."""

import hashlib
import json
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path("/tmp/ravel-owner-k39")
LIVE = Path("/Users/antony/.config/ravel")
FRONT = Path("/opt/homebrew/Cellar/ravel/2.0.0/bin/ravel")
OLD = Path("/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel")
PROMPT = "Owner route check: ünicode\nsecond line; $literal `text`"


def digest_tree(root: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root).as_posix()
        if path.is_symlink():
            data = f"link {relative} {os.readlink(path)}".encode()
        elif path.is_file():
            data = f"file {relative} {path.stat().st_mode:o} ".encode() + path.read_bytes()
        else:
            continue
        digest.update(len(data).to_bytes(8, "big"))
        digest.update(data)
    return digest.hexdigest()


def run_check() -> None:
    subjects = [LIVE, FRONT.parent.parent, OLD.parent.parent]
    before = {str(path): digest_tree(path) for path in subjects}
    source = (LIVE / "config.rhai").read_text()
    kinds = re.findall(r'^\s*"([a-z-]+)"\s*=>', source, re.MULTILINE)
    expected: dict[str, tuple[str, str, str]] = {}
    for kind in kinds:
        if kind in ("requirements", "integrate-review-requirements"):
            route = ("codex", "gpt-6.1-sol", "max")
        elif kind == "review-requirements":
            route = ("claude", "claude-opus-5-5", "max")
        elif kind in ("design", "integrate-review-design", "research-a", "combine-research"):
            route = ("codex", "gpt-6.1-sol", "xhigh")
        elif kind in ("review-design", "research-b"):
            route = ("claude", "claude-opus-5-5", "xhigh")
        elif kind.startswith("review-"):
            route = ("claude", "claude-sonnet-5-5", "medium" if "prototype" in kind else "high")
        elif kind in ("draft", "finish"):
            route = ("claude", "claude-opus-5-5", "high" if kind == "draft" else "medium")
        elif kind in ("copy-edit", "art"):
            route = ("codex", "gpt-6.1-terra", "high")
        elif kind == "proof":
            route = ("codex", "gpt-6.1-sol", "medium")
        elif kind == "release-notes":
            route = ("claude", "claude-sonnet-5-5", "medium")
        else:
            assert kind in (
                "planning",
                "integrate-review-planning",
                "prototype",
                "integrate-review-prototype",
                "impl",
                "integrate-review-impl",
            ), kind
            route = ("codex", "gpt-6.1-sol", "medium" if "prototype" in kind else "high")
        expected[kind] = route
    assert len(kinds) == len(expected) == 24, kinds
    guard = '(version 1)(allow default)(deny file-read* file-write* (subpath "/Users/antony"))(deny network*)'
    control = subprocess.run(
        ["/usr/bin/sandbox-exec", "-p", guard, "/bin/ls", "/Users/antony"],
        check=False,
        capture_output=True,
        timeout=10,
    )
    assert control.returncode != 0, "sandbox admitted real home"
    rows: list[dict[str, object]] = []
    skill_sources: list[dict[str, str]] | None = None
    with tempfile.TemporaryDirectory(prefix="ravel-k39-") as directory:
        work = Path(directory).resolve()
        home = work / "home"
        copied = home / ".config/ravel"
        shutil.copytree(LIVE, copied)
        tools = work / "tools"
        tools.mkdir()
        fake = tools / "bili"
        fake.write_text(
            '#!/bin/sh\nset -eu\nprintf "%s\\0" "$@" >"$CAPTURE"\n'
            'printf "%s" "${RAVEL_SKILL_DIRECTORIES-}" >"$CAPTURE.descriptor"\n'
            'exec "$FRONT" end-session verified\n'
        )
        fake.chmod(0o700)
        for name in ("codex", "claude"):
            (tools / name).write_text("#!/bin/sh\nexit 99\n")
            (tools / name).chmod(0o700)
        plain = work / "plain"
        primary = work / "main"
        secondary = work / "secondary"
        plain.mkdir()
        (primary / ".jj/repo").mkdir(parents=True)
        (secondary / ".jj").mkdir(parents=True)
        (secondary / ".jj/repo").write_text("../../main/.jj/repo")
        temporary = work / "tmp"
        temporary.mkdir()
        for case, cwd, grants in [
            ("plain", plain, []),
            ("primary", primary, []),
            ("secondary", secondary, ["--add-dir", str(primary)]),
        ]:
            for kind, (program, model, effort) in expected.items():
                capture = work / "capture"
                capture.unlink(missing_ok=True)
                env = {
                    "HOME": str(home),
                    "PATH": str(tools),
                    "TMPDIR": str(temporary),
                    "FRONT": str(FRONT),
                    "CAPTURE": str(capture),
                }
                run = subprocess.run(
                    [
                        "/usr/bin/sandbox-exec",
                        "-p",
                        guard,
                        str(FRONT),
                        "start-session",
                        "--kind",
                        kind,
                        "--prompt",
                        PROMPT,
                        "--json",
                    ],
                    cwd=cwd,
                    check=False,
                    env=env,
                    capture_output=True,
                    timeout=60,
                )
                assert run.returncode == 0 and run.stdout == b"verified", (
                    case,
                    kind,
                    run.returncode,
                    run.stderr.decode(),
                )
                args = capture.read_bytes().decode().split("\0")[:-1]
                if program == "codex":
                    wanted = [
                        program,
                        "--",
                        "--dangerously-bypass-hook-trust",
                        "--no-alt-screen",
                        "--model",
                        model,
                        "-c",
                        f"model_reasoning_effort={effort}",
                        "-c",
                        "default_permissions=:danger-full-access",
                        "--ask-for-approval",
                        "never",
                        *grants,
                    ]
                else:
                    wanted = [program, "--", *grants, "--model", model, "--effort", effort]
                assert args[:-1] == wanted, (case, kind, args, wanted)
                descriptor = Path(capture.with_suffix(".descriptor").read_text())
                assert descriptor.is_relative_to(home / ".cache/ravel/skills/v1")
                assert descriptor.is_file()
                prompt = args[-1]
                block = re.search(
                    r"BEGIN RAVEL CALLER TASK ([0-9a-f]+)\n(.*?)\nEND RAVEL CALLER TASK \1",
                    prompt,
                    re.DOTALL,
                )
                assert block is not None and block.group(2) == PROMPT
                assert prompt.count(PROMPT) == 1
                assert str(descriptor) in prompt
                notices = [
                    json.loads(line)
                    for line in run.stderr.decode().splitlines()
                    if line.startswith("{")
                ]
                assert len(notices) == 1, notices
                notice = notices[0]
                assert notice["selection"]["model"] == model
                assert notice["selection"]["effort"] == effort
                if skill_sources is None:
                    skill_sources = notice["skills"]["sources"]
                assert skill_sources == notice["skills"]["sources"]
                rows.append(
                    {
                        "case": case,
                        "kind": kind,
                        "program": "bili",
                        "harness": program,
                        "model": model,
                        "effort": effort,
                        "arguments": "exact match",
                        "prompt": "skill preamble plus exact caller block",
                        "stop_diagnostics": notice.get("operationReports", []),
                    }
                )
        env = {
            "HOME": str(home),
            "PATH": str(tools),
            "TMPDIR": str(temporary),
            "FRONT": str(FRONT),
            "CAPTURE": str(work / "refused-capture"),
        }
        refused = subprocess.run(
            [
                "/usr/bin/sandbox-exec",
                "-p",
                guard,
                str(FRONT),
                "start-session",
                "--kind",
                "unlisted-k39",
                "--prompt",
                PROMPT,
                "--json",
            ],
            cwd=plain,
            check=False,
            env=env,
            capture_output=True,
            timeout=60,
        )
        assert refused.returncode == 3 and b"incomplete_mapping" in refused.stderr
        assert not (work / "refused-capture").exists()
        old = subprocess.run(
            [
                "/usr/bin/sandbox-exec",
                "-p",
                guard,
                str(OLD),
                "start-session",
                "--kind",
                "impl",
                "--prompt",
                PROMPT,
                "--json",
            ],
            cwd=plain,
            check=False,
            env=env,
            capture_output=True,
            timeout=30,
        )
        assert old.returncode == 2 and b"settings_invalid" in old.stderr, old.stderr
        assert not (work / "refused-capture").exists()
    after = {str(path): digest_tree(path) for path in subjects}
    assert before == after, (before, after)
    report = {
        "binary": str(FRONT),
        "config_sha256": hashlib.sha256(source.encode()).hexdigest(),
        "subjects_before": before,
        "subjects_after": after,
        "rows": rows,
        "skill_sources": skill_sources,
        "sandbox_control": "real-home read denied; network denied by profile",
        "unknown_kind": "status 3 incomplete_mapping; no fake launched",
        "old_with_copied_settings": "status 2 settings_invalid; no fake launched",
        "historical_comparison": "unavailable: original policy.ts absent",
    }
    (ROOT / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    print(
        f"PASS: {len(rows)} routes; unknown-kind refusal; 1.1 settings refusal; unchanged subjects"
    )


if __name__ == "__main__":
    run_check()
