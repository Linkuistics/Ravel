# External-source inventory and fragments baseline

## Current sample state

No valid repetition completed. Each of the five planned repetitions exhausted
its initial attempt and two permitted replacements. All 15 attempts reached
`thread.started`, failed DNS resolution for both the WebSocket and HTTPS
transports, emitted no final assistant message, and ended at the frozen
20-minute timeout. The frozen invalid-run rule therefore requires a sample
shortfall of five; no fourth attempt, answer selection, scoring, blind
adjudication, or criterion classification was performed.

## Shared execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- Timeout: GNU coreutils `9.11`, 20 minutes with a 30-second kill grace
- Frozen-rubric SHA-256 at execution:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Fixture SHA-256 before execution:
  `2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`
- Control template: one `auth.json` file and no skills, instructions, hooks,
  plugins, MCP configuration, or `config.toml`; every attempt records its
  template and fresh-home manifests without preserving the credential-bearing
  file.
- Harness- or hook-injected messages: none observed in any raw stream.
- Model tool calls: none observed in any raw stream.
- Run-directory writes or mutations: none; every post-run manifest contains
  exactly the unchanged declared fixture.
- Final assistant messages: none.
- Service truncation: not applicable because no response began.
- Replacement-run stdin: closed before launch, so the frozen prompt was the
  only model input.

The complete command shape was:

```text
env -u CODEX_CI -u CODEX_PERMISSION_PROFILE -u CODEX_SANDBOX \
  -u CODEX_SANDBOX_NETWORK_DISABLED -u CODEX_SESSION_ID \
  -u CODEX_THREAD_ID -u GROVE_SIGNAL_FILE -u HERDR_ENV \
  -u HERDR_PANE_ID -u HERDR_SOCKET_PATH -u HERDR_TAB_ID \
  -u HERDR_WORKSPACE_ID CODEX_HOME=<fresh-codex-home> \
timeout --signal=TERM --kill-after=30s 20m \
codex exec --ignore-user-config --ignore-rules --ephemeral \
  --skip-git-repo-check --sandbox read-only --model gpt-5.4 \
  -c model_reasoning_effort='high' \
  -c skills.bundled.enabled=false --json \
  --cd <fresh-run-directory> <frozen-prompt>
```

The exact frozen prompt was:

```text
Prepare an authoring plan for a complete-source Markdown walkthrough of the
external OCaml utility in this directory. The only in-scope production source is
targets/ocaml/check_floor.ml; no other project file is supplied or in scope. The
snapshot is authoritative and stable. The audience knows OCaml and Unix build
tooling but not this utility. You may use read-only shell commands only to inspect
that declared file. Do not write the walkthrough. Return: the exact source
inventory and unknown/excluded material; a concept-oriented reader sequence; a
concrete fragment ledger with one file root, named child fragments, source ranges,
and insertion relationships; one low-resolution worked execution from production
inputs through stage boundaries to outputs and observable results; and the exact
mechanical checks required before publication.
```

## Access boundary

Every fresh run directory was outside this repository and contained exactly
`targets/ocaml/check_floor.ml`, 6,698 bytes, SHA-256
`2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`.
The same path, length, and digest appear in all 15 post-run manifests. The raw
streams contain no model tool calls, so no model-visible filesystem result was
delivered before transport failure. The operating-system readability limitation
stated by the frozen rubric still applies; these records claim only the audited
model interface boundary.

## Invalid-attempt summary

| Planned repetition | Invalid attempts | Machine-checkable reason | Valid repetitions |
|---|---:|---|---:|
| 1 | 3 | exit `124`; no final assistant message | 0 |
| 2 | 3 | exit `124`; no final assistant message | 0 |
| 3 | 3 | exit `124`; no final assistant message | 0 |
| 4 | 3 | exit `124`; no final assistant message | 0 |
| 5 | 3 | exit `124`; no final assistant message | 0 |

Each attempt ran for exactly 1,200 seconds and preserved 29 JSONL events: one
thread-start event, one turn-start event, and 27 reconnect errors. Stderr names
the common cause as DNS lookup failure for `chatgpt.com` followed by failed
network requests. The per-attempt records preserve distinct raw streams,
stderr, timestamps, and manifests.

## Results and limitations

There are no atomic scores, success counts, classifications, recurring
omissions, or rationalizations because there is no valid final answer. Blind
adjudication and the independent re-score were not commissioned: an adjudicator
would have received no scoreable surface. The intended sample is short by five
valid repetitions.

This infrastructure outcome establishes nothing about unguided source
inventory, concept ordering, fragment design, worked executions, or mechanical
validation. It also leaves the rubric's stated Case B priming limitation
unevaluated: the fixture comment was shared by all attempts, but no model
response was produced from it.

