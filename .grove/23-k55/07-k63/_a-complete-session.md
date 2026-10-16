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

## Handed forward

- **Art — chapter 3, after “Admit the request and evaluate the policy” or
  beside the lifecycle explanation:** add the contract's successful-session
  UML sequence, with notation teaching, caption and text alternative. Reuse
  the opening unit's visual vocabulary. Distinguish request admission,
  worker identity/evaluation, front judgement, confirmed worker reap and
  private-directory removal, command resolution, result-channel allocation,
  launch, publication, group stopping before direct-child reap, terminal
  recovery/group absence, result read, temporary cleanup and final stdout.
  The runner is inside the native front, not a service. Publication does not
  return a result to the caller. `choice.rs::evaluate_and_resolve`,
  `worker.rs::evaluate`, `run.rs::run` and the runner's `watch` /
  `supervise_reported` ground the order. Keep this orientation-sized; chapter
  8 owns the full code trace and chapter 9 the timing/terminal figures.
- **Art — chapter 3, “Recognise the other outcomes”:** add the contract's UML
  outcome state machine, with notation teaching, caption and text alternative.
  Distinguish refusal before launch, cancellation in either phase, natural
  exit without publication, and failure from success after cleanup and final
  write. Empty publication is not absence; nonzero managed exit is not alone
  failure; cancellation suppresses a published value. Reuse in chapter 5.

## Draft evidence carried to later stages

- The example convention is `docs/book/examples.md`. Stdout and exit status
  are exact; stderr uses the supported `Standard error includes` label.
  Scratch runs on macOS produced additional PID-dependent EPERM stop reports.
  `supervise_reported` explicitly retains such reports after confirmed
  reap/recovery/group absence. The chapter explains this limitation instead
  of hiding reports or inventing an exact cross-platform stderr transcript.
- The install fragment uses a private prefix built from this checkout. The
  worker and front keep their paired layout. Do not replace it with a v1.0.0
  release-install instruction while the book describes this unreleased tree.
- Forward chapter mentions are plain text while those pages do not exist;
  the parent brief assigns their link conversion to the first-half set proof
  and second-half work. The example check supplies no terminal interaction,
  human comprehension result, GitHub rendering or published-site inspection.
