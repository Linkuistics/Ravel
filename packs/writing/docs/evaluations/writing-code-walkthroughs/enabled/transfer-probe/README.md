# Out-of-sample transfer probe

## Outcome

The frozen transfer probe stopped with a sample shortfall. No control repetition
produced a valid sample. One enabled repetition produced a valid final refusal;
the other nine planned repetitions remained incomplete when control repetition
1 exhausted its two-replacement allowance. Atomic scoring and a comparative
transfer verdict are therefore not defined.

All 21 attempted contexts exited `0`, returned a normal final message, and left
the 7,316-byte fixture unchanged. Twenty attempts are invalid because their raw
events accessed undeclared model-interface surfaces: every invalid attempt called
an MCP resource surface, seven also called web search, and one also called an
external document connector. Enabled repetition 5 attempt 2 made no tool call
and is valid under the frozen invalid-run rule; its refusal remains a poor but
valid substantive outcome.

The frozen criteria contain 16 non-compliance rows (`T05`–`T20`). A complete
sample would succeed only if at least 8 rows improved by at least 2 successes out
of 5 over control and no row regressed by 2 or more. With `0/5` valid controls
and `1/5` valid enabled samples, neither per-row five-sample counts nor deltas
exist. The transfer claim is **not established**; this is a sample-shortfall
verdict, not a threshold failure inferred from invalid answers.

No skill wording changed in this slice.

## Frozen selection and criteria

A fresh no-skill selector received only the evaluation subject and transfer
constraints. It selected `junegunn/fzf` at immutable commit
`15f64c492a08f0840b81540c7d1de35737448086`, bounded to the production file
`bin/fzf-tmux`. The selection did not occur elsewhere in the repository before
this probe. Its complete raw evidence is under [`selection/`](selection/).

A separate fresh no-skill criterion author received the selection metadata and
paired prompt but no source bytes. It made no tool call and froze 20 binary rows,
including 4 compliance controls and 16 non-compliance criteria, in
[`criteria.md`](criteria.md). Only after that final message was preserved was the
source retrieved from the selector's immutable raw URL.

| Artifact | SHA-256 |
|---|---|
| Paired [`prompt.txt`](prompt.txt) | `aa9e81bf31edc1d96d74f361048c15123ea7554eebeafd9a6742bed1b0278322` |
| Frozen [`criteria.md`](criteria.md) | `1b22cba057d5a2c269f7385ddbb2de2350350a8b40c6be6b2bdd3f7a060e6081` |
| Selector final | `18e8e526c31b780689e9efee034d4f223af7b48b33e8079232d244737c95bc6d` |
| Fixture `bin/fzf-tmux` | `ea14943d739fc81952d5071a642591df25ed8dac166040a6d6036b3be197cecd` |

The fixture has 256 total lines and 233 nonblank lines, within the selector's
350-line bound.

## Execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- GNU timeout: `9.11`, 20 minutes with 30-second kill grace
- Frozen rubric SHA-256:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Skill revision: `9cc8ccd8c5b8f070a572378bd61953f7b3bbb8ac`
- Installed `SKILL.md` SHA-256:
  `795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`
- Skill manifest aggregate SHA-256:
  `1f530317f817805eb88bf45f8ba66a0f135f8b279f36df2c5d864cfcd26251c1`
- Schedule: `control enabled enabled control enabled control control enabled control enabled`
- Evaluation root: `/private/tmp/grove-enabled-transfer.djF4XX`

The control template contains only `auth.json`. The enabled template differs
only by `skills/writing-code-walkthroughs/SKILL.md`; removing that manifest row
makes the sealed template manifests byte-identical. Credential contents and
temporary homes are not preserved.

## Attempt accounting

| Planned sample | Attempts made | Valid | Invalid reason / stopping state |
|---|---:|---:|---|
| control 1 | 3 | 0 | all three called undeclared MCP resources; allowance exhausted |
| control 2 | 2 | 0 | both called undeclared MCP resources; stopped after control 1 shortfall |
| control 3 | 2 | 0 | both called undeclared MCP resources; stopped after control 1 shortfall |
| control 4 | 2 | 0 | both called undeclared MCP resources; attempt 1 also searched the web |
| control 5 | 2 | 0 | both called undeclared MCP resources; attempt 1 also searched the web |
| enabled 1 | 2 | 0 | both called undeclared MCP resources |
| enabled 2 | 2 | 0 | both called undeclared MCP resources |
| enabled 3 | 2 | 0 | both called undeclared MCP resources and searched the web |
| enabled 4 | 2 | 0 | both called undeclared MCP/search surfaces; attempt 2 also called a document connector |
| enabled 5 | 2 | 1 | attempt 1 invalid; attempt 2 valid refusal with no tool event |

There are 20 invalid attempts by model-interface boundary violation. Seven have
the additional web-search violation. No attempt mutated the run directory, no
process exited nonzero, no final message was absent, and no service truncation was
observed.

Per-repetition records link every raw stream and manifest:

- [`control-repetition-1.md`](control-repetition-1.md) through
  [`control-repetition-5.md`](control-repetition-5.md)
- [`enabled-repetition-1.md`](enabled-repetition-1.md) through
  [`enabled-repetition-5.md`](enabled-repetition-5.md)

## Limits

The probe establishes that the selection and criteria freeze boundaries were
executed and that the current runtime could not complete the declared paired
sample under the frozen command. It provides no evidence of skill improvement,
regression, or discovery frequency. The model alias is not an immutable service
snapshot. The preserved raw events establish the audited model-interface
boundary, not operating-system filesystem inaccessibility.
