# rhai-book-use-k27

## Goal

Rewrite chapter 4, "Write an owner policy", for Rhai.
A reader can write and test a policy with fake agents and ask a model through a harness run.

## Context

- The structure brief is `docs/book/contract.md`, section 1. The node brief names the other inputs.
- The owner settled the revised structure in the running log below. Change the contract in place to match those decisions.
- Chapter 4 is `docs/book/src/04-writing-an-owner-policy.md`.
  Check each claim against `docs/specs/policy-evaluation.md`, the source and the built CLI.
- The book teaches Rhai directly. The owner's configuration conversion is a separate manual task, `owner-policy-conversion-k39`.
- Every complete example runs through the CLI seam under the convention in `docs/book/examples.md`.

## Done when

- The owner's answers are in the running log, and the contract agrees with them.
- Chapter 4 teaches the Rhai module, request, context, selection and refusals.
  It teaches measured reads, data helpers, imports, harness runs, settings, budgets and environment grants.
- Both model-call recipes use the argument vectors from `rhai-harness-smoke-k66` and have complete examples with fake harnesses.
  The text distinguishes those examples from the real smoke tasks. It states that Ravel builds no harness-specific argument.
- Every complete example runs. `task docs:examples` and `task docs:excerpts` pass.
- Links reach the revised sections. Chapter numbers stay unchanged.
- Defects for later stages are in the node brief's `## Handed forward` list.
- A copy-edit leaf exists under the document node, after the second draft.

## Decisions (running log)

<!-- tc: off because these are the owner's quoted words -->
- The owner said: "we should do a manual migration of my configuration as a one-off manual task, so we don't need typescript going forward."
  The owner then said: "rest is agreed".
<!-- tc: on -->
- The book has no migration chapter and assumes no TypeScript knowledge. Chapter numbers stay unchanged.
  The existing `owner-policy-conversion-k39` leaf owns the manual conversion. It needs no migration guide in the book.
- The book retains shell, Git, JSON and Rust as prerequisites. Chapter 4 teaches the Rhai syntax that its policies need.
  It then teaches Ravel's host capabilities, restrictions and examples. The book adds no separate Rhai tutorial chapter.
- Read 1 and Read 2 remain obligations under section 4 of the contract. The owner waived neither read.
- The retired-format remedy and release notes will point to chapter 4 for Rhai policy authoring.
  The live cutover, release and conversion tasks carry that change, so none depends on the removed migration chapter.
- The technical review used the leaf's one reviewer. The reviewer found one actionable overstatement about schema diagnostics.
  Property names can appear in a mismatch location. Property values do not appear there.
  Direct reads of `schema.rs` and `rhai_worker/helpers.rs` established the finding. The chapter and specification now state that distinction.
  This wording correction needs no further review or code change.
- The reviewer also identified the default-entry transition as a visible, accepted release dependency.
  The chapter describes 2.0.0, while this checkout still accepts the old default until `typescript-worker-cutover-k31`.
  The existing cutover task retains that obligation. No additional review finding remained.
- Source verification used direct reads because the graph could not start.
  `list_projects` reported an active pre-coordination or unverified generation. No graph project, generation or coverage claim was available.
  The bounded source set was the Rhai worker, context validation, settings, limits, environment grants, harness entries and example policies.
  The host-function inventory came from `rhai_worker/bridge.rs` and `helpers.rs`. Context fields came from `context.rs`.
  The request and module exports came from `engine.rs`. The run contract came from `host.rs`, `harness.rs` and the specification.
- Both installed harness versions still match the smoke record: Claude Code 2.1.310 and Codex CLI 0.164.0.
  Their help documents the model-call and interactive arguments used here. This session made no real model call.
- The examples now create their own local home and temporary directories and clear each Ravel command's environment.
  Their settings cannot overwrite the reader's personal settings. The CLI example seam proves these commands with fake harnesses.
- Verification: the whole-book example run passed 19 examples, with no failure or skip.
  After the sandbox command changes, the focused chapter run passed all 11 examples, with no failure or skip.
  The subsequent prose-only change split the introductory paragraph. No example block changed after that run.
- All 9 book excerpts pass. The book builds without warnings, and all 14 figures match their sources.
  Chapter 4 passes strict STE and prose-break checks. The controlled-English checker reports no hard finding in the changed prose.
  The whole-book STE scan retains one pre-existing finding in `CONTEXT.md`, assigned to copy-edit in the node brief.
- The rendered link check examined 207 local links into or out of chapter 4 and found none broken.
  Its controls accepted a known heading and rejected a deliberately absent heading.
- SHA-256 digests stayed unchanged across the final example run for 323 input files.
  The subjects were `docs/book`, `scripts`, `crates`, `target/tools`, the manifests, `Taskfile.yml`, `CONTEXT.md`, `README.md` and both built Ravel binaries.
  The file set excluded dependency directories and build outputs other than the named tools and binaries.
  The local manifest is `target/verification/rhai-book-use-k27.before.json` and is not a repository artifact.
