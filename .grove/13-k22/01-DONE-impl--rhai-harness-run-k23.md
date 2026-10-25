# rhai-harness-run-k23

## Goal

Make `ravel::run_harness` work end to end. A policy runs a harness that settings name, through the runner, and gets back what happened. The leaf also adds the re-entry refusal and the separation of completion authority.

## Context

- The input is "Harness runs", "Re-entry and completion authority" and the requirements "Direct invocation without re-entry" and "Harness failures are data" in `docs/specs/policy-evaluation.md`.
- The `harnesses` settings key, discovery and the bridge exist after the earlier increments. The harness-run budget is `budgets.harnessRuns`, with a default of 0.
- The program of a run is the program of the entry. The script supplies the arguments. Ravel builds no argument that is specific to a harness.
- A run uses the job machinery of `crates/keyed-launch` and the process-group cleanup that the managed process uses. Read `docs/adr/the-launched-child-is-a-job.md` first.

## Done when

- `ravel::run_harness(request)` accepts `harness`, `args`, `stdin` and `timeoutMs`. A malformed request ends the evaluation as `host_request_invalid`. Without `stdin`, standard input is at end of file.
- A run is noninteractive, in a new POSIX session with no controlling terminal. Its working directory is `request.cwd`.
- A run receives the environment of a managed process, which is the caller's environment with the same scrub, or the confined environment. When the caller passed `--confine`, Ravel confines the run with a private run directory of its own.
- A run receives `RAVEL_EXIT_FILE` for a fresh result channel of its own, and the marker `RAVEL_POLICY_RUN`. The value that the run sends reaches the script as `result`.
- The result map has `status`, `exitCode`, `signal`, `stdout`, `stderr`, `result`, `elapsedMs` and `error`. The statuses are `exited`, `signalled`, `timed_out`, `output_limit` and `unavailable`. Invalid UTF-8 shows as U+FFFD.
- `unavailable` has the codes `unknown_harness`, `harness_not_found`, `harness_not_executable` and `harness_launch_failed`. These runs use no budget.
- Each output stream has a bound of 1 MiB, and Ravel stops the run at the first byte beyond it.
- A run uses one harness run when the runner launches it. A call with none left ends the evaluation as `harness_budget_exhausted`, exit 3. A script cannot catch it.
- `start-session` refuses as `policy_reentry_refused`, exit 3, when `RAVEL_POLICY_RUN` is in its environment. It refuses before it reads settings or a policy. The final managed process never receives the marker, because Ravel scrubs `RAVEL_*` names from it.
- The channel of a run replaces an enclosing `RAVEL_EXIT_FILE`, so a run cannot complete the enclosing session. The worker receives no `RAVEL_EXIT_FILE`.
- The notice lists each run with its harness, status, exit code or signal, elapsed time and whether it sent a result.
- Tests at the CLI seam cover these scenarios: "Missing program", "Output bound", "Deadline of one run", "Caught budget breach", "Re-entry attempt" and "Completion attempt". They also cover a successful run with stdin and a nonzero exit. The other cases are a signalled run, the budget at 0 and a confined run.
- In "Output bound" and "Deadline of one run", `run_harness` returns `output_limit` or `timed_out`. The process group of the run is gone, the script goes on, and Ravel launches the route that the policy selects. Each case has a firing control.
- `task check` and `task test` pass.

## Notes

- A harness run that clears its own environment escapes the marker. The budget and the deadline still bound it. The spec states this, and a test need not prove otherwise.
- Ravel 1.x ignores the marker.
- A session identifier that a harness prints reaches the script only in the output of the run.

## Decisions (running log)

- **The runner gets a captured job, and no second launcher.** `keyed_launch::run_captured` reuses `run_with_output` with a new `Mode::Captured`: a new POSIX session, the descriptor close-on-exec loop, the group kill before the reap, the confined command and the escalation are the ones the managed process uses. The new parts are three pipes, read and written by one `poll(2)` thread (`capture.rs`), and a `StopRequest` that the supervision loop polls. A captured job installs no signal handler and takes no signal of the launcher, so the Ravel CLI keeps its own latch during the evaluation. It polls every 20 ms, not every 500 ms, because a deadline that waits half a second is late.
- **A stop is `End::Escalated`, and `Ended` is unchanged.** `Ended` is built by hand in tests of the `ravel` crate, so a new field or a new `End` variant would touch them. `Captured` carries the output and whether a stream passed its bound. The caller knows whether it asked for the stop, so it labels `timed_out` itself.
- **The run executes on the reader thread, and the runtime thread owns the deadline of the run.** The runtime sleeps for the smaller of `timeoutMs` and the time that remains, and then sets the stop. A stop of the evaluation drops the waiting call, so the run is kept in `Host::flight`, and `serve` calls `Host::settle` before it returns the stop. That asks the run to stop, waits at most 8 seconds for the runner to clear the group, and turns a left-over into `harness_cleanup_failed`. `rhai-stop-order-k24` proves this order and owns the rest of it.
- **The budget counts launches.** The program lookup comes before the budget check, so an unknown harness, a missing program and a launch that fails use no budget, and a call with no run left and an available harness ends the evaluation. The count rises when the runner reports `Started`.
- **The notice lists unavailable runs too.** `harnessRuns` holds every call with its status, so a policy that tried a missing harness shows it. Only launched runs count against the budget.
- **The managed scrub list keeps six names.** `RAVEL_POLICY_RUN` is not in it. `start-session` refuses when the marker is set, so the managed process cannot inherit it, and the book states the removed names as exactly those six. A harness run receives the marker through the grant list.
- **A run without the confinement it needs is `harness_launch_failed`.** `Confined::prepare_for` takes a `Basis` of the selection, so each run prepares a private run directory of its own and removes it. A directory that stays is a cleanup failure.
- **A result channel that cannot be read is no result.** `result_after` fails when the harness replaced the channel with a link or a FIFO. The script then gets `()` and the notice says the run sent none.
- **The leaf's one in-session review is spent, on the runner and the host call.** A fresh reviewer read the diff against the contract. Three findings were real and are fixed. (1) A reply of three 1 MiB strings of control bytes encodes to about 18 MiB, over the 17 MiB frame bound, so `FRONT_FRAME_BYTES` now has a floor of 19 MiB for a run reply, and a test sends the largest reply. (2) The pipe thread looped while a pipe stayed full, so it published an overflow late and `finish` could wait for a writer that left the group. A pass now reads at most 16 chunks. (3) The per-run confinement probe set SIGCHLD to ignored while the worker lived, so `Confined::prepare_for` takes a `probe` flag and a run skips the probe that the managed process already made.
- **Accepted trade-offs from that review, with the reason.** `SETTLE` is 8 seconds, not the 6 of the spec (kill-grace and one second), because the runner needs up to 2 seconds to reap and 1 second to confirm after the KILL. `rhai-stop-order-k24` owns the teardown bound and should settle it. A runner error after `Started` is `harness_cleanup_failed`, because the runner could not confirm the job, even when a pipe thread that failed to start was the cause. A stop that races a result or an overflow labels the run by the first cause the code sees (`output_limit`, then `timed_out`), and the exit status stays in the map for the script. A result that the channel holds but Ravel cannot read is `()`. A cleanup failure leaves no entry in the notice, because the evaluation ends and the failure document carries the report.
