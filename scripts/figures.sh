#!/usr/bin/env bash
# The book's figure toolchain: fetch and verify the pinned renderers (PlantUML,
# d2 and Typst with the CeTZ package), then regenerate every figure under
# docs/book/src/figures and compare it with the committed SVG. The layout on disk and the reasons for each choice below are
# in docs/book/figures.md.
#
# Like scripts/book.sh this is separate from crates/ravel/scripts/dispatch.sh,
# whose bytes decide the compiled worker's identity; the book is not product.
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly PLANTUML_VERSION="1.2026.8"
readonly D2_VERSION="0.9.0"
readonly TYPST_VERSION="0.15.1"
readonly CETZ_VERSION="0.4.2"
readonly OXIFMT_VERSION="1.0.0"
readonly DEJAVU_VERSION="2.37"
# The tools directory is the one `task setup` already fills with Bun and mdBook.
TOOLS="${BOOK_TOOLS_DIR:-$ROOT/target/tools}"
readonly TOOLS
FIGURES_DIR="${FIGURES_DIR:-$ROOT/docs/book/src/figures}"
readonly FIGURES_DIR

readonly PLANTUML_JAR_NAME="plantuml-$PLANTUML_VERSION.jar"
readonly PLANTUML_SHA256="5e1ecfa8ecd32c90b03bbf3b1eb6f020943f98ab0fcf4032be31a0002ee2c462"
readonly DEJAVU_ARCHIVE="dejavu-fonts-ttf-$DEJAVU_VERSION.tar.bz2"
readonly DEJAVU_SHA256="fa9ca4d13871dd122f61258a80d01751d603b4d3ee14095d65453b4e846e17d7"
# Only the font directory is a variable, so a control can point a render at a
# directory without the pinned fonts and see the output change.
fontdir="$TOOLS/fonts/dejavu-$DEJAVU_VERSION"
# The Typst package cache that setup fills and renders read; a variable for the
# same reason as the font directory.
packages="$TOOLS/typst-packages"

# PlantUML 1.2026.8 (GPL build, the one Homebrew installs) from
# https://github.com/plantuml/plantuml/releases/tag/v1.2026.8, published
# 2026-09-05. The digest equals the one GitHub lists for the asset and was
# recomputed from the download on 2026-10-16. The jar bundles C4-PlantUML 2.13.0
# (`-stdlib` lists `c4 Version 2.13.0`) and the Smetana layout, and needs a
# Java 11 or later runtime (class-file major version 55), which is not fetched.
#
# d2 0.9.0 from https://github.com/terrastruct/d2/releases/tag/v0.9.0, published
# 2026-09-07. Each digest equals the line in the release's SHA256SUMS and was
# recomputed from the download on 2026-10-16. The macOS archive was run on
# Darwin arm64, the Linux arm64 archive in Ubuntu arm64 and the Linux amd64
# archive in Ubuntu amd64 under QEMU user-mode emulation (Docker); each rendered
# the controls' figures byte for byte as the others did.
d2_target() {
  case "$(uname -s)/$(uname -m)" in
    Darwin/arm64) echo "macos-arm64" ;;
    Linux/aarch64 | Linux/arm64) echo "linux-arm64" ;;
    Linux/x86_64) echo "linux-amd64" ;;
    *) die "no pinned d2 for $(uname -s)/$(uname -m); the pinned hosts are Darwin/arm64, Linux/aarch64 and Linux/x86_64" ;;
  esac
}

d2_sha256() {
  case "$1" in
    macos-arm64) echo "eaf6c0c143e56dd9fa97bfb6df25ea9c1ebce40245f056a0768cf1a6c15d3064" ;;
    linux-arm64) echo "ac2c028697199479acb321db1e3d68caee9f2ba492ed73caa3cd13f3829bf913" ;;
    linux-amd64) echo "5669ddc46b99e942cc96078f4a4e36d5e62103348f4c05179ede27802fdd87a9" ;;
  esac
}

# Typst 0.15.1 from https://github.com/typst/typst/releases/tag/v0.15.1,
# published 2026-07-17 and the latest release when checked on 2026-10-17. Each
# digest equals the one GitHub lists for the asset and was recomputed from the
# download on 2026-10-17. The Linux builds are static (musl). Only the macOS
# arm64 archive has been run here; the Linux ones are pinned and not yet run.
typst_target() {
  case "$(uname -s)/$(uname -m)" in
    Darwin/arm64) echo "aarch64-apple-darwin" ;;
    Linux/aarch64 | Linux/arm64) echo "aarch64-unknown-linux-musl" ;;
    Linux/x86_64) echo "x86_64-unknown-linux-musl" ;;
    *) die "no pinned Typst for $(uname -s)/$(uname -m); the pinned hosts are Darwin/arm64, Linux/aarch64 and Linux/x86_64" ;;
  esac
}

typst_sha256() {
  case "$1" in
    aarch64-apple-darwin) echo "48f62ed034aa3a7978309579ac6ca00045e2ef0da73114e8af27cfd8e74dc05a" ;;
    aarch64-unknown-linux-musl) echo "5aa8d74a3d906e60ea12a66ac2f37f8eef1b14cbad7182a745e393a10c23dcee" ;;
    x86_64-unknown-linux-musl) echo "a6d077d0a95eed5a2eba715b2dae06be954f624ccbf85758a03f389ded33118c" ;;
  esac
}

# CeTZ 0.4.2 (LGPL-3.0-or-later) and its one dependency oxifmt 1.0.0 (MIT OR
# Apache-2.0) from the Typst package registry, https://packages.typst.org/preview/.
# 0.4.2 is the version the k107 trial drew with. 0.5.2 was the newest on
# 2026-10-17 and is not adopted: moving up is a new trial of every drawing
# primitive, a decision for its own leaf and not a pin. The
# registry lists no digests, so each tarball digest was computed from the
# download on 2026-10-17, and the unpacked tree matched the copy Typst itself
# had fetched. They are fetched and not vendored: CeTZ's licence and its bundled
# WebAssembly plugin should not be copied into this repository, and the SVGs it
# draws carry none of its code.
#
# One package per line: name, version, tarball digest, tree digest. The tree
# digest (see tree_digest) is what every render re-checks, because Typst reads
# the unpacked files and not the tarball.
packages_pinned() {
  cat <<EOF
cetz $CETZ_VERSION e1d2f121b166f1520fc529cad834e2ca5e0b6bcab6816e1d0e07d70836aa0b59 8d159fed15cb54998b1d184b3e028af21ca420218e8985be087253dfa23a4213
oxifmt $OXIFMT_VERSION 7d17a1fc8ad01740ec3cb2b03c7360a4225ff9318e5710765fa98ea6fd59594f aa4fc11c351ae98bec1ec3783a1b105cb33a88f40cbf6317bc9abf29bbc858d2
EOF
}

# DejaVu fonts 2.37, released 2016-07-30 at
# https://github.com/dejavu-fonts/dejavu-fonts/releases/tag/version_2_37. GitHub
# lists no digest for it, so the archive digest above and these faces' digests
# were computed from the download on 2026-10-16. They are this archive's files
# and not any host's copy: Ubuntu's fonts-dejavu-core package has the same
# release number and different bytes.
#
# Why fonts are pinned at all: PlantUML measures text with the Java runtime's
# fonts, so on a host without the font its SVG differs (seen on 2026-10-16
# between Darwin and Ubuntu arm64). With these faces and the font path below the
# same source rendered byte for byte alike on both. d2 embeds its own font and
# needs nothing.
#
# Only the upright faces are pinned. With the Oblique faces present, macOS and
# Ubuntu measured the same italic label 55.945 and 55.816 wide (seen on
# 2026-10-16), so the figure differed by host. Without them Java slants the
# upright face itself and both hosts measure alike.
font_files() {
  cat <<'EOF'
DejaVuSans.ttf 7da195a74c55bef988d0d48f9508bd5d849425c1770dba5d7bfc6ce9ed848954
DejaVuSans-Bold.ttf e6476c1b80502924294eed40894c5b18e06c181444ca953e5334262df9c27724
DejaVuSansMono.ttf b4a6c3e4faab8773f4ff761d56451646409f29abedd68f05d38c2df667d3c582
DejaVuSansMono-Bold.ttf bce60f1b4421acd9ea51ba6623d7024ecbe6817a953e3654df62a5e6bdf8f769
EOF
}

die() {
  echo "figures: $*" >&2
  exit 1
}

# A private scratch directory per call, all removed when the process exits.
# The result is left in SCRATCH rather than printed: a command substitution
# would run the exit trap in its own subshell and delete the directory at once.
make_scratch() {
  if [[ -z "${WORK:-}" ]]; then
    WORK="$(mktemp -d)"
    # shellcheck disable=SC2064 # remove precisely this private directory
    trap "rm -rf '$WORK'" EXIT
  fi
  SCRATCH="$(mktemp -d "$WORK/d.XXXXXX")"
}

digest_of() { # FILE, or - for standard input
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d' ' -f1
  else
    sha256sum "$1" | cut -d' ' -f1
  fi
}

# One digest for an unpacked directory: the digest of the sorted list of each
# file's digest and relative path. A file added, removed, renamed or changed
# moves it, and so does anything that is not a plain file or directory (a
# symlink is not followed), which prints a marker no pin can equal.
tree_digest() {
  local dir="$1" file
  if [[ -n "$(find "$dir" -mindepth 1 ! -type f ! -type d)" ]]; then
    echo "not-a-plain-tree"
    return 0
  fi
  while read -r file; do
    printf '%s %s\n' "$(digest_of "$dir/$file")" "$file"
  done < <(cd "$dir" && find . -type f | LC_ALL=C sort) | digest_of -
}

# Put a verified copy of the release asset NAME in the tools directory and print
# the path of a private copy inside SCRATCH. The digest is checked on that copy,
# so the bytes verified are the bytes used. A cached asset that fails the check
# is fetched again once; a fresh download that still differs is refused and
# nothing is cached.
fetch_verified() {
  local url="$1" name="$2" expected="$3" scratch="$4"
  local cached="$TOOLS/$name" copy="$scratch/$name" found
  mkdir -p "$TOOLS"
  if [[ ! -f "$cached" ]] || ! cp "$cached" "$copy" || [[ "$(digest_of "$copy")" != "$expected" ]]; then
    if ! curl --fail --silent --show-error --location --output "$copy" "$url"; then
      die "cannot download the pinned $name"
    fi
    found="$(digest_of "$copy")"
    if [[ "$found" != "$expected" ]]; then
      die "$name has SHA-256 $found, but $expected is pinned; refusing to use it"
    fi
    cp "$copy" "$cached"
  fi
  echo "$copy"
}

setup_figures() {
  local scratch target
  mkdir -p "$TOOLS"
  make_scratch
  scratch="$SCRATCH"

  fetch_verified \
    "https://github.com/plantuml/plantuml/releases/download/v$PLANTUML_VERSION/$PLANTUML_JAR_NAME" \
    "$PLANTUML_JAR_NAME" "$PLANTUML_SHA256" "$scratch" >/dev/null

  target="$(d2_target)"
  local d2_archive="d2-v$D2_VERSION-$target.tar.gz" d2_copy
  d2_copy="$(fetch_verified \
    "https://github.com/terrastruct/d2/releases/download/v$D2_VERSION/$d2_archive" \
    "$d2_archive" "$(d2_sha256 "$target")" "$scratch")"
  tar -xzf "$d2_copy" -C "$scratch" "d2-v$D2_VERSION/bin/d2" || die "cannot extract d2 from $d2_archive"
  install -m 0755 "$scratch/d2-v$D2_VERSION/bin/d2" "$TOOLS/d2"

  local typst_target typst_archive typst_copy
  typst_target="$(typst_target)"
  typst_archive="typst-$typst_target.tar.xz"
  typst_copy="$(fetch_verified \
    "https://github.com/typst/typst/releases/download/v$TYPST_VERSION/$typst_archive" \
    "$typst_archive" "$(typst_sha256 "$typst_target")" "$scratch")"
  tar -xJf "$typst_copy" -C "$scratch" "typst-$typst_target/typst" || die "cannot extract Typst from $typst_archive (tar needs xz support)"
  install -m 0755 "$scratch/typst-$typst_target/typst" "$TOOLS/typst"

  local name version tarball_digest tree pkg_copy
  while IFS=' ' read -r name version tarball_digest tree; do
    pkg_copy="$(fetch_verified \
      "https://packages.typst.org/preview/$name-$version.tar.gz" \
      "$name-$version.tar.gz" "$tarball_digest" "$scratch")"
    mkdir -p "$scratch/pkg/$name"
    tar -xzf "$pkg_copy" -C "$scratch/pkg/$name" || die "cannot extract $name-$version.tar.gz"
    [[ "$(tree_digest "$scratch/pkg/$name")" == "$tree" ]] ||
      die "the unpacked $name $version does not match its pinned tree digest; refusing to use it"
    rm -rf "${packages:?}/preview/$name/$version"
    mkdir -p "$packages/preview/$name"
    cp -R "$scratch/pkg/$name" "$packages/preview/$name/$version"
  done < <(packages_pinned)

  local font_copy
  font_copy="$(fetch_verified \
    "https://github.com/dejavu-fonts/dejavu-fonts/releases/download/version_${DEJAVU_VERSION//./_}/$DEJAVU_ARCHIVE" \
    "$DEJAVU_ARCHIVE" "$DEJAVU_SHA256" "$scratch")"
  local face digest
  mkdir -p "$scratch/fonts" "$fontdir"
  tar -xjf "$font_copy" -C "$scratch/fonts" || die "cannot extract $DEJAVU_ARCHIVE (tar needs bzip2 support)"
  # IFS is newline and tab for the whole script; these lines are space-separated.
  while IFS=' ' read -r face digest; do
    local source="$scratch/fonts/dejavu-fonts-ttf-$DEJAVU_VERSION/ttf/$face"
    [[ -f "$source" && "$(digest_of "$source")" == "$digest" ]] ||
      die "the DejaVu $DEJAVU_VERSION archive's $face does not match its pinned digest; refusing to use it"
    install -m 0644 "$source" "$fontdir/$face"
  done < <(font_files)
  require_tools
  echo "figures: PlantUML $PLANTUML_VERSION, d2 $D2_VERSION, Typst $TYPST_VERSION with CeTZ $CETZ_VERSION and DejaVu $DEJAVU_VERSION fonts in $TOOLS"
}

# The package cache holds exactly the pinned packages, unpacked as pinned. A
# missing or changed package, or an extra one, is refused here, so a render never
# needs the network and never imports an unpinned package from a leftover cache.
require_packages() {
  local name version tarball_digest tree found dir entry allowed
  allowed="$(packages_pinned | while IFS=' ' read -r name version _; do printf '%s\n%s/%s\n' "$name" "$name" "$version"; done)"
  while IFS=' ' read -r name version tarball_digest tree; do
    dir="$packages/preview/$name/$version"
    [[ -d "$dir" ]] || die "the Typst package @preview/$name:$version is not installed; run 'task setup'"
    found="$(tree_digest "$dir")"
    [[ "$found" == "$tree" ]] ||
      die "the Typst package @preview/$name:$version has tree digest $found, but $tree is pinned; run 'task setup'"
  done < <(packages_pinned)
  # Beside the pinned packages: nothing outside preview/, and nothing in it that
  # is not a pinned name or a pinned name's pinned version.
  while read -r entry; do
    [[ -n "$entry" ]] || continue
    printf '%s\n' "$allowed" | grep -qxF -- "$entry" ||
      die "the Typst package cache holds $entry, which is not pinned; run 'task setup'"
  done < <(
    cd "$packages" && find . -mindepth 1 -maxdepth 1 ! -name preview | sed 's|^\./||'
    cd "$packages/preview" && find . -mindepth 1 -maxdepth 2 | sed 's|^\./||'
  )
}

# Print the first Java runtime that starts and is at least version 11. The
# macOS /usr/bin/java is a stub that may be present and broken, so PATH is not
# trusted alone; FIGURES_JAVA names one explicitly.
find_java() {
  local candidate major
  for candidate in "${FIGURES_JAVA:-}" "${JAVA_HOME:+$JAVA_HOME/bin/java}" java \
    /opt/homebrew/opt/openjdk/bin/java /usr/local/opt/openjdk/bin/java; do
    [[ -n "$candidate" ]] || continue
    candidate="$(command -v "$candidate" 2>/dev/null || true)"
    [[ -n "$candidate" ]] || continue
    # The same empty environment the render runs in, so a developer's
    # _JAVA_OPTIONS cannot make a working runtime look broken.
    major="$(env -i PATH=/usr/bin:/bin "$candidate" -version 2>&1 | sed -n '1s/^[^"]*"\([0-9]*\).*/\1/p' || true)"
    if [[ "$major" =~ ^[0-9]+$ ]] && ((major >= 11)); then
      echo "$candidate"
      return 0
    fi
  done
  return 1
}

require_tools() {
  local jar="$TOOLS/$PLANTUML_JAR_NAME" found face digest
  [[ -f "$jar" ]] || die "PlantUML $PLANTUML_VERSION is not installed; run 'task setup'"
  found="$(digest_of "$jar")"
  [[ "$found" == "$PLANTUML_SHA256" ]] ||
    die "the PlantUML jar has SHA-256 $found, but PlantUML $PLANTUML_VERSION ($PLANTUML_SHA256) is pinned; run 'task setup'"
  [[ -x "$TOOLS/d2" ]] || die "d2 $D2_VERSION is not installed; run 'task setup'"
  found="$("$TOOLS/d2" --version 2>&1 || true)"
  [[ "$found" == "v$D2_VERSION" ]] || die "d2 $D2_VERSION is pinned, found '$found'; run 'task setup'"
  [[ -x "$TOOLS/typst" ]] || die "Typst $TYPST_VERSION is not installed; run 'task setup'"
  # Typst's SVG carries no version stamp, so this up-front check is the only
  # place a different Typst is named. Every render below runs this binary, never
  # one found on PATH.
  found="$(env -i PATH=/usr/bin:/bin "$TOOLS/typst" --version 2>&1 || true)"
  [[ "$found" == "typst $TYPST_VERSION "* ]] || die "Typst $TYPST_VERSION is pinned, found '$found'; run 'task setup'"
  require_packages
  while IFS=' ' read -r face digest; do
    [[ -f "$fontdir/$face" ]] || die "the pinned font $face is not installed; run 'task setup'"
    [[ "$(digest_of "$fontdir/$face")" == "$digest" ]] ||
      die "the font $face does not match its pinned digest; run 'task setup'"
  done < <(font_files)
  # The whole directory is Java's font path, so it holds the pinned faces only.
  while read -r face; do
    [[ -n "$face" ]] || continue
    font_files | cut -d' ' -f1 | grep -qxF -- "$face" ||
      die "the font directory holds $face, which is not pinned; run 'task setup'"
  done < <(ls -A "$fontdir")
  find_java >/dev/null ||
    die "PlantUML needs a Java 11 or later runtime; install one or set FIGURES_JAVA to its java executable"
}

# Figures are the *.puml, *.d2 and *.typ files directly in the figures directory
# whose names do not start with an underscore; an underscore file is a shared
# include or module. Print the figure sources, one path per line.
figure_sources() {
  local dir="$1" file
  for file in "$dir"/*.puml "$dir"/*.d2 "$dir"/*.typ; do
    [[ -f "$file" ]] || continue
    [[ "$(basename "$file")" == _* ]] && continue
    echo "$file"
  done
}

# Refuse anything the layout does not account for, so the compare covers every
# file in the directory: two sources with one stem, an SVG with no source, a
# subdirectory or a file of another kind.
lint_layout() {
  local dir="$1" problems=0 file name stem stems
  for file in "$dir"/* "$dir"/.[!.]*; do
    [[ -e "$file" ]] || continue
    name="$(basename "$file")"
    if [[ -d "$file" ]]; then
      echo "figures: $name is a directory; figures are flat files beside their SVG" >&2
      problems=1
      continue
    fi
    case "$name" in
      *.puml | *.d2 | *.typ | *.svg) ;;
      *)
        echo "figures: $name is not a .puml, .d2, .typ or .svg file" >&2
        problems=1
        ;;
    esac
  done
  stems="$(figure_sources "$dir" | while read -r file; do
    name="$(basename "$file")"
    echo "${name%.*}"
  done)"
  while read -r stem; do
    [[ -n "$stem" ]] || continue
    # Name the sources that exist, whichever notations they are.
    echo "figures: $(for ext in puml d2 typ; do [[ -f "$dir/$stem.$ext" ]] && echo "$stem.$ext"; done | paste -sd' ' - | sed 's/ / and /g') would both write $stem.svg" >&2
    problems=1
  done < <(printf '%s\n' "$stems" | sort | uniq -d)
  for file in "$dir"/*.svg; do
    [[ -f "$file" ]] || continue
    stem="$(basename "$file" .svg)"
    if ! printf '%s\n' "$stems" | grep -qxF -- "$stem"; then
      echo "figures: $stem.svg has no $stem.puml, $stem.d2 or $stem.typ source" >&2
      problems=1
    fi
  done
  return "$problems"
}

# The layout settings live in each source so they are edited with what they
# affect and a reader of the source sees them. Without them PlantUML would use
# whichever Graphviz the host has (seen to change the output on 2026-10-16) and
# d2 would take its engine, theme and sketch mode from D2_* variables or flags.
lint_source() {
  local file="$1" name starts ends
  name="$(basename "$file")"
  case "$name" in
    *.puml)
      starts="$(grep -c -E '^@startuml[[:space:]]*$' "$file" || true)"
      ends="$(grep -c -E '^@enduml[[:space:]]*$' "$file" || true)"
      if [[ "$starts" != 1 || "$ends" != 1 ]] || [[ "$(grep -c -E '^[[:space:]]*@(start|end)' "$file" || true)" != 2 ]]; then
        echo "figures: $name must hold exactly one bare '@startuml' ... '@enduml' diagram" >&2
        return 1
      fi
      if ! grep -q -E '^[[:space:]]*!pragma[[:space:]]+layout[[:space:]]+smetana[[:space:]]*$' "$file"; then
        echo "figures: $name must declare '!pragma layout smetana' (no Graphviz)" >&2
        return 1
      fi
      if grep -q -i -E '^[[:space:]]*(!includeurl|!include[a-z_]*[[:space:]]+https?:)' "$file"; then
        echo "figures: $name includes a remote file, which is not reproducible" >&2
        return 1
      fi
      ;;
    *.d2)
      if ! grep -q -E '^[[:space:]]*layout-engine:[[:space:]]*(dagre|elk)[[:space:]]*$' "$file"; then
        echo "figures: $name must set 'layout-engine: dagre' or 'layout-engine: elk' in its d2-config" >&2
        return 1
      fi
      if ! grep -q -E '^[[:space:]]*theme-id:[[:space:]]*[0-9]+[[:space:]]*$' "$file"; then
        echo "figures: $name must set 'theme-id: <number>' in its d2-config" >&2
        return 1
      fi
      if ! grep -q -E '^[[:space:]]*sketch:[[:space:]]*false[[:space:]]*$' "$file"; then
        echo "figures: $name must set 'sketch: false' in its d2-config" >&2
        return 1
      fi
      ;;
  esac
}

# The Typst rules, for every .typ in the directory (modules included). A package
# the render would fetch is a package nobody pinned, so every `@namespace/name:
# version` spec anywhere in the text must be a pinned one; the cache is the other
# half of that rule (see require_packages), and the render has no network. And a
# figure source draws through its `_` modules and imports no package itself, so
# the one place that touches CeTZ can change how every stroke is drawn.
lint_typst() {
  local file="$1" name token pinned problems=0
  name="$(basename "$file")"
  pinned="$(packages_pinned | while IFS=' ' read -r pkg version _; do echo "@preview/$pkg:$version"; done)"
  while read -r token; do
    [[ -n "$token" ]] || continue
    printf '%s\n' "$pinned" | grep -qxF -- "$token" || {
      echo "figures: $name uses $token, which is not a pinned Typst package ($(printf '%s' "$pinned" | paste -sd' ' - | sed 's/ /, /g'))" >&2
      problems=1
    }
  done < <(grep -o -E '@[A-Za-z][A-Za-z0-9_-]*/[A-Za-z0-9_-]+(:[0-9A-Za-z.+-]+)?' "$file" | sort -u || true)
  if [[ "$name" != _* ]] && grep -q -E 'import[[:space:]]*"@' "$file"; then
    echo "figures: $name imports a package; a figure source draws through its _*.typ modules" >&2
    problems=1
  fi
  return "$problems"
}

# Text of the renderer's own version stamp, which every generated SVG carries.
stamp_of() {
  case "$1" in
    *.puml) echo "<?plantuml $PLANTUML_VERSION?>" ;;
    *.d2) echo "data-d2-version=\"v$D2_VERSION\"" ;;
    *.typ) echo "" ;; # Typst writes no version into its SVG; require_tools names a wrong Typst
  esac
}

# PlantUML names the requested font with no generic fallback, so a reader whose
# browser lacks DejaVu would get its default serif face. Append the generic
# family. The change is a fixed text substitution made identically by check and
# update, so it does not touch reproducibility.
add_font_fallbacks() {
  sed -e 's/font-family="DejaVu Sans"/font-family="DejaVu Sans, sans-serif"/g' \
    -e 's/font-family="DejaVu Sans Mono"/font-family="DejaVu Sans Mono, monospace"/g' \
    -e "s/font-family=\"'DejaVu Sans'\"/font-family=\"'DejaVu Sans', sans-serif\"/g" \
    -e "s/font-family=\"'DejaVu Sans Mono'\"/font-family=\"'DejaVu Sans Mono', monospace\"/g" \
    "$1" >"$1.tmp"
  mv "$1.tmp" "$1"
}

# Render every figure source in DIR to OUT/<stem>.svg. All three renderers run
# under an empty environment, so a developer's D2_* variables, GRAPHVIZ_DOT,
# PLANTUML_* settings, TYPST_* variables, locale and home directory cannot change
# a figure, and
# from a private working directory, so a plantuml.skin or similar file in the
# caller's directory is not read.
render_all() {
  local dir="$1" out="$2" java file work stem plantuml=()
  java="$(find_java)" || die "PlantUML needs a Java 11 or later runtime; set FIGURES_JAVA"
  make_scratch
  work="$SCRATCH"
  mkdir -p "$out" "$work/home"
  while read -r file; do
    [[ -n "$file" ]] || continue
    case "$file" in
      *.puml) plantuml+=("$file") ;;
      *.typ)
        stem="$(basename "$file" .typ)"
        # Offline and from the pinned inputs only: no system or embedded fonts, the
        # package cache that require_tools has verified (the empty environment and
        # private HOME leave no @local packages), and a proxy that refuses
        # connections, so anything outside the pins is an error instead of a
        # download. Typst reports a missing font family as a warning and still
        # succeeds, so any output on stderr fails the figure.
        (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9 \
          "$TOOLS/typst" compile --format svg --root "$dir" --ignore-system-fonts --ignore-embedded-fonts \
          --font-path "$fontdir" --package-cache-path "$packages" \
          "$file" "$out/$stem.svg" >/dev/null 2>"$work/typst.err") || die "Typst could not render $(basename "$file"): $(cat "$work/typst.err")"
        [[ ! -s "$work/typst.err" ]] || die "Typst warned while rendering $(basename "$file"): $(cat "$work/typst.err")"
        ;;
      *.d2)
        stem="$(basename "$file" .d2)"
        (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" "$TOOLS/d2" "$file" "$out/$stem.svg" >/dev/null 2>"$work/d2.err") ||
          die "d2 could not render $(basename "$file"): $(cat "$work/d2.err")"
        ;;
    esac
  done < <(figure_sources "$dir")
  if ((${#plantuml[@]} > 0)); then
    # -nometadata keeps PlantUML from embedding an encoded copy of the source.
    # 1.2026.8 lazily publishes shared format options before hiding metadata;
    # parallel file workers can retain the default with metadata enabled.
    # One worker avoids that race without altering or filtering the SVG.
    # Observed failing bytes and verification: docs/evidence/book-figures-k92/.
    (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" "$java" \
      -Djava.awt.headless=true -Dfile.encoding=UTF-8 -Dsun.java2d.fontpath="$fontdir" \
      -jar "$TOOLS/$PLANTUML_JAR_NAME" --threads 1 -tsvg -nometadata -charset UTF-8 \
      -SdefaultFontName="DejaVu Sans" -SmonospacedFontName="DejaVu Sans Mono" \
      -o "$out" "${plantuml[@]}" >/dev/null 2>"$work/plantuml.err") ||
      die "PlantUML could not render the figures: $(cat "$work/plantuml.err")"
    for file in "${plantuml[@]}"; do
      stem="$(basename "$file" .puml)"
      [[ -f "$out/$stem.svg" ]] || die "PlantUML wrote no $stem.svg for $(basename "$file")"
      add_font_fallbacks "$out/$stem.svg"
    done
  fi
}

# `check` regenerates into a scratch directory and compares; `update` also
# writes what differs. Every problem is reported, not only the first.
verify_figures() {
  local mode="$1" dir="$FIGURES_DIR" scratch problems=0 count=0 file name stem committed fresh stamp
  require_tools
  if [[ ! -d "$dir" ]]; then
    echo "figures: no figures yet ($dir does not exist); tools verified"
    return 0
  fi
  lint_layout "$dir" || problems=1
  while read -r file; do
    [[ -n "$file" ]] || continue
    lint_source "$file" || problems=1
  done < <(figure_sources "$dir")
  for file in "$dir"/*.typ; do
    [[ -f "$file" ]] || continue
    lint_typst "$file" || problems=1
  done
  ((problems == 0)) || die "the figure sources do not follow the layout in docs/book/figures.md"

  make_scratch
  scratch="$SCRATCH"
  cp -R "$dir" "$scratch/in"
  render_all "$scratch/in" "$scratch/out"

  while read -r file; do
    [[ -n "$file" ]] || continue
    count=$((count + 1))
    name="$(basename "$file")"
    stem="${name%.*}"
    fresh="$scratch/out/$stem.svg"
    committed="$dir/$stem.svg"
    stamp="$(stamp_of "$file")"
    if [[ -n "$stamp" ]] && ! grep -qF -- "$stamp" "$fresh"; then
      echo "figures: $name rendered without the expected renderer stamp $stamp" >&2
      problems=1
      continue
    fi
    if [[ ! -f "$committed" ]]; then
      if [[ "$mode" == update ]]; then
        install -m 0644 "$fresh" "$committed"
        echo "figures: wrote $stem.svg"
      else
        echo "figures: $stem.svg is missing; run 'task docs:figures:update'" >&2
        problems=1
      fi
    elif ! cmp -s "$fresh" "$committed"; then
      if [[ "$mode" == update ]]; then
        install -m 0644 "$fresh" "$committed"
        echo "figures: rewrote $stem.svg"
      elif [[ -n "$stamp" ]] && ! grep -qF -- "$stamp" "$committed"; then
        echo "figures: $stem.svg was not made by the pinned renderer (expected $stamp)" >&2
        problems=1
      elif [[ -z "$stamp" ]]; then
        # Typst's SVG has no stamp to tell a different Typst from a changed figure.
        echo "figures: $stem.svg differs from what $name renders with the pinned Typst $TYPST_VERSION and CeTZ $CETZ_VERSION (an SVG made with another version differs in the same way); run 'task docs:figures:update' if the change is intended" >&2
        problems=1
      else
        echo "figures: $stem.svg differs from what $name renders; run 'task docs:figures:update' if the change is intended" >&2
        problems=1
      fi
    fi
  done < <(figure_sources "$dir")
  ((problems == 0)) || die "one or more figures do not match their sources"
  echo "figures: $count figure(s) $([[ "$mode" == update ]] && echo up to date || echo match their sources) (PlantUML $PLANTUML_VERSION, d2 $D2_VERSION, Typst $TYPST_VERSION, CeTZ $CETZ_VERSION, DejaVu $DEJAVU_VERSION)"
}

main() {
  local command="${1:-}"
  shift || true
  case "$command" in
    setup) setup_figures ;;
    check) verify_figures check ;;
    update) verify_figures update ;;
    *) die "usage: figures.sh setup | check | update" ;;
  esac
}

# Sourcing the file (the test does) defines the functions only.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
