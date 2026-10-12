#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p target
log="$(mktemp "$root/target/worker-check.XXXXXX")"
if bash crates/ravel/scripts/dispatch.sh "$@" >"$log" 2>&1; then
  printf 'worker: %s passed; complete log: %s\n' "$1" "$log"
else
  printf 'worker: %s FAILED; complete log: %s\n' "$1" "$log" >&2
  tail -n 60 "$log" >&2
  exit 1
fi
