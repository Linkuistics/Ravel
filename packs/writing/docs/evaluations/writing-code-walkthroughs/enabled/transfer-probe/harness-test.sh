#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
readonly harness="$here/harness.sh"
test_root=$(mktemp -d /private/tmp/grove-transfer-harness-test.XXXXXX)
readonly test_root
trap 'rm -rf "$test_root"' EXIT

printf '{}\n' >"$test_root/auth.json"

if [[ ! -x "$harness" ]]; then
  printf 'FAIL: transfer paired harness is not executable\n' >&2
  exit 1
fi

expected_skill_sha=$(awk -F '\t' '$1 == "SKILL.md" { print $3 }' "$here/skill.manifest.tsv")
readonly expected_skill_sha
grep -q "readonly expected_skill_sha=$expected_skill_sha" "$harness"

if "$harness" init "$test_root/campaign" "$test_root/auth.json"; then
  printf 'FAIL: historical transfer harness accepted different skill bytes\n' >&2
  exit 1
fi
[[ ! -e "$test_root/campaign" ]]

printf 'PASS: historical transfer harness preserves its executed skill guard\n'
