# the-parts-of-ravel-k168

## Goal

Take `docs/book/src/07-the-parts-of-ravel.md` through the art stage under
`docs/book/contract.md` and `docs/book/figures.md`. Decide whether the
chapter needs a figure on its own page, and make the figure text, captions
and alt text follow the prose contract.

## Context

- The node brief's “Handed forward” list holds the art decision. Chapter 7
  links to the process-and-authority figure in chapter 2
  (`02-actors-and-words.md#the-managed-process-and-its-result`) and to the
  source and installed views in chapter 6
  (`06-reference.md#installed-layout`). It has no figure of its own. Seven
  tables carry the parts, owners, conditions and interfaces. Clear the
  handoff entry when you decide it.
- The contract makes every figure a direct entry. A figure that repeats a
  chapter 2 or chapter 6 view on this page needs the local context of this
  chapter in its caption and alt text. Reuse the proved layout record where
  the question is the same. A new figure is justified only by a reader
  question that the tables and the linked figures do not answer.
- Copy-edit `the-parts-of-ravel-k167` changed sentences and table cells
  only. No heading, anchor, section order, table row or code block changed.
  The chapter now says “managed process” throughout, as the rest of the book
  does. It links “managed process”, “refusal”, “confinement” and
  “persistent runner” at their first use. Figure labels follow the same
  words.
- The chapter passes the strict STE and prose-break checks. No prose run is
  above 450 words, so a figure is not needed as a break.
- Small text in the dense shared figures belongs to whole-book art
  `ravel-book-k101`, not to this leaf.

## Done when

- The art handoff in the node brief is decided and cleared. If a figure
  changed or is new, `task docs:figures` passes and `docs/book/figures.md`
  records the dated view.
- Strict STE, prose-break checks, book build and complete examples pass.
- Last act: add the `proof` leaf under this node, with the bare stem
  `the-parts-of-ravel`, unless a live later sibling already holds that stage.
