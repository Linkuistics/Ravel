#!/usr/bin/env bash
# Check the composition of `doubt-driven-development` with Grove.
#
# Usage: doubt-composition.sh <doubt SKILL.md> <Grove plugins/grove/skills>
#
# Grove's `docs/specs/pack-membership.md`, "The Grove assets of moving craft
# skills", moved these assertions from Grove's `composition_guidance.rs` and
# `session_kind_guidance.rs` to this pack. Each assertion below carries the
# skill's half of one of those Grove tests, which the assertion's name gives.
# Grove keeps the half that reads its own files.
#
# Each assertion also names the Grove phrase that the skill's half composes
# with. The check reads that phrase from the Grove checkout of `GROVE_COMMIT`
# only to compare, and it fails when the checkout does not hold it. A change of
# Grove's wording therefore fails here when the Taskfile moves `GROVE_COMMIT`.
#
# Each comparison collapses runs of white space to one space and is
# case-sensitive, as the Rust tests were. `doubt-composition.test.sh` holds the
# controls, which mutate a copy of each side and expect one failure.

set -euo pipefail

usage() {
  echo "usage: doubt-composition.sh <doubt SKILL.md> <Grove plugins/grove/skills>" >&2
  exit 2
}

[[ $# -eq 2 ]] || usage
skill_file=$1
grove_skills=$2
[[ -f $skill_file ]] || {
  echo "error: no doubt skill at $skill_file" >&2
  exit 2
}
[[ -d $grove_skills/grove ]] || {
  echo "error: no Grove spine under $grove_skills" >&2
  exit 2
}

normalise() {
  local text
  text=$(tr -s '[:space:]' ' ' <"$1")
  printf '%s' "${text# }"
}

skill=$(normalise "$skill_file")
failures=0
problems=()

# The skill holds the phrase.
holds() {
  [[ $skill == *"$1"* ]] || problems+=("the skill does not hold \"$1\"")
}

# The skill does not hold the phrase.
lacks() {
  [[ $skill != *"$1"* ]] || problems+=("the skill holds the withdrawn phrase \"$1\"")
}

# The Grove file, relative to the skills directory, holds the phrase.
grove_holds() {
  local file=$grove_skills/$1 text
  if [[ ! -f $file ]]; then
    problems+=("Grove has no file $1")
    return
  fi
  text=$(normalise "$file")
  [[ $text == *"$2"* ]] || problems+=("Grove's $1 does not hold \"$2\"")
}

# Run one assertion function and report it by name.
assert() {
  local name=$1 problem
  problems=()
  "$name"
  if [[ ${#problems[@]} -eq 0 ]]; then
    echo "ok   $name"
  else
    failures=$((failures + 1))
    for problem in "${problems[@]}"; do
      echo "FAIL $name: $problem"
    done
  fi
}

# no_provisioned_surface_instructs_a_deleted_composition_verb. The skill names
# the verb that Grove's decompose reference teaches, and neither deleted verb.
no_deleted_composition_verb() {
  local verb='grove-llm leaf-add <parent> <stem> --kind review-<producer>'
  holds "$verb"
  lacks 'leaf-add-chain'
  lacks 'leaf-promote-chain'
  grove_holds grove/references/decompose.md "$verb"
}

# guidance_cuts_the_integrate_step_where_pick_reaches_it_next. Grove's
# decompose reference states the placement of an integration step. The skill
# does not state the withdrawn exception to it.
no_withdrawn_adjacency_exception() {
  lacks 'touches no file the findings cite'
  grove_holds grove/references/decompose.md \
    'the first sibling entry after the review whose subtree still holds live work'
}

# doubt_guidance_yields_to_grove_without_changing_standalone_doubt. The skill's
# section "Composition with Grove" states the allowance of Grove's execute
# reference, and the standalone process keeps its bound.
yields_to_grove_review_allowance() {
  holds '## Composition with Grove'
  holds 'Bootstrap'
  holds 'whole picked leaf'
  holds '--kind review-<producer>'
  holds 'Substantial redesign becomes a new producer review chain beside the leaf being integrated'
  holds 'Outside that predicate'
  holds '3 cycles'
  grove_holds grove/references/execute.md 'across the whole picked leaf'
  grove_holds grove/references/execute.md \
    'outside that predicate, doubt keeps its standalone bounded cycles unchanged'
  grove_holds grove/references/execute.md \
    'substantial redesign as a new producer review chain beside the leaf it is integrating'
}

# every_handoff_passage_retires_before_it_commits. In the skill's handoff
# passage, retirement comes before the commit, as in Grove's loop.
handoff_retires_before_commit() {
  local start='After cutting that leaf' end='Grove owns the escalated route'
  local rest passage
  if [[ $skill != *"$start"* ]]; then
    problems+=("the skill does not hold the passage opening \"$start\"")
  else
    rest=${skill#*"$start"}
    if [[ $rest != *"$end"* ]]; then
      problems+=("the skill does not hold the passage close \"$end\"")
    else
      passage=${rest%%"$end"*}
      if [[ $passage != *retir* ]]; then
        problems+=("the handoff passage never retires")
      elif [[ $passage != *commit* ]]; then
        problems+=("the handoff passage never commits")
      else
        local before_retire=${passage%%retir*} before_commit=${passage%%commit*}
        ((${#before_retire} < ${#before_commit})) ||
          problems+=("the handoff passage commits before it retires")
      fi
    fi
  fi
  # The backticks are Markdown, not command substitution.
  # shellcheck disable=SC2016
  grove_holds grove/SKILL.md \
    'retire the leaf as `references/retire.md` directs, before committing'
}

# canonical_guidance_drops_receipt_and_diversity_era_review_routing. The skill
# states what Grove's decompose reference states: the driver compares nothing,
# so no receipt, notice or routing peek exists.
no_receipt_routing() {
  local rejected
  for rejected in 'Producer launch' 'producer generation' 'handoff receipt' \
    'diversity warning' 'routing peek'; do
    lacks "$rejected"
  done
  holds 'Grove compares no targets and raises no notice, so there is none to wait for or consume'
  grove_holds grove/references/decompose.md \
    'task-tree driver records nothing about how the producer ran, compares nothing, and warns about nothing'
}

# grove_and_doubt_state_one_mandate_based_ownership_predicate. The skill states
# the mandate-based predicate of Grove's execute reference, not the superseded
# pick-based one.
mandate_ownership_predicate() {
  holds 'mandate'
  holds 'Bootstrap'
  lacks 'Bootstrap-and-pick'
  holds 'adopted that mandate by running'
  grove_holds grove/references/execute.md 'adopted that mandate by running Bootstrap'
}

# every_session_kind_is_named_by_the_provisioned_taxonomy. The skill names the
# three research kinds that Grove's execute reference gives no reviewer.
names_research_kinds() {
  holds 'research-a'
  holds 'research-b'
  holds 'combine-research'
  # The backticks are Markdown, not command substitution.
  # shellcheck disable=SC2016
  grove_holds grove/references/execute.md \
    '`research-a`, `research-b` and `combine-research` spend **none**'
}

assertions=(
  no_deleted_composition_verb
  no_withdrawn_adjacency_exception
  yields_to_grove_review_allowance
  handoff_retires_before_commit
  no_receipt_routing
  mandate_ownership_predicate
  names_research_kinds
)

for name in "${assertions[@]}"; do
  assert "$name"
done

if ((failures > 0)); then
  echo "doubt composition: $failures of ${#assertions[@]} assertions failed" >&2
  exit 1
fi
echo "doubt composition: ${#assertions[@]} assertions passed"
