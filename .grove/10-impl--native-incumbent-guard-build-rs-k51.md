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
