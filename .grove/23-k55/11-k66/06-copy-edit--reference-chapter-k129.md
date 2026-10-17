# reference-chapter-k129

## Goal

Make chapter 6 conform to the amended prose contract: ASD-STE100, the word
decisions in `docs/book/ste.md` and “send” for the result.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). The owner read
  chapter 1, made ASD-STE100 the prose base for all documentation and chose
  “send” to replace “publish” for the result. The owner did not read chapter
  6 yet. The contract change causes this run. It is not a finding against
  chapter 6.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- Chapter 6 is the lookup chapter. Each entry is a direct entry, so it keeps
  its own context after the change. Entries and tables are text, so they
  follow STE too. Keep code identifiers, codes, statuses and paths exact.
- The `end-session RESULT` entry describes the command. K131 changes the
  command's help text to “send” after you. Describe the behaviour, not the
  help wording.
- Change the chapter's mentions of other chapters' titles to the contract's
  new titles. Links to the renamed heading of chapter 3 change in k126.
- Set art k130 owns the figure labels, keys, captions and alt text, and the
  colour text before each figure (R8). Do not edit figures.

## Done when

- Every sentence in chapter 6 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/06-reference.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in chapter 6. A search with a control seen to fail shows this.
- Every local link and anchor in chapter 6 resolves. A control with a broken
  anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set art k130 and set proof k132 already stand for this
  document.
- Chapter 6 is the longest chapter. If it does not fit one session,
  decompose this leaf along the chapter's own inventories: the command line,
  exit statuses, streams and environment (inventory 1), then policy,
  settings, bounds, SDK and installed layout (inventory 2).
