# book-figure-record-and-draw-k121

## Goal

Split figure layout from drawing: define the layout record, move every stroke
into a new `_draw.typ` that renders a record under a named style, make
`_sequence.typ` return a record instead of drawing, and ship the sketched
default. Nothing drawn changes except by the decisions below.

## Context

Owner's decisions (2026-10-17, `book-figure-layout-record-k120`; survey
`docs/research/layout-draw-separation-a.md`, option B): a layout module returns
plain data; `_draw.typ` is the only module that draws; text stays measured and
drawn by Typst in DejaVu; shipped sketched at roughness 2 with key boxes and
tables crisp.

What exists in `docs/book/src/figures/`: `_style.typ` holds the palette, scale,
`size-of` text measuring, the roughness switch and the `draw-*` primitives;
`_sequence.typ` computes positioned `items` and then draws them in a `canvas`.
No `.typ` figure is committed yet (the figures are still `.puml`); the trial
sequence figure under `docs/book/trial-typst/` (`successful-session.typ`) is the
working input. `scripts/figures.sh` passes no `--input` to Typst, so today
roughness is 0 and the figure name is empty.

Design points for the session to settle and record:

- **The record's shape.** A dictionary: canvas `w`/`h`, then a list of elements.
  Each has an `id`, a `role` (the `kinds` roles, `relation`, `result`, `key`, and
  so on), a box or point list in y-down canvas units, dash and arrowhead, and for
  each label its text, anchor and measured width (and wrapped height). It
  contains no style: no fill, stroke weight, font size or roughness. Colours enter
  only as roles, so a restyle cannot change it. Plain data only (strings,
  numbers, booleans, arrays, dictionaries), so `typst eval` prints it as JSON.
  Write the shape as a header comment in `_draw.typ` and a short worked example.
- **Public entry.** `_draw.typ` exports one function a figure source calls with a
  record (`#render(sequence(...))` or similar). It draws the record and attaches
  the record as `metadata(record) <layout>` so `typst eval 'query(<layout>).first().value'
  --in FIG.typ` prints it (checked offline on the pinned Typst 0.15.1: appendix of
  the survey). It must add nothing visible to the page.
- **Styles.** `crisp` is roughness 0. `sketch` takes a roughness; the shipped
  default is sketch at 2, k110's deterministic jitter. Select by Typst input
  (`--input style=crisp`, `--input roughness=N`) with the default compiled in, so
  a bare `typst compile` of a figure gives the shipped look.
- **Keys and tables are never sketched** — a rule of `_draw.typ`, keyed on the
  element's `role` (`key`), not on a flag the layout module sets. Today
  `legend-block` goes through `draw-box`, so keys wobble above roughness 0.
- **What stays in `_style.typ`:** palette, roles, scale, strokes, text measuring.
  The drawing primitives, jitter functions and the CeTZ import move to `_draw.typ`.
- **Per-figure seed.** The jitter seed comes from the figure name. Make
  `scripts/figures.sh` pass `--input figure=STEM` for every render; without it
  every figure seeds from the empty name.

## Done when

- `_draw.typ` exists, is the only `.typ` that imports CeTZ or calls a stroke
  primitive, and has the record shape documented in its header. `_style.typ` no
  longer imports CeTZ. `_sequence.typ` returns a record and does not draw.
- Equivalence is shown, not asserted: render the trial `successful-session`
  through the old modules (take them from git, into a scratch directory) and the
  new ones, and compare. At `--input style=crisp` the SVG is byte-identical. At
  roughness 2 the SVG differs from the old roughness-2 output only in the key
  box, and the diff is listed. If crisp is not byte-identical, say where and why.
- `typst eval` on that figure prints the record as JSON; two evaluations are
  byte-identical, and evaluating with `--input roughness=3` prints the same bytes
  as roughness 0 (the record carries no style). Seen by running it.
- The shipped default is sketch at roughness 2 with keys crisp, visible in the
  trial figure rendered with no `--input` at all.
- `scripts/figures.sh` passes `--input figure=STEM`; `lint_typst` refuses a figure
  source that imports CeTZ or a package (as now) and a module other than
  `_draw.typ` that imports CeTZ, with a fixture in `scripts/figures-fixtures/` and
  a control in `scripts/figures.test.sh` seen to fail first.
- `scripts/figures.test.sh` and `task check` pass. No committed figure changes in
  this leaf (none is a `.typ` yet); `docs/book/figures.md` is edited only where it
  would now be false (the draw-through-`_style.typ` statement); the full rewrite
  stays with `book-figure-retire-plantuml-k117`.

## Notes

Open for the session, not decided: whether `figure-page` sets the page up in
`_draw.typ`'s `render` or stays in `_style.typ`. Prefer whichever leaves a figure
source at two lines: an import and one call.
