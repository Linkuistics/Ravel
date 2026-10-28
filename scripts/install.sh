#!/usr/bin/env bash
# Build and verify only the Ravel-owned files in a caller-selected prefix.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
[[ $# == 1 && -n "$1" ]] || { echo 'usage: install.sh PREFIX' >&2; exit 2; }
# shellcheck source=scripts/release-common.sh
source "$root/scripts/release-common.sh"
pin_rust_toolchain
# Match release-build's fixed Cargo output path, including with caller overrides.
export CARGO_TARGET_DIR="$root/target"
# --target overrides both build.target and CARGO_BUILD_TARGET. Stage that output,
# never a same-version stale binary from a previous unqualified build.
# https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options
host="$("$(resolved_rustc)" -vV | sed -n 's/^host: //p')"
[[ -n "$host" ]] || { echo 'install: cannot determine the Rust host target' >&2; exit 1; }
cargo build --release --locked --target "$host" -p ravel
task --dir "$root" notices
mkdir -p "$1"
prefix="$(cd "$1" && pwd -P)"
stage_installation "$root" "$root/target/$host/release/ravel" "$prefix"
# This directory was the old Ravel payload, never a caller configuration home.
rm -rf "$prefix/libexec/ravel"
id="$(cargo pkgid --quiet -p ravel)"
version="${id##*[#@]}"
env -i PATH=/usr/bin:/bin /bin/bash "$root/scripts/installed-smoke.sh" "$prefix" "$version"
echo "ravel: installed ravel $version under $prefix"
