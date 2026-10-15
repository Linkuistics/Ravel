#!/usr/bin/env bash
# Native-entry campaign driver: freeze subjects, run the finite native cases with
# one writer, digest every subject before and after, and reconcile the rows the
# instruments recorded against the frozen manifest and plan item by item.
#
# This script measures nothing by itself. Instruments record one write-once file
# per judged (row, subject) under ROWS; this script is the only writer of every
# other file in the campaign directory.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="docs/native-entry-cases.json"
plan="docs/native-entry-campaign-plan.json"

die() {
  printf 'native-campaign: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat >&2 <<'EOF'
usage: native-campaign.sh run --out DIR
       native-campaign.sh reconcile --out DIR   (offline: DIR already holds rows/)
       native-campaign.sh digest --out FILE     (write one subject digest map)
       native-campaign.sh compare BEFORE AFTER  (exit 1 if any subject moved)
       native-campaign.sh r01 --out FILE        (reconcile the 310 protected identities)
       native-campaign.sh selftest              (failing controls for this driver)

run builds the private fixtures, freezes and digests every subject, runs the
three finite native Taskfile selections under separate nextest profiles, digests
again and reconciles. DIR must not exist: a campaign directory has one writer
and is never reused.
EOF
  exit 2
}

# Every file a measurement reads or executes. Generated dependency trees and
# build output are excluded here and covered by the built-binary digests below.
subject_paths() {
  (
    cd "$root"
    find Cargo.toml Cargo.lock Taskfile.yml .cargo .config scripts crates \
      -type f \
      -not -path 'crates/ravel/worker/node_modules/*' \
      -not -path '*/target/*' \
      -not -name '.DS_Store' | LC_ALL=C sort
    printf '%s\n' \
      "$manifest" "$plan" \
      docs/native-entry-transition-design.md \
      docs/specs/result-job-completion.md
  )
}

# Digest map as one JSON object: path -> sha256. Built binaries are named by
# the kind of subject they are, never by a path that changes between builds.
digest_to() {
  local out="$1" listing
  listing="$(mktemp)"
  {
    (cd "$root" && subject_paths | while IFS= read -r path; do
      [ -f "$path" ] || die "subject missing: $path"
      printf 'source\t%s\t%s\n' "$path" "$(shasum -a 256 "$path" | cut -d' ' -f1)"
    done)
    for built in target/native-entry-fixture/debug/ravel target/libexec/ravel/ravel-policy; do
      [ -f "$root/$built" ] || die "built subject missing: $built (run the build first)"
      printf 'built\t%s\t%s\n' "$built" "$(shasum -a 256 "$root/$built" | cut -d' ' -f1)"
    done
    (cd "$root" && cargo nextest list --locked --workspace --message-format json 2>/dev/null |
      jq -r '."rust-suites"[] | "\(."binary-id")\t\(."binary-path")"' | LC_ALL=C sort |
      while IFS=$'\t' read -r id path; do
        printf 'test-binary\t%s\t%s\n' "$id" "$(shasum -a 256 "$path" | cut -d' ' -f1)"
      done)
  } >"$listing"
  jq -Rn '[inputs | split("\t") | {kind: .[0], name: .[1], sha256: .[2]}]
          | map({key: "\(.kind):\(.name)", value: .sha256}) | from_entries' <"$listing" >"$out"
  rm -f "$listing"
}

domain_to() {
  local out="$1"
  jq -n \
    --arg rustc "$(rustc -vV | tr '\n' ';')" \
    --arg cargo "$(cargo -V)" \
    --arg nextest "$(cargo nextest --version | head -n 1)" \
    --arg task "$(task --version)" \
    --arg uname "$(uname -a)" \
    --arg os "$( (sw_vers 2>/dev/null || cat /etc/os-release 2>/dev/null) | tr '\n' ';')" \
    --arg libc "$(awk '/^name = "libc"$/{getline; print $3}' "$root/Cargo.lock" | tr -d '"' | head -n 1)" \
    --arg revision "$( (cd "$root" && jj log --no-graph -r @- -T 'commit_id' 2>/dev/null) || echo unknown)" \
    '{rustc: $rustc, cargo: $cargo, nextest: $nextest, task: $task, uname: $uname,
      os: $os, libc_crate: $libc, parent_revision: $revision}' >"$out"
}

# compare BEFORE AFTER: a subject that moved, appeared or vanished is a failure.
compare() {
  local before="$1" after="$2" moved
  moved="$(jq -n --slurpfile b "$before" --slurpfile a "$after" '
    ($b[0]) as $x | ($a[0]) as $y
    | [ ($x | keys[]) | select(($y[.] // null) != $x[.]) ] + [ ($y | keys[]) | select($x[.] == null) ]
    | unique')"
  if [ "$moved" != "[]" ]; then
    printf 'native-campaign: subjects moved during the run:\n%s\n' "$moved" >&2
    return 1
  fi
}

# One line per executed test case from a nextest junit file, with its verdict.
# A <failure> or <error> element inside a case makes that case failed.
junit_tests() {
  awk '
    /<testcase /{
      match($0, /name="[^"]*"/); name = substr($0, RSTART + 6, RLENGTH - 7)
      match($0, /classname="[^"]*"/); class = substr($0, RSTART + 11, RLENGTH - 12)
      current = class "\t" name; status[current] = "passed"; order[++count] = current
    }
    /^ *<(failure|error)[ >]/ { if (current != "") status[current] = "failed" }
    END { for (i = 1; i <= count; i++) print order[i] "\t" status[order[i]] }
  ' "$@"
}

tests_json_to() {
  local out="$1"
  shift
  junit_tests "$@" | jq -Rn '[inputs | split("\t") | {binary: .[0], test: .[1], status: .[2]}]' >"$out"
}

# Expand the plan over the manifest and compare it with the recorded rows.
# Writes RECONCILIATION and one receipt per manifest row under RECEIPTS; the
# exit status is nonzero unless every row reconciled and nothing was unknown.
reconcile_into() {
  local rows="$1" manifest_file="$2" plan_file="$3" receipts="$4" summary="$5" tests_file="${6:-}"
  mkdir -p "$receipts"
  if [ -z "$tests_file" ]; then
    tests_file="$(mktemp)"
    printf '[]' >"$tests_file"
    local own_tests=1
  fi
  local records
  records="$(mktemp)"
  if compgen -G "$rows/*.json" >/dev/null; then
    jq -s '.' "$rows"/*.json >"$records"
  else
    printf '[]' >"$records"
  fi
  jq -n --slurpfile m "$manifest_file" --slurpfile p "$plan_file" --slurpfile r "$records" \
    --slurpfile t "$tests_file" --argjson check_tests "$([ -n "${own_tests:-}" ] && echo false || echo true)" \
    --arg receipts "$receipts" '
    ($m[0].rows) as $rows | ($p[0]) as $plan | ($r[0]) as $records
    | ($records | group_by(.id) | map({key: .[0].id, value: .}) | from_entries) as $byid
    | ($rows | map(.id)) as $ids
    | ($rows | map(
        . as $row
        | ($plan.rows[$row.id]
           // ([$plan.patterns[]? | select(. as $pat | $pat.family == $row.family and ($row.id | test($pat.match)))] | first | .spec)
           // $plan.families[$row.family]) as $spec
        | if $spec == null then {id: $row.id, family: $row.family, status: "unplanned",
              expected: [], observed: [], problems: ["no plan entry for the row or its family"]}
          else
            ([($spec.repaired // [])[] | {subject: ., conforms: true}]
             + [($spec.violates // [])[] | {subject: ., conforms: false}]
             + [($spec.conforms // [])[] | {subject: ., conforms: true}]) as $expected
            | ($byid[$row.id] // []) as $seen
            | ($seen | map({subject, conforms, test, note})) as $observed
            | ([ $expected[] | . as $e
                 | if ($observed | map(select(.subject == $e.subject)) | length) == 0
                   then "missing record: \($e.subject)" else empty end ]
               + [ $observed[] | . as $o
                   | if ($expected | map(select(.subject == $o.subject)) | length) == 0
                     then "unexpected record: \($o.subject)" else empty end ]
               + [ $observed[] | . as $o
                   | ($expected | map(select(.subject == $o.subject)) | first) as $e
                   | if $e != null and $e.conforms != $o.conforms
                     then "verdict \($o.conforms) where \($e.conforms) was required: \($o.subject)"
                     else empty end ]) as $problems
            | {id: $row.id, family: $row.family,
               status: (if ($problems | length) == 0 then "reconciled" else "unreconciled" end),
               failing_broken_detected: (($spec.violates // []) | length > 0),
               reason: ($spec.reason // null),
               expected: $expected, observed: $observed, problems: $problems}
          end)) as $results
    | ($records | map(.id) | unique | map(select(. as $i | $ids | index($i) | not))) as $unknown
    | ($t[0]) as $tests
    | ($tests | map(select(.status == "passed") | .test)) as $passed
    | {total_rows: ($ids | length),
       tests_executed: ($tests | length),
       tests_failed: ($tests | map(select(.status != "passed") | "\(.binary) \(.test)")),
       records_from_unexecuted_tests:
         (if $check_tests
          then ($records | map(select(.test as $n | ($tests | map(.test) | index($n)) | not) | "\(.id)@\(.subject)"))
          else [] end),
       reconciled: ($results | map(select(.status == "reconciled")) | length),
       unreconciled: ($results | map(select(.status != "reconciled")) | map(.id)),
       unknown_ids: $unknown,
       duplicate_manifest_ids: ($ids | group_by(.) | map(select(length > 1) | .[0])),
       rows: $results}' >"$summary.tmp" || {
    printf 'native-campaign: reconciliation could not be computed\n' >&2
    rm -f "$records" "$summary.tmp"
    return 1
  }
  local row id
  while IFS= read -r row; do
    id="$(jq -r '.id' <<<"$row")"
    if [ -e "$receipts/$id.json" ]; then
      printf 'native-campaign: receipt already exists (second writer): %s\n' "$id" >&2
      rm -f "$records" "$summary.tmp"
      [ -z "${own_tests:-}" ] || rm -f "$tests_file"
      return 1
    fi
    jq '.' <<<"$row" >"$receipts/$id.json"
    chmod a-w "$receipts/$id.json"
  done < <(jq -c '.rows[]' "$summary.tmp")
  jq 'del(.rows) + {rows_receipted: (.rows | length)}' "$summary.tmp" >"$summary"
  rm -f "$records"
  rm -f "$summary.tmp"
  [ -z "${own_tests:-}" ] || rm -f "$tests_file"
  jq -e '(.unreconciled | length) == 0 and (.unknown_ids | length) == 0
         and (.duplicate_manifest_ids | length) == 0
         and (.tests_failed | length) == 0
         and (.records_from_unexecuted_tests | length) == 0
         and .reconciled == .total_rows' "$summary" >/dev/null
}

# Compare a nextest listing with the manifest's protected identities one by one:
# present, and with the same ignored flag. New identities are counted, never
# allowed to stand in for a missing one.
r01_into() {
  local listing="$1" manifest_file="$2" out="$3"
  jq -n --slurpfile m "$manifest_file" --slurpfile l "$listing" '
    ($l[0]["rust-suites"]) as $suites
    | [ $m[0].R01[] | . as $e
        | ($suites[$e.binary].testcases[$e.test] // null) as $c
        | {binary: $e.binary, test: $e.test, expected_ignored: $e.ignored,
           observed_ignored: ($c.ignored // null),
           status: (if $c == null then "missing"
                    elif $c.ignored != $e.ignored then "ignored-flag-changed"
                    else "preserved" end)} ] as $rows
    | ([ $suites | to_entries[] | .key as $b | .value.testcases | keys[] | "\($b)\t\(.)" ]) as $listed
    | ([ $m[0].R01[] | "\(.binary)\t\(.test)" ]) as $protected
    | {total: ($rows | length),
       preserved: ($rows | map(select(.status == "preserved")) | length),
       not_preserved: ($rows | map(select(.status != "preserved"))),
       new_identities: ($listed - $protected | length),
       identities: $rows}' >"$out" || return 1
  jq -e '.not_preserved | length == 0' "$out" >/dev/null
}

cmd_r01() {
  [ "${1:-}" = "--out" ] && [ -n "${2:-}" ] || usage
  [ ! -e "$2" ] || die "output exists (one writer): $2"
  local listing
  listing="$(mktemp)"
  (cd "$root" && cargo nextest list --locked --workspace --message-format json 2>/dev/null) >"$listing"
  local status=0
  r01_into "$listing" "$root/$manifest" "$2" || status=1
  rm -f "$listing"
  jq -c '{total, preserved, new_identities, not_preserved: (.not_preserved | length)}' "$2"
  return "$status"
}

cmd_run() {
  local out=""
  while [ $# -gt 0 ]; do
    case "$1" in
      --out)
        out="$2"
        shift 2
        ;;
      *) usage ;;
    esac
  done
  [ -n "$out" ] || usage
  [ ! -e "$out" ] || die "campaign directory exists (one writer, never reused): $out"
  mkdir -p "$out/rows" "$out/logs" "$out/junit"
  out="$(cd "$out" && pwd)"
  cd "$root"

  printf 'native-campaign: building the private fixtures\n'
  task native-entry:build >"$out/logs/build.txt" 2>&1 || die "build failed; see $out/logs/build.txt"
  domain_to "$out/domain.json"
  digest_to "$out/subjects-before.json"

  export K40_ROW_DIR="$out/rows"
  local failed=0
  for step in front runner start; do
    printf 'native-campaign: native-entry:%s\n' "$step"
    if ! NEXTEST_PROFILE="campaign-$step" task "native-entry:$step" >"$out/logs/$step.txt" 2>&1; then
      printf 'native-campaign: native-entry:%s FAILED; see %s\n' "$step" "$out/logs/$step.txt" >&2
      failed=1
    fi
    if [ -f "target/nextest/campaign-$step/junit.xml" ]; then
      cp "target/nextest/campaign-$step/junit.xml" "$out/junit/$step.xml"
    fi
  done
  unset K40_ROW_DIR

  digest_to "$out/subjects-after.json"
  local drift=0
  compare "$out/subjects-before.json" "$out/subjects-after.json" || drift=1
  local reconciled=0
  tests_json_to "$out/tests.json" "$out"/junit/*.xml
  reconcile_into "$out/rows" "$root/$manifest" "$root/$plan" "$out/receipts" "$out/reconciliation.json" \
    "$out/tests.json" || reconciled=1
  jq -n --argjson failed "$failed" --argjson drift "$drift" --argjson unreconciled "$reconciled" \
    '{native_steps_failed: ($failed == 1), subjects_moved: ($drift == 1),
      rows_unreconciled: ($unreconciled == 1)}' >"$out/verdict.json"
  cat "$out/verdict.json"
  [ "$failed" -eq 0 ] && [ "$drift" -eq 0 ] && [ "$reconciled" -eq 0 ]
}

cmd_reconcile() {
  [ "$1" = "--out" ] && [ -n "${2:-}" ] || usage
  local out="$2"
  [ -d "$out/rows" ] || die "no rows directory in $out"
  [ ! -e "$out/receipts" ] || die "receipts already written (one writer): $out/receipts"
  local tests=""
  if compgen -G "$out/junit/*.xml" >/dev/null; then
    tests_json_to "$out/tests.json" "$out"/junit/*.xml
    tests="$out/tests.json"
  fi
  reconcile_into "$out/rows" "$root/$manifest" "$root/$plan" "$out/receipts" "$out/reconciliation.json" "$tests"
}

# Failing controls. A guarantee is credited only after the same instrument is
# seen to reject a deliberately wrong input in the way it names.
cmd_selftest() {
  local work
  work="$(mktemp -d)"
  trap 'chmod -R u+w "$work" 2>/dev/null; rm -rf "$work"' RETURN
  local m="$work/manifest.json" p="$work/plan.json"
  jq -n '{rows: [
      {id: "X01__a=1", family: "X01"}, {id: "X01__a=2", family: "X01"},
      {id: "Y01__b=1", family: "Y01"}]}' >"$m"
  jq -n '{families: {
      X01: {repaired: ["repaired"], violates: ["broken:one"]},
      Y01: {repaired: ["repaired"], conforms: ["broken:neg"], reason: "negative control"}}}' >"$p"

  fresh() { # fresh NAME: a complete, correct record set in $work/NAME/rows
    local rows="$work/$1/rows"
    mkdir -p "$rows"
    local id subject conforms
    while IFS=' ' read -r id subject conforms; do
      jq -n --arg id "$id" --arg s "$subject" --argjson c "$conforms" \
        '{schema: 1, id: $id, subject: $s, conforms: $c, test: "t", note: ""}' \
        >"$rows/$id@$subject.json"
    done <<'RECORDS'
X01__a=1 repaired true
X01__a=1 broken:one false
X01__a=2 repaired true
X01__a=2 broken:one false
Y01__b=1 repaired true
Y01__b=1 broken:neg true
RECORDS
  }
  expect_pass() {
    fresh "$1"
    reconcile_into "$work/$1/rows" "$m" "$p" "$work/$1/receipts" "$work/$1/summary.json" ||
      die "selftest: the correct record set was rejected"
    printf 'selftest ok   %s\n' "$2"
  }
  expect_fail() { # expect_fail NAME MUTATOR NEEDLE DESCRIPTION
    fresh "$1"
    "$2" "$work/$1/rows"
    if reconcile_into "$work/$1/rows" "$m" "$p" "$work/$1/receipts" "$work/$1/summary.json"; then
      die "selftest: $4 was accepted"
    fi
    grep -q "$3" "$work/$1/summary.json" "$work/$1/receipts"/*.json 2>/dev/null ||
      die "selftest: $4 failed, but not as '$3'"
    printf 'selftest ok   %s (rejected)\n' "$4"
  }
  drop_record() { rm "$1/X01__a=2@broken:one.json"; }
  add_unknown() {
    jq -n '{schema: 1, id: "Z99__q=1", subject: "repaired", conforms: true, test: "t", note: ""}' \
      >"$1/Z99__q=1@repaired.json"
  }
  flip_verdict() {
    jq '.conforms = true' "$1/X01__a=1@broken:one.json" >"$1/tmp" && mv "$1/tmp" "$1/X01__a=1@broken:one.json"
  }
  extra_subject() {
    jq -n '{schema: 1, id: "X01__a=1", subject: "broken:other", conforms: false, test: "t", note: ""}' \
      >"$1/X01__a=1@broken:other.json"
  }
  negative_violates() {
    jq '.conforms = false' "$1/Y01__b=1@broken:neg.json" >"$1/tmp" && mv "$1/tmp" "$1/Y01__b=1@broken:neg.json"
  }

  expect_pass good "complete and correct records"
  expect_fail missing drop_record "missing record: broken:one" "a missing record"
  expect_fail unknown add_unknown "Z99__q=1" "an identity outside the manifest"
  expect_fail flipped flip_verdict "verdict true where false was required" "a broken subject that conformed"
  expect_fail extra extra_subject "unexpected record: broken:other" "a record the plan does not expect"
  expect_fail negative negative_violates "verdict false where true was required" "a negative control that violated"

  # The real plan must resolve every real manifest row: with no records at all
  # each row is unreconciled for missing records, and none is unplanned.
  mkdir -p "$work/real/rows"
  if reconcile_into "$work/real/rows" "$root/$manifest" "$root/$plan" "$work/real/receipts" \
    "$work/real/summary.json" 2>/dev/null; then
    die "selftest: an empty record set reconciled against the real manifest"
  fi
  jq -s -e '
    (map(select(.status == "unplanned")) | length) == 0
    and length == 153
    and (map(.problems | length) | all(. > 0))' "$work/real/receipts"/*.json >/dev/null ||
    die "selftest: the plan does not resolve every manifest row"
  printf 'selftest ok   the plan resolves all %s manifest rows to %s expected records\n' \
    "$(jq -s 'length' "$work/real/receipts"/*.json)" \
    "$(jq -s 'map(.expected | length) | add' "$work/real/receipts"/*.json)"

  # A second writer for one receipt is refused rather than overwritten.
  fresh twice
  reconcile_into "$work/twice/rows" "$m" "$p" "$work/twice/receipts" "$work/twice/summary.json"
  if reconcile_into "$work/twice/rows" "$m" "$p" "$work/twice/receipts" "$work/twice/again.json" 2>/dev/null; then
    die "selftest: a second writer overwrote a receipt"
  fi
  printf 'selftest ok   a second writer for one receipt (rejected)\n'

  # Test-level reconciliation: a failed test, and a record whose test never ran.
  fresh tests
  jq -n '[{binary: "b", test: "t", status: "passed"}]' >"$work/tests-ok.json"
  reconcile_into "$work/tests/rows" "$m" "$p" "$work/tests/receipts" "$work/tests/summary.json" \
    "$work/tests-ok.json" || die "selftest: a passing test list was rejected"
  jq -n '[{binary: "b", test: "t", status: "failed"}]' >"$work/tests-failed.json"
  fresh failedtest
  if reconcile_into "$work/failedtest/rows" "$m" "$p" "$work/failedtest/receipts" \
    "$work/failedtest/summary.json" "$work/tests-failed.json"; then
    die "selftest: a failed test was accepted"
  fi
  jq -e '.tests_failed | length == 1' "$work/failedtest/summary.json" >/dev/null ||
    die "selftest: the failed test was not named"
  jq -n '[{binary: "b", test: "other", status: "passed"}]' >"$work/tests-other.json"
  fresh unexecuted
  if reconcile_into "$work/unexecuted/rows" "$m" "$p" "$work/unexecuted/receipts" \
    "$work/unexecuted/summary.json" "$work/tests-other.json"; then
    die "selftest: records from a test that never ran were accepted"
  fi
  jq -e '.records_from_unexecuted_tests | length == 6' "$work/unexecuted/summary.json" >/dev/null ||
    die "selftest: unexecuted-test records were not named"
  printf 'selftest ok   a failed test, and records from a test that never ran (rejected)\n'

  # The protected identities: all present, one missing, one with a flipped flag.
  jq -n '{R01: [{binary: "b", test: "kept", ignored: false}, {binary: "b", test: "fixture", ignored: true}]}' \
    >"$work/r01-manifest.json"
  jq -n '{"rust-suites": {b: {testcases: {kept: {ignored: false}, fixture: {ignored: true}, added: {ignored: false}}}}}' \
    >"$work/r01-listing.json"
  r01_into "$work/r01-listing.json" "$work/r01-manifest.json" "$work/r01-ok.json" ||
    die "selftest: preserved identities were rejected"
  jq -e '.new_identities == 1 and .preserved == 2' "$work/r01-ok.json" >/dev/null ||
    die "selftest: new identities were not counted apart from protected ones"
  jq -n '{"rust-suites": {b: {testcases: {fixture: {ignored: true}, added: {ignored: false}}}}}' \
    >"$work/r01-missing.json"
  if r01_into "$work/r01-missing.json" "$work/r01-manifest.json" "$work/r01-gone.json"; then
    die "selftest: a missing protected identity was masked by a new one"
  fi
  jq -e '.not_preserved[0].status == "missing"' "$work/r01-gone.json" >/dev/null ||
    die "selftest: the missing identity was not named"
  jq -n '{"rust-suites": {b: {testcases: {kept: {ignored: true}, fixture: {ignored: true}}}}}' \
    >"$work/r01-flipped.json"
  if r01_into "$work/r01-flipped.json" "$work/r01-manifest.json" "$work/r01-flag.json"; then
    die "selftest: a changed ignored flag was accepted"
  fi
  jq -e '.not_preserved[0].status == "ignored-flag-changed"' "$work/r01-flag.json" >/dev/null ||
    die "selftest: the changed flag was not named"
  printf 'selftest ok   protected identities: a missing one and a changed ignored flag (rejected)\n'

  # Digest drift: one changed subject, one appearing, one vanishing.
  jq -n '{"source:a": "1", "source:b": "2"}' >"$work/before.json"
  compare "$work/before.json" "$work/before.json" || die "selftest: identical digests rejected"
  jq -n '{"source:a": "1", "source:b": "9"}' >"$work/moved.json"
  if compare "$work/before.json" "$work/moved.json" 2>/dev/null; then die "selftest: a changed subject was accepted"; fi
  jq -n '{"source:a": "1"}' >"$work/gone.json"
  if compare "$work/before.json" "$work/gone.json" 2>/dev/null; then die "selftest: a vanished subject was accepted"; fi
  jq -n '{"source:a": "1", "source:b": "2", "source:c": "3"}' >"$work/new.json"
  if compare "$work/before.json" "$work/new.json" 2>/dev/null; then die "selftest: a new subject was accepted"; fi
  printf 'selftest ok   digest drift: changed, vanished and appeared subjects (rejected)\n'
}

[ $# -ge 1 ] || usage
mode="$1"
shift
case "$mode" in
  run) cmd_run "$@" ;;
  reconcile) cmd_reconcile "$@" ;;
  digest)
    [ "${1:-}" = "--out" ] && [ -n "${2:-}" ] || usage
    digest_to "$2"
    ;;
  compare)
    [ $# -eq 2 ] || usage
    compare "$1" "$2"
    ;;
  r01) cmd_r01 "$@" ;;
  selftest) cmd_selftest ;;
  *) usage ;;
esac
