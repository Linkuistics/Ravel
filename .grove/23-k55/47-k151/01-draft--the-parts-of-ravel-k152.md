# the-parts-of-ravel-k152

## Goal

Draft chapter 7 as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/07-the-parts-of-ravel.md`, linked
from `docs/book/src/SUMMARY.md` under the new title “The parts of Ravel”.

## Context

- Chapters 1–6 are proved and the first half is closed. Read chapters 1 and 2
  first: this chapter uses their terms, the authority statement and the
  container figure, and it must agree with chapter 1's statement of the loose
  contract.
- `docs/book/ste.md` and `task docs:ste -- --strict` apply from the first
  sentence. Runs of prose have 450 words or fewer (`task docs:breaks`); a table
  of the parts and their responsibilities is the natural break. Use “Ravel
  CLI”, “send”, “unchanged” and “return no result to the caller” as the shared
  decisions say.
- Make the title change in all the places that the node brief lists. Search
  with a pattern that you first see find a known occurrence (chapter 1's route
  table has one).
- Read the node brief's “Sources” bullet. The guides are sources that may be
  stale; verify each claim against the code at the checkout.

## Done when

- The chapter exists, builds (`task docs:book:build`) and meets the node
  brief's “Done when” for the draft stage. It passes the strict STE and
  prose-break checks, and `task docs:examples` still passes.
- The log lists each obligation O1, O2 and O10 and the section that teaches it,
  and for O3–O9 the chapter (8 or 9) that is to teach it.
- The log lists the destination of each sentence group of
  `docs/components.md` and `docs/local-runner.md` that this chapter covers, and
  of each relation R1–R8.
- Last act: `leaf-add` the `copy-edit` leaf under this node with the bare stem
  `the-parts-of-ravel`, and a body that carries what this draft learned. Cut
  nothing if a live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claim that is hardest to check: why the worker is a separate process that is
  reaped before the launch.
- Forward mentions of chapters 8, 9, 11 and 12 are plain text until those
  targets exist. The second half's set proof (`book-second-half`) turns them
  into links.
