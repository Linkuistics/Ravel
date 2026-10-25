# rhai-llm-budgets-k18

## Goal

Enforce the evaluation budgets for LLM calls and output tokens. A policy cannot raise a budget, and it cannot catch its exhaustion. The accounting stays safe when an endpoint reports no usage.

## Context

- The input is "Evaluation budgets and their composition" in `docs/specs/policy-evaluation.md`. The `budgets` settings key and `ravel::remaining()` exist after `rhai-settings-discovery`.
- The review of `rhai-policy-k4` checked the accounting. Rig keeps missing token counters as `Option<u64>`, so the conservative debit of the spec is implementable.
- The adapter and the host function exist after `rhai-llm-anthropic`.

## Done when

- The default budgets are 8 LLM calls and 32,768 output tokens. The owner sets them in `budgets.llmCalls` and `budgets.llmOutputTokens`.
- A call that Ravel sends uses one LLM call. It then uses its reported output tokens, or its effective `maxTokens` when the endpoint reports none.
- A call that fails before Ravel sends it uses no budget. The failures are `unknown_endpoint`, `endpoint_unavailable`, `model_not_offered` and `unsupported_capability`.
- Ravel lowers `maxTokens` to the output tokens that remain.
- A call with no LLM call or no output token left ends the evaluation as `llm_budget_exhausted`, exit 3, before Ravel sends it. A script cannot catch it. The host latch refuses even when the script swallows the error.
- When the whole deadline passes during a call, the evaluation ends and not only the call. The deadline of a call is the smaller of its `timeoutMs` and the time that remains.
- `ravel::remaining()` and the `limits` map report the live values.
- The notice lists the effective `bounds`, including `llmCalls` and `llmOutputTokens`.
- Tests at the CLI seam cover the scenario "Caught budget breach", where a policy wraps a ninth call in `try`. They also cover usage present and usage absent, a token budget that runs out, and a failed call that uses no budget.
- `task check` and `task test` pass.

## Notes

- A nested session in a managed process has budgets of its own. The budgets belong to one evaluation, which is one `start-session`.
