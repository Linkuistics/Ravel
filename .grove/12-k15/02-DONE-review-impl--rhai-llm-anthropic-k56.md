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

## Decisions (running log)

- The review covers producer commits `d2ea16b8` and `40140865`, against the source at `40140865`. It is an inspection-only review: no test, build, lint or format command runs, and no production or test source changes. The graph CLI refused `list_projects` because a pre-coordination or unverified generation is active. No project, generation or coverage result was available; direct source reads and targeted searches supply the evidence instead.
- The credential boundary has a concrete counterexample. Only `Failure.message` is redacted. An endpoint can echo the credential in `request-id` on a 401 or in the text, model or ID of a successful Messages body, and those strings reach the worker unchanged. This is finding F1.
- The runtime-thread concern is a defect in client construction on Linux, rather than an unresolved question about the first handshake. reqwest 0.13.5 builds `rustls-platform-verifier` 0.7.1 synchronously, which loads native certificate files at construction. A caller's `SSL_CERT_FILE` pointing at an unwritten FIFO blocks the only runtime thread before the call timeout exists. This is finding F2.

## Findings

### F1 — P1: Endpoint echoes bypass credential redaction outside the failure message

**Location:** `crates/ravel/src/llm/mod.rs:273–280`, with the unsanitised fields populated at `:355–357`, `:396–410` and `:454`. `crates/ravel/src/rhai_worker/llm_call.rs:314–340` then serialises them into the reply to the worker.

**Breaking input:** Configure an Anthropic endpoint with credential `K`. Have it answer `401`, a body of `{"error":"bad key"}`, and the header `request-id: K`. The call returns `status: "failed"`, but its `requestId` is the complete credential. A second counterexample is a normal successful Messages reply whose `content[0].text`, `model` or `id` equals `K`. Each becomes, respectively, `text`, `reportedModel` or `responseId` in a successful response, without redaction. A successful reply with `request-id: K` leaks in the same way.

`Adapter::call` sanitises only `failure.message`. Its successful branch returns the entire response unchanged, and its failed branch leaves `failure.request_id` unchanged. Rig's `Secret` protects the outbound configuration; it does not know that an ordinary response field contains an echo of that secret. Once the response reaches the script, `print(ravel::json_encode(response))` can put the key in diagnostics, and the script can send it as a message to a different configured endpoint. This breaks the explicit credential boundary even though redirects are disabled and the outbound credential header is correct.

**Pinned-source evidence:** In `rig-core` 0.45.0, `providers/anthropic/wire.rs:128` chooses `request-id`. `driver/http_transport.rs:302–306` and `:319–333` preserve that header on non-successful and successful replies. `providers/anthropic/streaming.rs:451–455` copies `id` and `model` from the body, and `:496–501` emits them as response metadata. Ravel copies those fields into its response without sanitising them.

**Test gap:** The leak probes in `crates/ravel/src/llm/tests.rs:488` and `crates/ravel/tests/rhai_llm.rs:686` echo the key only in a 401 body, where the existing message redaction works. They do not exercise an echoed request ID or any successful response field. The integration needs evidence at the CLI seam that the entire response and its printed diagnostic form preserve the credential boundary for both outcomes.

### F2 — P1: TLS client construction can block the only runtime thread indefinitely

**Location:** `crates/ravel/src/rhai_worker/host.rs:324–335`; the synchronous operation is `crates/ravel/src/llm/transport.rs:84–93`.

**Breaking input:** On Linux, set the caller's `SSL_CERT_FILE` to a FIFO with no writer, unset `SSL_CERT_DIR`, and make the first otherwise valid `ravel::llm` call to an allowed endpoint. The endpoint can even be HTTP on loopback; reqwest constructs its TLS configuration regardless. Give the call a short `timeoutMs` and the evaluation a short whole deadline. `Adapter::new()` blocks in a normal `File::open` of that FIFO before any HTTP request is sent. The evaluation cannot poll its deadline, cancellation latch, worker health or output limit while that call is blocked.

The timeout is computed only after construction at `host.rs:336`, and `tokio::time::timeout` is entered later in `llm/mod.rs:262`. Neither can interrupt synchronous filesystem work on the current-thread runtime. This violates the bridge contract at `docs/specs/policy-evaluation.md:300` and the promise that a long host operation leaves the stop machinery responsive. It is distinct from the later leaf's HTTP stall and in-flight cancellation work: the runtime is held before the network future starts.

**Pinned-source evidence:** reqwest 0.13.5 `src/async_impl/client.rs:758–760` invokes `rustls_platform_verifier::Verifier::new` inside `ClientBuilder::build`. On Linux, `rustls-platform-verifier` 0.7.1 `src/verification/others.rs:92` calls `rustls_native_certs::load_native_certs`. `rustls-native-certs` 0.8.5 `src/lib.rs:118–124` honours the certificate paths from the environment, and `:346–347` loads them through `CertificateDer::pem_file_iter`. `rustls-pki-types` 1.15.1 `src/pem.rs:50–55` performs an ordinary blocking `File::open`. Thus the producer's question has a concrete answer: client construction belongs off the runtime thread, using the reader's abandonable work path. Moving construction, rather than just wrapping the network call in another timeout, is the relevant boundary.

**Test gap:** The existing HTTP fixtures exercise the network timeout after client construction. None supplies a certificate source that blocks. The integration should establish that this source cannot prevent the whole deadline from stopping the evaluation.

## Review evidence and limits

These findings are source-derived counterexamples, not executed reproductions. No tests, builds, linters or formatters were run, as required for this review kind. The producer's task logs record body-bound and last-role mutations and repeated oversized-response checks; the commit descriptions state the archive build proof. This review does not independently certify those runs.

The inspected transport disables proxies, redirects and retries explicitly, measures successful bodies before Rig decodes them, and truncates the retained failure body. The worker validates and measures its LLM frame before sending it; the front continues to use the existing serial host-call protocol. These reads do not establish an exhaustive absence of other defects.

For the JSON ordering concern, `context::deliver` sorts objects recursively before hashing its encoding. The other inspected Ravel digest sites hash source bytes rather than JSON encodings. The data helper converts Rhai maps in their sorted iteration order before encoding, so the `preserve_order` feature does not change that path's member order. No additional ordering finding resulted from those inspected paths.
