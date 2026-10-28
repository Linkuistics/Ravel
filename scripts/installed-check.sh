#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p target
prefix="$(mktemp -d "$root/target/installed café.XXXXXX")"
# shellcheck disable=SC2064 # remove only the private fresh prefix
trap "rm -rf '$prefix'" EXIT
# An upgrade removes the old Ravel-owned payload and preserves other tools.
mkdir -p "$prefix/libexec/ravel/sdk" "$prefix/bin"
printf 'retired payload\n' >"$prefix/libexec/ravel/sdk/index.ts"
printf 'unrelated tool\n' >"$prefix/bin/other-tool"
# The caller's cross-target preference must not change this native installation.
CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu bash scripts/install.sh "$prefix"
[[ "$(cat "$prefix/bin/other-tool")" == 'unrelated tool' ]]
