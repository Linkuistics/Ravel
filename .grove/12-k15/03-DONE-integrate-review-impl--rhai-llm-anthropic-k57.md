# rhai-llm-anthropic-k57

**Integrates:** rhai-llm-anthropic-k56


## Goal

Triage the findings of `rhai-llm-anthropic-k56` against the implementation of `rhai-llm-anthropic-k16`, and apply the valid findings. Read the findings from the review's committed task file.



## Context

- The producer is the node `rhai-llm-anthropic-k16`, whose two implementation commits are `d2ea16b8` and `40140865`.
- Follow the producer's brief and the policy evaluation specification. Read the pinned dependency source before relying on a Rig or TLS API.
- This integration is immediately after the review so its source coordinates remain useful.

## Done when

- Each review finding has an explicit disposition supported by source evidence.
- Valid findings are fixed, with verification at the agreed CLI seam and the project's required checks.
- The implementation and its task notes describe the resulting behaviour and any remaining limits.

## Notes

## Dispositions

- **F1 (credential echo outside the failure message): a real issue, fixed.** Reproduced first with two adapter tests that failed on the unfixed code: a 401 with `request-id` set to the key, and a 200 whose `id`, `model`, text and `request-id` hold the key. `Adapter::call` now redacts `text`, `reportedModel`, `responseId` and `requestId` of a response, and `message` and `requestId` of a failure (`crates/ravel/src/llm/mod.rs`). A seam test (`an_endpoint_that_echoes_the_credential_in_any_field_of_an_answer_leaks_nothing`) prints and debugs both outcomes and checks the script's values, stderr and the report. The spec's client rules name the redaction.
- **F2 (TLS client construction blocks the runtime thread): a real issue, fixed.** `Host::attempt` builds the `Adapter` with `reader.run(Adapter::new)`, so the whole deadline abandons a blocked trust-store read like any other read (`crates/ravel/src/rhai_worker/host.rs`). This answers the open question of `rhai-llm-anthropic-call-k55`: the client belongs on the reader thread. The spec's client rules record it.
- The JSON-ordering check of the review found nothing, and nothing changed.

## Verification and limits

- `task check` and `task test` pass: 601 tests, plus clippy and fmt.
- The test `a_trust_store_that_blocks_cannot_hold_the_whole_deadline` points `SSL_CERT_FILE` at a FIFO with no writer. It runs on Linux only, because macOS reads no certificate file. It did not run in this session, which ran on macOS, so F2 has no executed proof. The fix rests on the pinned-source reading in F2 and on the existing reader-abandonment test in `rhai_worker/serve.rs`.
- A reader thread stuck on that read also holds later reads. The whole deadline still ends the evaluation, and the spec already states that the kernel can delay the exit of the Ravel CLI.
- Redaction removes the exact credential string. An endpoint that splits or transforms the key in its reply is out of reach, as it was for the failure message.
