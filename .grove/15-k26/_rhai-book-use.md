# rhai-book-use-k26 — brief

## Goal

Revise Parts I and II of the Ravel book for Ravel 2.0.0.
The chapters teach Rhai owner policies directly. The pipeline is draft, copy-edit, art and proof.

## Done when

- Chapter 4 teaches Rhai and owns the recipes of an LLM call through Claude Code and Codex.
  Each recipe gives the arguments, prompt placement, structured-output flags and answer extraction.
  Complete examples use fake harnesses and agree with the recorded real smoke calls.
- Chapters 1–6 describe Rhai policies, host capabilities, harness entries and budgets.
  They describe an LLM call as a harness run.
- Every complete example runs through the CLI seam, and `task docs:examples` passes.
  Every excerpt is exact, and `task docs:excerpts` passes.
- The prose follows the contract, and `task docs:ste` reports no problem.
  Figures agree with the text, and `task docs:figures` passes.
- Proof reads both parts as one document and cuts the required Read 1 leaf.

## Decomposition

- `rhai-book-use-k27` rewrites chapter 4 and records the owner's revised contract.
- `rhai-book-use-k28` updates chapters 1, 2, 3, 5 and 6.
- Copy-edit, art and proof follow both drafts.

The two drafts share one stem. Refer to each by key.

## Pointers

- Structure brief and prose contract: `docs/book/contract.md`.
  It states the reader, ordered section plan, emphasis and exclusions.
- Mechanics: `docs/book/examples.md`, `docs/book/excerpts.md`, `docs/book/figures.md` and `docs/book/ste.md`.
- Technical sources: `docs/specs/policy-evaluation.md`, `CONTEXT.md`, the source and the built CLI.
- Example policies and equivalence evidence: `rhai-example-policies-k25`.
- Measured model-call arguments and versions: the running log of `rhai-harness-smoke-k66`.

## Owner decisions

The owner settled these changes in `rhai-book-use-k27` on 2026-10-26:

- The book has no migration chapter or TypeScript prerequisite. Chapter numbers stay unchanged.
- The owner's configuration conversion is a one-off manual task under `owner-policy-conversion-k39`.
- Shell, Git, JSON and Rust remain prerequisites. Chapter 4 teaches the Rhai that its policies need.
- Read 1 and Read 2 remain required under section 4 of the contract.

## Notes

- The book describes Ravel 2.0.0. The checkout still supports both policy formats until the cutover increment.
  The cutover leaf adds the example of a retired-format refusal.
- Chapters 7–11 remain with `rhai-book-inside` after the removal increment.
  Mechanical link fixes can precede that rewrite.
- Run `task setup` before the book checks. The checkout-local Bun remains a development tool after release.

## Handed forward

- **Draft (`rhai-book-use-k28`):** chapter 6 still describes the old interface, as its assigned rewrite expects.
  Three links from its old declaration sections now reach chapter 4's policy-validation section.
  Replace those obsolete entries during the chapter 6 rewrite. Preserve the unchanged chapter numbers.
- **Copy-edit:** the whole-book STE scan reports `binding` in `CONTEXT.md` under the Host capability term's avoided names.
  This predates the draft. Preserve the terminology guidance while resolving the prose-check finding.
  Chapter 4 itself passes strict STE and prose-break checks.
- **Art:** chapter 4 distinguishes evaluation calls from the final managed session through tables and tested examples.
  Check that the process and sequence figures elsewhere teach the same authority boundary after the second draft.
- **Proof:** the book describes 2.0.0. The checkout's default-entry behaviour still accepts the old format until `typescript-worker-cutover-k31`.
  Preserve the cutover dependency. Fake harness examples prove argument delivery and answer checks, not real model behaviour.
  The real-call evidence remains the smoke record, and both owner reader checkpoints remain required.
