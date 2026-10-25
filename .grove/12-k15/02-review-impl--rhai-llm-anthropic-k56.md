# rhai-llm-anthropic-k56

**Reviews:** rhai-llm-anthropic-k16

## Goal

Read the whole of the first live call for the `anthropic` API type, with fresh eyes, and report what is wrong. The node `rhai-llm-anthropic-k16` is the producer. Its two leaves are `rhai-llm-adapter-proof-k54`, which nobody reviewed, and `rhai-llm-anthropic-call-k55`.

## Context

- The producer's brief is `rhai-llm-anthropic-k16`. Its `Done when` is the contract. Read the running logs of both leaves for the decisions they record.
- The claims that the compiler cannot check, and that this review should try to break:
  - The bound on every body. A success stops at 8 MiB counted as the bytes arrive, and a declared length beyond it fails at once. A failure body is read to 64 KiB, and the connection closes.
  - The redirect and proxy rules. The client follows no redirect and ignores every proxy variable.
  - The credential boundary. It reaches only the base URL of its endpoint, in one header. No error text, notice, log, `Debug` output or reply to the script holds it. Try an endpoint that echoes it in a header, a body, a redirect target or a Rig error.
  - The worker and front protocol of `ravel::llm`. A script cannot make the front read a frame past its bound, hold two calls at once, or catch an end.
- One open question from `rhai-llm-anthropic-call-k55`: the runtime thread builds the HTTP client and runs the TLS handshake. On Linux, `rustls-platform-verifier` can read the system trust store at the first handshake. The spec says the runtime thread makes no filesystem call. Say whether the client belongs on the reader thread.
- Read the source of the pinned Rig version (`=0.45.0`) before any claim about an API. `rig-core` turns on `serde_json/preserve_order`, and `context::deliver` now sorts its keys itself. Check that nothing else hashes or compares a JSON encoding.
- The tests are `crates/ravel/src/llm/tests.rs` and `crates/ravel/tests/rhai_llm.rs`, with the fake server in `crates/ravel/tests/support/llm_server.rs`.

## Done when

- Each finding names a file and a line, the input that breaks it and what goes wrong. A finding with no input that breaks it is a question, and says so.
- No fix is written.

## Notes

- This leaf does not review budgets, the answer text bound or the stall bound. `rhai-llm-budgets` and `rhai-llm-bounds` own them and come after this leaf.
