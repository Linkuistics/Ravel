#!/usr/bin/env bash
# Record a real-data run that shows how the pipeline resumes. docs/resumability.md maps each
# section to one claim.
#
# Usage: scripts/resumability-transcript.sh WORK_DIR > docs/resumability-transcript.txt
#
# S1 to S3 use the real store in data/ without any correction. S4 to S6 use copy-on-write clones
# of it in WORK_DIR. S7 and S8 apply the correction files in corrections/ to data/. A shim named
# claude comes first on PATH and logs each model call. In S1 and S7 it passes each call to the
# real claude. In every other section it refuses each call. A count of zero shows that a section
# made no model call. Each section stops the script when a stage fails.
set -uo pipefail

work=$(cd "${1:?usage: $0 WORK_DIR}" && pwd)
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"
python=.venv/bin/python
calls="$work/model-calls.log"
real_claude=$(command -v claude)

mkdir -p "$work/bin"
cat > "$work/bin/claude" <<EOF
#!/bin/sh
# One line per call. A refused call counts even when it is only the version check, because a
# stage checks the version only when it needs the model.
if [ -f "$work/refuse" ]; then
  echo "refused" >> "$calls"
  echo "model call refused by the resumability transcript" >&2
  exit 1
fi
[ "\$1" = "--version" ] || echo "called" >> "$calls"
exec "$real_claude" "\$@"
EOF
chmod +x "$work/bin/claude"
export PATH="$work/bin:$PATH"
: > "$calls"

hide() { sed -e "s|$work|\$WORK|g" -e "s|$HOME|~|g"; }
run() {
  printf '\n$ %s\n' "$*" | hide
  "$@" 2>&1 | hide
  local status=${PIPESTATUS[0]}
  printf '[exit %s]\n' "$status"
  return "$status"
}
section() { printf '\n\n== %s\n' "$*"; }
listing() { "$python" scripts/transcript_store.py listing "$1"; }
model_calls() { printf 'model calls in this section: %s\n' "$(wc -l < "$calls" | tr -d ' ')"; : > "$calls"; }
new_artifacts() {
  printf 'new artifacts, by name:\n'
  comm -13 "$1" "$2" | awk '{print "  " $1 " from stage " $3}' | sort | uniq -c
  printf 'artifacts in the store before: %s, after: %s\n' "$(wc -l < "$1" | tr -d ' ')" "$(wc -l < "$2" | tr -d ' ')"
}
clone() { rm -rf "$work/$1" && cp -cR data "$work/$1"; }
# The sections before S7 run without corrections, so that S7 adds the first one.
no_corrections="$work/no-corrections"
rm -rf "$no_corrections" && mkdir -p "$no_corrections"
# Run `task pipeline` on the store in $1 with the corrections in $2. Stop the script if it fails.
pipeline() {
  run task pipeline DATA_DIR="$1" CORRECTIONS_DIR="$2" || { echo "task pipeline failed, so the transcript stops"; exit 1; }
}

section "S0 Context"
printf 'commit: %s\n' "$(git rev-parse --short HEAD)"
printf 'working copy differs from the commit in: %s\n' "$(git status --porcelain | wc -l | tr -d ' ') paths"
listing data > "$work/L0"
printf 'artifacts in data/: %s\n' "$(wc -l < "$work/L0" | tr -d ' ')"

section "S1 Bring data/ up to date with the real model, without corrections"
printf 'This run makes any model call that the current code needs.\n'
rm -f "$work/refuse"
pipeline data "$no_corrections"
touch "$work/refuse"
listing data > "$work/L1"
new_artifacts "$work/L0" "$work/L1"
model_calls

section "S2 Claim 2: restart after the complete run, with each model call refused"
pipeline data "$no_corrections"
listing data > "$work/L2"
new_artifacts "$work/L1" "$work/L2"
if cmp -s "$work/L1" "$work/L2"; then echo "the store is unchanged"; else echo "THE STORE CHANGED"; fi
model_calls

section "S3 Claim 1: the dependency records in the store, by shape"
run "$python" scripts/transcript_store.py records data

section "S4 Claim 3: in a clone without extractions, kill chapter-extraction before the second chapter is renamed into place"
clone kill
rm -rf "$work/kill/artifacts/chapter-extraction" "$work/kill/artifacts/chapter-extraction-run" "$work/kill/staging"
listing "$work/kill" > "$work/L4a"
run "$python" -m tests.pipeline kill before-rename chapter-extraction@2 -- \
  chapter-extraction --selection docs/probes/chapter-selection.json --data-dir "$work/kill"
listing "$work/kill" > "$work/L4b"
new_artifacts "$work/L4a" "$work/L4b"
printf 'staging directories, and whether each holds a record:\n'
for d in "$work"/kill/staging/*/; do
  [ -d "$d" ] && printf '  %s record.json=%s\n' "$(basename "$d")" "$([ -f "$d/record.json" ] && echo yes || echo no)"
done
printf 'The store reads artifacts/ only. A directory in staging/ is never an output.\n'
printf 'Rerun the pipeline on the clone:\n'
pipeline "$work/kill" "$no_corrections"
listing "$work/kill" > "$work/L4c"
new_artifacts "$work/L4b" "$work/L4c"
printf 'artifacts in the clone that are not in data/: %s\n' "$(comm -13 "$work/L2" "$work/L4c" | wc -l | tr -d ' ')"
model_calls

clone damage
clone weights

section "S5 Claim 4: damage one stored analysis in a clone, then rerun the pipeline on the clone"
run "$python" scripts/transcript_store.py corrupt "$work/damage"
listing "$work/damage" > "$work/L5a"
printf 'artifacts that the store rejects before the rerun:\n'
grep rejected "$work/L5a" | sed 's/^/  /'
pipeline "$work/damage" "$no_corrections"
listing "$work/damage" > "$work/L5b"
new_artifacts "$work/L5a" "$work/L5b"
printf 'artifacts that the store rejects after the rerun: %s\n' "$(grep -c rejected "$work/L5b")"
printf 'damaged directories moved aside: %s\n' "$(find "$work/damage/staging" -maxdepth 1 -name 'damaged-*' | wc -l | tr -d ' ')"
model_calls

section "S6 Claim 6: change one book weight in a clone, then rerun the pipeline on the clone"
cp docs/weights/illustrative-weights.json "$work/weights.json"
"$python" - "$work/weights.json" <<'EOF'
import json, sys
path = sys.argv[1]
weights = json.load(open(path))
weights["default_book_weight"] = 1.5
json.dump(weights, open(path, "w"), indent=2)
print("default_book_weight: 1.0 -> 1.5")
EOF
listing "$work/weights" > "$work/L6a"
for task in catalogue-manifest chapter-extraction episode-analysis corrections pattern-discovery; do
  run task "$task" DATA_DIR="$work/weights" CORRECTIONS_DIR="$no_corrections" || exit 1
done
run uv run urg weighted-report --weights "$work/weights.json" --data-dir "$work/weights" || exit 1
listing "$work/weights" > "$work/L6b"
new_artifacts "$work/L6a" "$work/L6b"
model_calls

section "S7 Claim 5: add the first correction to data/, then run the pipeline with the real model"
printf 'corrections/ holds these files:\n'
ls corrections | sed 's/^/  /'
listing data > "$work/L7a"
rm -f "$work/refuse"
pipeline data corrections
touch "$work/refuse"
listing data > "$work/L7b"
new_artifacts "$work/L7a" "$work/L7b"
run "$python" scripts/transcript_store.py corrected data
model_calls

section "S8 Claim 2: restart after the corrected run, with each model call refused"
pipeline data corrections
listing data > "$work/L8"
new_artifacts "$work/L7b" "$work/L8"
if cmp -s "$work/L7b" "$work/L8"; then echo "the store is unchanged"; else echo "THE STORE CHANGED"; fi
model_calls
