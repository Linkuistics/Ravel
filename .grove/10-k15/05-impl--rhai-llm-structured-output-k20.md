# rhai-llm-structured-output-k20

## Goal

Let a policy ask for structured output with a `schema`. Ravel sends the schema natively where the endpoint supports it, and it checks the answer itself. The script receives a checked value in `json`.

## Context

- The input is "LLM calls" in `docs/specs/policy-evaluation.md`, from the field `schema` to the paragraph about Rig's rewrite of the schema.
- Rig 0.45.0 drops `output_schema` with only a warning on a Chat Completions dialect without structured output. Ravel therefore never sends a schema to such an endpoint.
- Rig rewrites the schema on each wire. It marks every property as required and adds `additionalProperties: false` to each object that does not set the keyword. Ravel checks the answer against the schema of the script, not against the rewritten schema.
- The review of `rhai-policy-k4` read the sanitiser in `rig-core` 0.45.0, `src/providers/internal/schema.rs`. Read it again at the pinned version.
- All three API types and their fake endpoints exist after the earlier leaves.

## Done when

- The schema subset holds. The keywords are `type`, `properties`, `required`, `additionalProperties`, `items`, `enum`, `title` and `description`. Any other keyword, or a nesting deeper than 32 levels, ends the evaluation as `host_request_invalid`.
- `schema` becomes `output_schema` of Rig on an endpoint that supports structured output. This covers `anthropic`, `openai`, and `openai-compatible` with `structuredOutput` set to true.
- A schema for an `openai-compatible` endpoint without `structuredOutput` fails the call as `unsupported_capability`. The fake endpoint receives nothing, and the call uses no budget.
- Ravel parses the whole answer, trimmed, as JSON and checks it against the schema. A mismatch fails the call as `malformed_response`, with the location in the message. A match returns the value in `json`.
- An endpoint can refuse a schema that the subset admits, such as one with `additionalProperties: true`. The call then fails as `provider_error` with the status of the reply, and it selects no fallback.
- Tests at the CLI seam cover the scenario "Unsupported capability". They also cover a matching answer and a mismatching answer with a location. The other cases are an answer that is not JSON, a forbidden keyword, a deep nesting and the provider refusal. Each case runs on the wires that apply.
- The fake endpoint checks the place of the schema in each request body.
- `task check` and `task test` pass.

## Notes

- A schema checks shape only. The script decides what the value means.
- Keep the validator inside the adapter or beside it. It needs no network, so unit tests may cover its edge cases. The CLI seam still proves the behaviour.
