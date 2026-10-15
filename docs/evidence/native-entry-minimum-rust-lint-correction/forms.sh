#!/usr/bin/env bash
# usage: forms.sh OUTDIR   (repo root; TARGET_BASE names a scratch build dir) — compiled forms of the keyed-launch lib at the current tree, with the nightly toolchain
set -euo pipefail
out="$1"; mkdir -p "$out"
tc="$HOME/.rustup/toolchains/nightly-aarch64-apple-darwin"
S="${TARGET_BASE:-$(mktemp -d)}"
export PATH="$tc/bin:/usr/bin:/bin" RUSTUP_TOOLCHAIN=nightly-aarch64-apple-darwin CARGO_TARGET_DIR="$S/target-mir"
cargo build --locked -p keyed-launch >/dev/null 2>&1
libc="$(find "$S/target-mir/debug/deps" -name 'liblibc-*.rlib' | head -1)"
common=(--crate-name keyed_launch --edition=2021 crates/keyed-launch/src/lib.rs --crate-type lib
        --extern "libc=$libc" -L "dependency=$S/target-mir/debug/deps" -C metadata=fixed)
rustc "${common[@]}" -Zunpretty=mir >"$out/mir.txt"
rustc "${common[@]}" -Zunpretty=hir >"$out/hir.txt"
# Object code is deliberately not compared: `EntrySignals::new` is generic, so it is
# monomorphised in the crates that call it and no object of this crate contains it. A
# deliberate fault in the site left both objects byte-identical here (see the evidence
# note), which is why MIR and HIR, taken before monomorphisation, are the instrument.
