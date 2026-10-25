# rhai-book-use-k28

## Goal

Draft the changes to chapters 1, 2, 3, 5 and 6 of the Ravel book for Rhai. The concept chapters, the session chapter and the reference match Ravel 2.0.0.

## Context

- The structure brief is `docs/book/contract.md`, as updated by `rhai-book-use-k27`. Read its running log and the new chapters first.
- Chapter 2 names the owner policy and the policy worker, and it mentions Bun. Chapter 3 is a complete session with a TypeScript policy. Chapter 5 runs sessions under real conditions. Chapter 6 is the one lookup chapter.
- Chapter 6 needs the most new content. It lists every setting, bound, request, selection and context shape, error code, exit code and notice field. Take each of them from the spec and the code. The new items are the settings keys `endpoints`, `harnesses` and `budgets`, the host functions, the error codes and the notice schema 2.
- The term "provider" keeps one meaning in the book: the owner's model-origin label. The LLM service is an LLM endpoint. See the term risks in the contract and `CONTEXT.md`.
- The policy worker is still a separate process. It is now Ravel's own binary in an internal role. Chapters 2 and 3 must say so without teaching the internals that Part III covers.
- Every complete example runs through the CLI test seam. Replace each TypeScript example with a Rhai example, and run `task docs:examples`.

## Done when

- Chapters 1, 2, 3, 5 and 6 describe Ravel 2.0.0. They mention no TypeScript policy, except in a link to the migration chapter.
- Chapter 6 lists every new setting, bound, code and notice member, and it removes the Bun and SDK entries.
- Every complete example and excerpt in these chapters passes `task docs:examples` and `task docs:excerpts`.
- The leaf records the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- If chapter 6 does not fit one session, decompose the leaf and put chapter 6 in a child of its own.
- Do not edit chapters 7 to 11. The node `rhai-book-inside` owns them.
