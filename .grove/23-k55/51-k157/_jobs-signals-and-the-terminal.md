# jobs-signals-and-the-terminal-k157 — brief

## Goal

Chapter 9 (`docs/book/src/09-jobs-signals-and-the-terminal.md`), taken through
draft → copy-edit → art → proof under `docs/book/contract.md`. After it the
reader can reason about process groups, escalation, reaping, terminal lending
and recovery, confinement, entry signal state and the failure guarantees with
their limits.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
The chapter teaches the mechanics that the contract says the book teaches where
it first needs them (process groups, the controlling terminal and its
foreground group, saved terminal modes, reaping and why PID reuse makes it
matter, signal dispositions and masks across exec, PTYs as the tests use them,
and the confinement backends). Chapter 3 introduced only what a first session
needs and said that chapter 9 gives the rest. Part A is the job and its end.
Part B is the terminal, the signal state at entry and the guarantees.

- **Part A: the job and its end.** The managed process as a job in its own
  process group (and why a process group, not a new POSIX session, and not the
  PID alone: `docs/adr/the-launched-child-is-a-job.md`); the escalation after
  Ravel receives the result (grace, TERM, kill grace, KILL) with its timing
  figure; group cleanup before the reap, and why the order matters (PID reuse);
  what Ravel confirms (that the group is gone) and what it cannot (a
  descendant that leaves the group); noninteractive isolation; confinement
  (`sandbox-exec` on macOS and bubblewrap on Linux; what is writable, what is
  readable, why the launch starts a new POSIX session, and the refusal when
  the backend is absent). The cancellation sequences for selection and for
  supervision are chapter 5's figures; reuse them and link.
- **Part B: the terminal, the entry state and the guarantees.** Lending the
  terminal to the managed process and recovering it (foreground group, saved
  modes, the failed-exec path and its recovery) with the lending sequence
  figure; the entry signal state (masks and dispositions across exec, what the
  runner changes and does not restore, what the Ravel CLI restores, and the
  reason for it); the failure guarantees stated as a list that a reader can
  quote, each with its limit: same-user tampering, abrupt death of the Ravel
  CLI, descendants that leave their group, detached policy children,
  concurrent embedding, and native x86-64 Linux not qualified.

## Context

- Needs chapters 1–3, 5 and 7, and the scaffold: the figure check, the example
  check, and the timing module with its first figure (`book-figure-timing-k156`).
  Chapter 8 is not needed to draft this chapter, and the two link to each other
  where the code path meets a mechanism.
- **Sources** (verify each claim against the code, not the guide):
  `docs/local-runner.md` (“Boundary”, “Native entry and return”, “Tests and
  authority”), `docs/components.md` (“Cancellation and partial failure”),
  `docs/specs/session-execution.md` (“Managed process and result”),
  `docs/specs/result-job-completion.md`, the three decision records, the runner
  `crates/keyed-launch/src/` (`run.rs`, `confinement.rs`, `native_barrier.rs`,
  `native_hooks.rs`, `channel.rs`), the Ravel CLI's `signal_state.rs`,
  `cancellation.rs`, `confinement.rs` and `authority.rs`, and the runner's tests
  under `crates/keyed-launch/tests/` for the behaviours as the tests observe
  them. The evidence documents (`docs/runtime-evidence.md`,
  `docs/result-job-source-evidence.md`, `docs/completion-failure-design-evidence.md`,
  `docs/native-entry-development-evidence.md`) stay outside the book. The
  chapter states the limits that a reader needs and does not teach the campaigns.
- The guides use words that the book does not use (“front”, “admission”,
  “publication”, “settle”, “consumer”). Write in the book's words.
- **Inventory 4.** Chapter 9 owns O6 (success requires reap and confirmed
  group cleanup), O8 (the terminal's foreground and modes are recovered) and
  the process side of O7 (cancellation suppresses results). Enumerate the ten
  obligations again and record where each is taught.
- **Inventory 3.** Terminal lending and recovery, noninteractive isolation,
  recovery after a failed exec, confinement granted or refused, the escalation
  schedule and the accepted limits are in this chapter's scope. Re-derive that
  list from the specifications at drafting and record the section for each.
- **Figures** (contract figure list): the escalation timing (here, from
  `book-figure-timing-k156`), the terminal lending and recovery sequence (here,
  from the existing sequence module), and the two cancellation sequences that
  chapter 5 already has. Art owns the figures.
- **Examples.** Cancellation that the fake agent or the policy causes is a
  complete example when it can run through `task docs:examples` (the book's
  convention is in `docs/book/examples.md`); only what needs a terminal is a
  fragment. A fragment says what it omits. The example check provides no
  terminal. The chapter does not claim that a terminal run was made.
- **The read 2 task.** “Explain why Ravel has a TypeScript worker and two Rust
  packages” is chapter 7's. This chapter explains why the runner exists apart
  from the Ravel CLI in terms of the mechanics it hides.

## Done when

- Chapter 9 exists, is linked from `SUMMARY.md`, and has passed all four
  stages.
- Every complete example passes `task docs:examples`. Every claim about a
  signal, a group, a time bound or a terminal mode was checked against the code
  and, where a runner test observes it, against that test. The chapter says
  what no test observes.
- The failure guarantees and their limits are stated plainly, as the contract's
  emphasis requires. The chapter does not present an undelivered contract (the
  persistent runner, native x86-64 qualification) as a feature or a plan.
- Each statement of `docs/local-runner.md` and `docs/components.md` that this
  chapter replaces is listed in the log with its destination, so that the
  migration leaf can verify it.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem `jobs-signals-and-the-terminal` and part B's scope, then
  cut copy-edit under the family rule (it lands after part B). Part B's session
  cuts nothing if copy-edit already stands.
- Part A settles the heading structure for the whole chapter now and writes
  down in its log which headings part B owns. Do not emit empty headings.
- Chapter 3 says that “Chapter 9 gives the exact times”, “the terminal rules”
  and “signal masks and failure guarantees”. The chapter must keep that
  promise, and the set proof checks it.
- Chapter 10 is not written. Name it in plain text where a component consumer
  would need a mechanism of this chapter.

## Handed forward

None yet.
