# rhai-llm-calls-k15 — brief

## Goal

Let a Rhai policy ask a model a question with `ravel::llm`. The Ravel CLI sends the request through Rig to an LLM endpoint that the owner named in settings. The script builds the messages and interprets the answer.

## Done when

- `ravel::llm` works for the three API types: `anthropic`, `openai` and `openai-compatible`. It supports plain text, structured output, usage reporting and the failure codes of the spec.
- Every failure of a call reaches the script as data. Every breach of a bound ends the evaluation. A script cannot catch a breach.
- The credential reaches only the base URL of its endpoint. No host function, notice or log carries it.
- Every response body, successful or not, has a bound that Ravel applies before Rig decodes it.
- The evaluation budgets for LLM calls and output tokens hold, and they compose with the whole deadline.
- A Rig upgrade has a task that proves it at the CLI seam, and opt-in smoke tasks make one real call for each API type.
- The TypeScript path works as before, and `task check` and `task test` pass.

## Decomposition

- `rhai-llm-anthropic` makes the first call work end to end for one API type, with the bounded collection of every response body. It also proves that Rig builds for the three archive targets.
- `rhai-llm-bounds` bounds the answer text, and it proves the stall deadline and the cancellation of a call in flight.
- `rhai-llm-budgets` adds the budgets and the usage accounting.
- `rhai-llm-openai-types` adds the other two API types.
- `rhai-llm-structured-output` adds the schema subset and the checks on the answer.
- `rhai-llm-upgrade-proof` adds the Rig upgrade task, the replay of recordings and the smoke tasks.

The first leaf comes first because every later leaf extends its adapter and its fake endpoint. The body bounds are part of the first call, because the Ravel CLI holds an unbounded body, and no limit of the worker reaches it. The remaining bounds come before the budgets. Structured output follows the other API types, because it must hold on all three wires.

## Pointers

- ADR: `docs/adr/owner-settings-name-the-llm-endpoints.md`.
- Spec: `docs/specs/policy-evaluation.md`. The sections are "LLM calls", "LLM endpoints and credentials", "Evaluation budgets and their composition", "Rig upgrades" and "Test seams".
- Review findings F2 and F3 of `rhai-policy-k4`, and their integration in `rhai-policy-k5`. Find both with `grove-llm resolve`. They define the bounded transport and the credential rules.
- Glossary: LLM endpoint, LLM call, Provider and Evaluation budget.
- Test seam: the real `ravel start-session` against a sandbox home. A deterministic local HTTP server stands behind it, with a dummy key and no network. The real Rig client runs.

## Notes

- Pins: `rig-core`, `rig-http` and `rig-reqwest` at `=0.45.0` each. `reqwest` 0.13 comes through `rig-reqwest`. `rig-cassette` `=0.45.0` is a development dependency only.
- All use of Rig stays behind one internal module, the LLM adapter. Its interface is one call. An endpoint entry and a checked request go in, and a response or a classified failure comes out.
- Ravel never uses the shared HTTP client of Rig or its environment constructors. No ambient `*_BASE_URL` or `*_API_KEY` variable takes part.
- Rig has no 1.0 release. Read the source of the pinned version before you rely on an API. The design and the review both checked the APIs that the spec names.
- The notice of the Rhai path has schema version 2. This increment adds `llmCalls` and `bounds`.
