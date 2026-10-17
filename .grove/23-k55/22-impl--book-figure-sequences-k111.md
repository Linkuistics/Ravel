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
- Notation facts to preserve: `docs/book/figures.md`, "What each view means"
  (X ends a lifetime only after confirmed removal; cancellation figures use
  direct INT; no X certifies teardown there). Palette and roughness come from
  `_style.typ`, including `book-figure-sketch-style`'s decision.

## Done when

- Each of the three `.puml` files is replaced by a `.typ` whose regenerated SVG
  is committed; no `.puml` for them remains. Every label, note, key line, colour
  role and ordering from the old figure is present; list any difference with its
  reason in the decision log.
- Adding a participant and lengthening a label in one cancellation figure needs
  no coordinate edits (the trial's claim, checked on a figure it did not cover).
- The pages that embed the figures still carry their alt text and captions; edit
  them only where the figure's facts changed.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
