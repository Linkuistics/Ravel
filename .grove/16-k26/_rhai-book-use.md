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
- After the Read 1 corrections, chapters 1–6 follow the owner's Read 1 patterns in section 2 of the contract.
  The proof of that correction run cuts nothing, and Read 1 resumes.

## Read 1 corrections

- Read 1, `rhai-book-use-k74`, stopped in chapter 1 on 2026-10-27.
  The owner asked for the patterns of eight findings in chapters 1–6 before a re-review.
- Section 2 of `docs/book/contract.md` holds the patterns. Two new rows of “Figure needs” hold the figures that the owner asked for.
- The correction run is draft `rhai-book-use-k75`, copy-edit `rhai-book-use-k76`, art `rhai-book-use-k77` and proof `rhai-book-use-k78`.
  Each stage finds its successor already cut.
- This run is the first correction run against this document. Another send-back against it goes to the owner.

## Decomposition

- `rhai-book-use-k27` rewrites chapter 4 and records the owner's revised contract.
- `rhai-book-use-k28` contains two drafts: `rhai-book-use-k70` updates chapters 1, 2, 3 and 5, and `rhai-book-use-k71` updates chapter 6.
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

## Draft closure

- `rhai-book-use-k28` delivered chapters 1, 2, 3, 5 and 6 through its two drafts.
  Chapter 6 is the lookup chapter, with Rhai host calls, harness settings and results, fixed bounds and notice schema 2.
  The example-convention note now uses a runnable Rhai sample and explains cancellation through a fake evaluation harness.
- The book assumes no TypeScript knowledge. References to retired extensions and reserved environment names remain explicit interface rules.
  The default-entry refusal and final installed layout still depend on `typescript-worker-cutover-k31` and delivery removal.
- `rhai-book-use-k69` is the existing next copy-edit stage for the whole document.
  No second editorial pipeline belongs inside the closed draft subtree.
- The final draft checks passed: all 19 book examples and the separate convention sample, nine excerpts, 14 figure reproductions and the book build.
  Chapter 6 passes strict STE and prose-break checks. All 317 rendered links to or from it resolve.
  Mechanical figure reproduction does not discharge the art updates above.
