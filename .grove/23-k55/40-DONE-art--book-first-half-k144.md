# book-first-half-k144

## Goal

Apply the owner's figure decision R10 (crisp, not sketched) to every figure in
chapters 1–6. Draw the figures that the drafts of this run request for R11.

## Context

- The contract's “Figure style” (2026-10-18) and R10 in `book-read-one-k70`'s
  log. The owner compared all nine figures in both styles before the choice.
- `_draw.typ` already draws the crisp style with `--input style=crisp`
  (roughness 0, exact geometry) from the unchanged layout records. Make it
  the shipped style, and regenerate the SVGs with `task docs:figures:update`.
  Show that no `.layout.json` record changes.
- `docs/book/figures.md` records the sketched style, its roughness and dated
  viewing results. Make it describe the crisp style. Decide whether the
  sketch code in `_draw.typ` stays unused or goes. The contract says that
  the sketch has no use. If the code goes, remove its controls from
  `task docs:figures:test`, and keep each other control meaningful.
- New figures: the brief of each unit node lists its figure needs under
  `## Handed forward`, with reader questions. Draw each one through the
  layout modules and `_draw.typ`, with a key, alt text and STE text. Clear
  each entry that you close.
- The small text of the dense figures stays with whole-book art
  `ravel-book-k101`, unless a new figure needs a fix now.
- This is the third art run on the set. The owner authorised it on
  2026-10-18 in `book-read-one-k70`.

## Done when

- Every figure in chapters 1–6 uses the crisp style. `task docs:figures` and
  `task docs:figures:test` pass. The nine earlier layout records do not
  change.
- Each requested figure exists, with alt text, and its page passes the prose
  check.
- `task docs:ste -- --strict` passes on the figure records and on the changed
  chapters.
- In the book that `task docs:book` serves, every figure loads at the
  750-pixel column. A control image with no file fails.

## Notes

Validation on 2026-10-18:

- `task docs:figures` and `task docs:figures:test` pass. The exact-outline
  control first failed on the old sketch renderer, then passed on the crisp
  renderer. All nine SVGs equal the former renderer's explicit crisp output.
  All nine layout records retain their pre-edit SHA-256 digests.
- `task docs:ste -- --strict`, `task docs:breaks -- --strict`,
  `task docs:book:build` and ShellCheck on both changed scripts pass.
  The focused strict STE run also passes on all six chapters and nine
  layout records. The 242 recorded validation subjects remain unchanged
  after the checks: source, scripts, configuration and pinned tools/assets.
- Chrome loaded all nine images at the served book's 750-pixel column,
  with nonempty alt text. The separate absent-image control failed the same
  load check. `docs/book/figures.md` records the view, sizes and limitations.
  Dense text remains assigned to whole-book art k101.
- All art handoffs are closed. Proof k147 remains live and records the
  retained diagram labels. The book parent still has live children; no
  parent retirement is due.

## Decisions (running log)

- 2026-10-18: Apply the owner's settled crisp style. Remove the unused sketch
  branches, seed and roughness controls from `_draw.typ`. Keep the palette,
  fonts, geometry and layout modules. Keep the tests for repeatable renders,
  exact outlines and a palette change that leaves the layout record unchanged.
- 2026-10-18: The five unit handoffs request no new diagram for R11. Their
  page tables carry the comparisons and sequences. This is an editorial
  judgement, not a limit of the medium. Read all six chapters for the art
  charter and keep the figure questions and positions.
- 2026-10-18: The requirement to keep the nine layout records unchanged also
  keeps their labels. Apply R12 and R13 to figure captions and alt text on
  the page. Retain the short diagram labels, with their meaning in that text.
  This is a contract restriction on a label change, not a medium limitation.
  Proof k147 already stands later in this node, so no new proof leaf is due.
