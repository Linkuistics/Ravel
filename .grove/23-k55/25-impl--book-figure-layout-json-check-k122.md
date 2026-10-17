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
  (`layers.typ` and `_shapes.typ` are the current Typst fixtures). The first
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
