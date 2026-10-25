# rhai-llm-adapter-proof-k54

## Goal

Build the LLM adapter for the `anthropic` API type, with the bounded response body, and prove that Rig and its TLS stack build for every archive target. No host function exists yet. The adapter takes an endpoint entry, a credential and a checked request. It returns a response or a classified failure. Tests at the adapter seam show each bound with a deterministic local HTTP server.

## Context

- The input is `docs/specs/policy-evaluation.md`. Read "LLM calls", "LLM endpoints and credentials" and "Rig upgrades". The node brief `rhai-llm-anthropic-k16` holds the full done conditions. This leaf covers the adapter half.
- All use of Rig stays in one module, `crates/ravel/src/llm`. Nothing outside it names a Rig type. The module is not yet called from the worker or the front, so it carries an `expect(dead_code)` that the next leaf removes.
- Ravel implements the `HttpClientExt` interface of Rig over a reqwest client. `rig-reqwest` is not used, because its `into_response` and `non_success_status_error` collect a body without a bound.
- The fake server lives in `crates/ravel/tests/support/llm_server.rs`. The adapter tests include it by path, and the next leaf uses it from the CLI seam.
- Record each Rig API call that the adapter uses in the running log, with the file and the line in the pinned source (0.45.0).

## Done when

- `task release:smoke` builds and smoke-tests all three archive targets with the new dependencies in the tree. If a target fails, stop and escalate with the target, the error and the options. Do not choose another provider without the owner.
- The module has one call: an endpoint entry, a credential and a checked request go in, and a response or a classified failure comes out. The response has `text`, `finishReason`, `usage`, `reportedModel`, `responseId` and `requestId`. The failure has `code`, `message`, `retryable`, `httpStatus` and `retryAfterMs`.
- The request maps onto `CompletionRequest` as the spec says. `on_unsupported` stays unset, and no tool, document or provider option is sent.
- The failure codes `endpoint_unavailable`, `unsupported_parameter`, `authentication`, `rate_limited`, `provider_error`, `network`, `timeout`, `malformed_response`, `content_filtered` and `output_limit` have a test each. `unknown_endpoint` and `model_not_offered` belong to the host function.
- The client ignores ambient proxy variables, follows no redirect, makes no retry, uses rustls with `ring` and the platform trust store, and accepts a base URL that is HTTPS or HTTP to a loopback address.
- A successful body has at most 8 MiB, counted as the bytes arrive. A declared length beyond the bound fails at once. A body without a declared length stops at the first byte beyond the bound, and the fake server sees its connection close. Ravel reads at most 64 KiB of a failure body and keeps the class of its status.
- The credential reaches the fake server only in the header of the API type. No error text, `Debug` output or message holds it.
- `task check` and `task test` pass.

## Notes

- The next leaf, `rhai-llm-anthropic-call`, adds `ravel::llm`, the checks of the request, the notice and the tests at the CLI seam.
- `rhai-llm-bounds` bounds the answer text and the stall. Keep the adapter so that both can plug into it.

## Decisions (running log)

- **`rig-reqwest` is not a dependency.** Ravel implements `HttpClientExt` itself (`crates/ravel/src/llm/transport.rs`), so the bound applies to every body before Rig sees it. `rig-reqwest`'s `into_response` and `non_success_status_error` collect without a bound. The dependencies are `rig-core` and `rig-http` at `=0.45.0` with default features off, plus `reqwest` 0.13 and `rustls` 0.23. The node brief lists `rig-reqwest` among the pins. `rhai-llm-upgrade-proof` should pin only the crates the build uses, and add `rig-cassette` as a development dependency.
- **TLS is `ring`.** reqwest 0.13's `rustls` feature brings `aws-lc-rs`, so Ravel enables `rustls-no-provider` and installs `rustls::crypto::ring::default_provider()` once, before the first client is built (`transport.rs`, `install_provider`; https://docs.rs/reqwest/0.13.5/reqwest/tls/index.html). `cargo tree` shows no `aws-lc-rs`. The trust store is the platform's, through `rustls-platform-verifier`, which reqwest uses by default.
- **No decoding, so the bound counts wire bytes.** The reqwest features `gzip`, `brotli`, `deflate` and `zstd` stay off, so the client sends no `Accept-Encoding` and decodes nothing. The test `a_call_sends_the_messages_wire_and_returns_the_answer` checks the absent header.
- **Redirect, proxy, retries, pool.** `Policy::none()`, `no_proxy()`, `retry::never()` and `pool_max_idle_per_host(0)`. The last one means no connection outlives its call, so the closing of the body closes the connection. Tests: a 302 whose target server sees no request, and an ambient `HTTP_PROXY` that sees none.
- **The error travels as a `Fault`.** A transport fault goes through Rig inside `http_client::Error::Instance`, and `classify` finds it again with `downcast_ref`. This keeps the classification independent of how Rig words its errors.
- **A declared length beyond 8 MiB fails from the head alone** (`transport.rs`, `content_length`). A body with no declared length counts chunks as they arrive and stops at the first chunk that passes the bound. A failure body is read to 64 KiB and the response is dropped, which closes the connection.
- **`Truncated` is not retryable.** Rig reports `ProviderError::Truncated` for a whole body that holds no Messages answer (for example `{"unexpected":true}`). The transport reports a real short body as `Fault::ShortBody`, so only that one is the retryable `malformed_response`.
- **A refusal is `content_filtered`.** Rig's Messages decoder sets finish `ContentFilter` and an `error` text together (`providers/anthropic/streaming.rs:133`), so the adapter reads the finish before the error.
- **Rig defaults the thinking option by model.** For `claude-sonnet-5-5` with no `reasoning`, Rig sends `thinking: {"type": "adaptive", ...}`. Ravel sets none of it. A script turns it off with `reasoning: "off"`, and Rig sends a different `thinking` object then.
- **`unsupported_parameter` needs a `reasoning` option.** Rig checks a request against its catalog only when the options are not all default, and `temperature` alone does not turn the check on. The test uses `claude-opus-4-5` with `xhigh`, which the catalog does not list for that model.
- **Credential redaction.** `redact` removes the credential from every failure message, because a reply body can echo it and the message reaches the script. The 401 test echoes the key and checks it is gone.
- **The fake server is shared by path.** `tests/support/llm_server.rs` has no dependency on the crate, and the unit tests include it with `#[path]`. The next leaf includes it from the integration tests with `mod`.
- **`rig-core` turns on `serde_json/preserve_order` for the whole build.** Its manifest enables the feature unconditionally (seen with `cargo tree -e features -i serde_json`), and feature unification applies it to Ravel. Every JSON object now keeps insertion order instead of sorted order. Two tests failed. One compared the keys of the request in order. The other showed the contract at risk: the SHA-256 of a delivered context is "the sorted encoding" (`context.rs`, `Delivered`), and it silently became the encoding in the order the caller's document arrived. `context::deliver` now sorts the keys of every object itself (`sorted`), so the digest no longer depends on a feature that a dependency can flip. The new test `the_digest_does_not_depend_on_the_order_the_keys_arrived_in` fails without the sort (seen). Notices and frames now list their keys in the order the code wrote them, which no test or document constrains. The upgrade-proof leaf should keep an eye on any new place that hashes or compares a JSON encoding.
