# Skill refinement and regression evaluation

## Outcome

The only wording change in this slice closes a scored generic loophole in
Case C. Pre-refinement enabled repetitions 2, 3, and 5 failed `C23`: their prose
rules required actors only for “important” or “key operational” sentences,
while the frozen criterion requires an explicit actor for every effect. The
skill now states: “Name the actor for every effect.” No incomplete Case A or B
sample and no transfer-probe outcome caused a wording change.

The unchanged Case C prompt was rerun on the frozen interleaved schedule with
five fresh no-skill controls and five fresh refined-skill contexts. All ten
attempts exited normally on attempt 1, emitted no tool event, and left their
empty run directories unchanged. The refined arm scores `57/120`; the
contemporaneous control scores `55/120`.

`C23` improves from the pre-refinement enabled count of `2/5` to `4/5`. The
remaining refined miss says “Every important sentence names the actor,” which
is still weaker than the frozen criterion. The contemporaneous control is
`5/5`, so this result establishes a bounded within-skill improvement, not a
comparative material-improvement or campaign-acceptance claim. Actual skill-body
use remains indeterminate because the frozen prompt prohibits tool calls and no
valid run visibly read `SKILL.md`.

The regression rows that were already successful in the pre-refinement Case C
sample remain successful: `C01`, `C04`, `C07`, `C08`, `C09`, `C14`, and `C22`
are all `5/5` in the refined arm. Distinct independent editorial review (`C17`)
rises from `4/5` to `5/5`. The rubric, prompt, atomic criteria, scoring rule,
sample size, schedule, and contamination controls are unchanged.

## Evidence-to-wording map

| Change | Scored evidence | Form |
|---|---|---|
| Replace “explicit actors” with “Name the actor for every effect.” | Pre-refinement enabled `C23=0` in repetitions 2, 3, and 5; the frozen resolution identifies “important sentence” as the loophole | Positive output contract for a wrong-shaped prose rule |

The sentence is generic: it names neither `ordinal-fs-tree` nor the evaluation's
key-value-service vocabulary. It adds no rule for infrastructure failures,
mixed evidence, transfer outcomes, or deterministic properties.

## Execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- GNU timeout: 20 minutes with 30-second kill grace
- Frozen prompt SHA-256:
  `e06829f69ad6b4a59761b2f9cd6fbadf1f8398b2b845ec3dc80231c91c54412a`
- Frozen rubric SHA-256:
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`
- Pre-refinement installed skill SHA-256:
  `795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`
- Refined installed skill SHA-256:
  `7bfd60fe825c85a40a49cfe0da4cb450e0cff6099dae586ea8aafb2c6262d9a7`
- Schedule: `control enabled enabled control enabled control control enabled control enabled`
- Evaluation root: `/private/tmp/grove-refinement-exposition.oJ8gD1`

The control template contained only `auth.json`. The refined template differed
only by `skills/writing-code-walkthroughs/SKILL.md`. Credential contents and
temporary homes are not preserved. [`skill.manifest.tsv`](skill.manifest.tsv),
[`codex-version.txt`](codex-version.txt), and
[`codex-executable.sha256`](codex-executable.sha256) preserve the non-secret
execution identity.

## Attempt accounting

| Arm | Repetitions | Valid | Invalid | Tool events | Changed run directories |
|---|---:|---:|---:|---:|---:|
| control | 5 | 5 | 0 | 0 | 0 |
| refined skill | 5 | 5 | 0 | 0 | 0 |

Every repetition's raw JSONL, final answer, stderr, timestamps, exit status,
tool-event audit, and pre/post manifests are under [`evidence/`](evidence/).

## Resolved atomic counts

| ID | Pre-refinement enabled | Contemporary control | Refined skill |
|---|---:|---:|---:|
| `C01` | 5/5 | 5/5 | 5/5 |
| `C02` compliance | 2/5 | 1/5 | 2/5 |
| `C03` compliance | 0/5 | 4/5 | 3/5 |
| `C04` | 5/5 | 5/5 | 5/5 |
| `C05` | 0/5 | 1/5 | 2/5 |
| `C06` | 2/5 | 4/5 | 5/5 |
| `C07` | 5/5 | 5/5 | 5/5 |
| `C08` | 5/5 | 5/5 | 5/5 |
| `C09` | 5/5 | 5/5 | 5/5 |
| `C10` | 0/5 | 0/5 | 0/5 |
| `C11` | 0/5 | 0/5 | 0/5 |
| `C12` | 0/5 | 0/5 | 0/5 |
| `C13` | 0/5 | 0/5 | 0/5 |
| `C14` | 5/5 | 5/5 | 5/5 |
| `C15` | 0/5 | 0/5 | 0/5 |
| `C16` | 0/5 | 0/5 | 0/5 |
| `C17` | 4/5 | 4/5 | 5/5 |
| `C18` | 0/5 | 0/5 | 0/5 |
| `C19` | 0/5 | 1/5 | 1/5 |
| `C20` | 0/5 | 0/5 | 0/5 |
| `C21` | 0/5 | 0/5 | 0/5 |
| `C22` | 5/5 | 5/5 | 5/5 |
| `C23` | 2/5 | 5/5 | 4/5 |
| `C24` | 0/5 | 0/5 | 0/5 |

The two valid blind scorers disagree on 6 of 240 decisions, including `S02 C09`
on a preservation row. `S08 C23` is the disagreement on the refinement target;
all six disagreements are resolved under the same score-1-only rule in
[`adjudication/resolution.md`](adjudication/resolution.md).

## Limits and unresolved acceptance gaps

This rerun does not repair the historical Case C shortfall, the incomplete Case
A and B enabled samples, or the transfer-probe shortfall. It cannot establish
the campaign's primary material-improvement endpoint or transfer claim. It also
does not justify new wording for `C10`–`C13`, `C15`–`C16`, or `C18`–`C24`:
those rows either lack historical candidate-rule evidence, concern deterministic
checks, or remain outside this leaf's demonstrated actor-wording gap.

The surviving `C23` miss is reported, not hidden by another wording iteration.
The skill already contains the exact positive contract; the valid stream gives
no evidence that the body was read, so strengthening unrelated frontmatter or
adding domain-shaped repetition would not be evidence-supported.
