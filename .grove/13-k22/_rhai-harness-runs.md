# rhai-harness-runs-k22 — brief

## Goal

Let a Rhai policy run a harness that the owner named in settings, with `ravel::run_harness`. The runner supervises the run as a job. The policy interprets the output and the exit status, and Ravel judges none of them.

## Done when

- A policy can run a configured harness with arguments and standard input. It receives the status, the output and the result that the run sent.
- Every outcome of a run reaches the script as data. A cleanup failure and a breach of a bound end the evaluation.
- `start-session` refuses inside a harness run, and a run cannot complete the enclosing session.
- A deadline or a cancellation stops a run in progress, reaps its process group, and launches nothing.
- A fake harness plays the model in every scenario of an LLM call, and the credential boundary of a run holds.
- Two opt-in tasks make one real LLM call through Claude Code and one through Codex. The default test run never includes them.
- The TypeScript path works as before, and `task check` and `task test` pass.

## Decomposition

- `rhai-harness-run` makes a run work end to end, with the re-entry and completion rules.
- `rhai-stop-order` proves the ordered stop for a read and a harness run, and the precedence of a signal over the deadline.
- `rhai-llm-call-scenarios` proves at the CLI seam that an LLM call is a harness run, with a fake harness for the model. It also proves the credential boundary of a run.
- `rhai-harness-smoke` adds the opt-in tasks that make a real LLM call through each harness, with the owner's sign-in.

The second leaf tests the stop of a run that the first leaf creates. The third one needs `run_harness` and `ravel::json_check`. The last one comes last, because it spends subscription usage and its recipes feed the example policy and the book.

## Pointers

- ADR: `docs/adr/the-launched-child-is-a-job.md` governs each harness run. `docs/adr/ravel-supervises-the-session.md` governs the session.
- Spec: `docs/specs/policy-evaluation.md`. The sections are "Harness runs", "Harness entries", "Cancellation and deadlines", "Re-entry and completion authority" and "Errors and refusals".
- Spec: `docs/specs/result-job-completion.md` owns the outcome order and the cancellation precedence that the stop must keep. `docs/specs/session-execution.md` describes the managed process.
- Review finding F4 of `rhai-policy-k4` and its integration in `rhai-policy-k5`. Find both with `grove-llm resolve`.
- Spec sections for the LLM leaves: "LLM calls through a harness" and "Credentials". ADR: `docs/adr/a-policy-asks-a-model-through-a-harness-run.md`.
- Glossary: Harness run, LLM call, Harness, Evaluation budget and Host capability.
- Test seam: the real `ravel start-session` against a sandbox home. Fake harness scripts on a private `PATH` serve both the managed process and the harness runs.

## Notes

- The runner is the crate `crates/keyed-launch`. A run reuses its job, process-group and result-channel machinery. Add no second launcher.
- The harness-run budget defaults to 0, so a policy can run nothing until the owner raises `budgets.harnessRuns`.
- The notice of the Rhai path has schema version 2. This increment adds `harnessRuns`.
- `rhai-llm-removal` removed `ravel::llm` and its tests before this node runs. Its running log names the removed tests, which `rhai-llm-call-scenarios` rebuilds through `run_harness`.
- Run `task test:native-linux` once at the end of the increment. The process-group rules differ between macOS and Linux.
