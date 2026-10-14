# component-contracts-k3

## Goal

Establish Ravel's current responsibility contracts and the evidence needed for
the first component-boundary decision. Produce a useful explanation of the
existing composition and a bounded redesign question; this increment precedes
choosing a target architecture, extracting code or changing packaging.

## Context

- The root brief's Current upstream guidance records the adopted practices,
  pinned source snapshots and unfinished evaluation. Read the pinned
  decomposition skill and its existing-system and views references. Use the
  campaign/records references when framing a subsequent comparison.
- Current behaviour is described by `docs/specs/session-execution.md`,
  `docs/session-cli.md`, `docs/owner-configuration.md` and
  `docs/local-runner.md`. The brief's three existing ADRs explain current
  obligations; investigate implementation choices instead of assuming them.
- `plan-k1` records why the human questions TypeScript and the two Rust crates,
  and the unchanged configuration requirement. Its initial map is orientation.
- The existing root `Taskfile.yml` owns repeatable verification. Use existing
  checks where an observation is necessary; do not rerun upstream evaluations.

## Done when

- A source-grounded explanation identifies logical responsibilities,
  guarantees, caller assumptions, hidden decisions, state/effect owners,
  dependencies and normal/failing composition. It distinguishes logical
  components, packages, languages and runtime participants. The reader can
  explain what TypeScript does and what each Rust crate is responsible for.
- A scoped behavioural obligation map covers unchanged owner configuration,
  policy decisions/refusals, launch/result handling and session authority,
  cancellation, reaping and terminal recovery. It links these promises to
  owners and existing acceptance seams, with observations and unknowns marked.
  Document scope rather than claiming complete legacy discovery.
- At least one successful operation and one failing/cancelled operation are
  traced through the actual composition. A concrete independent-change
  scenario shows what must currently be understood or changed together. Use
  graph discovery and exact source verification with coverage fallback; static
  dependencies alone establish no runtime result.
- Candidate concerns about language/runtime boundaries, generic capability
  reuse, caller obligations, state/effects and composition costs have explicit
  evidence or are marked as questions. Identify the first concern to compare
  by its consequence for the human's comprehensibility goal, rather than its
  file size or presumed crate destination. A well-supported no-change finding
  for a questioned boundary remains a valid result.
- Record the resulting contracts and supported explanation in normal durable
  repository documentation, reconciling existing documentation instead of
  duplicating it. Put process bounds and the next question in the continuation
  brief/task; create ADRs only for settled decisions that earn them.
- Cut the precise continuation before retiring. If evidence supports a
  comparative design step, its body identifies the selected concern, fixed
  reference and scenarios, preservation definition, improvement rows, numeric
  bounds and checks still owed. If discovery leaves a consequential unknown,
  cut the bounded observation that would settle it instead. If the design is
  adequate, state its retained costs and continue with the still-required
  independent-use/documentation work. Do not leave the root's unmet goals with
  no live continuation or generate an implementation/editorial tree here.

## Notes

Keep this to one focused discovery/design-frame session. Before discovery,
declare a limit of 90 minutes of active investigation and at most three scoped
operation/change traces; choose the traces from the agreed acceptance seams.
This is a planning allowance, not a measurement of elapsed launch time. Widen
only for an observed dependency needed by the selected decision, spending the
same allowance. At the bound, preserve useful evidence and hand the precise
remaining question forward; do not invent missing contracts or extend the
allowance silently. No alternative generation or catalogue-entry qualification
is required in this increment; a later comparative campaign sets its own
source-method bounds before generating options.

Existing TypeScript policies, SDK use, settings, discovery and configuration
behaviour must work unchanged. The CLI and complete-session contract remain
required. Rust-only implementation, crate merging, further splitting and
extraction remain hypotheses. Independent use requires a consumer-facing
contract and an example of a real use outside Ravel-specific conventions;
similar-looking code or a new package name is insufficient.

The eventual design must assess its target and working migration states
separately and compare regrouping as well as splitting. Leave code unchanged
in this increment. Keep the later human documentation-structure agreement and
practical reader checkpoint in the continuation; upstream unfinished evaluation
is a limit to report, not a reason to wait or a result to infer.
