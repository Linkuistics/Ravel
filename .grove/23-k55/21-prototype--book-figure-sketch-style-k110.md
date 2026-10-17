# book-figure-sketch-style-k110

## Goal

Show the owner the book's figures in a hand-drawn style and record their
reaction, so the style is decided before the remaining figures are ported.

## Context

The owner said on 2026-10-17 that a hand-drawn style "would be very helpful in
making the document more 'friendly'". CeTZ 0.4.2 has no sketch mode (its only
path decorations are zigzag, wave, coil and brace), so the effect is a
deterministic jitter helper in `_style.typ`, built on the primitives
`book-figure-engine` put there. d2's `sketch: true` was never used; the book set
it false.

Read the plan in `book-figure-migration-k108`. The trial figures
(`successful-session.typ`, `process-authority.typ` under
`docs/book/trial-typst/`) are the test material.

## Done when

- A switch in `_style.typ` (roughness 0 draws the crisp figure byte for byte as
  before) turns on hand-drawn outlines, lines, arrow shafts and cylinders.
  Output is deterministic: jitter comes from a pure function of the figure name
  and element index, not from a clock or a random source, so two renders are
  byte-identical and `task docs:figures` still compares exactly.
- Jitter amplitude is small against box padding and label clearance so no text
  touches a line, and arrowheads, X marks and dashes stay recognisable. Text is
  still laid out from measured metrics.
- The owner sees crisp and sketched versions of both trial figures side by side
  at the 750px chapter column (extend `compare.html`), with three roughness
  levels, and says which they want. Also ask the one font question: keep
  DejaVu Sans for legibility, or try a pinned handwriting face. A new face means
  a licence, a SHA-256, a measured re-layout and a legibility check at 14pt, so
  show it only if the owner wants to see it.
- The owner's words are recorded, with the decision: the default roughness the
  book ships (possibly 0), the font, and whether the key boxes and tables
  stay crisp. `_style.typ` carries the decision; later leaves read it from there.
- `task docs:figures` and `task docs:figures:test` still pass.

## Notes

Human-in-the-loop: the reaction is the deliverable. If the owner has not seen
it, stop and ask rather than choosing a default. Ask with the trade-off named:
friendliness against ink noise in the densest views (`process-authority`,
`successful-session`) at chapter width.
