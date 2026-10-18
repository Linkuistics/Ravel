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

## Plan

Cut on 2026-10-17 as ten leaves inserted ahead of `book-read-one-k70`, in order:
`book-figure-engine-k109`, `book-figure-sketch-style-k110`,
`book-figure-sequences-k111`, `book-figure-c4-context-container-k112`,
`book-figure-source-components-k113`, `book-figure-installed-files-k114`,
`book-figure-session-outcomes-k115`, `book-figure-settled-exit-status-k116`,
`book-figure-retire-plantuml-k117`, `book-figure-viewing-check-k118`.

**Why this order.** Every handoff leaves `task docs:figures` green. The engine
leaf adds Typst beside PlantUML, and the stem rule (one source per name) lets a
port replace a figure's `.puml` with its `.typ` in one commit. The retirement
leaf runs only when no `.puml` remains. That is expand, migrate, contract,
because the contract, mechanics and nine committed SVGs cannot flip at once and
stay checkable. The sketch decision comes before any port so no figure is drawn
twice. The ports run from proven notation (sequence, then C4) to unproven
(component, deployment, state), and the two state views come last because they
are the densest layouts. Stopping after any port leaves a working book.

**Decisions.**

- *Text in the SVG.* Accept Typst's glyph-path SVG: no selectable or searchable
  text in the figure, and files 5–7× larger. The contract never promised
  selectable figure text, the page's alt text, caption and surrounding prose are
  the text alternative, and `file.svg` is a full-size link, not a text source.
  Budget, checked by `book-figure-viewing-check`: each SVG at most 300 KB and
  the nine together at most 2 MB (the PlantUML set is about 240 KB). Exceeding
  it prompts an investigation (glyph reuse, label length), not a blocked leaf.
- *Two editable sources.* The ban stays and extends to `.typ`: one authoritative
  `.typ` per figure, no `.puml` or `.d2` beside it. No PlantUML-to-Typst
  converter is built; the trial found the Smetana SVG exposes boxes and Béziers
  but not label roles, and such a converter keeps PlantUML as a second layer.
  Layout is computed from measured text or placed by hand in the one source.
- *`_*.puml` includes.* They become `_*.typ` modules in the same flat
  directory (`_style.typ`, `_sequence.typ`, `_c4.typ`, `_state.typ`,
  `_deployment.typ`). The underscore rule is unchanged: a module is not a figure
  and has no SVG. d2 has no book figure (the fixture set only), so its mechanics
  and the `sketch: false` rule leave with PlantUML.
- *Pins.* Typst and the CeTZ package are fetched by `task setup` with SHA-256
  checks into `target/tools`, like d2 and the fonts, and renders use that
  package cache offline. Vendoring into the repo is rejected as the default
  because nothing else the book needs is vendored; the engine leaf checks the
  CeTZ licence and records the dated decision. Typst 0.15.1 (the installed version) and CeTZ 0.4.2 (the version the
  k107 trial used) are the starting pins, not rechecked as current; the leaf
  rechecks them and records the date. The Java requirement and
  the Oblique-face pin question go away with PlantUML; italics are the engine
  leaf's decision.
- *Hand-drawn style.* The owner asked for it on 2026-10-17 (CeTZ has none, so it
  is a deterministic jitter in `_style.typ`). It is a prototype leaf whose
  deliverable is the owner's reaction, with roughness 0 equal to the crisp
  figure. The engine routes all strokes through shared primitives so the effect
  changes in one place.
- *No review chain.* The figures are checked by exact regeneration, each leaf's
  Done-when lists differences, the viewing check follows, and `ravel-book-k101`
  (whole-book art) and the book's final proof read all figures after.

**Layout cost, estimated before it is spent.** The owner asked to see this up
front. These are structural counts from the current sources and a relative rank
(Low, Medium, High) set against the trial, where the container view's added
entity took five lines of entity, one hand-routed edge, a legend offset and two
label retunes. They are estimates; each leaf records the actual figures and says
whether this table held. If `session-outcomes` runs well over, its leaf stops
and the owner decides before `settled-exit-status` is spent.

| Leaf | View | Entities | Relations | Hand-routing and new code | Estimate |
|---|---|---|---|---|---|
| `...-c4-context-container` | system context (+ promoted container) | 4 | 5, labelled | reuse of the trial's helpers; extract `_c4.typ` | Low |
| `...-source-components` | component | 4 + one partial boundary | 3 | boundary around some entities only | Low to Medium |
| `...-installed-files` | deployment | 5 artifacts, 3 enclosures | 1 dashed | nested enclosures with tabs; no edge routing | Medium (new primitive, little routing) |
| `...-session-outcomes` | state | 9 | 11, long guard labels, 2 green | fan-out to 4 outcomes, label placement | High |
| `...-settled-exit-status` | state | 9, one box with 8 lines | 8, 2 green | long boxes drive the width; reuses `_state.typ` | Medium |

**Not in this plan.** Chapter 6's figure for packages and the Part III and IV
figures are drawn later by their own art stages on this renderer; the contract
change in `book-figure-retire-plantuml` is what makes that the rule.

## Decisions (running log)

2026-10-17 — Owner asked, mid-session, for a hand-drawn style ("would be very
helpful in making the document more 'friendly'"). This answers the Notes
question, so the jitter helper is now a leaf (`book-figure-sketch-style-k110`),
placed before the ports.

2026-10-17 — Plan cut as ten leaves ahead of `book-read-one-k70` (see `## Plan`);
accepted glyph-path SVGs with a size budget; kept and extended the one-source rule;
`_*.puml` becomes `_*.typ`; Typst and CeTZ fetched by `task setup`, not vendored.
No review chain cut.
