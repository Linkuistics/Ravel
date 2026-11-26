# No-skill code-walkthrough baseline rubric

## Purpose and freeze boundary

This rubric defines the control campaign used before the
`writing-code-walkthroughs` skill exists. Commit `baseline-rubric-k38` made the
initial predeclaration; the review-integration commit `baseline-rubric-k44` is
the final freeze point for this file and its external fixture before any
evaluated run. Baseline and skill-enabled runs use the same prompts, fixtures,
atomic criteria, and
scoring. Leaves that run scenarios append outcomes and reports but do not edit
this file.

The campaign measures behavior visible in returned protocols, plans, prose, and
assurance artifacts. It does not claim reader outcomes. Because the skill will
be authored from observed control gaps, improvement on these same cases is
evidence only that those gaps were addressed; it is not evidence of general
walkthrough capability.

After the skill bytes are frozen, `skill-evaluation-k22` also commissions a
transfer probe. A fresh selector given only the campaign's subject and transfer
constraints—not the skill bytes, completed book, baseline outcomes, or selected
codebase—selects and freezes a bounded codebase and prompt absent from all four.
A separate criterion author freezes any case-specific criteria before receiving
the selected codebase's source. The probe then runs five contemporaneous
no-skill and five skill-enabled repetitions under this rubric's execution and
atomic-scoring rules. It succeeds only when at least half of its non-compliance
criteria improve by at least two of five over the contemporaneous control and no
criterion regresses by two or more. All counts and the verdict are reported
regardless of outcome. The probe is reported separately and cannot cause skill
wording changes without a new evaluation cycle.

## Common execution controls

Each frozen case has five valid baseline repetitions. Every repetition uses a
new empty directory created under the system temporary directory, with its
resolved path outside this repository and outside any ancestor containing the
repository, plus a new ephemeral Codex context. Record that absolute path. The
runner allows one Codex turn, applies a 20-minute wall-clock limit with a
30-second termination grace, applies no client-side output-byte truncation, and
uses Codex CLI `0.150.1` with this command shape:

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

The model name is a service alias, not an immutable runtime snapshot. Record the
CLI version, executable SHA-256, model, reasoning effort, complete flags, UTC
start time, wall-clock outcome, whether the final message terminated normally,
and raw JSONL event stream. Enabled evaluation must use the same CLI version and
command shape. It also reruns five contemporaneous no-skill controls per case,
interleaved ABBA/BAAB across the enabled repetitions, so model-service or harness
drift is visible rather than attributed to the skill.

Each repetition receives a fresh copy of one sealed, purpose-built `CODEX_HOME`
template. The control template contains only the authentication material needed
to start Codex: it has no `skills/`, `AGENTS.md`, `AGENTS.override.md`,
`hooks.json`, plugins, MCP configuration, or `config.toml`. Disabling bundled
skills in the command prevents Codex from populating `skills/.system`. The
enabled template differs only by the installed
`skills/writing-code-walkthroughs/` subtree. Contemporaneous controls always use
the control template; enabled repetitions always use the enabled template, so no
shared home is installed or uninstalled between adjacent runs.

Before every repetition, record the selected template's recursive
path-and-SHA-256 manifest and the fresh home copy's manifest. Afterward record
all Codex-created state separately. Enabled evaluation also records the target
skill's VCS revision, recursive manifest, aggregate digest, installation path,
discovery mechanism, and complete command. After removing only the target skill
subtree, the two sealed template manifests must be identical. The user prompt is
byte-identical in both arms and contains no explicit skill invocation. Failure
to discover or use the skill is a valid enabled sample; report discovery count
per case as an outcome rather than filtering it from the sample.

The Codex filesystem sandbox does not prove that only the run directory is
operating-system-readable. The auditable boundary is therefore stricter at the
model interface:

- Cases A and C forbid tool calls; a raw event containing one invalidates the
  repetition.
- Case B permits read-only commands whose operands stay under the run directory
  and access only its declared fixture. Preserve every tool call and output; a
  call outside that boundary invalidates the repetition.
- Record the exact user prompt, the startup skill/config manifests above, every
  harness- or hook-injected message (normally none under the sealed templates),
  every file path and digest in the run directory, and every tool result
  delivered to the model. Do not claim that an unenumerated host filesystem was
  inaccessible.
- Never place the Grove tree, completed book, walkthrough research synthesis,
  rubric, draft skill, repository agent instructions, or prior conversation in
  a run directory or prompt.

The post-run manifest includes path, byte length, and SHA-256, not path alone.
Any unexpected write or content change invalidates the repetition.

## Invalid runs and sampling

Use the first final answer emitted by a valid context. A run is replaceable only
when the process exits nonzero, emits an explicit failed/cancelled response
status, emits no final assistant message, violates a declared tool-access rule,
or mutates the run directory. Truncation, refusal, irrelevant content, and poor
substantive answers remain valid samples when a final message exists.
A wall-clock termination emits no final answer and is therefore replaceable; it
is not scored as truncation.

Permit at most two replacement attempts for each planned repetition. Preserve
every invalid attempt and its machine-checkable reason. Report invalid-attempt
counts by reason for each case and arm. If the third attempt is invalid, stop the
leaf and report the sample shortfall; do not select a different answer to
complete the quota.

## Atomic scoring and classifications

One fresh adjudicating context, distinct from the sessions that authored the
skill and produced the runs, scores both arms from randomized bundles whose arm
labels and path names have been stripped. It receives final-answer text and raw
events but no skill bytes or campaign hypothesis. One complete case is scored a
second time by another independent blind context; report the criterion-level
disagreement count and both citations before resolving disagreements under the
same rules.

Score each atomic criterion from the final answer and raw events:

- `0` — the named behavior has no directly citable evidence or is contradicted.
- `1` — the named behavior is explicitly present. Cite the smallest supporting
  passage or event.

For an absence-shaped criterion, score `1` only after an exhaustive scan of its
named surface finds no prohibited instance; cite the surface and the closest
near-miss considered. Score `0` at the first prohibited instance and cite it.
If a valid final answer is service-truncated, mark criteria whose necessary
surface lies beyond the truncation point as `truncated`, not `0`; report them
separately and classify that criterion as `incomplete due to truncation` rather
than as a baseline gap.

Do not award partial credit, infer intent, or let one bundled impression replace
the individual rows. A rationale earns no point unless its criterion's behavior
is present. When a criterion requires an example, a plan without the example is
zero; when it requires a plan, an unsupported promise is zero.

For each criterion across five baseline repetitions:

- `repeated gap`: score `0` in at least three repetitions;
- `mixed`: score `0` in one or two repetitions;
- `present in this sample`: score `1` in all five repetitions.

Rows marked `compliance control` measure obedience to an answer supplied by the
prompt. Report them separately; they are excluded from the three classifications
above, from reliable-without-guidance claims, and from skill-rule decisions.

These labels describe only this sample. The final report may propose skill
guidance only for a repeated gap that requires author judgment. A deterministic
syntax, inventory, graph, link, or byte property is routed to mechanical tooling;
guidance may require running that tool only when omission of the proof step is
itself a repeated gap. Mixed and present-in-sample behaviors do not justify new
skill wording.

For same-case enabled evaluation, an atomic criterion improves materially when
its enabled success count exceeds both the historical baseline and the
contemporaneous no-skill count by at least two of five. Compliance controls and
criteria incomplete due to truncation are excluded. Report all counts even when
this condition is unmet.

The campaign's primary endpoint is the set `R` of non-compliance,
non-deterministic criteria the baseline report classifies as repeated judgment
gaps. The campaign meets the
material-improvement requirement only when at least `ceil(2 × |R| / 3)` members
of `R` improve materially. Its separate regression guard is the set `G` of
non-compliance criteria classified present in the baseline sample: every member
of `G` must lose at most one success relative to both its historical and
contemporaneous control counts. Deterministic rows, mixed evidence, compliance
controls, transfer outcomes, and improvements outside `R` are secondary and
cannot establish the primary endpoint. If `R` is empty, the endpoint is unmet
rather than redefined after seeing enabled outcomes.

## Case A — scope elicitation protocol

### Frozen prompt

```text
You are about to help me create a complete code walkthrough for an existing
software subsystem. Do not inspect files, call tools, or start authoring. Return
an interactive intake protocol. First, quote the exact single question you would
ask in the first turn; it must be the only question in that quoted turn. Then
list the later questions in the order you would ask them, with an explicit rule
that you ask only one per turn and record its answer before continuing. Finish
with the contract you would freeze before inspecting code.
```

### Supplied material

No files. The frozen prompt and common harness context are the only material
delivered to the model. Any tool call invalidates the repetition.

### Atomic criteria

| ID | Score `1` only when |
|---|---|
| `A01` | The quoted first turn contains exactly one question. |
| `A02` | The protocol explicitly asks one question per turn. |
| `A03` | The protocol records each answer before the next question. |
| `A04` | A later question bounds the repository, package, executable, or subsystem target. |
| `A05` | A later question identifies included manifests. |
| `A06` | A later question identifies included production source. |
| `A07` | A later question separately classifies tests and fixtures as included source, evidence, or excluded. |
| `A08` | A later question separately classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| `A09` | A later question asks whether the corpus may change during authoring. |
| `A10` | A later question identifies the authoritative artifact if the corpus changes. |
| `A11` | A later question elicits audience language proficiency separately. |
| `A12` | A later question elicits audience systems or tooling proficiency separately. |
| `A13` | A later question elicits audience familiarity with the codebase's domain separately. |
| `A14` | A later question elicits explicit depth rather than treating “complete” as self-defining, and the closing contract records the answer. |
| `A15` | A later question elicits output form, and the closing contract records the answer. |
| `A16` | A later question asks what must remain usable if custom tooling disappears. |
| `A17` | A later question elicits prose, terminology, and citation constraints, and the closing contract records the answer. |
| `A18` | A later question elicits navigation and cross-reference constraints, and the closing contract records the answer. |
| `A19` | A later question elicits mechanical proof requirements separately from judgment. |
| `A20` | A later question elicits independent technical review requirements. |
| `A21` | A later question elicits independent editorial review requirements. |

## Case B — external source inventory and fragments

### Frozen fixture

- Original workspace: `/Users/antony/Development/APIAnyware.add-ocaml-target`
- Origin remote: `https://github.com/Linkuistics/APIAnyware.git`
- Original revision: `309646b549d31d318c84ce73f3c9d0a47ab3e3c8`
- Original path: `targets/ocaml/check_floor.ml`
- Original SHA-256: `2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`
- Committed fixture:
  `fixtures/external-check-floor/targets/ocaml/check_floor.ml`
- Accessible run path: `targets/ocaml/check_floor.ml`
- Accessible file count: one

The committed fixture is the portable source of run bytes. Before copying it to
the fresh directory, verify that its digest equals the frozen original digest.
Abort rather than refresh it on mismatch.

### Frozen prompt

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

### Atomic criteria

| ID | Score `1` only when |
|---|---|
| `B01` | **Compliance control:** the inventory contains exactly `targets/ocaml/check_floor.ml` as production source. |
| `B02` | The answer says unsupplied tests and fixtures are unknown or out of scope, not absent from the project. |
| `B03` | The answer says unsupplied docs, models, generated files, examples, and dependencies are unknown or out of scope, not absent. |
| `B04` | Reader order starts with the utility's observable purpose. |
| `B05` | Reader order introduces only vocabulary needed for the first operation. |
| `B06` | Reader order traces one real complete operation at low resolution before detailed parts. |
| `B07` | Later ordering follows causal behavior or invariants rather than file line order alone. |
| `B08` | The fragment ledger has exactly one root for the file. |
| `B09` | Every child fragment has a unique intent-expressive identifier. |
| `B10` | Every child fragment has a concrete source range. |
| `B11` | The root or parent rows state explicit child insertion order or positions. |
| `B12` | The answer states that Markdown definition order does not determine expansion. |
| `B13` | A check rejects duplicate fragment definitions. |
| `B14` | A check rejects unresolved references and cycles. |
| `B15` | A check rejects gaps and overlapping or multiply owned source bytes. |
| `B16` | A check proves every fragment reachable from the one intended root. |
| `B17` | A check compares recursive expansion byte-for-byte with the authoritative file. |
| `B18` | Compilation or copied-snippet presence is not offered as a substitute for byte equality. |
| `B19` | Validation is explicitly read-only and never regenerates production source. |
| `B20` | A changed source byte is expected to produce a localized equality failure. |
| `B21` | Raw Markdown exposes where fragments are inserted and what they insert. |
| `B22` | Removing custom tooling leaves ordinary source and full Markdown readable; only automated proof is lost. |
| `B23` | The worked execution names concrete production inputs and outputs. |
| `B24` | The worked execution names its causal stage boundaries. |
| `B25` | The worked execution states the governing invariant at each non-obvious transition. |
| `B26` | The worked execution states observable intermediate or final results. |
| `B27` | The worked execution explains why each transition exists. |

## Case C — exposition and assurance

### Frozen prompt

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

### Supplied material

No files. The frozen prompt and common harness context are the only material
delivered to the model. Any tool call invalidates the repetition.

### Atomic criteria

| ID | Score `1` only when |
|---|---|
| `C01` | The representative paragraph locally states that `NormalizedKey` is the canonical tenant-qualified identity. |
| `C02` | **Compliance control:** the paragraph connects that meaning to the later write without inventing a function, type, effect, or invariant. |
| `C03` | **Compliance control:** unknown actors, inputs, outputs, and invariants are explicit placeholders or source-verification obligations. |
| `C04` | The plan does not duplicate the authoritative source fragment. |
| `C05` | The plan repeats only the fragment's current semantic consequence. |
| `C06` | Any link label names its destination and purpose rather than “here”, “this”, or “more”. |
| `C07` | The paragraph remains understandable if its optional link is removed. |
| `C08` | The paragraph uses direct declarative prose and no rhetorical question or suspense. |
| `C09` | The plan does not teach ordinary language or storage mechanics granted by the audience. |
| `C10` | Mechanical validation checks the frozen source inventory. |
| `C11` | Mechanical validation checks fragment identity, references, and cycles. |
| `C12` | Mechanical validation checks reachability, exact coverage, and byte equality. |
| `C13` | Mechanical validation checks Markdown headings, anchors, links, and navigation. |
| `C14` | Technical review is assigned to an independent context or person. |
| `C15` | Technical review checks contracts, invariants, control/data flow, effects, and failure/refusal distinctions. |
| `C16` | Technical review checks concurrency and rollback only after source evidence establishes applicability. |
| `C17` | Editorial review is assigned to an independent context or person distinct from technical review. |
| `C18` | Editorial review checks concept order, local completeness, examples, and terminology. |
| `C19` | Editorial review checks both missing local context and redundant repetition. |
| `C20` | A walk-away check removes walkthrough-specific tooling. |
| `C21` | The walk-away result requires ordinary source and full raw Markdown to remain readable. |
| `C22` | The representative paragraph leads with its technical claim. |
| `C23` | The prose rules require stable vocabulary and an explicit actor for every effect. |
| `C24` | The prose rules distinguish modeled refusals, environmental failures, and defects, while requiring source evidence before applying a category. |

## Campaign records

Each case directory preserves one Markdown record per planned repetition with
the execution/access manifests, absolute run directory, exact prompt, raw JSONL
location or digest, exact final answer, normal/truncated termination status,
atomic score table, total successes, and invalid-attempt history and counts by
reason.
The raw stream itself is preserved in a compact machine-readable file.

The final baseline report aggregates per-criterion success counts and applies
the predeclared classifications. It quotes representative omissions and
rationalizations without discarding raw outcomes. It distinguishes repeated
judgment gaps, present-in-sample behavior, mixed evidence, and deterministic
properties better enforced mechanically. It reports per-arm invalid-attempt
counts, truncation counts, enabled-arm discovery counts, blind re-score
disagreements, the primary endpoint, and the regression guard. It never turns an
unobserved preference into skill guidance.

Case B's source is real external production code, and its opening verification
comment primes the mechanical-proof behavior scored by `B13`-`B20`. The prime is
shared by both arms, so it does not invalidate the comparative endpoint, but
those rows cannot support an absolute claim of unguided reliability. Report this
as a stated limitation in both baseline and enabled reports.
