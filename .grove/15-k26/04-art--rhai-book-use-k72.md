# rhai-book-use-k72

## Goal

Make the figures of Parts I and II agree with the copy-edited text of chapters 1–6.
Keep one visual style across the book, as section 3 of `docs/book/contract.md` states.

## Context

- Structure brief and prose contract: `docs/book/contract.md`. Figure mechanics: `docs/book/figures.md`.
- Read the node brief's `## Handed forward` list. Its art entries name the figures to change:
  `selection-cancellation.typ`, `process-authority.typ`, `source-components.typ` and `installed-files.typ`.
- Copy-edit `rhai-book-use-k69` fixed one name for each term in chapters 1–6.
  A figure label uses the same words as the text:
  - "policy worker", not "Bun worker" or "Rhai worker".
  - "harness run" for a run during evaluation, and "final managed process" for the selected program.
  - "host call" for a request of a host capability.
  - "standard output", "standard error" and "standard input" in text. Code font keeps `stdout` where it names a field.
- The alt text of `source-components.svg` and `installed-files.svg` in chapter 6 says "internal Rhai worker role".
  Copy-edit did not change it, because the alt text describes the figure. Update the alt text with each figure.
- Chapter 5 states that the fake evaluation harness sends INT to the Ravel CLI. The policy itself cannot send a signal.
  The current selection-cancellation figure and its alt text show the policy worker as the sender.

## Done when

- Each figure in chapters 1–6 agrees with its alt text, its caption and the prose beside it.
- The handed-forward art entries are closed and cleared from the node brief.
- `task docs:figures` passes. `task docs:ste -- --strict` passes on each changed `.layout.json` and chapter.
- `task docs:book:build` succeeds.
- Work outside this charter follows the editorial family's handoff or correction procedure.
- The proof stage exists under this document node, with the bare stem `rhai-book-use`.
  Cut it only if a live later sibling does not already hold it.

## Notes

Proof cuts the Read 1 requirements leaf. The owner did not waive either reader checkpoint.
