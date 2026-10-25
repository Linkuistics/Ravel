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
