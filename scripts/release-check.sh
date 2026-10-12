#!/usr/bin/env bash
# Save full instrument diagnostics and emit a concise summary.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p target
case "${1:-}" in
  archives) command=(bash scripts/release-archives.test.sh) ;;
  lifecycle) command=(bash scripts/release-lifecycle.test.sh) ;;
  formula) command=(bash scripts/release-formula.sh target/archives) ;;
  *)
    echo 'usage: release-check.sh archives | formula | lifecycle' >&2
    exit 2
    ;;
esac
log="$(mktemp "$root/target/release-$1.XXXXXX")"
if "${command[@]}" >"$log" 2>&1; then
  printf 'release: %s passed; complete log: %s\n' "$1" "$log"
else
  printf 'release: %s FAILED; complete log: %s\n' "$1" "$log" >&2
  tail -n 60 "$log" >&2
  exit 1
fi
