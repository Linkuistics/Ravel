# ravel-book-k201

## Goal

Run the art stage on the eleven-chapter book after the copy-edit. Check that
each figure, its alt text, caption and key agree with the book's words.
Check the visual consistency that the contract requires across all fourteen
figures.

## Context

- The node brief `ravel-book-without-chapter-ten-k196` gives the owner's
  decision and the limits of the work. `docs/book/contract.md` gives the
  figure needs, the figure style and the formats. `docs/book/figures.md`
  gives the figure convention and the regeneration recipe.
- The draft `ravel-book-k197` removed the planned component chapter. No
  figure names chapters 10–12. The copy-edit `ravel-book-k200` searched the
  figure sources and layout records for “chapter” and “component”.
- The book now says that Ravel supports no separate component. The C4
  component view `source-components` uses “component” in its C4 sense. Its
  key says “Dashed outline: components inside the Ravel CLI, not a sandbox.”
  Decide whether that key needs words that separate C4 components from a
  separately supported component. Keep the C4 notation term.
- The copy-edit changed no figure source, alt text or caption. In the
  prose, the copy-edit used one verb and one name for the caller
  interface: another program “invokes the Ravel CLI as a child process”. If
  figure text names that interface, use the same words.

## Done when

- Each figure and its text agree with the copy-edited prose and with
  `CONTEXT.md`. Each change stays within the art charter. Later-stage
  findings are in the node's Handed forward list.
- `task docs:figures` passes. If a source changes, `task docs:figures:update`
  regenerates the SVG and layout record, and the comparison passes again.
- `task docs:book:build`, `docs:ste -- --strict` and
  `docs:breaks -- --strict` pass.
- The last act adds a proof leaf under this node, with the bare stem
  `ravel-book`, unless a later live proof leaf already exists.

## Notes

- The art stage is mandatory, also when it changes nothing.
- The node brief's Handed forward list has one entry for proof. It is not
  an art item.
- A local render and build establish no view on GitHub or GitHub Pages.

## Decisions (running log)

- Keep the C4 term `component`. Clarify its key, caption and
  alt text: C4 components here are internal code. Ravel supports no separate
  component. A reader can open the SVG alone, so its key needs this limit.
  The figure keeps the same participants, relations and process boundary.
- Keep the other thirteen figures. The whole-book read found their roles,
  captions, alt text and keys consistent with the current prose. The three
  C4 views distinguish system, process and internal source responsibilities.
  The sequences distinguish selection, supervision, cleanup and delivery.
  The state and activity views distinguish phases, status and precedence.
  The timing, terminal, installation and release views retain their stated
  limits. No figure presents a supported runner component.
- Decline extra diagrams for nested sessions, environment selection, test
  seams, installation steps and Pages setup. This is an editorial judgement:
  existing figures, tables and numbered steps already carry those relations.
  Neither the contract nor the medium prevents a useful diagram.
- Keep the existing proof handoff about the chapter 11 paragraph's position.
  This art pass found no additional defect outside its charter.

## Verification

- Read all eleven chapters, the contents and each figure source. Compare
  their relationships and terminology with the contract and `CONTEXT.md`.
  Graph coverage excludes generated SVG files and has older document metadata.
  Current file reads and rendered images supplied the evidence instead.
- Inspect all fourteen exported SVGs at 750 pixels wide with `rsvg-convert`.
  Inspect the two longest views in sections at that width. Labels, arrowheads,
  boundaries, colours and keys remain legible, without clipping or collisions.
- Inspect the changed chapter in Chrome from the local book build. Check
  its caption, figure key and full-size link. The link opens the revised SVG.
  This establishes no view on GitHub or GitHub Pages.
- The isolated macOS VM failed to start because local-network access was
  denied. Its temporary clone was not kept. Chrome supplied the local check.
- `task docs:figures:update` regenerated only the changed figure's SVG and
  layout record. The renderer reports Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37.
- The first strict STE check found seven sentences in the changed alt text.
  Combining its two new sentences fixed that finding. The next check passed.
- Final checks passed: `task docs:figures`, `task docs:book:build`,
  `task docs:ste -- --strict` and `task docs:breaks -- --strict`.
  The figure check matched all fourteen sources. STE found no problems in
  28 files. The breaks check found no long prose runs in eleven chapters.
- SHA-256 lists before and after those checks matched for all 3,537 tracked
  files outside `.grove/`. These include the book, manifests, scripts and
  fixtures. No validation input changed during the final run.
- The personal prose checker found no hard findings in chapter 6. Its
  eighteen advisory findings precede this change. The task record also passes
  that checker after a semicolon correction.
