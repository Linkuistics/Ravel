# rhai-llm-anthropic-call-k55

## Goal

Add `ravel::llm` for the `anthropic` API type on top of the adapter of `rhai-llm-adapter-proof-k54`. A policy sends messages to a fake endpoint through the real Rig client and routes on the answer, at the CLI seam.

## Context

- The node brief `rhai-llm-anthropic-k16` holds the done conditions for this half: the request checks, the response map, the failure data, the notice and the CLI scenarios.
- The adapter, its bounded transport and the fake server in `crates/ravel/tests/support/llm_server.rs` exist after the first child. Read its running log for the Rig calls it uses and for any decision about the shape of its interface.
- The bridge, the host and the front follow the pattern of `read_text` and `env`: the worker sends a `host` frame, the front answers from the runtime, and the adapter call runs on the Tokio current-thread runtime that serves the worker.

## Done when

- Every condition of the node brief that the first child did not cover holds. That includes the request checks (`host_request_invalid`), `unknown_endpoint`, `model_not_offered`, the lowering of `timeoutMs` to the time that remains, the notice entries for calls, and the CLI scenarios the brief lists.
- The `expect(dead_code)` on the adapter module is gone.
- `task check` and `task test` pass.

## Notes

- This leaf does not count budgets, and it does not bound the answer text or the time of a stalled call. `rhai-llm-budgets` and `rhai-llm-bounds` own those.
- The adapter of `rhai-llm-adapter-proof-k54` was not reviewed. This leaf closes the node `rhai-llm-anthropic-k16`, so the session that finishes it should cut a `review-impl` leaf for the node: the bound on every body, the redirect and proxy rules, and the credential boundary are claims that the compiler cannot check.
- Read the running log of `rhai-llm-adapter-proof-k54`. It records that `rig-core` turns on `serde_json/preserve_order`, and that `context::deliver` now sorts its keys itself.

## Decisions (running log)

- **The worker checks, the front decodes.** `rhai_worker/llm_call.rs` holds both halves. The worker validates the request map and sends a normalised `llm` frame with the default `maxTokens` (1,024) applied. The front decodes it again and treats a bad frame as a broken protocol. The front answers with the whole response map in a `reply` whose status is always `ok`. A failed call is `status: "failed"` inside the map, so no `error` reply exists for `llm`.
- **A malformed request names a path like `messages[1].role`.** `host_request_invalid` reports `ravel::llm(messages[1].role)` and the line. The last role is checked after each role. `schema` is an unknown field until `rhai-llm-structured-output` adds it. An empty `endpoint` or `model` is malformed. An empty `content` is not, because the endpoint decides what it accepts.
- **`temperature` is a float.** The spec says Float, so `temperature: 1` ends the evaluation and `1.0` passes. Accepting integers is a one-line change in `llm_call.rs` if the owner wants it.
- **A request over the message bound is `host_request_invalid`.** The front reads no host frame past 1 MiB, and a script can build a larger request from large reads. The worker measures the frame and ends the evaluation with the field `request` rather than let the front call it a broken protocol. The existing `message_too_large` refusal speaks only of a `select` result.
- **`unknown_endpoint` and `model_not_offered` are `llm::Code` variants.** The host function builds them, so one `Failure` type serves every code. `Failure::new` is public to the crate. Both fail before the adapter is asked, so they need no HTTP client. The model list is the owner's own text and is safe to name in the message.
- **The HTTP client starts at the first call that sends.** `Host` keeps an `Option<Adapter>`. A policy that never calls pays nothing, and a platform with no TLS configuration fails the call as `network`, not retryable, rather than the evaluation. The credential comes from `std::env::var_os`, as `ravel::endpoints` reads it. A variable that is unset or empty is `endpoint_unavailable` and sends nothing.
- **The call deadline is `min(timeoutMs, remaining)`** (`deadline_of`, unit-tested). The seam cannot tell a lowered call deadline from the whole deadline, because both end the evaluation or the call at about the same time. The seam test shows the part it can: a call that asks for 60 s under a 1 s evaluation ends as `selection_timeout`, exit 124.
- **The notice has `llmCalls`.** Each entry holds `endpoint`, `model`, `status`, `errorCode`, `usage` and `elapsedMs`, and no text. The list is empty for a policy with no call. It is added to schema version 2 beside `reads`. `bounds` still lacks `llmCalls` and `llmOutputTokens`, which `rhai-llm-budgets` adds. `ravel::remaining()` still reports whole budgets, because no call counts yet, and its comment says so.
- **Rig refuses `temperature` on `claude-sonnet-5-5`.** The catalog says the model takes no `temperature`, and a request that also sets `reasoning` turns the check on, so the call fails as `unsupported_parameter`. The options seam test uses `claude-opus-4-5` for that reason. The adapter test with `xhigh` on the same model already covers the other path.
- **The fake server waits for the close.** The old fixed pause of 100 ms after 8 MiB passed in the adapter tests and failed at the seam: under load the client had not yet read past the bound, and the kernel held the whole 9 MiB, so the server saw no close. The server now writes the first piece past 8 MiB and then waits up to 5 s for the client to close. A client that keeps reading ends the wait and gets the rest. The test failed before this change (seen), and it passes three runs in a row after it.
- **Mutations seen to fail.** With the body bound at 16 MiB, the oversized, the declared-length and the malformed-request tests fail. With the last-role check removed, the malformed-request test fails. Both files are restored.
- **A risk for the review leaf.** The runtime thread builds the client and runs the TLS handshake. On Linux, `rustls-platform-verifier` can read the system trust store at the first handshake, which is a filesystem call on a thread that the spec says makes none. The seam uses `http` to loopback, so no test reaches it. The review should decide whether the client belongs on the reader thread.
- **Not done here, as the brief says.** No budget counts, the answer text has no bound, and a stalled call is bounded only by its deadline. `openai` and `openai-compatible` endpoints fail the call as `provider_error` until `rhai-llm-openai-types`.
