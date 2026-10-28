# rhai-book-use-k84

## Goal

Apply all accumulated documentation lessons to chapters 1–6 before the owner proceeds to Part II.
Draft the corrections across both parts, with concrete explanations that lead to the general case.

## Context

- The owner completed Part I and then asked to apply all lessons before Part II on 2026-10-28.
  The exact findings F1–F16 and the latest timing decision are in `rhai-book-use-k74`.
  The owner explicitly authorised this second correction run. The earlier choice to collect more notes no longer governs its timing.
- Read all of `docs/documentation-lessons.md` and the current `docs/book/contract.md`.
  Apply each relevant lesson throughout chapters 1–6, including the earlier findings that the first correction run addressed.
  The new findings are examples of patterns to find, not the complete search scope.
- The previous correction proof is `rhai-book-use-k78`, at commit `3c80f840`.
  It records source-derived coverage, exact examples and bounded visual evidence. Recheck claims that this draft changes.
- Copy-edit `rhai-book-use-k85`, art `rhai-book-use-k86` and proof `rhai-book-use-k87` already follow this draft.
  All four stages read chapters 1–6. Read 1 stays live and resumes at chapter 4 after proof.

## Required corrections

- Show a concrete review task before the general caller model.
  Name the files, the prompt's instructions, the permitted completion results and the caller's next action for each result.
  Show the prompt tell the harness or LLM to call `ravel end-session RESULT` when its work is complete.
  Explain that the caller assigns meaning to those results and Ravel returns the opaque value after cleanup.
- Prefer familiar configuration terms in reader-facing prose. Reconcile the glossary, chapter titles, dependent terms and descriptions.
  The proposed distinction is configuration for the setup, configuration script for the Rhai file, and settings for `settings.json`.
  Keep exact filenames, CLI options, schema keys, diagnostics and source identifiers when the reader must use them.
  Reconcile the structure brief, contents and cross-references with the final terminology.
- Remove incidental JSON version labels from the narrative. Name the document's purpose instead.
  Keep exact required fields in runnable examples, and explain their values in the reference.
- Explain that Ravel launches its own executable as a separate worker to evaluate the configuration script.
  Explain the parent's continuing supervision and host service, the deadline and failure isolation, and the worker's completion before the final launch.
  Check the account against `crates/ravel/src/rhai_worker/front.rs`, `crates/ravel/src/worker.rs` and the ADR `policy-evaluation-precedes-the-launch`.
- Link Grove to `https://github.com/Linkuistics/grove` where each page introduces it.
  Keep enough local explanation for a reader who does not follow the link.
- Replace unusual shorthand in headings with the subject or the reader's question. Check the whole set of headings.
  The proposed replacements are “Who does what in a Ravel session” and “See what we tested and what remains unchecked”.
  These address chapter 2's title and chapter 1's final heading. Reconcile the contents and every affected anchor or reference.
- Use `cd` directly in chapter 3's directory instruction.
  Apply the same principle to other commands that the stated shell prerequisites already make familiar.

## Done when

- The whole of chapters 1–6 follows the current contract and all applicable lessons, with the prior corrections preserved.
- Explanations open with the reader's need and a concrete case. Each request leads through the action, observable result and caller response.
- Terminology is consistent across the prose, glossary, contents and chapter references. Exact public interfaces still agree with the running CLI.
- Complete examples and exact excerpts pass their repository checks. The book builds and the changed prose passes its checks.
- The node brief's `## Handed forward` list names each figure or other change that a later stage owns, with its location.
- New general lessons extend `docs/documentation-lessons.md`, with their source, trigger, action, scope and a way to check them.
  Product-specific corrections remain examples for the future skill.

## Notes

If the draft needs more than one session, decompose it by coherent chapter groups.
The existing later stages remain the only editorial sequence. Do not add another tail below a draft subtree.
Parts III and IV apply the same lessons in their scheduled rewrite before Read 2.
