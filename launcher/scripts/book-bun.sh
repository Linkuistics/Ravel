#!/usr/bin/env bash
# Checkout-only Bun for the book tools. No worker, SDK or installed runtime.
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly BUN_VERSION="1.4.2"
TOOLS="${BOOK_TOOLS_DIR:-$ROOT/target/tools}"
readonly TOOLS

bun_package() {
  case "$(uname -s)/$(uname -m)" in
    Darwin/arm64) echo bun-darwin-aarch64 ;;
    Linux/aarch64 | Linux/arm64) echo bun-linux-aarch64 ;;
    Linux/x86_64) echo bun-linux-x64 ;;
    *) die "no pinned book Bun for $(uname -s)/$(uname -m)" ;;
  esac
}

# Preserve the npm archive pins verified for 1.4.2 by the former worker setup.
# https://registry.npmjs.org/@oven/bun-darwin-aarch64/1.4.2 (and the Linux packages).
bun_sha256() {
  case "$1" in
    bun-darwin-aarch64) echo a9df486eaf7e9db9bdebb1fa425e8c9809abd783b1c518d1dbc5096a53b861ed ;;
    bun-linux-aarch64) echo 9ab3970a19660b5cd089f17fb021d900e1ca1b988dafd461d66d0a0ff4d6eac4 ;;
    bun-linux-x64) echo 0c75e0b94e9d56cece77abb7d03e9995b47b79506df5dcca3046eeb888db5934 ;;
    *) die "no pinned book Bun archive for '$1'" ;;
  esac
}

die() { echo "book-bun: $*" >&2; exit 1; }
digest_of() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d' ' -f1
  else
    sha256sum "$1" | cut -d' ' -f1
  fi
}

setup_bun() {
  local package expected archive scratch copy found
  package="$(bun_package)"
  expected="$(bun_sha256 "$package")"
  mkdir -p "$TOOLS"
  archive="$TOOLS/$package-$BUN_VERSION.tgz"
  scratch="$(mktemp -d "$TOOLS/.bun.XXXXXX")"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$scratch'" EXIT
  copy="$scratch/archive.tgz"
  # Verify and extract the same private copy, independent of cache writers.
  if [[ ! -f "$archive" ]] || ! cp "$archive" "$copy" || [[ "$(digest_of "$copy")" != "$expected" ]]; then
    if ! curl --fail --silent --show-error --location --output "$copy" \
      "https://registry.npmjs.org/@oven/$package/-/$package-$BUN_VERSION.tgz"; then
      die "cannot download the pinned $package book Bun $BUN_VERSION"
    fi
    found="$(digest_of "$copy")"
    [[ "$found" == "$expected" ]] ||
      die "the $package book Bun $BUN_VERSION has SHA-256 $found, but $expected is pinned; refusing to use it"
    cp "$copy" "$archive"
  fi
  tar -xzf "$copy" -C "$scratch" package/bin/bun || die "cannot extract Bun from $package"
  found="$("$scratch/package/bin/bun" --version)"
  [[ "$found" == "$BUN_VERSION" ]] || die "Bun $BUN_VERSION is pinned, found '$found'"
  install -m 0755 "$scratch/package/bin/bun" "$TOOLS/bun"
  echo "book-bun: Bun $BUN_VERSION development tool in $TOOLS"
}

main() {
  case "${1:-}" in
    setup) setup_bun ;;
    version) echo "$BUN_VERSION" ;;
    *) die 'usage: book-bun.sh setup | version' ;;
  esac
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then main "$@"; fi
