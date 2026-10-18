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

## Decisions (running log)

**Module shape (2026-10-18).** `_activity.typ` places every node itself. A figure
source is a structured flow (`action`, `outcome`, `decision`, `fork`,
`partition`, `initial`, `final`), so it holds no coordinate. This follows
`_deployment.typ` and not `_c4.typ`. A decision has two flows: `down` is the spine
and `right` is a branch beside the diamond. A flow that rejoins uses
`merge: true`. I rejected a general graph layout (edges between named nodes):
it needs edge routing and crossing avoidance that Typst cannot do cheaply, and
the two figures that the contract lists are both structured flows. Seen: the
release flow of `docs/RELEASING.md` drawn as an uncommitted scratch figure
(three partitions, a decision with an exit, two forks and joins of three
columns, a note, a labelled flow, a final node), and a merge fixture, with no
change to the module.

**Notation departures (2026-10-18).** An `outcome` is a grey labelled box and not
the bare final ring (the reader needs the exit status; the state views do the
same); `final` is the usual ring. A decision's question is left of a small
diamond. A `partition` is a titled outline, not a lane across the figure. A
guard of `none` draws no guard, so a deleted guard fails the compare and not
the render. No Typst or CeTZ limit made a decision node or a partition
impractical, so no owner question was needed. `_style.typ` gained two roles
(`control`, `bar`); `_draw.typ` gained a square corner for `bar` and header
text. The nine existing SVGs and records are byte-for-byte unchanged
(`git diff --name-only` over them lists none).

**The figure's decisions, checked against the code (2026-10-18).** Source:
`crates/ravel/src/run.rs` (`run`, `settle`) and
`crates/ravel/src/cancellation.rs` (`Handlers::finalise`). The guide may be
stale, and it is, in one place.
- Cleanup is attempted on every path, before the choice: `run` computes
  `mask`, `channel_cleanup`, `private_cleanup` and `restore` after the closure
  returns, whatever it returned (run.rs lines 256-273).
- Signal order: `finalise` takes `front` (latch read twice, around the entry mask
  restore), then `outcome` (the runner's consumed signal, `runner_signal`), then
  `late` (`keyed_launch::take_interrupt`), then `received()` again, then
  `pending` (the final snapshot of a blocked pending set), as
  `front.or(outcome).or(late).or(received()).or(pending)` (cancellation.rs line
  333). **`docs/components.md` ("Error and signal precedence") names only the
  first three; it omits the final pending snapshot.** The figure shows all four
  as "queued signal at the last check". I did not edit the guide, which the
  contract folds into the book; chapter 8's draft and the guide fold-in should
  carry the fourth source. Test: `runner_error_conversion_joins_cancellation_before_no_ending_settlement`
  (run.rs) for front, outcome and late; `a_handled_signal_pending_behind_the_block_cancels`
  (cancellation.rs) for the pending source.
- Every cleanup error and a surviving group are reported on every path, before
  the choice: `settle`'s two loops over `cleanups` and `group`, ahead of the first
  return. Test: `cancellation_overrides_prelaunch_failure_and_reports_every_cleanup_error`
  and `cancellation_reports_a_surviving_group_and_suppresses_the_result`.
- Signal found gives `session_cancelled`, exit 128 plus the signal number (HUP 129,
  INT 130, TERM 143: `HANDLED` in cancellation.rs, `Signal::exit`), no result, and
  the original failure's text is written first: `settle`'s `if signal.is_some()`
  block. Tests as above.
- No signal and an original failure: `result?` returns it with its own exit
  status. Test: `runner_error_conversion_preserves_start_classifications_and_success_settlement`
  (exit 127, 126, 5).
- Then the first cleanup error: `cleanups.into_iter().find_map(Result::err)`,
  in order mask, channel, private directory, output, then signal reports. Each is
  `failed(...)`, exit `EXIT_WORKER` (5). **No unit test exercises this
  noncancelled path**; it is source-supported only, as `docs/components.md` also
  says.
- Then a surviving group: `process_group_survived`, exit 5. **Source-supported
  only**; the tests cover the group under cancellation, not without it.
- Then the write: `result_write_failed`, exit 5, only after `settle` returns Ok;
  success is exit 0 with the exact bytes. **Source-supported only**; the success
  case is the last loop of the "success settlement" test above.

**Controls (2026-10-18).** `scripts/figures.test.sh` has an activity fixture
(every node kind) that passes unchanged, and mutated copies that fail: an added
node, a changed label and a missing guard each fail both the record compare and
the SVG compare; a right branch that continues with no merge, a node after an
outcome and a duplicate id each fail for their own named reason. Seen failing:
with both compares skipped in a mutated copy of `figures.sh`, the node, label
and guard mutations pass the check (so those controls fail); with each module
assertion removed in a copy, the matching structural mutation no longer shows its
reason (a different reason appears, so those controls fail). An emptied guard
`[]` was a render error in `_layout.typ`, not a compare failure; that led to the
`none` guard above.

**Viewing and text (2026-10-18).** The figure was viewed at the 750-pixel column
in the Playwright browser from a scratch page (the leaf edits no chapter), and
the record is in `docs/book/figures.md`. `task docs:ste -- --strict` first
refused "front", "pending", "surviving" and "settlement" in the figure's text;
they became "Ravel CLI", "queued signal", "survives" and a plain sentence.
Beyond the two additions the leaf names, `figures.md` also changed where the new
module made a sentence false: the module list, "Seven modules", and one row in
"What each view means".
