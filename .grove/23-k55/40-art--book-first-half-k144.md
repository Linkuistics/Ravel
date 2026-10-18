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
