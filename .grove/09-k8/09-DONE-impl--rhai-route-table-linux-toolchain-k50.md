# rhai-route-table-linux-toolchain-k50

## Goal

Make `task test:native-linux PLATFORM=arm64` reach the Rhai worker, then run it once and report the result. The run in `rhai-route-table-linux-check-k49` failed before it built anything.

## Context

- Cause, measured in k49: `rust-toolchain.toml` lists the targets `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-gnu`. The leaf `rust-2024-toolchain-k7` added that file. The campaign image (`scripts/native-linux/Dockerfile`) installs only the native Linux target, and `/opt/rustup` belongs to root. Inside the container, `rustup` tries to install the two missing targets as the unprivileged user and stops with `could not create temp file /opt/rustup/tmp/...: Permission denied`.
- Result: `rust-minimum-version`, `rust-build`, `rust-clippy`, `rust-fmt`, `worker-setup`, `campaign` and `r01` all exited non-zero. Only `signal-domain` and `emulation-probe` passed. No step measured the worker. The `r01` and `campaign` failures follow from the missing build, and so does the one TypeScript-path test that failed (`worker_missing`).
- A diagnostic outside the campaign bypassed the toolchain file with `RUSTUP_TOOLCHAIN=1.99.1-aarch64-unknown-linux-gnu`. On Linux arm64 under Rust 1.99.1, 107 of 108 Rhai-related tests passed. That includes every test in `rhai_limits` and the flaky `a_native_call_that_does_not_yield_stops_at_the_deadline`. The one failure was `rhai_policy::a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path`, which needs the Bun worker that `task worker` could not build. This diagnostic is not the campaign and does not close the check.
- The earlier native-entry campaign passed on this image before k7 added the toolchain file.

## Done when

- The image or the campaign environment lets `rustup` find 1.99.1 with its components and targets without installing anything at run time. Two ways are open: install all listed targets when the image is built, or give the campaign a toolchain choice that ignores the foreign targets. Pick the one that leaves `rust-toolchain.toml` and the macOS path unchanged.
- `task test:native-linux PLATFORM=arm64 OUT=<new directory>` has run, and its result is in the running log.
- A failure that comes from the Rhai worker has a fix, or a leaf that names the gap. A failure that has nothing to do with the worker is reported.
- The brief `rhai-route-table-k8` is checked against its `Done when`, and the node closes.

## Notes

- `OUT` must not exist. Use a directory outside the repository (the scratchpad), because the campaign copies large logs.
- `task test:native-linux` leaves the container up. Run `bash scripts/native-linux.sh down arm64` afterwards.
- Do not run the amd64 variant unless the arm64 run points at an architecture problem.
- One test, `rhai_limits::a_native_call_that_does_not_yield_stops_at_the_deadline`, asserts a 1 s wall-clock bound. A failure of that test alone needs a rerun on a quiet machine before it is called a finding.

## Decisions (running log)

- **Image, not environment.** `scripts/native-linux/Dockerfile` passes `-t` for all three targets of `rust-toolchain.toml` to `rustup-init`. The other way, a toolchain override in the campaign, would bypass the toolchain file that the check should exercise. `rust-toolchain.toml` and the macOS path are unchanged.
- **Run 1** (`OUT` `scratchpad/k50-arm64`, Linux arm64, Rust 1.99.1, image rebuilt): `rust-build`, `rust-clippy`, `rust-fmt` and `worker-setup` pass. `campaign` exits 201 and `r01` exits 1. Both stop at the incumbent guard in `scripts/native-incumbent.sh`, which refuses the `build.rs` that `358b63c` extended. The campaign selects only `native_` cases, so it would never have run a Rhai test even past the guard.
- **Added a `rhai-worker` step** to `scripts/native-linux/measure.sh`. It runs the `rhai_*` test binaries and every test that names `rhai`.
- **Run 2** (`OUT` `scratchpad/k50-arm64-b`): `rhai-worker` passes, 108 of 108, including `a_native_call_that_does_not_yield_stops_at_the_deadline` (3.1 s) and the TypeScript-path test `a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path`. `campaign` and `r01` fail as in run 1.
- **The guard failure is not a worker defect.** It is an instrument that pins one diff of `build.rs`. Cut `native-incumbent-guard-build-rs-k51` before `rhai-local-capabilities-k11`, because later leaves (k24, k31) expect the task to pass.
- **Host checks.** `task check` passes (447 of 447). `task test` failed once on `partial_acquisition_tests::partial_acquisition_cleanup_worker_timeout_remove_wait` and passed alone and on the rerun of the whole suite (447 of 447). It does not touch the Rhai worker. Treat one recurrence as a finding.
- **Brief `rhai-route-table-k8` against its `Done when`.** The route table, the notice and the hostile-script limits were delivered and reviewed by k9, k10, k44 to k48. The TypeScript path works, and `task check` and `task test` pass (this session). The Linux check of the worker holds through the `rhai-worker` step. The node closes. The campaign repair stays with k51.
