# deliver-ravel-k189


## Goal

Change the `release-delivery` activity figure of chapter 12
(`docs/book/src/figures/release-delivery.typ`) so that it shows the delivered
book steps. Keep the figure's conventions and regenerate its SVG and layout record.

## Context

- Read the draft's figure handoff in `deliver-ravel-k187` and the figure's entry in
  `docs/book/figures.md`. The figure has no Pages step today and its caption and
  entry say so.
- The order to show comes from `Taskfile.yml`: the book is built from the tag after
  the version guard and before the archive build; the book goes to Pages after the
  GitHub Release and before the tap; the Pages site is read last. A failed book
  build stops with the cut local. The new recovery exit is a Pages site that does
  not show the book. Check each position against the Taskfile and
  `scripts/release-publish.sh`.
- Keep the public-write boundary clear: the book build is before it, and the Pages
  push is after it. Keep the grey exits as manual recovery points, and change their
  count in the caption and in `figures.md`.
- Use `task docs:figures:update`, then `task docs:figures`, and view the result at
  the 750 px column. Record the file size and the set size. Whole-book art
  `ravel-book-k101` owns the set-size issue.

## Done when

- The figure, its caption in chapter 12, and its `figures.md` entry agree with the
  Taskfile and with each other. The figure check passes.
- The figure is legible at the column width.

## Decisions (running log)

- Retain the shared activity module and crisp style. The mandated correction
  and the book contract already fix the figure's design. No renderer, medium,
  shared module or architectural decision changes.
- Direct reads of the release Taskfile and scripts confirm the order: Pages
  remote preflight, tagged book build before archives, GitHub release, Pages
  branch push with `verify`, tap, installation, then `live`. The existing
  grouped-decision convention keeps eight recovery exits. A book-build failure
  joins local-cut recovery. A branch failure joins GitHub/book recovery.
- Keep the page's tables for the archive manifest, platform comparisons,
  task and mode mappings, and recovery. A second installation diagram or a
  separate Pages diagram would repeat those relations. These are editorial
  choices, not contract restrictions or limits of the medium.
- Proof k190 is already live after this correction leaf. The conditional
  successor rule therefore cuts no duplicate proof leaf.

### Verification and limits

The Tier 2 graph project was ready at generation `2026-10-18T12:47:51Z`.
Coverage matched metadata for the sixteen checked text/source paths with no
recorded gap. The SVG is excluded by suffix. Direct reads of the Taskfile,
scripts, Markdown and figure source supplied the evidence. Regeneration and
browser inspection supplied the excluded SVG's evidence. Graph coverage is
a best-effort signal, not a completeness proof.

The whole chapter was read for relations and figure roles. The changed
activity, caption, alternative, key, figure entry and third dated check row
now agree. The book node's art handoff is cleared. Its size handoff now
records 302,385 bytes for this SVG and 3,214,907 bytes for all fourteen.
Whole-book art k101 keeps that assessment. No other chapter-local defect
needs an earlier-stage correction or a later-stage handoff.

On 2026-10-18, `task docs:figures:update` changed only this figure's SVG and
layout record. All six required checks then passed: fourteen figures match,
the book build has no error or warning, fourteen examples pass with none
failed or skipped, and eight excerpts pass. Strict STE reports zero problems
in its 28 files. Strict prose breaks reports no long run in eleven chapters.
The 220 tracked source inputs from `docs/book`, `scripts`, `crates`, Cargo
manifests/lockfile, Taskfile, release configuration and `.cargo` have identical
SHA-256 lists before and after the completed checks. The local subject lists
and digests are in `target/k189-checks/`.

Chrome supplied a local light-page view at a measured 750 px column and
1227 × 919 viewport. All actions, eight exits, region boundaries and the key
were inspected. Labels and arrows do not overlap. Text is about 13.3 px at
column width. The full-size SVG opened directly after a link-click timeout,
and its upper region was inspected. The dated record in `docs/book/figures.md`
names the uninspected environments. No public delivery, new platform result
or reader-comprehension result follows from this work.

No renderer, shared module, product source, release script or ADR changed.
No in-session reviewer was needed or dispatched. The ordinary last act is
discharged by live proof k190. Retire this leaf and seal one focused change.
The book node still has live work, so no parent closure or cascade is due.

## Notes
