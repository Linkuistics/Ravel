# Exposition and assurance baseline

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
- Prompt SHA-256:
  `46051c728695a44c5cfaecf1a9542d6ffe2cfe6171c84f5a44f683c8f6546413`
- Frozen-rubric SHA-256 at execution:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Control template: one `auth.json` file and no skills, instructions, hooks,
  plugins, MCP configuration, or `config.toml`; every attempt records its
  template and fresh-home manifests without preserving the credential-bearing
  file.
- Harness- or hook-injected messages: none observed in any raw stream.
- Model tool calls: none observed in any raw stream. Each stream's sole item
  event is an error item emitted by the client during transport failure.
- Run-directory writes or mutations: none; every pre- and post-run manifest is
  empty.
- Final assistant messages: none.
- Service truncation: not applicable because no response began.
- Runner stdin: closed before launch, so the frozen prompt was the only model
  input.

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
Design the exposition and assurance plan for a multi-page code walkthrough of a
key-value service. Readers know the implementation language and storage APIs but
not this service's domain. The only established codebase fact is that a
NormalizedKey is the canonical tenant-qualified identity. An early page explains
that fact. A later mutation page must use the meaning while explaining a write
path, and an early source fragment is relevant again there. Decide what the later
page states locally, what it links to, and whether it repeats the source fragment.
Include one representative later-page paragraph and state the prose rules for
claims, stable vocabulary, explicit actors, and failure categories. Then specify
mechanical checks and independent review before publication. Do not call tools. Do not invent
implementation names or behavior: express unknown implementation details as
placeholders or verification obligations.
```

## Access boundary

Every fresh run directory was outside this repository and empty. All 15 empty
post-run manifests match their empty pre-run state. The raw streams contain no
model tool calls, so no filesystem result was delivered to the model before
transport failure. The operating-system readability limitation in the frozen
rubric still applies; these records claim only the audited model-interface
boundary.

## Invalid-attempt summary

| Planned repetition | Invalid attempts | Machine-checkable reason | Valid repetitions |
|---|---:|---|---:|
| 1 | 3 | exit `124`; no final assistant message | 0 |
| 2 | 3 | exit `124`; no final assistant message | 0 |
| 3 | 3 | exit `124`; no final assistant message | 0 |
| 4 | 3 | exit `124`; no final assistant message | 0 |
| 5 | 3 | exit `124`; no final assistant message | 0 |

Each attempt ran for exactly 1,200 seconds and preserved 29 JSONL events: one
thread-start event, one turn-start event, one completed client error item, and
26 reconnect errors. Stderr names the common cause as DNS lookup failure for
`chatgpt.com` followed by failed network requests. The per-attempt records
preserve distinct raw streams, stderr, timestamps, access manifests, and
post-run Codex-created state.

## Results and limitations

There are no atomic scores, success counts, classifications, recurring
omissions, or rationalizations because there is no valid final answer. Blind
adjudication and the independent re-score were not commissioned: an adjudicator
would have received no scoreable surface. The intended sample is short by five
valid repetitions.

This infrastructure outcome establishes nothing about self-contained prose,
semantic repetition, optional links, validation boundaries, technical review,
editorial review, or the walk-away property. It supplies no evidence for or
against skill guidance.
