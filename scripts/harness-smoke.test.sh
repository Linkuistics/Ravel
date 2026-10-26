#!/usr/bin/env bash
# Test scripts/harness-smoke.sh against fake harnesses, so that no test makes a
# real call. Each success carries a firing control: the same script with a
# fake that answers otherwise must fail, so a check that read clean for every
# input would fail here. The real calls are the opt-in tasks harness:smoke:*.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
smoke="$root/scripts/harness-smoke.sh"
work="$(mktemp -d "${TMPDIR:-/tmp}/harness-smoke-test.XXXXXX")"
trap 'rm -rf "$work"' EXIT

(cd "$root" && cargo build --locked --quiet -p ravel)
export HARNESS_SMOKE_RAVEL="$root/target/debug/ravel"

fail() {
  printf 'harness-smoke.test: %s\n' "$*" >&2
  exit 1
}

# A fake harness that records its arguments and its standard input, and then
# runs $2, a shell fragment.
fake() {
  local path="$work/$1"
  # shellcheck disable=SC2016 # the fake script expands $1 and $@ when it runs
  printf '#!/bin/sh\nif [ "$1" = "--version" ]; then echo "fake 1.0"; exit 0; fi\nprintf "%%s\\n" "$@" > "%s.argv"\ncat > "%s.stdin"\n%s\n' \
    "$path" "$path" "$2" >"$path"
  chmod +x "$path"
  printf '%s' "$path"
}

expect_pass() {
  local label="$1" harness="$2" program="$3" log="$work/$1.log"
  HARNESS_SMOKE_PROGRAM="$program" bash "$smoke" "$harness" >"$log" 2>&1 || {
    cat "$log" >&2
    fail "$label: the task failed and should pass"
  }
  grep -q "harness-smoke: $harness passed" "$log" || fail "$label: no pass line"
}

expect_fail() {
  local label="$1" harness="$2" program="$3" pattern="$4" log="$work/$1.log"
  if HARNESS_SMOKE_PROGRAM="$program" bash "$smoke" "$harness" >"$log" 2>&1; then
    cat "$log" >&2
    fail "$label: the task passed and should fail"
  fi
  grep -q "$pattern" "$log" || {
    cat "$log" >&2
    fail "$label: the failure does not match '$pattern'"
  }
}

# Claude Code: the answer is in structured_output of one JSON document.
claude="$(fake claude-yes 'echo "{\"type\":\"result\",\"structured_output\":{\"prime\":\"yes\"}}"')"
expect_pass claude-yes claude "$claude"
grep -qx -- '--json-schema' "$claude.argv" || fail "claude: --json-schema is not in the arguments"
grep -qx -- '--output-format' "$claude.argv" || fail "claude: --output-format is not in the arguments"
grep -q 'Is 17 a prime number' "$claude.stdin" || fail "claude: the question did not reach stdin"
expect_fail claude-no claude \
  "$(fake claude-no 'echo "{\"structured_output\":{\"prime\":\"no\"}}"')" "fixed answer is 'routed yes'"
expect_fail claude-no-structured claude \
  "$(fake claude-none 'echo "{\"type\":\"result\",\"result\":\"yes\"}"')" "smoke_call_failed\|structured_output\|the call did not end"
expect_fail claude-exit-1 claude \
  "$(fake claude-bad 'echo "rate limited" >&2; exit 1')" "rate limited"

# Codex: the schema is a file that the harness is told to read, and the answer
# is the whole of stdout.
codex="$(fake codex-yes 'echo "{\"prime\":\"yes\"}"')"
expect_pass codex-yes codex "$codex"
grep -qx -- '--output-schema' "$codex.argv" || fail "codex: --output-schema is not in the arguments"
schema_file="$(grep -A1 -x -- '--output-schema' "$codex.argv" | tail -n 1)"
[[ "$schema_file" == */answer-schema.json ]] || fail "codex: the schema argument is '$schema_file'"
expect_fail codex-no codex \
  "$(fake codex-no 'echo "{\"prime\":\"no\"}"')" "fixed answer is 'routed yes'"
expect_fail codex-bad-shape codex \
  "$(fake codex-shape 'echo "{\"prime\":\"yes\",\"extra\":1}"')" "json_mismatch\|did not end in a session"

# A missing program, and an unknown harness, name what is missing.
expect_fail missing-program claude "$work/does-not-exist" "no 'claude' program found"
if bash "$smoke" nobody >"$work/unknown.log" 2>&1; then fail "an unknown harness passed"; fi
grep -q "unknown harness 'nobody'" "$work/unknown.log" || fail "an unknown harness is not named"

printf 'harness-smoke.test: passed\n'
