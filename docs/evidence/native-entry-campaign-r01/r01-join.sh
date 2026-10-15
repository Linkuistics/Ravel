#!/usr/bin/env bash
# Join one nextest log with the manifest's protected identities, item by item.
#   r01-join.sh MANIFEST LOG SCOPE_REGEX
# SCOPE_REGEX selects the protected binaries the task is expected to run (an
# extended regex over the binary id, anchored by this script). Prints a JSON
# summary and exits nonzero unless every protected, non-ignored identity in
# scope has a PASS line and no FAIL/TIMEOUT/SIGSEGV-style line names one.
set -euo pipefail
manifest="$1" log="$2" scope="$3"
passed="$(mktemp)"
failed="$(mktemp)"
# nextest status lines: "        PASS [   0.123s] ( 1/35) <binary-id> <test name>"
# (the progress counter is present only on some runs, so it is optional).
sed -E -n 's/^[[:space:]]*PASS \[[^]]*\] +(\( *[0-9]+\/[0-9]+\) +)?([^ ]+) +(.*)$/\2\t\3/p' "$log" |
  sort -u >"$passed"
grep -E '^[[:space:]]*(FAIL|TIMEOUT|SIGABRT|SIGSEGV|LEAK|LEAK-FAIL|ABORT) ' "$log" >"$failed" || true
jq -n --slurpfile m "$manifest" --arg scope "^(${scope})\$" \
  --rawfile passed "$passed" --rawfile failed "$failed" '
  ($passed | split("\n") | map(select(length > 0) | split("\t") | "\(.[0])\t\(.[1])")) as $p
  | [ $m[0].R01[] | select(.binary | test($scope)) ] as $inscope
  | [ $inscope[] | select(.ignored | not)
      | . as $e
      | {binary: $e.binary, test: $e.test,
         status: (if ($p | index("\($e.binary)\t\($e.test)")) != null then "passed" else "not-passed" end)} ] as $rows
  | {in_scope: ($inscope | length),
     ignored_fixtures_in_scope: ($inscope | map(select(.ignored)) | length),
     checked: ($rows | length),
     passed: ($rows | map(select(.status == "passed")) | length),
     not_passed: ($rows | map(select(.status != "passed"))),
     failure_lines: ($failed | split("\n") | map(select(length > 0)))}' |
  tee "$log.r01.json" |
  jq -c '{in_scope, ignored_fixtures_in_scope, checked, passed, not_passed: (.not_passed | length), failure_lines: (.failure_lines | length)}'
rm -f "$passed" "$failed"
jq -e '(.not_passed | length) == 0 and (.failure_lines | length) == 0 and .checked > 0' "$log.r01.json" >/dev/null
