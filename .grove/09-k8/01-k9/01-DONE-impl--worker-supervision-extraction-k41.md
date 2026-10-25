# worker-supervision-extraction-k41

## Goal

Separate the generic supervision of a policy worker from the Bun protocol in `crates/ravel/src/worker.rs`. The Rhai worker can then use the same private directory, socket, descriptor hygiene, diagnostics bound, bounded stop and teardown report. Behaviour of the TypeScript path does not change.

## Context

- The parent brief `rhai-policy-worker-k9` explains the split. The generic part is in `evaluate`, from the private directory to the teardown report. The Bun part is `locate`, `verify_hello`, `converse`, `Loaded` and the frame readers.
- The existing tests in `worker.rs`, `worker_admission_tests.rs`, `partial_acquisition_tests.rs`, `native_entry_tests.rs` and `tests/hostile.rs` hold the behaviour. They hook into `evaluate` through `#[cfg(test)]` calls. Keep every hook where it is observable.

## Done when

- A function in `worker.rs` supervises one child from a launch description (program, `arg0`, environment) and a conversation closure over the bounded channel. It owns everything that `evaluate` owns today except the Bun protocol.
- `evaluate` is a thin caller of that function. It keeps its signature.
- No test changes meaning. `task check` and `task test` pass.

## Notes

- This is a refactor. Do not change a message, a code, an exit status or an ordering.

## Decisions (running log)

- `supervise` stays in `worker.rs` and takes the launch, the entry name, the limits and a conversation closure. A new module would move the `#[cfg(test)]` hooks away from the code they observe. `Launch`, `Supervised`, `Conversation` and `Bounded` are `pub(crate)`.
- `Launch` carries the remedy (`rebuild`), because the Bun remedy ("run `task worker`") is wrong for the Rhai role.
- Verified with `task check` and `task test` after the change: 339 tests passed and 46 were skipped, as the suite configures. No test file changed.
