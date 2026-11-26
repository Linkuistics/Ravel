#!/usr/bin/env bash
# The controls of `doubt-composition.sh`.
#
# Usage: doubt-composition.test.sh <doubt SKILL.md> <Grove plugins/grove/skills>
#
# A control that nobody has seen fail is not a control. For each assertion of
# the check, this script mutates a copy of the skill and, separately, a copy of
# the Grove file that holds the assertion's Grove phrase. Each mutant must fail
# the check on that assertion alone. The unchanged copies must pass. A mutation
# that does not change its copy fails the control, so a stale mutation cannot
# pass as a reading.
#
# A mutant is the normalised text of its file, with runs of white space
# collapsed to one space. The check normalises the same way, so the
# normalisation alone changes no reading. The baseline run proves that.

set -euo pipefail

[[ $# -eq 2 ]] || {
  echo "usage: doubt-composition.test.sh <doubt SKILL.md> <Grove plugins/grove/skills>" >&2
  exit 2
}
skill_file=$1
grove_skills=$2
check="$(cd "$(dirname "$0")" && pwd)/doubt-composition.sh"

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

# Each control: assertion, side, file, operation, text, replacement.
# The side is `skill` or `grove`. A grove file is relative to the skills
# directory. The operation `append` adds the text at the end of the file, and
# `replace` replaces every occurrence of the text.
controls=(
  "no_deleted_composition_verb|skill|-|append|Run leaf-add-chain.|"
  "no_deleted_composition_verb|grove|grove/references/decompose.md|replace|--kind review-<producer>|--kind review"
  "no_withdrawn_adjacency_exception|skill|-|append|A leaf that touches no file the findings cite can go anywhere.|"
  "no_withdrawn_adjacency_exception|grove|grove/references/decompose.md|replace|whose subtree still holds live work|whose subtree holds work"
  "yields_to_grove_review_allowance|skill|-|replace|Substantial redesign becomes|Substantial redesign is"
  "yields_to_grove_review_allowance|grove|grove/references/execute.md|replace|across the whole picked leaf|across each artifact"
  "handoff_retires_before_commit|skill|-|replace|retire the producer, then|"
  "handoff_retires_before_commit|grove|grove/SKILL.md|replace|before committing|after committing"
  "no_receipt_routing|skill|-|append|Wait for the handoff receipt.|"
  "no_receipt_routing|grove|grove/references/decompose.md|replace|compares nothing, and warns about nothing|compares targets, and warns about drift"
  "mandate_ownership_predicate|skill|-|append|Ownership follows Bootstrap-and-pick.|"
  "mandate_ownership_predicate|grove|grove/references/execute.md|replace|adopted that mandate by running Bootstrap|adopted that mandate by running pick"
  "names_research_kinds|skill|-|replace|combine-research|combine-survey"
  "names_research_kinds|grove|grove/references/execute.md|replace|spend **none**|spend **one**"
)

passed=0
failed=0

pass() {
  passed=$((passed + 1))
  echo "ok   $1"
}

fail() {
  failed=$((failed + 1))
  echo "FAIL $1"
}

# Make fresh copies of both sides under the scratch directory.
fresh_copies() {
  rm -rf "$scratch/run"
  mkdir -p "$scratch/run/skills"
  cp "$skill_file" "$scratch/run/SKILL.md"
  cp -R "$grove_skills/grove" "$scratch/run/skills/grove"
}

# Mutate one file. Print nothing and return 1 when the mutation changes nothing.
mutate() {
  local file=$1 operation=$2 text=$3 replacement=$4 before after
  before=$(tr -s '[:space:]' ' ' <"$file")
  case $operation in
    append) after="$before $text" ;;
    replace) after=${before//"$text"/"$replacement"} ;;
    *)
      echo "error: unknown operation $operation" >&2
      exit 2
      ;;
  esac
  [[ $after != "$before" ]] || return 1
  printf '%s\n' "$after" >"$file"
}

fresh_copies
if bash "$check" "$scratch/run/SKILL.md" "$scratch/run/skills" >"$scratch/out" 2>&1; then
  pass "the unchanged copies pass"
else
  fail "the unchanged copies pass"
  cat "$scratch/out"
fi

for control in "${controls[@]}"; do
  IFS='|' read -r assertion side file operation text replacement <<<"$control"
  label="$assertion fails on a mutated copy of the $side side"
  fresh_copies
  if [[ $side == skill ]]; then
    target=$scratch/run/SKILL.md
  else
    target=$scratch/run/skills/$file
  fi
  if ! mutate "$target" "$operation" "$text" "$replacement"; then
    fail "$label: the mutation changed nothing"
    continue
  fi
  status=0
  bash "$check" "$scratch/run/SKILL.md" "$scratch/run/skills" >"$scratch/out" 2>&1 || status=$?
  failing=$(sed -n 's/^FAIL \([a-z_]*\):.*/\1/p' "$scratch/out" | sort -u)
  if [[ $status -eq 1 && $failing == "$assertion" ]]; then
    pass "$label"
  else
    fail "$label: exit $status, failing assertions: ${failing:-none}"
    cat "$scratch/out"
  fi
done

echo "doubt composition controls: $passed passed, $failed failed"
((failed == 0))
