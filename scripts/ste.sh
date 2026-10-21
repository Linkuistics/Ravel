#!/usr/bin/env bash
# Examine the book's text against the STE writing rules that a script can
# decide and the word decisions in docs/book/ste.md (scripts/ste.ts does the
# work). With no file arguments it examines the book's text: every chapter
# under docs/book/src/ (so chapters 7-11 are examined when they exist),
# SUMMARY.md, CONTEXT.md, README.md and each figure's .layout.json record.
# --strict makes a problem fail the run; without it the run is a report.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bun="${BOOK_TOOLS_DIR:-$root/target/tools}/bun"
decisions="${STE_DECISIONS:-$root/docs/book/ste.md}" # STE_DECISIONS and STE_TS: mutated copies for the controls

[[ -x "$bun" ]] || {
  echo "ste: run task setup first (no pinned Bun at $bun)" >&2
  exit 1
}

args=()
files=()
for a in "$@"; do
  if [[ "$a" == --strict ]]; then args+=("$a"); else files+=("$a"); fi
done
if ((${#files[@]} == 0)); then
  cd "$root"
  files=(docs/book/src/[0-9][0-9]-*.md docs/book/src/SUMMARY.md CONTEXT.md README.md docs/book/src/figures/*.layout.json)
fi

exec "$bun" "${STE_TS:-$root/scripts/ste.ts}" "$decisions" "${args[@]+"${args[@]}"}" "${files[@]}"
