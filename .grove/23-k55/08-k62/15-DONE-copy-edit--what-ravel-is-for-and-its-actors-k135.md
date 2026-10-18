# what-ravel-is-for-and-its-actors-k135

## Goal

Copy-edit chapters 1 and 2 after draft `what-ravel-is-for-and-its-actors-k134`.
Apply the owner's wording findings R12 and R13 and the STE writing rules.

## Context

- **R12.** Chapter 1, line 128 at `1ef02398`: “A successful `start-session`
  writes the exact bytes of the result to standard output.” Write that Ravel
  writes the result unchanged to standard output, and that Ravel does not
  interpret the result.
- **R13.** Chapter 1, line 138: “These outcomes give no successful result:”.
  Write “Some outcomes return no result to the caller:”, or a sentence with
  the same words.
- The word decisions are in `docs/book/ste.md`, “The result”. The draft can
  move these passages. Find them by their meaning, and apply the decisions
  wherever chapters 1 and 2 state the same concepts.
- The draft's new text for R14, R15, R16 and R11 has had no copy-edit.
- This is the fourth copy-edit run on this document. The owner authorised it
  on 2026-10-18 in `book-read-one-k70`.

## Done when

- R12 and R13 apply wherever chapters 1 and 2 state those concepts.
- `task docs:ste -- --strict` passes on chapters 1 and 2.
- The prose check finds no run that is longer than the limit in chapters 1
  and 2.
- `task docs:book:build` passes.

## Notes

## Decisions (running log)

- 2026-10-18: R12 and R13 occur only at their two places in chapter 1. The
  other statements of the result on standard output already say “unchanged”
  (chapter 1, steps table and `continue` example; chapter 2, opaque result).
  Chapter 1 now says: “After a successful session, Ravel writes the result
  unchanged to standard output. Ravel does not interpret the result and adds
  no newline.” The partial-write sentence says “only the start of the
  result” and “the output alone”, not “prefix” and “the bytes alone”. The
  list opens with “Some outcomes return no result to the caller:”.
- 2026-10-18: R15 consistency. Chapter 1 says that the policy decides
  “whether and how” to include the prompt, not only “how”. In the table,
  “The policy decides whether the program receives it” moves to the column
  of what Ravel needs, because it is part of Ravel's contract, not of the
  harness use. “Ravel needs nothing more from the program” replaces
  “harness-specific behaviour”, and “does not have to” replaces “need not”.
- 2026-10-18: R14 keeps its conditional cost (“can be much lower”). “Run
  under the subscription of its user” replaces “use its user's
  subscription”. The only “We” in the book becomes “This paragraph agrees
  with those pages as they were on 18 October 2026”: the reader can check
  the date and the pages.
- 2026-10-18: One spelling for each concept: `$HOME/.config/ravel/policy.ts`
  (chapters 1, 4 and 6 use it, chapter 2 used `~`), “result value” (not
  “return value”), “operating-system” as an adjective, “a small policy”
  (chapter 2 and 3 wording), “take the place of”, `end-session` (not
  “completion command”), the `provider`, `model` and `effort` fields in code
  font in both chapters, and “the managed process and its process group”
  (chapter 1 does not teach “managed job”).
- 2026-10-18: Checkable claims and clear referents: the limits table says
  “Other code runs as the same user as Ravel”. “These facts are limits of
  the evidence” and “the same evidence” replace a bare “These” and “equal
  support measurements”. “The absence of a store” replaces a bare “this”.
  Cancellation suppresses “the result” “before the cancellation”, as in
  chapter 1. Both chapter 6 cells in the route table use one form.
- 2026-10-18: No heading changes, so no anchor changes. Figure prose, alt
  text and figure files belong to set art k144 and stay unchanged.
- 2026-10-18: No in-session reviewer. Each change keeps the meaning of the
  draft's sentence, and the only moved fact (the prompt cell) follows the
  draft's own R15 statement.

## Verification

- `task docs:ste -- --strict` on both chapters: 0 problems before and after.
- `task docs:breaks -- --strict` on both chapters: 0 runs longer than 450
  words before and after.
- Old wording: a case-insensitive search for “exact bytes”, “give no
  (successful) result”, “the bytes alone”, “need not”, “managed job”,
  `~/.config`, “return value”, “completion command”, “harness's place”,
  “We”, “tiny policy” and “harness-specific” finds each one in the
  committed chapters and none in the edited chapters. A scratch file with
  the two old R12 and R13 sentences fails it. The new R12 and R13 words are
  present.
- `task docs:book:build` exits 0, with no warning or error line.
- A search for `**Reviews:**` lines that name k135 or k62 finds none. The
  same search finds other review lines. `GROVE_PROVIDER_EVIDENCE` is not
  set. Thus, no Creator line is written.
- This is not a reader-comprehension result. `task docs:examples` did not
  run: no example or code block changed.

## Handoff

Cut nothing: set art `book-first-half-k144` and set proof
`book-first-half-k147` serve this document (node brief). The node brief's
entry for this stage is cleared. Its entries for drafts k140 and k142 and set
art k144 stay, because those leaves read them there. This retirement leaves
node k62 with no live leaf, so k62 closes again.
