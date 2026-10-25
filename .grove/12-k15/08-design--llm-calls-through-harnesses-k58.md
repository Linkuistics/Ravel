# llm-calls-through-harnesses-k58

## Goal

Decide how a Rhai policy reaches a model with the sign-ins the owner already uses: Claude Code and Codex. Settle what happens to `ravel::llm`, the three API types, the Rig adapter and its four pins. Deliver a spec change, an ADR change, or both, for the owner to agree.

## Context

- The owner was asked to supply credentials for a real recording of each API type. The owner answered that Rhai should use Codex and Claude Code "with the existing credentials that we use here". The owner then chose "Design change: LLM calls via the harnesses".
- Claude Code and Codex sign-ins are subscription credentials. They are not API keys. The three API types in the spec (`anthropic`, `openai`, `openai-compatible`) send a key that Ravel reads from an environment variable named by `apiKeyEnv`. A subscription sign-in cannot fill that slot.
- The spec already has `ravel::run_harness`. It runs a harness that settings name, with arguments and `stdin` from the script, and it returns `stdout`, `stderr`, the exit status and `result`. A script can therefore already call `claude -p` or `codex exec` with the sign-in of that CLI. The increment `rhai-harness-runs` has not built it yet.
- The design must therefore answer what `ravel::llm` still adds beyond `ravel::run_harness`. Candidates are the typed request, the schema check on the answer, the usage and output-token budget, the bounded response, and the credential rules.
- The leaves `rhai-llm-anthropic` to `rhai-llm-structured-output` are built and committed. They hold the Rig adapter, the bounds, the budgets, the two other API types and structured output. The design may keep, shrink or remove any of them.
- Spec sections that a change reaches: "LLM calls", "LLM endpoints and credentials", "Evaluation budgets and their composition", "Rig upgrades" and "Test seams" in `docs/specs/policy-evaluation.md`. The ADR `owner-settings-name-the-llm-endpoints` is in play. The glossary terms LLM endpoint, LLM call, Provider and Evaluation budget are in play.
- `rig-cassette` 0.45.0 findings from the stopped session: `ProviderCassette` needs its `http` feature. It replays a recorded exchange through a local server. It matches request bodies exactly unless `RIG_CASSETTE_MATCHING=shape` is set. It refuses a recording with an undeclared account failure. Recording needs `RIG_PROVIDER_TEST_MODE=record`.
- The stopped session's running log is in `rhai-llm-upgrade-proof-k21`. Find the file with `grove-llm resolve rhai-llm-upgrade-proof-k21`.

## Done when

- The spec or the ADR set states one answer to each question below. Ask the owner where the evidence does not settle it.
  - Does `ravel::llm` stay, shrink or go?
  - If it stays, which API types stay? Does a harness become a fourth type, with the harness name in place of a base URL and a key?
  - What do structured output, usage reporting and the LLM budgets become for a call through a harness? A harness prints text, and Ravel builds no argument that is specific to a harness.
  - Does the Rig adapter stay? If it goes, which of the four pins go, and what replaces the Rig upgrade proof?
  - What is the credential rule, and which environment does a harness call receive?
- Each answer says what happens to the committed leaves that it touches. A change that removes or rewrites built code names the leaf that does it.
- `rhai-llm-upgrade-proof-k21` has a stated fate: it runs as written, it runs in a reduced form, or the owner prunes it. A prune needs the owner's explicit confirmation.
- The brief `rhai-llm-calls-k15` and the grove brief still match the answer. Update their "Done when" and "Decomposition" where they do not.
- A `review-design` leaf follows this one if the answer changes the spec. It sits before any impl leaf that builds on the change.

## Notes

- Read the official documentation of the installed version before you rely on a claim about Claude Code or Codex. Examples are the noninteractive flags, the structured-output flags and the usage reporting. Cite the source in the spec.
- Never read, print or record a credential. Do not make a real call to a model in this leaf.
- This leaf sits ahead of `rhai-llm-upgrade-proof-k21` so that leaf stays live until the answer exists.
