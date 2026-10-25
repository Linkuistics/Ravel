# rhai-llm-upgrade-proof-k21

## Goal

Make a Rig upgrade cheap to prove. Rig released seven minor versions between June and October 2026. A task runs the proof in a fixed order, and opt-in smoke tasks make one real call for each API type.

## Context

- The input is "Rig upgrades" and "Test seams" in `docs/specs/policy-evaluation.md`.
- All use of Rig stays behind the LLM adapter. An upgrade changes the four Rig pins together and touches only that module.
- The earlier leaves of this increment own the scripted cases. This leaf checks that the list of the spec is complete, and it adds the layers around it.
- `rig-cassette` 0.45.0 records and replays provider HTTP traffic. It is a development dependency only.
- A real recording needs real credentials and costs money. Ask the owner before any real call. The owner may supply credentials, or may decline.

## Done when

- The scripted cases of the spec exist for each API type. The list is success, structured output, usage present and absent, 401, 429 with a wait and 500. It also has an undecodable body, a body that stops short and a stall past the deadline. The body cases are an oversized success body, a large failure body and a body with no declared length. The last cases are an unsupported option and a redirect that Ravel does not follow.
- `rig-cassette` replays one recorded exchange for each API type through the local server. A wire change in Rig shows as a replay mismatch.
- If the owner gives no credentials, hand-author each recording from the provider documentation. State in the running log that the recordings are not real, and which command records a real one.
- A Taskfile task runs an upgrade proof in this order: the scripted fakes, then the replays. A second task makes a new recording, and it is separate and opt-in.
- A Taskfile task makes one real call for each API type with real credentials. The default test run never includes it.
- A Rhai upgrade has a task of the same kind. It runs the engine tests and the hostile classes.
- The book chapter about developing Ravel is not changed here. Record the task names in the running log for the documentation increment.
- `task check` and `task test` pass.

## Notes

- The smoke task for a real harness run comes with the harness-run increment.
- Never write a credential to the repository, a log or a recording. Check every recording for the key and the account identifiers before it is committed.
