# running-sessions-k128

## Goal

Make chapter 5 conform to the amended prose contract: ASD-STE100, the word
decisions in `docs/book/ste.md`, “send” for the result and the new chapter
title “Run sessions”.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). The owner read
  chapter 1, made ASD-STE100 the prose base for all documentation, chose
  “send” to replace “publish” for the result and accepted command-form
  titles. The owner did not read chapter 5 yet. The contract change causes
  this run. It is not a finding against chapter 5.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- The chapter's H1 becomes “Run sessions”. The file name stays. Change the
  chapter's mentions of other chapters' titles to the contract's new titles.
- Chapter 5 uses “publish” and “publication” more than any other chapter,
  also in complete examples: the kind `publish`, and prompts such as
  “Publish inner.” and “Publish confined.”. Change an example's input and
  its expected output together, and run `task docs:examples`.
- The exit-status table and the outcome rules say “even if one was
  published” and “after a result was published”. Keep the meaning exact:
  cancellation stops a result that Ravel already received.
- Set art k130 owns the figure labels, keys, captions and alt text, and the
  colour text before each figure (R8). Do not edit figures.

## Done when

- Every sentence in chapter 5 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/05-running-sessions.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in chapter 5. A search with a control seen to fail shows this.
- Every local link and anchor in chapter 5 resolves. A control with a broken
  anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set art k130 and set proof k132 already stand for this
  document.
- If chapter 5 does not fit one session, decompose this leaf along its
  sections.
