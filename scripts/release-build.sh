#!/usr/bin/env bash
#
# Build exact independent archives from a clean release tag or working copy.
# Usage: scripts/release-build.sh [--unreleased [TARGET...]]
# A target override still clears target/archives, so stale archives cannot pass.
# The working-copy workspace version determines the archive/front/worker version.
# The pinned Bun runtime and declarations come from crates/ravel/scripts/dispatch.sh;
# runtime provenance and supported floors: docs/runtime-evidence.md.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly ARCHIVES_DIR="$REPO_ROOT/target/archives"
readonly DISPATCH="$REPO_ROOT/crates/ravel/scripts/dispatch.sh"

# TARGETS, pin_rust_toolchain, archive_manifest and assert_archive — shared
# with the release tests, so the tests check this build's layout.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# Glibc floor for Linux targets — wide compatibility, RHEL 7-era.
readonly LINUX_GLIBC=2.17

die() {
  echo "release-build: $*" >&2
  exit 1
}

# The version every shipped crate inherits, read without a tag. `cargo pkgid`
# prints `<source>#<version>`, or `<source>#<name>@<version>` when the name
# differs from the directory.
workspace_version() {
  local id
  id="$(cargo pkgid --quiet -p ravel)" || die "cannot read ravel's package version"
  echo "${id##*[#@]}"
}

# The Bun compile target whose pinned runtime the worker for a release target
# is built from (see `runtime_package` in dispatch.sh).
bun_target() {
  case "$1" in
    aarch64-apple-darwin) echo bun-darwin-arm64 ;;
    aarch64-unknown-linux-gnu) echo bun-linux-arm64 ;;
    x86_64-unknown-linux-gnu) echo bun-linux-x64 ;;
    *) die "no Bun target for $1" ;;
  esac
}

build_target() {
  local target="$1" version="$2" out_dir="$3"
  # NOTE: the explicit `|| return 1`s are load-bearing. build_target runs inside
  # a command substitution (`archive="$(build_target ...)"`), and `set -e` does
  # NOT reliably abort a function in that context (notably under macOS's bash
  # 3.2). A bare failing cargo would otherwise fall through to the `cp` below
  # and tarball a STALE binary from a previous build. The caller also tests the
  # exit status. For the same reason everything but the archive path goes to
  # stderr: stdout is the caller's value.
  case "$target" in
    *-apple-darwin)
      cargo build --release --locked --target "$target" \
        -p ravel || return 1
      ;;
    *-unknown-linux-gnu)
      # The front has no system-library dependency beyond libc.
      cargo zigbuild --release --locked --target "${target}.${LINUX_GLIBC}" \
        -p ravel || return 1
      ;;
    *)
      die "unknown target: $target"
      ;;
  esac

  local top="ravel-v${version}-${target}" bun
  local stage="$out_dir/staging/$top"
  mkdir -p "$stage/bin" || return 1
  cp "$REPO_ROOT/target/$target/release/ravel" "$stage/bin/ravel" || return 1
  bun="$(bun_target "$target")" || return 1
  bash "$DISPATCH" build --target "$bun" "$stage/libexec/ravel" >&2 || return 1
  cp "$REPO_ROOT/LICENSE" "$REPO_ROOT/README.md" "$stage/" || return 1

  local archive="$out_dir/$top.tar.xz"
  if ! pack_archive "$out_dir/staging" "$top" "$archive.partial" ||
    ! assert_archive "$archive.partial" "$top" >&2; then
    rm -f "$archive.partial"
    return 1
  fi
  mv "$archive.partial" "$archive" || return 1
  echo "$archive"
}

main() {
  ((BASH_VERSINFO[0] >= 4)) || die "bash 4 or later is required; found $BASH_VERSION"
  cd "$REPO_ROOT"
  # Build with one coherent Rust toolchain, including cross-target std.
  pin_rust_toolchain
  echo "release-build: cargo $(command -v cargo || echo '(not found)')"
  # build_target copies from this directory, so cargo must build into it
  # whatever CARGO_TARGET_DIR or a `build.target-dir` setting would say.
  export CARGO_TARGET_DIR="$REPO_ROOT/target"

  local release=1 out_dir="$REPO_ROOT/target/dist" version targets=() target known tag
  if [[ "${1:-}" == --unreleased ]]; then
    release=0
    out_dir="$ARCHIVES_DIR"
    shift
    targets=("$@")
    ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
    for target in "${targets[@]}"; do
      for known in "${TARGETS[@]}"; do [[ "$target" == "$known" ]] && continue 2; done
      die "unknown target '$target'"
    done
    version="$(workspace_version)"
    echo "release-build: unreleased archives of the working copy at v$version"
  else
    (($# == 0)) || die 'usage: release-build.sh [--unreleased [TARGET...]]'
    [[ -z "$(git status --porcelain)" ]] || die 'working tree is dirty; return to the exact release revision'
    tag="$(git describe --tags --exact-match HEAD)" || die 'HEAD is not tagged; resume the cut before building'
    version="${tag#v}"
    [[ "$tag" == "v$version" && "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$version" != 0.0.0 ]] || die "not a release tag: $tag"
    [[ "$(workspace_version)" == "$version" ]] || die 'tag and workspace version differ'
    "$REPO_ROOT/scripts/release-doctor.sh"
    targets=("${TARGETS[@]}")
    echo "release-build: building ravel v$version"
  fi
  rm -rf "$out_dir"

  # The worker build emits its SDK declarations with the pinned type checker.
  task --dir "$REPO_ROOT" deps

  mkdir -p "$out_dir/staging"

  for target in "${targets[@]}"; do
    echo "release-build: target $target"
    local archive
    # Test the substitution explicitly — do not rely on `set -e` propagating out
    # of a command substitution (it does not, reliably). A failed build_target
    # must abort the release, never silently ship a stale tarball.
    if ! archive="$(build_target "$target" "$version" "$out_dir")"; then
      die "build failed for $target — see the output above"
    fi
    shasum -a 256 "$archive"
  done

  rm -rf "$out_dir/staging"

  if ((release)); then
    task --dir "$REPO_ROOT" probes
    bash "$REPO_ROOT/scripts/release-formula.sh" "$out_dir"
  fi

  echo
  echo "release-build: artifacts in $out_dir"
  ls -la "$out_dir"

}

main "$@"
