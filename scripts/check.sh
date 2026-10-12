#!/usr/bin/env bash
# Preserve complete diagnostics while keeping successful checks cheap to read.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p target
log="$(mktemp "$root/target/check.XXXXXX")"

run_check() {
  local label="$1"
  shift
  printf '\n=== %s\n' "$label" >>"$log"
  if "$@" >>"$log" 2>&1; then
    printf 'check: %s passed\n' "$label"
  else
    printf 'check: %s FAILED; full log: %s\n' "$label" "$log" >&2
    tail -n 60 "$log" >&2
    return 1
  fi
}

run_check formatting cargo fmt --all --check
run_check scripts shellcheck scripts/check.sh
run_check lint cargo clippy --locked --workspace --all-targets
printf 'check: complete log: %s\n' "$log"
# Keep nextest's own reporting visible; do not wrap its UI in log scraping.
# https://nexte.st/docs/running/
cargo nextest run --locked --workspace 2>&1 | tee -a "$log"
