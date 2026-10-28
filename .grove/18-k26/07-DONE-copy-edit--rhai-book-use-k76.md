# rhai-book-use-k76

## Goal

Copy-edit chapters 1–6 after the Read 1 draft `rhai-book-use-k75`, under the amended prose contract.

## Context

- The draft applied the owner's Read 1 patterns. Section 2 of `docs/book/contract.md` states them, after “Term list”.
- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> The owner's words of 2026-10-27: “Good human-centered writing doesn't use DRY at all. Repetition and restatement are important techniques.”
  Keep each restatement, preview and summary that helps the reader. Do not replace a repeated explanation with a link.
- The owner also asked for “a more conversational tone where possible”.
  Keep “you”, the question that the reader has, and the reason before the detail.
- The STE rules still apply. When a sentence of a pattern is too long, split it.
  Keep its order: the reader's goal first, then the means.
- “Terms” now gives each term a short explanation at its first use on each page. Check that each page has these explanations.

## Done when

- Chapters 1–6 follow the prose contract and `docs/book/ste.md`. A new word decision goes into `docs/book/ste.md`.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–6.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Art `rhai-book-use-k77` and proof `rhai-book-use-k78` already follow this leaf. Cut no next stage.
- This run is the first correction run against this document. Another send-back against it goes to the owner.

## Decisions (running log)

- Correct the refusal table in chapter 4 to name the third command, whose
  values it explains.
- Qualify the inheritance explanation in chapter 6's `RAVEL_EXIT_FILE` entry,
  so it accounts for a parent that changes its child's environment.
- Explain the terms in the opening passages before their first use. Keep the
  chapter-level restatements, because they help the reader.
- Qualify environment inheritance in chapter 5: a parent can change the
  environment that its child receives.
- Apply the same qualification to chapter 4's interactive harness example.
- Qualify the explanation of inheritance in chapter 1 as well: the path reaches
  `end-session` when each parent leaves the variable unchanged. Apply the same
  qualification in chapters 2 and 6.
- Explain a process group at its first use in chapter 2. Describe the fake
  agent as showing the selected command, because the policy makes the choice.

## Continuation

- The copy-edit is incomplete. Read all of chapters 1–6 against the contract,
  including the portions not yet reviewed, before retirement.
- On 2026-10-28, strict STE and prose-break checks passed for chapters 1–6.
  The examples check passed 21 examples, the excerpts check passed nine
  excerpts, and the book build passed. SHA-256 checks confirmed that the six
  chapter files did not change during those checks. Run the checks again if
  later edits change their inputs.
- Art and proof already follow this leaf. Do not add another successor.

- On 2026-10-28, the continuation read chapter 2 and the beginning of chapter 6.
  It qualified inheritance in chapters 1, 2 and 6: a parent can change the
  variable before it starts a child. Strict STE and prose-break checks passed
  for chapters 1–6 after these changes. The full read is still incomplete.
  Complete the whole-document read, then rerun all checks in Done when.
- A further continuation read chapter 2 in full, explained a process group at
  its first use there and corrected the attribution of the command choice.
  Strict STE and prose-break checks passed for chapter 2 after that edit.
  The combined output requested for chapters 1–3 was truncated, so it is not
  evidence of a complete read of those chapters. The full read remains open.
- This continuation read chapters 1, 3 and 4 in full, and chapter 5 through
  line 730, within “Confine the managed process”. It corrected chapter 4's
  refusal table: the example values come from the third command. Strict STE
  and prose-break checks passed for chapter 4 after that correction.
  It then read the rest of chapter 5 and all of chapter 6, and qualified the
  inheritance explanation in the `RAVEL_EXIT_FILE` entry.
- After the edits, setup, strict STE and prose-break checks for chapters 1–6,
  all 21 examples, all nine excerpts and the book build passed. SHA-256
  digests of the six chapter files matched before and after the checks.
  No new documentation lesson arose from these corrections.
