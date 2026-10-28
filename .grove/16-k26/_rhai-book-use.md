# rhai-book-use-k26 — brief

## Goal

Revise Parts I and II of the Ravel book for Ravel 2.0.0.
The chapters teach Ravel configuration with Rhai directly. The pipeline is draft, copy-edit, art and proof.

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
- The draft node `rhai-book-use-k75` closed on 2026-10-28 after four drafts, `rhai-book-use-k80` to `rhai-book-use-k83`.
  Chapters 1–6 now open each section with the reader's goal and explain each term at its first use on the page. Each mechanism has a real example.
  Chapter 6 restates the words of every page, Rhai and chapter 1's real values, because a reader can enter it at any entry.
  The F2, F4 and F5 entries under “Handed forward” name each place for a figure. No draft placed an image.

## Decomposition

- `rhai-book-use-k27` rewrites chapter 4 and records the owner's revised contract.
- `rhai-book-use-k28` contains two drafts: `rhai-book-use-k70` updates chapters 1, 2, 3 and 5, and `rhai-book-use-k71` updates chapter 6.
- Copy-edit, art and proof follow both drafts.

The two drafts share one stem. Refer to each by key.

## Corrections before Part II

The owner completed Part I on 2026-10-28 and then asked to apply all lessons before proceeding to Part II.
This supersedes the earlier choice to collect the remaining notes first. The exact words and findings F9–F16 are in `rhai-book-use-k74`.
The second correction run is explicitly authorised. Another substantial send-back requires escalation to the owner.

The run is draft `rhai-book-use-k84`, copy-edit `rhai-book-use-k85`, art `rhai-book-use-k86` and proof `rhai-book-use-k87`.
Each stage reads the whole of chapters 1–6 under its charter and finds its successor already standing.
No stage adds another editorial tail. Proof cuts no checkpoint, and Read 1 stays live.

Read the full current contract and `docs/documentation-lessons.md`.
Apply all relevant lessons, including F1–F16, throughout both parts. The new quotations are examples of patterns, not a boundary around isolated sentences.
The work includes concrete cases before general explanations, the complete caller task contract, configuration terms and descriptive headings.
It also includes the worker's purpose, JSON descriptions, Grove links and familiar shell commands.

After proof, Read 1 resumes at chapter 4. It gives the owner a correction recap and access to the amended Part I.
The owner need not restart at chapter 1. The practical reader tasks still need the owner's results.
Parts III and IV apply the same current lessons in their scheduled rewrite before Read 2.

## Handed forward

- **Art (`rhai-book-use-k86`): figure labels with the old terms.** Draft `rhai-book-use-k88` changed the prose to the term map in the brief of `rhai-book-use-k84`.
  The figure sources below still say “policy worker”, “owner policy” or “policy” for the configuration script.
  Use “Rhai worker”, “configuration script” and “settings”. Keep the literal `RAVEL_POLICY_RUN`.
  - `routing-notice.typ`, chapters 1 and 6. The chapter 1 alt text already says “Rhai worker”.
  - `inherited-result-path.typ`, chapters 1, 5 and 6. The chapter 1 alt text already uses the new terms.
  - `system-context.typ`, chapter 1: “owner policy and settings” in the owner's description and relation.
  - `process-authority.typ`, chapter 2. F12 also applies: the figure must agree with the chapter 2 text.
    That text says that the worker is a second copy of the executable. At the deadline, the parent can signal it, and a kernel stall can delay its exit. The process isolates a script failure.
  - `successful-session.typ`, chapter 3, and `evaluation-harness.typ`, chapters 4 and 6.
  - `selection-cancellation.typ` and `settled-exit-status.typ`, chapter 5.
  - `source-components.typ` and `installed-files.typ`, chapter 6. Chapters 7 and 11 also show them, and `rhai-book-inside` owns that prose.
  Check each alt text and caption after the label change. Drafts `rhai-book-use-k89` to `k91` change the alt text of chapters 4–6.
  Draft `rhai-book-use-k89` changed chapter 4's alt text and caption of `evaluation-harness.typ` to “Rhai worker” and “configuration script”. The figure labels still say “policy worker” and “policy”.
  Draft `rhai-book-use-k90` changed chapter 5's alt text of `inherited-result-path.typ` to chapter 1's words and the alt text of `selection-cancellation.typ` to “Rhai worker”. The figure labels still use the old terms.
  Draft `rhai-book-use-k91` changed chapter 6's alt text of `routing-notice.typ`, `inherited-result-path.typ` and `evaluation-harness.typ` to the words of chapters 1 and 4.
  It also changed the alt text and the captions of `source-components.typ` and `installed-files.typ` to “Rhai worker” and “configuration script”. The labels of all five figures still use the old terms.
- **Art (`rhai-book-use-k86`): the review flow of chapter 1.** “Follow one code review from request to result” gives the review as two tables.
  The tables give the order of actions and the orchestrator's action for each result. No figure shows that flow.
  Decide whether a figure helps the reader there, under the contract's “Give a diagram and a concrete example”.
  Chapter 5, “Accept a result only after success”, now has the same action table and a complete example in which an orchestrator script acts on each outcome. Decide the figure question for both places together.

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
  Mechanical link corrections can precede that rewrite.
- Run `task setup` before the book checks. The checkout-local Bun remains a development tool after release.

## Draft closure

- `rhai-book-use-k28` delivered chapters 1, 2, 3, 5 and 6 through its two drafts.
  Chapter 6 is the lookup chapter, with Rhai host calls, harness settings and results, constant bounds and notice schema 2.
  The example-convention note now uses a runnable Rhai sample and explains cancellation through a fake evaluation harness.
- The book assumes no TypeScript knowledge. References to retired extensions and reserved environment names remain explicit interface rules.
  The default-entry refusal and final installed layout still depend on `typescript-worker-cutover-k31` and delivery removal.
- `rhai-book-use-k69` is the existing next copy-edit stage for the whole document.
  No second editorial pipeline belongs inside the closed draft subtree.
- The final draft checks passed: all 19 book examples and the separate convention sample, nine excerpts, 14 figure reproductions and the book build.
  Chapter 6 passes strict STE and prose-break checks. All 317 rendered links to or from it resolve.
  Mechanical figure reproduction does not discharge the art updates above.

## First Read 1 correction closure

Proof `rhai-book-use-k78` read chapters 1–6 after the correction run and
checked the Read 1 patterns throughout. It corrected notice-time summaries,
the final-launch scope of refusals and bounds, a partial-output claim and
repeated inheritance explanations. It also corrected the prose of the three
new figures and regenerated their artifacts. No hand-forward entry remains.

All 21 examples, nine excerpts, seventeen figure reproductions, strict prose
checks and the book build passed. The local link check passed 762 targets,
with a broken-anchor control. Safari in an isolated macOS guest supplied the
local viewing evidence. The proof task records its precise scope and limits.

At that boundary, the node's Done when held. Read 1 stayed live as `rhai-book-use-k74`, ready
for the owner's re-review from chapter 1. No second checkpoint was cut.
The later findings now require the correction run above before the owner proceeds to Part II.

## Second correction draft closure

The draft node `rhai-book-use-k84` closed on 2026-10-28 after four drafts, `rhai-book-use-k88` to `rhai-book-use-k91`.
Chapters 1–6 use the term map of `ste.md`, “Configuration words”. Outside code and link targets, “policy” remains only where a page explains Ravel's literal names, help and diagnostic words.
Each chapter opens with chapter 1's review, and chapter 6 restates the worker's three reasons. JSON version labels stay only in examples and in the entries that teach their values.

The last draft passed all 22 examples, nine excerpts, seventeen figure reproductions and the book build.
Strict prose and break checks passed for chapters 1–6. A local link check of the built book and the README passed, with failing controls.
The figure labels with the old terms remain under “Handed forward” for art.
