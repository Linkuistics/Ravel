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
