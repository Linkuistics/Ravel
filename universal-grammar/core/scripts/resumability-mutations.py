"""Break the resumability mechanism on purpose and show which test catches each break.

Usage: .venv/bin/python scripts/resumability-mutations.py [ID ...] > docs/resumability-mutations.txt

Each mutation replaces one exact passage of the source. The script runs the resumability
tests, restores the file and records each failing test. A mutation that no test catches
prints "SURVIVED". After the last mutation the script checks that urg/ matches the commit
and runs the tests on the restored source.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path
from typing import NamedTuple

REPO = Path(__file__).resolve().parent.parent
TESTS = ["tests/test_resumability.py"]


class Mutation(NamedTuple):
    id: str
    claim: str
    description: str
    path: str
    old: str
    new: str


MUTATIONS = [
    Mutation(
        "M1",
        "2: a return to earlier inputs reuses their results",
        "reuse becomes find in the store: a reused output does not become the latest",
        "urg/store.py",
        "            self._set_latest(found.record.dependency.artifact, found.record.key)\n"
        "        return found",
        "            pass\n        return found",
    ),
    Mutation(
        "M1b",
        "2: a return to earlier inputs reuses their results",
        "reuse becomes find at one call: the analysis run report",
        "urg/analysis.py",
        "    )\n    found = store.reuse(dependency)\n"
        "    if found is not None:\n        return found\n",
        "    )\n    found = store.find(dependency)\n"
        "    if found is not None:\n        return found\n",
    ),
    Mutation(
        "M2",
        "4: the store rejects a damaged output",
        "load skips the size and hash check of each payload file",
        "urg/store.py",
        "if len(data) != digest.size or hashlib.sha256(data).hexdigest() != digest.sha256:",
        "if False:",
    ),
    Mutation(
        "M3",
        "1: each dependency record names each input",
        "the corrected analysis omits its correction files",
        "urg/corrections.py",
        '            _artifact_identity("base-analysis", ANALYSIS, book.base),\n'
        "            *(f.identity() for f in book.files),",
        '            _artifact_identity("base-analysis", ANALYSIS, book.base),',
    ),
    Mutation(
        "M3b",
        "1: each dependency record names each input",
        "a chapter extraction omits its chapter-selection entry",
        "urg/extraction.py",
        "            InputIdentity(\n"
        '                role="chapter-selection-entry",\n'
        '                kind="file",\n'
        "                locator=f\"chapter-selection#{identity['item_key']}\",\n"
        "                identity=_sha256(canonical_json(entry)),\n"
        "            ),\n",
        "",
    ),
    Mutation(
        "M3c",
        "1: each dependency record names each input",
        "the analysis run report omits its instructions",
        "urg/analysis.py",
        '        configuration={"instructions_version": instructions.version, '
        '"results": results},\n'
        "        inputs=[\n"
        "            instructions.identity(),\n",
        '        configuration={"instructions_version": instructions.version, '
        '"results": results},\n'
        "        inputs=[\n",
    ),
    Mutation(
        "M4",
        "1: each dependency record names each input",
        "the code version omits the hash of the module closure",
        "urg/version.py",
        'return f"{PACKAGE} {version(PACKAGE)} source-sha256:{digest.hexdigest()}"',
        'return f"{PACKAGE} {version(PACKAGE)}"',
    ),
    Mutation(
        "M5",
        "2 and 3: a restart and a rerun reuse valid work",
        "find never finds a stored output",
        "urg/store.py",
        "        record_dependency = validate_dependency(dependency)\n"
        "        directory = self.directory_for(",
        "        record_dependency = validate_dependency(dependency)\n"
        "        return None\n"
        "        directory = self.directory_for(",
    ),
    Mutation(
        "M6",
        "6: a weight change reuses extraction, analysis and discovery",
        "the baseline report also depends on the weights file",
        "urg/weighted_report.py",
        "        inputs=[*upstream, relevance_input],",
        "        inputs=[*upstream, relevance_input,"
        ' _file_identity("preference-weights", weights_path, weights_bytes)],',
    ),
]


def pytest() -> subprocess.CompletedProcess[str]:
    command = ["uv", "run", "pytest", "-p", "no:cacheprovider", *TESTS]
    return subprocess.run(command, cwd=REPO, capture_output=True, text=True, check=False)


def outcome(result: subprocess.CompletedProcess[str]) -> str:
    lines = re.findall(r"^=* ?(\d+ (?:passed|failed).*?) ?=*$", result.stdout, re.M)
    return lines[-1] if lines else f"no summary line, exit {result.returncode}"


def main(selected: list[str]) -> int:
    survivors = 0
    for mutation in MUTATIONS:
        if selected and mutation.id not in selected:
            continue
        path = REPO / mutation.path
        original = path.read_text()
        assert original.count(mutation.old) == 1, f"{mutation.id}: the passage is not unique"
        path.write_text(original.replace(mutation.old, mutation.new))
        try:
            result = pytest()
        finally:
            path.write_text(original)
        failed = sorted(set(re.findall(r"^(?:FAILED|ERROR) \S+::(\S+)", result.stdout, re.M)))
        print(f"## {mutation.id} {mutation.description}")
        print(f"claim: {mutation.claim}")
        print(f"file: {mutation.path}")
        print(f"tests: {outcome(result)}")
        for name in failed:
            print(f"  fails: {name}")
        if not failed:
            survivors += 1
            print("  SURVIVED: no test fails")
        print(flush=True)
    status = subprocess.run(
        ["git", "status", "--porcelain", "urg"],
        cwd=REPO,
        capture_output=True,
        text=True,
        check=True,
    )
    changed = len(status.stdout.splitlines())
    print(f"urg/ after restoration, paths that differ from the commit: {changed}")
    print(f"tests on the restored source: {outcome(pytest())}")
    print(f"mutations that survived: {survivors}")
    return 1 if survivors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
