# rhai-llm-anthropic-k16

## Goal

Make `ravel::llm` work end to end for the `anthropic` API type. A policy sends messages to a fake endpoint through the real Rig client and routes on the answer. The leaf also proves that Rig and its TLS stack build for every archive target.

## Context

- The input is `docs/specs/policy-evaluation.md`. Read "LLM calls", "LLM endpoints and credentials", "Rig upgrades" and the scenarios of the requirements "Generic LLM interaction", "Endpoint failures are data" and "Credential boundary".
- The settings keys and the credential name rules exist after `rhai-settings-discovery`. The bridge and the reader thread exist after `rhai-measured-reads`.
- The TLS crypto provider must build for all three archive targets with the existing Zig glibc 2.17 toolchain. The design prefers `ring`. Prove this with `task release:smoke` before anything relies on it.
- If the build fails on a target, stop and escalate. State the target, the error and the options. Do not choose another provider without the owner.
- Ravel builds its own HTTP client. It implements the `HttpClientExt` interface of Rig over a reqwest client that Ravel configured, and it connects the Rig client with `connect`.
- If the leaf proves too big, decompose it. A natural first child adds the dependencies, the adapter skeleton and the build proof. A second child adds the host function and the tests.

## Done when

- `task release:smoke` builds and smoke-tests all three archive targets with the new dependencies linked in.
- `ravel::llm(request)` accepts `endpoint`, `model`, `messages`, `maxTokens`, `temperature`, `reasoning` and `timeoutMs`. It accepts no `schema` yet. An unknown field, a bad role or a last role other than `user` ends the evaluation as `host_request_invalid` with the field and the line.
- The request maps onto Rig's `CompletionRequest` as the spec says. Ravel leaves `on_unsupported` unset, and it sends no tools, no documents and no provider options.
- A successful call returns the response map of the spec: `status`, `endpoint`, `model`, `text`, `json`, `finishReason`, `usage`, `reportedModel`, `responseId`, `requestId` and `elapsedMs`.
- A failure returns `status: "failed"` with `error` holding `code`, `message`, `retryable`, `httpStatus` and `retryAfterMs`. This leaf covers `unknown_endpoint`, `endpoint_unavailable`, `model_not_offered`, `unsupported_parameter`, `authentication`, `rate_limited`, `provider_error`, `network`, `timeout`, `malformed_response` and `content_filtered`.
- Ravel makes no retry. The HTTP client ignores ambient proxy variables and follows no redirect. It switches off the default retries of reqwest. It uses rustls with the trust store of the platform. A base URL is HTTPS, or HTTP to a loopback address.
- Only the Ravel CLI reads the credential variable, at the time of each call. Rig carries it as a `Secret`. It reaches only the base URL of its endpoint, in the header of the API type.
- Messages and answers stay out of the notices and the logs. The notice lists each call with its endpoint, model, status, error code, usage and elapsed time.
- The per-call `timeoutMs` is lowered to the time that remains. A call that reaches its own deadline fails as `timeout`.
- A deterministic local HTTP server lives in `crates/ravel/tests/support`. It plays scripted responses, and it checks the path, the credential header, the model, the token limit and the body of each request. It reports whether a connection closed.
- Tests at the CLI seam cover these scenarios: "LLM-assisted selection", "Another purpose", "Wrong role", "Rate limit and fallback" and "Leak probes". Further cases are 401, 500, an undecodable body and a body that stops short. The last cases are a per-call timeout, a redirect that Ravel does not follow, and an unsupported option.
- `task check` and `task test` pass.

## Notes

- This leaf does not bound the response body, and it does not count budgets. `rhai-llm-bounds` and `rhai-llm-budgets` do. Keep the adapter so that both can plug into it.
- A body that stops short is the one `malformed_response` that is retryable.
- Record each Rig API call that the adapter uses in the running log. Give the file and the line in the pinned source.
