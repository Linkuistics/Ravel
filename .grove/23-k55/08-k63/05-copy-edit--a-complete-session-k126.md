# a-complete-session-k126

## Goal

Make chapter 3 conform to the amended prose contract: ASD-STE100, the word
decisions in `docs/book/ste.md` and “send” for the result.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). The owner read
  chapter 1, made ASD-STE100 the prose base for all documentation and chose
  “send” to replace “publish” for the result. The owner did not read chapter
  3 yet. The contract change causes this run. It is not a finding against
  chapter 3.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- The heading “Publish the result and finish the job” and its anchor
  `#publish-the-result-and-finish-the-job` change. Other chapters link to
  it. Change every inbound link in `docs/book/src/`, the README and `docs/`
  in this session.
- The complete examples run through `task docs:examples`. If an example's
  kind or prompt uses “publish”, change it and its expected output together.
- Set art k130 owns the figure labels, keys, captions and alt text, and the
  colour text before each figure (R8). Do not edit figures.

## Done when

- Every sentence in chapter 3 follows STE: approved words or `ste.md`
  decisions, the sentence and paragraph limits, the active voice, simple
  tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in chapter 3. A search with a control seen to fail shows this.
- Every link to the renamed heading resolves across the book. A control with
  a broken anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set art k130 and set proof k132 already stand for this
  document.
- If chapter 3 needs draft work, cut a draft run under this node as Grove's
  editorial reference directs, and say why.
