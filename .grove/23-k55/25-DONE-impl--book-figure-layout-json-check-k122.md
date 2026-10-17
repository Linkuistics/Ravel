# book-figure-layout-json-check-k122

## Goal

Make `task docs:figures` write, regenerate and compare each Typst figure's layout
record as `<figure>.layout.json` beside its SVG, so a restyle shows SVG changes
and no layout change, and make the directory rules admit exactly those files.

## Context

- Follows `book-figure-record-and-draw-k121`: every `.typ` figure is a call to
  `_draw.typ`'s render with a record, which attaches `metadata(record) <layout>`.
  `typst eval 'query(<layout>).first().value' --in FIG.typ` prints it as JSON
  (byte-identical across runs on the pinned Typst 0.15.1).
- No `.typ` figure is committed until `book-figure-sequences-k111`, so this
  leaf's tests run on fixtures under `scripts/figures-fixtures/figures/`
  (`layers.typ` and `_draw.typ` are the current Typst fixtures). The first
  real `.layout.json` arrives with k111, which is the leaf that proves the check
  on a real figure.
- The eval must run under the same isolation as the render in `render_all`
  (empty environment, private home, no system or embedded fonts, pinned package
  cache, refusing proxy), and stderr output must fail it. Text widths in the
  record come from the pinned DejaVu, so a missing font must not pass silently.
- `lint_layout` today admits `.puml`, `.d2`, `.svg` and `.typ`; `docs/book/figures.md`
  states "the directory holds `.puml`, `.d2` and `.svg` files only", which is
  already stale for `.typ` — correct that sentence for what the check admits now.

## Done when

- For each `.typ` figure, `check` evaluates the record, normalises nothing beyond
  Typst's own JSON output, and fails if `<stem>.layout.json` is missing or differs;
  `update` rewrites it; an unreadable or label-less figure fails with a message
  naming the figure.
- The directory lint refuses a `.layout.json` with no `.typ` source (and one
  beside a `.puml`/`.d2`), and refuses a `.typ` figure with no `.layout.json` in
  `check`, each message naming the file.
- Controls in `scripts/figures.test.sh`, each seen to fail on the unguarded
  behaviour first: a changed element position fails the JSON compare and the
  SVG compare; a restyle only (`style` input, or a change to the style tables in
  `_style.typ`/`_draw.typ`) changes the SVG and leaves the JSON byte-identical
  and the JSON compare passing; a stale JSON, an orphan JSON, and a record that
  would contain a style value, if the session finds a way to detect one.
- `docs/book/figures.md`'s directory rule and the Taskfile `docs:figures*`
  descriptions state that the record is checked; fuller documentation of the
  record stays with `book-figure-retire-plantuml-k117`.
- `task docs:figures`, `task docs:figures:test` and `task check` pass.

## Notes

If the JSON turns out to depend on font or Typst build in a way the SVG compare
does not already cover, record the difference rather than loosening the compare.

## Decisions (running log)

2026-10-17 — The record is evaluated in `render_all`, beside the SVG compile, through
one shared `typst_isolated` helper (empty environment, private HOME, no system or
embedded fonts, pinned package cache, refusing proxy, `--input figure=STEM`), so the
eval cannot drift from the render's isolation. Output is `typst eval --pretty`, written
as Typst prints it (it ends in a newline): pretty-printed because Typst's compact
JSON is one line and a one-line file gives git nothing to diff. Stderr output fails
the eval as it fails the compile.

2026-10-17 — `check` compares the record apart from the SVG, so a restyle (SVG
differs, record unchanged) and a layout change (both differ) read differently in the
output. `update` rewrites it ("wrote"/"rewrote layers.layout.json"). A figure with no
`<layout>` label fails naming the figure and the remedy (`render` attaches it).

2026-10-17 — Directory lint: `*.layout.json` is admitted, and refused unless a
figure `.typ` of that stem sits beside it; a module's stem (leading underscore) does
not count, so `_draw.layout.json` is refused as well as one beside a `.puml`/`.d2`.

2026-10-17 — Style-value detection, found: Typst prints a colour as the string
`rgb("#e8edf2")`, a length as `"1.5pt"` and content as an object with a `"func"` key.
`lint_record` refuses a record with a string value of those shapes, or a key naming a
style property (fill, stroke, paint, thickness, color, weight, size, font, roughness,
radius, opacity, dash, style). It runs before `update` writes, so a leaking record is
never committed. A denylist and not an allowlist of keys, so a new layout module's
fields do not need `figures.sh` edited; a label that is exactly a hex colour or a
length is a false positive, accepted and documented in the function's comment.

2026-10-17 — The fixture `_draw.typ`/`layers.typ` now follow the book's contract
(`render(record)` attaches `metadata(record) <layout>`), so the fixtures hold the
first committed `.layout.json` (`layers.layout.json`). `layers.svg` is unchanged.

2026-10-17 — Controls, each seen to fail in a mutated copy of `figures.sh` run
through `FIGURES_SCRIPT`: the JSON compare disabled (the moved-element control
died: only the SVG message appeared), `lint_record` a no-op (the colour-leak
control died), the orphan rule removed (the orphan control died), the missing-record
refusal removed (the missing-record control died), the eval's `die` replaced by
`true` (the unlabelled-figure control died; the stderr guard still refused it but with
the wrong message). The restyle controls pass by construction and check the output
names no `.layout.json`; on the book's modules the record is byte-identical at the
shipped style, crisp and roughness 3, and a change to `_style.typ`'s palette changes
`engine.svg` and not `engine.layout.json`. The controls were not run in parallel
after a run of five at once failed once in the PlantUML section on an unpinned-package
message that did not reproduce in a serial run; they share `target/tools`.

2026-10-17 — The font dependence of the record is covered by the same check as the
SVG's: text widths in the record are measured in the pinned DejaVu, and a missing
face is a warning, which fails the eval on its stderr. No difference between the
record's and the SVG's dependence on font or Typst build was found, so nothing is
loosened.
