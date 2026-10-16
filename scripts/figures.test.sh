#!/usr/bin/env bash
# Controls for scripts/figures.sh. Every refusal below is made to fire against a
# deliberately wrong subject, and the unchanged fixtures are the one pass:
# a check that never failed would read the same as a check that cannot.
#
# Part 1 drives the fetch-and-verify function offline, emulating only `curl`.
# Part 2 runs the real renderers (needs `task setup`) over the committed
# fixtures in scripts/figures-fixtures and over mutated copies of them.
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

[[ -f "$tools/plantuml-1.2026.8.jar" && -x "$tools/d2" ]] || die 'run task setup before the figure controls'
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
ok 'every fixture renders twice to identical bytes, equal to the committed SVG'

# Nothing about the caller's environment may reach a figure, and the same holds
# for a stray skin file in the working directory.
hostile="$work/hostile"
mkdir -p "$hostile"
printf 'skinparam backgroundColor red\nskinparam defaultFontSize 30\n' >"$hostile/plantuml.skin"
(
  cd "$hostile"
  GRAPHVIZ_DOT=/nonexistent D2_LAYOUT=dagre D2_THEME=4 D2_SKETCH=true D2_PAD=0 \
    PLANTUML_LIMIT_SIZE=100 _JAVA_OPTIONS=-Xmx1m JAVA_TOOL_OPTIONS=-Duser.language=tr \
    LANG=tr_TR.UTF-8 TZ=Asia/Kolkata FIGURES_DIR="$fixtures" BOOK_TOOLS_DIR="$tools" \
    bash "$script" check
) >"$work/out" 2>&1 || die "a hostile environment changed the result: $(cat "$work/out")"
ok 'D2_*, GRAPHVIZ_DOT, PLANTUML_*, JVM options, locale and a skin file in the working directory change nothing'

# A changed source, a changed SVG, a missing SVG and an SVG without a source.
d="$(copy_fixtures source-changed)"
sed -i.bak 's/starts a session/begins a session/' "$d/context.puml" && rm "$d/context.puml.bak"
expect_refused 'a changed source fails the compare' 'context.svg differs' check "$d"

d="$(copy_fixtures svg-changed)"
printf ' ' >>"$d/states.svg"
expect_refused 'a changed SVG fails the compare' 'states.svg differs' check "$d"

d="$(copy_fixtures svg-missing)"
rm "$d/sequence.svg"
expect_refused 'a missing SVG fails' 'sequence.svg is missing' check "$d"

d="$(copy_fixtures svg-orphan)"
cp "$d/flow.svg" "$d/gone.svg"
expect_refused 'an SVG with no source fails' 'gone.svg has no gone.puml or gone.d2 source' check "$d"

# update rewrites what differs, then check passes, then update changes nothing.
d="$(copy_fixtures update)"
sed -i.bak 's/starts a session/begins a session/' "$d/context.puml" && rm "$d/context.puml.bak"
rm "$d/sequence.svg"
expect_ok 'update rewrites a stale SVG and writes a missing one' update "$d"
grep -Fq 'rewrote context.svg' "$work/out" || die "update did not report rewriting context.svg: $(cat "$work/out")"
grep -Fq 'wrote sequence.svg' "$work/out" || die "update did not report writing sequence.svg: $(cat "$work/out")"
expect_ok 'check passes after update' check "$d"
before="$(cat "$d"/*.svg | shasum -a 256)"
expect_ok 'a second update succeeds' update "$d"
if grep -Fq 'wrote' "$work/out"; then die 'a second update rewrote a file'; fi
[[ "$before" == "$(cat "$d"/*.svg | shasum -a 256)" ]] || die 'a second update changed the SVGs'
ok 'update is idempotent'

# A renderer or layout pin that changes the output fails the compare.
d="$(copy_fixtures engine)"
sed -i.bak 's/layout-engine: elk/layout-engine: dagre/' "$d/flow.d2" && rm "$d/flow.d2.bak"
expect_refused 'changing the d2 layout engine changes the output and fails' 'flow.svg differs' check "$d"

d="$(copy_fixtures theme)"
sed -i.bak 's/theme-id: 0/theme-id: 4/' "$d/flow.d2" && rm "$d/flow.d2.bak"
expect_refused 'changing the d2 theme changes the output and fails' 'flow.svg differs' check "$d"

d="$(copy_fixtures include)"
printf 'skinparam ArrowColor #CC0000\n' >>"$d/_style.puml"
expect_refused 'changing a shared include changes the figures that use it' 'differs' check "$d"

d="$(copy_fixtures fallback)"
sed -i.bak 's/font-family="DejaVu Sans, sans-serif"/font-family="DejaVu Sans"/g' "$d/states.svg" && rm "$d/states.svg.bak"
expect_refused 'an SVG without the generic font fallback fails' 'states.svg differs' check "$d"

# The font pin is load-bearing: the same source rendered with Java's font path
# pointing at a directory without the pinned faces gives different bytes.
mkdir -p "$work/no-fonts"
(
  # shellcheck disable=SC2031 # each subshell sets its own tools directory
  export BOOK_TOOLS_DIR="$tools"
  # shellcheck disable=SC1090
  source "$script"
  # shellcheck disable=SC2034 # read by render_all in the sourced script
  fontdir="$work/no-fonts"
  render_all "$fixtures" "$work/render-nofont"
)
if cmp -s "$work/render-nofont/context.svg" "$fixtures/context.svg"; then
  die 'rendering without the pinned fonts gave the same bytes, so the font pin proves nothing here'
fi
ok 'without the pinned fonts the same source renders differently (the font pin changes the output)'

# The renderer's own stamp, on the committed SVG.
d="$(copy_fixtures stamp-plantuml)"
sed -i.bak 's/<?plantuml 1.2026.8?>/<?plantuml 1.2025.0?>/' "$d/context.svg" && rm "$d/context.svg.bak"
expect_refused 'a PlantUML SVG from another version is named as such' 'was not made by the pinned renderer' check "$d"
d="$(copy_fixtures stamp-d2)"
sed -i.bak 's/data-d2-version="v0.9.0"/data-d2-version="v0.8.0"/' "$d/flow.svg" && rm "$d/flow.svg.bak"
expect_refused 'a d2 SVG from another version is named as such' 'was not made by the pinned renderer' check "$d"

# Layout lint: the settings that keep a host's defaults out of a figure.
d="$(copy_fixtures no-pragma)"
sed -i.bak '/pragma layout smetana/d' "$d/states.puml" && rm "$d/states.puml.bak"
expect_refused 'a PlantUML source without the Smetana pragma is refused' "must declare '!pragma layout smetana'" check "$d"
d="$(copy_fixtures two-diagrams)"
printf '@startuml\n@enduml\n' >>"$d/states.puml"
expect_refused 'a PlantUML file with two diagrams is refused' 'exactly one bare' check "$d"
d="$(copy_fixtures remote-include)"
sed -i.bak 's|^!include _style.puml|!include https://example.invalid/x.puml|' "$d/states.puml" && rm "$d/states.puml.bak"
expect_refused 'a remote include is refused' 'remote file' check "$d"
d="$(copy_fixtures sketch)"
sed -i.bak 's/sketch: false/sketch: true/' "$d/flow.d2" && rm "$d/flow.d2.bak"
expect_refused 'a sketched d2 figure is refused' "'sketch: false'" check "$d"
d="$(copy_fixtures tala)"
sed -i.bak 's/layout-engine: elk/layout-engine: tala/' "$d/flow.d2" && rm "$d/flow.d2.bak"
expect_refused 'the d2 TALA engine is refused' 'layout-engine' check "$d"
d="$(copy_fixtures no-config)"
sed -i.bak '/theme-id/d' "$d/flow.d2" && rm "$d/flow.d2.bak"
expect_refused 'a d2 source without its theme is refused' 'theme-id' check "$d"

# Layout lint: what is in the directory.
d="$(copy_fixtures duplicate)"
cp "$d/flow.d2" "$d/context.d2"
expect_refused 'two sources with one stem are refused' 'would both write context.svg' check "$d"
d="$(copy_fixtures stray)"
printf 'notes\n' >"$d/notes.txt"
expect_refused 'a file of another kind is refused' 'notes.txt is not a .puml, .d2 or .svg file' check "$d"
d="$(copy_fixtures subdir)"
mkdir "$d/nested"
expect_refused 'a subdirectory is refused' 'nested is a directory' check "$d"

# Tool pins: each is a different renderer, which is a different figure.
t="$work/tools-empty"; mkdir -p "$t"
expect_refused 'a missing PlantUML is refused with the remedy' "run 'task setup'" check "$fixtures" "$t"

t="$work/tools-jar"; mkdir -p "$t"
cp "$tools/plantuml-1.2026.8.jar" "$t/"
printf 'x' >>"$t/plantuml-1.2026.8.jar"
expect_refused 'a tampered PlantUML jar is refused' 'is pinned' check "$fixtures" "$t"

t="$work/tools-d2"; mkdir -p "$t"
ln -s "$tools/plantuml-1.2026.8.jar" "$t/plantuml-1.2026.8.jar"
printf '#!/bin/sh\necho v0.8.0\n' >"$t/d2" && chmod +x "$t/d2"
expect_refused 'a d2 of another version is refused' "d2 0.9.0 is pinned, found 'v0.8.0'" check "$fixtures" "$t"

t="$work/tools-font"; mkdir -p "$t/fonts"
ln -s "$tools/plantuml-1.2026.8.jar" "$t/plantuml-1.2026.8.jar"
ln -s "$tools/d2" "$t/d2"
cp -R "$tools/fonts/dejavu-2.37" "$t/fonts/dejavu-2.37"
printf 'x' >>"$t/fonts/dejavu-2.37/DejaVuSans.ttf"
expect_refused 'a tampered pinned font is refused' 'does not match its pinned digest' check "$fixtures" "$t"
cp "$tools/fonts/dejavu-2.37/DejaVuSans.ttf" "$t/fonts/dejavu-2.37/DejaVuSans.ttf"
cp "$tools/fonts/dejavu-2.37/DejaVuSans.ttf" "$t/fonts/dejavu-2.37/Extra.ttf"
expect_refused 'an extra font on the font path is refused' 'Extra.ttf, which is not pinned' check "$fixtures" "$t"

# ----------------------------------------------------------- part 3: a page

# One page build with a figure embedded: the relative path the Markdown uses
# must resolve both beside the page (what GitHub's file view follows) and in the
# mdBook output. This runs on a copy of the book configuration, so no figure
# enters the real book before the art stage draws one.
book="$work/book"
mkdir -p "$book/src/figures"
cp "$root/docs/book/book.toml" "$book/book.toml"
sed -i.bak 's|^build-dir = .*|build-dir = "out"|' "$book/book.toml" && rm "$book/book.toml.bak"
cp "$fixtures"/*.svg "$fixtures"/*.puml "$fixtures"/*.d2 "$book/src/figures/"
cat >"$book/src/SUMMARY.md" <<'EOF'
# Summary

- [A page with a figure](01-page.md)
EOF
cat >"$book/src/01-page.md" <<'EOF'
# A page with a figure

![Caller starts a session in Ravel, which launches an agent program](figures/context.svg)
EOF
[[ -f "$book/src/figures/context.svg" ]] || die 'the figure is not where the Markdown path resolves in the source tree'
build_log="$work/mdbook.log"
"$tools/mdbook" build "$book" >"$build_log" 2>&1 || die "mdBook build failed: $(cat "$build_log")"
if grep -Eq '^ *(ERROR|WARN) ' "$build_log"; then die "mdBook logged a problem: $(cat "$build_log")"; fi
cmp -s "$book/out/figures/context.svg" "$fixtures/context.svg" || die 'the built book does not carry the committed SVG byte for byte'
grep -Fq 'src="figures/context.svg"' "$book/out/01-page.html" || die 'the built page does not reference figures/context.svg'
grep -Fq 'alt="Caller starts a session in Ravel' "$book/out/01-page.html" || die 'the built page lost the alt text'
ok 'one page with a figure builds in mdBook: the SVG is copied byte for byte and the page references it with its alt text'
if [[ -f "$book/out/figures/context.puml" ]]; then
  ok 'note: mdBook also copies the editable sources beside the SVGs into the built book'
fi

printf 'figures-test: fetch refusals, reproducibility, environment isolation, compare and lint controls and the page build passed\n'
