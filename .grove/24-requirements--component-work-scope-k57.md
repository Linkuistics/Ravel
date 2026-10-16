# component-work-scope-k57

## Goal

With the human, settle the scope of the component work the root still
requires, using what the owner's two stand-in reads of the book found hard in
the code.

## Context

- The root brief: the component-boundary, extraction and delivery goals; k4's
  comparison, which retained the existing implementation and left its
  comparison incomplete (unread alternatives, six-month support cost, delivery
  and use beyond entry, target execution, runtime and lifecycle debt); k6's
  separate observation; and k22's closure, which left these as scope questions
  for a requirements session with the human.
- The book contract `docs/book/contract.md`, the book's chapters 7–9 (which by
  then explain the current components), and the recorded results of read 1 and
  read 2 inside `ravel-book-k55`.
- `book-structure-k5` placed this leaf after the book by the human's decision
  of 2026-10-16: draft the current code first, so that this step can use where
  a reader got lost.

## Done when

The human's answers are recorded verbatim, with what each means, on:

- whether to resume a comparative redesign, and with which frame, or to keep
  the current arrangement;
- which component, if any, becomes independently usable by other projects,
  with its interface, consumer and packaging, and the book's chapter 10 as part
  of that work;
- whether any consumer needs a persistent-runner (returning host) contract;
- delivery of this workstream's corrected code: merging to `main`, the next
  release, and the book's first publication with it.

Each answer that implies work gets its next precise leaf. Nothing is narrowed.

## Notes

The root's standing budget direction applies: budget does not gate required
work, and zero production disruption still binds. If the human wants the
repairs released before the book is finished, that delivery question can move
ahead of the book node by `leaf-insert`; it is the human's call.
