# rhai-runtime-fixtures-k98


## Goal

Port ordinary CLI and runtime fixtures to Rhai while both runtimes still work, so the later runtime contraction can remain green.



## Context

- Start with `crates/ravel/tests/cli.rs`, `session_process.rs`, `tests/support/mod.rs` and unit tests in `src/run.rs`, `context.rs`, `settings.rs`, `choice.rs`, `limits.rs`, `frame.rs`, `native_entry_tests.rs`, `partial_acquisition_tests.rs` and `worker_admission_tests.rs`.
- This path list is provisional source-search evidence, not an exhaustive inventory. The graph CLI refused access in the creating session; establish its status before graph claims.
- The approved policy spec and the Rhai suites show the current syntax and command seam.

## Done when

- All ordinary executable TypeScript fixtures under the Rust source and tests become Rhai, except cases reserved for retired-format behavior and the still-pending hostile suite.
- Preserve the observable behavior each case proves, including failure stages, process cleanup, native entry and confinement. Where a case proves only a TypeScript property, state its removal and reason in the running log.
- Shared support names and setup describe Rhai accurately. Do not break consumers that still require the TypeScript helper in the hostile suite; remove it only once unused.
- Focused affected suites, `task check` and `task test` pass. Re-anchor any moved book excerpt and run both book checkers if quoted source moved.

## Notes

Keep both runtime paths. The final runtime child owns format refusal, snapshot-validator deletion and notice schema contraction. Do not prematurely change runtime discovery or production behavior to make this migration pass.
