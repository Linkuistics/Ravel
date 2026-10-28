#!/usr/bin/env bash
# One GNU/Linux target's current development checks, run inside its environment
# (scripts/native-linux.sh campaign starts it). Everything it writes goes under
# OUT, which must not exist: one writer, never reused.
#
#   measure.sh OUT
#
# In order:
#   1. the domain: toolchain, kernel, libc and tool digests, as domain.json
#      records them for the campaign, plus the handled-signal domain read from
#      the target's headers and kernel, and the emulation control probe;
#   2. the declared minimum Rust building and linting the workspace: build with
#      every target (test binaries included), clippy with warnings denied, and
#      rustfmt's check;
#   3. development setup, the Rhai worker tests and the current native-entry
#      checks, including the private main-thread executable;
#   4. the complete current check and test tasks, with source digests unchanged.
#
# The frozen TypeScript incumbent and its R01 identity inventory belong to the
# archived native-entry campaign. Runtime contraction intentionally retires
# their Bun-specific tests, and the incumbent cannot run current Rhai fixtures.
# Keep its original evidence intact; this task measures the current product and
# makes no new claim about conformance to that frozen campaign.
#
# Each step's exit status is recorded and the script continues, so a failing
# step is read in full rather than hiding the steps after it; the script's own
# exit status is nonzero if any step's was.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[ $# -eq 1 ] || {
  echo "usage: measure.sh OUT" >&2
  exit 2
}
out="$1"
[ ! -e "$out" ] || {
  echo "measure: output exists (one writer, never reused): $out" >&2
  exit 1
}
mkdir -p "$out"
out="$(cd "$out" && pwd)"
cd "$root"

failed=0
status="$out/step-status.txt"
: >"$status"

# step NAME CMD...: run, log to OUT/NAME.txt, record the exit status.
step() {
  local name="$1" code=0
  shift
  printf 'measure: %s\n' "$name"
  date -u +%FT%TZ >"$out/$name.started"
  "$@" >"$out/$name.txt" 2>&1 || code=$?
  date -u +%FT%TZ >"$out/$name.finished"
  printf '%s\t%s\n' "$name" "$code" >>"$status"
  if [ "$code" -ne 0 ]; then
    printf 'measure: %s exited %s\n' "$name" "$code" >&2
    failed=1
  fi
}

fingerprint_sources() {
  find Cargo.toml Cargo.lock rust-toolchain.toml Taskfile.yml .cargo .config scripts crates docs CONTEXT.md README.md \
    -type f -not -path '*/node_modules/*' -not -path '*/target/*' -not -name '.DS_Store' -print0 |
    LC_ALL=C sort -z | xargs -0 sha256sum
}

fingerprint_sources >"$out/subjects-before.sha256"

# 1. The domain.
{
  echo "== uname"
  uname -a
  echo "== os-release"
  cat /etc/os-release
  echo "== libc"
  ldd --version | head -n 1
  echo "== rustc"
  rustc -vV
  echo "== cargo"
  cargo -V
  echo "== nextest"
  cargo nextest --version
  echo "== task"
  task --version
  echo "== tool digests (the image's own record)"
  cat /opt/native-linux/tools.txt
  echo "== emulation"
  echo "${NATIVE_CAMPAIGN_EMULATION:-none}"
  [ -z "${NATIVE_CAMPAIGN_EMULATION:-}" ] || /qemu/qemu --version | head -n 1
} >"$out/domain.txt" 2>&1 || failed=1

step signal-domain bash -c 'cc -O1 -o /tmp/signal-domain scripts/native-linux/signal-domain.c && /tmp/signal-domain'
step emulation-probe bash -c 'cc -O1 -o /tmp/emulation-probe scripts/native-linux/emulation-probe.c && /tmp/emulation-probe'

# 2. The declared minimum Rust builds and lints the workspace. The toolchain
# this image installs is exactly rust-version, so this is the minimum itself.
step rust-minimum-version bash -c 'grep -n "^rust-version" Cargo.toml && rustc --version'
step rust-build cargo build --locked --workspace --all-targets
step rust-clippy cargo clippy --locked --workspace --all-targets -- -D warnings
step rust-fmt cargo fmt --all --check

# 3. Current worker and native-entry checks. Delivery removal will also remove
# the unused worker/probe build prerequisites from setup and the Taskfile.
step worker-setup bash -c 'task setup && task worker && task probes'
# The Rhai policy worker: its limits, role and policy tests and every unit test
# that names it, through the real CLI. No nextest profile here writes junit, so
# the summary line of OUT/rhai-worker.txt is the record.
step rhai-worker cargo nextest run --locked -p ravel --no-fail-fast \
  -E 'binary(/^rhai_/) | test(/rhai/)'
step native-entry task test:native-entry

# 4. The current workspace, including the CLI and retained hostile tests.
step check task check
step test task test

fingerprint_sources >"$out/subjects-after.sha256"
step subjects-unchanged diff -u "$out/subjects-before.sha256" "$out/subjects-after.sha256"

printf 'measure: complete; failed=%s\n' "$failed"
exit "$failed"
