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
