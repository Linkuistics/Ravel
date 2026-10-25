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
