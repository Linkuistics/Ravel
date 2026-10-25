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

## Decisions (running log)

**The workspace declares Rust 1.99.1 and edition 2024.** Both crates inherit the two values from `Cargo.toml`. `cargo fix --edition` changed one site. It wrapped the test-only `set_var` in `native_entry_tests.rs` in `unsafe`. I replaced its placeholder `FIXME` with a `SAFETY` comment. The role process starts no thread before that call. The only `thread::spawn` in that file belongs to the parent test.

**The drop-order warning needs no change.** The lint `tail-expr-drop-order` named the final `match` of `native_barrier.rs`. Both arms move the `Child` or the `String` out of the scrutinee, so no destructor runs in a new order. The lint stops once the edition is 2024.

**The 2024 style edition reformats the code, and the leaf accepts that.** `cargo fmt` changed long string arguments, `assert!` layout and trailing semicolons. These changes are mechanical. `task docs:excerpts` still passes after them.

**Clippy 1.99 collapses nested conditions into let chains.** Let chains are stable in edition 2024. The deny-level `collapsible_if` fired on five sites. `cargo clippy --fix` rewrote them. Each rewrite keeps the order of evaluation and has no `else` branch.

**The leaf adds `rust-toolchain.toml`.** The release scripts put rustup's shims first on `PATH`, and the shims read this file. Without it, the shims select the rustup default, which is 1.93.1 on this machine. The file names 1.99.1, the components `clippy` and `rustfmt`, and the three release targets. One `rustup toolchain install` in the checkout then installs everything. No separate `rustup target add` is needed. Cargo 1.93.1 did not refuse the new `rust-version` for a workspace member, so the file selects the compiler and nothing enforces the floor. Plain `cargo` from Homebrew ignores the file, and Homebrew's cargo is already 1.99.1.

**The old floor stays where it records a past measurement.** The evidence documents under `docs/` and the book passages that say "the measurement used Rust 1.85.0" are history, and they stay. The leaf changed the statements that call 1.85 the current floor: the Dockerfile, the setup sentences of chapters 4, 10 and 11, the "declared minimum" words in chapter 1, the sentence in `result-job-completion.md`, and the comment in `signal_state.rs`. For that comment the leaf read the 1.99.1 standard library source. `reset_sigpipe` and the order in `do_exec` are unchanged, at the paths that the comment cites.

**Rust 1.99 broke the `aarch64-unknown-linux-gnu` link under cargo-zigbuild 0.22.3.** The target spec of 1.99.1 passes `-Wl,--fix-cortex-a53-843419`. The spec of 1.93.1 does not. Zig 0.17.0 answers `unsupported linker arg`. cargo-zigbuild filters the flag from 0.23.3 on, and the leaf installed 0.23.4 with `cargo install cargo-zigbuild --locked --version 0.23.4 --force`. `scripts/release-doctor.sh` now fails with that remedy when the installed version is older than 0.23.3. A control with a higher minimum made the check fail, so the check works. Chapter 11 names the minimum version.

## Result

- `task check` passes: eight steps, and nextest ran 339 tests with none failed. `task test` passes with the same 339 tests.
- `task release:doctor` reports all prerequisites met, with rustc 1.99.1 and the three target libraries.
- `task release:archives` built the three archives with 1.99.1. They are for version 1.1.0, because no cut was made.
- `task docs:excerpts` passes: 8 excerpts, 0 failed.
- The machine needed `rustup toolchain install` in the checkout and the cargo-zigbuild upgrade above. This workspace also needed `task setup` for the pinned Bun 1.4.2.
- `task release:lifecycle:test` fails in `release-doctor.test.sh`, and this leaf did not cause it. The test deletes the line `ravel @RAVEL_BOOK_VERSION@` from the book page. The committed page holds the released line `ravel 1.1.0`, so the `sed` deletes nothing and the doctor correctly finds the line. The release leaf `release-ravel-2-0-0-k37` should expect this failure, or repair the test to delete the line of either form. The doctor itself works.
- `task test:native-linux` was not run, as the leaf directs.
