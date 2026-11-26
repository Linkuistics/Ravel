# Skill-enabled scope-elicitation evaluation

## Outcome

The frozen campaign is non-comparable for two independent reasons. First,
control repetition 1 attempt 2 is the selected behavioral sample under the
rubric's exhaustive replacement rule, but its prompt omitted the canonical
terminal LF and therefore breaches the byte-identical execution contract. The
later attempt 3 is preserved only as a post-breach diagnostic; it cannot repair
or replace attempt 2. Second, enabled repetition 5 exhausted all three attempts.
Each discovered the installed skill by issuing a shell command to read
`SKILL.md`; Case A forbids every tool call, so all three are invalid.

The same conflict invalidated two attempts each for enabled repetitions 1 and
3. Their third attempts, and enabled repetitions 2 and 4, emitted no tool call
and are valid. Those four valid streams contain no observable skill-file read or
skill announcement. That does not establish non-discovery because skill
metadata may be available without a raw discovery event; the valid-sample
discovery count is indeterminate, with `0/4` observable reads or uses. The
required five-versus-five material-improvement and regression verdicts cannot
be calculated, and this incomplete, protocol-breached sample is not evidence
that the skill met or missed either endpoint.

## Execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- GNU timeout: `9.11`, 20 minutes with a 30-second kill grace
- Prompt SHA-256, including its canonical terminal LF:
  `dc74d6806b05a8cca5d94c8cfa6f8790dda083510ea690aa694d8176f34e3ad1`
- Skill revision: `9cc8ccd8c5b8f070a572378bd61953f7b3bbb8ac`
- Installed `SKILL.md` SHA-256:
  `795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`
- Skill manifest aggregate SHA-256:
  `1f530317f817805eb88bf45f8ba66a0f135f8b279f36df2c5d864cfcd26251c1`
- Schedule: `control enabled enabled control enabled control control enabled control enabled`

The sealed control template contains only `auth.json`. The enabled template
differs only by `skills/writing-code-walkthroughs/SKILL.md`; after removing that
subtree, their manifests are identical. Every attempt used a fresh copied home
and an empty directory outside the repository. All preserved run-directory post
manifests are empty. The exact command remains the frozen command in
[`baseline/rubric.md`](../../baseline/rubric.md#common-execution-controls).

## Attempt accounting

| Arm | Planned repetition | Attempts | Valid attempt | Invalid reason |
|---|---:|---:|---:|---|
| control | 1 | 3 | 2, protocol-breached | attempt 1: network/DNS, no final; attempt 3: post-breach diagnostic, not a replacement |
| enabled | 1 | 3 | 3 | attempts 1-2: one prohibited `SKILL.md` read each |
| enabled | 2 | 1 | 1 | — |
| control | 2 | 1 | 1 | — |
| enabled | 3 | 3 | 3 | attempts 1-2: one prohibited `SKILL.md` read each |
| control | 3 | 1 | 1 | — |
| control | 4 | 1 | 1 | — |
| enabled | 4 | 1 | 1 | — |
| control | 5 | 1 | 1 | — |
| enabled | 5 | 3 | none | all three: one prohibited `SKILL.md` read each |

There are 18 preserved executions: eight contract-valid selected samples, one
protocol-breached selected control sample, eight authorized invalid attempts,
and one post-breach diagnostic. Seven invalid enabled attempts explicitly
announced the skill, read its file, and then returned a protocol. They are
invalid because of the read, not because of their substantive answers. The four
valid enabled samples have no observable skill-file read or use; the rubric
requires retaining rather than filtering them. No discovery count is inferred
from that absence.

## Blind adjudication

A fresh context scored the nine selected behavioral samples from a randomized
bundle containing their final answers and raw events but no arm labels, path
names, skill bytes, or campaign hypothesis. This includes the selected but
protocol-breached control repetition 1 attempt 2, so the scores are descriptive
only. The mapping, bundle, raw adjudication, citations, superseded selection,
and adversarial protocol review are preserved under
[`adjudication/`](adjudication/). Every sample has 21 decisions, and the
per-sample totals reproduce the aggregate arithmetic.

The rubric calls for an independent re-score of one complete case. This case is
not complete, so no second-score disagreement count is claimed.

## Incomplete results

| ID | Historical baseline | Contemporary control | Enabled valid sample |
|---|---:|---:|---:|
| `A01` | 4/5 | 4/5 | 3/4 |
| `A02` | 5/5 | 5/5 | 4/4 |
| `A03` | 5/5 | 5/5 | 4/4 |
| `A04` | 4/5 | 5/5 | 3/4 |
| `A05` | 0/5 | 0/5 | 0/4 |
| `A06` | 0/5 | 1/5 | 4/4 |
| `A07` | 0/5 | 0/5 | 0/4 |
| `A08` | 0/5 | 0/5 | 0/4 |
| `A09` | 0/5 | 0/5 | 0/4 |
| `A10` | 0/5 | 0/5 | 0/4 |
| `A11` | 0/5 | 0/5 | 0/4 |
| `A12` | 0/5 | 0/5 | 0/4 |
| `A13` | 0/5 | 0/5 | 0/4 |
| `A14` | 5/5 | 5/5 | 2/4 |
| `A15` | 5/5 | 5/5 | 4/4 |
| `A16` | 0/5 | 0/5 | 0/4 |
| `A17` | 0/5 | 0/5 | 1/4 |
| `A18` | 0/5 | 0/5 | 0/4 |
| `A19` | 0/5 | 0/5 | 0/4 |
| `A20` | 0/5 | 0/5 | 0/4 |
| `A21` | 0/5 | 0/5 | 0/4 |

Across all nine selected samples, the recurring omissions are manifests; separate
classification of tests, fixtures, models, generated files, examples, and
dependencies; corpus-change governance and conditional authoritative-artifact
selection; the three audience proficiencies;
walk-away behavior; navigation and cross-reference constraints; mechanical
proof distinct from judgment; and independent technical and editorial review.
Depth capture varies in the enabled arm. Production-source intake occurs more
often in the incomplete enabled-template sample, but no five-repetition endpoint
comparison or skill-wording attribution is permitted and no valid stream
visibly read the skill body.

## Exact frozen prompt

```text
You are about to help me create a complete code walkthrough for an existing
software subsystem. Do not inspect files, call tools, or start authoring. Return
an interactive intake protocol. First, quote the exact single question you would
ask in the first turn; it must be the only question in that quoted turn. Then
list the later questions in the order you would ask them, with an explicit rule
that you ask only one per turn and record its answer before continuing. Finish
with the contract you would freeze before inspecting code.
```
