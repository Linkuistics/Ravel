# book-figure-activity-k153

## Goal

Add the UML activity notation to the book's figure engine, and deliver the first
activity figure with it: how the outcome of a session is chosen when failures and
signals combine. The contract's figure list needs an activity figure in chapter 8
(this one) and another in chapter 12 (how a release is built and checked). The
engine has layout modules for sequence, C4, state and deployment diagrams but
none for activity, and the chapter units' art stages must not build a layout
module inside a figure stage.

## Context

- Contract: `docs/book/contract.md` ("Format and media" and the figure list).
  Mechanics: `docs/book/figures.md` ("Layout, then draw", "The layout modules",
  "Visual vocabulary", "Porting record") and the five existing modules in
  `docs/book/src/figures/` (`_layout.typ`, `_style.typ`, `_draw.typ`, `_c4.typ`,
  `_sequence.typ`, `_state.typ`, `_deployment.typ`). The seam is: a layout module
  measures its text and returns a layout record with no colour, weight, size or
  roughness; `_draw.typ` draws any record under a named style; the figure's
  `.layout.json` is committed and compared. A new module follows that seam, and
  the crisp style (roughness 0) is the only style.
- **The reader question** (contract): “How is the outcome chosen when failures
  and signals combine?” The figure is the decision flow that
  `docs/components.md` (“Error and signal precedence”) states in words and that
  `crates/ravel/src/run.rs` (`run`, `settle`) and
  `crates/ravel/src/cancellation.rs` implement: the signal sources are taken in
  an order; cancellation wins over a result failure and a cleanup failure;
  without cancellation an existing result failure stays primary; then the first
  cleanup error; then a surviving group prevents success; only a successful
  settlement reaches the final result write, whose failure is still an error.
  Verify every decision and every order against the code and against
  `docs/specs/result-job-completion.md` (“Required failure settlement”) before
  you draw it. The guide may be stale.
- **Notation to cover**: the elements that figure needs (initial and final
  nodes, actions, decision and merge nodes with guards, flow arrows with
  labels, notes), and the elements that the release figure in chapter 12 is
  likely to need (a fork and join, a swimlane or partition). Read
  `docs/RELEASING.md` (“Version cuts and publication”) and `release.toml` to see
  the release flow's shape, and draw it as a scratch figure that you do not
  commit, to show the module can express it. The chapter 12 art stage draws and
  commits that figure.
- The book teaches no notation. The figure's own conventions (what a guard
  label, a colour role or a dashed flow means) are on or beside the figure.
- Text rules apply to every label, key and note: ASD-STE100 writing rules,
  `docs/book/ste.md`, “Ravel CLI”, “send”. Check the figure's
  `.layout.json` with `task docs:ste -- --strict`.

## Done when

- A layout module (`_activity.typ`, or the name the existing modules suggest)
  measures its text and returns a layout record. Adding a node or lengthening a
  label edits no coordinates in a figure source when the module places nodes
  itself; if placement is manual as in `_c4.typ`, say so in `figures.md` and
  keep the source's coordinates few.
- `_draw.typ` draws the new record under the existing style. A palette role,
  stroke weight or text size that the figure needs is added to `_style.typ` as
  a shared definition, not set in the figure. The new figure uses the same
  colours, fonts, weights and arrow style as the nine existing figures.
- A figure source (name it for its subject, for example
  `outcome-precedence.typ`), its SVG and its `.layout.json` are committed.
  `task docs:figures` passes: the nine existing figures and the new one match
  their sources, and `task docs:figures:test` has a control for the new module
  (a changed source, a changed label and a missing guard each fail the compare).
  Watch each new control fail before you credit it.
- The figure is accurate: each decision, guard and order was checked against
  the code, and the log names the function or test that supports each one.
- `docs/book/figures.md` describes the module (what it covers, the placement
  rule, departures from the notation's usual form) and records a dated viewing
  of the figure at the 750-pixel column in a browser, with the limits of that
  view. Nothing else in the file changes.
- No chapter is edited. The figure has no page reference until the chapter 8
  art stage places it, and the figure check does not need one.

## Notes

- The new figure's labels may change when chapter 8's draft is proved. That is
  the art stage's change to make through the shared module, not a reason to
  hold this leaf.
- If a Typst or CeTZ limit makes a decision node or a swimlane impractical,
  record the limit and ask the owner through the log before you change the
  notation. Do not substitute a different notation without saying so.
- Typst 0.15.1 and CeTZ 0.4.2 are pinned by the contract. A change of version
  is a change to the contract.
