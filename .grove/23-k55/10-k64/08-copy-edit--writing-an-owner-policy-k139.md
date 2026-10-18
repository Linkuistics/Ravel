# writing-an-owner-policy-k139

## Goal

Copy-edit chapter 4 after draft `writing-an-owner-policy-k138`, under the STE
writing rules and the word decisions in `docs/book/ste.md`.

## Context

- The draft's new text for R17 and R11 has had no copy-edit.
- Apply “unchanged” and “return no result to the caller” (R12 and R13 in
  `docs/book/ste.md`, “The result”) wherever chapter 4 states those concepts.
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:ste -- --strict` passes on chapter 4.
- The prose check finds no run that is longer than the limit in chapter 4.
- `task docs:book:build` and `task docs:examples` pass.

## Notes
