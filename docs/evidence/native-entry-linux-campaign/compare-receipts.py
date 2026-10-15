#!/usr/bin/env python3
"""Item-by-item comparison of campaign receipts against k43's immutable Darwin
receipts (docs/evidence/native-entry-campaign).

  python3 -I compare-receipts.py K43_DIR TARGET_DIR...

Each TARGET_DIR is a campaign directory (receipts/, rows/, tests.json). Free text
(`note`, pids, temporary paths) and the order of lists are not compared; every
other field is, so identity, subjects, verdicts, baseline class, status and a
platform reason all are. A difference is printed with the fields that moved."""
import glob, json, os, sys

SKIP = {"note", "pid", "limit"}


def norm(o):
    if isinstance(o, dict):
        return {k: norm(v) for k, v in o.items() if k not in SKIP}
    if isinstance(o, list):
        return sorted((norm(x) for x in o), key=lambda x: json.dumps(x, sort_keys=True))
    return o


def load(d, sub):
    return {os.path.basename(p): json.load(open(p)) for p in sorted(glob.glob(f"{d}/{sub}/*.json"))}


def main():
    k43 = sys.argv[1]
    for target in sys.argv[2:]:
        print(f"== {target}")
        for sub in ("receipts", "rows"):
            a, b = load(k43, sub), load(target, sub)
            names = sorted(set(a) | set(b))
            moved = [n for n in names if norm(a.get(n)) != norm(b.get(n))]
            print(f"{sub}: k43 {len(a)}, target {len(b)}, same names {set(a) == set(b)}, differing {len(moved)}")
            if sub != "receipts":
                continue
            for n in moved:
                x, y = norm(a.get(n)) or {}, norm(b.get(n)) or {}
                fields = [k for k in sorted(set(x) | set(y)) if x.get(k) != y.get(k)]
                status = (b.get(n) or {}).get("status")
                print(f"  {n[:-5]}: {','.join(fields)} (status {status})")


main()
