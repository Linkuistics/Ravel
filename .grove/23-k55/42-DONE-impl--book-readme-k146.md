# book-readme-k146

## Goal

Make the opening of the README agree with the corrected chapter 1 for the
owner's findings R14 (the reason for a harness) and R15 (the loose contract).

## Context

- R14 and R15 are in `book-read-one-k70`'s log and in the contract's “The
  intended domain”. Chapter 1, after copy-edit
  `what-ravel-is-for-and-its-actors-k135`, has the book's wording.
- The owner said on 2026-10-18 that the reduced README and the removal of the
  two guides read correctly. Thus, change only what R14 and R15 need.
- Model observations, not the owner's: the README says “typically an
  orchestrator” and “cannot normally exit”. The house style does not permit
  a hedge without a condition. Correct them if the changed sentences contain
  them.
- Name no price or plan.

## Done when

- The opening of the README states R14 and R15 and agrees with chapter 1.
- `task docs:ste -- --strict README.md` passes.

## Notes

## Decisions (running log)

- Rewrote only the README's opening. R14: a paragraph says a harness can run
  under its user's subscription, so a caller can use it instead of paid API
  access; it names no price or plan and carries no external link, because
  chapter 1 holds the sourced statement. R15: a paragraph says the contract is
  loose (policy selects a program, Ravel runs it, a program sends one value
  with `end-session`, any such program can stand where a harness stands).
- Removed the model's two hedges as the leaf allowed: “typically an
  orchestrator” became “such as an orchestrator”; “cannot normally exit”
  became “waits for more input when the LLM completes its task”, chapter 1's
  wording. “The harness command” became “the command to run, which is normally
  a harness”, to agree with R15.
- `task docs:ste -- --strict README.md` passes (0 problems); a mutated copy
  with a 41-word sentence and “being” fails, so the check is not vacuous.
  `task docs:breaks` passes.
