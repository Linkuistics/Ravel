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
printf '#!/bin/sh\nexit 70\n' >"$scratch/bin/gh"
printf '#!/bin/sh\necho 1.4.2\n' >"$scratch/repo/target/tools/bun"
chmod +x "$scratch/bin/gh" "$scratch/repo/target/tools/bun"
export PATH="$scratch/bin:$PATH"
if bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: doctor accepted denied auth' >&2
  exit 1
fi
grep -Fq 'gh: not authenticated' "$scratch/output"
# The supported release host is macOS arm64; its system Bash is the floor control.
if /bin/bash "$scratch/repo/scripts/release-doctor.sh" >"$scratch/bash3" 2>&1; then
  echo 'FAIL: doctor accepted Bash 3' >&2
  exit 1
fi
grep -Fq 'release builds require bash 4 or later' "$scratch/bash3"
echo 'release doctor controls: all passed'
