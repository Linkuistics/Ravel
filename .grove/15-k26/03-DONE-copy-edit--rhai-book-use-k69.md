# rhai-book-use-k69

## Goal

Copy-edit Parts I and II as one document after both Rhai drafts.
Apply the prose and terminology contract within the copy-edit charter.

## Context

- Structure brief and prose contract: `docs/book/contract.md`.
- The drafts are `rhai-book-use-k27` and `rhai-book-use-k28`. Read their delivered chapters and the node's handed-forward entries.
- The owner removed the migration chapter and the TypeScript prerequisite. Chapter numbers stay unchanged.
  Chapter 4 teaches the Rhai that its policies need. Both owner reader checkpoints remain required.
- Chapter 4 has tested examples of deterministic policies, imports, measured reads, context, data helpers, settings and both kinds of harness execution.
  A model call during evaluation and the final managed session have different result authority.
- The examples create local home and temporary directories. Their commands run fake harnesses with a cleared environment.
- The node brief records the existing STE finding in the Host capability term of `CONTEXT.md`.
  Resolve that wording issue while preserving the guidance about avoided terms.

## Done when

- The whole of Parts I and II follows the prose contract and uses the glossary terms consistently.
- The copy-edit resolves its handed-forward entries and clears them from the node brief.
- `task docs:examples` and `task docs:excerpts` pass after any example or excerpt changes.
- The affected text passes strict STE and prose-break checks, and the book builds.
- Work outside this charter follows the editorial family's handoff or correction procedure.
- The art stage exists under this document node, with the bare stem `rhai-book-use`.
  Cut it only if a live later sibling does not already hold it.

## Notes

Read 1 belongs after proof of Parts I and II. The proof stage cuts that requirements leaf.
The owner did not waive either reader checkpoint.

## Decisions (running log)

- The `binding` finding in `CONTEXT.md` comes from an `_Avoid_` line, which names a word and does not use it.
  `scripts/ste.ts` already exempts these lines from the table of words that the book does not use.
  The fix extends that exemption to the check of words in -ing, because both are word decisions.
  The avoided term `host binding` stays unchanged. `docs/book/ste.md` states the exemption.
  `ste.test.sh` gets a pass control with `host binding` and a mutant that removes the exemption.
- The prose of chapters 1–6 writes "standard output", "standard error" and "standard input".
  Chapters 1–3 already did. Chapters 4–6 also wrote bare stdout, stderr and stdin.
  Code font keeps the identifiers, such as the `stdout` field and the `policy stdout:` prefix.
  `docs/book/ste.md` records the decision for every stage. Chapters 8, 10 and 11 still use bare forms, and `rhai-book-inside` owns them.
- The owner policy has one name. "The script" and "the Rhai script" become "the policy" where they mean the owner policy.
  A fake program's shell script keeps the word "script", as do code identifiers such as `script_limit`.
- The policy worker has one name. "The Rhai worker" becomes "the policy worker".
- A harness run has one name. "Evaluation call", "evaluation run" and "harness call" become "harness run".
  "Evaluation harness run" stays where the text contrasts it with the final managed process.
  Chapter 2 defines "host call" as the request for a host capability, because chapters 2–6 use that word.
- Each page links the first use of each glossary term to the place in chapter 2 that teaches it.
  Chapter 4 had two links to chapter 2, and both lacked an anchor.
