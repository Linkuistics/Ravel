#!/usr/bin/env bash
# The book's figure toolchain: fetch and verify the pinned renderer (Typst with
# the CeTZ package, and the DejaVu fonts it sets text in), then regenerate every
# figure under docs/book/src/figures and compare it with the committed SVG, and
# each figure's layout record with its committed .layout.json. The layout on disk
# and the reasons for each choice below are in docs/book/figures.md.
#
# Like scripts/book.sh this is separate from crates/ravel/scripts/dispatch.sh,
# whose bytes decide the compiled worker's identity; the book is not product.
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly TYPST_VERSION="0.15.1"
readonly CETZ_VERSION="0.4.2"
readonly OXIFMT_VERSION="1.0.0"
readonly DEJAVU_VERSION="2.37"
# The tools directory is the one `task setup` already fills with Bun and mdBook.
TOOLS="${BOOK_TOOLS_DIR:-$ROOT/target/tools}"
readonly TOOLS
FIGURES_DIR="${FIGURES_DIR:-$ROOT/docs/book/src/figures}"
readonly FIGURES_DIR

readonly DEJAVU_ARCHIVE="dejavu-fonts-ttf-$DEJAVU_VERSION.tar.bz2"
readonly DEJAVU_SHA256="fa9ca4d13871dd122f61258a80d01751d603b4d3ee14095d65453b4e846e17d7"
# Only the font directory is a variable, so a control can point a render at a
# directory without the pinned fonts and see the output change.
fontdir="$TOOLS/fonts/dejavu-$DEJAVU_VERSION"
# The Typst package cache that setup fills and renders read; a variable for the
# same reason as the font directory.
packages="$TOOLS/typst-packages"

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
# Why fonts are pinned at all: a figure's layout is sized from the text it holds,
# measured in the face it is drawn in, so a host's own fonts must not reach it.
# Every render below ignores the system's fonts and the ones Typst embeds, so these
# faces are the only ones it can use; a family that is missing is an error here
# (see render_all) and not a fallback to another face.
#
# Only the upright faces are pinned. A label in italics would draw as the upright
# face, because Typst does not slant a face it lacks; the book sets no label in
# italics (docs/book/figures.md says why), so no Oblique face is pinned.
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
  local scratch
  mkdir -p "$TOOLS"
  make_scratch
  scratch="$SCRATCH"

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
  echo "figures: Typst $TYPST_VERSION with CeTZ $CETZ_VERSION and DejaVu $DEJAVU_VERSION fonts in $TOOLS"
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

require_tools() {
  local found face digest
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
  # The whole directory is Typst's font path, so it holds the pinned faces only.
  while read -r face; do
    [[ -n "$face" ]] || continue
    font_files | cut -d' ' -f1 | grep -qxF -- "$face" ||
      die "the font directory holds $face, which is not pinned; run 'task setup'"
  done < <(ls -A "$fontdir")
}

# Figures are the *.typ files directly in the figures directory whose names do
# not start with an underscore; an underscore file is a shared module. Print the
# figure sources, one path per line.
figure_sources() {
  local dir="$1" file
  for file in "$dir"/*.typ; do
    [[ -f "$file" ]] || continue
    [[ "$(basename "$file")" == _* ]] && continue
    echo "$file"
  done
}

# Refuse anything the layout does not account for, so the compare covers every
# file in the directory: an SVG with no source, a subdirectory, or a file of
# another kind. A second editable source for a figure (a .mmd, say, beside its
# .typ) is a file of another kind, so there is exactly one source per figure.
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
      *.typ | *.svg | *.layout.json) ;;
      *)
        echo "figures: $name is not a .typ, .svg or .layout.json file" >&2
        problems=1
        ;;
    esac
  done
  stems="$(figure_sources "$dir" | while read -r file; do
    name="$(basename "$file")"
    echo "${name%.*}"
  done)"
  for file in "$dir"/*.svg; do
    [[ -f "$file" ]] || continue
    stem="$(basename "$file" .svg)"
    if ! printf '%s\n' "$stems" | grep -qxF -- "$stem"; then
      echo "figures: $stem.svg has no $stem.typ source" >&2
      problems=1
    fi
  done
  # A layout record is the JSON a figure's layout evaluates to; a module, whose
  # name starts with an underscore, is not a figure and has none.
  for file in "$dir"/*.layout.json; do
    [[ -f "$file" ]] || continue
    stem="$(basename "$file" .layout.json)"
    if [[ "$stem" == _* || ! -f "$dir/$stem.typ" ]]; then
      echo "figures: $stem.layout.json has no $stem.typ source (only a figure has a layout record)" >&2
      problems=1
    fi
  done
  return "$problems"
}

# The rules for every .typ in the directory (modules included). A package
# the render would fetch is a package nobody pinned, so every `@namespace/name:
# version` spec anywhere in the text must be a pinned one; the cache is the other
# half of that rule (see require_packages), and the render has no network. And a
# figure source imports no package itself, and no module but _draw.typ imports
# CeTZ: layout and style modules produce data and _draw.typ alone draws it, so
# how every stroke is drawn can change in that one file.
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
  if [[ "$name" == _* && "$name" != _draw.typ ]] && grep -q -F '@preview/cetz' "$file"; then
    echo "figures: $name imports CeTZ; only _draw.typ draws" >&2
    problems=1
  fi
  return "$problems"
}

# Run the pinned Typst for one figure: WORK is the private working directory
# (it holds the empty HOME), ROOT the directory the figure's imports resolve in
# and STEM the figure's name; the remaining arguments are the subcommand and its
# own options. Standard output and error are the caller's to redirect.
typst_isolated() {
  local work="$1" root="$2" stem="$3" subcommand="$4"
  shift 4
  (cd "$work" && env -i PATH=/usr/bin:/bin HOME="$work/home" HTTP_PROXY=http://127.0.0.1:9 HTTPS_PROXY=http://127.0.0.1:9 \
    "$TOOLS/typst" "$subcommand" --root "$root" --ignore-system-fonts --ignore-embedded-fonts \
    --font-path "$fontdir" --package-cache-path "$packages" --input "figure=$stem" "$@")
}

# Render every figure source in DIR to OUT/<stem>.svg and its layout record to
# OUT/<stem>.layout.json. Typst runs under an empty environment, so a developer's
# TYPST_* variables, locale and home directory cannot change a figure, and from a
# private working directory, so a file in the caller's directory is not read.
# Offline and from the pinned inputs only: no system or embedded fonts, the
# package cache that require_tools has verified (the empty environment and
# private HOME leave no @local packages), and a proxy that refuses connections,
# so anything outside the pins is an error instead of a download. Typst reports a
# missing font family as a warning and still succeeds, so any output on stderr
# fails the figure. The figure's name is the one input: _draw.typ seeds each
# figure's sketch from it, and says nothing else about the style, so the shipped
# look is compiled in.
render_all() {
  local dir="$1" out="$2" file work stem
  make_scratch
  work="$SCRATCH"
  mkdir -p "$out" "$work/home"
  while read -r file; do
    [[ -n "$file" ]] || continue
    stem="$(basename "$file" .typ)"
    # The SVG and the layout record come from one source under one isolation.
    typst_isolated "$work" "$dir" "$stem" compile --format svg "$file" "$out/$stem.svg" >/dev/null 2>"$work/typst.err" ||
      die "Typst could not render $(basename "$file"): $(cat "$work/typst.err")"
    [[ ! -s "$work/typst.err" ]] || die "Typst warned while rendering $(basename "$file"): $(cat "$work/typst.err")"
    # The layout record is whatever Typst prints for the value of the figure's
    # <layout> metadata, pretty-printed so a change is a readable diff, and
    # written as printed. A figure that attaches none has no value to print.
    typst_isolated "$work" "$dir" "$stem" eval --pretty --in "$file" 'query(<layout>).first().value' >"$out/$stem.layout.json" 2>"$work/typst.err" ||
      die "Typst could not evaluate the layout record of $(basename "$file") (a figure attaches one with _draw.typ's render, as metadata(record) <layout>): $(cat "$work/typst.err")"
    [[ ! -s "$work/typst.err" ]] || die "Typst warned while evaluating the layout record of $(basename "$file"): $(cat "$work/typst.err")"
  done < <(figure_sources "$dir")
}

# A layout record holds positions, roles and measured text and no style, so a
# restyle cannot change it. Typst prints a colour as the string rgb("#..."), a
# length as "1.5pt" and content as an object with a "func" key, and none of them
# is a position or a measurement. Refuse a record with one, or with a key that
# names a style property, so the committed record cannot drift with a restyle. A
# label that is exactly a hex colour or a length, with nothing else in it, looks
# like one and is refused too.
lint_record() {
  local json="$1" name="$2" found
  found="$(grep -n -E '^[[:space:]]*"(fill|stroke|paint|thickness|color|colour|weight|size|font|roughness|radius|opacity|dash|style|func)":' "$json" || true)"
  found="$found$(grep -n -E '^[[:space:]]*("[^"]*":[[:space:]]*)?"(rgb\(|luma\(|cmyk\(|oklab\(|oklch\(|color\.|#[0-9A-Fa-f]{3,8}"|-?[0-9.]+(pt|mm|cm|in|em|%)")' "$json" || true)"
  if [[ -n "$found" ]]; then
    echo "figures: $name's layout record holds a style value, which a restyle would change (line: ${found%%$'\n'*}); styles belong in _draw.typ and _style.typ" >&2
    return 1
  fi
}

# Compare one figure's freshly evaluated record with the committed
# <stem>.layout.json, or in update mode write it. Prints what it did and returns
# non-zero for a problem.
sync_layout_record() {
  local mode="$1" fresh="$2" committed="$3" stem="$4" name="$5"
  lint_record "$fresh" "$name" || return 1
  if [[ ! -f "$committed" ]]; then
    if [[ "$mode" == update ]]; then
      install -m 0644 "$fresh" "$committed"
      echo "figures: wrote $stem.layout.json"
    else
      echo "figures: $stem.layout.json is missing; run 'task docs:figures:update'" >&2
      return 1
    fi
  elif ! cmp -s "$fresh" "$committed"; then
    if [[ "$mode" == update ]]; then
      install -m 0644 "$fresh" "$committed"
      echo "figures: rewrote $stem.layout.json"
    else
      echo "figures: $stem.layout.json differs from the layout record $name evaluates to; run 'task docs:figures:update' if the change is intended" >&2
      return 1
    fi
  fi
}

# `check` regenerates into a scratch directory and compares; `update` also
# writes what differs. Every problem is reported, not only the first.
verify_figures() {
  local mode="$1" dir="$FIGURES_DIR" scratch problems=0 count=0 file name stem committed fresh
  require_tools
  if [[ ! -d "$dir" ]]; then
    echo "figures: no figures yet ($dir does not exist); tools verified"
    return 0
  fi
  lint_layout "$dir" || problems=1
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
    # The record is compared apart from the SVG, so a restyle (the SVG changes, the
    # record does not) and a layout change (both change) read differently.
    sync_layout_record "$mode" "$scratch/out/$stem.layout.json" "$dir/$stem.layout.json" "$stem" "$name" || problems=1
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
      else
        # Typst's SVG carries no version stamp, so a different Typst cannot be told
        # from a changed figure; regenerating is the remedy for both.
        echo "figures: $stem.svg differs from what $name renders with the pinned Typst $TYPST_VERSION and CeTZ $CETZ_VERSION (an SVG made with another version differs in the same way); run 'task docs:figures:update' if the change is intended" >&2
        problems=1
      fi
    fi
  done < <(figure_sources "$dir")
  ((problems == 0)) || die "one or more figures do not match their sources"
  echo "figures: $count figure(s) $([[ "$mode" == update ]] && echo up to date || echo match their sources) (Typst $TYPST_VERSION, CeTZ $CETZ_VERSION, DejaVu $DEJAVU_VERSION)"
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
