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
