#!/usr/bin/env bash
# Controls for scripts/figures.sh. Every refusal below is made to fire against a
# deliberately wrong subject, and the unchanged fixtures are the one pass:
# a check that never failed would read the same as a check that cannot.
#
# Part 1 drives the fetch-and-verify function offline, emulating only `curl`.
# Part 2 runs the real renderer (needs `task setup`) over the committed fixture
# in scripts/figures-fixtures and over mutated copies of it, and renders the
# book's own Typst modules (_style.typ, _layout.typ, _draw.typ, _sequence.typ)
# with the pinned Typst.
# Part 3 builds one page with an embedded figure through mdBook.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script="${FIGURES_SCRIPT:-$root/scripts/figures.sh}" # a mutated copy, when the controls are themselves tested
fixtures="$root/scripts/figures-fixtures/figures"
tools="$root/target/tools"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT

die() {
  echo "figures-test: $*" >&2
  exit 1
}
ok() { printf 'figures-test: ok: %s\n' "$*"; }

[[ -x "$tools/typst" && -d "$tools/typst-packages/preview/cetz" ]] || die 'run task setup before the figure controls'
[[ -x "$tools/mdbook" ]] || die 'run task setup (mdBook is needed for the page control)'

# ---------------------------------------------------------------- part 1: fetch

fake_fetch() { # TOOLS MODE NAME DIGEST: run fetch_verified with an emulated curl
  local t="$1" mode="$2" name="$3" digest="$4"
  (
    # shellcheck disable=SC2030 # each subshell sets its own tools directory
    export BOOK_TOOLS_DIR="$t"
    # shellcheck disable=SC1090 # the production script, sourced for its definitions
    source "$script"
    # shellcheck disable=SC2329 # shadows curl for the sourced fetch_verified
    curl() {
      local output=""
      while (($#)); do
        if [[ "$1" == --output ]]; then output="$2"; shift 2; else shift; fi
      done
      echo fetched >>"$t/fetches"
      case "$mode" in
        good) printf 'the pinned release asset' >"$output" ;;
        corrupt) printf 'a corrupt download' >"$output" ;;
        none) return 22 ;;
      esac
    }
    make_scratch
    fetch_verified "https://example.invalid/$name" "$name" "$digest" "$SCRATCH"
  )
}
fetches() { if [[ -f "$1/fetches" ]]; then wc -l <"$1/fetches" | tr -d ' '; else echo 0; fi; }
good_digest="$(printf 'the pinned release asset' | shasum -a 256 | cut -d' ' -f1)"
asset=asset.bin

t="$work/f-fresh"; mkdir -p "$t"
fake_fetch "$t" good "$asset" "$good_digest" >/dev/null 2>"$work/err" || die "fresh fetch failed: $(cat "$work/err")"
[[ "$(fetches "$t")" == 1 && -f "$t/$asset" ]] || die 'a fresh fetch did not fetch once and cache'
fake_fetch "$t" none "$asset" "$good_digest" >/dev/null 2>"$work/err" || die "cache reuse failed: $(cat "$work/err")"
[[ "$(fetches "$t")" == 1 ]] || die 'a verified cache was fetched again'
ok 'fresh fetch caches once and a verified cache is reused without a fetch'

t="$work/f-corrupt-cache"; mkdir -p "$t"
printf 'a corrupt cached asset' >"$t/$asset"
fake_fetch "$t" good "$asset" "$good_digest" >/dev/null 2>"$work/err" || die "corrupt cache did not recover: $(cat "$work/err")"
[[ "$(fetches "$t")" == 1 && "$(shasum -a 256 <"$t/$asset" | cut -d' ' -f1)" == "$good_digest" ]] ||
  die 'a corrupt cache was not refetched exactly once and repaired'
ok 'a corrupt cached asset is fetched again once and repaired'

t="$work/f-corrupt-download"; mkdir -p "$t"
if fake_fetch "$t" corrupt "$asset" "$good_digest" >/dev/null 2>"$work/err"; then die 'a corrupt download was accepted'; fi
grep -Fq 'refusing to use it' "$work/err" || die "corrupt download failed for the wrong reason: $(cat "$work/err")"
[[ ! -e "$t/$asset" ]] || die 'a refused download was cached'
ok 'a corrupt download is refused for its digest and not cached'

t="$work/f-both"; mkdir -p "$t"
printf 'a corrupt cached asset' >"$t/$asset"
if fake_fetch "$t" corrupt "$asset" "$good_digest" >/dev/null 2>"$work/err"; then die 'a corrupt cache and download were accepted'; fi
grep -Fq 'refusing to use it' "$work/err" || die 'corrupt cache and download failed for the wrong reason'
ok 'a corrupt cache is not trusted as a fallback for a corrupt download'

t="$work/f-wrong-pin"; mkdir -p "$t"
if fake_fetch "$t" good "$asset" 0000000000000000000000000000000000000000000000000000000000000000 >/dev/null 2>"$work/err"; then
  die 'a wrong pin accepted the asset'
fi
grep -Fq 'refusing to use it' "$work/err" || die 'wrong pin failed for the wrong reason'
ok 'a wrong pinned digest refuses an asset that is otherwise intact'

t="$work/f-none"; mkdir -p "$t"
if fake_fetch "$t" none "$asset" "$good_digest" >/dev/null 2>"$work/err"; then die 'a failed download was accepted'; fi
grep -Fq 'cannot download' "$work/err" || die 'failed download failed for the wrong reason'
ok 'a failed download is reported as one'

# ------------------------------------------------------------ part 2: renderers

# figs MODE DIR [TOOLS]: run the script, leaving its output in $work/out and
# returning its exit status.
figs() {
  local mode="$1" dir="$2" tooldir="${3:-$tools}" status=0
  BOOK_TOOLS_DIR="$tooldir" FIGURES_DIR="$dir" bash "$script" "$mode" >"$work/out" 2>&1 || status=$?
  return "$status"
}
expect_ok() { # LABEL MODE DIR [TOOLS]
  local label="$1"
  shift
  figs "$@" || die "$label: expected success, got: $(cat "$work/out")"
  ok "$label"
}
expect_refused() { # LABEL PATTERN MODE DIR [TOOLS]
  local label="$1" pattern="$2"
  shift 2
  if figs "$@"; then die "$label: expected a refusal, but it passed: $(cat "$work/out")"; fi
  grep -Fq -- "$pattern" "$work/out" || die "$label: refused for the wrong reason (wanted '$pattern'): $(cat "$work/out")"
  ok "$label"
}
copy_fixtures() { # NAME -> prints the path of a fresh private copy
  rm -rf "${work:?}/$1"
  cp -R "$fixtures" "$work/$1"
  echo "$work/$1"
}

# The unchanged tree is the one pass, and rendering is reproducible: two
# independent renders of every figure are byte-identical to each other and to
# what is committed.
expect_ok 'the committed fixtures match what the pinned renderers produce' check "$fixtures"
(
  # shellcheck disable=SC2030,SC2031 # each subshell sets its own tools directory
  export BOOK_TOOLS_DIR="$tools"
  # shellcheck disable=SC1090
  source "$script"
  render_all "$fixtures" "$work/render-1"
  render_all "$fixtures" "$work/render-2"
)
for svg in "$fixtures"/*.svg; do
  name="$(basename "$svg")"
  cmp -s "$work/render-1/$name" "$work/render-2/$name" || die "two renders of $name differ: it is not reproducible"
  cmp -s "$work/render-1/$name" "$svg" || die "$name differs from a fresh render"
done
for json in "$fixtures"/*.layout.json; do
  name="$(basename "$json")"
  cmp -s "$work/render-1/$name" "$work/render-2/$name" || die "two evaluations of $name differ: the record is not reproducible"
  cmp -s "$work/render-1/$name" "$json" || die "$name differs from a fresh evaluation"
done
ok 'every fixture renders twice to identical bytes, equal to the committed SVG and layout record'

# Nothing about the caller's environment may reach a figure.
hostile="$work/hostile"
mkdir -p "$hostile"
(
  cd "$hostile"
  TYPST_FONT_PATHS=/nonexistent TYPST_PACKAGE_PATH=/nonexistent TYPST_PACKAGE_CACHE_PATH=/nonexistent \
    TYPST_ROOT=/nonexistent TYPST_FEATURES=html SOURCE_DATE_EPOCH=1 HTTPS_PROXY=http://127.0.0.1:1 \
    XDG_DATA_HOME="$hostile" XDG_CACHE_HOME="$hostile" \
    LANG=tr_TR.UTF-8 TZ=Asia/Kolkata FIGURES_DIR="$fixtures" BOOK_TOOLS_DIR="$tools" \
    bash "$script" check
) >"$work/out" 2>&1 || die "a hostile environment changed the result: $(cat "$work/out")"
ok 'TYPST_*, SOURCE_DATE_EPOCH, a proxy, XDG_*, locale and time zone change nothing'

# A changed source, a changed SVG, a missing SVG and an SVG without a source.
d="$(copy_fixtures source-changed)"
sed -i.bak 's/Ravel runs/Ravel stops/' "$d/layers.typ" && rm "$d/layers.typ.bak"
expect_refused 'a changed source fails the compare' 'layers.svg differs' check "$d"

d="$(copy_fixtures svg-changed)"
printf ' ' >>"$d/layers.svg"
expect_refused 'a changed SVG fails the compare' 'layers.svg differs' check "$d"

d="$(copy_fixtures svg-missing)"
rm "$d/layers.svg"
expect_refused 'a missing SVG fails' 'layers.svg is missing' check "$d"

d="$(copy_fixtures svg-orphan)"
cp "$d/layers.svg" "$d/gone.svg"
expect_refused 'an SVG with no source fails' 'gone.svg has no gone.typ source' check "$d"

# update rewrites what differs, then check passes, then update changes nothing.
d="$(copy_fixtures update)"
sed -i.bak 's/Ravel runs/Ravel stops/' "$d/layers.typ" && rm "$d/layers.typ.bak"
rm "$d/layers.svg"
expect_ok 'update rewrites a stale layout record and writes a missing SVG' update "$d"
grep -Fq 'rewrote layers.layout.json' "$work/out" || die "update did not report rewriting layers.layout.json: $(cat "$work/out")"
grep -Fq 'wrote layers.svg' "$work/out" || die "update did not report writing layers.svg: $(cat "$work/out")"
expect_ok 'check passes after update' check "$d"
before="$(cat "$d"/*.svg "$d"/*.layout.json | shasum -a 256)"
expect_ok 'a second update succeeds' update "$d"
if grep -Fq 'wrote' "$work/out"; then die 'a second update rewrote a file'; fi
[[ "$before" == "$(cat "$d"/*.svg "$d"/*.layout.json | shasum -a 256)" ]] || die 'a second update changed the figures'
ok 'update is idempotent'

# What is in the directory. A second editable source for a figure, in any other
# notation, is a file of another kind: there is one source per figure.
d="$(copy_fixtures second-source)"
printf 'graph TD\n' >"$d/layers.mmd"
expect_refused 'a second editable source beside a figure is refused' 'layers.mmd is not a .typ, .svg or .layout.json file' check "$d"
d="$(copy_fixtures module-svg)"
cp "$d/layers.svg" "$d/_draw.svg"
expect_refused 'a _*.typ module gets no SVG: one that has one is refused' '_draw.svg has no _draw.typ source' check "$d"
d="$(copy_fixtures stray)"
printf 'notes\n' >"$d/notes.txt"
expect_refused 'a file of another kind is refused' 'notes.txt is not a .typ, .svg or .layout.json file' check "$d"
d="$(copy_fixtures subdir)"
mkdir "$d/nested"
expect_refused 'a subdirectory is refused' 'nested is a directory' check "$d"

# A changed source and a changed SVG, in both of the ways a figure can differ.
# Typst writes no version stamp, so the message names the pinned versions instead.
d="$(copy_fixtures typst-source)"
sed -i.bak 's/Ravel runs/Ravel stops/' "$d/layers.typ" && rm "$d/layers.typ.bak"
expect_refused 'a changed Typst source fails the compare' 'layers.svg differs from what layers.typ renders with the pinned Typst 0.15.1 and CeTZ 0.4.2' check "$d"
d="$(copy_fixtures typst-module)"
sed -i.bak 's/#E8EDF2/#E8EDF3/' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'changing a shared Typst module changes the figures that use it' 'layers.svg differs' check "$d"
d="$(copy_fixtures typst-svg)"
printf ' ' >>"$d/layers.svg"
expect_refused 'a changed Typst SVG fails the compare' 'layers.svg differs' check "$d"
d="$(copy_fixtures typst-update)"
sed -i.bak 's/Ravel runs/Ravel stops/' "$d/layers.typ" && rm "$d/layers.typ.bak"
expect_ok 'update rewrites a stale Typst SVG' update "$d"
grep -Fq 'rewrote layers.svg' "$work/out" || die "update did not report rewriting layers.svg: $(cat "$work/out")"
expect_ok 'check passes after a Typst update' check "$d"

# Typst: the layout record, committed as <stem>.layout.json beside the SVG and
# compared apart from it. A moved element changes both; a restyle changes only
# the SVG, which is the point of keeping the record free of style.
not_in_output() { # LABEL PATTERN: the last run's output does not mention PATTERN
  if grep -Fq -- "$2" "$work/out"; then die "$1: output should not mention '$2': $(cat "$work/out")"; fi
}
d="$(copy_fixtures record-moved)"
sed -i.bak 's/x0: 200, y0: 0, x1: 320/x0: 210, y0: 0, x1: 320/' "$d/layers.typ" && rm "$d/layers.typ.bak"
expect_refused 'a moved element fails the layout record compare' "layers.layout.json differs from the layout record layers.typ evaluates to" check "$d"
grep -Fq 'layers.svg differs' "$work/out" || die "a moved element did not also fail the SVG compare: $(cat "$work/out")"
ok 'a moved element fails the SVG compare too'
d="$(copy_fixtures record-restyle)"
sed -i.bak 's/#E8EDF2/#E8EDF3/' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a restyle changes the SVG' 'layers.svg differs' check "$d"
not_in_output 'a restyle leaves the layout record alone' 'layers.layout.json'
d="$(copy_fixtures record-stale)"
sed -i.bak 's/"w": 320/"w": 321/' "$d/layers.layout.json" && rm "$d/layers.layout.json.bak"
expect_refused 'a stale layout record fails the compare' "layers.layout.json differs from the layout record layers.typ evaluates to; run 'task docs:figures:update'" check "$d"
not_in_output 'a stale layout record leaves the SVG alone' 'layers.svg'
d="$(copy_fixtures record-missing)"
rm "$d/layers.layout.json"
expect_refused 'a Typst figure with no layout record fails check' "layers.layout.json is missing; run 'task docs:figures:update'" check "$d"
expect_ok 'update writes a missing layout record' update "$d"
grep -Fq 'wrote layers.layout.json' "$work/out" || die "update did not report writing layers.layout.json: $(cat "$work/out")"
cmp -s "$d/layers.layout.json" "$fixtures/layers.layout.json" || die 'update wrote a layout record that differs from the committed one'
expect_ok 'check passes after the record is written' check "$d"
d="$(copy_fixtures record-update)"
sed -i.bak 's/"w": 320/"w": 321/' "$d/layers.layout.json" && rm "$d/layers.layout.json.bak"
expect_ok 'update rewrites a stale layout record' update "$d"
grep -Fq 'rewrote layers.layout.json' "$work/out" || die "update did not report rewriting layers.layout.json: $(cat "$work/out")"
cmp -s "$d/layers.layout.json" "$fixtures/layers.layout.json" || die 'update did not restore the layout record'
d="$(copy_fixtures record-orphan)"
cp "$fixtures/layers.layout.json" "$d/gone.layout.json"
expect_refused 'a layout record with no source fails' 'gone.layout.json has no gone.typ source' check "$d"
d="$(copy_fixtures record-orphan-module)"
cp "$fixtures/layers.layout.json" "$d/_draw.layout.json"
expect_refused 'a layout record for a module fails' '_draw.layout.json has no _draw.typ source' check "$d"
d="$(copy_fixtures record-unlabelled)"
sed -i.bak 's/\[#metadata(record) <layout>\]//' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a Typst figure that attaches no layout record fails, naming the figure' 'Typst could not evaluate the layout record of layers.typ' check "$d"

# A record is positions, roles and measured text. A colour, a length or a style
# key in one would make a restyle change it. Each shape of leak is refused, and
# update writes nothing for it.
leak() { # LABEL SED-EXPRESSION: a record element carrying a style value
  d="$(copy_fixtures "record-leak-$1")"
  rm "$d/layers.layout.json"
  sed -i.bak "s/dashed: false, arrow: true/dashed: false, arrow: true, $2/" "$d/layers.typ" && rm "$d/layers.typ.bak"
  expect_refused "a layout record holding $3 is refused" "layers.typ's layout record holds a style value" update "$d"
  [[ ! -e "$d/layers.layout.json" ]] || die "update wrote a layout record holding $3"
}
leak colour 'paint: rgb("#334155")' 'a colour'
leak length 'gap: 1.5pt' 'a length'
leak key 'stroke: "thin"' 'a style key'
leak content 'label: [hi]' 'a piece of content'

# Typst: the layout rules that keep a package or a host default out of a figure.
d="$(copy_fixtures typst-unpinned-version)"
sed -i.bak 's|@preview/cetz:0.4.2|@preview/cetz:0.4.1|' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a Typst module importing another version of a pinned package is refused' '@preview/cetz:0.4.1, which is not a pinned Typst package' check "$d"
d="$(copy_fixtures typst-unpinned-package)"
sed -i.bak '1a\
#import "@preview/oxifmt:0.3.0": strfmt
' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a Typst module importing an unpinned package is refused' '@preview/oxifmt:0.3.0, which is not a pinned Typst package' check "$d"
d="$(copy_fixtures typst-local-package)"
sed -i.bak '1a\
#import "@local/mine:1.0.0": x
' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a Typst module importing a local package is refused' '@local/mine:1.0.0, which is not a pinned Typst package' check "$d"
d="$(copy_fixtures typst-direct-import)"
sed -i.bak '1a\
#import "@preview/cetz:0.4.2"
' "$d/layers.typ" && rm "$d/layers.typ.bak"
expect_refused 'a Typst figure source importing a package itself is refused' 'layers.typ imports a package' check "$d"
d="$(copy_fixtures typst-direct-import-pinned-module)"
sed -i.bak '1a\
#import "@preview/cetz:0.4.2"
' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_ok '_draw.typ may import a pinned package' check "$d"
# Only _draw.typ imports CeTZ, so how every stroke is drawn can change in one
# place: a layout or style module that imports it is refused, pinned or not.
d="$(copy_fixtures typst-cetz-in-layout)"
cp "$root/scripts/figures-fixtures/cetz-in-layout/_layout.typ" "$d/"
expect_refused 'a Typst module other than _draw.typ importing CeTZ is refused' '_layout.typ imports CeTZ; only _draw.typ draws' check "$d"

# Typst: Typst reports an unknown font family as a warning and still renders, so
# a warning on stderr fails the figure instead of passing with a fallback face.
d="$(copy_fixtures typst-font)"
sed -i.bak 's/"DejaVu Sans"/"Nonexistent Sans"/' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
expect_refused 'a Typst figure in a font that is not on the font path is refused' 'Typst warned while rendering layers.typ' check "$d"

# Typst: version, package cache and font path are pins like the others.
t="$work/tools-typst-version"; mkdir -p "$t"
printf '#!/bin/sh\necho "typst 0.14.2 (0123abcd)"\n' >"$t/typst" && chmod +x "$t/typst"
expect_refused 'a Typst of another version is refused, naming both' "Typst 0.15.1 is pinned, found 'typst 0.14.2 (0123abcd)'" check "$fixtures" "$t"
rm "$t/typst"
expect_refused 'a missing Typst is refused with the remedy' "Typst 0.15.1 is not installed; run 'task setup'" check "$fixtures" "$t"

t="$work/tools-typst-nocache"; mkdir -p "$t"
ln -s "$tools/typst" "$t/typst"
expect_refused 'a missing Typst package cache is refused with the remedy' "the Typst package @preview/cetz:0.4.2 is not installed; run 'task setup'" check "$fixtures" "$t"
mkdir -p "$t/typst-packages"
expect_refused 'an empty Typst package cache is refused with the remedy' "the Typst package @preview/cetz:0.4.2 is not installed; run 'task setup'" check "$fixtures" "$t"
if [[ -n "$(ls -A "$t/typst-packages")" ]]; then die 'a refused check wrote into the empty package cache'; fi

t="$work/tools-typst-tampered"; mkdir -p "$t"
ln -s "$tools/typst" "$t/typst"
cp -R "$tools/typst-packages" "$t/typst-packages"
printf '\n' >>"$t/typst-packages/preview/cetz/0.4.2/src/lib.typ"
expect_refused 'a changed file in a pinned Typst package is refused' 'the Typst package @preview/cetz:0.4.2 has tree digest' check "$fixtures" "$t"
cp "$tools/typst-packages/preview/cetz/0.4.2/src/lib.typ" "$t/typst-packages/preview/cetz/0.4.2/src/lib.typ"
printf 'x' >"$t/typst-packages/preview/cetz/0.4.2/extra.typ"
expect_refused 'an added file in a pinned Typst package is refused' 'the Typst package @preview/cetz:0.4.2 has tree digest' check "$fixtures" "$t"
rm "$t/typst-packages/preview/cetz/0.4.2/extra.typ"
ln -s /etc/hosts "$t/typst-packages/preview/cetz/0.4.2/link.typ"
expect_refused 'a symlink in a pinned Typst package is refused' 'the Typst package @preview/cetz:0.4.2 has tree digest not-a-plain-tree' check "$fixtures" "$t"
rm "$t/typst-packages/preview/cetz/0.4.2/link.typ"
mkdir -p "$t/typst-packages/preview/cetz/0.5.2"
expect_refused 'another version of a pinned Typst package in the cache is refused' 'holds cetz/0.5.2, which is not pinned' check "$fixtures" "$t"
rmdir "$t/typst-packages/preview/cetz/0.5.2"
mkdir -p "$t/typst-packages/preview/other/1.0.0"
expect_refused 'an unpinned Typst package in the cache is refused' 'which is not pinned' check "$fixtures" "$t"
rm -r "${t:?}/typst-packages/preview/other"
mkdir -p "$t/typst-packages/local"
expect_refused 'a namespace other than preview in the package cache is refused' 'holds local, which is not pinned' check "$fixtures" "$t"
rmdir "$t/typst-packages/local"
ln -s "$tools/fonts" "$t/fonts"
expect_ok 'the same tools directory passes once the tamper is undone' check "$fixtures" "$t"

# Typst: a render itself reaches no network, whatever the lint and the cache
# checks above would have said. These call render_all directly, so the only
# thing between Typst and the registry is the render's own refusal.
render_directly() { # DIR PACKAGES: render_all with that package cache, output in $work/out-direct
  (
    # shellcheck disable=SC2030,SC2031 # each subshell sets its own tools directory
    export BOOK_TOOLS_DIR="$tools"
    # shellcheck disable=SC1090
    source "$script"
    # shellcheck disable=SC2034 # read by render_all in the sourced script
    packages="$2"
    render_all "$1" "$work/out-direct"
  )
}
mkdir -p "$work/empty-cache"
if render_directly "$fixtures" "$work/empty-cache" >/dev/null 2>"$work/err"; then die 'a Typst render with an empty package cache succeeded'; fi
grep -Fq 'failed to download package' "$work/err" || die "a render with an empty package cache failed for the wrong reason: $(cat "$work/err")"
grep -Fq 'Connection refused' "$work/err" || die "a render with an empty package cache did not stop at the refused connection: $(cat "$work/err")"
[[ -z "$(ls -A "$work/empty-cache")" ]] || die 'a render with an empty package cache downloaded into it'
ok 'a Typst render with an empty package cache fails at a refused connection and downloads nothing'

cp -R "$tools/typst-packages" "$work/full-cache"
d="$(copy_fixtures typst-render-unpinned)"
sed -i.bak '1a\
#import "@preview/oxifmt:0.3.0": strfmt
' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
if render_directly "$d" "$work/full-cache" >/dev/null 2>"$work/err"; then die 'a Typst render importing an unpinned package succeeded'; fi
grep -Fq 'failed to download package' "$work/err" || die "an unpinned import failed for the wrong reason: $(cat "$work/err")"
[[ ! -e "$work/full-cache/preview/oxifmt/0.3.0" ]] || die 'a render downloaded an unpinned package'
ok 'a Typst render importing an unpinned package is not downloaded'

# The font pin for Typst: with no fonts on the path the figure cannot render,
# rather than falling back to a face of Typst's own. Typst embeds DejaVu Sans
# Mono, so the figure is set in that face: it renders only if the embedded fonts
# are not ignored.
mkdir -p "$work/no-fonts-typst"
d="$(copy_fixtures typst-embedded-font)"
sed -i.bak 's/"DejaVu Sans"/"DejaVu Sans Mono"/' "$d/_draw.typ" && rm "$d/_draw.typ.bak"
if (
  # shellcheck disable=SC2031 # each subshell sets its own tools directory
  export BOOK_TOOLS_DIR="$tools"
  # shellcheck disable=SC1090
  source "$script"
  # shellcheck disable=SC2034 # read by render_all in the sourced script
  fontdir="$work/no-fonts-typst"
  render_all "$d" "$work/render-nofont-typst"
) >/dev/null 2>"$work/err"; then
  die 'a Typst figure rendered with no pinned fonts on the font path'
fi
grep -Fq 'layers.typ' "$work/err" || die "a render without fonts failed for the wrong reason: $(cat "$work/err")"
ok 'without the pinned fonts a Typst figure does not render, even in a face Typst embeds (the font path and the ignored embedded fonts hold)'

# The book's own Typst modules, with the pinned Typst and CeTZ. No bytes are
# committed for these: a change to the book's style is a change to every figure,
# so this checks that the modules compile, draw every kind of row, keep to the
# drawing primitives, render the same twice, and ship the sketched style with
# the key crisp and a seed per figure.
modules="$work/modules"
mkdir -p "$modules"
cp "$root/docs/book/src/figures"/_{style,layout,draw,sequence}.typ "$modules/"
cat >"$modules/engine.typ" <<'EOF'
#import "_sequence.typ": *
#import "_draw.typ": render

#render(sequence(
  [A fixture sequence],
  (
    part("a", [caller]),
    part("b", [Ravel CLI \ (runner inside)], kind: "ravel"),
    part("c", [result channel], kind: "temporary", db: true),
    part("d", [owner policy], kind: "owner"),
  ),
  (
    msg("a", "b", [start-session: kind and prompt]),
    msg("b", "b", [Check inputs and policy path]),
    div([Selection]),
    msg("b", "d", [Start in an empty private directory], create: true),
    msg("d", "b", [Identity; the CLI checks the match], dashed: true),
    destroy("d"),
    msg("b", "c", [Allocate a fresh result channel], create: true),
    note("b", "c", [First complete publication wins.]),
    msg("c", "b", [Exact opaque value], role: "result", dashed: true),
    destroy("c"),
  ),
  ([A key line.], [Another key line.]),
))
EOF
expect_ok 'the book modules draw a sequence with every kind of row' update "$modules"
expect_ok 'the book modules render a stable sequence' check "$modules"
[[ -s "$modules/engine.svg" && "$(head -c 4 "$modules/engine.svg")" == '<svg' ]] || die 'the book modules wrote no SVG'
[[ ! -e "$modules/_style.svg" && ! -e "$modules/_layout.svg" && ! -e "$modules/_draw.svg" && ! -e "$modules/_sequence.svg" ]] || die 'a book module was rendered as a figure'

# The shipped look is compiled in: figures.sh passes only the figure's name, and
# the SVG it writes equals sketch at roughness 2, not the crisp style.
direct() { # STEM OUT [INPUT...]: the same Typst call as render_all, with chosen inputs
  local stem="$1" out="$2" input
  shift 2
  local inputs=()
  for input in "$@"; do inputs+=(--input "$input"); done
  (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9 \
    "$tools/typst" compile --format svg --root "$modules" --ignore-system-fonts --ignore-embedded-fonts \
    --font-path "$tools/fonts/dejavu-2.37" --package-cache-path "$tools/typst-packages" "${inputs[@]}" \
    "$modules/$stem.typ" "$out") || die "Typst could not render $stem with inputs $*"
}
mkdir -p "$work/home"
direct engine "$work/engine-sketch.svg" figure=engine style=sketch roughness=2
direct engine "$work/engine-crisp.svg" figure=engine style=crisp
cmp -s "$modules/engine.svg" "$work/engine-sketch.svg" || die 'the shipped figure is not sketch at roughness 2'
if cmp -s "$modules/engine.svg" "$work/engine-crisp.svg"; then die 'the shipped figure is the crisp style'; fi
ok 'a figure rendered by figures.sh is sketch at roughness 2, and differs from the crisp style'

# Keys are never sketched, whatever the roughness: the key box is a pale box drawn
# as a single filled and outlined path, and a sketched box is its fill with the
# outline drawn apart. The note is pale too, so the crisp style has two such paths
# and the shipped style has one: the key.
count_solid() { grep -o '<path fill="#f8fafc"[^>]*stroke="#64748b"[^>]*>' "$1" | wc -l | tr -d ' '; }
[[ "$(count_solid "$work/engine-crisp.svg")" == 2 ]] || die "the crisp style should draw the key and the note as solid paths, found $(count_solid "$work/engine-crisp.svg")"
[[ "$(count_solid "$modules/engine.svg")" == 1 ]] || die "the shipped style should draw only the key as a solid path, found $(count_solid "$modules/engine.svg")"
ok 'in the shipped style the key box is drawn crisp and the note box is sketched'

# Each figure seeds its own sketch from its name: one source under two names
# gives two different drawings, and each is stable.
cp "$modules/engine.typ" "$modules/other.typ"
expect_ok 'the same source under another name renders' update "$modules"
if cmp -s "$modules/engine.svg" "$modules/other.svg"; then die 'two figures with one source but different names drew the same sketch: the figure name does not seed it'; fi
expect_ok 'both named figures are stable' check "$modules"
rm "$modules/other.typ" "$modules/other.svg" "$modules/other.layout.json"
ok 'a figure seeds its sketch from its name'

# The book's own modules: the record is the same at every style, and a change to
# the style tables changes the drawing and not the record.
direct_record() { # OUT [INPUT...]: engine.typ's record, evaluated as figures.sh does
  local out="$1" input
  shift
  local inputs=()
  for input in "$@"; do inputs+=(--input "$input"); done
  (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9 \
    "$tools/typst" eval --pretty --root "$modules" --ignore-system-fonts --ignore-embedded-fonts \
    --font-path "$tools/fonts/dejavu-2.37" --package-cache-path "$tools/typst-packages" "${inputs[@]}" \
    --in "$modules/engine.typ" 'query(<layout>).first().value' >"$out") || die "Typst could not evaluate engine's record with inputs $*"
}
direct_record "$work/engine-sketch.layout.json" figure=engine
direct_record "$work/engine-crisp.layout.json" figure=engine style=crisp
direct_record "$work/engine-rough3.layout.json" figure=engine roughness=3
cmp -s "$modules/engine.layout.json" "$work/engine-sketch.layout.json" || die 'the record figures.sh wrote is not what the shipped style evaluates to'
cmp -s "$work/engine-sketch.layout.json" "$work/engine-crisp.layout.json" || die 'the crisp style changed the layout record'
cmp -s "$work/engine-sketch.layout.json" "$work/engine-rough3.layout.json" || die 'roughness 3 changed the layout record'
ok 'the book modules'"'"' record is identical at the shipped style, crisp and roughness 3, while the drawings differ (above)'
d="$work/modules-restyle"
rm -rf "$d"
cp -R "$modules" "$d"
sed -i.bak 's/grey: rgb("#E8EDF2")/grey: rgb("#E8EDF3")/' "$d/_style.typ" && rm "$d/_style.typ.bak"
cmp -s "$modules/_style.typ" "$d/_style.typ" && die 'the restyle control did not change _style.typ'
expect_refused 'a change to the style tables changes the SVG' 'engine.svg differs' check "$d"
not_in_output 'a change to the style tables leaves the record alone' 'engine.layout.json'
{ printf '#import "@preview/cetz:0.4.2"\n'; cat "$modules/engine.typ"; } >"$modules/engine.typ.new" && mv "$modules/engine.typ.new" "$modules/engine.typ"
expect_refused 'a figure drawing with CeTZ itself, beside the book modules, is refused' 'engine.typ imports a package' check "$modules"

# Font pins: the faces are the only ones a render can use.
t="$work/tools-font"; mkdir -p "$t/fonts"
ln -s "$tools/typst" "$t/typst"
ln -s "$tools/typst-packages" "$t/typst-packages"
cp -R "$tools/fonts/dejavu-2.37" "$t/fonts/dejavu-2.37"
printf 'x' >>"$t/fonts/dejavu-2.37/DejaVuSans.ttf"
expect_refused 'a tampered pinned font is refused' 'does not match its pinned digest' check "$fixtures" "$t"
cp "$tools/fonts/dejavu-2.37/DejaVuSans.ttf" "$t/fonts/dejavu-2.37/DejaVuSans.ttf"
cp "$tools/fonts/dejavu-2.37/DejaVuSans.ttf" "$t/fonts/dejavu-2.37/Extra.ttf"
expect_refused 'an extra font on the font path is refused' 'Extra.ttf, which is not pinned' check "$fixtures" "$t"
rm "$t/fonts/dejavu-2.37/Extra.ttf" "$t/fonts/dejavu-2.37/DejaVuSans-Bold.ttf"
expect_refused 'a missing pinned font is refused with the remedy' "the pinned font DejaVuSans-Bold.ttf is not installed; run 'task setup'" check "$fixtures" "$t"
cp "$tools/fonts/dejavu-2.37/DejaVuSans-Bold.ttf" "$t/fonts/dejavu-2.37/DejaVuSans-Bold.ttf"
expect_ok 'the same tools directory passes once the tamper is undone' check "$fixtures" "$t"

# ----------------------------------------------------------- part 3: a page

# One page build with a figure embedded: the relative path the Markdown uses
# must resolve both beside the page (what GitHub's file view follows) and in the
# mdBook output. This runs on a copy of the book configuration, so no figure
# enters the real book before the art stage draws one.
book="$work/book"
mkdir -p "$book/src/figures"
cp "$root/docs/book/book.toml" "$book/book.toml"
sed -i.bak 's|^build-dir = .*|build-dir = "out"|' "$book/book.toml" && rm "$book/book.toml.bak"
cp "$fixtures"/*.svg "$fixtures"/*.typ "$fixtures"/*.layout.json "$book/src/figures/"
cat >"$book/src/SUMMARY.md" <<'EOF'
# Summary

- [A page with a figure](01-page.md)
EOF
cat >"$book/src/01-page.md" <<'EOF'
# A page with a figure

![Caller starts and Ravel runs](figures/layers.svg)
EOF
[[ -f "$book/src/figures/layers.svg" ]] || die 'the figure is not where the Markdown path resolves in the source tree'
build_log="$work/mdbook.log"
"$tools/mdbook" build "$book" >"$build_log" 2>&1 || die "mdBook build failed: $(cat "$build_log")"
if grep -Eq '^ *(ERROR|WARN) ' "$build_log"; then die "mdBook logged a problem: $(cat "$build_log")"; fi
cmp -s "$book/out/figures/layers.svg" "$fixtures/layers.svg" || die 'the built book does not carry the committed SVG byte for byte'
grep -Fq 'src="figures/layers.svg"' "$book/out/01-page.html" || die 'the built page does not reference figures/layers.svg'
grep -Fq 'alt="Caller starts and Ravel runs' "$book/out/01-page.html" || die 'the built page lost the alt text'
ok 'one page with a figure builds in mdBook: the SVG is copied byte for byte and the page references it with its alt text'
if [[ -f "$book/out/figures/layers.typ" ]]; then
  ok 'note: mdBook also copies the editable sources beside the SVGs into the built book'
fi

printf 'figures-test: fetch refusals, reproducibility, environment isolation, compare and lint controls and the page build passed\n'
