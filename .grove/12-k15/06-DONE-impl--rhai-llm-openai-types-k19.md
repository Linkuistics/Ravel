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

## Decisions (running log)

- **One `send`, two Rig clients.** `Adapter::send` matches on the API type. `anthropic` keeps its client. `openai` and `openai-compatible` build an `OpenAIConfig` with `openai_config`, connect it to the same bounded transport, and call `completion`. Both return the same `CompletionResponse`, so `answer` and `classify` serve all three wires with no change to their rules. The only edit to `classify` is the text for `Truncated`, which named the Messages wire and now names none.
- **The routes are pinned.** `openai` uses `OpenAIConfig::new` with `with_route(Route::Responses)`. `openai-compatible` uses `OpenAIConfig::with_key` with `with_route(Route::Chat)`. Both defaults already hold in Rig 0.45.0, and the pin keeps a later Rig from changing the wire of an endpoint that the owner named. The module comment cites the file and the line of each Rig item.
- **The compatible dialect is `Dialect::gateway("openai-compatible", "", "")` with `request_id_header` set to `x-request-id`.** `gateway` gives `Quirks::openai()`, which the brief called the baseline. The base URL and the key variable of the dialect are empty, because Ravel never reads them: the settings give the base URL, and the adapter reads the credential. The header name is the one OpenAI documents, and compatible servers copy it.
- **`reasoning` fails on `openai-compatible`, at every level, including `off`.** Rig maps `reasoning` onto a Chat Completions server only for the dialects that it registers (`providers/openai/options.rs:523`, with `refuse_all` at `providers/internal/mod.rs:42`). Naming the dialect `openai` would have made Rig send `reasoning_effort` and judge it against the OpenAI catalog by model name. That would have been a guess about a server that Ravel cannot know. The refusal is `unsupported_parameter`, it uses no budget, and the mutation that names the dialect `openai` fails the test. The spec now says so, in "LLM calls". The brief said "the mapping covers `reasoning`", and for this type the mapping is the refusal. A policy that needs `reasoning` on an unlisted server has no way to ask for it, and that is the price of sending no provider options.
- **`Auth::OptionalBearer` for `openai-compatible`.** An endpoint without `apiKeyEnv` gets an empty credential, and Rig then sends no `Authorization` header. With a credential it sends `Authorization: Bearer`. The `openai` type keeps `Auth::Bearer`, and settings always require its key variable. The seam test reads the headers of the request to show that none is sent.
- **Discovery needed no change.** `rhai_worker/discovery.rs` already reported `available: true` for an `openai-compatible` endpoint without `apiKeyEnv`, and `rhai_settings.rs` tests it. The new seam test asserts it again next to a working keyless call.
- **The fake server gained a `Wire`.** `Wire` in `tests/support/llm_server.rs` names the Messages, Responses and Chat wires. It holds the path, the model, the credential header, the answer bodies (with usage, without usage, ending for a reason, reporting output tokens) and the oversized body of each wire. `Reply::messages*` and `oversized` stay for the earlier tests. The base URL of an OpenAI endpoint ends in `/v1`, as the services publish it, so the paths are `/v1/responses` and `/v1/chat/completions`.
- **The seam helpers moved to `tests/support/llm_seam.rs`.** `settings`, `start`, `selecting`, `ask`, `answered`, `one_call` and the rest left `rhai_llm.rs` and gained a `Wire` variant (`settings_on`, `ask_on`, `one_call_on`). The Anthropic names and signatures stay, so the 37 earlier tests did not change and all pass. The new cases are in `tests/rhai_llm_openai.rs`, and each runs once for each wire where the rule is the same.
- **Tests.** At the CLI seam: success with the path, the credential header, the model and the token field; the default token limit and the order of the messages; the full response map; usage present and absent; `length` and content-filter endings; 401, 403, 429 with a wait, 500, 400 and an undecodable body; a fall back from a rate-limited `openai` endpoint to a compatible one; a stall after the head and silence, both past the deadline of the call; an oversized body, a declared length beyond the bound, and a text beyond 1 MiB; the `models` list; a keyless compatible endpoint; `reasoning`; a `schema` still refused; the credential in the header only and redacted from an echo; a redirect; the budgets by reported tokens and by the effective `maxTokens`. At the adapter: the same classes against the server, plus `Auth` and the routes. The unit test "the other API types are not available yet" is gone.
- **Mutations seen to fail.** `Auth::Bearer` for the compatible type fails the keyless test. The compatible type on the Responses route fails 12 seam cases. The `openai` type on the Chat route fails 9 adapter cases. A compatible dialect named `openai` fails the `reasoning` test and no other. An `openai` endpoint that ignores its `baseUrl` fails 18 seam cases.
- **No in-session reviewer.** The claims that the compiler cannot check are the wire mapping and the credential path. The mutations above cover the mapping. The credential path is covered by a seam test that scans every header, the path and the body of each request, and by the redaction tests. Rig reads no environment variable in `OpenAIConfig::new` and `with_key`. Only the `from_env` constructors do, and Ravel calls neither.
- **Left for the next leaf.** `Quirks::openai()` has `supports_response_format: true`. `rhai-llm-structured-output` must build the compatible dialect with `without_response_format()` when `structuredOutput` is false, though the host function already refuses a `schema` for that endpoint as `unsupported_capability`. It must also add that code to `Code::was_sent`.
- **The spec has older prose findings.** The technical-communication check lists eight findings in `docs/specs/policy-evaluation.md` on lines this leaf did not write. The document was not converted before this edit, so the conversion is its own change. This leaf added one five-sentence paragraph.
