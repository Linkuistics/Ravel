#!/usr/bin/env bash
# One clippy reading, pinned to a named toolchain, with the dispatched clippy
# version written at the top of the log. Run from the repository root.
#
#   lint.sh TOOLCHAIN MODE OUTFILE [TARGET_DIR]
#
# MODE: deny    cargo clippy --locked --workspace --all-targets -- -D warnings
#       faults  the same with --features native-faults
#       enum    the same, every lint demoted to a warning by RUSTFLAGS, so a
#               failing crate does not hide the sites behind it from the reading
#
# Why RUSTUP_TOOLCHAIN and not PATH: cargo looks a subcommand up in
# ~/.cargo/bin (rustup's proxies) before PATH, so `cargo clippy` run with a
# toolchain's bin first on PATH still dispatched to the default toolchain's
# clippy (0.1.93 here). That produced four "clean" readings at 1.85 before the
# header line below was added; the header is what makes a reading mean something.
set -uo pipefail
toolchain="$1" mode="$2" out="$3"
target="${4:-$(mktemp -d)}"
tc="$HOME/.rustup/toolchains/$toolchain"
export PATH="$tc/bin:/usr/bin:/bin"
export CARGO_TARGET_DIR="$target"
unset RUSTC RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
export RUSTUP_TOOLCHAIN="$toolchain"
{
  echo "# cargo:  $(cargo --version)"
  echo "# rustc:  $(rustc --version)"
  echo "# clippy: $(cargo clippy --version)   <- what cargo clippy dispatches to"
  echo "# mode:   $mode"
  case "$mode" in
    deny)   echo '# cmd: cargo clippy --locked --workspace --all-targets -- -D warnings'
            cargo clippy --locked --workspace --all-targets -- -D warnings ;;
    faults) echo '# cmd: cargo clippy --locked --workspace --all-targets --features native-faults -- -D warnings'
            cargo clippy --locked --workspace --all-targets --features native-faults -- -D warnings ;;
    enum)   echo '# cmd: RUSTFLAGS="-W clippy::all" cargo clippy --locked --workspace --all-targets'
            RUSTFLAGS="-W clippy::all" cargo clippy --locked --workspace --all-targets ;;
  esac
  echo "# exit: $?"
} >"$out" 2>&1
tail -1 "$out"
