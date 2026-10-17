# book-figure-retire-plantuml-k117

## Goal

Delete the PlantUML and d2 mechanics and rewrite the figure rules to describe
Typst/CeTZ only, in one coherent change, once no `.puml` remains.

## Context

- Plan: `book-figure-migration-k108`, `## Plan`.
- By now `docs/book/src/figures/` holds only `.typ` and `.svg`. Remaining
  PlantUML/d2 surface: `scripts/figures.sh` (jar, d2, Java discovery, layout
  pragma lint, font fallback sed, the `-nometadata`/`--threads 1` workaround),
  `scripts/figures.test.sh` and `scripts/figures-fixtures/figures/*.puml|*.d2`,
  the `docs:figures*` and `setup` descriptions in `Taskfile.yml`,
  `docs/book/figures.md` (layout, renderers, reproducibility, "Diagram kinds
  checked under Smetana"), the contract's "Figure renderers", "Figure delivery"
  and "Figure notations" bullets in `docs/book/contract.md`, the `plantuml` and
  `d2` fence names in `scripts/examples-extract.awk` and `docs/book/examples.md`.
- Record design to document (`book-figure-layout-record-k120`; by now built by
  `book-figure-record-and-draw-k121` and `book-figure-layout-json-check-k122`): the
  layout record and its shape, `_draw.typ` as the only module that draws, the
  `crisp` and `sketch` styles and the shipped default (roughness 2, keys and tables
  crisp), the committed `<figure>.layout.json` and its compare, and rough.js on Bun
  as a held-in-reserve backend at the record seam (not adopted). The survey is
  `docs/research/layout-draw-separation-a.md`.
- `docs/book/figures.md` still says "TALA is a separate proprietary install"; d2
  0.9.0 bundles TALA. Fix or drop the line with the d2 text.
- Historical evidence stays: `docs/evidence/book-figures-k92/` records dated
  PlantUML observations and is not rewritten. The dated viewing records in
  `figures.md` are history; keep them, marked as describing the former renderer.
- Contract rule to change: "A change of renderer or version is a change to this
  contract." This leaf makes that change with the owner's 2026-10-17 decision
  ("migrate full") as its authority.

## Done when

- A repository-wide search for PlantUML, d2, Smetana, `.puml`, Java and the jar
  finds only the evidence directory, dated history in `figures.md`, and Grove
  files. Run it with the controls `references/execute.md` requires, seen to fail.
- `scripts/figures.sh` does Typst only; the Java requirement is gone from
  `task setup`, the task descriptions and `docs/book/figures.md`.
- `contract.md` and `figures.md` state: Typst/CeTZ renderer with its pins; one
  authoritative `.typ` source per figure and no second editable source; `_*.typ`
  modules replace `_*.puml` includes; the SVG is text-as-paths by design, so the
  alt text lives in the page (the plan's decision); the roughness decision from
  `book-figure-sketch-style`, now settled as sketched by default at 2; and the layout
  record, `_draw.typ` and the `.layout.json` check described above.
- `task setup`, `task docs:figures`, `task docs:figures:test` and `task check`
  pass on a clean `target/tools`.
