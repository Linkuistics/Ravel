# rhai-llm-calls-k15 — brief

## Goal

Let a Rhai policy ask a model a question. This increment first built `ravel::llm` through Rig. The owner then moved every LLM call to a harness run of Claude Code or Codex, which uses the owner's own sign-in. The increment now removes `ravel::llm`, Rig and the endpoint settings. It keeps the schema check as the data helper `ravel::json_check`.

## Done when

- `ravel::llm`, `ravel::endpoints()`, the `endpoints` settings key, the two LLM budgets and `llm_budget_exhausted` are gone from the code, the tests and the notice.
- The LLM adapter is gone, with its HTTP client and its body bounds. Rig, reqwest and rustls are gone from the dependency tree.
- `ravel::json_check` applies the schema subset of the spec in the worker. Tests at the CLI seam cover a match, a mismatch with its location, a schema outside the subset and a value with no JSON form.
- The TypeScript path works as before, and `task check` and `task test` pass.

The harness run itself, and the smoke tasks for a real LLM call through each harness, belong to `rhai-harness-runs`.

## Decomposition

- `rhai-llm-anthropic`, `rhai-llm-bounds`, `rhai-llm-budgets`, `rhai-llm-openai-types` and `rhai-llm-structured-output` built `ravel::llm` through Rig. They are done.
- `llm-calls-through-harnesses-k58` (design) moved every LLM call to a harness run. It changed the spec, the ADR set and the glossary.
- The owner pruned `rhai-llm-upgrade-proof-k21`. Its Rig upgrade proof and its real recordings have no subject now.
- A `review-design` leaf reads the change of the spec before any leaf builds on it.
- A `planning` leaf, `llm-harness-replan`, then cuts the removal. It names `rhai-llm-removal` as the `impl` leaf that removes the built code and moves the schema check. It also brings the later leaves that still name `ravel::llm` into line with the spec.

## Pointers

- ADR: `docs/adr/a-policy-asks-a-model-through-a-harness-run.md`.
- Spec: `docs/specs/policy-evaluation.md`. The sections are "Data helpers", "LLM calls through a harness", "Credentials", "Evaluation budgets and their composition" and "Upgrades".
- The running log of `llm-calls-through-harnesses-k58` holds the owner's decisions.
- Glossary: LLM call, Harness run, Provider and Evaluation budget.
- Test seam: the real `ravel start-session` against a sandbox home, with fake harness scripts on a private `PATH`.

## Notes

- The schema checker that `rhai-llm-structured-output` built moves into the worker, where the data helpers run. Its tests move to the CLI seam with it.
- A schema outside the subset now throws `schema_invalid`, which the script can catch. `ravel::llm` ended the evaluation for it.
- `rhai-llm-adapter-proof-k54` proved that `ring` builds for the three archive targets. That proof has no subject once rustls is gone.
- `excluded_grant` stays for the names that 1.x excludes, such as `NODE_OPTIONS`. Only its clause for an `apiKeyEnv` goes.
- The notice of the Rhai path keeps schema version 2. `llmCalls`, `worker.rigVersion` and the LLM members of `bounds` go.
