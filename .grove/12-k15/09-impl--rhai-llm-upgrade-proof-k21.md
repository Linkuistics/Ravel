# rhai-llm-upgrade-proof-k21

## Goal

Make a Rig upgrade cheap to prove. Rig released seven minor versions between June and October 2026. A task runs the proof in a fixed order, and opt-in smoke tasks make one real call for each API type.

## Context

- The input is "Rig upgrades" and "Test seams" in `docs/specs/policy-evaluation.md`.
- All use of Rig stays behind the LLM adapter. An upgrade changes the four Rig pins together and touches only that module.
- The earlier leaves of this increment own the scripted cases. This leaf checks that the list of the spec is complete, and it adds the layers around it.
- `rig-cassette` 0.45.0 records and replays provider HTTP traffic. It is a development dependency only.
- A real recording needs real credentials and costs money. Ask the owner before any real call. The owner may supply credentials, or may decline.
- `tests/rhai_llm_schema.rs` already holds the scripted structured-output cases for each API type, with the place of the schema in each request body. The list of the spec needs no new case for it.
- The scripted fakes already use provider documents that the tests author. A replay of another authored document adds no evidence about the wire, so the spec asks for a real exchange.

## Done when

- The scripted cases of the spec exist for each API type. The list is success, structured output, usage present and absent, 401, 429 with a wait and 500. It also has an undecodable body, a body that stops short and a stall past the deadline. The body cases are an oversized success body, a large failure body and a body with no declared length. The last cases are an unsupported option and a redirect that Ravel does not follow.
- `rig-cassette` replays one recorded real exchange for each API type through the local server, as the spec requires. A wire change in Rig shows as a replay mismatch. A hand-authored document is not a real exchange, and it does not meet this item.
- If the owner gives no credentials for an API type, stop and ask the owner. Recommend that this leaf adds an `impl` leaf `rhai-llm-real-recordings` before `release-ravel-2-0-0`. That leaf records the missing exchanges. This leaf then retires with the replay machinery proven on one fixture for each such type. Put `synthetic` in the file name of the fixture and in the running log. The other option is a change of the spec, which only the owner approves. Record the owner's answer in the running log.
- A Taskfile task runs an upgrade proof in this order: the scripted fakes, then the replays. A second task makes a new recording, and it is separate and opt-in.
- A Taskfile task makes one real call for each API type with real credentials. The default test run never includes it.
- A Rhai upgrade has a task of the same kind. It runs the engine tests and the hostile classes.
- The book chapter about developing Ravel is not changed here. Record the task names in the running log for the documentation increment.
- `task check` and `task test` pass.

## Notes

- The smoke task for a real harness run comes with the harness-run increment.
- Never write a credential to the repository, a log or a recording. Check every recording for the key and the account identifiers before it is committed.

## Decisions (running log)

- **Stopped before any work, on the owner's answer.** I asked whether the owner would supply credentials for a real recording of each API type. The owner answered that Rhai should use Codex and Claude Code, "with the existing credentials that we use here". I asked what that means. The owner chose "Design change: LLM calls via the harnesses".
- **No real call was made, and no credential was read.** Claude Code and Codex sign-ins are subscription credentials, not API keys. The three API types in the spec send a key from an environment variable.
- **The design change reaches past this leaf.** It questions `ravel::llm`, the three API types and the Rig adapter, which the leaves `rhai-llm-anthropic` to `rhai-llm-structured-output` already built and committed. The spec sections "LLM calls", "Rig upgrades" and "Test seams" would change. Only the owner can approve that.
- **The tree is unchanged, and nothing was pruned.** The session ended with this leaf live. A prune needs the owner's explicit confirmation.
- **Open for the owner.** Decide one of these.
  - Start a `design` leaf for LLM calls through harnesses. It settles whether `ravel::llm` stays, shrinks or goes, and what happens to the Rig leaves and the pins.
  - Keep `ravel::llm` and finish this leaf with synthetic fixtures. Then add `rhai-llm-real-recordings` before `release-ravel-2-0-0`.
- **The owner chose the design leaf.** A second session asked again, with a recommendation to keep `ravel::llm`. The owner chose "Start a design leaf for LLM calls through harnesses". That session inserted `llm-calls-through-harnesses-k58` ahead of this leaf. This leaf stays live and unchanged until that design states its fate. No prune was made.
- **Findings for either route.** `rig-cassette` 0.45.0 needs its `http` feature for `ProviderCassette`. It replays a recorded exchange through a local server. It matches request bodies exactly unless `RIG_CASSETTE_MATCHING=shape` is set. It refuses a recording with an undeclared account failure. Recording needs `RIG_PROVIDER_TEST_MODE=record`.
