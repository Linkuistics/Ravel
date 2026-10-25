# rhai-llm-removal-k63

## Goal

Remove `ravel::llm` and everything that exists only for it. That is the LLM adapter, Rig, its HTTP client and TLS stack, the two LLM budgets, and the notice members for LLM calls. The product keeps working. A policy asks a model through `ravel::run_harness` instead.

## Context

- The input is the brief of `rhai-llm-calls-k15`, and the sections "LLM calls through a harness", "Evaluation budgets and their composition" and "Errors and refusals" in `docs/specs/policy-evaluation.md`.
- The leaves `rhai-llm-adapter-proof-k54`, `rhai-llm-anthropic-call-k55`, `rhai-llm-bounds-k17`, `rhai-llm-budgets-k18` and `rhai-llm-openai-types-k19` built the code that goes. Read their running logs to find every place they touched.
- The code to remove is the module `src/llm`, the worker side `rhai_worker/llm_call.rs`, the tests `tests/rhai_llm.rs` and `tests/rhai_llm_openai.rs`, and the fake provider server and seam under `tests/support`. Search for the rest, because the list ages.
- `rhai-json-check-k62` moved the schema checker out of `src/llm`. Keep it. Remove only the old tests that remain.
- `rhai-endpoints-removal-k64` follows. It removes the `endpoints` settings key and `ravel::endpoints()`. This leaf keeps both, and the product works with them unused.
- `excluded_grant` keeps the names that 1.x excludes, such as `NODE_OPTIONS`. Only its clause for an `apiKeyEnv` goes, and it goes with the endpoints. Leave it for the next leaf.

## Done when

- `ravel::llm`, the budgets `llmCalls` and `llmOutputTokens`, and the refusal `llm_budget_exhausted` are gone from the code, the tests and the notice. A settings file with `budgets.llmCalls` refuses as an unknown budget.
- The notice of the Rhai path keeps schema version 2. `llmCalls`, `worker.rigVersion` and the LLM members of `bounds` are gone.
- The LLM adapter is gone, with its HTTP client and its body bounds. Rig, reqwest and rustls are gone from `Cargo.toml` and `Cargo.lock`. A dependency that only they needed goes too. Prove each removal with `cargo tree`.
- The tokio runtime stays only if the worker front or the reader still needs it. Record which crates remain and why.
- The cross-build proof for `ring` in `rhai-llm-adapter-proof-k54` has no subject. Remove any script, task or note that exists only for it.
- The `endpoints` key and `ravel::endpoints()` still work, and a test still covers them.
- The tests of the scenarios "LLM-assisted selection", "Another purpose", "A structured answer that does not match", "A failed call and a fallback" and "Leak probes" are gone with the fake provider. `rhai-llm-call-scenarios-k67` rebuilds them through `run_harness`. Name the removed tests in the running log, so that leaf can match them.
- The TypeScript path works as before. `task check` and `task test` pass.

## Notes

- Check the claim that nothing remains with a sweep. Enumerate every token that matches `llm`, `rig`, `reqwest`, `rustls` and `ring` in tracked files, then classify each one. Run a control that finds a known hit.
- A dead-code lint can fire on an endpoint field after the call goes. Remove only what the lint names, and leave the rest to `rhai-endpoints-removal-k64`.
- The book and the spec describe the final state already. Do not edit them here, except an excerpt that moved.
- If the leaf proves too big, decompose it. A natural cut is the tests first, then the adapter and the dependencies.
