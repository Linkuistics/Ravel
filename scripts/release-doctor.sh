#!/usr/bin/env bash
#
# Verify the prerequisites for running the release pipeline. Installs
# nothing; the one thing it does change is its own PATH, pinning the Rust
# toolchain exactly as release-build.sh does (see release-common.sh) so
# that what it checks is what the build will get. Emits a punch list of
# every missing item with its remediation command, then exits non-zero if
# anything is missing.
#
# Designed to run twice: standalone before committing to a release
# attempt, and (cheaply) as the first step of release-build.sh so the
# build fails fast on a misconfigured machine instead of mid-toolchain.

set -euo pipefail
IFS=$'\n\t'
trap 'echo "release-doctor: error on line $LINENO" >&2' ERR

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT

# TARGETS, pin_rust_toolchain, resolved_rustc, target_std_installed.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# Pin the toolchain before checking anything, so every check below sees the
# same PATH release-build.sh will build under. Standalone this is the fix; run
# from release-build.sh it is a no-op, because that script pinned already and
# exported PATH.
pin_rust_toolchain

failed=0

mark_pass() {
  echo "  ✓ $*"
}

mark_fail() {
  echo "  ✗ $*"
  failed=1
}

remediation() {
  echo "      remediation: $*"
}

note() {
  echo "      note: $*"
}

check_zig() {
  if ! command -v zig >/dev/null 2>&1; then
    mark_fail "zig: not on PATH"
    remediation "brew install zig"
    return
  fi
  local version
  version="$(zig version 2>/dev/null || echo unknown)"
  mark_pass "zig: $version"
}

check_cargo_zigbuild() {
  if cargo zigbuild --help >/dev/null 2>&1; then
    mark_pass "cargo-zigbuild: available"
  else
    mark_fail "cargo-zigbuild: not installed"
    remediation "cargo install cargo-zigbuild"
  fi
}

# Set by check_rust_toolchain, consumed by check_target_std. Empty means the
# toolchain check failed and the target check has nothing to interrogate.
rustc_bin=""
rustc_is_rustup=0

# Report the toolchain the BUILD will use, not the one rustup reports.
#
# This distinction is the whole point of the check. `rustup target list
# --installed` describes rustup's toolchain; `cargo build` uses whatever cargo
# and rustc resolve to on PATH, which on a machine with Homebrew's `rust`
# formula is a different toolchain entirely, with a different set of installed
# target stds. Asking the wrong tool is how this doctor printed "all
# prerequisites met" and the build then died on a missing `std`.
check_rust_toolchain() {
  local cargo_bin shim_dir
  cargo_bin="$(command -v cargo || true)"
  rustc_bin="$(resolved_rustc)"
  shim_dir="$(rustup_shim_dir)"

  if [[ -z "$cargo_bin" || -z "$rustc_bin" ]]; then
    mark_fail "rust toolchain: cargo and/or rustc not on PATH"
    remediation "install rustup from https://rustup.rs/"
    rustc_bin=""
    return
  fi

  mark_pass "cargo: $cargo_bin ($("$cargo_bin" --version 2>/dev/null | cut -d' ' -f1-2))"
  mark_pass "rustc: $rustc_bin ($("$rustc_bin" --version 2>/dev/null | cut -d' ' -f1-2))"

  # Informational, never a failure: a non-rustup toolchain carrying all three
  # target stds builds a release perfectly well, and check_target_std is the
  # gate that says so. This line exists to sharpen the remediation when that
  # gate does fail.
  if [[ "$rustc_bin" == "$shim_dir/"* ]]; then
    rustc_is_rustup=1
  fi
}

# The load-bearing check: does the resolved toolchain actually carry a std for
# every target the build will ask for?
check_target_std() {
  if [[ -z "$rustc_bin" ]]; then
    return
  fi
  local target
  for target in "${TARGETS[@]}"; do
    if target_std_installed "$rustc_bin" "$target"; then
      mark_pass "target std: $target"
    else
      mark_fail "target std: $target has no std in the rustc above"
      remediation "rustup target add $target"
      if ((rustc_is_rustup == 0)); then
        note "no rustup shim at $(rustup_shim_dir), so the build falls back to"
        note "the rustc above — any target rustup has already installed is"
        note "invisible to it. Install rustup, or set CARGO_HOME to where it is."
      fi
    fi
  done
}

check_gh_auth() {
  if ! command -v gh >/dev/null 2>&1; then
    mark_fail "gh: not installed"
    remediation "brew install gh && gh auth login"
    return
  fi
  if gh auth status >/dev/null 2>&1; then
    mark_pass "gh: authenticated"
  else
    mark_fail "gh: not authenticated"
    remediation "gh auth login"
  fi
}

# `scripts/check.sh`, which the release runs, compiles ravel's
# policy worker with exactly one Bun release. The pin lives in the worker's
# build script, and is read from there rather than restated here.
check_bun() {
  local pinned version
  pinned="$(sed -n 's/^readonly BUN_VERSION="\(.*\)"$/\1/p' \
    "$REPO_ROOT/crates/ravel/scripts/dispatch.sh")"
  local runtime="$REPO_ROOT/target/tools/bun"
  if [[ -x "$runtime" ]] && version="$("$runtime" --version)" && [[ "$version" == "$pinned" ]]; then
    mark_pass "checkout-local Bun: $version"
  else
    mark_fail "checkout-local Bun $pinned unavailable; no host Bun is required"
    remediation "task setup"
  fi
}

# The release smoke-tests every archive before it publishes anything
# (release-smoke.sh): macOS arm64 natively, and the Linux targets in Docker,
# whose QEMU and interpreters are pinned for arm64 and whose private
# binfmt_misc instance needs kernel 6.7 or later. Checked here, before the cut,
# so that a missing Docker does not stop a release whose version is already
# cut.
check_smoke_hosts() {
  local host platform kernel major minor
  host="$(uname -sm)"
  if [[ "$host" == "Darwin arm64" ]]; then
    mark_pass "host: $host, where the macOS arm64 archive runs natively"
  else
    mark_fail "host: $host, but the macOS arm64 archive's smoke test runs natively on Darwin arm64"
    remediation "run the release on an Apple silicon Mac"
  fi
  if ! command -v docker >/dev/null 2>&1; then
    mark_fail "docker: not on PATH"
    remediation "install Docker Desktop (https://docs.docker.com/desktop/)"
    return
  fi
  if ! platform="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}' 2>/dev/null)"; then
    mark_fail "docker: the daemon is not running"
    remediation "start Docker"
    return
  fi
  kernel="$(docker info --format '{{.KernelVersion}}' 2>/dev/null || echo unknown)"
  IFS=. read -r major minor _ <<<"$kernel"
  if [[ "$platform" != linux/arm64 ]]; then
    mark_fail "docker: $platform, but the Linux smoke tests' QEMU is pinned for linux/arm64"
    remediation "use an arm64 Docker, such as Docker Desktop on Apple silicon"
  elif ! [[ "$major" =~ ^[0-9]+$ && "$minor" =~ ^[0-9]+$ ]] || ((major < 6 || (major == 6 && minor < 7))); then
    mark_fail "docker: kernel $kernel, but a private binfmt_misc instance needs 6.7 or later"
    remediation "update Docker Desktop"
  else
    mark_pass "docker: $platform, kernel $kernel"
  fi
}

# The cut replaces one line of the book with the release version
# (release.toml). Checked here so that a missing line does not stop a release
# whose version is already cut.
check_book_version() {
  local message
  if message="$("$REPO_ROOT/scripts/release-book-version.sh" ready 2>&1)"; then
    mark_pass "book version line: ${message#release-book-version: }"
  else
    mark_fail "book version line: ${message#release-book-version: }"
    remediation "restore the one line 'ravel @RAVEL_BOOK_VERSION@' (or the last release's 'ravel X.Y.Z') in docs/book/src/03-a-complete-session.md"
  fi
}

check_release_commands() {
  local program
  for program in jj jq brew task curl tar xz shasum shellcheck; do
    if command -v "$program" >/dev/null 2>&1; then
      mark_pass "$program: available"
    else
      mark_fail "$program: not on PATH"
      remediation "install $program before running a release task"
    fi
  done
  if [[ "$(cargo release --version 2>/dev/null || true)" == 'cargo-release 1.1.2' ]]; then
    mark_pass 'cargo-release: verified 1.1.2'
  else
    mark_fail 'cargo-release: not installed'
    remediation 'cargo install cargo-release --version 1.1.2 --locked'
  fi
}

main() {
  echo "release-doctor: checking release prerequisites"
  echo

  if ((BASH_VERSINFO[0] >= 4)); then
    mark_pass "bash: $BASH_VERSION"
  else
    mark_fail "bash: $BASH_VERSION, but release builds require bash 4 or later"
    remediation 'brew install bash; put its bin directory before /bin on PATH'
  fi
  check_rust_toolchain
  check_target_std
  check_zig
  check_cargo_zigbuild
  check_gh_auth
  check_bun
  check_smoke_hosts
  check_book_version
  check_release_commands

  echo
  if ((failed == 0)); then
    echo "release-doctor: all prerequisites met"
    exit 0
  fi
  echo "release-doctor: missing prerequisites — fix the items marked above" >&2
  exit 1
}

main "$@"
