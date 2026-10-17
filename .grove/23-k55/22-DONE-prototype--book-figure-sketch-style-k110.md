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

## Decisions (running log)

- 2026-10-17: Prototype built. `_style.typ` has a `roughness` switch
  (`--input roughness=0..3`; 0 is byte-identical to the previous output, checked
  with `cmp`; two renders are byte-identical; `task docs:figures` and
  `task docs:figures:test` pass). The two trial figures are ported onto the engine
  as `docs/book/trial-typst/sk-*.typ` and rendered by `render-sketch.sh`;
  `compare.html` shows crisp and roughness 1-3 at 750px. Jitter is a pure function
  of `--input figure=NAME` plus the element's own geometry, not an element index
  (Typst has no mutable counter inside a canvas).
- 2026-10-17: Owner, mid-review, redirected the approach: "We should investigate
  d2, excalidraw and any other libraries that can do pure drawing (without
  layout), and completely separate the process of text->diagram/layout and the
  subsequent styling." Cut `layout-draw-separation-k119` (research-a) ahead of
  the port leaves. The style decision here (roughness, font, key boxes) is still
  open and may be superseded by that research.
- 2026-10-17: Owner reaction to the sketch comparison: "Roughness 2 is good.
  I like DejaVu. [Key boxes crisp:] yes. [Ship the default:] wait." Recorded
  decision: preferred roughness 2; font stays DejaVu Sans (no handwriting face);
  key boxes and tables stay crisp. `_style.typ` keeps default roughness 0, so
  `task docs:figures` is unchanged; the shipped default waits on
  `layout-draw-separation-k119`, which may replace the in-primitive switch. Note
  `_style.typ` does not yet keep key boxes crisp under roughness > 0:
  `legend-block` goes through `draw-box`, so a later leaf must give it a crisp
  option if the switch survives.
- 2026-10-17: Owner asked whether an existing tool already does this. Tried
  both on real figures. PlantUML 1.2026.8 `skinparam handwritten true`
  (it warns to prefer `!option handwritten true`) wobbles every shape, keeps
  colours and DejaVu text, but has one fixed roughness, curved edges from its
  layout, a wobbly key, and needs Java. d2 0.9.0 `sketch: true` gives a heavy,
  hatched look with d2's own font and layout, not DejaVu; the book sets
  `sketch: false` on purpose. Neither separates layout from style. Passed to
  k119.
