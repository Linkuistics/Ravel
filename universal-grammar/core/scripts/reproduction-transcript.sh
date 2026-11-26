#!/usr/bin/env bash
# Record a run of the whole pipeline from a clean data directory. The milestone completion
# report, docs/milestones/vertical-slice-completion.md, cites this transcript.
#
# Usage: scripts/reproduction-transcript.sh RUN_DIR > docs/milestones/vertical-slice-reproduction.txt
#
# RUN_DIR must not exist. The store is RUN_DIR/data. A shim named claude comes first on PATH
# and logs each model call. In R1 it passes each call to the real claude. In R2 it refuses each
# call, so a count of zero shows that the restart made no model call.
#
# The script checks each outcome and exits 1 when one check fails. The checks are these:
#   - Each stage exits 0. The one accepted exception is the corrections stage when each of its
#     refusals names an interpretation that no analysed book holds. That refusal is the expected
#     result of a correction whose target belongs to another store.
#   - The restart in R2 makes no model call and leaves the store unchanged.
#   - The SHA-256 inventory of the run's subjects is the same before R1 and after R2.
# The final section lists each failed check.
set -uo pipefail

work=${1:?usage: $0 RUN_DIR}
[ -e "$work" ] && { echo "$work exists. The run needs a clean directory." >&2; exit 2; }
mkdir -p "$work"
work=$(cd "$work" && pwd)
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo" || exit 2
python=.venv/bin/python
calls="$work/model-calls.log"
real_claude=$(command -v claude)
store="$work/data"

mkdir -p "$work/bin"
cat > "$work/bin/claude" <<EOF
#!/bin/sh
# One line per call. A refused call counts even when it is only the version check, because a
# stage checks the version only when it needs the model.
if [ -f "$work/refuse" ]; then
  echo "refused" >> "$calls"
  echo "model call refused by the reproduction transcript" >&2
  exit 1
fi
[ "\$1" = "--version" ] || echo "called" >> "$calls"
exec "$real_claude" "\$@"
EOF
chmod +x "$work/bin/claude"
export PATH="$work/bin:$PATH"
: > "$calls"

hide() { sed -e "s|$work|\$RUN|g" -e "s|$HOME|~|g"; }
# Run a command. Show its output and its exit status, and keep its output in $work/last.
run() {
  printf '\n$ %s\n' "$*" | hide
  "$@" 2>&1 | hide | tee "$work/last"
  local status=${PIPESTATUS[0]}
  printf '[exit %s]\n' "$status"
  return "$status"
}
failures=()
fail() { failures+=("$*"); printf 'CHECK FAILED: %s\n' "$*"; }
section() { printf '\n\n== %s\n' "$*"; }
listing() {
  if [ -d "$1/artifacts" ]; then "$python" scripts/transcript_store.py listing "$1"; fi
}
# Print the count of model calls in a section, then clear the log. The count goes to $work/count.
model_calls() {
  wc -l < "$calls" | tr -d ' ' > "$work/count"
  printf 'model calls in this section: %s\n' "$(cat "$work/count")"
  : > "$calls"
}
# The run's subjects: each path that differs from the commit, each correction file, this script,
# its listing helper and the task file. Each line holds a SHA-256 and a path.
inventory() {
  {
    git status --porcelain --untracked-files=all | cut -c4- | sed 's/.* -> //'
    ls corrections/*
    printf '%s\n' scripts/reproduction-transcript.sh scripts/transcript_store.py Taskfile.yml
  } | sort -u | while read -r path; do
    if [ -f "$path" ]; then shasum -a 256 "$path"; else printf 'absent  %s\n' "$path"; fi
  done
}
# Accept a corrections failure only when each refusal names an interpretation absent from the store.
stale_refusals_only() {
  grep -q '^refused ' "$work/last" &&
    ! grep '^refused ' "$work/last" | grep -qv ': no analysed book holds interpretation '
}
# Run the stages of `task pipeline` that follow a failed corrections stage, and check each one.
run_after_corrections() {
  local task
  for task in "${after_corrections[@]}"; do
    run task "$task" DATA_DIR="$store" || fail "$1: task $task exited with a failure"
  done
}
# Run `task pipeline` and check its outcome. The argument names the section.
pipeline() {
  if run task pipeline DATA_DIR="$store"; then return; fi
  if grep -q 'Failed to run task "corrections"' "$work/last" && stale_refusals_only; then
    printf '\nThe corrections stage refused only corrections whose targets are absent from this store.\n'
    printf 'This refusal is expected. The later stages run one at a time:\n'
    run_after_corrections "$1"
  else
    fail "$1: task pipeline failed, and the failure is not the expected corrections refusal"
  fi
}
new_artifacts() {
  printf 'new artifacts, by name:\n'
  comm -13 "$1" "$2" | awk '{print "  " $1}' | sort | uniq -c
  printf 'artifacts in the store before: %s, after: %s\n' "$(wc -l < "$1" | tr -d ' ')" "$(wc -l < "$2" | tr -d ' ')"
}
# The stages that `task pipeline` runs after the corrections stage. `task pipeline` stops at the
# first failure, so these run one at a time when the corrections stage fails.
after_corrections=(pattern-discovery weighted-report)

section "R0 Context"
printf 'commit: %s\n' "$(git rev-parse HEAD)"
printf 'working copy differs from the commit in: %s\n' "$(git status --porcelain | wc -l | tr -d ' ') paths"
inventory > "$work/inventory-before"
printf 'SHA-256 inventory of the subjects before R1:\n'
sed 's/^/  /' "$work/inventory-before"
printf 'store: $RUN/data, which does not exist before R1\n'
printf 'claude: %s\n' "$("$real_claude" --version 2>&1)"
printf 'correction files: %s\n' "$(ls corrections | tr '\n' ' ')"
: > "$work/L0"

section "R1 Run every stage from the clean data directory, with the real model"
rm -f "$work/refuse"
pipeline R1
listing "$store" > "$work/L1"
new_artifacts "$work/L0" "$work/L1"
model_calls

section "R2 Restart: run every stage again, with each model call refused"
touch "$work/refuse"
pipeline R2
listing "$store" > "$work/L2"
new_artifacts "$work/L1" "$work/L2"
if cmp -s "$work/L1" "$work/L2"; then echo "the store is unchanged"; else fail "R2: the store changed"; fi
model_calls
[ "$(cat "$work/count")" = 0 ] || fail "R2: the restart made $(cat "$work/count") model calls"

section "R3 Outcome"
inventory > "$work/inventory-after"
if cmp -s "$work/inventory-before" "$work/inventory-after"; then
  echo "the SHA-256 inventory of the subjects is unchanged after R2"
else
  fail "the SHA-256 inventory of the subjects changed during the run"
  diff "$work/inventory-before" "$work/inventory-after" | sed 's/^/  /'
fi
if [ ${#failures[@]} -eq 0 ]; then
  echo "every check passed"
  exit 0
fi
printf 'failed checks: %s\n' "${#failures[@]}"
printf '  %s\n' "${failures[@]}"
exit 1
