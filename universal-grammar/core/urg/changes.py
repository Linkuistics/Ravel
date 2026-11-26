"""Summarise what a corrections run changed in the derived results, and what it reused.

The summary compares two pattern-discovery runs with the same settings and instructions.
The earlier run read the analyses before the corrections. The later run read the
corrected analyses. The stage also compares the reports of the two runs, which it builds
with the current report code, so that a change of report code does not appear as a change.

Pattern discovery calls a model. A new call can change patterns that no correction
touched. The stage therefore gives each pattern difference a cause:

- ``correction``: the difference involves an observation that the corrections affected.
- ``rerun``: the difference involves only observations that the corrections did not affect.
- ``either``: the text of a pattern changed, and the pattern cites an affected observation.

The rule is a heuristic. It cannot prove that a model would repeat a ``rerun`` difference.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from typing import Any

from urg.analysis import ANALYSIS
from urg.analysis import RECORD_FILE as ANALYSIS_FILE
from urg.corrections import REPORT_FILE as CORRECTIONS_FILE
from urg.corrections import RUN_REPORT as CORRECTIONS_RUN
from urg.discovery import DISCOVERY, ROLES
from urg.discovery import RECORD_FILE as DISCOVERY_FILE
from urg.errors import UrgError
from urg.report import DEFAULT_DPI, build_report
from urg.store import ArtifactStore, DependencyRecord, InputIdentity, StoredArtifact
from urg.version import code_version

Json = dict[str, Any]

STAGE = "change-summary"
SCHEMA_VERSION = 2
RECORD_FILE = "changes.json"
SUMMARY_FILE = "summary.md"
REFERENCE_FIELDS = ("episode", "source", "target", "alternative_to")
COMPARED_FIELDS = ("label", "description", "uncertainty", *REFERENCE_FIELDS)
PATTERN_FIELDS = (
    "name",
    "description",
    "basis",
    "books",
    "rules",
    "representation",
    "uncertainty",
)
OVERLAP = 0.3


class ChangeSummaryError(UrgError):
    """The change summary cannot be built."""


# --- Locating the runs ----------------------------------------------------------------


def _keys(store: ArtifactStore, name: str) -> list[str]:
    directory = store.directory_for(name, "x").parent
    if not directory.is_dir():
        return []
    return sorted(p.name for p in directory.iterdir() if p.is_dir() and p.name != "latest")


def _analysis_keys(discovery: Json) -> dict[str, str]:
    return {s["item_key"]: s["analysis_artifact"] for s in discovery["sources"]}


def _method(record: Json) -> list[tuple[str, Any]]:
    """Return the parts of a discovery run that must agree before and after the corrections.

    The settings, the full instruction identity and the model identity all count. The run
    attempt and the file paths do not, because they do not change what the model receives.
    """
    run = record["discovery_run"]
    return [
        ("method", run["method"]),
        ("settings", run["settings"]["content"]),
        ("instructions", (run["instructions"]["version"], run["instructions"]["sha256"])),
        ("model", run["model"]),
        ("preference weights", run["preference_weights"]),
    ]


def _incompatibilities(
    wanted: dict[str, str],
    before: StoredArtifact,
    before_record: Json,
    after: StoredArtifact,
    after_record: Json,
) -> list[str]:
    """Return why a run cannot be the run before the corrections. An empty list means it can."""
    reasons: list[str] = []
    if _analysis_keys(before_record) != wanted:
        reasons.append("It does not read the uncorrected analyses of the corrections run.")
    for (name, old), (_, new) in zip(_method(before_record), _method(after_record), strict=True):
        if old != new:
            reasons.append(f"Its {name} differ from the later run.")
    if before.record.dependency.code_version != after.record.dependency.code_version:
        reasons.append("Its code version differs from the later run.")
    return reasons


def _load_discovery(store: ArtifactStore, key: str) -> tuple[StoredArtifact, Json]:
    artifact = store.load(DISCOVERY, key)
    return artifact, json.loads(artifact.read_bytes(DISCOVERY_FILE))


def _base_keys(corrections: Json) -> dict[str, str]:
    return {
        r["item_key"]: r["base_analysis_key"]
        for r in corrections["results"]
        if r["status"] == "analysed"
    }


def check_before(
    corrections: Json,
    before: StoredArtifact,
    before_record: Json,
    after: StoredArtifact,
    after_record: Json,
) -> None:
    """Refuse a named earlier run that cannot be compared with the later run."""
    reasons = _incompatibilities(
        _base_keys(corrections), before, before_record, after, after_record
    )
    if reasons:
        raise ChangeSummaryError(
            f"pattern-discovery run {before.record.key} cannot be the run before the "
            f"corrections. {' '.join(reasons)}"
        )


def find_before(
    store: ArtifactStore, corrections: Json, after: StoredArtifact, after_record: Json
) -> StoredArtifact:
    """Find the one discovery run over the uncorrected analyses that matches the later run."""
    wanted = _base_keys(corrections)
    matches: list[StoredArtifact] = []
    for key in _keys(store, DISCOVERY):
        artifact, record = _load_discovery(store, key)
        if not _incompatibilities(wanted, artifact, record, after, after_record):
            matches.append(artifact)
    if not matches:
        raise ChangeSummaryError(
            "no pattern-discovery run reads the uncorrected analyses with the same settings, "
            "instructions and code. Run pattern-discovery before the corrections, or pass --before"
        )
    if len(matches) > 1:
        keys = ", ".join(m.record.key for m in matches)
        raise ChangeSummaryError(f"more than one earlier run matches ({keys}). Pass --before")
    return matches[0]


# --- Comparing interpretations ----------------------------------------------------------


def _ref_id(value: Any) -> str | None:
    if isinstance(value, dict):
        ref: Json = value  # pyright: ignore[reportUnknownVariableType]
        return str(ref["id"])
    return None


def _field_changes(old: Json, new: Json) -> list[Json]:
    changes: list[Json] = []
    for name in COMPARED_FIELDS:
        if old.get(name) != new.get(name):
            changes.append({"field": name, "before": old.get(name), "after": new.get(name)})
    old_evidence = [e["id"] for e in old["evidence"]]
    new_evidence = [e["id"] for e in new["evidence"]]
    if old_evidence != new_evidence:
        changes.append(
            {
                "field": "evidence",
                "added": [e for e in new_evidence if e not in old_evidence],
                "removed": [e for e in old_evidence if e not in new_evidence],
            }
        )
    return changes


@dataclass(frozen=True)
class Corrected:
    interpretations: list[Json]
    affected: set[str]


def _analysis(store: ArtifactStore, key: str) -> Json:
    return json.loads(store.load(ANALYSIS, key).read_bytes(ANALYSIS_FILE))


def corrected_interpretations(store: ArtifactStore, corrections: Json) -> Corrected:
    """Compare each corrected interpretation with its version in the base analysis.

    One run can apply several corrections to one interpretation. The comparison spans the
    whole interval, from the base analysis that the earlier discovery run read to the
    corrected analysis that the later run read. The entry lists every correction in that
    interval, in the order of application. An alternative reading has no base version, so
    each of its fields counts as added.

    The affected observations are the corrected interpretations, the interpretations that
    refer to them, and the evidence and references that the corrections added or removed.
    """
    entries: list[Json] = []
    affected: set[str] = set()
    for result in corrections["results"]:
        if not result.get("corrected"):
            continue
        record = _analysis(store, result["analysis_key"])
        base = {
            i["id"]: i for i in _analysis(store, result["base_analysis_key"])["interpretations"]
        }
        superseded: dict[str, list[Json]] = {}
        for old in record["history"]:
            superseded.setdefault(old["id"], []).append(old)
        changed: set[str] = set()
        for item in record["interpretations"]:
            if item.get("correction") is None:
                continue
            changed.add(item["id"])
            chain = [*sorted(superseded.get(item["id"], []), key=lambda v: v["version"]), item]
            applied = [
                v["correction"] | {"author": v["author"]} for v in chain if v.get("correction")
            ]
            old = base.get(item["id"])
            changes = _field_changes(old or {"evidence": []}, item)
            for change in changes:
                affected.update(change.get("added", []))
                affected.update(change.get("removed", []))
                if change["field"] not in REFERENCE_FIELDS:
                    continue
                for side in ("before", "after"):
                    ref = _ref_id(change.get(side))
                    if ref:
                        affected.add(ref)
            entries.append(
                {
                    "item_key": result["item_key"],
                    "id": item["id"],
                    "kind": item["kind"],
                    "corrections": applied,
                    "before_version": old["version"] if old else None,
                    "after_version": item["version"],
                    "changes": changes,
                }
            )
        affected |= changed
        for item in record["interpretations"]:
            if any(_ref_id(item.get(name)) in changed for name in REFERENCE_FIELDS):
                affected.add(item["id"])
    return Corrected(entries, affected)


# --- Comparing patterns -----------------------------------------------------------------


def _normalised(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", name.lower()).strip()


def _cited(pattern: Json, role: str | None = None) -> set[str]:
    roles = (role,) if role else ROLES
    return {c["observation"]["id"] for r in roles for c in pattern[r]}


def _overlap(a: Json, b: Json) -> float:
    x, y = _cited(a), _cited(b)
    return len(x & y) / len(x | y) if x | y else 0.0


def match_patterns(before: list[Json], after: list[Json]) -> list[tuple[Json, Json, str]]:
    """Pair the patterns of two runs: first by name, then by the overlap of citations."""
    pairs: list[tuple[Json, Json, str]] = []
    left = list(before)
    right = list(after)
    for old in list(left):
        same = [new for new in right if _normalised(new["name"]) == _normalised(old["name"])]
        if same:
            pairs.append((old, same[0], "name"))
            left.remove(old)
            right.remove(same[0])
    scored = sorted(
        ((_overlap(old, new), i, j) for i, old in enumerate(left) for j, new in enumerate(right)),
        reverse=True,
    )
    used_left: set[int] = set()
    used_right: set[int] = set()
    for score, i, j in scored:
        if score < OVERLAP or i in used_left or j in used_right:
            continue
        pairs.append((left[i], right[j], f"citations {score:.2f}"))
        used_left.add(i)
        used_right.add(j)
    return pairs


def _cause(ids: set[str], affected: set[str]) -> str:
    return "correction" if ids & affected else "rerun"


def _notes(pattern: Json, role: str) -> dict[str, str]:
    return {c["observation"]["id"]: c["note"] for c in pattern[role]}


def compare_patterns(before: list[Json], after: list[Json], affected: set[str]) -> Json:
    pairs = match_patterns(before, after)
    matched_before = {id(p[0]) for p in pairs}
    matched_after = {id(p[1]) for p in pairs}
    changed: list[Json] = []
    unchanged: list[Json] = []
    for old, new, how in pairs:
        differences: list[Json] = []
        for role in ROLES:
            added = sorted(_cited(new, role) - _cited(old, role))
            removed = sorted(_cited(old, role) - _cited(new, role))
            for obs in added:
                differences.append({"role": role, "added": obs, "cause": _cause({obs}, affected)})
            for obs in removed:
                differences.append({"role": role, "removed": obs, "cause": _cause({obs}, affected)})
            old_notes, new_notes = _notes(old, role), _notes(new, role)
            for obs in sorted(old_notes.keys() & new_notes.keys()):
                if old_notes[obs] != new_notes[obs]:
                    cause = "either" if obs in affected else "rerun"
                    differences.append({"role": role, "note": obs, "cause": cause})
        touches = bool((_cited(old) | _cited(new)) & affected)
        for name in PATTERN_FIELDS:
            if old[name] != new[name]:
                differences.append({"field": name, "cause": "either" if touches else "rerun"})
        entry = {
            "before": old["id"],
            "after": new["id"],
            "name": new["name"],
            "matched_by": how,
            "differences": differences,
        }
        (changed if differences else unchanged).append(entry)

    def lone(pattern: Json) -> Json:
        return {
            "id": pattern["id"],
            "name": pattern["name"],
            "cause": _cause(_cited(pattern), affected),
        }

    return {
        "matched_unchanged": unchanged,
        "matched_changed": changed,
        "dropped": [lone(p) for p in before if id(p) not in matched_before],
        "new": [lone(p) for p in after if id(p) not in matched_after],
    }


def compare_unfitted(before: list[Json], after: list[Json], affected: set[str]) -> Json:
    old = {u["id"] for u in before}
    new = {u["id"] for u in after}
    return {
        "now_unfitted": [{"id": i, "cause": _cause({i}, affected)} for i in sorted(new - old)],
        "now_fitted": [{"id": i, "cause": _cause({i}, affected)} for i in sorted(old - new)],
    }


# --- Comparing artifacts ----------------------------------------------------------------


def _row(name: str, before: str, after: str) -> Json:
    return {
        "artifact": name,
        "before": before,
        "after": after,
        "status": "reused" if before == after else "changed",
    }


def artifact_rows(store: ArtifactStore, before: Json, after: Json) -> list[Json]:
    rows: list[Json] = []
    old_sources = {s["item_key"]: s for s in before["sources"]}
    for source in after["sources"]:
        old = old_sources[source["item_key"]]
        rows.append(
            _row(
                f"chapter-extraction {source['item_key']}",
                old["extraction_artifact"],
                source["extraction_artifact"],
            )
        )
        old_analysis = json.loads(
            store.load(ANALYSIS, old["analysis_artifact"]).read_bytes(ANALYSIS_FILE)
        )
        new_analysis = json.loads(
            store.load(ANALYSIS, source["analysis_artifact"]).read_bytes(ANALYSIS_FILE)
        )
        rows.append(
            _row(
                f"episode-analysis-response {source['item_key']}",
                old_analysis["response_artifact"],
                new_analysis["response_artifact"],
            )
        )
        rows.append(
            _row(
                f"episode-analysis {source['item_key']}",
                old["analysis_artifact"],
                source["analysis_artifact"],
            )
        )
    rows.append(
        _row(
            "pattern-discovery-response",
            before["discovery_run"]["response_artifact"],
            after["discovery_run"]["response_artifact"],
        )
    )
    return rows


def report_files(before: StoredArtifact, after: StoredArtifact) -> Json:
    old = before.record.payload
    new = after.record.payload
    names = sorted(set(old) | set(new))
    return {
        "reused": [n for n in names if n in old and n in new and old[n] == new[n]],
        "changed": [n for n in names if n in old and n in new and old[n] != new[n]],
        "added": [n for n in names if n not in old],
        "removed": [n for n in names if n not in new],
    }


# --- Stage ------------------------------------------------------------------------------


def _identity(role: str, name: str, artifact: StoredArtifact) -> InputIdentity:
    key = artifact.record.key
    return InputIdentity(role=role, kind="artifact", locator=f"{name}/{key}", identity=key)


def change_dependency(
    corrections: StoredArtifact,
    before: StoredArtifact,
    after: StoredArtifact,
    reports: tuple[StoredArtifact, StoredArtifact],
) -> DependencyRecord:
    return DependencyRecord(
        stage=STAGE,
        artifact=STAGE,
        schema_version=SCHEMA_VERSION,
        code_version=code_version(__name__),
        configuration={"overlap": OVERLAP},
        inputs=[
            _identity("corrections-run", CORRECTIONS_RUN, corrections),
            _identity("before-discovery", DISCOVERY, before),
            _identity("after-discovery", DISCOVERY, after),
            _identity("before-report", "pattern-report", reports[0]),
            _identity("after-report", "pattern-report", reports[1]),
        ],
    )


def _cell(value: Any) -> str:
    return str(value).replace("|", "/").replace("\n", " ")


def render_summary(record: Json) -> str:
    lines = [
        "# Change summary",
        "",
        f"Corrections run `{record['corrections_run']}`.",
        f"Discovery before `{record['before']['discovery']}`, "
        f"after `{record['after']['discovery']}`.",
        f"Report before `{record['before']['report']}`, after `{record['after']['report']}`.",
        "",
        "## Corrected interpretations",
        "",
    ]
    for item in record["interpretations"]:
        start = item["before_version"] or "none (added)"
        lines.append(
            f"- `{item['id']}` ({item['kind']}) version {start} to {item['after_version']}"
        )
        for block in item["corrections"]:
            lines.append(
                f"  - correction `{block['id']}` by {block['author']['name']} "
                f"({block['origin']}): {block['rationale']}"
            )
        for change in item["changes"]:
            if change["field"] == "evidence":
                lines.append(
                    f"  - evidence added {change['added'] or 'none'}, "
                    f"removed {change['removed'] or 'none'}"
                )
            else:
                lines.append(f"  - {change['field']} changed")
    lines += ["", f"Affected observations: {', '.join(sorted(record['affected']))}.", ""]
    lines += [
        "## Artifacts",
        "",
        "| Artifact | Status | Before | After |",
        "| --- | --- | --- | --- |",
    ]
    for row in record["artifacts"]:
        lines.append(
            f"| {row['artifact']} | {row['status']} | `{row['before'][:12]}` | "
            f"`{row['after'][:12]}` |"
        )
    files = record["report_files"]
    lines += [
        "",
        f"Report files: {len(files['reused'])} reused, {len(files['changed'])} changed, "
        f"{len(files['added'])} added, {len(files['removed'])} removed.",
        f"Changed report files: {', '.join(files['changed']) or 'none'}.",
        "",
        "## Patterns",
        "",
    ]
    patterns = record["patterns"]
    lines.append(
        f"{len(patterns['matched_unchanged'])} patterns match with no difference, "
        f"{len(patterns['matched_changed'])} match with differences, "
        f"{len(patterns['dropped'])} disappear and {len(patterns['new'])} are new."
    )
    lines += [
        "",
        "| Before | After | Name | Difference | Cause |",
        "| --- | --- | --- | --- | --- |",
    ]
    for entry in patterns["matched_changed"]:
        for d in entry["differences"]:
            if "field" in d:
                what = f"{d['field']} field"
            elif "note" in d:
                what = f"{d['role']} note {d['note']}"
            else:
                what = f"{d['role']} {'+' if 'added' in d else '-'}{d.get('added') or d['removed']}"
            lines.append(
                f"| `{entry['before']}` | `{entry['after']}` | {_cell(entry['name'])} | "
                f"{what} | {d['cause']} |"
            )
    for p in patterns["dropped"]:
        lines.append(f"| `{p['id']}` | | {_cell(p['name'])} | disappears | {p['cause']} |")
    for p in patterns["new"]:
        lines.append(f"| | `{p['id']}` | {_cell(p['name'])} | new | {p['cause']} |")
    causes = record["causes"]
    unfit = record["unfitted"]
    lines += [
        "",
        f"Unfitted observations: {len(unfit['now_unfitted'])} newly unfitted, "
        f"{len(unfit['now_fitted'])} newly fitted.",
        "",
        "## Causes",
        "",
        f"{causes['correction']} differences involve an affected observation (correction). "
        f"{causes['rerun']} involve none (rerun). "
        f"{causes['either']} are field changes of patterns that cite an affected observation, "
        "or note changes on an affected observation (either).",
        "The attribution rule is a heuristic, and the module documentation states it.",
    ]
    return "\n".join(lines) + "\n"


def _count_causes(patterns: Json, unfit: Json) -> Json:
    counts = {"correction": 0, "rerun": 0, "either": 0}
    for entry in patterns["matched_changed"]:
        for d in entry["differences"]:
            counts[d["cause"]] += 1
    for p in [*patterns["dropped"], *patterns["new"]]:
        counts[p["cause"]] += 1
    for u in [*unfit["now_unfitted"], *unfit["now_fitted"]]:
        counts[u["cause"]] += 1
    return counts


@dataclass(frozen=True)
class ChangeRun:
    summary: StoredArtifact
    reused: bool
    record: Json


def summarise_changes(
    store: ArtifactStore,
    before_key: str | None = None,
    after_key: str | None = None,
    dpi: int = DEFAULT_DPI,
) -> ChangeRun:
    """Compare the discovery runs before and after the latest corrections run."""
    corrections_key = store.latest_key(CORRECTIONS_RUN)
    if corrections_key is None:
        raise ChangeSummaryError("no corrections run is in the store. Run corrections first")
    corrections_run = store.load(CORRECTIONS_RUN, corrections_key)
    corrections: Json = json.loads(corrections_run.read_bytes(CORRECTIONS_FILE))
    after_key = after_key or store.latest_key(DISCOVERY)
    if after_key is None:
        raise ChangeSummaryError("no pattern-discovery run is in the store")
    after, after_record = _load_discovery(store, after_key)
    wanted = {
        r["item_key"]: r["analysis_key"]
        for r in corrections["results"]
        if r["status"] == "analysed"
    }
    if _analysis_keys(after_record) != wanted:
        raise ChangeSummaryError(
            f"pattern-discovery run {after_key} does not read the corrected analyses. "
            "Run pattern-discovery after the corrections"
        )
    if before_key is None:
        before = find_before(store, corrections, after, after_record)
    else:
        before = store.load(DISCOVERY, before_key)
    before_record: Json = json.loads(before.read_bytes(DISCOVERY_FILE))
    if before.record.key == after.record.key:
        raise ChangeSummaryError("the runs before and after the corrections are the same run")
    check_before(corrections, before, before_record, after, after_record)
    reports = (build_report(store, before, dpi).report, build_report(store, after, dpi).report)

    dependency = change_dependency(corrections_run, before, after, reports)
    found = store.reuse(dependency)
    if found is not None:
        return ChangeRun(found, True, json.loads(found.read_bytes(RECORD_FILE)))

    corrected = corrected_interpretations(store, corrections)
    patterns = compare_patterns(
        before_record["patterns"], after_record["patterns"], corrected.affected
    )
    unfit = compare_unfitted(
        before_record["unfitted"], after_record["unfitted"], corrected.affected
    )
    record: Json = {
        "schema_version": SCHEMA_VERSION,
        "stage": STAGE,
        "corrections_run": corrections_key,
        "before": {"discovery": before.record.key, "report": reports[0].record.key},
        "after": {"discovery": after.record.key, "report": reports[1].record.key},
        "interpretations": corrected.interpretations,
        "affected": sorted(corrected.affected),
        "artifacts": [
            *artifact_rows(store, before_record, after_record),
            _row("pattern-discovery", before.record.key, after.record.key),
            _row("pattern-report", reports[0].record.key, reports[1].record.key),
        ],
        "report_files": report_files(*reports),
        "patterns": patterns,
        "unfitted": unfit,
        "causes": _count_causes(patterns, unfit),
    }
    stored = store.write(
        dependency,
        {
            RECORD_FILE: (json.dumps(record, indent=2) + "\n").encode(),
            SUMMARY_FILE: render_summary(record).encode(),
        },
    )
    return ChangeRun(stored, False, record)
