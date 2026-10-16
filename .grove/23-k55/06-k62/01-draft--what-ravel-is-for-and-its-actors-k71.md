# what-ravel-is-for-and-its-actors-k71

## Goal

Draft chapters 1 and 2 of the book as the document node's brief describes,
under `docs/book/contract.md` (the structure brief is its section 1; the prose
contract its section 2). Write `docs/book/src/01-what-ravel-is-for.md` and
`docs/book/src/02-actors-and-words.md`, link them from
`docs/book/src/SUMMARY.md`, and leave the book building.

## Context

- The scaffold (`book-toolchain-k58`, `book-figures-k59`, `book-examples-k60`)
  has landed, so `task docs:book` builds. Verify every claim about Ravel
  against the spec and source, not against `docs/session-cli.md` or
  `docs/authority.md`, which this book replaces.
- Report the contract's omissions that bear on these chapters (the reader
  checkpoints are the owner's stand-in reads; chapter 10 is unwritten; the
  support statement) in the form the chapters carry them.

## Done when

- Both chapters exist and meet the node brief's "Done when" for the draft
  stage: the reader outcomes are delivered, every term is defined where first
  needed, no placeholder text remains, and the book builds with
  `task docs:book`.
- The log lists any claim whose guide wording disagreed with the code, so the
  guide fold-in can reconcile it.
- Last act: `leaf-add` the `copy-edit` leaf under the family rule, naming in its
  body anything the draft left for wording that is really a teaching gap.
