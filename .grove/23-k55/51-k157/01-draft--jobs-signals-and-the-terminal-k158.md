# jobs-signals-and-the-terminal-k158

## Goal

Draft **part A** of chapter 9, as the document node's brief describes, under
`docs/book/contract.md`, into
`docs/book/src/09-jobs-signals-and-the-terminal.md`, linked from
`docs/book/src/SUMMARY.md`: the managed process as a job, the escalation after
Ravel receives the result, group cleanup before the reap, noninteractive
isolation and confinement. The headings for part B are not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 3 introduced the
  process group and reaping only as far as a first session needs, and it points
  here for the rest; keep its words for the same concepts and link back.
- Run the escalation yourself on a scratch copy before you write the times: a
  fake agent that sends a result and then ignores TERM shows the grace and the
  kill grace on a real run. The runner's tests under
  `crates/keyed-launch/tests/` (`job.rs`, `interrupt.rs`, `confinement.rs`,
  `noninteractive.rs`) show what the project already observes. State in the
  chapter only what the code or a test supports.
- The macOS stop-report behaviour that chapter 3 describes (accepted `EPERM`
  stop reports, retained after confirmed reap) belongs here too. Keep the
  explanation consistent with chapter 3 and with `supervise_reported` in
  `crates/keyed-launch/src/run.rs`.
- Settle the heading structure for the whole of chapter 9 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- Confinement on Linux needs bubblewrap and on macOS `sandbox-exec`. Say what
  the checkout's tests can and cannot show on the host you use, and what no
  test shows.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict` and `task docs:examples`. The
  figure needs (the participants, states and bounds for the timing figure) are
  in the log for the art stage. Do not draw them here.
- The log lists the obligations that part A teaches (O6 and the process side of
  O7) and what remains for part B (O8).
- Last act: `leaf-add` a `draft` leaf (bare stem
  `jobs-signals-and-the-terminal`) whose body states part B's scope from the
  node brief, the headings reserved for it and anything part A learned that
  changes it; then `leaf-add` the `copy-edit` leaf if no live later sibling
  already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claim about the order of group cleanup and reap, because a wrong order is the
  kind of mistake the compiler cannot find.
