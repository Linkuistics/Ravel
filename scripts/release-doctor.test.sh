#!/usr/bin/env bash
# Exercise the real prerequisite doctor against denied auth and macOS's Bash3.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/repo/scripts" "$scratch/repo/crates/ravel/scripts" "$scratch/bin" "$scratch/repo/target/tools"
cp "$root/scripts/release-doctor.sh" "$root/scripts/release-common.sh" "$scratch/repo/scripts/"
cp "$root/crates/ravel/scripts/dispatch.sh" "$scratch/repo/crates/ravel/scripts/"
cp "$root/scripts/release-book-version.sh" "$scratch/repo/scripts/"
page=docs/book/src/03-a-complete-session.md
mkdir -p "$scratch/repo/$(dirname "$page")"
cp "$root/$page" "$scratch/repo/$page"
printf '#!/bin/sh\nexit 70\n' >"$scratch/bin/gh"
printf '#!/bin/sh\necho 1.4.2\n' >"$scratch/repo/target/tools/bun"
chmod +x "$scratch/bin/gh" "$scratch/repo/target/tools/bun"
export PATH="$scratch/bin:$PATH"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: doctor accepted denied auth' >&2
  exit 1
fi
grep -Fq 'gh: not authenticated' "$scratch/output"
grep -Fq '✓ book version line' "$scratch/output"
# A book without the line the release replaces is a missing prerequisite.
sed -i.bak '/^ravel @RAVEL_BOOK_VERSION@$/d' "$scratch/repo/$page"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/no-line" 2>&1; then
  echo 'FAIL: doctor accepted a book without the version line' >&2
  exit 1
fi
grep -Fq '✗ book version line' "$scratch/no-line"
# The supported release host is macOS arm64; its system Bash is the floor control.
if /bin/bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/bash3" 2>&1; then
  echo 'FAIL: doctor accepted Bash 3' >&2
  exit 1
fi
grep -Fq 'release builds require bash 4 or later' "$scratch/bash3"
echo 'release doctor controls: all passed'
