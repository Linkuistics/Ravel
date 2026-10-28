# rhai-book-use-k84 — brief

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
  Check the account against `crates/ravel/src/rhai_worker/front.rs`, `crates/ravel/src/worker.rs` and the ADR `configuration-evaluation-precedes-the-launch`.
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

## Decomposition

- `rhai-book-use-k88` settles the shared terms and applies all lessons to Part I, chapters 1–3.
  It writes the new explanations in chapter 1: the concrete review task, the caller's complete task contract and the separate worker.
- `rhai-book-use-k89` applies all lessons and the term map to chapter 4.
- `rhai-book-use-k90` applies all lessons and the term map to chapter 5.
- `rhai-book-use-k91` applies all lessons and the term map to chapter 6, the lookup chapter.
  It also runs the mechanical gates for all of chapters 1–6, and it closes this node.

The four drafts share the stem `rhai-book-use`. Refer to each by key.
Copy-edit `rhai-book-use-k85` follows this node. No draft cuts a later stage.

## Term map

This map is the draft's decision under F10. The owner's words were “configuration” in place of “owner policy”.
The qualifiers are this draft's choice, not a separate owner quotation. `docs/book/ste.md` and `CONTEXT.md` hold the same map.

| Earlier words in prose | Words to use | What the words name |
|---|---|---|
| owner policy, policy (the owner's code) | configuration script | The Rhai file that selects a program or refuses |
| owner settings, settings of the owner | settings | The values in `settings.json` |
| the owner's policy and settings together | configuration | The configuration script and the settings |
| policy worker | Rhai worker | The second copy of the `ravel` executable that evaluates the configuration script |
| policy context | selection context | The JSON data that `select` receives as `context` |
| policy evaluation | evaluation of the configuration script, or evaluation | The run of the configuration script by the Rhai worker |
| policy author | owner, or you | The person who writes the configuration script |

Literal interfaces keep the word “policy”: `policy.rhai`, `--policy-env`, `policyEnv`, `RAVEL_POLICY_RUN`, the `policy_*` diagnostic codes, the CLI help and quoted diagnostics.
Each page that shows such a literal says once that Ravel's file names, options and codes use “policy” for the configuration script.

Titles:

- Chapter 2: “Who does what in a Ravel session” (F15). The file name stays `02-actors-and-words.md`.
- Chapter 4: “Write a configuration script”. The file name stays `04-write-a-configuration-script.md`.
- Chapter 1, last section: “See what we tested and what remains unchecked” (F14).

File names stay unchanged, because chapters 7–11, the README and outside documents link to them.

## Decisions (running log)

- **Four drafts, 2026-10-28.** Chapters 1–6 have about 7,050 lines, against 4,870 at the first correction run.
  The term map changes most paragraphs: “policy” occurs about 1,250 times in the six chapters.
  Part I is one draft, because chapter 1 holds the new explanations and the owner reviewed Part I as one unit.
  Chapters 4, 5 and 6 each get a draft of their own, as in the first correction run (L53).
- **One term map before any chapter edit, 2026-10-28.** L13 asks for shared terms before independent chapter edits.
  The bare word “policy” names the same concept as “owner policy”. If only the two-word phrase changed, one concept would have two names.
  Thus, the map replaces both. The CLI option for the script is already `--config`, so the new term matches an interface that the reader types.
- **“Rhai worker”, not “configuration worker”, 2026-10-28.** The source module is `crates/ravel/src/rhai_worker/`.
  Contract §1 already says “separate Rhai worker” in the chapter 7 row and in Read 2.
- **“Selection context”, not “configuration context”, 2026-10-28.** The data describes the request, not the configuration.
  “Selection” ties the data to `select`, which receives it, and keeps “context” qualified, as the term risks require.
- **Specifications, ADRs, README and CLI help, 2026-10-28.** They still say “owner policy”. That reconciliation is outside the book.
  The live leaf `rhai-spec-reconciliation-k33` already owns the specs, ADRs, glossary and README after the TypeScript removal. Its body now carries this case.
