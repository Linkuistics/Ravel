# native-incumbent-guard-build-rs-k51

## Goal

Make `task test:native-linux PLATFORM=arm64` complete its `campaign` and `r01` steps again, or retire the native-entry campaign from that task with a stated reason. The campaign stops while it builds the frozen incumbent, so it measures nothing.

## Context

- Measured in `rhai-route-table-linux-toolchain-k50`, in two runs on Linux arm64 at Rust 1.99.1. Both runs ended at `campaign exited 201` and `r01 exited 1`. The log is `campaign/logs/incumbent-build.txt` in each run directory. It reads: `the incumbent's crates/ravel/build.rs differs from the repaired tree by more than the pinned hex rewrite`. The message ends with `its worker would have to be built separately`.
- Cause: `worker_identity` in `scripts/native-incumbent.sh` accepts exactly one difference in `crates/ravel/build.rs`. It is the `hex` rewrite of `native-entry-minimum-rust-lint-correction-k51`, pinned by the SHA-256 of the `diff` output. The commit `358b63c` (`rhai-engine-worker-k42`) added `embed_crate_digest` and `collect_rust` to the same file, and it changed `hex` again. The diff no longer matches the pin.
- The added code embeds `RAVEL_BUILD_ID`, a digest of the crate sources. It does not change `RAVEL_WORKER_BUILD_ID`, the digest of the TypeScript worker sources. The incumbent reuses the compiled TypeScript worker, so the guard's purpose still holds. Check this claim before relying on it.
- The `r01` failure follows from the missing incumbent binary (`target/native-entry-fixture/debug/ravel` was never built).
- The campaign selects only `native_` cases. It never ran a Rhai test. `scripts/native-linux/measure.sh` now has a `rhai-worker` step for that. In k50 it ran 108 of 108 tests green on Linux arm64.
- The `typescript-removal` increment deletes the TypeScript worker. After that, the incumbent cannot share a compiled worker with the repaired tree.

## Done when

- Decide between three ways, and record the reason in the running log:
  - re-pin the guard to the new `build.rs` diff, with the controls the pin already has.
  - make the guard compare only what decides the worker's identity.
  - build the incumbent's worker separately.
- The chosen way keeps the guard's failing controls. `bash scripts/native-incumbent.sh selftest` passes and fails where it must.
- `task test:native-linux PLATFORM=arm64 OUT=<new directory>` has run. `campaign` and `r01` either pass or fail on a measured row, and not at the incumbent build.
- If the campaign cannot outlive `typescript-removal`, the leaf says what deletes it and when.

## Notes

- `OUT` must not exist. Use a directory outside the repository.
- After the run, `bash scripts/native-linux.sh down arm64`.
- `crates/ravel/build.rs` changes again in later increments. A pin by digest needs a new value each time. Weigh that against the other two ways.
- Do not change `rust-toolchain.toml`.

## Decisions (running log)

- Chose the second way, and made it measure rather than read. `build.rs` leaves `worker_paths`. The compiled worker is a function of `crates/ravel/worker` and `crates/ravel/scripts/dispatch.sh`, and the guard still compares those byte for byte. `build.rs` compiles nothing. It computes the digest `RAVEL_WORKER_BUILD_ID`, which the front compares with the digest `dispatch.sh` stamps into the worker. The claim that the added `embed_crate_digest` and `collect_rust` leave that digest alone holds: the diff of `build.rs` against revision `0515419edd29` touches `RAVEL_BUILD_ID` code and `hex` only, and `source_files` and `collect_typescript` are identical.
- Rejected re-pinning. The pin names a digest of the whole `build.rs` diff, so each later edit of `build.rs` needs a new value, and `typescript-delivery-removal-k32` and the `RAVEL_BUILD_ID` work will edit it. A pin that changes whenever an unrelated function changes does not say what it guards.
- Rejected building the incumbent's worker separately. It needs Bun in the Linux image and a second compile, to fix a guard whose purpose already holds. It becomes the only way to keep the campaign after `typescript-delivery-removal-k32`, and that leaf decides it (see the last decision).
- New check, `verify_pairing`, runs after the incumbent front builds. It asks `dispatch.sh build-id` for the digest and requires the literal in both the shared compiled worker and the incumbent front. Measured on the host: the worker and the current front carry it, and the digest with a changed last character is not found. This replaces what the pin stood in for, and it also catches an incumbent `build.rs` that lists other files than `dispatch.sh` does.
- Controls in `selftest`, each seen to fail by breaking the guard on a scratch copy: a changed TypeScript source and a changed `dispatch.sh` are rejected; any `build.rs` is accepted; a file without the digest, an empty file and an empty digest are refused; a front or a worker without it is not paired.
- The guard failure had hidden a second one: `instruments.patch` did not apply, because `rust-2024-toolchain-k7` reformatted the `eprintln!` in `native_entry_tests.rs` that one hunk replaces. That hunk now carries the reformatted context. The two assemblies of the tree are still identical.
- The first Linux run with the new guard passed the guard and then failed in the compiler: the instrument `crates/keyed-launch/src/native_barrier.rs` uses let chains, and the incumbent crates are edition 2021. `instruments.patch` now carries two adaptations in that file, a `filter` in `reap` and a nested `if` in `row`. Both keep the behaviour.
- The second run built the incumbent and failed 7 of 53 baseline tests on `worker_identity_mismatch`. The front pairs with its worker on three things: protocol, build digest and package version. The digest agreed, as `verify_pairing` had shown. The package version did not: the worker reports 1.1.0, the repaired tree's, and the incumbent front needs 1.0.0, its own. So "the digest decides the worker's identity" was true but incomplete, and only a run showed it.
- `align_version` now sets the incumbent's `[workspace.package]` version and the two `Cargo.lock` entries to the repaired tree's version when the tree is assembled, and it fails unless both read back. The version is read from the repaired `Cargo.toml`, not pinned, so the 2.0.0 release needs no edit. The cost is that the incumbent test build reports the repaired version. No baseline instrument reads it. The script header says so. Controls: the assembled tree carries the repaired version in both files, and a tree whose version cannot be set is rejected. The second control was seen to fail with the read-back check removed.
- Result, third run, `task test:native-linux PLATFORM=arm64 OUT=<scratchpad>/k51-run3` on Linux arm64 at Rust 1.99.1: every step exited 0 and `failed=0`. `rhai-worker` ran 108 of 108. `campaign` reconciled 153 of 153 rows with no unreconciled row (`native_steps_failed`, `subjects_moved` and `rows_unreconciled` all false), and the incumbent baseline ran 53 of 53 tests. `r01` checked 290 of 290 in scope (310 in scope, 20 ignored fixtures). The container was removed with `native-linux.sh down arm64`. On the host the incumbent also builds and the baseline passes 53 of 53.
- The campaign cannot outlive `typescript-delivery-removal-k32`. That leaf deletes `crates/ravel/worker` and `crates/ravel/scripts/dispatch.sh`. The incumbent shares the compiled worker built from those files, and `worker_identity`, `worker_digest` and `verify_pairing` read them, so the guard would refuse and the shared worker could not be built. The leaf already lists `scripts/native-incumbent` among its consumers. It must either retire the native-entry campaign from `task test:native-linux` and `scripts/native-incumbent.sh`, with the 153 reconciled rows frozen as evidence, or build the incumbent's TypeScript worker separately from its own revision, which brings Bun into the image and `align_version` becomes unnecessary. Keeping the TypeScript path only for a frozen baseline is the cost of the second way. This leaf does not choose between them.
