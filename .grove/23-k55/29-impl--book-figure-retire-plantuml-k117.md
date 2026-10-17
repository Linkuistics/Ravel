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
  `book-figure-sketch-style`.
- `task setup`, `task docs:figures`, `task docs:figures:test` and `task check`
  pass on a clean `target/tools`.
