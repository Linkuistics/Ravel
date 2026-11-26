"""Read and damage a research-artifact store for the resumability transcript.

Usage: ``python scripts/transcript_store.py COMMAND DATA_DIR [ARGUMENT]``.

- ``listing``: print one line per stored artifact, as ``name key stage``.
- ``records``: print each distinct shape of dependency record, by artifact name and stage.
- ``corrupt``: flip one byte in the analysis payload of one analysis of the latest run.
- ``corrected``: print each book of the latest corrections run, with the analysis that
  discovery reads and the corrections that apply to it.
"""

from __future__ import annotations

import json
import sys
from collections import Counter
from pathlib import Path

from urg import analysis, corrections
from urg.store import ArtifactStore, InterruptedWrite, StoredArtifact


def all_artifacts(store: ArtifactStore) -> list[StoredArtifact]:
    found: list[StoredArtifact] = []
    for directory in sorted((store.root / "artifacts").glob("*/*")):
        if directory.is_dir():
            found.append(store.load(directory.parent.name, directory.name))
    return found


def listing(store: ArtifactStore) -> None:
    """Print each stored artifact by directory, with its stage. Mark each rejected artifact.

    Two stages write an ``episode-analysis``: the analysis stage and the corrections stage.
    The stage tells them apart.
    """
    for directory in sorted((store.root / "artifacts").glob("*/*")):
        if not directory.is_dir():
            continue
        try:
            artifact = store.load(directory.parent.name, directory.name)
        except InterruptedWrite:
            print(directory.parent.name, directory.name, "rejected")
        else:
            print(directory.parent.name, directory.name, artifact.record.dependency.stage)


def records(store: ArtifactStore) -> None:
    shapes: Counter[str] = Counter()
    for artifact in all_artifacts(store):
        dependency = artifact.record.dependency
        roles = Counter(i.role for i in dependency.inputs)
        shape = (
            f"{dependency.artifact} (stage {dependency.stage}, "
            f"schema {dependency.schema_version})\n"
            f"    code: {dependency.code_version.split(' source-sha256:')[0]} + source hash\n"
            f"    configuration: {', '.join(sorted(dependency.configuration))}\n"
            f"    inputs: {', '.join(f'{r} x{n}' for r, n in sorted(roles.items()))}"
        )
        shapes[shape] += 1
    for shape, count in sorted(shapes.items()):
        print(f"{shape}\n    artifacts with this shape: {count}")


def latest_analyses(store: ArtifactStore) -> list[StoredArtifact]:
    """Return the proposed analyses that the latest analysis run names."""
    run_key = store.latest_key(analysis.RUN_REPORT)
    assert run_key is not None, "no analysis run"
    text = "".join(
        p.read_text(errors="replace")
        for p in store.directory_for(analysis.RUN_REPORT, run_key).iterdir()
    )
    return [
        a
        for a in all_artifacts(store)
        if a.record.dependency.artifact == analysis.ANALYSIS
        and a.record.dependency.stage == analysis.STAGE
        and a.record.key in text
    ]


def corrupt(store: ArtifactStore) -> None:
    victim = latest_analyses(store)[0]
    item_key = json.loads(victim.read_bytes(analysis.RECORD_FILE))["source"]["item_key"]
    path = store.directory_for(analysis.ANALYSIS, victim.record.key) / analysis.RECORD_FILE
    data = bytearray(path.read_bytes())
    data[len(data) // 2] ^= 0x01
    path.write_bytes(bytes(data))
    print(f"flipped one byte of {analysis.ANALYSIS}/{victim.record.key} (book {item_key})")


def corrected(store: ArtifactStore) -> None:
    run_key = store.latest_key(corrections.RUN_REPORT)
    assert run_key is not None, "no corrections run"
    report = json.loads(
        store.load(corrections.RUN_REPORT, run_key).read_bytes(corrections.REPORT_FILE)
    )
    print(f"latest corrections run: {run_key}")
    for outcome in report["corrections"]:
        target = outcome["target"]["id"] if outcome["target"] else "-"
        print(f"  correction {outcome['id']}: {outcome['status']}, target {target}")
    for result in report["results"]:
        if result["status"] != "analysed":
            continue
        state = "corrected analysis" if result["corrected"] else "analysis unchanged"
        print(f"  book {result['item_key']}: {state}, discovery reads {result['analysis_key']}")


def main(argv: list[str]) -> None:
    command, data_dir = argv
    store = ArtifactStore(Path(data_dir))
    if command == "listing":
        listing(store)
    elif command == "records":
        records(store)
    elif command == "corrupt":
        corrupt(store)
    elif command == "corrected":
        corrected(store)
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
