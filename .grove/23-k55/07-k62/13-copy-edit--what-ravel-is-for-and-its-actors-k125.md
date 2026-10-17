# what-ravel-is-for-and-its-actors-k125

## Goal

Make chapters 1 and 2 conform to the amended prose contract: ASD-STE100, the
word decisions in `docs/book/ste.md` and “send” for the result.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). Draft k124 ran
  before you and corrected the owner's teaching findings. Read its commit.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- “Publish” and its forms occur throughout both chapters: in headings,
  anchors, link text, tables and examples. Change each one that names the
  result concept to the `ste.md` wording. Chapter 3's heading “Publish the
  result and finish the job” changes in chapter 3's copy-edit (k126). Until
  then, keep links to it as they are. K126 changes every inbound link.
- Chapters 1 and 2 keep their titles. Change their mentions of other
  chapters' titles to the contract's new titles, for example “Write an owner
  policy” and “Run sessions”.
- R8: the paragraph before each figure repeats its colour key. Set art k130
  removes it. Do not edit figures or figure prose for R8.

## Done when

- Every sentence in chapters 1 and 2 follows STE: approved words or `ste.md`
  decisions, the sentence and paragraph limits, the active voice, simple
  tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays.
- No “publish”, “publication”, “submit” or “submission” for the result stays.
  A search with a control seen to fail shows this.
- Every local link and anchor in both chapters resolves. A control with a
  broken anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set art k130 and set proof k132 already stand for this
  document.
- A defect that needs a new draft makes a fourth run on this document. Stop
  and tell the owner.
