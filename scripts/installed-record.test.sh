#!/usr/bin/env bash
# Test scripts/installed-record.sh. The record that the formula writes passes,
# and each wrong record fails, so a check that read clean for every input
# would fail here. The formula itself is exercised by task release:formula.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
check="$root/scripts/installed-record.sh"
work="$(mktemp -d "${TMPDIR:-/tmp}/installed-record-test.XXXXXX")"
trap 'rm -rf "$work"' EXIT
fail() { printf 'installed-record.test: %s\n' "$*" >&2; exit 1; }
directory='/opt/home brew/share/ravel/skills'

# record NAME CONTENT: a prefix whose record holds CONTENT exactly.
record() {
  mkdir -p "$work/$1/share/ravel"
  printf '%s' "$2" >"$work/$1/share/ravel/installation.json"
}
passes() { bash "$check" "$work/$1" "$directory" >/dev/null 2>&1 || fail "$1 failed"; }
fails() { ! bash "$check" "$work/$1" "$directory" >/dev/null 2>&1 || fail "$1 passed"; }

# The bytes of the formula's JSON.generate and newline.
record formula '{"schemaVersion":1,"skillDirectory":"/opt/home brew/share/ravel/skills"}
'
passes formula
record other-directory '{"schemaVersion":1,"skillDirectory":"/usr/local/share/ravel/skills"}
'
fails other-directory
record repeated-member '{"schemaVersion":1,"skillDirectory":"/x","skillDirectory":"/opt/home brew/share/ravel/skills"}
'
fails repeated-member
record extra-member '{"schemaVersion":1,"skillDirectory":"/opt/home brew/share/ravel/skills","x":1}
'
fails extra-member
record schema-2 '{"schemaVersion":2,"skillDirectory":"/opt/home brew/share/ravel/skills"}
'
fails schema-2
mkdir -p "$work/absent/share/ravel"
fails absent
mkdir -p "$work/directory/share/ravel/installation.json"
fails directory
! bash "$check" "$work/formula" relative/skills >/dev/null 2>&1 || fail 'a relative directory passed'
echo 'installed-record.test: the formula record passed and 7 wrong records failed'
