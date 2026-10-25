# rust-2024-toolchain-k7

## Goal

Move the workspace to Rust 1.99.1 and edition 2024. Rig 0.45.0 needs Rust 1.97.1 and edition 2024, so this increment comes first. It changes no behaviour of Ravel.

## Context

- The input is the section "Versions and toolchain" of `docs/specs/policy-evaluation.md`. The owner chose the newest stable Rust over Rig's own floor.
- The workspace now declares `rust-version = "1.85"` and edition 2021 in `Cargo.toml`. The crates `ravel` and `keyed-launch` both inherit them.
- Other files name the old floor. The Dockerfile `scripts/native-linux/Dockerfile` has `RUST_VERSION=1.85.0`. Search the scripts, the Taskfile and the book for `1.85` and for `edition`.
- The release scripts build with one Rust toolchain. See `pin_rust_toolchain` in `scripts/release-common.sh` and the toolchain checks in `scripts/release-doctor.sh`. The rustup default on this machine is 1.93.1, and the Homebrew Rust is 1.99.1.
- Add a `rust-toolchain.toml` only if the release scripts need it to select 1.99.1. Decide this from the scripts, and record the reason.
- Edition 2024 changes some rules. Examples are `unsafe` on `extern` blocks, `unsafe_op_in_unsafe_fn`, the scope of temporaries at the end of a block, and `set_var`. The runner uses `libc` heavily, so expect many fixes.
- Keep every Rust fix mechanical. Do not change the behaviour of the runner or of the TypeScript path.

## Done when

- `Cargo.toml` declares Rust 1.99.1 and edition 2024 for both crates.
- `task check` and `task test` pass with the 1.99.1 toolchain.
- The Dockerfile and every script or document that names the old floor name the new one.
- `task release:doctor` reports the toolchain as ready, or the leaf records the exact blocker for the owner.
- `task release:archives` builds the three archive targets with 1.99.1. If a target needs a new `rustup target add`, the leaf records the command.
- `task docs:excerpts` passes. Fix an excerpt that a mechanical change moved.

## Notes

- The target std libraries may be missing for 1.99.1. The release doctor reports this, and the fix is a `rustup` command.
- Do not run `task test:native-linux`. It is slow, and the later leaves use it.
