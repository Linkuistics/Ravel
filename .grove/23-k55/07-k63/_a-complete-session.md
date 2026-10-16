# a-complete-session-k63 — brief

## Goal

Chapter 3 (`docs/book/src/03-a-complete-session.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
run a real session with a tiny policy and a fake agent, then describe its
stages; the process and terminal ideas are introduced as the session needs
them.

## Context

- Needs chapters 1–2 (terms, the system-context figure, the authority
  statement) and the scaffold: the examples check for its complete example,
  the figures check for its figures.
- This chapter carries the book's first complete example, run through
  `task docs:examples`: a tiny policy, a fake agent that calls
  `ravel end-session`, the command, and the exact stdout, stderr and exit
  status. `crates/ravel/tests/cli.rs` (`routed_result_is_exact_*`) shows the
  seam.
- The stages to teach, each verified against the code: admission of the
  request and policy path, evaluation in the policy worker, judgement of the
  selection, the worker reaped, launch as a job, `end-session` publishing,
  the escalation after publication, reaping and cleanup, the result on stdout.
  Sources: `docs/specs/session-execution.md` ("Policy authority and runtime
  discovery", "Managed process and result"), the `ravel` run composition under
  `crates/ravel/src`, `docs/adr/the-launched-child-is-a-job.md`.
- Introduce exactly what the session needs: a process group, the managed
  process as its own group, the terminal's foreground group and its lending,
  reaping. Defer escalation detail, signal masks and failure guarantees to
  chapter 9, and say so in text.
- How the reader obtains `ravel`: state what is true now for the code this
  book is drafted against. The workstream is past the v1.0.0 release and not
  yet released, so check `docs/RELEASING.md`, `task install` and the formula,
  and say plainly which build the examples assume. Do not teach installing an
  agent; the example uses no agent.
- Figures from the contract: a UML sequence for the successful session (it
  returns in full in chapter 8) and a UML state machine for the outcomes (it
  returns in chapter 5).

## Done when

- Chapter 3 exists, is linked from `SUMMARY.md`, and has passed draft,
  copy-edit, art and proof.
- Its complete example runs under the checker and matches exactly; the
  reader's first session works from the chapter alone.
- Refusal, cancellation and natural exit without a result are named as normal,
  distinct outcomes and pointed at chapter 5, not taught here.
- The chapter says, in the form a reader can quote, what success means: the
  result reaches the caller only after the managed process is reaped, its
  whole group is gone and cleanup succeeded.

## Notes

- "Session" here is Ravel's, never a POSIX session; say so where the two could
  be confused, as the contract's term risks direct.
