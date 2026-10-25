# llm-harness-replan-k60

## Goal

Cut the work that the design `llm-calls-through-harnesses-k58` implies, and bring the later leaves into line with it. The design moved every LLM call to a harness run of Claude Code or Codex, and it removed `ravel::llm` and Rig.

## Context

- Read the spec sections that the brief of `rhai-llm-calls-k15` lists. Read them after the review `llm-calls-through-harnesses-k59` and any integration of it.
- Leaves `rhai-llm-adapter-proof-k54`, `rhai-llm-anthropic-call-k55`, `rhai-llm-bounds-k17`, `rhai-llm-budgets-k18`, `rhai-llm-openai-types-k19` and `rhai-llm-structured-output-k20` built the code that goes. Leaf `rhai-settings-discovery-k14` built the `endpoints` key and `ravel::endpoints()`.
- The owner pruned `rhai-llm-upgrade-proof-k21`. It held the Taskfile task for a Rhai upgrade, which still needs a leaf.
- The integration `llm-calls-through-harnesses-k61` changed some scenarios. The scenario "Deadline of one run" is new. "Output bound" now requires the policy to go on. "Leak probes" and "A harness finds its own sign-in" now share one ungranted sentinel. Give each of them a live leaf.

## Done when

- An `impl` leaf `rhai-llm-removal` under `rhai-llm-calls-k15` meets the "Done when" of that brief. Cut more than one leaf if the work does not fit one session.
- A leaf owns the Taskfile task for a Rhai upgrade. The task runs the engine tests and the hostile classes.
- A leaf in `rhai-harness-runs` owns the opt-in smoke tasks. They make one real LLM call through Claude Code and one through Codex, unconfined, with the owner's sign-in. The default test run never includes them.
- Each live leaf or brief that names `ravel::llm`, an LLM endpoint, a fake provider endpoint, Rig or an LLM budget matches the spec. At design time these files did: `.grove/13-k22/_rhai-harness-runs.md`, `.grove/13-k22/02-impl--rhai-stop-order-k24.md`, `.grove/14-impl--rhai-example-policies-k25.md`, `.grove/15-k26/_rhai-book-use.md`, `.grove/15-k26/01-draft--rhai-book-use-k27.md`, `.grove/15-k26/02-draft--rhai-book-use-k28.md`, `.grove/16-k29/03-impl--typescript-delivery-removal-k32.md`, `.grove/17-k34/02-draft--rhai-book-inside-k36.md` and `.grove/20-impl--owner-policy-conversion-k39.md`. Search again, because the list ages.
- The book leaf that owns the recipes of an LLM call through Claude Code and through Codex says so.

## Notes

- `llm.rhai` becomes an example that asks a model through a harness run, with a fake harness in its test.
- The owner's local configuration gains harness entries, not endpoints. Ravel 1.1.0 refuses an unknown settings key, so the conversion adds no new key until the driver restarts.

## Decisions (running log)

- **Four leaves cut the removal, in dependency order.** Under `rhai-llm-calls-k15`: `rhai-json-check-k62`, `rhai-llm-removal-k63`, `rhai-endpoints-removal-k64` and `rhai-upgrade-task-k65`. The schema checker has no dependency on Rig, so `ravel::json_check` lands first as an addition and `ravel::llm` keeps working. The call and its adapter go second. The endpoints go third, because the call reads them, and `endpoints.rs` also holds the harness parser, which stays. Each handoff leaves main green. One `rhai-llm-removal` leaf, as the brief named it, would have removed about 7,300 lines of code and tests. It would also have moved a module, removed three crates and reworked the notice and the settings. Six sessions built that code.
- **The Rhai upgrade task stays in `rhai-llm-calls-k15`.** The pruned leaf sat there, and the closed node `rhai-local-capabilities` cannot take a live leaf without reopening. The task uses only `rhai_*` test targets, so `typescript-removal-k29` does not remove it with `tests/hostile.rs`.
- **The scenarios of the LLM call need their own leaf, `rhai-llm-call-scenarios-k67`.** The tests of "LLM-assisted selection", "Another purpose", "A structured answer that does not match", "A failed call and a fallback" and "Leak probes" went with the fake provider. "A harness finds its own sign-in" is new. The leaf takes both credential scenarios, so that they share one ungranted sentinel. It sits in `rhai-harness-runs-k22`, after the stop order, because it needs `run_harness`.
- **The new and changed scenarios each have a live leaf.** "Deadline of one run" and "Output bound" went to `rhai-harness-run-k23`, which already owned the runs that stop at a bound. "Caught budget breach" went there too. "Leak probes" and "A harness finds its own sign-in" went to `rhai-llm-call-scenarios-k67`.
- **The smoke tasks are `rhai-harness-smoke-k66`.** They were a bullet in `rhai-stop-order-k24`. That bullet is gone. The new leaf comes last in the node. It spends subscription usage, and its argument vectors feed three later leaves. It asks the owner before the first real call.
- **The stale files now match the spec.** The sweep covered every live task file and brief, and it found these: the root brief, `rhai-llm-calls`, `rhai-harness-runs`, `rhai-stop-order`, `rhai-example-policies`, `rhai-book-use` with its two drafts, `rhai-book-inside` with its two drafts, and `owner-policy-conversion`. Each is edited. The stop order now has six steps, as the spec says, and the leaf had said seven.
- **The book leaf that owns the recipes is `rhai-book-use-k27`.** Chapter 4 gives the recipe of an LLM call through Claude Code and through Codex. The arguments come from the log of `rhai-harness-smoke-k66`. The brief of `rhai-book-use-k26` says so too.
- **No `review-planning` leaf was cut.** The cut is small, and every leaf ends in a green tree. The removal leaves can earn a `review-impl` leaf at their own last act.
- **Verification and its limits.** A sweep of the live tree for `ravel::llm`, endpoint, Rig, LLM budget, recording and related words now finds only statements of what was removed. The same pattern found the old text in the design leaf and in the pruned leaf, which shows that it sees them. Closed nodes keep their old text, as history. The prose checker reports no hard finding in the new leaves or the edited briefs. No code ran, because this session changes only the tree.
