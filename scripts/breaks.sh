#!/usr/bin/env bash
# Report each run of prose in the book that is longer than the limit of
# docs/book/contract.md, section 2, "Breaks in prose" (scripts/breaks.ts does
# the work). With no file arguments it examines every chapter under
# docs/book/src/, so chapters 7-12 are examined when they exist.
# --strict makes a long run fail the run; without it the run is a report.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun="${BOOK_TOOLS_DIR:-$root/target/tools}/bun"

[[ -x "$bun" ]] || {
  echo "breaks: run task setup first (no pinned Bun at $bun)" >&2
  exit 1
}

args=()
files=()
for a in "$@"; do
  if [[ "$a" == --strict ]]; then args+=("$a"); else files+=("$a"); fi
done
if ((${#files[@]} == 0)); then
  cd "$root"
  shopt -s nullglob
  files=(docs/book/src/[0-9][0-9]-*.md)
  ((${#files[@]} > 0)) || {
    echo "breaks: no chapter under docs/book/src/" >&2
    exit 1
  }
fi

exec "$bun" "${BREAKS_TS:-$root/scripts/breaks.ts}" "${args[@]+"${args[@]}"}" "${files[@]}" # BREAKS_TS: a mutated copy for the controls
