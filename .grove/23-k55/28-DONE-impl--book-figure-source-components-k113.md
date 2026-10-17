# book-figure-source-components-k113

## Goal

Port the C4 component view and report what its hand-placed layout cost, so the
owner can see the effort the remaining unproven views are likely to take.

## Context

- Figure: `source-components`. Size: 3 entities and 1 external system, 3
  relations, one dashed boundary around two components, and an 8-line key. The
  `Lay_D` hint kept the worker above the managed process in PlantUML.
- Record design (`book-figure-layout-record-k120`, survey option B): `_c4.typ` returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
- It reuses `_c4.typ`. The new feature is a boundary that encloses some entities
  and not others, with a stereotype line; if `_c4.typ` does not support it, extend
  the module.
- Preserve: the Ravel CLI's checking code and the linked runner share one
  process, the worker is separate, the boundary means process membership not
  confinement, no third runner executable is installed.

## Done when

- `source-components.puml` is replaced by a committed `.typ`, regenerated SVG and `.layout.json`.
- The decision log records the layout cost as numbers: entities, relations placed
  by hand, label moves, renders needed to settle, minutes of work, and anything
  that was clumsy. Compare with the estimate in `book-figure-migration-k108`'s
  cost table and say whether the table was right, so the owner can judge the
  rest before they are spent.
- Facts and key lines are preserved; differences are listed.
- `task docs:figures`, `task docs:book:build` and `task check` pass.

## Decisions (running log)

- 2026-10-17 — `source-components` is ported with no change to `_c4.typ`, `_draw.typ` or `_style.typ`. The k112 sketch had already fixed the two boundary gaps (a boundary narrower than its name, a header above the title), and a boundary around two of four entities with a `[Container]` tag worked as it stood. `source-components.puml` is removed; the committed set is `source-components.typ` (39 lines, 8 of them the key), the regenerated `source-components.svg` (211 KB) and `source-components.layout.json`.
- 2026-10-17 — Layout cost, as numbers: 4 entities (2 in the boundary, 2 outside), 1 boundary (2 members, a tag), 3 relations. Relations routed by hand: 0 — all three took the default facing sides, because every pair of boxes faces across a gap, so no side, offset, `jog` or `seg` was written. Label moves: 0 (`dx`, `dy`, `t` and `side` all unused); label placement chosen for 3 of 3, each a wrapping width (`w: 300`, `w: 110`, `w: 300`). Coordinates chosen: one column pair (the worker and managed process share `cx`, as `Lay_D` required), found by eye and edited once (640 → 740). Renders to settle: 2 — the first was structurally right and showed two placement faults (the technology lines were wider than the gap and ran into both boxes; "Uses after worker cleanup" ran across the boundary's right edge), the second fixed both. Minutes of work: about 3 from the first write of the source to the settled render, not counting the reading of `_c4.typ`, the old source and `figures.md` beforehand; the second render is also when the SVG and record were written.
- 2026-10-17 — Was the k108 table right? It estimated Low to Medium for this view; the actual was Low, so the table was a little pessimistic here. Two reasons the estimate should not be extended to the views still to come: the partial boundary, the new element, cost nothing because k112 had already tried it, and this figure has no routing at all (three straight, axis-aligned runs between facing boxes), which is the cost that dominated `process-authority` (7 hand routes). The rest of the table concerns different notations: `installed-files` needs a nested-enclosure primitive that does not exist, and the two state views have fan-out and long guard labels, so neither is evidence from this port. The owner's reading: a C4 figure whose boxes face each other costs minutes; the cost is routes and label collisions, and it scales with the number of relations that do not face.
- 2026-10-17 — What was clumsy: (1) the module does not take a label's width into account when the figure source spaces two boxes, so the first render put technology lines wider than the 230-point gap between the boxes, and the source had to widen the gap by eye (to 330 points); (2) a wrapped technology line ("[Rust library / interface]") breaks inside its brackets, because `w` wraps label and technology line alike; (3) a long relation label beside a vertical run inside a boundary has to be wrapped by hand to stay inside the dashed outline. None of these needed a module change; each is a render and a number.
- 2026-10-17 — Facts and key lines are preserved. A script (`python3 -I`) compared the 28 quoted strings, key lines and title of the old `.puml` (taken from `HEAD`, with typographic apostrophes normalised) with every string in the new layout record and found none missing; with `selected program and arguments` removed from the haystack the same comparison reports exactly that string missing, so it can fail. Differences, recorded in `docs/book/figures.md` ("Differences from the PlantUML C4 output"): orthogonal relations, bold names and title, measured wrapping, sketched style, and the `Lay_D` hint, which became the shared `cx` of the worker and the managed process. Nothing else moved: the boundary still reads "Ravel CLI — one Rust process (bin/ravel)" with `[Container]`, the key keeps its eight lines in order and the worker's description keeps its line break.
- 2026-10-17 — Observed, not changed (outside this leaf): the key's last line says "Worker paths in its box are relative to crates/ravel", while the first line of the worker's description is `crates/ravel/worker/src` (full) and the two after it (`worker/sdk`, `worker/examples/static.ts`) are relative. That is the old figure's wording, preserved so the port stays a port; whether it is worth a one-word fix is an art-stage call (`ravel-book-k101`).
- 2026-10-17 — The k122 control: moving the `managed` entity 10 points (`cx` 740 → 750) makes `task docs:figures` fail on both `source-components.layout.json` and `source-components.svg`; restoring it passes. `docs/book/figures.md` now says the component view is on `_c4.typ`, gives these counts under "Effort and edit cost", and lists the figure's SVG at 211 KB (the nine figures total 1.32 MB, of which the three PlantUML ones are 64 KB).
- 2026-10-17 — `task docs:figures` (9 figures match), `task docs:figures:test` (exit 0) and `task docs:book:build` pass.
- 2026-10-17 — `task check` passes (339 of 339, one run, no `.rs` file changed).
