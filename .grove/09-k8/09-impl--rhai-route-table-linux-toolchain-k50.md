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
