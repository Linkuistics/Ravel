# book-figure-c4-context-container-k112

## Goal

Replace the C4 system-context and container figures with Typst sources on a
shared C4 module, and remove the trial directory.

## Context

- Figures: `process-authority` (trial source exists, hand-placed, measured
  boxes) and `system-context`.
- The trial's container source holds its own entity, relation and label helpers.
  With a second C4 figure, extract them into `_c4.typ` (entity in a semantic
  role, boundary, relation with label and technology, key). The component view
  in `book-figure-source-components` is the third user; design for it without
  porting it here.
- Record design (`book-figure-layout-record-k120`, survey option B): `_c4.typ` returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
- Layout is manual. Edge routes and label positions were the cost in the trial
  (the added container took an entity, an edge, a legend offset and two retunes).
  Record the effort for `system-context` in the decision log with counts: edges
  routed by hand, label moves, renders to settle.
- Preserve `docs/book/figures.md` "What each view means" for both figures: the
  opaque-result path is green and separate from the worker response; worker end,
  reap and private-directory removal precede the managed launch; boundaries are
  not security boundaries.

## Done when

- Both `.puml` files are replaced by `.typ` with committed regenerated SVGs and
  `.layout.json` records, and `_c4.typ` exists as a reusable module used by both,
  producing records and drawing only through `_draw.typ`.
- Differences from the PlantUML output (orthogonal relations instead of splines,
  cylinder label inside) are listed with the reason, and every fact, tag, number
  and key line is present.
- `docs/book/trial-typst/` is removed after its findings are confirmed to be in
  `docs/book/figures.md` (pins, effort, the extraction test that was not
  adopted, the Typst markup trap `2\.`); the git history of the k107 commit keeps
  the trial.
- `task docs:figures`, `task docs:book:build` and `task check` pass.

## Decisions (running log)

- 2026-10-17 — `_c4.typ` is a layout module on the record design (k120): `c4(title, ents, bounds, rels, key)` returns a deferred layout for `_draw.typ`'s `render`, and nothing in it draws or imports CeTZ. Entities are placed by hand (`cx`, `top` or `cy-of`, `w`) and sized from measured text; a relation names two entities, a side and offset for each end, and the module routes a straight, L or Z orthogonal polyline and places the label beside a segment (`seg`, `t`, `side`, `dx`, `dy`, `w`). It does not choose positions or avoid collisions. Reason: the brief fixes layout as manual and the trial's cost was routes and label positions, so the module removes the entity, key and legend arithmetic and leaves the two placements to the source.
- 2026-10-17 — Additive style roles in `_style.typ`, documented in `_draw.typ`'s header: faces `name` (bold 16) and `qual` (12); kind `result` (green text for a result relation's label); stroke `boundary`. `_draw.typ` draws a boundary with 4pt dashes because CeTZ's `"dashed"` preset drew dots at 1.5pt (seen in the first render). The sequence figures' SVGs and records did not change (`task docs:figures` matched them after the edit).
- 2026-10-17 — Layout effort, system-context (counts as recorded in `docs/book/figures.md`): 4 entities, 5 relations, all 5 routes by hand (1 default sides, 4 explicit side and offset), label placement chosen for 3 of 5 (2 `side`, 2 wrapping widths), no `dx`/`dy`/`t`, 2 renders to settle. process-authority: 5 entities, a boundary, 7 relations, 7 routes by hand, label placement for 6 of 7 (2 `t`, 1 `seg`, 1 `dy`, 1 `dx`, 2 `side`, 5 widths), 3 renders. Scratch copy with the trial's extra `audit sink`: 1 entity (4 lines) + 1 relation (2 lines, `jog`) + 1 boundary member, first render usable.
- 2026-10-17 — Third user checked without porting it: a scratch sketch of `source-components` showed that a boundary around two of four entities works, and exposed two gaps, fixed in the module: a boundary narrower than its own name overflowed it, and a boundary header could sit above the title. k113 inherits both fixes; its own gaps (long relation labels that need a wider gap than the boxes leave) are placement, in its source.
- 2026-10-17 — Differences from the PlantUML C4 output, each with its reason, are in `docs/book/figures.md` ("Differences from the PlantUML C4 output"): orthogonal relations (no layout engine), bold entity names and titles (matches the sequence figures), typographic apostrophes, no person icon (C4 sprite; the `«person»` stereotype and the slate role say it), measured wrapping, sketched style. Every quoted string in the two old `.puml` sources (26 and 43, including titles, stereotypes via the layout text and all key lines) was found in the new layout records by script; with `standard output` removed from the haystack the same comparison reports it missing, so the comparison can fail.
- 2026-10-17 — The k122 control: moving `managed` 10 points in `system-context.typ` makes `task docs:figures` fail on both the record and the SVG; restoring it passes.
- 2026-10-17 — Trial findings confirmed in `docs/book/figures.md` before `docs/book/trial-typst/` was removed: the pins and reproducibility (already in the Typst section's table and paragraphs), the effort figures, the extraction/conversion test that was not adopted, the single-engine risk, paths-not-text and size (already there; sizes updated), and the markup trap. The trap's description changed: under the record design a label starting `2. ` is not drawn as a list, it stops the render (`_layout.typ`'s `flat` refuses an item), seen by removing the backslash from a scratch copy. One research table row naming a trial file got a note that the file is removed and kept in the k107 commit.
- 2026-10-17 — Final checks: `task docs:figures` (9 figures match), `task docs:figures:test` (exit 0), `task docs:book:build` and `task check` (339 of 339) pass. `task check` is not a stable instrument on this host while other sessions load it (load average about 8): of four full runs after the figure work, two passed and two failed on a different Rust test each (`worker::admission_tests::worker_reap_admission_success_gate_both_front`, `selection_timeout` where `worker_failed` was expected; `keyed-launch run::terminal_tests::terminal_recovery_reports_and_controls`, a missing signal 15). The change has no `.rs` file; the second test failed 2 of 3 times in isolation on unchanged code and passed the third. The recorded pass is the last, unedited run. These two timing-sensitive tests are not this leaf's to fix and are not routed anywhere; they are a flake worth a later look.
