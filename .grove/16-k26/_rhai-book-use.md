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

## Handed forward

- **Art: the selection and the routing notice (F2).** Question: what does the policy return, and how does Ravel tell the caller which harness, model and effort the policy chose?
  Notation: UML sequence, with the real routing notice as its example. Row 2 of “Figure needs” in the contract.
  Place: chapter 1, “The selection and the routing notice”, after the text fragment of the routing notice. Chapter 3, “Run the first session”, and chapter 5, “Read notices separately from results”, can reuse it.
  Real values: the review policy of chapter 1 returns `provider: "local"`, `model: "fake"`, `effort: "low"` for the kind `review`.
  The notice is `ravel: starting provider local, model fake, effort low for kind "review"`.
  Order: the worker returns the selection, and Ravel reaps the worker. The Ravel CLI then writes the line on the caller's standard error and launches the program.
  The JSON form replaces the line after the supervision attempt. The labels do not reach the program. Only `program` and `args` do.
  Owner: art `rhai-book-use-k77`. Draft `rhai-book-use-k80` placed no image.
- **Art: how `end-session` finds its session (F5).** Question: how does `ravel end-session`, which runs inside the harness, find the session that launched the harness, also when sessions nest?
  Notation: a process tree with the inherited `RAVEL_EXIT_FILE`, as C4 dynamic or UML object. Row 3 of “Figure needs”.
  Place: chapter 1, “How `end-session` finds its session”, after the table of processes and before the nested fragment. Chapter 2, “Three different kinds of information”, and chapter 5, “Keep nested sessions separate”, can reuse it.
  Real values: the nested run in chapter 1. The outer path is `/private/tmp/ravel-session-mlcN86/signal-c7d5…`, and the inner path is `/private/tmp/ravel-session-QpztGn/signal-b643…`.
  Show the caller, `ravel start-session`, the harness, a child shell and `ravel end-session`, each with its value of the variable.
  The policy worker has none. An evaluation harness run has its own path, and its result goes back to the policy.
  Owner: art `rhai-book-use-k77`. Draft `rhai-book-use-k80` placed no image.
- **Art: a harness run during evaluation (F4, chapter 4).** Question: how does a harness run that the policy launches return its answer to the policy, and how does it differ from the managed process?
  Notation: UML sequence, with the policy worker, the Ravel CLI, the harness run, the caller and the managed process. Art decides whether this needs a new row of “Figure needs”.
  Place: chapter 4, “Run a harness during evaluation”, after the order table and before the `harnessRuns` fragment. Chapter 4, “Follow the evaluation of a policy” and “Route to Claude Code and Codex”, can reuse it.
  Order, from the source: the policy calls `ravel::run_harness`. The worker sends the request over its private socket and waits. The Ravel CLI finds the entry's program, checks the run budget and launches the run in a new POSIX session. The run has its own result channel and the marker `RAVEL_POLICY_RUN`.
  The run reads standard input, writes its answer and exits. The CLI completes the cleanup of the run's group and returns a map to the policy. Later, `select` returns, the CLI reaps the worker, writes the routing notice and launches the managed process.
  Real values: the chapter's Claude Code example, entry `claude`, record `{"elapsedMs":314,"exitCode":0,"harness":"claude","sentResult":false,"signal":null,"status":"exited"}`, final result `prime`.
  The chapter 1 figure of F2 can also serve chapter 4, “Select a command or refuse”.
  Owner: art `rhai-book-use-k77`. Draft `rhai-book-use-k81` placed no image.

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
