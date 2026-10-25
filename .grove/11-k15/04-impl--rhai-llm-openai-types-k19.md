# rhai-llm-openai-types-k19

## Goal

Add the `openai` and `openai-compatible` API types to `ravel::llm`. The owner can then point a policy at the OpenAI service, or at any compatible endpoint that the owner names by URL.

## Context

- The input is the table of API types in "LLM endpoints and credentials" of `docs/specs/policy-evaluation.md`.
- The `openai` type uses the `OpenAIConfig` of Rig with the OpenAI dialect, on the Responses wire. Its default base URL is `https://api.openai.com/v1`.
- The `openai-compatible` type uses `OpenAIConfig` with a Ravel dialect that comes from the baseline quirks of Rig. It uses the Chat Completions wire. The base URL is required, and the credential variable is optional.
- Read the source of `rig-core` 0.45.0 for the dialect and the wire details. Do not rely on memory of the API.
- The adapter, the bounded transport and the budgets exist after the earlier leaves of this increment.

## Done when

- `ravel::llm` works for both new types, with the request fields and the response map that the first LLM leaf built. Each type sends its credential in its own header.
- The mapping to Rig is right for each wire. This covers `reasoning`, `maxTokens` and `temperature`. An option that the wire or the model cannot honour fails as `unsupported_parameter`.
- The classification of failures is the same on each wire. This covers `authentication`, `rate_limited` with `retryAfterMs`, `provider_error`, `content_filtered` and `malformed_response`. `finishReason` is `stop`, `length` or `other`.
- The `models` list of an endpoint works for every type. A model outside the list fails as `model_not_offered`.
- An `openai-compatible` endpoint without a credential variable works. Its `available` flag in discovery is true.
- The fake endpoint serves both wires with scripted responses. Tests at the CLI seam run these cases for each new type: success, and usage present and absent. The other cases are 401, 429 with a wait, 500, an undecodable body and a stall past the deadline.
- The bounded transport and the budgets apply to the new types without a change to their rules. A test shows an oversized body on each new wire.
- `task check` and `task test` pass.

## Notes

- Keep any difference between the wires inside the adapter. A policy sees one request and one response shape.
- Structured output stays for the next leaf. A request with a `schema` is still a `host_request_invalid` here.
