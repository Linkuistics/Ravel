# layout-draw-separation-k119

## Goal

Survey tools that draw diagrams without laying them out, and the ways to split
"text to diagram and layout" from "styling and drawing", so the book can restyle
every figure (hand-drawn or crisp) without touching its layout.

## Context

The owner said on 2026-10-17, while reviewing the `book-figure-sketch-style-k110`
prototype: "We should investigate d2, excalidraw and any other libraries that
can do pure drawing (without layout), and completely separate the process of
text->diagram/layout and the subsequent styling."

Today the Typst/CeTZ engine (`docs/book/src/figures/_style.typ`, `_sequence.typ`)
does layout and drawing in one pass, and the k110 sketch effect is a jitter
helper inside its drawing primitives. Candidates to cover at least: d2 (its
`sketch: true` mode and its layout engines, dagre/ELK/TALA, and whether its
layout can be exported as data), Excalidraw (the JSON scene format and its
rough.js rendering), rough.js itself, and any others the survey finds (for
example Graphviz/ELK emitting positions, Mermaid's look setting, tldraw,
svg-to-sketch post-processors).

The downstream questions, leaf by leaf: `book-figure-sequences-k111` through
`book-figure-settled-exit-status-k116` ports each figure; whether they should
emit a layout intermediate (positions, sizes, routes, labels) and hand it to a
separate style stage decides what those leaves build. The constraints that stay:
offline, pinned and byte-reproducible output; text laid out from measured
metrics so no label touches a line; no JVM or network at render time unless
admitted (`docs/book/figures.md`).

## Done when

- A cited survey of the candidates: what each does for layout, what for drawing,
  whether the two can be run apart, determinism and pinning, licence, size and
  build weight, and how its sketch rendering looks.
- The survey names the architecture options (for example layout-to-JSON then a
  style stage, Typst layout then a post-process of the SVG, d2 layout then
  Excalidraw/rough.js render) with the trade-offs, and says what each would
  change in the k111-k116 leaves.
- It ends with a recommendation and the questions the owner must answer.

## Already observed (k110, 2026-10-17)

PlantUML `handwritten` and d2 `sketch: true` each give a sketch look but bundle
it with their own layout; see the k110 running log. Start from there, and also
check which of the two can emit layout as data.
