# `writing-code-walkthroughs` evaluation report

## Verdict

The unchanged acceptance rubric is **not met**. Its primary material-improvement
endpoint is unreachable under every completion of the missing fifth enabled
scope-elicitation sample: at most 2 of the required 10 rows could improve by
`2/5`. The `A14` regression guard also fails under every completion because its
enabled count can reach at most `3/5` against the historical `5/5`. One selected
contemporary control breached the byte-identical prompt contract, and the
source-and-fragments case and transfer probe stopped with sample shortfalls.
Those missing samples are not failures, but they leave no valid basis for a
deployment-wide behavioral improvement claim.

The campaign does establish narrower facts:

- the historical no-skill sample repeatedly omitted 15 scope-intake behaviors;
- the complete pre-refinement exposition sample showed equal aggregate scores
  in the enabled and contemporary-control arms, with two criterion-level
  differences of two successes in opposite directions;
- after a generic actor-wording refinement, a later enabled round scored `4/5`
  on `C23` rather than the earlier round's `2/5`, while seven named Case C
  preservation rows remained `5/5`; the fresh control scored `5/5` on `C23`;
- the final skill and plugin pass deterministic structure, installation, local
  link, and applicable repository checks, while the historical harnesses
  correctly refuse the final skill's changed bytes.

The judged-output evidence therefore supports only the retained samples and the
chronology of one evidence-driven wording change. Because valid outputs do not
show that they read or used the skill body, it does not establish that the
wording caused the later count, that installing the skill causes materially
better walkthrough behavior, or that the method transfers to a new codebase.

## Frozen comparison contract

The byte-unchanged [rubric](baseline/rubric.md) defines three same-case prompts,
binary atomic scoring, five repetitions per arm, an interleaved contemporary
control, invalid-run replacement limits, and a separate out-of-sample transfer
probe. The executed rubric SHA-256 was
`54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`.

The primary endpoint set is
`R = {A05, A06, A07, A08, A09, A10, A11, A12, A13, A16, A17, A18, A19, A20, A21}`.
A criterion improves materially only if its enabled count exceeds both the
historical and contemporary-control count by at least `2/5`. Acceptance requires
at least 10 of the 15 rows in `R` to improve materially. The regression guard is
`G = {A02, A03, A14, A15}`; each row may lose at most one success against both
controls.

The rubric does not define partial-sample arithmetic. A missing fifth sample,
protocol-breached selected sample, or absent historical arm prevents the
corresponding comparison rather than supplying a zero.

The initial skill-enabled Case A, Case B, and Case C arms and the transfer probe
used skill SHA-256
`795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`.
Only the five refined-skill repetitions used the deployed skill SHA-256
`7bfd60fe825c85a40a49cfe0da4cb450e0cff6099dae586ea8aafb2c6262d9a7`;
historical and contemporary-control arms installed no skill.

## Campaign status and aggregate results

| Surface | Historical baseline | Contemporary comparison | Enabled or refined result | Frozen verdict |
|---|---|---|---|---|
| A — scope elicitation | `5/5` valid; `28/105` successes | five selected answers, one prompt-byte breach; `30/105` descriptive successes | `4/5` valid; `25/84` descriptive successes | primary endpoint unreachable and `A14` guard breached under every completion; remaining comparisons incomplete |
| B — source and fragments | `0/5` valid after 15 DNS/transport timeouts | one descriptive, access-audit-breached control | `0/5` valid; first enabled repetition exhausted three invalid attempts | no comparison |
| C — exposition and assurance | `0/5` valid historically | `5/5` valid; `45/120` successes | `5/5` valid; `45/120` successes | descriptive only; historical comparison absent |
| C refinement rerun | not applicable | `5/5` valid; `55/120` successes | `5/5` valid; `57/120` successes | bounded within-skill evidence only |
| Transfer probe | separately frozen after the skill | `0/5` valid | `1/5` valid refusal | transfer not established |

The historical Case A totals have mean `5.6/21` and range `5–6`; its later
contemporary totals have mean `6.0/21` and range `5–7`; the four valid enabled
totals have mean `6.25/21` and range `5–7`. Unequal denominators and the breached
control contract make an aggregate delta unavailable. The complete
pre-refinement Case C arms both have mean `9.0/24`; control totals range `8–10`
and enabled totals `8–11`. The refined Case C mean is `11.4/24`, compared with
`11.0/24` for its fresh control, and both ranges are `10–13`. The two-point
refinement aggregate difference is not a rubric endpoint and does not override
the criterion-level rules.

## Per-behavior comparison

### Case A — scope elicitation

The historical baseline classified `A05`–`A13` and `A16`–`A21` as repeated
judgment gaps, `A02`, `A03`, `A14`, and `A15` as present in all five samples,
and `A01` and `A04` as mixed. The later partial counts remain descriptive.

| ID | Behavior | Historical | Contemporary | Enabled valid sample | Interpretation |
|---|---|---:|---:|---:|---|
| `A01` | exactly one first-turn question | `4/5` | `4/5` | `3/4` | mixed; outside endpoint |
| `A02` | one question per turn | `5/5` | `5/5` | `4/4` | guard row; verdict unavailable |
| `A03` | record each answer before continuing | `5/5` | `5/5` | `4/4` | guard row; verdict unavailable |
| `A04` | bound the target subsystem | `4/5` | `5/5` | `3/4` | mixed; outside endpoint |
| `A05` | identify included manifests | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A06` | identify production source | `0/5` | `1/5` | `4/4` | favorable partial count; no endpoint verdict |
| `A07` | classify tests and fixtures | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A08` | classify models, generated files, examples, dependencies | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A09` | ask whether the corpus may change | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A10` | name authority if the corpus changes | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A11` | elicit language proficiency separately | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A12` | elicit systems/tooling proficiency separately | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A13` | elicit domain familiarity separately | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A14` | elicit and record explicit depth | `5/5` | `5/5` | `2/4` | guard breached under every completion: at most `3/5` against historical `5/5` |
| `A15` | elicit and record output form | `5/5` | `5/5` | `4/4` | guard row; verdict unavailable |
| `A16` | establish walk-away behavior | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A17` | freeze prose, terminology, and citation constraints | `0/5` | `0/5` | `1/4` | endpoint row; no complete comparison |
| `A18` | freeze navigation and cross-reference constraints | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A19` | separate mechanical proof from judgment | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A20` | require independent technical review | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |
| `A21` | require independent editorial review | `0/5` | `0/5` | `0/4` | endpoint row; no complete comparison |

Seven enabled attempts visibly read `SKILL.md` and were invalid because Case A
forbids every tool call. The four valid enabled streams contain no observable
skill-file read or use. Discovery and skill-body use are therefore
indeterminate, and the partial counts cannot be attributed to the instructions.
The invalidation rule is treatment-correlated: it discarded every attempt with
visible skill use and retained exactly the attempts without it, so the enabled
sample is selected against observable treatment use. See the
[Case A record](enabled/scope-elicitation/README.md).

### Case B — source inventory and fragments

No per-behavior delta exists. The historical arm produced no valid sample. The
enabled campaign stopped after its first enabled repetition exhausted all three
attempts on prohibited model-interface accesses; its sole control answer has an
access-audit protocol breach and a descriptive resolved score of `1/27`.
Consequently none of `B01`–`B27` has a comparable enabled count, variance, or
materiality classification. This surface provides no evidence for or against
inventory accuracy, concept order, fragment design, worked executions, or
mechanical exactness. See the [historical](baseline/source-fragments/README.md)
and [enabled](enabled/source-fragments/README.md) records.

### Case C — exposition and assurance

The historical arm produced no valid sample, so these complete contemporary
counts describe behavior and variance but cannot satisfy the rubric's
historical-plus-contemporary materiality test. `C02` and `C03` are compliance
controls.

| ID | Behavior (abridged) | Control | Enabled | Delta | Refined control | Refined skill |
|---|---|---:|---:|---:|---:|---:|
| `C01` | repeat `NormalizedKey` meaning locally | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C02` | connect meaning without invention | `2/5` | `2/5` | 0 | `1/5` | `2/5` |
| `C03` | mark unknown details explicitly | `0/5` | `0/5` | 0 | `4/5` | `3/5` |
| `C04` | do not duplicate source fragment | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C05` | repeat only the current consequence | `0/5` | `0/5` | 0 | `1/5` | `2/5` |
| `C06` | purposeful link labels | `2/5` | `2/5` | 0 | `4/5` | `5/5` |
| `C07` | paragraph survives link removal | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C08` | direct declarative prose | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C09` | omit audience-granted mechanics | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C10` | validate frozen inventory | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C11` | validate fragment graph | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C12` | validate coverage and byte equality | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C13` | validate Markdown and navigation | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C14` | independent technical review | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C15` | technical-review scope | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C16` | evidence-gated concurrency/rollback review | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C17` | distinct independent editorial review | `2/5` | `4/5` | +2 | `4/5` | `5/5` |
| `C18` | editorial-review scope | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C19` | check missing context and redundancy | `0/5` | `0/5` | 0 | `1/5` | `1/5` |
| `C20` | remove custom tooling in walk-away check | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C21` | source and raw Markdown remain readable | `0/5` | `0/5` | 0 | `0/5` | `0/5` |
| `C22` | lead with the technical claim | `5/5` | `5/5` | 0 | `5/5` | `5/5` |
| `C23` | stable vocabulary and actor for every effect | `4/5` | `2/5` | -2 | `5/5` | `4/5` |
| `C24` | distinguish refusals, failures, and defects | `0/5` | `0/5` | 0 | `0/5` | `0/5` |

The initial enabled arm had a `+2` descriptive difference on `C17` and a `-2`
difference on `C23`; all other non-compliance rows tied. The only wording change
replaced a qualified actor rule with the generic positive contract “Name the
actor for every effect.” In the later round `C23` was `4/5` rather than `2/5`,
while one answer retained the weaker “important sentence” qualifier. This is an
across-round descriptive association, not evidence that the wording caused the
change: the fresh control was `5/5`, and valid streams do not establish skill-body
use. The seven successful pre-refinement preservation rows `C01`, `C04`, `C07`,
`C08`, `C09`, `C14`, and `C22` remained `5/5`; they are not the frozen rubric's
Case A regression guard. See the [initial Case C record](enabled/exposition-assurance/README.md)
and [refinement record](refinement-regression/README.md).

### Transfer probe

The separately frozen probe selected `junegunn/fzf` at commit
`15f64c492a08f0840b81540c7d1de35737448086`, bounded to `bin/fzf-tmux`, and
froze 16 non-compliance criteria before execution. It required at least 8 rows
to improve by `2/5` with no regression of two or more.

Twenty of 21 attempts violated the declared model-interface boundary. The
remaining enabled attempt was a valid refusal. With `0/5` valid controls and
`1/5` valid enabled samples, atomic comparative scoring and a transfer verdict
are undefined. The transfer claim is **not established**; invalid answers are
not scored as behavioral failures. See the [transfer record](enabled/transfer-probe/README.md).

## What the evidence proves

### Judged-output evidence

The blind atomic scoring establishes the reported behavior of the retained
answers under the frozen prompts and binary criteria. It supports:

- the 15 repeated historical Case A omissions;
- equal aggregate pre-refinement Case C performance with the two stated
  criterion-level differences;
- the across-round `C23` count change after the wording refinement, without a
  causal attribution; and
- preservation of the named Case C regression rows.

These are sample-specific behavior claims. They remain judgment-dependent:
adjudicators disagreed on 2 of 105 historical Case A decisions and 28 of 240
initial Case C decisions before frozen-rule resolution. The two valid refinement
scorers disagreed on 6 of 240 decisions, including one decision on preservation
row `C09`. The enabled Case A arm and the Case B contemporary control were each
scored by only one blind context.

### Deterministic verification

Mechanical checks establish properties of the final skill SHA-256
`7bfd60fe825c85a40a49cfe0da4cb450e0cff6099dae586ea8aafb2c6262d9a7`
and this repository working copy rather than model behavior. The final run used:

| Property | Exact check | Result |
|---|---|---|
| final skill structure | `bash docs/evaluations/writing-code-walkthroughs/refinement-regression/template-test.sh` | frontmatter, plugin manifest, and skill bytes/digest pass |
| installation and reconciliation | `bash plugins/install.test.sh` | 20 passed, 0 failed |
| historical digest guards | `bash docs/evaluations/writing-code-walkthroughs/enabled/exposition-assurance/harness-test.sh`; `bash docs/evaluations/writing-code-walkthroughs/enabled/transfer-probe/harness-test.sh` | both reject drift and pass |
| transfer freeze template | `bash docs/evaluations/writing-code-walkthroughs/enabled/transfer-probe/freeze-harness-test.sh` | pass |
| Bash static analysis | `find docs/evaluations/writing-code-walkthroughs -type f -name '*.sh' -print0 \| xargs -0 shellcheck` | all eight retained shell scripts pass |
| local Markdown links | `cargo test --test reference_navigation every_repository_markdown_reference_resolves` | 1 passed, 0 failed |
| repository tests | `cargo test --workspace --all-targets` | pass, 0 failed |
| formatting | `cargo fmt --all -- --check` | pass |
| lint baseline | `cargo clippy --workspace --all-targets` | pass with no diagnostics |

These checks cannot guarantee that a model discovers, reads, or obeys the skill,
nor that a produced walkthrough is technically or editorially sound.

## Unresolved gaps

- The primary `10/15` material-improvement threshold is unreachable and the
  `A14` regression guard is breached under every completion of the missing
  enabled sample. Endpoint rows `A06` and `A17` and guard rows `A02`, `A03`, and
  `A15` still lack complete comparisons.
- Case B has no valid historical or enabled behavioral sample.
- Case C has no historical baseline, so its complete contemporary and
  refinement results cannot establish same-case material improvement.
- The transfer probe has no valid control arm and only one valid enabled
  refusal.
- Skill discovery and body use are indeterminate in valid Case A and C samples.
  In Case A the frozen no-tool prompt conflicts structurally with observable
  file-based skill loading: visible use invalidates an enabled attempt, selecting
  the retained arm against the treatment it is meant to measure. Closing this
  gap requires a new predeclared prompt or access rule, not another run under the
  frozen contract.
- All judged skill-enabled evidence except the five-repetition Case C refinement
  rerun describes the superseded `795846cb…` skill revision rather than the
  deployed `7bfd60fe…` revision.
- The refinement slice preserves the underlying per-attempt evidence but not the
  rubric's required one-Markdown-record-per-repetition shape or a reusable
  campaign harness.
- The model name is a mutable service alias, and the preserved interface audit
  does not prove operating-system filesystem inaccessibility.
- The campaign measures returned artifacts, not reader comprehension or general
  walkthrough quality.

Closing these gaps requires a new predeclared evaluation cycle. This report does
not reopen, repair, or reinterpret the frozen rubric.
