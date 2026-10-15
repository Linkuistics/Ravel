#!/usr/bin/env bash
# Private test binaries only: no runtime testing switch or real model service.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
receipt_dir="$(mktemp -d "${TMPDIR:-/tmp}/ravel-c4.XXXXXXXX")"
trap 'rm -rf "$receipt_dir"' EXIT
export C4_RECEIPTS="$receipt_dir"
cargo nextest run --locked -p keyed-launch -E 'test(terminal_recovery)' --no-capture
cargo nextest run --locked -p ravel --bin ravel \
  -E 'test(terminal_recovery_consumes_pty_receipts)' --run-ignored only --no-capture
