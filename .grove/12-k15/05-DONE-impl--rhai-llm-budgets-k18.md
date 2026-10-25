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
- A call whose answer text passes 1 MiB fails as `output_limit` after Rig decoded the answer, so the endpoint reported usage and spent the tokens. `Failure` carries no usage today. The accounting must debit the effective `maxTokens` for such a call, or carry the usage on the failure. See the running log of `rhai-llm-bounds-k17`.

## Decisions (running log)

- **The ledger lives in `Host`.** `calls_left` and `tokens_left` start at the bounds, and `ravel::remaining()` reads them. A call that finds either at zero returns `Answer::End` with `llm_budget_exhausted`. The worker then hears nothing, so `try` around the call catches nothing, which is the same latch that `too_many_sources` uses. The check comes before every other check of the call, so a ninth call to an unknown endpoint ends the evaluation too. The alternative was to fail cheap checks first as data. It would have needed the credential check pulled out of `Adapter::call`, and a script that has spent its budget gains nothing from a probe of an endpoint.
- **A call is debited only when it was sent.** `Code::was_sent` says no for `unknown_endpoint`, `endpoint_unavailable`, `model_not_offered` and `unsupported_parameter`. A client that cannot start is also unsent, and `attempt` returns that fact beside the outcome, because it carries the code `network`. `network` itself counts as sent: the code cannot tell a request that never left from one cut mid-flight.
- **`unsupported_parameter` uses no budget, and the brief did not list it.** The brief and the spec named four codes, with `unsupported_capability` among them, which no leaf has added yet. Rig refuses an unsupported option before it opens a connection (the existing seam test asserts zero requests started), so the spec principle "fails before Ravel sends it" covers it. The spec now says so. When `rhai-llm-structured-output` adds `unsupported_capability`, it must add it to `Code::was_sent`.
- **A sent call that failed costs its effective `maxTokens`.** `Failure` carries no usage, and Rig drops the usage of an error path, so a 429, a timeout and an answer text past 1 MiB all report none. The brief offered two ways for the last case: carry usage on the failure, or debit `maxTokens`. Debiting `maxTokens` for every sent failure is one rule with no new field, and it errs on the side of the owner's money, because an endpoint can bill a request that Ravel fails or abandons. The spec records the rule.
- **`maxTokens` is lowered before the send.** `min(requested, tokens_left)` goes to the wire, and the debit uses that effective value when the endpoint reports no usage. A reported count above what remains leaves zero, not a wrap.
- **The deadline needed no new code.** `Watch::until` in `serve.rs` already races every host call against the whole deadline, and the deadline of a call (`deadline_of`) is never later than it, so the watch wins a tie and the evaluation ends as `selection_timeout`. The stall tests of `rhai-llm-bounds` already ask for a call of a minute inside an evaluation of a second. I added no test and no defensive wait for a tie that the ordering rules out.
- **`limits` keeps the budgets of the evaluation, and `remaining()` reports what is left.** `request.limits` is built before the first call and a script cannot change it, so the brief's "live values" reads as: both report the owner's values, not constants, and `remaining()` counts down. The test pins both.
- **The notice `bounds` gets the three budgets, with origins.** `Budgets` now carries where each value came from (`default` or `settings.json`) and `Budgets::bounds()` renders them as `llmCalls` (`calls`), `llmOutputTokens` (`tokens`) and `harnessRuns` (`runs`). `run.rs` adds them for the Rhai path only. The 1.x `bounds` stays as it was. The refusal for a breach carries the same bound, so its `bound` names the budget and says who set it. `harnessRuns` is in the notice already because the spec lists it, though no leaf spends it yet.
- **Tests at the CLI seam**, in `tests/rhai_llm.rs`: the ninth call in `try` (Caught budget breach), an owner budget of 2 and of 0, usage present, usage absent, a token budget that runs out with the wire `max_tokens` lowered each time, failures before the send (one call and one token of budget, whole at the end), an unset credential, sent failures (429 and a text past 1 MiB), the notice bounds, and the line of the breach. The fake server gained `Reply::messages_reporting` and `Reply::messages_without_usage`.
- **Mutations seen to fail.** The exhaustion check replaced by `false` fails four tests. Debiting unsent calls fails two. Removing the lowering fails the token test. Ignoring `maxTokens` as the fallback debit fails three.
