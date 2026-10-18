# book-figure-timing-k156

## Goal

Add the UML timing notation to the book's figure engine, and deliver the first
timing figure with it: the timeline after `end-session` sends the result. The
contract's figure list asks, for chapter 9: “What is the timing after
`end-session` sends the result (2-second grace, TERM, up to 5 seconds, KILL)?”
The engine has no timing module, and the chapter 9 art stage must not build a
layout module inside a figure stage.

## Context

- Contract: `docs/book/contract.md` ("Format and media" and the figure list).
  Mechanics: `docs/book/figures.md` ("Layout, then draw", "The layout modules",
  "Visual vocabulary"). The seam is: a layout module measures its text and
  returns a layout record with no colour, weight, size or roughness; `_draw.typ`
  draws any record under a named style; the figure's `.layout.json` is
  committed and compared. The crisp style (roughness 0) is the only style.
  `book-figure-activity-k153` added the previous module. Read what it did and
  follow it, including its entry in `figures.md`.
- **The reader question.** After the result appears in the channel (or the
  managed process exits on its own), what does Ravel do, and when? The
  schedule is in `crates/keyed-launch/src/run.rs` (the escalation: `grace` runs
  from the channel's appearance to SIGTERM, `kill_grace` runs from SIGTERM to
  SIGKILL; the defaults are the 2 and 5 seconds the specification states), in
  `docs/specs/session-execution.md` (“Managed process and result”) and in
  `docs/adr/the-launched-child-is-a-job.md` (descendants receive time during
  their parent's shutdown, and surviving group members are terminated before
  the direct child is reaped). The file also holds other bounded waits (for
  example the group confirmation and the error settlement). Read the code and
  decide which times belong on the figure: the figure answers the question
  above and does not become a list of every constant.
- **Notation to cover**: the lifelines the question needs (for example the
  result channel, the managed process, the rest of its group, and the runner),
  states on each lifeline, state changes at points on a shared time axis, a
  time constraint (a bracket with a bound such as “up to 5 seconds”), and the
  condition that ends a wait early (the process exits inside its grace, and is
  never touched). Verify each state and each bound against the code and the
  runner tests. Decide whether the time axis is drawn to scale. If it is not,
  the figure says so on the figure.
- Chapter 9 also shows the lending and recovery of the terminal as a sequence.
  That figure uses the existing sequence module and is not part of this leaf.
- The book teaches no notation. The figure's own conventions are on or beside
  the figure.
- Text rules apply to every label, key and note: ASD-STE100 writing rules,
  `docs/book/ste.md`, “Ravel CLI”, “send”. Check the figure's
  `.layout.json` with `task docs:ste -- --strict`.

## Done when

- A layout module (`_timing.typ`, or the name the existing modules suggest)
  measures its text and returns a layout record. `_draw.typ` draws it under the
  existing style, with palette roles, strokes and sizes from `_style.typ`. The
  new figure uses the same colours, fonts, weights and arrow style as the other
  figures.
- A figure source (name it for its subject, for example
  `escalation-timing.typ`), its SVG and its `.layout.json` are committed.
  `task docs:figures` passes for every figure, and `task docs:figures:test` has
  a control for the new module (a changed source, a changed label and a changed
  bound each fail the compare). Watch each new control fail before you credit
  it.
- The figure is accurate: each state, bound and ordering was checked against
  the code and a runner test, and the log names the function or test that
  supports each one. The log says which time constants are on the figure and why
  the others are not.
- `docs/book/figures.md` describes the module and records a dated viewing of
  the figure at the 750-pixel column in a browser, with the limits of that view.
- No chapter is edited. The chapter 9 art stage places the figure and may refine
  its labels through the shared module.

## Notes

- If a Typst or CeTZ limit makes the notation impractical, record the limit and
  ask the owner through the log before you change the notation.
- Typst 0.15.1 and CeTZ 0.4.2 are pinned by the contract.
