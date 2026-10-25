# llm-harness-replan-k60

## Goal

Cut the work that the design `llm-calls-through-harnesses-k58` implies, and bring the later leaves into line with it. The design moved every LLM call to a harness run of Claude Code or Codex, and it removed `ravel::llm` and Rig.

## Context

- Read the spec sections that the brief of `rhai-llm-calls-k15` lists. Read them after the review `llm-calls-through-harnesses-k59` and any integration of it.
- Leaves `rhai-llm-adapter-proof-k54`, `rhai-llm-anthropic-call-k55`, `rhai-llm-bounds-k17`, `rhai-llm-budgets-k18`, `rhai-llm-openai-types-k19` and `rhai-llm-structured-output-k20` built the code that goes. Leaf `rhai-settings-discovery-k14` built the `endpoints` key and `ravel::endpoints()`.
- The owner pruned `rhai-llm-upgrade-proof-k21`. It held the Taskfile task for a Rhai upgrade, which still needs a leaf.

## Done when

- An `impl` leaf `rhai-llm-removal` under `rhai-llm-calls-k15` meets the "Done when" of that brief. Cut more than one leaf if the work does not fit one session.
- A leaf owns the Taskfile task for a Rhai upgrade. The task runs the engine tests and the hostile classes.
- A leaf in `rhai-harness-runs` owns the opt-in smoke tasks. They make one real LLM call through Claude Code and one through Codex, unconfined, with the owner's sign-in. The default test run never includes them.
- Each live leaf or brief that names `ravel::llm`, an LLM endpoint, a fake provider endpoint, Rig or an LLM budget matches the spec. At design time these files did: `.grove/13-k22/_rhai-harness-runs.md`, `.grove/13-k22/02-impl--rhai-stop-order-k24.md`, `.grove/14-impl--rhai-example-policies-k25.md`, `.grove/15-k26/_rhai-book-use.md`, `.grove/15-k26/01-draft--rhai-book-use-k27.md`, `.grove/15-k26/02-draft--rhai-book-use-k28.md`, `.grove/16-k29/03-impl--typescript-delivery-removal-k32.md`, `.grove/17-k34/02-draft--rhai-book-inside-k36.md` and `.grove/20-impl--owner-policy-conversion-k39.md`. Search again, because the list ages.
- The book leaf that owns the recipes of an LLM call through Claude Code and through Codex says so.

## Notes

- `llm.rhai` becomes an example that asks a model through a harness run, with a fake harness in its test.
- The owner's local configuration gains harness entries, not endpoints. Ravel 1.1.0 refuses an unknown settings key, so the conversion adds no new key until the driver restarts.
