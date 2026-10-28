# rhai-native-fixtures-k103

## Goal

Port `native_entry_tests.rs`, `partial_acquisition_tests.rs` and `worker_admission_tests.rs` to Rhai without losing their fault discriminators or cleanup and native-entry assertions.

## Context

The parent k98 running log records the graph refusal. Establish graph availability again; no graph evidence exists for this handoff. Source reads found ordinary policies beside fake protocol workers: native E08 installs a script speaking the TypeScript protocol; admission and acquisition use test executables which do not dispatch the Rhai role by argv[0]. The existing Rhai `front.rs` tests use `rhai_worker::role::tests::role_fixture` through a shell wrapper. The actual private native-main binary supports the real role. Inspect these seams before changing formats, and retain each injected failure, stage, diagnostic and no-launch assertion. `context.rs`, `settings.rs`, `choice.rs`, `limits.rs`, `frame.rs` and `run.rs` must also be checked for any remaining fixture work after k102.

During k102 verification, `partial_acquisition_cleanup_worker_timeout_remove_wait` passed in `task check`, then failed in a separate full test run and an isolated run while build work was active: the 1000 ms timeout arrived before the expected owner diagnostics (once before worker readiness). It passed alone after build/book work finished. Host load averages reached 31.80, 17.39, 12.61. The unchanged fixture assumes startup reaches its diagnostic prints before the whole selection deadline. Preserve those diagnostics assertions in the Rhai port and make readiness and the intended timeout phase explicit; do not accept missing diagnostics merely because the clock expired. Failed full-run evidence: `/tmp/ravel-k102-test.log` (ephemeral).

## Done when

- Native-entry, admission and acquisition policies run Rhai, with fake protocol fixtures updated to its protocol as necessary.
- Every existing fault discriminator remains effective, including native E08 readers and escaped pipe holder.
- Focused suites, `task check`, `task test`, and both book checkers pass. Re-anchor quoted source if moved.
- Parent k98's full done conditions hold; keep both production runtime paths until k101.


## Goal



## Context

## Done when

## Notes
