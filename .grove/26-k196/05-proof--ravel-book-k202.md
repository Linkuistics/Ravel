# ravel-book-k202

## Goal

Proof the whole eleven-chapter book after the art stage. Check every class
in the document's contract and close the publishing pipeline for this node.

## Context

- The node brief `ravel-book-without-chapter-ten-k196` states the owner's
  decision: Ravel supports no separate component. Develop is chapter 10,
  Deliver is chapter 11, and Part IV covers development and delivery.
- Read the contract, the book's prose and figure conventions, `CONTEXT.md`
  and the whole book. The prior stages are `ravel-book-k197`,
  `release-script-chapter-names-k199`, `ravel-book-k200` and `ravel-book-k201`.
- Art checked all fourteen figures and changed only `source-components`.
  Its key, caption and alt text now explain that C4 components are internal
  code. Ravel supports no separate component. The C4 term remains.
  The SVG and layout record match the revised Typst source.
- Art inspected all SVGs at the book's 750-pixel column width. It also
  inspected the revised chapter and its full-size link in local Chrome.
  That evidence establishes no view on GitHub or GitHub Pages.
- The node's Handed forward list has a chapter 11 placement question for
  proof. Resolve it under the editorial family rules. A structural move
  belongs to a draft correction run.
- Read the log of `adr-native-waitability-status-k191` for any chapter 9
  finding sent to this whole-book proof.

## Done when

- The whole-book read covers the contract's requirements, technical truth,
  terms, links, chapter numbers, figure roles and prose rules. No finding
  remains unresolved or silently passed beyond proof.
- The node's Handed forward list is clear. If an earlier charter owns a
  defect, use the editorial family's ordered correction run.
- The node's required checks pass: `task docs:book:build`, `docs:examples`,
  `docs:excerpts`, `docs:figures`, `docs:ste -- --strict`,
  `docs:breaks -- --strict`, `release:lifecycle:test` and `check`.
- Resolve each remaining node acceptance condition before closing it.
  Retire and commit under the Grove procedure, including any parent cascade.

## Notes

- Art's final figure comparison, book build, strict STE and strict breaks
  checks passed with unchanged validation inputs. Its task records the
  visual inspection and declined figures.
- Proof normally adds no successor. Follow the family rules if a finding
  requires a correction run.
