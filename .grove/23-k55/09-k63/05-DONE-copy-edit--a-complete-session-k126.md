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

- Every sentence in chapter 3 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/03-a-complete-session.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
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

## Decisions (running log)

- The heading “Publish the result and finish the job” becomes “Send the
  result and finish the job”. Its one inbound link is in chapter 5 (line
  163). The other five headings stay. They break no rule, and chapters 1, 4,
  5 and 6 link to their anchors.
- As in k125, this stage converts the words and sentences of the two alt
  texts and the two captions, because the strict check reads them. It keeps
  their facts. The colour paragraphs before the figures (R8) and every
  figure asset stay for set art k130.
- “The starting notice” becomes “the routing notice”, the name that
  chapters 5 and 6 and `ste.md` use for the `ravel: starting provider` line.
- “Lending the terminal” is not a kept “-ing” word. The text now says “Ravel
  lends the terminal”. Chapter 5's “[lend](…#launch-the-managed-job)” link
  still finds the definition.
- “Log in again”, “keep out of”, “take back” and “enter” (a directory) were
  phrasal or vague. The text uses “a new login”, “does not use”, “recovers …
  from” and “make it the current directory”.
- The prerequisite tools became a vertical list (Rule 4.3). Long paragraphs
  were split at a topic change, not shortened. No fact was removed.
- A conditional (“would otherwise succeed”) became “A cleanup failure prevents
  success, even when everything else succeeds” (Rule 3.2).
- One in-session reviewer compared the committed and the new chapter for
  meaning changes and prose-contract breaches the script cannot find. All
  14 findings were valid and fixed, except as noted. Lost or changed facts:
  “always runs” (the fake's `set -eu` can stop it first) is now the
  author's design, same-user code can also launch unrelated processes, a
  refusal is defined by “when” again, a new “only” claim about the build
  was removed, “confirmed” covers group absence again, and cleanup problems
  are inputs to the decision and optional (“any further”). The cleanup-
  failure sentence (“alone”) was already fixed. Wording: unclear pronouns
  (four), “remain before” attachment, “policy-context loader” as in chapter
  2, “pass” (not “send”) for the prompt, one verb for the result channel,
  and list punctuation. Visible trade-off: “routing notice” stays, because
  chapters 5 and 6 use it. The text now names it as the `ravel: starting`
  line, so the reader can see it.

## Verification

- `task docs:ste -- --strict docs/book/src/03-a-complete-session.md`: 0
  problems (115 before). `task docs:ste:test` passes. No “-ing” word needed
  a new `ste.md` decision. Each one was written again without it.
- Result words: a case-insensitive search for publish, publication, submit
  and submission finds nothing in chapter 3. The same search finds 25 in
  the committed chapter 3 and finds a planted “Submission”.
- Links: a scratch checker resolved every local link and anchor in all six
  chapters (406) against the built HTML ids. A planted link to the old
  anchor and a planted broken self anchor each failed it. The old anchor
  occurs nowhere outside `.grove/` task records.
- The script cannot check voice, tense or phrasal verbs. A search for their
  usual forms, seen to find four planted sentences, finds nothing in the
  final chapter. A manual read and the reviewer covered the rest.
- `task docs:book:build` passed. `task docs:examples` passed all 13
  complete examples (6 fragments not run), with the SHA-256 digests of
  chapters 3 and 5 the same before and after the run. `task docs:figures`
  matched all nine figures. No code block, example, figure asset or product
  source changed. This is not a reader-comprehension result.

## Handoff

Cut nothing: set art k130 and set proof k132 already stand. No chapter 3
draft work was found. The parent brief's Read 1 section carries the art
handoff (the figure labels that still say “publish”), because k130 does not
read this node's brief. This retirement leaves node k63 with no live leaf,
so k63 closes. No ADR or `CONTEXT.md` change: this stage changed wording
only.
