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
