#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
readonly harness="$here/freeze-harness.sh"
test_root=$(mktemp -d /private/tmp/grove-transfer-freeze-test.XXXXXX)
readonly test_root
trap 'rm -rf "$test_root"' EXIT

printf '{}\n' >"$test_root/auth.json"
printf 'Return exactly: frozen\n' >"$test_root/prompt.txt"

if [[ ! -x "$harness" ]]; then
  printf 'FAIL: transfer freeze harness is not executable\n' >&2
  exit 1
fi

"$harness" init "$test_root/context" "$test_root/auth.json"

[[ -f "$test_root/context/codex-home/auth.json" ]]
[[ ! -e "$test_root/context/codex-home/skills" ]]
[[ $(wc -l <"$test_root/context/home-pre.manifest.tsv" | tr -d ' ') == 1 ]]
[[ -d "$test_root/context/run-directory" ]]

printf 'PASS: transfer freeze harness initializes a sealed no-skill context\n'
