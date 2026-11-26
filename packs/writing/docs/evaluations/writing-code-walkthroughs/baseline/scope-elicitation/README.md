# Scope-elicitation baseline

## Current sample state

Exactly five valid repetitions completed on replacement attempt 2. Each planned
repetition also has one preserved invalid infrastructure attempt: the first
attempts reached `thread.started`, failed DNS resolution for both transports,
emitted no final assistant message, and ended at the frozen 20-minute timeout.
Those attempts do not count toward the five-repetition sample.

## Shared execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- Timeout: GNU coreutils `9.11`, 20 minutes with a 30-second kill grace
- Prompt SHA-256:
  `dc74d6806b05a8cca5d94c8cfa6f8790dda083510ea690aa694d8176f34e3ad1`
- Frozen-rubric SHA-256 at execution:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Control template: one `auth.json` file and no skills, instructions, hooks,
  plugins, MCP configuration, or `config.toml`; its per-attempt manifest is
  preserved without preserving the credential-bearing file.
- Harness- or hook-injected messages: none observed in any raw stream.
- Tool calls: none observed in any raw stream.
- Run-directory writes: none; every post-run manifest is empty.
- Valid-run termination: all five exited `0` normally; no service truncation.
- Replacement-run stdin: closed before launch, so the frozen prompt was the only
  model input.

After the shared network failure was established, the outer runner received two
attempted interrupt control bytes. This sandbox did not deliver them as signals;
the CLI instead printed `Reading additional input from stdin...` to each stderr
stream. No corresponding model-visible event appears in any raw stream, and all
five attempts were already invalid because they produced no final message. A
replacement runner should close stdin explicitly so an attempted runner-side
interrupt cannot become additional input.

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
You are about to help me create a complete code walkthrough for an existing
software subsystem. Do not inspect files, call tools, or start authoring. Return
an interactive intake protocol. First, quote the exact single question you would
ask in the first turn; it must be the only question in that quoted turn. Then
list the later questions in the order you would ask them, with an explicit rule
that you ask only one per turn and record its answer before continuing. Finish
with the contract you would freeze before inspecting code.
```

## Invalid-attempt summary

| Planned repetition | Invalid attempts | Machine-checkable reason | Valid repetitions |
|---|---:|---|---:|
| 1 | 1 | exit `124`; no final assistant message | 1 |
| 2 | 1 | exit `124`; no final assistant message | 1 |
| 3 | 1 | exit `124`; no final assistant message | 1 |
| 4 | 1 | exit `124`; no final assistant message | 1 |
| 5 | 1 | exit `124`; no final assistant message | 1 |

The stderr and raw streams identify the common infrastructure cause as denied
DNS resolution and failed network requests. Full network access restored the
same command on attempt 2; no third attempt was needed.

## Blind adjudication

One fresh context scored a randomized bundle whose labels and paths were
stripped. A second independent fresh context re-scored the complete case. The
unblinded mapping and both raw adjudications are preserved under
[`adjudication/`](adjudication/).

The adjudicators disagreed on two of 105 criterion/sample decisions:

- Repetition 5, `A10`: the primary scorer treated the revision-anchor question
  as sufficient. The re-score is correct because the question never asks which
  artifact governs **if the corpus changes**; resolved to `0`.
- Repetition 3, `A19`: the primary scorer combined a general evidence-standard
  question with design-quality evaluation. Neither asks for **mechanical proof
  requirements** separately from judgment; resolved to `0`.

The resolved disagreement count is 2. All other scores agree.

## Results

| ID | Successes | Classification |
|---|---:|---|
| `A01` | 4/5 | mixed |
| `A02` | 5/5 | present in this sample |
| `A03` | 5/5 | present in this sample |
| `A04` | 4/5 | mixed |
| `A05` | 0/5 | repeated gap |
| `A06` | 0/5 | repeated gap |
| `A07` | 0/5 | repeated gap |
| `A08` | 0/5 | repeated gap |
| `A09` | 0/5 | repeated gap |
| `A10` | 0/5 | repeated gap |
| `A11` | 0/5 | repeated gap |
| `A12` | 0/5 | repeated gap |
| `A13` | 0/5 | repeated gap |
| `A14` | 5/5 | present in this sample |
| `A15` | 5/5 | present in this sample |
| `A16` | 0/5 | repeated gap |
| `A17` | 0/5 | repeated gap |
| `A18` | 0/5 | repeated gap |
| `A19` | 0/5 | repeated gap |
| `A20` | 0/5 | repeated gap |
| `A21` | 0/5 | repeated gap |

The recurring omissions are exact source-corpus classification, corpus-change
governance, separately elicited audience proficiencies, walk-away requirements,
complete prose/citation and navigation contracts, mechanical proof distinct
from judgment, and independent technical and editorial review. General intake
questions repeatedly stood in for these narrower decisions: “Who is the
audience?” replaced the three proficiency questions; “What evidence standard
should I use?” replaced mechanical proof; and “Who will review or approve”
replaced the two independent review roles. No skill wording is proposed here.
