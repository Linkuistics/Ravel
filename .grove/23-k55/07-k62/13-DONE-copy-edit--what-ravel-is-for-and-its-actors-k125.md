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

- Every sentence in chapters 1 and 2 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/01-what-ravel-is-for.md docs/book/src/02-actors-and-words.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
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

## Decisions (running log)

- Every heading in both chapters stays. None breaks a rule, and chapters
  3–6 link to five chapter 2 anchors and one chapter 1 anchor. Thus, no
  inbound link changes.
- The check keeps image alt text as prose (`scripts/ste.ts`, line 61). Thus,
  this stage converts the words and sentences of both alt texts and of the
  prose before each figure. It keeps their content, the colour key (R8, set
  art k130) and every figure asset unchanged.
- “Ending instructions” and bare “routing” are not in the kept “-ing” list.
  Chapters 3–6 also use them. `ste.md` gets one replacement for each, so
  that k126–k129 use the same words: “the instructions to end the session”,
  and “the choice of command” or a verb such as “selects”.
- “Stand in for”, “clean up”, “put together” and “move out of” are phrasal
  verbs (Rule 9.3). Use “take the place of”, “complete the cleanup”, “use
  together” and “leave”.
- The heading “Choose a reading route” became “Choose a route through the
  book”. “Reading route” is not a contract name, and no tracked file links
  to its anchor.
- One in-session reviewer compared the committed and the new text of both
  chapters for meaning changes. It found no new technical error, no
  term-rule breach and no lost table row or caveat. Valid and fixed: SIGKILL
  narrowed from an example to the only case, “Chapter 10 does not exist”
  (SUMMARY lists it), the noun phrase that made `end-session` end the
  session, a causal “Thus” on the x86-64 status, a ranking and a certainty
  that the old text did not claim, `RESULT` as a placeholder, the
  kind-to-arguments link, three scope shifts, the lost “interpret the
  outcome” clause, “normal” for both paths, two alt-text losses, and “the
  results of the owner” (a misuse of “result” from the draft). Unclear
  contract, no change: the new chapter titles in link text differ from the
  H1s of chapters 4 and 5 until k127 and k128 change them. Noise: the
  confinement wording, which the reviewer confirmed against
  `crates/ravel/src/cli.rs`.

## Verification

- `task docs:ste -- --strict` on both chapters: 0 problems (170 before).
  `task docs:ste:test` passes. A planted “ending instructions” and
  “routing code” still fail. Thus, the new `ste.md` table keeps no word.
- Result words: a case-insensitive search for publish, publication, submit
  and submission finds nothing in either chapter. The same search finds 13
  in the committed chapter 1, finds a planted “Publishes”, and finds 25 in
  chapter 3 (copy-edit k126).
- Links: a scratch checker resolved every local link and anchor in all six
  chapters (406) against the built HTML ids. A planted broken cross-chapter
  anchor, a planted broken self anchor and the old heading's anchor each
  failed it.
- `task docs:book:build` passed without warnings. `task docs:examples`
  passed all 13 complete examples. Six fragments did not run. The SHA-256
  digests of the chapters, `ste.md` and `scripts/ste.ts` were the same
  before and after the final run.
- The script cannot check voice, tense or phrasal verbs. A search for their
  usual forms, seen to find a planted sentence, found only adjectival “is
  qualified” and “is trusted”. A manual read covered the rest.
- No figure asset, example or product source changed. This is not a
  reader-comprehension result.

## Handoff

Cut nothing: set art k130 and set proof k132 already stand. The parent
brief's Read 1 section now carries the art handoff, because k130 does not
read this node's brief. This retirement leaves node k62 with no live leaf,
so k62 closes. Its third correction run completes through k130 and k132 in
the parent node. No ADR or `CONTEXT.md` change: this stage changed wording
only.
