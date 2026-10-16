# book-figures-k59

## Goal

Make a figure reproducible: a Taskfile task regenerates every committed figure
from its editable source with PlantUML 1.2026.8 and d2 0.9.0 and compares the
result byte for byte with the committed SVG, and the layout for a figure on
disk, with its recorded renderer version and regeneration command, is settled
once so the art stage and every drafting session use it.

## Context

- Contract, "Format and media" (figure notations, renderers, delivery) and
  "Figure needs": `docs/book/contract.md`. PlantUML with its bundled
  C4-PlantUML 2.13.0 and the built-in Smetana layout, so no separate Graphviz;
  d2 for structure and flow diagrams; every figure committed as editable text
  source beside the SVG it generates, with the renderer version, layout
  settings and regeneration command recorded; no reliance on GitHub's Mermaid;
  a figure made only of the page's own text needs no source.
- On the host used for this planning, `plantuml -version` reports 1.2026.8 and
  `d2 --version` reports v0.9.0 (checked 2026-10-16); PlantUML needs a Java
  runtime. Whether to fetch the pinned jar and d2 archive into `target/tools`
  with pinned digests (the `setup_bun` pattern in
  `crates/ravel/scripts/dispatch.sh`) or only to assert the installed versions
  is this leaf's decision. Record it with the reason. Either way a version other
  than the pin must refuse, because a different renderer is a different figure.
- Depends on `book-toolchain-k58` for `docs/book/src/` to exist and for the
  stable page names. Existing editable views under `docs/design/components/`
  are Mermaid and stay where they are; they become figure sources only when the
  second half's art stage redraws them.

## Done when

- A layout is recorded for a figure: where its source, its SVG and its metadata
  live under `docs/book/src/` (for example `figures/<name>.puml|.d2` beside
  `<name>.svg`), how the renderer version, layout settings (Smetana pragma, d2
  layout engine, theme, sketch off) and regeneration command are recorded for
  each, and how a page references the SVG with alt text.
- `task docs:figures` regenerates every figure into a scratch directory and
  fails on any difference from the committed SVG; `task docs:figures:update`
  (or the leaf's equivalent) rewrites the committed ones. Names follow the
  `docs:` family.
- Both renderers are proven reproducible: regenerate each sample twice and
  compare, and note anything non-deterministic (timestamps, ids) with how it
  is neutralised. A renderer or layout pin that changes the output is seen to
  fail the compare.
- A control that has been seen to fail: a changed source, a changed SVG and a
  wrong renderer version each make the task fail; an unchanged tree passes.
- A C4 context sample and a d2 sample exist only as the task's controls, kept
  under a test location that is not in the book, unless a book figure from the
  contract's figure list can serve as the first real one without pre-empting the
  art stage; if so, say which.
- The task and its prerequisites (Java, PlantUML, d2) are documented where a
  contributor looks (the task `desc` and a short comment), and `task check`
  does not require them.
- One page build is run with a figure embedded, in the mdBook build and as the
  raw Markdown path GitHub would use, to confirm the relative SVG path loads
  in both. The GitHub rendering itself is unchecked until a push; say so.

## Notes

- The contract says a change of renderer or version is a change to the
  contract. A needed deviation (a Smetana limitation that forces Graphviz, a
  rendering defect) is an escalation to the owner, not a quiet substitution.
- The visual vocabulary (palette, line weights, how a boundary or an authority
  is drawn) belongs to the art stage and to the first figures; this leaf only
  makes figures reproducible. Do not design the vocabulary here.
- One in-session reviewer is allowed by the spine's rule.
