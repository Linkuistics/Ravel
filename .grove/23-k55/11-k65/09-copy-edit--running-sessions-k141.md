# running-sessions-k141

## Goal

Copy-edit chapter 5 after draft `running-sessions-k140`, under the STE writing
rules and the word decisions in `docs/book/ste.md`.

## Context

- **R12.** Line 189 at `1ef02398` says that values “keep their exact bytes”,
  and line 632 says “delivered the exact sent result”. Use “unchanged”, and
  say that Ravel does not interpret the result (`docs/book/ste.md`, “The
  result”). Apply R13 (“return no result to the caller”) wherever chapter 5
  states that concept.
- The draft's new text for R16, R17 and R11 has had no copy-edit.
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- R12 and R13 apply wherever chapter 5 states those concepts.
- `task docs:ste -- --strict` passes on chapter 5.
- The prose check finds no run that is longer than the limit in chapter 5.
- `task docs:book:build` and `task docs:examples` pass.

## Notes
