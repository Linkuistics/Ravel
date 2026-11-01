#!/usr/bin/env bash
# Prove a Rhai upgrade at the command-line seam: print the pin, run the engine
# tests, then run the hostile classes in a fixed order. Only the `rhai_*` test
# targets of crates/ravel run, and every case starts the real `ravel
# start-session`. The task changes no pin. An owner who upgrades Rhai edits the
# pin, then runs it (docs/specs/configuration-evaluation.md, "Upgrades").
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

manifest=crates/ravel/Cargo.toml
tests_dir=crates/ravel/tests

# The engine tests, in order. Each is a `rhai_*` test target.
engine_targets=(
  rhai_config
  rhai_request
  rhai_settings
  rhai_helpers
  rhai_json_check
  rhai_reads
  rhai_role
  rhai_harness_run
  rhai_llm_calls
  rhai_examples
)

# The hostile target and its classes, in order. The firing control of each class
# sits in the test of the same name: the hostile step fires a named bound, and
# the same shape with a harmless amount selects. The running log of
# rhai-upgrade-task-k65 says where each control sits.
hostile_target=rhai_limits
hostile_tests=(
  an_endless_loop_ends_at_the_operations_bound
  unbounded_recursion_ends_at_the_call_depth
  a_string_that_doubles_ends_at_the_string_size
  an_array_that_doubles_ends_at_the_array_size
  try_does_not_catch_a_breach_and_the_host_refuses_one_that_a_closure_call_wrapped
  a_breach_inside_a_function_pointer_call_ends_the_evaluation_although_try_surrounds_it
  a_breach_inside_a_function_named_by_drain_or_retain_ends_the_evaluation_under_try
  a_breach_inside_a_comparer_ends_the_evaluation_although_no_try_is_involved
  a_config_that_fills_the_heap_ends_at_the_heap_bound_and_the_cli_reports_it
  a_native_call_that_does_not_yield_stops_at_the_deadline
  a_config_that_prints_without_end_ends_as_output_limit
  a_result_over_the_message_bound_ends_as_message_too_large
  a_signal_during_the_evaluation_kills_and_reaps_the_worker_and_launches_nothing
  a_worker_killed_from_outside_refuses_as_worker_failed
  nothing_a_script_returns_can_be_called_after_it_ends
)

fail() {
  printf 'rhai-upgrade-proof: %s\n' "$*" >&2
  exit 1
}

# The pin that a Rhai upgrade changes, and the version that the lock file builds.
pin="$(sed -nE 's/^rhai = \{ version = "=([^"]+)".*/\1/p' "$manifest")"
[[ -n "$pin" ]] || fail "$manifest has no exact pin for rhai (expected rhai = { version = \"=X.Y.Z\", ... })"
locked="$(awk '$0 == "name = \"rhai\"" { getline; gsub(/version = |"/, ""); print; exit }' Cargo.lock)"
[[ "$locked" == "$pin" ]] || fail "the pin is $pin and Cargo.lock builds ${locked:-nothing}; run cargo update -p rhai --precise $pin"
printf 'rhai-upgrade-proof: pinned Rhai %s (%s, Cargo.lock agrees)\n' "$pin" "$manifest"

# Every `rhai_*` test file must be named above, so that a new target cannot
# pass unrun, and every name must exist.
declared="$(printf '%s\n' "${engine_targets[@]}" "$hostile_target" | sort)"
present="$(find "$tests_dir" -maxdepth 1 -name 'rhai_*.rs' -exec basename {} .rs \; | sort)"
[[ "$declared" == "$present" ]] || {
  printf 'declared rhai_* targets:\n%s\npresent in %s:\n%s\n' "$declared" "$tests_dir" "$present" >&2
  fail "the declared targets and the files of $tests_dir differ"
}

# nextest leaves an ignored test out of its list, so an ignored case would pass
# unrun. Refuse it in every target.
if ignored="$(grep -n '#\[ignore' "$tests_dir"/rhai_*.rs)"; then
  printf '%s\n' "$ignored" >&2
  fail "a rhai_* test is ignored, so the proof would skip it"
fi

# Run one test target and demand that it ran at least one test, and that every
# listed test ran and passed. `expected` is the number of cases the run must
# report.
run_target() {
  local target="$1" expected="$2" log summary run passed
  log="$(mktemp "${TMPDIR:-/tmp}/rhai-upgrade-proof.XXXXXX")"
  printf '\n=== %s\n' "$target"
  if ! cargo nextest run --locked --color never --no-tests fail -p ravel --test "$target" 2>&1 | tee "$log"; then
    rm -f "$log"
    fail "$target failed"
  fi
  summary="$(grep -E 'Summary \[.*\] +[0-9]+ tests? run:' "$log" | tail -n 1 || true)"
  rm -f "$log"
  [[ -n "$summary" ]] || fail "$target printed no summary, so it ran no test"
  run="$(sed -E 's/.*\] +([0-9]+) tests? run:.*/\1/' <<<"$summary")"
  passed="$(sed -E 's/.*: ([0-9]+) passed.*/\1/' <<<"$summary")"
  [[ "$run" -gt 0 ]] || fail "$target ran no test"
  [[ "$run" == "$passed" ]] || fail "$target: $run run and $passed passed"
  [[ "$run" == "$expected" ]] || fail "$target ran $run cases and $expected were listed"
  printf 'rhai-upgrade-proof: %s ran %s tests\n' "$target" "$run"
}

list_target() {
  cargo nextest list --locked --color never -p ravel --test "$1" --message-format oneline 2>/dev/null \
    | sed -E "s/^ravel::$1 //" | sort
}

printf '\n=== engine tests\n'
for target in "${engine_targets[@]}"; do
  listed="$(list_target "$target")" || fail "$target is not a test target of ravel"
  [[ -n "$listed" ]] || fail "$target lists no test"
  run_target "$target" "$(wc -l <<<"$listed" | tr -d ' ')"
done

printf '\n=== hostile classes\n'
listed="$(list_target "$hostile_target")" || fail "$hostile_target is not a test target of ravel"
expected="$(printf '%s\n' "${hostile_tests[@]}" | sort)"
[[ "$listed" == "$expected" ]] || {
  printf 'hostile classes named by the task:\n%s\nlisted by %s:\n%s\n' "$expected" "$hostile_target" "$listed" >&2
  fail "a named hostile class is missing from $hostile_target, or $hostile_target holds a test that the task does not name"
}
# One run per class, in the fixed order, so that a failure names its class.
for name in "${hostile_tests[@]}"; do
  printf '\n--- hostile class: %s\n' "$name"
  log="$(mktemp "${TMPDIR:-/tmp}/rhai-upgrade-proof.XXXXXX")"
  if ! cargo nextest run --locked --color never --no-tests fail -p ravel --test "$hostile_target" -E "test(=$name)" 2>&1 | tee "$log"; then
    rm -f "$log"
    fail "hostile class $name failed"
  fi
  grep -qE 'Summary \[.*\] +1 test run: 1 passed' "$log" || {
    rm -f "$log"
    fail "hostile class $name did not run exactly one passing test"
  }
  rm -f "$log"
done

printf '\nrhai-upgrade-proof: Rhai %s passed %s engine targets and %s hostile classes\n' \
  "$pin" "${#engine_targets[@]}" "${#hostile_tests[@]}"
