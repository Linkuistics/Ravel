# rhai-stop-order-k24

## Goal

Prove and finish the ordered stop of an evaluation. A deadline, INT, TERM or HUP stops the script, the read and the harness run in the order of the spec. An LLM call is a harness run, so it needs no case of its own. A consumed signal wins over the deadline.

## Context

- The input is "Cancellation and deadlines" in `docs/specs/policy-evaluation.md` and the view `docs/design/rhai-policy/stopping-sequence.mmd`.
- The review finding F4 of `rhai-policy-k4` and its integration in `rhai-policy-k5` define the precedence. The outcome order in `docs/specs/result-job-completion.md` is the rule that Ravel keeps.
- The stop has six steps. The CLI records the first cause. It abandons a read and asks the runner to stop a run. It kills and reaps the worker and removes the private directories. Last, it reads the cancellation latch again.
- Only one of steps 2 and 3 applies, because one host call runs at a time.
- The earlier leaves built parts of this order. Read their running logs, and then close the gaps.

## Done when

- The stop follows the six steps for each kind of host call. Tests cover a stop during a read and during a harness run.
- The runner stops a run in progress. It escalates from TERM to KILL, clears the group and reaps the child. A run whose group survives, or whose reap fails, ends the evaluation as `harness_cleanup_failed`, exit 5.
- Teardown adds at most the kill-grace of the runner and one second. It does not wait for a read in progress.
- A consumed signal wins over an earlier deadline. The refusal is `selection_cancelled` with 128 plus the signal number, and Ravel raises the same signal again. The first consumed signal stays the cause among signals. The report keeps an earlier deadline as the original failure.
- Without a signal, the deadline refuses as `selection_timeout`, exit 124. The final read of the latch follows every other refusal too, so a consumed signal also wins over a breach of a bound.
- Tests at the CLI seam cover the scenarios "Descendants of a harness run" and "A signal after the deadline". The first one sends TERM while a fake harness and its child sleep, and neither process exists afterwards. The second one lets the deadline pass during a harness run that ignores TERM, then sends TERM before the kill-grace ends.
- The stopping view and the spec agree with the code. Fix any difference in the view, or in the code, and record which.
- `task check` and `task test` pass. `task test:native-linux` passes for the descendant cases on a Linux target.

## Notes

- A native call that does not yield stops only when the worker dies. The engine limits leaf already proves that case.
- The opt-in smoke tasks for a real LLM call belong to `rhai-harness-smoke-k66`.

## Decisions (running log)

- **The earlier leaves had built the order, so the gaps were tests and text.** `serve` drops the host call at a stop, `Host::settle` asks the runner to stop and waits for it, `converse` in `worker.rs` stops the worker and removes the private directory, and the final read of the latch (`cancelled_by`) lets a consumed signal win and keeps the replaced refusal in the report. No change to that path was needed. Six new tests at the CLI seam in `rhai_harness_run.rs` prove it: the TERM-ignoring harness that the deadline alone ends after the kill-grace (the firing control of the two window cases), "A signal after the deadline" for INT, TERM and HUP, the first consumed signal staying the cause, a cleanup failure after a run (exit 5), a cleanup failure when the deadline stops the run (exit 5), and a signal winning over a cleanup failure. "Descendants of a harness run" was already proved by `a_signal_during_a_run_stops_the_run_with_its_group_and_cancels_the_session`.
- **A cleanup failure is forced with a directory that nobody can remove.** The harness makes `locked/file` in the private run directory and removes all permissions from `locked`. The group then clears and the reap succeeds, but the removal of the private directory fails, and that is the same `Executed::Cleanup` path that a surviving group takes. A surviving group itself cannot be forced at the seam, because SIGKILL reaches every member of the group, so only the runner's own tests reach `Group::Present`. The reap that fails has the same limit.
- **The teardown bound is the runner's, and the ceiling in `SETTLE` stays at 8 seconds.** An idle machine ends a stop 6.78 s after the spawn with `--timeout-ms 1500` and a harness that ignores TERM: the deadline, the 5 s kill-grace and about 0.3 s of teardown. That is inside "the kill-grace and one second", and the test asserts it with 2 s of slack. The ceiling is not a schedule. A smaller one, such as 6 s, would turn a stop that the runner reports one tick late into `harness_cleanup_failed`, and it would claim a survivor that was not shown. `SETTLE` is now derived from `ESCALATION.kill_grace` plus the runner's two seconds for a failed reap and one second to confirm the group, so it follows the constant. The spec states both the bound and the ceiling.
- **The worker gets TERM and then KILL after one second, not an immediate KILL.** `stop(&mut child, Some(SIGTERM))` in `worker.rs` is shared with the TypeScript path, which must work as before, and the Rhai worker dies at TERM. The spec step 4 and the view said "kills the worker", so they now describe the code. The measured teardown above includes this stop.
- **The view and the spec now agree with the code.** Changes: the worker line of the view, the cleanup-failure note, the exit 5 in the last line, the caption in `diagrams.json`, the entry in the README of the views, and in the spec the worker step, the teardown bound, and the replacement of the deadline by `harness_cleanup_failed` with the signal still winning over it. The STE lint of the spec stays at its baseline of 0 hard findings.
- **No test at the seam proves a stop during a read, and the limit stands.** Every read in `rhai_worker` rejects a FIFO at once and bounds its bytes, so no file in a sandbox makes a read last. The mechanism test `a_deadline_does_not_wait_for_the_call_the_reader_is_in` stays, and a twin for the output bound joins it. Both fail when `Watch::until` waits for the work, which a mutation showed. A signal takes the same arm, but its latch is shared by every test of the binary, so only the seam tests cover that cause.
- **`task test:native-linux` (arm64, native in the Docker VM) passes for this leaf and fails a lint step that another leaf owns.** The `rhai-worker` step ran 240 of 240, including the six new tests and `a_signal_during_a_run_stops_the_run_with_its_group_and_cancels_the_session`. The `rust-clippy` step exited 101 on three dead `#[cfg(test)]` helpers in `heap.rs` (`live`, `reset_peak`, `peak`). I did not touch that file, and macOS shows the same three as warnings. The fix is `rhai-linux-clippy-dead-code-k68`, inserted before `rhai-llm-call-scenarios-k67` so the final Linux run of the increment is green. Only arm64 ran. The amd64 emulation cannot establish process-group semantics, as `scripts/native-linux.sh` says.
