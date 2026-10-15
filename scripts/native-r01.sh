#!/usr/bin/env bash
# R01 on one measured host: the protected test identities, preserved through the
# existing Taskfile tasks, joined identity by identity.
#
#   native-r01.sh run --out DIR     list and freeze the identities, digest every
#                                   subject, run the six tasks one at a time,
#                                   digest again, then join each log
#   native-r01.sh join MANIFEST LOG SCOPE_REGEX
#   native-r01.sh selftest          the join is seen to reject its wrong inputs
#
# This measures nothing by itself and invents no test: it runs the tasks the
# Taskfile already has, in the order below, and reads their logs. The join is the
# one k43 wrote after the Darwin driver was frozen (docs/evidence/
# native-entry-campaign-r01/r01-join.sh), kept here so a second target uses the
# same reading and so it has failing controls of its own.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="docs/native-entry-cases.json"

die() {
  printf 'native-r01: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat >&2 <<'EOF'
usage: native-r01.sh run --out DIR
       native-r01.sh join MANIFEST LOG SCOPE_REGEX
       native-r01.sh selftest
EOF
  exit 2
}

# The tasks, in the order k43 ran them, with the binaries each is expected to
# run (an extended regex over the nextest binary id, anchored by `join`). A
# crate's own library tests have the bare crate name as their binary id, with
# no `::`, so a crate's scope names both forms. test:terminal-recovery selects
# two private PTY cases by name; it has no identity scope and is recorded by
# exit status and its own nextest summary.
tasks=(
  "test:cli|ravel::cli"
  "test:session|ravel(::.*)?"
  "test:runner|keyed-launch(::.*)?"
  "test:terminal-recovery|"
  "test|.*"
  "check|.*"
)

# Join one nextest log with the manifest's protected identities, item by item.
# Prints a JSON summary beside the log and exits nonzero unless every
# protected, non-ignored identity in scope has a PASS line, no failure-class line
# names any test, and at least one identity was checked.
cmd_join() {
  [ $# -eq 3 ] || usage
  local manifest_file="$1" log="$2" scope="$3" passed failed status=0
  passed="$(mktemp)"
  failed="$(mktemp)"
  # nextest status lines: "        PASS [   0.123s] ( 1/35) <binary-id> <test name>"
  # (the progress counter is present only on some runs, so it is optional).
  sed -E -n 's/^[[:space:]]*PASS \[[^]]*\] +(\( *[0-9]+\/[0-9]+\) +)?([^ ]+) +(.*)$/\2\t\3/p' "$log" |
    sort -u >"$passed"
  grep -E '^[[:space:]]*(FAIL|TIMEOUT|SIGABRT|SIGSEGV|LEAK|LEAK-FAIL|ABORT) ' "$log" >"$failed" || true
  jq -n --slurpfile m "$manifest_file" --arg scope "^(${scope})\$" \
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
       failure_lines: ($failed | split("\n") | map(select(length > 0)))}' >"$log.r01.json"
  jq -c '{in_scope, ignored_fixtures_in_scope, checked, passed,
          not_passed: (.not_passed | length), failure_lines: (.failure_lines | length)}' "$log.r01.json"
  jq -e '(.not_passed | length) == 0 and (.failure_lines | length) == 0 and .checked > 0' \
    "$log.r01.json" >/dev/null || status=1
  rm -f "$passed" "$failed"
  return "$status"
}

cmd_run() {
  [ "${1:-}" = "--out" ] && [ -n "${2:-}" ] || usage
  local out="$2"
  [ ! -e "$out" ] || die "output exists (one writer, never reused): $out"
  mkdir -p "$out"
  out="$(cd "$out" && pwd)"
  cd "$root"
  local failed=0 entry name scope code

  # Freeze the identities first: every protected identity present with its
  # ignored flag, new ones counted apart.
  bash scripts/native-campaign.sh r01 --out "$out/frozen-listing.json" ||
    die "protected identities are not all preserved in the listing"
  bash scripts/native-campaign.sh digest --out "$out/subjects-before-r01.json"

  : >"$out/exit-codes.txt"
  for entry in "${tasks[@]}"; do
    name="${entry%%|*}"
    scope="${entry#*|}"
    local slug="${name//:/-}"
    date -u +%FT%TZ >"$out/$slug.started"
    code=0
    task "$name" >"$out/$slug.txt" 2>&1 || code=$?
    date -u +%FT%TZ >"$out/$slug.finished"
    printf '%s\t%s\n' "$name" "$code" >>"$out/exit-codes.txt"
    printf 'native-r01: %s exited %s\n' "$name" "$code"
    [ "$code" -eq 0 ] || failed=1
    if [ -n "$scope" ]; then
      cmd_join "$root/$manifest" "$out/$slug.txt" "$scope" || failed=1
    fi
  done

  bash scripts/native-campaign.sh digest --out "$out/subjects-after-r01.json"
  bash scripts/native-campaign.sh compare "$out/subjects-before-r01.json" "$out/subjects-after-r01.json" ||
    failed=1
  [ "$failed" -eq 0 ]
}

# Failing controls: the join is credited only after it rejects each wrong input.
cmd_selftest() {
  local work
  work="$(mktemp -d)"
  trap 'rm -rf "$work"' RETURN
  jq -n '{R01: [
      {binary: "pkg::a", test: "one", ignored: false},
      {binary: "pkg::a", test: "two", ignored: false},
      {binary: "pkg::a", test: "fixture", ignored: true},
      {binary: "other::b", test: "three", ignored: false}]}' >"$work/m.json"
  {
    printf '        PASS [   0.010s] ( 1/3) pkg::a one\n'
    printf '        PASS [   0.020s] ( 2/3) pkg::a two\n'
    printf '        PASS [   0.030s] ( 3/3) other::b three\n'
  } >"$work/good.txt"
  cmd_join "$work/m.json" "$work/good.txt" '.*' >/dev/null ||
    die "selftest: a complete log was rejected"
  jq -e '.checked == 3 and .ignored_fixtures_in_scope == 1' "$work/good.txt.r01.json" >/dev/null ||
    die "selftest: the ignored fixture was counted as a checked identity"
  printf 'selftest ok   a complete log, the ignored fixture counted apart\n'

  grep -v ' two$' "$work/good.txt" >"$work/missing.txt"
  if cmd_join "$work/m.json" "$work/missing.txt" '.*' >/dev/null 2>&1; then
    die "selftest: a log missing one identity's PASS line was accepted"
  fi
  jq -e '.not_passed | length == 1 and .[0].test == "two"' "$work/missing.txt.r01.json" >/dev/null ||
    die "selftest: the missing identity was not named"
  printf 'selftest ok   one identity without a PASS line (rejected, and named)\n'

  { cat "$work/good.txt"; printf '        FAIL [   0.040s] ( 4/4) pkg::a extra\n'; } >"$work/failure.txt"
  if cmd_join "$work/m.json" "$work/failure.txt" '.*' >/dev/null 2>&1; then
    die "selftest: a log with a FAIL line was accepted"
  fi
  printf 'selftest ok   an appended FAIL line (rejected)\n'

  : >"$work/empty.txt"
  if cmd_join "$work/m.json" "$work/empty.txt" '.*' >/dev/null 2>&1; then
    die "selftest: an empty log was accepted"
  fi
  printf 'selftest ok   an empty log (rejected)\n'

  if cmd_join "$work/m.json" "$work/good.txt" 'nothing::.*' >/dev/null 2>&1; then
    die "selftest: a scope that matches nothing was accepted"
  fi
  printf 'selftest ok   a scope that matches no identity (rejected)\n'

  # A crate's library tests carry the bare crate name; the scope used for a
  # crate must reach them as well as its integration binaries.
  jq -n '{R01: [{binary: "pkg", test: "lib", ignored: false},
                {binary: "pkg::a", test: "one", ignored: false}]}' >"$work/m-lib.json"
  printf '        PASS [   0.010s] ( 1/2) pkg lib\n        PASS [   0.020s] ( 2/2) pkg::a one\n' >"$work/lib.txt"
  cmd_join "$work/m-lib.json" "$work/lib.txt" 'pkg(::.*)?' >/dev/null ||
    die "selftest: a crate scope did not reach both its library and its binaries"
  jq -e '.checked == 2' "$work/lib.txt.r01.json" >/dev/null ||
    die "selftest: the crate scope did not check both identities"
  if cmd_join "$work/m-lib.json" "$work/lib.txt" 'pkg::.*' >/dev/null 2>&1; then
    # The narrower scope reaches one identity and passes: it is the count that
    # shows the library identity was left out.
    jq -e '.checked == 1' "$work/lib.txt.r01.json" >/dev/null ||
      die "selftest: the narrow scope did not leave the library identity out"
  fi
  printf 'selftest ok   a crate scope reaches its library tests (the narrower scope leaves them out, by count)\n'

  # The scope is anchored: a prefix of a binary id selects nothing.
  if cmd_join "$work/m.json" "$work/good.txt" 'pkg' >/dev/null 2>&1; then
    die "selftest: an unanchored scope selected a binary"
  fi
  printf 'selftest ok   a scope that is only a prefix of a binary id (rejected)\n'
}

[ $# -ge 1 ] || usage
mode="$1"
shift
case "$mode" in
  run) cmd_run "$@" ;;
  join) cmd_join "$@" ;;
  selftest) cmd_selftest ;;
  *) usage ;;
esac
