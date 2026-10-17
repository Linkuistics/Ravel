# writing-an-owner-policy-k127

## Goal

Make chapter 4 conform to the amended prose contract: ASD-STE100, the word
decisions in `docs/book/ste.md`, “send” for the result and the new chapter
title “Write an owner policy”.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). The owner read
  chapter 1, made ASD-STE100 the prose base for all documentation, chose
  “send” to replace “publish” for the result and accepted command-form
  titles. The owner did not read chapter 4 yet. The contract change causes
  this run. It is not a finding against chapter 4.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- The chapter's H1 becomes “Write an owner policy”. The file name stays.
  Change the chapter's mentions of other chapters' titles to the contract's
  new titles.
- The routes to Claude Code and Codex show real flags and the ending
  instructions of a caller's prompt. Keep the flags and code exact. A prompt
  is example text, so it follows STE where it is prose.
- The complete examples run through `task docs:examples`. If an example
  changes, change its expected output with it.
- Chapter 4 has no figures. Thus, set art k130 has no work in this chapter.

## Done when

- Every sentence in chapter 4 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/04-writing-an-owner-policy.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in chapter 4. A search with a control seen to fail shows this.
- Every local link and anchor in chapter 4 resolves. A control with a broken
  anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set proof k132 already stands for this document.
- Chapter 4 is long. If it does not fit one session, decompose this leaf
  along the chapter's two parts (A: policy anatomy and fake-agent
  validation, B: context, settings, grants, module resolution, real routes
  and copying).
