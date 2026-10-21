# ravel-book-k205


## Goal

Read the whole book under the art charter after its placement correction.



## Context

Read `ravel-book-k202` for the two findings and `ravel-book-k203` for their
correction. Chapter 11's component-policy paragraph moves to its introduction.
Check figure roles, text alternatives and page layout against the contract
and `docs/book/figures.md`. The prior art leaf, `ravel-book-k201`, records
the fourteen-figure inspection and the internal-component clarification.

## Done when

- The whole-book art read covers the changed placement and figure roles.
  Any figure change has matching source, SVG and layout record.
- The book build, examples, excerpts, figure comparison, strict STE and
  strict prose-break checks pass. State which viewing environments were available.
- Keep the queued proof leaf. Add no duplicate successor.

## Notes

This is a correction run. The k202 log records the workspace-check scheduling
limit. No new platform or public-site evidence follows from a local view.

Copy-edit `ravel-book-k204` changed words only, in chapters 6, 7, 9, 10
and 11 and the contract. Chapter 11's second paragraph now links “runner”
at its first use. No figure, caption, key or alt text changed.

## Decisions (running log)

- Read the contents, all eleven chapters, the contract and glossary, and all
  fourteen figure sources. Keep the figures, captions, keys and text alternatives.
  Their relations still agree with the corrected prose.
- Chapter 11 now places the component limit beside its opening account of the
  delivered pair. Its release activity still explains release order and recovery.
  The paragraph needs no separate figure.
- Decline extra diagrams for policy choices, nested results, test seams,
  installation and Pages setup. This is an editorial judgement: existing
  tables, steps and linked figures already carry those relations.
  Neither the contract nor the medium prevents such diagrams.
- Keep the queued proof leaf `ravel-book-k206`. Add no duplicate successor.
  The parent stays open for that proof. No new finding needs a handoff.
- TestAnyware's macOS clone could not reach its agent because local-network
  access was denied. The failed start removed its clone, `ravel-k205`.
  Use local browser and SVG views, and state their scope.
- Inspected all fourteen SVGs as 750-pixel-wide renders. Labels, arrows,
  boundaries and keys remain legible without clipping or overlap. Inspected
  the long code-path and release figures in sections at that same width.
  Local Chrome showed chapter 11's corrected introduction, the release
  activity's role paragraph and key, and its working full-size SVG link.
  Raw Markdown, local mdBook and local SVG views were available. This run
  adds no editor-preview, GitHub or public Pages viewing evidence.
- Graph verification used Tier 2 in project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T14:48:50Z`. Coverage records no gap for the queried non-SVG
  paths, but freshness records changed Markdown. The graph excludes SVGs.
  Direct file reads and rendered views supply the evidence for this art read.
  A clean coverage result does not establish completeness.
- Final validation passed on 2026-10-21. `task docs:book:build` passed.
  `task docs:examples` passed all 14 examples, with none failed or skipped.
  `task docs:excerpts` passed all 8 excerpts. `task docs:figures` found 14 matches.
  `task docs:ste -- --strict` found no problems in 28 files.
  `task docs:breaks -- --strict` found no long prose runs in 11 chapters.
  The figure check used Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37.
- SHA-256 comparisons found all 4,641 recorded inputs unchanged across
  validation. They cover tracked files outside `.grove/`, checkout tools
  and assets, worker dependencies and the personal prose checker.
  Scratch manifests, logs and figure renders are under `/tmp/ravel-k205-*`.
  No book, figure, product, test, specification or ADR changed in this leaf.
