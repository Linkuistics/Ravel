# No-skill code-walkthrough baseline report

## Result

The control campaign produced a usable scope-elicitation baseline and no
behavioral evidence for the other two cases. Case A completed five valid,
blind-scored repetitions. Cases B and C exhausted all permitted attempts after
transport and DNS failures; neither emitted a final answer. Their missing
samples are not scored as failures and do not justify skill guidance.

The primary endpoint for the later skill-enabled evaluation is therefore the
fifteen non-compliance, non-deterministic Case A criteria classified as repeated
judgment gaps. At least ten of those fifteen must improve materially under the
frozen endpoint rule. The regression guard contains the four Case A criteria
present in all five controls.

## Frozen inputs and retained outcomes

- The unchanged [rubric](rubric.md) defines the prompts, fixtures, atomic
  criteria, scoring, classifications, primary endpoint, regression guard, and
  transfer probe. Its SHA-256 at execution was
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`.
- [Scope elicitation](scope-elicitation/README.md) retains five valid answers,
  their resolved scores, both blind adjudications, and five earlier invalid
  infrastructure attempts.
- [External source inventory and fragments](source-fragments/README.md) retains
  all fifteen invalid attempts, the unchanged external fixture manifests, and
  the five-sample shortfall.
- [Exposition and assurance](exposition-assurance/README.md) retains all fifteen
  invalid attempts and the five-sample shortfall.

The case reports link each planned repetition to its raw JSONL, final answer
when one exists, stderr, and access and state manifests. This report aggregates
those records; it does not replace or discard them.

## Campaign aggregate

| Case | Planned repetitions | Valid repetitions | Invalid attempts | Scored decisions | Successes | Truncated criteria | Re-score disagreements |
|---|---:|---:|---:|---:|---:|---:|---:|
| A — scope elicitation | 5 | 5 | 5 | 105 | 28 | 0 | 2 |
| B — source and fragments | 5 | 0 | 15 | 0 | n/a | n/a | n/a |
| C — exposition and assurance | 5 | 0 | 15 | 0 | n/a | n/a | n/a |
| **Campaign** | **15** | **5** | **35** | **105** | **28** | **0** | **2** |

This is a baseline-only report, so enabled-arm repetitions and skill-discovery
counts are not applicable. They become required outcomes in
`skill-evaluation-k22`.

All five valid Case A runs exited normally without tool calls, writes, injected
messages, or service truncation. The primary and independent adjudicators
agreed on 103 of 105 decisions. The two disagreements, `A10` in repetition 5
and `A19` in repetition 3, were resolved to `0` because a revision anchor did
not govern a changing corpus and a general evidence question did not separately
elicit mechanical proof.

Every invalid attempt exited `124` after 1,200 seconds without a final assistant
message. The preserved streams identify failed DNS lookup and transport
reconnection as the shared cause. Case A succeeded on replacement attempt two
after network access was restored. Cases B and C exhausted the initial attempt
and both allowed replacements, as the frozen invalid-run rule required.

## Case A classifications

| Classification | Atomic criteria | Count |
|---|---|---:|
| Repeated judgment gap | `A05`–`A13`, `A16`–`A21` | 15 |
| Present in this sample | `A02`, `A03`, `A14`, `A15` | 4 |
| Mixed | `A01`, `A04` | 2 |

The aggregate success count is 28 of 105 atomic decisions. `A02`, `A03`,
`A14`, and `A15` each scored 5/5; `A01` and `A04` each scored 4/5; every
repeated-gap criterion scored 0/5.

### Recurring omissions and substitutions

The controls consistently produced a plausible general intake and stopped
before the decisions needed for a complete-source walkthrough:

- Broad scope questions such as “What code boundaries are in scope?” and “What
  artifacts should the walkthrough include?” substituted for separately
  identifying manifests, production source, tests and fixtures, models,
  generated files, examples, and dependencies (`A05`–`A08`).
- Revision or change-policy language did not ask whether the corpus may change
  and which artifact becomes authoritative if it does (`A09`–`A10`). One answer
  froze an “exact revision or workspace state”; another said to “pause and
  re-freeze,” but neither supplied both decisions.
- “Who is the audience?” and “what level of prior context can I assume?”
  substituted for separate language, systems/tooling, and domain-proficiency
  questions (`A11`–`A13`).
- General constraints and evidence questions did not elicit walk-away behavior,
  a complete prose/citation contract, navigation rules, or mechanical proof as
  a category distinct from judgment (`A16`–`A19`). Representative controls
  asked “What should count as sufficient evidence?” or offered an
  `Evidence standard` field without naming executable proof.
- “Who will review or approve the walkthrough?” treated approval as one role
  and did not establish independent technical and editorial reviews
  (`A20`–`A21`).

The answers did not explicitly defend these omissions. The representative
language above records the recurring substitution: one broad question was
treated as sufficient for several narrower contracts. This is the only
rationalization visible in the returned protocols; no rationale is inferred
beyond their text.

## Authoring evidence for `writing-code-walkthroughs`

The future skill may add only the following behavior-shaping rules from this
historical baseline. Each rule maps entirely to repeated observed gaps.

| Candidate rule | Repeated-gap evidence | Required behavior |
|---|---|---|
| Freeze the source taxonomy explicitly | `A05`–`A08`, all 0/5 | Elicit included manifests and production source, then separately classify tests and fixtures, models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| Govern corpus changes | `A09`–`A10`, all 0/5 | Ask whether source may change during authoring and name the authoritative artifact if it does. |
| Split audience proficiency | `A11`–`A13`, all 0/5 | Elicit language proficiency, systems or tooling proficiency, and domain familiarity separately. |
| Preserve walk-away readability | `A16`, 0/5 | Ask what source and Markdown must remain usable when walkthrough-specific tooling is absent. |
| Freeze prose and navigation contracts | `A17`–`A18`, all 0/5 | Elicit prose, terminology, citation, navigation, and cross-reference constraints and record them in the frozen contract. |
| Separate proof and review roles | `A19`–`A21`, all 0/5 | Elicit mechanical proof separately from judgment and establish independent technical and editorial review requirements. |

These rules govern author decisions and intake behavior. They do not turn the
rubric's deterministic properties into prose-only obligations.

## Behavior excluded from the authoring brief

No new skill wording is justified for the following behavior:

- One-question-per-turn discipline and recording each answer (`A02`, `A03`) were
  present in all five controls.
- Explicit depth and output-form elicitation recorded in the contract (`A14`,
  `A15`) were present in all five controls.
- A first turn containing exactly one question and a later subsystem-boundary
  question (`A01`, `A04`) were mixed at 4/5, below the repeated-gap threshold.
- None of the Case B or Case C behavior can be called failed, reliable, or
  mixed because those cases produced no valid answer.

The four present-in-sample criteria form the historical regression guard
`G = {A02, A03, A14, A15}`. In the enabled evaluation each must lose at most one
success relative to both this 5/5 historical count and the contemporaneous
no-skill count.

## Mechanical enforcement boundary

The frozen rubric assigns deterministic syntax, inventory, fragment-graph,
link, reachability, exact-coverage, and byte-equality properties to mechanical
tooling. A skill can require use of such a check only when omission of the proof
step is itself a repeated gap. Case A establishes only the intake-level gap in
asking for mechanical proof (`A19`); it does not establish omission of any
particular validator.

Case B would have tested exact inventory, fragment ownership and expansion, and
a worked execution. Case C would have tested local exposition, repetition versus
links, Markdown checks, independent reviews, and walk-away behavior. With no
valid samples, none of those atomic criteria enters the candidate-rule set.
The Case B fixture's verification comment also remains a stated priming
limitation for any future absolute reliability claim.

The frozen compliance controls are reported separately from behavioral
classifications: `B01`, `C02`, and `C03` received no score because Cases B and C
had no valid answer. They do not enter `R`, `G`, or candidate-rule decisions.

## Evaluation handoff

The enabled evaluation must use the byte-unchanged [rubric](rubric.md), prompts,
fixture, criteria, and scoring rules represented by the execution hash above.
The three case directories linked under [Frozen inputs and retained
outcomes](#frozen-inputs-and-retained-outcomes) are the historical control arm.

For the same-case primary endpoint:

- `R = {A05, A06, A07, A08, A09, A10, A11, A12, A13, A16, A17, A18, A19, A20, A21}`;
- `|R| = 15`, so at least `ceil(2 × 15 / 3) = 10` criteria must improve
  materially;
- each criterion must exceed both its historical success count of 0/5 and its
  contemporaneous no-skill count by at least two of five;
- `G = {A02, A03, A14, A15}` supplies the regression guard described above;
- `A01` and `A04`, all deterministic rows, compliance controls `B01`, `C02`,
  and `C03`, and all Case B and C unscored criteria are outside the primary
  endpoint.

The enabled campaign still runs every unchanged case, five contemporaneous
controls per case, and the separately frozen transfer probe. Case B and Case C
need valid contemporaneous samples to establish any comparative result; this
report supplies no historical success count for them and does not substitute
zero.

## Limitations

This baseline measures returned artifacts, not reader outcomes or general
walkthrough capability. Its only behavioral evidence comes from one prompt,
one model alias, one CLI version, and five Case A repetitions. The two unexecuted
behavioral surfaces make the baseline incomplete for source/fragment authoring
and exposition/assurance. The later evaluation must report that incompleteness
even if contemporaneous reruns succeed.
