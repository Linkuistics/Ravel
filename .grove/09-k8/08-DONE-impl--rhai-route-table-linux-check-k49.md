# rhai-route-table-linux-check-k49

## Goal

Run `task test:native-linux` once on the finished increment `rhai-route-table-k8`. Report the result. The brief asks for this run as the last check before the increment closes.

## Context

- The brief is `.grove/09-k8/_rhai-route-table.md`. Its Notes say the worker executes the running image on Linux and the canonical path on macOS.
- Every leaf of the increment is done, and none of them ran a Linux check. The producers `rhai-engine-limits-k10` and `rhai-try-boundary-limits-k46` and their integrations `rhai-engine-limits-k45` and `rhai-try-boundary-limits-k48` changed the worker. The last change is the replacement of the string-name `drain` and `retain` and of `order` and `order_by`.
- `task test:native-linux` takes `PLATFORM` (`arm64` runs natively in the Docker VM) and `OUT`, a directory that must not exist. The recipe and its limits are in `scripts/native-linux.sh`.
- The macOS suite is green: `task check` passes (447 tests). One test, `rhai_limits::a_native_call_that_does_not_yield_stops_at_the_deadline`, is flaky under machine load because its control asserts a 1 s wall-clock bound. A failure of that test alone on Linux needs a rerun on a quiet machine before it is called a finding.

## Done when

- `task test:native-linux PLATFORM=arm64 OUT=<new directory>` has run, and the result is in the running log.
- A failure that comes from the Rhai worker has a fix, or a leaf that names the gap. A failure of the campaign that has nothing to do with the worker is reported, not fixed here.
- The brief `rhai-route-table-k8` is checked against its `Done when`, and the node closes.

## Notes

- Do not run the amd64 variant unless the arm64 run points at an architecture problem. It runs under emulation and measures less.

## Decisions (running log)

- Ran `task test:native-linux PLATFORM=arm64 OUT=<scratchpad>/k49-arm64`. The task exited 1. Step status: `signal-domain` 0, `emulation-probe` 0, and `rust-minimum-version`, `rust-build`, `rust-clippy`, `rust-fmt`, `worker-setup`, `campaign` and `r01` all non-zero. The campaign measured nothing about the worker.
- Cause: `rust-toolchain.toml` (added by `rust-2024-toolchain-k7`) lists the targets `aarch64-apple-darwin` and `x86_64-unknown-linux-gnu`. The image installs only `aarch64-unknown-linux-gnu`, and `/opt/rustup` belongs to root. The unprivileged user's `rustup` fails with `could not create temp file /opt/rustup/tmp/...: Permission denied`. This is an environment gap and has nothing to do with the Rhai worker, so it is reported and not fixed here. The leaf `rhai-route-table-linux-toolchain-k50` names it.
- Diagnostic, not the campaign: with `RUSTUP_TOOLCHAIN=1.99.1-aarch64-unknown-linux-gnu` set in the left-up container, the Rhai tests of the `ravel` crate gave 107 passed and 1 failed of 108. Every `rhai_limits` test passed, and so did the flaky deadline test. The failure was `rhai_policy::a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path` with `worker_missing`, because `task worker` never built `target/libexec/ravel/ravel-policy`. No Rhai worker defect shows. The check stays open until k50 runs the campaign.
- The amd64 variant was not run, because nothing points at an architecture problem.
- The node `rhai-route-table-k8` does not close in this session: its brief asks for the Linux check, and k50 holds it.
