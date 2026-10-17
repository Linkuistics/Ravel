#!/usr/bin/env bash
# k110 prototype: crisp and hand-drawn renders of the two trial figures on the
# engine primitives. Throwaway, like the rest of this directory.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root="$here/.."
tools="$here/../../../target/tools"
out="$here/out-sketch"; mkdir -p "$out"
for f in successful-session process-authority; do
  for r in 0 1 2 3; do
    "$tools/typst" compile --format svg --root "$root" --ignore-system-fonts --ignore-embedded-fonts \
      --font-path "$tools/fonts/dejavu-2.37" --package-cache-path "$tools/typst-packages" \
      --input roughness=$r --input figure=$f "$here/sk-$f.typ" "$out/$f-r$r.svg"
  done
done
