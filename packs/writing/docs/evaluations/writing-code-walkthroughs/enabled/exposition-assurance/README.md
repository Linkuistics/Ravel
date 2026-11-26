# Skill-enabled exposition and assurance evaluation

## Outcome

The frozen Case C schedule completed with five valid contemporaneous controls
and five valid skill-enabled repetitions. Every scheduled repetition succeeded
on its first attempt, emitted one normal final answer, made no model tool call,
and left its fresh empty run directory unchanged. No replacement, truncation,
or sample shortfall occurred.

Both arms score `45/120` across all 24 rows (`37.5%`) and `43/110` after the two
compliance controls are excluded (`39.1%`). The control totals are
`10, 9, 9, 8, 9`; the enabled totals are `8, 9, 9, 11, 8`. Both means are `9.0`;
the control range is `8–10` and the enabled range is `8–11`.

The enabled arm exceeds the contemporaneous control by two successes only on
`C17` (independent editorial review, `4/5` versus `2/5`) and trails it by two on
`C23` (stable vocabulary plus an explicit actor for every effect, `2/5` versus
`4/5`). Every other non-compliance row is tied. The historical baseline has no
valid Case C sample or criterion classification, so the frozen material-change
test—which requires comparison against both historical and contemporary
controls—and the historical regression guard cannot be calculated. The `C17`
difference is descriptive, not a material-improvement verdict; the `C23`
difference is likewise a descriptive concern, not a classified regression.

The skill was not refined in this slice.

## Frozen execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- GNU timeout: `9.11`, 20 minutes with a 30-second kill grace
- Prompt SHA-256, including its canonical terminal LF:
  `e06829f69ad6b4a59761b2f9cd6fbadf1f8398b2b845ec3dc80231c91c54412a`
- Frozen rubric SHA-256:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Skill revision: `9cc8ccd8c5b8f070a572378bd61953f7b3bbb8ac`
- Installed `SKILL.md` SHA-256:
  `795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`
- Skill manifest aggregate SHA-256:
  `1f530317f817805eb88bf45f8ba66a0f135f8b279f36df2c5d864cfcd26251c1`
- Schedule: `control enabled enabled control enabled control control enabled control enabled`
- Evaluation root: `/private/tmp/grove-enabled-exposition.zA19sz`

The historical baseline record's `46051c…` prompt digest is the same text after
ordinary command substitution stripped its terminal LF. This enabled campaign
uses the canonical-LF command-argument construction established by the preceding
enabled slices, and both contemporary arms receive byte-identical arguments.
The historical Case C attempts produced no final answer, so they supply no score
whose interpretation could be changed by this byte distinction.

The sealed control template contains only `auth.json`. The enabled template
differs only by `skills/writing-code-walkthroughs/SKILL.md`; removing that row
from its manifest makes the template manifests identical. The skill bytes match
the preceding enabled slices. Every fresh home and template manifest is
preserved per attempt without preserving credential contents.

The exact command is the frozen command in
[`baseline/rubric.md`](../../baseline/rubric.md#common-execution-controls). The
prompt file is preserved as [`prompt.txt`](prompt.txt), and
[`harness.sh`](harness.sh) retains its terminal LF in the command argument with
a sentinel before verifying the digest.

## Attempt accounting

| Position | Arm | Repetition | UTC start | Wall time | Final bytes | Outcome |
|---:|---|---:|---|---:|---:|---|
| 1 | control | 1 | `2026-08-29T12:44:01Z` | 35 s | 5,276 | valid attempt 1 |
| 2 | enabled | 1 | `2026-08-29T12:44:57Z` | 31 s | 4,684 | valid attempt 1 |
| 3 | enabled | 2 | `2026-08-29T12:45:36Z` | 61 s | 4,947 | valid attempt 1 |
| 4 | control | 2 | `2026-08-29T12:46:44Z` | 31 s | 4,677 | valid attempt 1 |
| 5 | enabled | 3 | `2026-08-29T12:47:21Z` | 59 s | 4,619 | valid attempt 1 |
| 6 | control | 3 | `2026-08-29T12:48:26Z` | 33 s | 5,003 | valid attempt 1 |
| 7 | control | 4 | `2026-08-29T12:49:10Z` | 33 s | 5,556 | valid attempt 1 |
| 8 | enabled | 4 | `2026-08-29T12:49:50Z` | 28 s | 5,174 | valid attempt 1 |
| 9 | control | 5 | `2026-08-29T12:50:24Z` | 42 s | 4,131 | valid attempt 1 |
| 10 | enabled | 5 | `2026-08-29T12:51:18Z` | 57 s | 4,174 | valid attempt 1 |

All ten `exit-status.txt` files contain `0`, all ten `tool-events.jsonl` files
are empty, and every `run-pre.manifest.tsv` equals its corresponding post-run
manifest. There were no invalid attempts by any reason and no service-truncated
answers.

One enabled answer explicitly says it skipped “the walkthrough skill” because
tools were forbidden. No enabled raw stream contains a skill-file read or other
tool event. Observable exact or generic skill mention is therefore `1/5`,
observable file reads are `0/5`, and actual skill-body use is indeterminate; the
mention is retained as behavior rather than filtered.

## Resolved atomic counts

| ID | Historical baseline | Contemporary control | Enabled | Difference |
|---|---:|---:|---:|---:|
| `C01` | `0/0` | `5/5` | `5/5` | 0 |
| `C02` compliance | `0/0` | `2/5` | `2/5` | 0 |
| `C03` compliance | `0/0` | `0/5` | `0/5` | 0 |
| `C04` | `0/0` | `5/5` | `5/5` | 0 |
| `C05` | `0/0` | `0/5` | `0/5` | 0 |
| `C06` | `0/0` | `2/5` | `2/5` | 0 |
| `C07` | `0/0` | `5/5` | `5/5` | 0 |
| `C08` | `0/0` | `5/5` | `5/5` | 0 |
| `C09` | `0/0` | `5/5` | `5/5` | 0 |
| `C10` | `0/0` | `0/5` | `0/5` | 0 |
| `C11` | `0/0` | `0/5` | `0/5` | 0 |
| `C12` | `0/0` | `0/5` | `0/5` | 0 |
| `C13` | `0/0` | `0/5` | `0/5` | 0 |
| `C14` | `0/0` | `5/5` | `5/5` | 0 |
| `C15` | `0/0` | `0/5` | `0/5` | 0 |
| `C16` | `0/0` | `0/5` | `0/5` | 0 |
| `C17` | `0/0` | `2/5` | `4/5` | +2 |
| `C18` | `0/0` | `0/5` | `0/5` | 0 |
| `C19` | `0/0` | `0/5` | `0/5` | 0 |
| `C20` | `0/0` | `0/5` | `0/5` | 0 |
| `C21` | `0/0` | `0/5` | `0/5` | 0 |
| `C22` | `0/0` | `5/5` | `5/5` | 0 |
| `C23` | `0/0` | `4/5` | `2/5` | -2 |
| `C24` | `0/0` | `0/5` | `0/5` | 0 |

## Blind adjudication

One fresh no-skill context scored all ten answers from a randomized bundle with
arm labels and path names removed. A second fresh no-skill context independently
scored the same complete case without receiving the first score. Each produced
240 ordered decisions and a sample-total line.

The two scorers disagree on 28 rows: two `C02`, nine `C03`, seven `C05`, six
`C06`, and four `C23`. Both citations for every row and the frozen-rule
resolution are preserved under [`adjudication/`](adjudication/). The resolved
sample totals and criterion counts reproduce the arithmetic above.

## Limits

This completed contemporaneous sample supports descriptive Case C behavior and
variance. It does not repair the historical baseline shortfall and therefore
cannot establish same-case material improvement or regression under the frozen
endpoint. The model alias is not an immutable service snapshot, and the
operating-system readability limitation in the rubric remains: the evidence
establishes the audited model-interface boundary, not host filesystem
inaccessibility.
