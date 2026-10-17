# what-ravel-is-for-and-its-actors-k103

## Goal

Copy-edit chapters 1 and 2 as one document under `docs/book/contract.md`,
after draft k102 corrects the account of what Ravel is for. Read the whole
unit and keep changes inside the copy-edit charter.

## Context

This is the document's second correction run, cut by read 1
(`book-read-one-k70`) and authorised by the owner on 2026-10-17. Draft k102's
body has the owner's findings in their words: R1, Ravel runs LLM sessions in
harnesses such as Claude Code and Codex, which typically cannot exit
themselves, hence `end-session`, so a harness can be a component of another
process, typically an orchestrator; and R2, “Publication is the start of
completion” did not make clear what was published.

Give the closest attention to the text k102 changed: chapter 1's opening,
"From a request to a result", "Decide whether Ravel fits", and chapter 2's
caller and harness definitions. Three things to check there:

- **Hedges.** The owner's words say “typically” twice. The prose contract
  allows no hedge without its condition, so the book states the condition
  (an interactive harness; a caller such as an orchestrator).
- **Terms.** "Harness", "LLM session", "orchestrator" and "interactive
  harness" are each spelled one way, match `CONTEXT.md` (**Caller**,
  **Harness**) and link at first use on each page to where chapter 2 defines
  them. "Ravel CLI" stays the term for the `ravel` program; no “front”, and
  no specialist substitutes such as “admits”.
- **Reader.** The contract's reader is a 16-year-old software engineer with
  the stated prerequisites. Do not assume they know how an agent harness
  behaves; the chapter says what they need.

Art k104 and proof k105 already stand after this leaf; do not append another
art or proof leaf. A structural or technical defect goes back to the owner as
an escalation, not as a third draft run.

## Done when

- The whole unit meets the prose contract without absorbing another charter.
- `task docs:book:build` and `task docs:examples` pass, and the document
  brief's handoffs are current.
- Retire this leaf with art k104 next.
