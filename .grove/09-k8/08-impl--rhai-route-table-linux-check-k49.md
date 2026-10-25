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
