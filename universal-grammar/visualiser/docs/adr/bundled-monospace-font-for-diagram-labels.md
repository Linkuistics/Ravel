# A bundled monospace font for diagram labels

The core measures each label of a rule diagram in one monospace font: IBM Plex Mono at 13 px. The page bundles that font and draws the labels in it. Each character takes one cell of 0.6 em, so the width of a label is its count of cells. A wide East Asian character takes two cells, by its Unicode East Asian Width. The page turns off ligatures. The font is under the SIL Open Font License 1.1.

## Trade-off

A monospace label is wider than a proportional label. In Chromium on macOS, "shows the simplest case" measured 13.8 em in IBM Plex Mono and 10.2 em in the system sans font. With a limit of 18 cells for each line, such a label wraps onto two lines, so the diagrams of the URG grammar grow taller. The page also ships a font file, and it waits for the file before it draws.

In return, the core needs no width table, only a count of cells. Every tested glyph of IBM Plex Mono advanced exactly 0.6 em, including `↻`, `–` and accented letters. So the drawing in each browser matches the coordinates of the core. Each character also has a cell of its own. Symbols that differ only by case, spaces, `_` or `-` stay easy to tell apart, and the rule checks warn about exactly such symbols. The grammar editor and the output view already show symbols in a monospace font.

## Rejected alternatives

A proportional font with a width table. The core would carry the advance of each glyph of the bundled font, and the page would turn off kerning. The diagrams would be narrower and look more like the usual BPMN drawings. But the table must match the font file, and each character outside the table needs a guessed width.

The monospace font of the system, without a bundled file. Its width differs between hosts. In Chromium on macOS, Menlo measured 0.602 em and SF Mono 0.618 em for each character, against the 0.6 em of the core. Chromium does not support `ui-monospace`, and Safari maps it to SF Mono. A long label could then overflow its box.

Source: the design of `rule-diagram-design-k12` in the spec `rule-diagrams`, the measurements of that session on 2026-11-20, and the layout-interaction survey. The font mock-up of `docs/design/rule-diagrams` compares both fonts.
