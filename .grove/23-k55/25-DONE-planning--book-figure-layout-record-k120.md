# book-figure-layout-record-k120

## Goal

Re-cut the figure ports (`book-figure-sequences-k111` through
`book-figure-settled-exit-status-k116`, and the docs in
`book-figure-retire-plantuml-k117`) for the owner's chosen architecture: a
layout record handed from layout to drawing inside Typst. No port should start
under the old one-pass shape.

## Context

The owner decided on 2026-10-17, answering `layout-draw-separation-k119`
(survey: `docs/research/layout-draw-separation-a.md`, options A–H and the
recommendation):

- **Separation: option B.** Each layout module (`_sequence.typ` now; the coming
  `_c4.typ`, `_state.typ`, `_deployment.typ`) returns a layout record: plain
  data giving the canvas size and, per element, an id, a role, a box or point
  list, dash/arrowhead, and each label's text, anchor and measured width. One
  `_draw.typ` renders any record under a named style. Text stays measured and
  drawn by Typst in DejaVu; rough.js on Bun (option C) is held in reserve as a
  drop-in drawing backend at the same seam, not adopted.
- **Commit each figure's record** as `<figure>.layout.json` beside its SVG, and
  have `task docs:figures` regenerate and compare it, so a restyle shows SVG
  changes and no layout changes. `typst eval 'query(<label>).first().value'
  --in fig.typ` prints a `metadata` value as JSON. It was checked offline and
  byte-identical on the pinned Typst 0.15.1; the survey's appendix has the
  scripts.
- **Ship sketched by default:** k110's deterministic jitter at roughness 2,
  DejaVu Sans, key boxes and tables crisp. This settles k110's "wait". Today
  `_style.typ` defaults to roughness 0, and `legend-block` goes through
  `draw-box`, so keys wobble under roughness > 0. The style rule "keys are never
  sketched" belongs in `_draw.typ`.

What exists: `_sequence.typ` already computes its positioned `items` before its
canvas draws, so its draw loop moves almost as is. `_style.typ` holds the
palette, scale, primitives and the roughness switch. The lint in
`scripts/figures.sh` refuses a figure that imports a package. The figures
directory admits only sources, modules and SVGs (`docs/book/figures.md`), so the
`.layout.json` files need its rules and controls extended.

## Done when

- The work from the record to a green check is cut as leaves ahead of the
  ports. That covers the record's shape, `_draw.typ` with the `crisp` and
  `sketch` styles, `_sequence.typ` returning a record, the `.layout.json` write
  and compare in `task docs:figures` with a control seen to fail, the shipped
  default (roughness 2, keys crisp), and the lint and directory rules. Planning
  decides whether that is one leaf or folded into k111.
- The bodies of k111–k116 say they produce records through their modules and
  draw only through `_draw.typ`. k115 keeps its hand-place-or-compute choice, now
  with the survey's option E (ELK on Bun, or Graphviz via diagraph-layout inside
  Typst) feeding the same record if hand placement proves costly.
- k117's body covers documenting the record, the JSON check, the sketch default
  and the stale "TALA is a separate proprietary install" line in
  `docs/book/figures.md` (d2 0.9.0 bundles TALA).
- The `_ravel-book.md` "Figure migration" section says the ports follow the
  record design.

## Notes

The k108 layout-cost table still holds as an estimate; the record adds a
module boundary, not routing work. Keep "every handoff leaves `task docs:figures`
green" from k108's plan.
