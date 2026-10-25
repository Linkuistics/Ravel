# rhai-book-use-k27

## Goal

Draft the migration chapter and rewrite chapter 4, "Write an owner policy", for Rhai. A reader can write a Rhai policy with fake agents, call a model from it, and translate a TypeScript policy.

## Context

- The structure brief is `docs/book/contract.md`, section 1. The brief of this node lists the other inputs.
- Start with the owner. The contract needs a change before the draft, as the brief says. Ask the four questions of the brief, with a recommendation for each, and record the answers in the running log. Change the contract in place. The fourth question decides whether Read 1 and Read 2 of section 4 stay. If the owner waives a read, change section 4 in place. Name that read in the running log.
- Chapter 4 is `docs/book/src/04-writing-an-owner-policy.md`. It has about 1,800 lines, and most of them describe the TypeScript SDK, the typecheck and the Bun worker. Rewrite it against `docs/specs/policy-evaluation.md` and the running binary.
- The migration chapter is new. It builds on the example policies and the difference list in the running log of `rhai-example-policies`.
- The book contract says that every complete example runs. Write each example as `docs/book/examples.md` says, and run `task docs:examples`.
- Chapter 4 is long, and the leaf may not fit one session. If so, decompose the leaf. Put the migration chapter in the first child, and chapter 4 in the next one.

## Done when

- The owner's answers are in the running log, and `docs/book/contract.md` agrees with them.
- Chapter 4 teaches the Rhai policy module, the request, the context and the selection. It teaches the refusals and the host functions for reads and data. It covers the LLM calls, the harness runs, the settings keys, the budgets and the environment grants.
- The migration chapter holds the translated policies, the difference table and the steps to convert an owner's own policy. It states the cutover rule: `policy.rhai` wins, and a lone `policy.ts` refuses.
- Every complete example in these two chapters runs. `task docs:examples` and `task docs:excerpts` pass for them.
- `docs/book/src/SUMMARY.md` and every cross-reference list the new chapter.
- The leaf names the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- Do not describe the TypeScript SDK except in the migration chapter.
- Chapters that other chapters cite by number may move. Search the whole book for the old numbers.
