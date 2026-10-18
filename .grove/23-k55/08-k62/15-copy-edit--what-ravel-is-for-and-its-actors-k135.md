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
