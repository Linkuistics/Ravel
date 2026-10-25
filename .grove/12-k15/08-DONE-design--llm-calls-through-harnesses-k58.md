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

## Decisions (running log)

- **Evidence before the question.** The Claude Code legal page permits a subscription sign-in in the unmodified `claude` program only. It forbids software that collects, stores or passes on a sign-in or a session token. The Codex authentication page tells a general API caller to use a Platform API key, not a ChatGPT access token. Ravel therefore cannot send a sign-in through Rig, and a harness process is the only legitimate route. `claude --bare` never reads the sign-in. `codex exec --output-schema` takes a file path. Installed versions: Claude Code 2.1.310 and Codex CLI 0.164.0.
- **The owner's direction.** Mid-session the owner wrote: "Ravel already can do this, so we just need to allow it to be recursive in some fashion." I offered three forms of recursion. The first was a harness run with no re-entry. The second was a nested `start-session` with a depth bound, and the third was a nested-session host call.
- **Settled: an LLM call through a sign-in is a harness run, with no re-entry.** The owner chose "Harness run, no re-entry". A policy calls its own route function and runs `claude` or `codex` through `ravel::run_harness`. One Ravel CLI supervises everything, one set of budgets applies, and the ordered stop reaches the run. `start-session` inside a harness run stays refused as `policy_reentry_refused`. The nested `start-session` was rejected. Each level would have budgets of its own. The outer stop cannot reach the process group of the nested managed process, and both levels use a kill-grace of 5 s. The `ravel` on `PATH` can also be another version, and 1.x ignores the marker. The nested-session host call was rejected because it is the most new code, and the policy already holds its own route table.
- **Settled: `ravel::llm`, the three API types and Rig go.** The owner chose "Remove them". The option said that LLM calls go only through harness runs, and that Ravel then holds no model credential, HTTP client or TLS stack. The option also said that the schema check stays as the data helper `ravel::json_check`. It said that issue 2 criteria 2 and 7 change, because a harness process is now required. Finally, it said that `rhai-llm-upgrade-proof-k21` is pruned and that a later leaf removes the built adapter. I take that selection as the owner's explicit confirmation of the prune of `rhai-llm-upgrade-proof-k21`.
- **Settled: the amendment goes on issue 2 as a comment.** The owner chose "Also comment on issue 2". The briefs, the spec and the ADR set record the change, and a short comment on GitHub issue 2 states the amended criteria.
- **Settled: the owner policy is trusted.** Mid-session the owner wrote: "The Rhai config file can be regarded as trusted." The Credentials section of the spec now says so. The credential rules keep Ravel itself from holding or passing on a credential, and they do not defend against the policy. The data that a policy reads stays untrusted, so `eval` stays disabled. I did not reopen the bounds of the worker, which earlier leaves built. They stop a runaway script, a native call that does not yield and an answer of a model that the policy follows.
- **The comment is posted.** It is <https://github.com/Linkuistics/Ravel/issues/2#issuecomment-6397905744>. It changes the goal, criteria 2, 5, 6 and 7 and the design questions about Rig. Every other criterion stands.
- **The tree after this design.** I pruned `rhai-llm-upgrade-proof-k21`. I cut `llm-calls-through-harnesses-k59` (`review-design`) and then `llm-harness-replan-k60` (`planning`) under `rhai-llm-calls-k15`. The planning leaf cuts `rhai-llm-removal`, which removes the built code of `ravel::llm` and moves the schema check. It also gives the Rhai upgrade task and the smoke tasks a leaf.
- **Settled without the owner: `ravel::json_check` throws `schema_invalid` for a schema outside the subset.** The other data helpers run in the worker and end nothing, so a catchable throw keeps them uniform. `ravel::llm` ended the evaluation as `host_request_invalid` for the same schema.
