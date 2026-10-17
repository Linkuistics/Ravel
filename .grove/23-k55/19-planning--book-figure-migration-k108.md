# book-figure-migration-k108

## Goal

Plan the migration of every book figure to a shared Typst/CeTZ renderer, as the
owner decided on 2026-10-17 after the `book-figure-renderer-k107` trial: "migrate
full". Cut the implementation leaves; do not migrate in this session.

## Context

- Trial and evidence: `docs/book/trial-typst/` (README.md records pins,
  measurements, regeneration, effort and caveats; `style.typ`, `seq.typ` and the
  two figure sources are the starting point). Decision record: the Decisions log
  of `book-figure-renderer-k107`.
- Current contract and mechanics to rewrite: `docs/book/contract.md` ("Format and
  media"), `docs/book/figures.md`, `scripts/figures.sh`, the `docs:figures*` and
  `setup` tasks in `Taskfile.yml`, the nine sources in `docs/book/src/figures/`.
- The trial covered one sequence and one C4 container view only. The state
  (`session-outcomes`, `settled-exit-status`), deployment (`installed-files`),
  component (`source-components`), context (`system-context`) and the two
  cancellation sequences are unproven.

## Done when

- Leaves exist, ordered, for: the shared style/engine promoted from the trial
  (decide what is vendored, how CeTZ and Typst are pinned and checksummed, whether
  offline builds work); porting each remaining notation; replacing the PlantUML/d2
  mechanics, regeneration check and `figures.md`/contract rules in one coherent
  change; and a viewing check of the migrated figures at chapter width.
- The plan decides what happens to the text-as-glyph-paths SVG (selectable text,
  size 5-7x larger), the ban on two editable sources, and the `_*.puml` includes.
- The plan includes a leaf for each state/deployment/component view's layout cost
  (hand-placed geometry) so the owner can see the effort before it is spent.

## Notes

Owner asked whether CeTZ has a hand-drawn ("sketch") style. Checked: CeTZ 0.4.2
has no sketch/roughen mode (the only path decorations are zigzag, wave, coil and
brace). d2 had `sketch: true` as an option, which the book sets to false. A
deterministic jittered-path helper could be written in `style.typ`, but that is
unrequested; ask the owner whether they want it before spending a leaf.
Preserve the facts and notation semantics in `figures.md` "What each view means".
