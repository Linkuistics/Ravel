# book-figure-sequences-k111

## Goal

Replace the three PlantUML sequence figures with Typst sources in the shared
sequence engine, and prove the engine covers the notation features they use.

## Context

- Figures: `successful-session` (trial source exists), `selection-cancellation`,
  `supervision-cancellation` under `docs/book/src/figures/`.
- The trial covered messages, replies, create, destroy, dividers, notes and one
  self-call. The cancellation figures add notes spanning participants, a
  `database` participant (cylinder head) and multi-line self-calls; check each
  against `_sequence.typ` and extend the engine, not the figure, if one is
  missing.
- Record design (`book-figure-layout-record-k120`, survey option B): `_sequence.typ` already returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
  k121 moved the draw loop out; this leaf proves the record covers the features below, extending the layout side only.
- Notation facts to preserve: `docs/book/figures.md`, "What each view means"
  (X ends a lifetime only after confirmed removal; cancellation figures use
  direct INT; no X certifies teardown there). Palette and roughness come from
  `_style.typ`, including `book-figure-sketch-style`'s decision.

## Done when

- Each of the three `.puml` files is replaced by a `.typ` whose regenerated SVG
  and `.layout.json` are committed (the first real ones: the check from k122 is seen
  to pass on them and to fail on a moved element); no `.puml` for them remains. Every label, note, key line, colour
  role and ordering from the old figure is present; list any difference with its
  reason in the decision log.
- Adding a participant and lengthening a label in one cancellation figure needs
  no coordinate edits (the trial's claim, checked on a figure it did not cover).
- The pages that embed the figures still carry their alt text and captions; edit
  them only where the figure's facts changed.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
