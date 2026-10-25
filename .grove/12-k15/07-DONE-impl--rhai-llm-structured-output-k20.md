# rhai-llm-structured-output-k20

## Goal

Let a policy ask for structured output with a `schema`. Ravel sends the schema natively where the endpoint supports it, and it checks the answer itself. The script receives a checked value in `json`.

## Context

- The input is "LLM calls" in `docs/specs/policy-evaluation.md`, from the field `schema` to the paragraph about Rig's rewrite of the schema.
- Rig 0.45.0 drops `output_schema` with only a warning on a Chat Completions dialect without structured output. Ravel therefore never sends a schema to such an endpoint.
- Rig rewrites the schema on each wire. It marks every property as required and adds `additionalProperties: false` to each object that does not set the keyword. Ravel checks the answer against the schema of the script, not against the rewritten schema.
- The review of `rhai-policy-k4` read the sanitiser in `rig-core` 0.45.0, `src/providers/internal/schema.rs`. Read it again at the pinned version.
- All three API types and their fake endpoints exist after the earlier leaves.
- `rhai-llm-budgets` debits a call only when `Code::was_sent` says Ravel sent it. The new code `unsupported_capability` fails before the send, so add it to the codes that say no, and add a seam case that shows the budget stays whole.

## Done when

- The schema subset holds. The keywords are `type`, `properties`, `required`, `additionalProperties`, `items`, `enum`, `title` and `description`. Any other keyword, or a nesting deeper than 32 levels, ends the evaluation as `host_request_invalid`.
- `schema` becomes `output_schema` of Rig on an endpoint that supports structured output. This covers `anthropic`, `openai`, and `openai-compatible` with `structuredOutput` set to true.
- A schema for an `openai-compatible` endpoint without `structuredOutput` fails the call as `unsupported_capability`. The fake endpoint receives nothing, and the call uses no budget.
- Ravel parses the whole answer, trimmed, as JSON and checks it against the schema. A mismatch fails the call as `malformed_response`, with the location in the message. A match returns the value in `json`.
- An endpoint can refuse a schema that the subset admits, such as one with `additionalProperties: true`. The call then fails as `provider_error` with the status of the reply, and it selects no fallback.
- Tests at the CLI seam cover the scenario "Unsupported capability". They also cover a matching answer and a mismatching answer with a location. The other cases are an answer that is not JSON, a forbidden keyword, a deep nesting and the provider refusal. The edge cases of the validator follow. A required property is missing. An extra property meets `additionalProperties: false`. An `enum` or `items` check mismatches. Nesting reaches 32 levels and 33 levels. Each case runs on the wires that apply.
- The fake endpoint checks the place of the schema in each request body.
- `task check` and `task test` pass.

## Notes

- A schema checks shape only. The script decides what the value means.
- Keep the validator inside the adapter or beside it. Test it only at the CLI seam, as the spec says, because the spec excludes a second in-process seam. A policy sends each edge case through the fake endpoint, and the answer decides the result. This leaf adds no unit test of the validator.

## Decisions (running log)

- **The schema is checked twice, with one parser.** `llm/schema.rs` parses the subset into a tree and keeps the JSON the script wrote. The worker calls it while it builds the `host` frame, so a forbidden keyword ends the evaluation as `host_request_invalid` with the field and the line, such as `ravel::llm(schema.properties.a.format)`. The front calls it again when it decodes the frame, and a failure there is a protocol break. The validator sits beside the adapter in `llm/` and has no unit test, as the notes ask. The new file `tests/rhai_llm_schema.rs` tests it through the fake endpoint only.
- **The root schema is level 1.** A schema of 32 nested schemas passes, and one of 33 ends the evaluation. A level is a `properties` member or an `items` schema. The spec says so now. Both ways of nesting are tested at 32 and 33 levels on all three wires.
- **A keyword applies to the kinds it describes.** `required` and `properties` say nothing about an array, `items` says nothing about an object, and `type: integer` accepts `3.0`. An `enum` compares numbers by value, so `1` matches `1.0`. A message names a location in the style of the converter (`json.items[1].name`) and the kinds found, never a value of the answer. A key of the answer is cut at 64 characters.
- **Rig 0.45.0 was read, not remembered.** `output_schema` is a `schemars::Schema` (`completion/request.rs:1230`), and `rig_core::schemars` re-exports the crate, so no new dependency was needed. Each wire runs `structured_output_schema` (`providers/openai/mod.rs:71`) and the sanitiser in `providers/internal/schema.rs`. The sanitiser overwrites `required` with every property name and adds `additionalProperties: false` where the keyword is absent. It leaves `additionalProperties: true` as written, so the provider refusal case reaches the wire. The module comment of the adapter carries each file and line. The seam test reads the schema from `output_config.format.schema`, `text.format.schema` and `response_format.json_schema.schema`, and it finds the rewrite on each.
- **`unsupported_capability` comes after the credential and before the send.** `Adapter::call` reads the credential first, so an unset variable still reports `endpoint_unavailable`, which is the order of the table. `Code::was_sent` says no to the new code. A seam case with a budget of one call and one token shows the budget whole after the refusal, and the next call spends it.
- **The compatible dialect backs the check.** `COMPATIBLE_PLAIN` is `COMPATIBLE` with `without_response_format`, and the adapter uses it for an endpoint without `structuredOutput`. Rig would drop a schema there with only a warning (`providers/openai/wire/chat.rs:293`). The check fails the call first, so the dialect only guards a later change. The mutation that removes both checks fails three seam cases, because the server then receives a request.
- **A mismatch is a sent call.** It fails as `malformed_response`, not retryable, and it costs the effective `maxTokens` like any failure, because a failure carries no usage. The spec table says the same. Carrying the usage on the failure would be a later change to all failures at once.
- **An answer nested past 100 levels fails the call.** The reply to the worker nests the value three levels down, and the worker reads no frame deeper than 128. serde_json reads an answer of 127 levels, so such an answer would have broken the protocol. The check uses `convert::check_depth`, the bound of the converter, and the spec table lists it under `malformed_response`. A level of exactly 100 passes. The test reads that value as `response.json.len()`, because encoding it again inside a map passes the limit of `json_encode`.
- **`json` is `()` when the matching answer is `null`.** The spec now says so. A script that must tell the two apart reads `status`.
- **The check reads the redacted text.** An endpoint that echoes the credential in a key or a value of the JSON shows `[redacted]` in `json` too, and a seam case shows it.
- **A large answer crosses the protocol.** An array of 400,001 zeros (about 0.8 MiB of text) arrives in `json` on all three wires, with the text beside it.
- **Mutations seen to fail.** No capability check and the plain dialect removed: three cases. `MAX_LEVELS` of 33: the 33-level case. A mismatch ignored: five cases. `UnsupportedCapability` counted as sent: the budget case. `required`, `additionalProperties: false`, `enum` and `items` each ignored: the table of mismatches, which is one test for all of them.
- **Old tests moved.** The two cases that expected `schema` to be an unknown field changed. One lists `schema.pattern` among the malformed requests, and the OpenAI file lost its copy.
- **No in-session reviewer.** The claims the compiler cannot check are the wire placement of the schema and the validator. The seam reads the placement from the body of every request, and the mutations above cover the validator.
- **The spec has 8 older findings from the technical-communication check.** This leaf added none. The Taskfile needed no change: `task check` and `task test` run the new file.
- **For `rhai-llm-upgrade-proof`.** The scripted structured-output case exists for each API type in `tests/rhai_llm_schema.rs`, and the unsupported-capability scenario is there too. The sanitiser rewrite is a wire detail of Rig, so a replay of a real exchange should show it.
