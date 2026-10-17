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

## Decisions (running log)

- One session holds the whole chapter. Decomposition along parts A and B is
  not necessary: the chapter has about 600 lines of prose, and the code
  blocks do not change.
- The H1 becomes “Write an owner policy”. Nine of the ten section headings
  stay: each is a bare infinitive with no “-ing” word, and chapters 3, 5
  and 6 link to their anchors. The tenth is renamed (below).
- The link text for chapter 5 becomes “Run sessions” (`ste.md` titles). The
  contract's entry route “Running sessions from your own tool” is not a
  chapter title and is not this leaf's text.
- The agent example's prompt said “When done, run …”. That omits words.
  It becomes “When you finish, run ravel end-session -- "reviewed" as your
  last action. Then do nothing else.” Its two expected-output lines change
  with it. No other code, policy string or expected output changes.
- The heading “Copy an existing policy” had a verb form in “-ing”
  (“existing”). It becomes “Copy a policy from another configuration”: the
  section copies a policy from a source directory into
  `$HOME/.config/ravel`. Chapter 6 linked to the old anchor three times.
  Those links now use the new anchor, and the one link text that gave the
  old heading gives the new one. The contract's chapter purpose (“copying an
  existing policy”) describes the topic. It is not this leaf's text.
- No “-ing” word needed a new `ste.md` decision. Each one was written again
  without it, for example “pending” became “unsettled”, the word in the code
  `context_loader_unsettled`.
- The vertical lists replace three run-on enumerations (Rule 4.3): the six
  `limits` fields, the three forms of `program` and the problems that give
  `settings_invalid`, `choice_unreadable` and a read refusal. Long
  paragraphs were split at a topic change, not shortened. No fact was
  removed.
- The “Work” column of the evaluation table had imperatives for work that
  Ravel does. It now says “Ravel imports …”, “Ravel calls …”.
- One in-session reviewer compared the committed and the new chapter for
  meaning changes and prose-contract breaches that the script cannot find.
  It reported 48 items. Classification:
  - Valid and fixed, meaning: `schemaVersion` is not caller input (the one
    serious item); “that refusal” again covers both `readChoice` refusals;
    the policy accepts a number of names, a file does not limit it; the
    context-row conditions are an assumption again; the caller-document
    record counts toward the limit only if there is one; “separate” (not
    “different”) limit; no invented author for the flag check; the static
    module supplies an example to copy, not an editable policy; evaluation
    does not continue past the deadline; “not `context_loader_unsettled`”
    restores the contrast; “even if” for caught read exceptions; runtime
    validation covers the policy, not only the object; the object “can
    have” four fields; `limits` items are maximums; standard output “stays
    free” (no new “only”); the installed-execution sentence returns to its
    paragraph with “in usual operation”; checking with a separate TypeScript
    is an option again; `choice_unreadable` covers files that exist; the
    failed-read source; each repeated read; the over-limit text file; owner
    “default”; `<scratch>/work` “stands for”; repeated grants; “after the
    launch”; try-then-copy order; the context fields hold facts and
    judgements; “chosen agent”; the settings file path in step 6.
  - Valid and fixed, STE and contract: one instruction per sentence (four
    places), step 6 at six sentences, unclear phrases and pronouns (seven),
    “tells about” (three), wording (nine), “makes sure”, “Fixed at”, three
    paragraphs with two topics, the first-use links of “session” and
    “policy context”, and the three-sentence `context_invalid` cell.
  - Contract stated unclearly: the self-contradictory log entry on
    headings, now corrected above.
  - Noise: chapter 9 is plain text by the book's link convention for
    chapters 7–12, which the second half's proof turns into links.
  The fixes are local rewordings of the reviewer's own findings, and each
  passed the same strict check and examples. No second review was needed.

## Verification

- `task docs:ste -- --strict docs/book/src/04-writing-an-owner-policy.md`:
  0 problems (222 before). `task docs:ste:test` passes. No “-ing” word
  needed a new `ste.md` decision.
- Result words: a case-insensitive search for publish, publication, submit
  and submission finds nothing in chapter 4. The same search finds 6 in the
  committed chapter 4 and finds a planted “Submission”.
- Voice and tense: a search for passive, perfect, future and conditional
  forms finds four planted forms in a test copy. In the final chapter it
  finds only “is frozen data”, where “frozen” is an adjective. The reviewer
  found no tense problem.
- Links: a scratch checker resolved every local link and anchor in chapters
  1–6 (408) against the built HTML ids. A planted broken self anchor and a
  planted link to the old `#copy-an-existing-policy` anchor each failed it.
  The old anchor occurs nowhere in `docs/`, `README.md` or `CONTEXT.md`.
- Code: a block-by-block diff against the committed chapter shows only the
  agent prompt and its two expected-output lines changed.
- `task docs:book:build` passed without warnings. `task docs:examples`
  passed all 13 complete examples (6 fragments not run), with the SHA-256
  digests of chapters 3–6 the same before and after the run. `task
  docs:figures` matched all nine figures. No figure asset or product source
  changed. This is not a reader-comprehension result.

## Handoff

Cut nothing: set art k130 and set proof k132 already stand, and chapter 4
has no figures. No draft work was found. Chapter 6's three links to the
renamed heading changed only their target and, in one case, the link text.
Chapter 6's own copy-edit (k129) converts the rest of that page. This
retirement leaves node k64 with no live leaf, so k64 closes. No ADR or
`CONTEXT.md` change: this stage changed wording only.
