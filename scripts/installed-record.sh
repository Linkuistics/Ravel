#!/usr/bin/env bash
# Check the installation record that a Homebrew formula of Ravel wrote under
# PREFIX: exactly schemaVersion 1 and SKILL_DIRECTORY, in the formula's bytes.
# docs/specs/skill-discovery.md, "The installation record", owns the contract.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "installed-record: $*" >&2; exit 1; }
[[ $# == 2 && -n "$1" && "$2" == /* ]] || fail 'usage: installed-record.sh PREFIX ABSOLUTE_SKILL_DIRECTORY'
record="$1/share/ravel/installation.json"
[[ -f "$record" ]] || fail "$record is not a regular file; reinstall the formula"
# Compare bytes, not a parse: a parser that keeps the last of two members
# would accept a record that Ravel refuses.
expected="$(jq -cn --arg directory "$2" '{schemaVersion: 1, skillDirectory: $directory}')"
printf '%s\n' "$expected" | cmp -s - "$record" ||
  fail "$record holds $(head -c 512 "$record"), not $expected"
echo "installed-record: $record names $2"
