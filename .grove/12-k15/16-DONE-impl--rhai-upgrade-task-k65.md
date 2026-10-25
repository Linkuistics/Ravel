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

## Decisions (running log)

- **The task is `task rhai:upgrade-proof`.** It runs `scripts/rhai-upgrade-proof.sh`. The name cannot collide with `test:hostile`, `smoke` or `release:smoke`, and the script reads nothing of the TypeScript worker: `tests/hostile.rs`, the `hostile` test target and `test:hostile` are never named. `typescript-removal-k29` can delete those and leave this task as it is.
- **Order.** The script prints the pin first (`pinned Rhai 1.27.0`). It reads the exact `=` pin in `crates/ravel/Cargo.toml` and refuses when `Cargo.lock` builds another version. Then it runs the six engine targets (`rhai_policy`, `rhai_settings`, `rhai_helpers`, `rhai_json_check`, `rhai_reads`, `rhai_role`), then the 15 hostile classes of `rhai_limits`, one `cargo nextest` run per class in a fixed order, so that a failure names its class.
- **What makes the task fail.**
  - The pin is not exact, or `Cargo.lock` disagrees with it.
  - A `rhai_*.rs` file is not in the script, or a named target is not in `crates/ravel/tests`.
  - A target lists no test, runs no test (`--no-tests fail`), or its summary shows a run count that differs from its list.
  - `rhai_limits` lists a test that the script does not name, or lacks a named class.
  - Any `rhai_*` file holds `#[ignore]`, because nextest leaves an ignored test out of its list and the case would pass unrun.
- **The seam.** Every case starts the real `ravel start-session` in a sandbox home, with a recording harness. The unit tests of `rhai_worker::engine` stay in `task test`, because they are not `rhai_*` test targets. They hold the limits that need no process: expression depth, the number of functions and the map size. An owner who wants them runs `task test`.
- **The hostile classes and where each firing control sits** (`crates/ravel/tests/rhai_limits.rs`). Every test has the hostile case, which must refuse with the named bound, and a control of the same shape with a harmless amount, which must select and start the harness. Both `limit` and `refused` assert that no harness started after a refusal.

  | Class | Test | Control | Hostile case |
  |---|---|---|---|
  | endless loop | `an_endless_loop_ends_at_the_operations_bound` | `survives` at line 74 (1,000 iterations) | `limit` at line 82, `operations` |
  | deep recursion | `unbounded_recursion_ends_at_the_call_depth` | `survives` at line 95 (`down(40)`) | `limit` at line 103, `call depth` |
  | large string | `a_string_that_doubles_ends_at_the_string_size` | `survives` at line 111 (10 doublings) | `limit` at line 126, `string size` |
  | array that grows | `an_array_that_doubles_ends_at_the_array_size` | `survives` at line 134 | `limit` at line 149, `array size` |
  | `try` around a breach | `try_does_not_catch_a_breach_and_the_host_refuses_one_that_a_closure_call_wrapped` | `survives` at line 159 (`try` catches `throw`) | `limit` at line 188 for three bodies |
  | breach in a function-pointer call under `try` | `a_breach_inside_a_function_pointer_call_ends_the_evaluation_although_try_surrounds_it` | `survives` at line 238 | `limit` at line 242 for call depth, string size, array size and variables |
  | breach in a function named by `drain` or `retain` | `a_breach_inside_a_function_named_by_drain_or_retain_ends_the_evaluation_under_try` | `survives` at line 264 | `limit` at line 268, `call depth` |
  | breach in a comparer | `a_breach_inside_a_comparer_ends_the_evaluation_although_no_try_is_involved` | `survives` at line 290 | `limit` at line 294, `call depth` |
  | heap | `a_policy_that_fills_the_heap_ends_at_the_heap_bound_and_the_cli_reports_it` | `survives` at line 316 (5 copies) | `refused` at line 320, `memory_limit` |
  | `sleep` and the `pad` loop | `a_native_call_that_does_not_yield_stops_at_the_deadline` | `sleep(0)` and `pad` with `"-"` at line 344, which must return at once and start the harness (line 365) | `refused` at line 377, exit 124 `selection_timeout` inside the deadline |
  | output flood | `a_policy_that_prints_without_end_ends_as_output_limit` | ten `print` calls start the harness (line 399) | `refused` at line 406, `output_limit` |
  | result over 1 MiB | `a_result_over_the_message_bound_ends_as_message_too_large` | `survives` at line 421 (13 doublings) | `refused` at line 425, `message_too_large` |
  | signal during evaluation | `a_signal_during_the_evaluation_kills_and_reaps_the_worker_and_launches_nothing` | none of the harmless kind: the class has no harmless amount of a signal | line 467 (`gone(worker)`) and line 470 (no harness started) fire for INT, TERM and HUP |
  | worker killed from outside | `a_worker_killed_from_outside_refuses_as_worker_failed` | none of the harmless kind | line 504 (`gone(worker)`) and the `worker_failed` assertions fire |
  | closure after the script ends (rhaiscript/rhai#1193) | `nothing_a_script_returns_can_be_called_after_it_ends` | none of the harmless kind | `refused` at line 521, `selection_malformed` at `result.args[0]` |

  The last three classes have no harmless control, so their firing control is the assertion that fails when the guard is missing: the worker survives a signal, the worker survives a kill, or the closure reaches a host call. The line numbers belong to the file as of this leaf. The task does not read them. The names do the pinning.
- **Deliberate breaks, each in the working tree and each restored before the commit** (the file hash matched its backup after each one). Every break made the task fail with the named message:
  - The hostile step of the operations class changed from `loop { }` to `let i = 0;`. The task stopped at `hostile class an_endless_loop_ends_at_the_operations_bound failed`, after six engine targets passed.
  - A hostile test renamed (`unbounded_recursion_ends_elsewhere`), and in a second run given `#[ignore]` on the same test. The task stopped at `a named hostile class is missing from rhai_limits…`.
  - `#[ignore]` on `the_entry_must_be_utf_8_and_at_most_one_mebibyte` in `rhai_policy.rs`. The task stopped at `a rhai_* test is ignored`. The first try applied nothing, because BSD `sed` has no `0,/re/` address, and it passed. I redid it with Python and the task failed.
  - `rhai_role.rs` moved away. The task stopped at `the declared targets and the files … differ`.
- **Gates.** `task rhai:upgrade-proof` passes on 1.27.0 (about 38 s). `task check` passes, including `shellcheck` on the new script, and `task test` runs 546 tests, all passed, 48 skipped.
- **For the documentation increment (`rhai-book-inside`).** The task name is `task rhai:upgrade-proof`. The chapter about developing Ravel is not changed here. An owner who upgrades Rhai edits `rhai = { version = "=X.Y.Z", … }` in `crates/ravel/Cargo.toml`, runs `cargo update -p rhai --precise X.Y.Z`, then runs the task. The task refuses when the lock file disagrees with the pin.
- **For `typescript-removal-k29`.** The Taskfile entry and the script are independent of the Bun worker. The only shared names are `rhai_*` test files and `Cargo.toml`.
- **Not run.** `task test:native-linux`. The task adds no Rust code and no platform behaviour.
