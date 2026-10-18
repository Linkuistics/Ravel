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

## Decisions (running log)

- One session holds the whole chapter. Decomposition along its sections is
  not necessary: the chapter has about 800 lines, and most code blocks do not
  change. The strict check found 159 problems before this stage.
- The H1 becomes “Run sessions”. Seven of the eight section headings stay:
  each is a bare infinitive with no “-ing” word, and chapter 6 links to
  their anchors. The heading “Read notices without confusing them with
  results” has a verb form in “-ing” (“confusing”). It becomes “Read
  notices separately from results”. (A first choice, “Read the notices and
  failures on stderr”, did not cover the stdout side of the section. The
  reviewer found this, and it was replaced before commit.) Chapter 6 linked
  to the old anchor twelve times, and chapter 5 one time. Those links now
  use the new anchor. Their link texts (“read notices”, “Operational
  failures” and others) agree with the new heading, so they do not change.
- As in k125 and k126, this stage converts the words and sentences of the
  three alt texts and of the paragraphs before the figures, because the
  strict check reads them. It keeps their facts. The colour repetition (R8)
  and every figure asset stay for set art k130.
- Examples: the kind `publish` becomes `send`. The prompts “Publish inner.”
  and “Publish confined.” become “Send inner.” and “Send confined.”, and the
  outer prompt says “Run the inner session. Then add |outer to its result
  and send it.” `prompt.txt` says “When you finish, send the agreed result.”,
  and its expected standard output changes with it. The policy strings
  `model: "publisher"` and the `reason` texts with “publication” change to
  “sender” and to sentences with “send” or “sent result”. No command line,
  status or other expected output changes.
- The exit-status rows for 129, 130 and 143 say “Ravel accepts no result,
  even if it already received one.” This keeps the exact meaning:
  cancellation stops a result that Ravel already received.
- The second paragraph of “Interpret the exit status” (one sentence of 83
  words) becomes a vertical list of the sections that teach each status
  (Rule 4.3).
- “Routing” as a bare noun in the last paragraph becomes “the choice of
  command” (`ste.md`). “Routing notice” stays: it is a kept technical name.
- The committed sentence on `--exit-dir` said, word for word, “choose a
  parent holding nothing the managed process should be able to change”. That
  is the reverse of its plain intent. The new text gives the intent with no
  new judgement: “do not choose a parent directory that contains files that
  the managed process must not change.”
- “Keep” is the result verb in `ste.md` (“Ravel keeps the first result”).
  Thus, stdout, the prompt text and `HOME` use other verbs: “uses stdout only
  for the result”, “Read stdout only as the result”, “Ravel's promise not to
  change the text ends when the policy receives it”.
- The signal fallback says “If the propagation of the signal fails”, not
  “cannot raise the signal”. `reraise` in `crates/ravel/src/cancellation.rs`
  treats a failure of any step (default action, unblock, raise) as one
  propagation failure.
- One in-session reviewer compared the committed and the new chapter for
  meaning changes and prose-contract breaches that the script cannot find.
  It reported 15 items. Classification:
  - Valid and fixed, meaning: the `--exit-dir` intent (above); the signal
    fallback scope (above); the reason why an inner group can remain
    (“Thus, if …”); “or to give the task a meaning” (the pronoun had moved
    to the inputs); the status-1 action names the managed program as the
    one that makes the call; “sends no result and exits naturally” keeps
    “natural” and the order; “the runner's check” keeps who checks.
  - Valid and fixed, STE and contract: “Neither the unconfined mode nor the
    confined mode” (unclear referent after a moved sentence); the `HOME`
    sentence had “this” with no noun and repeated a fact; the status-5 cell
    is full sentences, not fragments; the measured-platform cell says “built
    Ravel for x86_64 GNU/Linux”; the telegraphic “only as far as the
    policy”; “keep” for stdout (above).
  - Valid and fixed, structure: the argument-limit sentences had moved one
    paragraph down. They are again directly after the prompt paragraph, as a
    paragraph of their own (a split at a topic change, not a reorder).
  - Valid and fixed, heading: the heading (above).
  - Visible trade-off: “smaller than 1 MiB” states the comparison that the
    committed text only implied, next to the 1 MiB limit. It is not a new
    fact.
  - No item was noise. The reviewer found no present perfect, no “would”,
    no phrasal verb and no instruction over 20 words, and all changed
    examples agree with their expected output.

## Verification

- `task docs:ste -- --strict docs/book/src/05-running-sessions.md`: 0
  problems (159 before). `task docs:ste:test` passes, with each mutant seen
  to fail a control. No “-ing” word needed a new `ste.md` decision. Each one
  was written again without it.
- Result words: a case-insensitive search for publish, publication, submit
  and submission finds no line in chapter 5. The same search finds 41 lines
  in the committed chapter 5 and finds a planted “Submission”.
- Links: a scratch checker resolved every local link and anchor in chapters
  1–6 (408) against the built HTML ids. A planted link to each of the two
  old anchors and a planted broken anchor each failed it. Neither old
  anchor occurs outside `.grove/` task records.
- `task docs:book:build` passed. `task docs:examples` passed all 13 complete
  examples (6 fragments not run), with the SHA-256 digests of chapters 1–6
  the same before and after the run. `task docs:figures` matched all nine
  figures. No figure asset or product source changed. In code blocks, only
  the words in the examples decision above changed. This is not a
  reader-comprehension result.

## Handoff

Cut nothing: set art k130 and set proof k132 already stand. No chapter 5
draft work was found. The parent brief's Read 1 section carries the art
handoff (the figure text that still uses the old words, and R8), because
k130 does not read this node's brief. It also tells copy-edit k129 that
chapter 6's links to this chapter's renamed anchor changed. This
retirement leaves node `running-sessions-k65` with no live leaf, so k65
closes. No ADR or `CONTEXT.md` change: this stage changed wording only.
