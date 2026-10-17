# book-figure-c4-context-container-k112

## Goal

Replace the C4 system-context and container figures with Typst sources on a
shared C4 module, and remove the trial directory.

## Context

- Figures: `process-authority` (trial source exists, hand-placed, measured
  boxes) and `system-context`.
- The trial's container source holds its own entity, relation and label helpers.
  With a second C4 figure, extract them into `_c4.typ` (entity in a semantic
  role, boundary, relation with label and technology, key). The component view
  in `book-figure-source-components` is the third user; design for it without
  porting it here.
- Record design (`book-figure-layout-record-k120`, survey option B): `_c4.typ` returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
- Layout is manual. Edge routes and label positions were the cost in the trial
  (the added container took an entity, an edge, a legend offset and two retunes).
  Record the effort for `system-context` in the decision log with counts: edges
  routed by hand, label moves, renders to settle.
- Preserve `docs/book/figures.md` "What each view means" for both figures: the
  opaque-result path is green and separate from the worker response; worker end,
  reap and private-directory removal precede the managed launch; boundaries are
  not security boundaries.

## Done when

- Both `.puml` files are replaced by `.typ` with committed regenerated SVGs and
  `.layout.json` records, and `_c4.typ` exists as a reusable module used by both,
  producing records and drawing only through `_draw.typ`.
- Differences from the PlantUML output (orthogonal relations instead of splines,
  cylinder label inside) are listed with the reason, and every fact, tag, number
  and key line is present.
- `docs/book/trial-typst/` is removed after its findings are confirmed to be in
  `docs/book/figures.md` (pins, effort, the extraction test that was not
  adopted, the Typst markup trap `2\.`); the git history of the k107 commit keeps
  the trial.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
