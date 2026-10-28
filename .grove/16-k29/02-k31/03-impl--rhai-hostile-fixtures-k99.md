# rhai-hostile-fixtures-k99


## Goal

Retire the Bun hostile-worker fixtures after establishing an executable Rhai peer for every relevant hostile class.



## Context

- `crates/ravel/tests/hostile.rs` and the existing `rhai_*` integration suites, especially limits, role, reads, harness runs and policy validation.
- Shared fixture helpers may have moved in `rhai-runtime-fixtures-k98`; read current source.

## Done when

- Enumerate every old hostile class and record the exact Rhai case that covers it, or explain why it only tests the removed TypeScript runtime.
- Port relevant command/supervision cases to Rhai; remove Bun-only tests with each removal stated in the running log.
- The Rust test suites no longer execute a TypeScript policy except the retired-format tests. Shared TypeScript helpers with no consumers are removed.
- Focused hostile and Rhai suites, `task check` and `task test` pass. Repair moved book anchors and run both book checkers where source is quoted.

## Notes

Keep the TypeScript runtime available until the final runtime child, because executable book examples migrate next. The delivery leaf still owns worker sources, SDK, probe artifacts and delivery tasks; coordinate any test task whose target is removed so this child stays green.
