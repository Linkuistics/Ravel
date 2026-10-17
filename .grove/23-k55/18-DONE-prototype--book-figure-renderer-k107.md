# book-figure-renderer-k107


## Goal

Trial Typst/CeTZ as a shared final renderer for the book's diagrams, so the
owner can judge visual consistency and maintenance cost before migration.

## Context

The owner proposed using diagram tools for layout and a common final renderer
for fonts, line weights and colours, then accepted the Typst/CeTZ recommendation
on 2026-10-17. They clarified: "Prefer Typst/CeTZ; trial before migration".
This prototype precedes the resumed `book-read-one-k70` reader session.

Use `process-authority.puml` and `successful-session.puml` as the dense
architecture and sequence cases. Read `docs/book/contract.md` and
`docs/book/figures.md`; preserve the facts and notation semantics of each.
The published book retains its current figures while this trial is evaluated.

## Done when

- A disposable prototype outside `docs/book/src/` renders both cases with a
  shared Typst/CeTZ style for fonts, strokes, colours, spacing and arrowheads.
- Existing and proposed figures can be compared at actual chapter width.
  Explain any semantic loss or layout tradeoff; do not flatten notation into
  generic boxes and arrows for the sake of similarity.
- Try a longer label and an extra participant. Record the changes required,
  regeneration commands, pinned tools/fonts/libraries and observed effort.
  Final font metrics must inform layout; test any extraction/conversion instead
  of assuming it exists or produces usable coordinates.
- Establish one authoritative source per trial figure: a reproducible pipeline
  from layout input, or maintained Typst source replacing that input. No two
  independent editable sources.
- Show the owner the trial, record their reaction and resulting decision, and
  cut follow-up work if migration is chosen. The reaction is the deliverable;
  this leaf does not authorise a full-book migration on its own.

## Notes

Consult the updated `grove-art` guidance. CeTZ is the shared drawing layer;
notation-specific Typst libraries may help. Prefer the smallest useful trial
over building a universal converter. Changes to admitted media, renderer pins
or regeneration checks belong to the later migration decision.

## Decisions (running log)

2026-10-17 — Trial built and shown to the owner (`docs/book/trial-typst/`, `compare.html`
at the 750px chapter width). Sequence (`successful-session`) and C4 container
(`process-authority`) both render from one maintained Typst source each, no second
editable source; extraction of PlantUML's Smetana SVG was tested (boxes and link
paths are present, text widths agree with Typst to 0.2 units mean) and not adopted.
Observed costs: glyph-path SVG (no `<text>`, 5-7x larger), manual routing for the
container view, CeTZ fetched from the package registry.

2026-10-17 — Owner reaction: "migrate full". Decision: migrate every book figure to
the shared Typst/CeTZ renderer; cut `book-figure-migration-k108` (planning) ahead
of `book-read-one-k70`. This leaf authorised no migration itself.

2026-10-17 — Owner asked whether CeTZ has a "hand drawn" style. Answer: not in CeTZ
0.4.2 (checked the package source: no sketch/wobble mode, only zigzag/wave/coil/brace
decorations). Recorded in k108's Notes; the owner has not requested building one.
