# a-complete-session-k83


## Goal

Add chapter 3's two contract figures to
`docs/book/src/03-a-complete-session.md` under `docs/book/contract.md` and
`docs/book/figures.md`: the successful-session UML sequence and the outcome
UML state machine, each with notation teaching, caption and text alternative,
in the opening unit's visual vocabulary.

## Context

The document brief's `## Handed forward` carries both figure obligations,
their placement and what each must distinguish; it is the specification for
this stage. Copy-edit (`a-complete-session-k82`) changed no structure and no
example, but fixed vocabulary the figures should label with the same words:

- the job: the managed process and the members of its group, defined in its
  job-control sense at the process-group paragraph;
- “the managed process's group”, never “the managed group”;
- lending the terminal: making the managed process's group the terminal's
  foreground process group, done only when Ravel's own group is the
  foreground group, then taken back and the saved modes restored;
- the worker starts in its empty private directory and changes to `/` before
  importing policy; the front removes that directory after confirmed reap,
  before resolving the selected executable;
- settlement removes the result channel's temporary directory; reap comes
  after the final group signals because the group ID is the managed process's
  PID, which is not reused before reap;
- cancellation is by INT, TERM or HUP, in selection or supervision.

The opening unit's figures (`system-context`, `process-authority`) fix the
colour and label conventions; reuse them. Chapter 8 owns the full code trace
and chapter 9 the timing and terminal figures, so keep the sequence
orientation-sized.

## Done when

- Both figures exist as editable sources beside committed SVGs, are placed
  and explained in the chapter, and `task docs:figures`, `task
  docs:book:build` and `task docs:examples` pass.
- The node brief's two art entries under `## Handed forward` are cleared.
- Cut the `proof` successor with the document's bare stem, unless a live
  later sibling already holds that stage, then retire and commit this leaf.

## Notes
