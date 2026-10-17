# Typst/CeTZ figure-renderer trial (k107) — disposable

Not part of the published book. Nothing under `docs/book/src/` changed.

## What is here

| File | Role |
|---|---|
| `style.typ` | The shared style: DejaVu Sans, 14/16/12 scale, palette roles, 1.5/0.5/0.75 strokes, one arrowhead, one cylinder, key box |
| `seq.typ` | Small declarative sequence-diagram engine on CeTZ (participants, messages, self-calls, create/destroy, dividers, notes) |
| `successful-session.typ` | The only source of the sequence figure |
| `process-authority.typ` | The only source of the C4 container figure (hand-placed, measured boxes) |
| `render.sh` | Regeneration; `./render.sh [outdir]`, default `out/` |
| `compare.html` | Current vs trial at the 750px chapter column, plus the long/extra variants |

## Regenerate

```sh
docs/book/trial-typst/render.sh          # writes docs/book/trial-typst/out/*.svg, ~0.5 s for all four
```

Variants are the same sources with `--input variant=long`.

## Pins

- Typst 0.15.1 (Homebrew `typst`; not fetched by `task setup`).
- CeTZ 0.4.2 (`@preview/cetz:0.4.2`, auto-downloaded on first compile into Typst's package cache; also pulls `oxifmt:1.0.0`). A migration would vendor or checksum it.
- DejaVu 2.37, the four faces already pinned in `target/tools/fonts/dejavu-2.37`, passed with `--font-path` and `--ignore-system-fonts`.
- Observed: two renders, a render from another directory, and a render with system fonts visible are byte-identical (md5, macOS arm64 only; Linux not checked).

## Observations

**Fidelity.** Both figures keep every fact, label, number, tag, key line and colour role of the originals. Fonts, scale, strokes, colours and arrowheads come from one module, so the two notations now match exactly. Differences, none dropped: the key sits in the figure the same as before; the result-channel cylinder carries its label inside instead of below; destroy crosses sit on their own row; C4 relations are orthogonal polylines rather than splines; the «stereotype»/[technology] lines are upright 12pt, not italic as in the PlantUML C4 output (DejaVu Oblique is not a pinned face; whether Typst would synthesise a slant was not tried).

**Final metrics drive layout.** Sequence column gaps, label wrap heights and row heights are computed from `measure()` on the real font. As a separate check, PlantUML's `textLength` for 240 text runs from the two current SVGs was compared with Typst's measured width for the same string, size and weight: mean difference 0.2 units, maximum 2.7 units on a 570-unit line (kerning). A PlantUML-layout → Typst-final conversion would therefore not shift text noticeably.

**Extraction/conversion tested, not adopted.** PlantUML's Smetana SVG does expose node boxes (`rect x/y/width/height`) and link Béziers (`<g class="link">` with `M… C…` paths), but the cylinder is path-only, edge-label positions and roles (`temporary`, `result`) were not checked for extractability and would need mapping, and the result would be a one-way converter with PlantUML as a second editable layer. Not built. Both trial figures are maintained Typst source; there is no `.puml` for them.

**Effort and edit cost.**
- Sequence: adding a participant (`audit sink`), a message to it and a 150-character label needed changes in two lists in the figure source only. Column widths, wrapping, row heights and the legend width recomputed. No coordinates touched.
- Container: adding the same extra container needed an entity (5 lines), an edge with a hand-chosen route and a label position, a legend offset, and re-tuning where the long relation label sits (first attempt overlapped the boundary and the route collided with another label; two more iterations). Edge routing and label placement are manual.
- Size: `style.typ` 98 lines, `seq.typ` 222, the two figure sources 60 and 136 lines, against 144 lines of shared `.puml` plus 86 of figure source. The Typst side is more code, but 320 lines of it are reusable engine/style; the per-figure part is comparable.
- Build: all four figures in 0.5 s, no JVM.

**Costs and caveats observed.**
- Typst's SVG has no `<text>`: every glyph is a path (`<use>` of an outlined glyph), so text is neither selectable nor searchable in the SVG and the files are 5–7× larger (216–246 KB against 36–40 KB). Alt text lives in the page either way.
- SVG root is sized in `pt` (`width="1204pt"`); mdBook scaling is unaffected, but anything reading the pixel size sees 4/3 larger.
- Typst's markup trap: a label starting `2. ` is parsed as a numbered list item (`2\.` fixes it).
- Package dependency: CeTZ is fetched from the Typst package registry; offline builds need it vendored.
- Single-engine risk: both PlantUML C4/UML conventions (standard shapes, `legend`, standard library) now exist only as this trial's hand-written primitives. A third notation (state diagram, deployment) would need new engine code. d2 diagrams, if any, have no counterpart here.

## Owner reaction and decision

Owner, 2026-10-17: "migrate full". Follow-up: `book-figure-migration-k108`. Asked whether CeTZ has a hand-drawn style: it has none (CeTZ 0.4.2 source checked); not pursued.
