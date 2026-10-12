#!/usr/bin/env bash
#
# One release target's installed smoke test, run inside that target's own
# environment. scripts/release-smoke.sh prepares the environment and runs this
# there, with no Bun or Node on PATH; it is not an entry point of its own.
#
#   release-smoke-target.sh TARGET ARCHIVE CASES [PROBES]
#
# It first asserts that it runs where TARGET runs: the system and machine, and
# on Linux a glibc of exactly 2.17 that is seen to refuse a newer binary (the
# probes release-smoke.sh builds into PROBES). Then it extracts ARCHIVE, named
# ravel-v<version>-<target>.tar.xz, into a fresh prefix with this environment's
# own tar, checks that ravel report that
# version, and hands the prefix to ravel's installed-layout cases,
# CASES (crates/ravel/scripts/installed-smoke.sh). Like those cases,
# it needs only bash 3.2, coreutils, grep and cmp, because macOS's /bin/bash
# and CentOS 7's both run it.

set -euo pipefail
IFS=$'\n\t'

fail() {
  echo "release-smoke: $TARGET: FAIL: $*" >&2
  exit 1
}

note() {
  echo "release-smoke: $TARGET: $*"
}

# `uname -sm` where TARGET's executables run.
target_system() {
  case "$1" in
    aarch64-apple-darwin) echo "Darwin arm64" ;;
    aarch64-unknown-linux-gnu) echo "Linux aarch64" ;;
    x86_64-unknown-linux-gnu) echo "Linux x86_64" ;;
    *) fail "no environment is known for this target" ;;
  esac
}

# The C library must be the floor, and must be seen to enforce it: the probe
# built against glibc 2.17 runs, and its twin, which needs getrandom's
# GLIBC_2.25 symbol version, is refused for that version. The twin's refusal is
# evidence only beside the floor probe's success, which shows this userland
# runs binaries of its architecture at all.
assert_glibc_floor() {
  local probes="$1" machine libc output status=0
  machine="$(uname -m)"
  libc="$(getconf GNU_LIBC_VERSION)" || fail "getconf reports no GNU C library"
  [[ "$libc" == "glibc 2.17" ]] || fail "this userland's C library is $libc, not the glibc 2.17 floor"
  output="$("$probes/floor-$machine" 2>&1)" ||
    fail "the probe built against glibc 2.17 did not run here: $output"
  [[ "$output" == "glibc probe ran" ]] || fail "the probe built against glibc 2.17 printed: $output"
  output="$("$probes/above-$machine" 2>&1)" || status=$?
  if ((status == 0)) || [[ "$output" != *"version \`GLIBC_2.25' not found"* ]]; then
    fail "positive control: the probe built against glibc 2.25 was not refused for its symbol version (exit $status): $output"
  fi
  note "$libc; control: the glibc 2.25 probe was refused (exit $status): $output"
}

main() {
  (($# == 3 || $# == 4)) || {
    echo "usage: release-smoke-target.sh TARGET ARCHIVE CASES [PROBES]" >&2
    exit 1
  }
  TARGET="$1"
  local archive="$2" cases="$3" probes="${4:-}"
  local system
  system="$(target_system "$TARGET")"
  [[ "$(uname -sm)" == "$system" ]] || fail "this is $(uname -sm), not $system, where this target runs"
  case "$TARGET" in
    *-linux-gnu)
      [[ -n "$probes" ]] || fail "a Linux target needs the glibc probes"
      assert_glibc_floor "$probes"
      ;;
    *-apple-darwin)
      note "$(uname -srm), macOS $(sw_vers -productVersion)"
      ;;
  esac

  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$scratch'" EXIT
  mkdir "$scratch/prefix" "$scratch/home" "$scratch/tmp"
  # Nothing outside the scratch directory: no personal configuration, and the
  # cases' own temporary files where the trap removes them.
  export HOME="$scratch/home" TMPDIR="$scratch/tmp"

  local top version reported
  top="$(basename "$archive" .tar.xz)"
  version="${top#ravel-v}"
  version="${version%-"$TARGET"}"
  [[ "$top" == "ravel-v$version-$TARGET" && -n "$version" ]] ||
    fail "$archive is not named ravel-v<version>-$TARGET.tar.xz"
  tar -xJf "$archive" -C "$scratch/prefix" || fail "$(tar --version | head -n 1) cannot extract $archive"
  [[ "$(ls -A "$scratch/prefix")" == "$top" ]] ||
    fail "$archive does not unpack to exactly $top/: $(ls -A "$scratch/prefix")"
  reported="$("$scratch/prefix/$top/bin/ravel" --version)" || fail "bin/ravel --version exited $?"
  [[ "$reported" == "ravel $version" ]] || fail "bin/ravel reports '$reported', not 'ravel $version'"
  note "extracted with $(tar --version | head -n 1); ravel report $version"
  "$BASH" "$cases" "$scratch/prefix/$top" "$version"
}

main "$@"
