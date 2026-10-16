#!/usr/bin/env bash
# The book's toolchain: fetch and verify the pinned mdBook, then build or serve
# docs/book. It is separate from crates/ravel/scripts/dispatch.sh on purpose:
# that script's bytes decide the compiled worker's identity, and the book is
# not part of the product.
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly MDBOOK_VERSION="0.5.4"
readonly BOOK_DIR="$ROOT/docs/book"
# The tools directory is the one `task setup` already fills with Bun.
TOOLS="${BOOK_TOOLS_DIR:-$ROOT/target/tools}"
readonly TOOLS

# The archives are the release assets of
# https://github.com/rust-lang/mdBook/releases/tag/v0.5.4 (published
# 2026-07-06, rechecked and downloaded 2026-10-16). Each digest below was
# computed from the downloaded archive and matched the digest GitHub lists for
# the asset. Both Linux targets use the static musl build, which needs no
# particular C library on the host. An mdBook upgrade re-pins all three and
# reads the configuration schema of the new version before it builds.
mdbook_target() {
  case "$(uname -s)/$(uname -m)" in
    Darwin/arm64) echo "aarch64-apple-darwin" ;;
    Linux/aarch64 | Linux/arm64) echo "aarch64-unknown-linux-musl" ;;
    Linux/x86_64) echo "x86_64-unknown-linux-musl" ;;
    *) die "no pinned mdBook for $(uname -s)/$(uname -m); the pinned hosts are Darwin/arm64, Linux/aarch64 and Linux/x86_64" ;;
  esac
}

mdbook_sha256() {
  case "$1" in
    aarch64-apple-darwin) echo "03e8a6d8b13a2971e0b3280affd03b388373c1485e26f73407c3a76b0b1838df" ;;
    aarch64-unknown-linux-musl) echo "753e5c5c363ee8a56972344dcf91466f005a51db84a7aeffe427ae3ef83d6d44" ;;
    x86_64-unknown-linux-musl) echo "5222beabd3e37dc5be0d18ff99b79058469354db5c220153a1b92db5ba12be89" ;;
  esac
}

die() {
  echo "book: $*" >&2
  exit 1
}

digest_of() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d' ' -f1
  else
    sha256sum "$1" | cut -d' ' -f1
  fi
}

# Install the pinned `mdbook` into the tools directory. The digest is checked on
# a private copy and that copy is what gets extracted, so the bytes verified
# are the bytes used. A cached archive that fails the check is fetched again
# once; a fresh download that still differs is refused and nothing is installed.
setup_mdbook() {
  local target expected archive scratch copy name
  target="$(mdbook_target)"
  expected="$(mdbook_sha256 "$target")"
  name="mdbook-v$MDBOOK_VERSION-$target.tar.gz"
  mkdir -p "$TOOLS"
  archive="$TOOLS/$name"
  scratch="$(mktemp -d "$TOOLS/.mdbook.XXXXXX")"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$scratch'" EXIT
  copy="$scratch/$name"
  if [[ ! -f "$archive" ]] || ! cp "$archive" "$copy" || [[ "$(digest_of "$copy")" != "$expected" ]]; then
    if ! curl --fail --silent --show-error --location --output "$copy" \
      "https://github.com/rust-lang/mdBook/releases/download/v$MDBOOK_VERSION/$name"; then
      die "cannot download the pinned mdBook $MDBOOK_VERSION archive for $target"
    fi
    local found
    found="$(digest_of "$copy")"
    if [[ "$found" != "$expected" ]]; then
      die "the mdBook $MDBOOK_VERSION archive for $target has SHA-256 $found, but $expected is pinned; refusing to use it"
    fi
    cp "$copy" "$archive"
  fi
  tar -xzf "$copy" -C "$scratch" mdbook || die "cannot extract mdbook from $name"
  install -m 0755 "$scratch/mdbook" "$TOOLS/mdbook"
  require_mdbook
  echo "book: mdBook $MDBOOK_VERSION development tool in $TOOLS"
}

require_mdbook() {
  [[ -x "$TOOLS/mdbook" ]] || die "mdBook $MDBOOK_VERSION is not installed; run 'task setup'"
  local found
  found="$("$TOOLS/mdbook" --version)"
  [[ "$found" == "mdbook v$MDBOOK_VERSION" ]] ||
    die "mdBook $MDBOOK_VERSION is pinned, found '$found'; run 'task setup'"
}

# `build` never serves; the output is under target/, set by book.toml.
#
# mdBook 0.5.4 exits 0 after logging an ERROR when a preprocessor fails (a
# `{{#include}}` of a missing file, seen on 2026-10-16), and logs a WARN for
# some malformed Markdown it recovers from. A check that read only the exit
# status would pass a book with a hole in it, so the build fails on either.
build_book() {
  require_mdbook
  local log status=0
  log="$(mktemp)"
  "$TOOLS/mdbook" build "$BOOK_DIR" >"$log" 2>&1 || status=$?
  cat "$log"
  if ((status != 0)); then
    rm -f "$log"
    exit "$status"
  fi
  if grep -Eq '^ *(ERROR|WARN) ' "$log"; then
    rm -f "$log"
    die "mdBook logged an error or warning; the book is not clean"
  fi
  rm -f "$log"
}

# Build, then serve on loopback and rebuild on change.
serve_book() {
  local port="${1:-8775}"
  require_mdbook
  "$TOOLS/mdbook" serve "$BOOK_DIR" --hostname 127.0.0.1 --port "$port"
}

main() {
  local command="${1:-}"
  shift || true
  case "$command" in
    setup) setup_mdbook ;;
    build) build_book ;;
    serve) serve_book "$@" ;;
    *) die "usage: book.sh setup | build | serve [PORT]" ;;
  esac
}

# Sourcing the file (the integrity test does) defines the functions only.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
