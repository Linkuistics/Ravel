#!/usr/bin/env bash
# shellcheck disable=SC2016 # the fixture scripts hold their own expansions
# Exercise the real prerequisite doctor against denied auth and macOS's Bash3.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/repo/scripts" "$scratch/repo/crates/ravel/scripts" "$scratch/bin" "$scratch/repo/target/tools"
cp "$root/scripts/release-doctor.sh" "$root/scripts/release-common.sh" "$scratch/repo/scripts/"
cp "$root/crates/ravel/scripts/dispatch.sh" "$scratch/repo/crates/ravel/scripts/"
cp "$root/scripts/release-book-version.sh" "$root/scripts/release-book.sh" "$root/scripts/book.sh" "$scratch/repo/scripts/"
page=docs/book/src/03-a-complete-session.md
mkdir -p "$scratch/repo/$(dirname "$page")"
cp "$root/$page" "$scratch/repo/$page"
printf '#!/bin/sh\nexit 70\n' >"$scratch/bin/gh"
printf '#!/bin/sh\necho 1.4.2\n' >"$scratch/repo/target/tools/bun"
printf '#!/bin/sh\necho "mdbook v%s"\n' "$(bash "$root/scripts/book.sh" version)" >"$scratch/repo/target/tools/mdbook"
chmod +x "$scratch/bin/gh" "$scratch/repo/target/tools/bun" "$scratch/repo/target/tools/mdbook"
export PATH="$scratch/bin:$PATH"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: doctor accepted denied auth' >&2
  exit 1
fi
grep -Fq 'gh: not authenticated' "$scratch/output"
grep -Fq '✓ book version line' "$scratch/output"
grep -Fq '✓ checkout-local mdBook' "$scratch/output"
# Unreadable Pages setting is a failure, and so is a missing or wrong mdBook.
grep -Fq '✗ book on Pages: cannot read the Pages configuration' "$scratch/output"
echo '#!/bin/sh
echo "mdbook v0.0.1"' >"$scratch/repo/target/tools/mdbook"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/wrong-mdbook" 2>&1; then
  echo 'FAIL: doctor accepted the wrong mdBook' >&2
  exit 1
fi
grep -Fq '✗ checkout-local mdBook' "$scratch/wrong-mdbook"
# Pages not enabled is the state before the first release, and a note.
printf '#!/bin/sh\n[ "$1" = auth ] && exit 0\necho "gh: Not Found (HTTP 404)"\nexit 1\n' >"$scratch/bin/gh"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/not-enabled" 2>&1; then :; fi
grep -Fq '✓ book on Pages: not enabled yet' "$scratch/not-enabled"
# Pages that serves another branch is a failure.
printf '#!/bin/sh\n[ "$1" = auth ] && exit 0\necho "{\\"build_type\\":\\"workflow\\",\\"source\\":{\\"branch\\":\\"main\\",\\"path\\":\\"/\\"}}"\n' >"$scratch/bin/gh"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/other-branch" 2>&1; then :; fi
grep -Fq '✗ book on Pages: Pages for Linkuistics/Ravel does not serve branch gh-pages' "$scratch/other-branch"
printf '#!/bin/sh\n[ "$1" = auth ] && exit 0\necho "{\\"build_type\\":\\"legacy\\",\\"source\\":{\\"branch\\":\\"gh-pages\\",\\"path\\":\\"/\\"}}"\n' >"$scratch/bin/gh"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/served" 2>&1; then :; fi
grep -Fq '✓ book on Pages: Pages serves branch gh-pages' "$scratch/served"
printf '#!/bin/sh\nexit 70\n' >"$scratch/bin/gh"
printf '#!/bin/sh\necho "mdbook v%s"\n' "$(bash "$root/scripts/book.sh" version)" >"$scratch/repo/target/tools/mdbook"
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
