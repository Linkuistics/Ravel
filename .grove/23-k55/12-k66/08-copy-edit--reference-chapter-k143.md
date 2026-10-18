# reference-chapter-k143

## Goal

Copy-edit chapter 6 after draft `reference-chapter-k142`, under the STE
writing rules and the word decisions in `docs/book/ste.md`.

## Context

- The draft's new text for R16, R17 and R11 has had no copy-edit.
- Apply “unchanged” and “return no result to the caller” (R12 and R13 in
  `docs/book/ste.md`, “The result”) wherever chapter 6 states those concepts.
- Notes in chapter 6 give information, not instructions (Rule 5.5, as set
  proof `book-first-half-k132` found).
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:ste -- --strict` passes on chapter 6.
- The prose check finds no run that is longer than the limit in chapter 6.
- `task docs:book:build` passes.

## Notes
