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

## Decisions (running log)

2026-10-17 — Three modules, not two. Layout vocabulary is a new `_layout.typ`
(constructors for each element shape, `plain`, `text-el`, the key box), beside
`_style.typ` (palette, roles, scale, text measuring; no CeTZ) and `_draw.typ`
(the only CeTZ import, the primitives, the jitter, `render`). The constructors
are the one place that spells the record, so the next layout modules
(`_c4.typ`, `_state.typ`, ...) cannot drift from the shape. `_draw.typ`'s header
carries the shape and a worked example, as the brief asked.

2026-10-17 — `render` takes a record or a deferred layout. Measuring text needs
`context`, and a value computed in `context` cannot leave it, so `sequence(...)`
returns a function and `render` calls it inside its own `context`, after
`figure-page` has set the font. A figure source is `#import` of the layout
module, `#import "_draw.typ": render` and `#render(sequence(...))`. `figure-page`
stays in `_style.typ` and `render` applies it (`show: figure-page` inside a
function sets the page correctly: crisp output is byte-identical).

2026-10-17 — Roles carry colour. `msg(..., color: pal.green)` became
`msg(..., role: "result")`; lines have roles `relation`, `result`, `lifeline`,
`rule`, `fold`, `end` (a `strokes` table in `_style.typ`); boxes keep the `kinds`
roles plus `key` and `divider` (which used to borrow `note` and `external`). Text
takes a `face` (`title`, `head`, `body`) for size and weight, and a role only to
colour it. A record holds no colour, weight, size or roughness.

2026-10-17 — A label in a record is a plain string. `plain` flattens text,
spaces, line breaks and the apostrophe smartquote, and drops the spaces on
either side of a line break (Typst drops them in content, keeps them in a
string; kept, they widened the head labels by one space). A double quote or
other mark must be written into a string. A label drawn from a string holding
"\n" is sized by CeTZ 0.23pt taller than the same text joined by `linebreak()`,
and a fixed-width box taller than an auto-width one, so `label()` rebuilds the
line breaks for both measuring and drawing, and a text element says `wrapped`.
Both were found by diffing the SVGs, not predicted.

2026-10-17 — Styles. `--input style=crisp|sketch`, default `sketch`;
`--input roughness=N` (default 2) selects sketch at N; `style=crisp` is 0
whatever `roughness` says. Keys are never sketched by `crisp-roles = ("key",)`
in `_draw.typ`, keyed on the role. The jitter still depends on the figure name
and element geometry and not on ids, which keeps roughness-2 output identical
to k110's apart from the key.

## Outcome

Verified by running, 2026-10-17, on the pinned Typst 0.15.1 and CeTZ 0.4.2
(macOS arm64), with the old modules taken from git into a scratch directory and
the trial `successful-session` pointed at each:

- **Crisp is byte-identical** to the old modules, base and `variant=long` (`cmp`
  clean). Two differences surfaced and were fixed rather than accepted: a label
  rebuilt from a string kept a space after its line break (head labels one space
  wider), and CeTZ sized a string holding "\n" and a fixed-width box 0.2 to 11pt
  taller than the old content (see the decisions above).
- **Roughness 2 differs from the old roughness 2 only in the key box**: the old
  SVG drew the key's rounded rectangle as a fill plus a wobbling outline; the new
  one draws it as one filled, outlined path, and the canvas height drops by
  0.83pt because the wobble no longer pushes the key out. The diff is 11 lines of
  `diff`, those two places.
- **`typst eval 'query(<layout>).first().value'`** prints the record as JSON
  (79 elements, unique ids). Five evaluations (default, run twice, `roughness=3`,
  `style=crisp`, no `figure` input) have one SHA-256, so the record carries no
  style and does not depend on the seed.
- **Rendered with no `--input` at all** the figure is sketched at roughness 2 with
  a crisp key box (read from the PNG), and equals `style=sketch roughness=2`.
- **Controls seen to fail first**: a layout module importing CeTZ passed the
  unchanged lint; the same render without `--input figure=STEM` failed the
  shipped-look control; an emptied `crisp-roles` failed the key control (found 0
  solid paths). `scripts/figures.test.sh` passes with them, `task docs:figures`
  matches 9 figures, and `task check` passed (339 tests).

`docs/book/figures.md` was edited only where it had become false; the full
rewrite stays with `book-figure-retire-plantuml-k117`. For the leaves that follow:
a layout module returns a deferred layout (a function) built from `_layout.typ`'s
constructors, and its roles are the ones in `_style.typ`.
