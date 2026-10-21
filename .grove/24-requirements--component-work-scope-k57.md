# component-work-scope-k57

## Goal

Settle with the human the scope of the component work that the root still
requires. Use the places in the code that the owner's two stand-in reads of
the book found hard.

## Context

- The root brief holds the component-boundary, extraction and delivery goals.
  It also holds three records:
  - K4's comparison retained the existing implementation and left its
    comparison incomplete. Its open items are unread alternatives, six-month
    support cost, delivery and use beyond entry, target execution, and runtime
    and lifecycle debt.
  - K6 made a separate observation.
  - K22 closed and left these items as scope questions for a requirements
    session with the human.
- The book contract `docs/book/contract.md`, the book's chapters 7–9 and the
  recorded results of read 1 and read 2 inside `ravel-book-k55`. By then,
  chapters 7–9 explain the current components.
- `book-structure-k5` placed this leaf after the book, by the human's decision
  of 2026-10-16. The decision was to draft the book against the current code
  first. Thus, this step can use the places where a reader got lost.

## Done when

This leaf records the human's answers verbatim, with the meaning of each, on
these questions:

- Whether to resume a comparative redesign, and with which frame, or to keep
  the current arrangement.
- Which component, if any, becomes independently usable by other projects.
  The answer includes its interface, consumer and packaging. The book's
  chapter 10 is part of that work.
- Whether any consumer needs a persistent-runner (returning host) contract.
- The delivery of this workstream's corrected code: the merge to `main`, the
  next release, and the book's first publication with that release.

Each answer that implies work gets its next precise leaf. This leaf narrows no
requirement.

## Notes

The root's standing budget direction applies. Budget does not gate required
work, and zero production disruption still binds. If the human wants the
corrections released before the book is complete, the delivery question can
move ahead of the book node by `leaf-insert`. That is the human's decision.
