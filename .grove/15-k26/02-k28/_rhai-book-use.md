# rhai-book-use-k28 — brief

## Goal

Draft the changes to chapters 1, 2, 3, 5 and 6 of the Ravel book for Rhai. The concept chapters, the session chapter and the reference match Ravel 2.0.0.

## Context

- The structure brief is `docs/book/contract.md`, as updated by `rhai-book-use-k27`. Read its running log and the new chapters first.
- Chapter 2 names the owner policy and its internal Rhai worker. Chapter 3 is a complete session with a Rhai policy. Chapter 5 runs sessions under real conditions. Chapter 6 is the lookup chapter.
- Chapter 6 needs the most new content. It lists every setting, bound, request, selection and context shape, error code, exit code and notice field. Take each of them from the spec and the code. The new items are the settings keys `harnesses` and `budgets`, the host functions, the error codes and the notice schema 2.
- The term "provider" keeps one meaning in the book: the owner's model-origin label. A call to a model is an LLM call, which is a harness run. See the term risks in the contract and `CONTEXT.md`.
- The policy worker is still a separate process. It is now Ravel's own binary in an internal role. Chapters 2 and 3 must say so without teaching the internals that Part III covers.
- Every complete example runs through the CLI test seam. Replace each TypeScript example with a Rhai example, and run `task docs:examples`.

## Done when

- Chapters 1, 2, 3, 5 and 6 describe Ravel 2.0.0 and teach Rhai policies directly.
  The owner removed the migration chapter in `rhai-book-use-k27`. Chapter numbers stay unchanged, and the book assumes no TypeScript knowledge.
- Chapter 6 lists every new setting, bound, code and notice member, and it removes the Bun and SDK entries.
- Every complete example and excerpt in these chapters passes `task docs:examples` and `task docs:excerpts`.
- The leaf records the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The existing document-level copy-edit leaf `rhai-book-use-k69` follows both drafts. No duplicate pipeline is necessary.

## Notes

- If chapter 6 does not fit one session, decompose the leaf and put chapter 6 in a child of its own.
- Do not edit chapters 7 to 11. The node `rhai-book-inside` owns them.

## Decomposition

- `rhai-book-use-k70` updates chapters 1, 2, 3 and 5, including their complete examples.
- `rhai-book-use-k71` derives the source inventory and rewrites chapter 6.
- Both children use the structure brief `docs/book/contract.md`.
  The document's editorial pipeline belongs to `rhai-book-use-k26`.
  Its live copy-edit leaf `rhai-book-use-k69` follows this subtree.
  Neither child cuts another copy-edit leaf inside this node.

## Decisions (running log)

- The reference chapter is a separate source-verification task: its settings, bounds, host functions, errors and notice fields need a complete inventory.
  The narrative chapters form the first independently runnable slice. Chapter 6 forms the second.
  This is the decomposition permitted by the original leaf, not a second editorial pipeline.
