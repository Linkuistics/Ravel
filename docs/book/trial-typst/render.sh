#!/usr/bin/env bash
# Regenerate the trial SVGs from the single Typst sources. Pins: see README.md.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
fonts="$here/../../../target/tools/fonts/dejavu-2.37"
out="${1:-$here/out}"
mkdir -p "$out"
run() { # name source [typst args]
  typst compile --ignore-system-fonts --font-path "$fonts" --format svg "${@:3}" "$here/$2" "$out/$1.svg"
}
run successful-session successful-session.typ
run successful-session-long successful-session.typ --input variant=long
run process-authority process-authority.typ
run process-authority-long process-authority.typ --input variant=long
