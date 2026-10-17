# book-figure-settled-exit-status-k116

## Goal

Port the second state view on `_state.typ` and report its layout cost.

## Context

- Figure: `settled-exit-status`: 9 states, 8 transitions, two green, with very
  long outcome labels (the failure box lists eight exit codes over eight lines)
  and a 10-line key. The long boxes drive the layout, which flows left-to-right
  with outcomes to the right. The filename stays for stable links.
- Record design (`book-figure-layout-record-k120`, survey option B): `_state.typ` returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
- Preserve the "What each view means" row: outcome precedence for
  `start-session`, not call order; cancellation, original failure, required
  cleanup, final write; cleanup is attempted before reporting; an original
  failure keeps its status if later cleanup fails.
- Every number in the exit-code boxes is checked against
  `crates/ravel/src` or chapter 6's status table, not copied blind.

## Done when

- `settled-exit-status.puml` is replaced by a committed `.typ` and `.layout.json`.
- Layout cost recorded as numbers, compared with the plan's cost table.
- `task docs:figures`, `task docs:book:build` and `task check` pass.

## Decisions (running log)

- 2026-10-17 — Method: no new module code. `settled-exit-status` is a ladder (four steps, then the success outcome, in one column) with an outcome beside each step, so it is grid cells on the existing `_state.typ`: steps in column 0 rows 1 to 4, the success outcome at column 0 row 5, the four other outcomes in column 1 level with their steps. The two transitions that leave a step for a tall outcome use `out: ("right", "align")`, which sits the port level with the outcome's centre; the rest take the facing sides.
- 2026-10-17 — Exit codes checked against the source, not copied: `crates/ravel/src/refusal.rs` constants give 2 (`EXIT_MALFORMED`), 3 (`EXIT_REFUSED`), 5 (`EXIT_WORKER`), 124 (`EXIT_TIMEOUT`), 126 (`EXIT_UNEXECUTABLE`), 127 (`EXIT_NOT_FOUND`); `cancellation.rs` gives `128 + signal` for the three handled signals (SIGINT 2, SIGTERM 15, SIGHUP 1), i.e. 130, 143, 129; chapter 6's status table and the CLI `after_help` give 1 for a child ending without a result. Every meaning in the eight-line box matches chapter 6's table. None changed.
- 2026-10-17 — `settled-exit-status` is ported to `settled-exit-status.typ` (47 lines, 10 of them the key), with the regenerated `settled-exit-status.svg` (208 KB) and `settled-exit-status.layout.json`. `settled-exit-status.puml` is removed. No module changed: `_state.typ`, `_layout.typ`, `_draw.typ` and `_style.typ` are untouched and the other eight SVGs and records are byte-identical (`task docs:figures:update` rewrote only this figure). This was the last `.puml` figure; retiring the renderer is `book-figure-retire-plantuml-k117`.
- 2026-10-17 — Layout cost, as numbers: 9 states, 1 initial marker, 9 transitions (8 labelled). Coordinates written: 0. Sides named by hand: 2 of 9 transitions (the "align" ones; 7 took the facing sides); `jog` 0. Label placement chosen: 0 of 8 (`seg`, `side`, `t`, `dx`, `dy` and wrapping widths all 0). Other numbers chosen: 1 `col-gap` (190), 1 row gap that differs from the rest (72, under the cleanup step, for the two-line green guard), 3 `limit` (320, on the outcomes wider than the default 240), 0 `min-w`. Renders to settle: 2 (the first was structurally right; the second widened the one row gap). Minutes of work: not clocked; about 15, an estimate, most of it reading `_state.typ`, the old source and the exit-code sources; no module code was written.
- 2026-10-17 — Was the k108 table right? It estimated **Medium** ("long boxes drive the width; reuses `_state.typ`"). The actual is **Low**: no module change, no label placement, two named sides, one gap each of the two kinds, three wrapping limits. The cost is below the estimate. The long boxes did not drive anything beyond a `limit`: the row grows to the box and "align" puts the line at its centre, which is what k115 predicted. The k115 clumsiness list held: a label's place is still only a segment, a side and a fraction (here never needed), and the one row gap that needed room is a number for a whole row.
- 2026-10-17 — Facts and key lines are preserved. A script (`python3 -I`, the one used for k115, extended to read the unbracketed `write or flush fails` and the `\n`-joined state names) compared the 9 state texts line by line, the title, the 8 labels and the 10 key lines of the old `.puml` (taken from the parent commit) with the texts in the new layout record: 41 strings, none missing. Controls seen to fail on a mutated copy of the record: a removed key line, an unbracketed label, a green label, a bracketed guard and an exit-code line, each reported by name. The "What each view means" row for `settled-exit-status` in `docs/book/figures.md` is unchanged: precedence not call order; cancellation, original failure, required cleanup, final write; cleanup attempted before reporting; an original failure keeps its status if cleanup fails. All of it is in the guards and the ten key lines.
- 2026-10-17 — The k122 control: changing `col-gap: 190` to `200` in `settled-exit-status.typ` makes `task docs:figures` fail on both `settled-exit-status.layout.json` and `settled-exit-status.svg`; restoring it passes.
- 2026-10-17 — `docs/book/figures.md` now lists `settled-exit-status` among the ported figures and says no `.puml` figure is left (the PlantUML sections stay for k117); it records the effort counts, the differences from the PlantUML output and the sizes (208 KB for this figure; the nine together 1.80 MB, all Typst, the largest 259 KB), and the single-engine-risk sentence no longer says one PlantUML figure remains. The k115 paragraph's size sentence, now stale, lost its totals.
- 2026-10-17 — `task docs:figures` (9 figures match), `task docs:figures:test` (passed), `task docs:book:build` and `task check` (339 of 339, no `.rs` file changed) pass.
