# rhai-book-use-k26 — brief

## Goal

Revise Parts I and II of the Ravel book for Ravel 2.0.0, and add the migration guide. The chapters teach Rhai owner policies. The pipeline is draft, copy-edit, art and proof.

## Done when

- Chapters 1 to 6 describe Rhai policies, the host functions, the LLM endpoints, the harness entries and the budgets. No chapter of these two parts teaches TypeScript as the policy language.
- A new chapter holds the migration guide. It translates the shipped static route table, the shipped sample policy and a policy with the shape of the owner's. It adds the LLM-assisted example. It explains the difference table of the spec, and the refusal for a lone `policy.ts`.
- Every complete example runs through the CLI test seam, and `task docs:examples` passes. Each excerpt is exact, and `task docs:excerpts` passes.
- The prose follows the prose contract, and `task docs:ste` reports no problem. The figures of these chapters match the text, and `task docs:figures` passes.
- The proof stage has read the two parts as one reader would.

## Decomposition

- `rhai-book-use` (first draft) writes the migration chapter and rewrites chapter 4, "Write an owner policy".
- `rhai-book-use` (second draft) updates chapters 1, 2, 3, 5 and 6.
- Copy-edit, art and proof follow. The second draft or the first one cuts them, as the editorial procedure says.

The two drafts share one stem, so name them by key. The migration chapter comes first because chapters 5 and 6 refer to its terms.

## Pointers

- Structure brief and prose contract: `docs/book/contract.md`. It states the reader, the section plan, the exclusions, the term risks, the examples policy and the figure needs. `docs/book/examples.md`, `docs/book/excerpts.md`, `docs/book/figures.md` and `docs/book/ste.md` state the mechanics.
- Source of truth for the content: `docs/specs/policy-evaluation.md`, `CONTEXT.md` and the code. Check each claim against the running binary and the source, not against the text of the old chapter.
- Example policies and their equivalence tests: the leaf `rhai-example-policies`.
- The running logs of the leaves in the increments `rhai-local-capabilities`, `rhai-llm-calls` and `rhai-harness-runs` record every difference from the spec.

## Notes

- The contract says that the reader can read and write TypeScript. The policy language is now Rhai, so the contract needs a change. A change to the structure is made with the owner. The first draft asks the owner before it drafts.
- The questions for the owner are four. Where does the migration chapter go? What new prerequisites does the reader need? What does the book teach about Rhai? Do Read 1 and Read 2 stay as obligations for this revision? Recommend that both stay. The revision rewrites the chapters that the reads test. The recommended place is Part II, after "Write an owner policy". This renumbers the later chapters, as the removal of chapter ten did before.
- The TypeScript path still exists on main while this node runs. The book describes Ravel 2.0.0. Do not add a complete example that needs a refusal of `policy.ts`. Describe that refusal in prose. The removal increment adds the example.
- Chapters 7 to 11 describe the worker, the installation and the release. They stay as they are until the node `rhai-book-inside`, because the removal increment changes what they describe.
- Section 4 of the contract places Read 1 after the proof of Parts I and II. It places Read 2 after the proof of Parts III and IV. Each read is a `requirements` leaf, and it is not optional. The proof stage cuts the Read 1 leaf, unless section 4 of the contract no longer requires it. Only the owner can remove the obligation. The first draft then changes the contract.
- Run the checks of the book with `task setup` done first. The book uses a checkout-local Bun as a development tool. That tool stays after the release.
