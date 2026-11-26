"""One-time inventory evidence instrument; run from the repository root."""

import copy
import hashlib
import json
import re
import subprocess
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path.cwd()
OUT = ROOT / "docs/evidence/rhai-book-workflows-inventory-k129"
PREVIOUS = ROOT / "docs/evidence/rhai-book-components-inventory-k128/subjects.sha256"


def read_json(path: Path) -> object:
    return json.loads(path.read_text())


def write_json(name: str, value: object) -> None:
    (OUT / name).write_text(json.dumps(value, indent=2) + "\n")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(path: Path) -> dict[str, str]:
    return {name: value for value, name in
            (line.split("  ", 1) for line in path.read_text().splitlines())}


def references(value: object) -> list[dict]:
    if isinstance(value, dict):
        found = [value] if {"path", "start", "end"} <= value.keys() else []
        return found + [ref for child in value.values() for ref in references(child)]
    if isinstance(value, list):
        return [ref for child in value for ref in references(child)]
    return []


def valid_reference(ref: dict) -> bool:
    path = ROOT / ref["path"]
    return path.is_file() and 1 <= ref["start"] <= ref["end"] <= len(
        path.read_bytes().splitlines()
    )


def check_membership(inventory: dict, public: set[str], declarations: set[str],
                     scripts: set[str], navigation: set[str]) -> bool:
    tasks = inventory["tasks"]
    names = [row["name"] for row in tasks]
    files = [row["path"] for row in inventory["scripts"]]
    return (
        len(names) == len(set(names))
        and set(names) == declarations
        and {row["name"] for row in tasks if row["visibility"] == "public"} == public
        and {row["name"] for row in tasks if row["visibility"] == "internal"} == {"release"}
        and len(files) == len(set(files))
        and set(files) == scripts
        and navigation == public
        and all(set(row["owning_tasks"]) <= declarations for row in inventory["scripts"])
    )


def main() -> None:
    previous = manifest(PREVIOUS)
    subjects = set(previous)
    subjects.update(path.as_posix() for path in Path("scripts").rglob("*") if path.is_file())
    subjects.update({
        "about.toml", PREVIOUS.relative_to(ROOT).as_posix(),
        "docs/evidence/rhai-book-components-inventory-k128/README.md",
        *[(OUT / name).relative_to(ROOT).as_posix()
          for name in ("README.md", "inventory.json", "task-list.json", "verify.py")],
    })
    before = {path: digest(ROOT / path) for path in sorted(subjects)}
    (OUT / "subjects.sha256").write_text("".join(
        f"{value}  {path}\n" for path, value in before.items()
    ))
    inventory = read_json(OUT / "inventory.json")
    fresh = subprocess.run(
        ["mise", "exec", "--", "task", "--list-all", "--json"],
        check=True, capture_output=True, text=True,
    )
    fresh_tasks = json.loads(fresh.stdout)["tasks"]
    public = {row["name"] for row in fresh_tasks}
    captured = read_json(OUT / "task-list.json")["tasks"]
    fresh_locations = {row["name"]: row["location"]["line"] for row in fresh_tasks}
    assert {row["name"]: row["source"]["start"] for row in captured} == fresh_locations
    task_lines = (ROOT / "Taskfile.yml").read_text().splitlines()
    declarations = {line[2:-1] for line in task_lines if re.fullmatch(r"  [\w:-]+:", line)}
    filesystem = {path.as_posix() for path in Path("scripts").rglob("*") if path.is_file()}
    rg = subprocess.run(["rg", "--files", "scripts"], check=True, capture_output=True, text=True)
    assert set(rg.stdout.splitlines()) == filesystem
    book = (ROOT / "docs/book/src/10-develop-ravel.md").read_text()
    table = book.split("## Find a development task\n", 1)[1]
    navigation = {name for line in table.splitlines() if line.startswith("|")
                  for name in re.findall(r"`([\w:-]+)`", line)}
    assert check_membership(inventory, public, declarations, filesystem, navigation)
    controls = []
    for target in ("tasks", "scripts"):
        bad = copy.deepcopy(inventory)
        removed = bad[target].pop()
        rejected = not check_membership(bad, public, declarations, filesystem, navigation)
        assert rejected
        controls.append({"mutation": f"omit one {target} row", "removed": removed, "rejected": rejected})
    bad = copy.deepcopy(inventory)
    bad["tasks"].append(copy.deepcopy(bad["tasks"][0]))
    assert not check_membership(bad, public, declarations, filesystem, navigation)
    controls.append({"mutation": "duplicate task row", "rejected": True})
    bad_navigation = navigation - {"build"} | {"build_missing_control"}
    assert not check_membership(inventory, public, declarations, filesystem, bad_navigation)
    controls.append({"mutation": "change known book-navigation task", "rejected": True})
    write_json("membership-validation.json", {
        "public": sorted(public), "declarations": sorted(declarations),
        "scripts": sorted(filesystem), "book_navigation": sorted(navigation),
        "fresh_task_locations_match_capture": True, "filesystem_matches_rg": True,
        "all_rows_match": True, "controls": controls,
    })
    refs = references(inventory)
    assert all(valid_reference(ref) for ref in refs)
    wrong = [
        {"path": "Taskfile.yml", "start": 1, "end": len(task_lines) + 1},
        {"path": "docs/book/src/10-develop-ravel.md", "start": 0, "end": 1},
    ]
    assert all(not valid_reference(ref) for ref in wrong)
    write_json("reference-validation.json", {
        "references": [{**ref, "valid": True,
                        "first_line": (ROOT / ref["path"]).read_text().splitlines()[ref["start"] - 1]}
                       for ref in refs],
        "controls": [{**ref, "rejected": True} for ref in wrong],
    })
    checks = []
    commands = [
        ("ste", ["docs:ste", "--", "--strict", "docs/book/src/10-develop-ravel.md",
                 "docs/book/src/11-deliver-ravel.md"]),
        ("breaks", ["docs:breaks", "--", "--strict", "docs/book/src/10-develop-ravel.md",
                    "docs/book/src/11-deliver-ravel.md"]),
        ("excerpts", ["docs:excerpts"]),
        ("book-build", ["docs:book:build"]),
    ]
    for name, arguments in commands:
        command = ["mise", "exec", "--", "task", *arguments]
        started = datetime.now(UTC).isoformat()
        with (OUT / f"{name}.log").open("w") as log:
            result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=False)
        checks.append({"name": name, "command": command, "started": started,
                       "finished": datetime.now(UTC).isoformat(),
                       "exit_status": result.returncode, "log": f"{name}.log"})
    write_json("checks.json", checks)
    after = {path: digest(ROOT / path) for path in sorted(subjects)}
    (OUT / "subjects-after.sha256").write_text("".join(
        f"{value}  {path}\n" for path, value in after.items()
    ))
    changes = [{"path": path, "before": value, "after": after[path]}
               for path, value in before.items() if after[path] != value]
    write_json("digest-validation.json", {"subjects": len(subjects), "unchanged": not changes,
                                        "changed": changes})
    write_json("reconciliation.json", {
        "previous_manifest": PREVIOUS.relative_to(ROOT).as_posix(),
        "subjects": len(previous),
        "items": [{"path": path, "previous": old, "current": before[path],
                   "unchanged": old == before[path]} for path, old in previous.items()],
    })
    print(json.dumps({"tasks": len(public), "declarations": len(declarations),
                      "scripts": len(filesystem), "references": len(refs),
                      "subjects": len(subjects), "changed": changes,
                      "checks": checks}, indent=2))
    assert not changes
    assert all(check["exit_status"] == 0 for check in checks)


if __name__ == "__main__":
    main()
