# rhai-upgrade-task-k65

## Goal

Make a Rhai upgrade cheap to prove. A Taskfile task runs the engine tests and the hostile classes at the command-line seam, in a fixed order. This is the Rhai half of the pruned `rhai-llm-upgrade-proof-k21`.

## Context

- The input is "Upgrades" in `docs/specs/policy-evaluation.md`. A Rhai upgrade changes one pin. The task proves it, and each hostile class keeps its firing control.
- The pin is `rhai = "=1.27.0"` in `crates/ravel/Cargo.toml`. The engine and hostile tests live in `crates/ravel/tests/rhai_*.rs`.
- `tests/hostile.rs` and the task `test:hostile` belong to the TypeScript worker. They go with `typescript-removal-k29`. This task must not depend on them.
- The running logs of `rhai-engine-limits-k10` and `rhai-try-boundary-limits-k46` list the hostile classes and their controls.
- The book chapter about developing Ravel is not changed here. Record the task name in the running log for the documentation increment.

## Done when

- A Taskfile task runs the Rhai engine tests, then the hostile classes. It uses only the `rhai_*` test targets and the real `ravel start-session`.
- The task fails when a named test target is missing or runs no test. It prints the pinned Rhai version first.
- The running log lists each hostile class and shows where its firing control sits.
- A deliberate break in a scratch copy of a hostile test makes the task fail. The break is not committed.
- The task passes on the current pin.
- `task check` and `task test` pass.

## Notes

- The task changes no pin. An owner who upgrades Rhai edits the pin, then runs the task.
- Name the task so that it cannot collide with `test:hostile`, `smoke` or `release:smoke`.
- `typescript-removal-k29` reconciles the Taskfile. It must find this task independent of the worker.
