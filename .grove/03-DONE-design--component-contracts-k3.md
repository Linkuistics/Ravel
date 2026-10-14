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

## Decisions (running log)

Investigation is bounded to 90 minutes of active investigation and three scoped
traces: a successful CLI/real-policy/fake-program result round trip; cancellation
of that composition with group cleanup; and an independent change to session
completion semantics, following the result-channel and runner obligations.
Ancillary source checks establish the scoped obligation map, not additional
operation traces. This leaf selects no target architecture and changes no code.
The clean starting reference is parent commit
`0bdcb1f9` (`upstream-guidance-k2`); exact source evidence will use its full identity.

The current explanation separates logical responsibilities from packaging:
TypeScript executes unchanged owner policies; the native front retains authority
and judges protocol data; `keyed-launch` supplies job mechanics within that same
front process. No target-language or package decision follows from this finding.
Durable explanation belongs in `docs/components.md`, linked from the existing
guides; no new agreement-point spec or ADR is warranted by this observation.

`task setup` and `task build` succeeded. The focused `task test:session` filter
ran three cases: exact routed results, cancellation in selection/managed execution,
and cancellation after publication. All passed (74 cases outside the filter).
The source/manifests/scripts/configuration and front/worker subject manifest had
identical before/after SHA-256 identity
`c5be21debad205bd914bfaf71de19a086f79a542c757827620732df25f8d8d8d`.
These observations support those cases on this host; they are not a full legacy,
terminal, hostile-runtime or cross-platform verification.

The human requests a book (user guide) about Ravel and its components, for a
software developer with no background in this context. The root documentation
goal now names that deliverable and audience. Domain teaching precedes internals;
Rust/TypeScript familiarity and the actual viewing medium remain questions for
the dedicated human book-structure agreement, not assumptions of this leaf.

One fresh-context completion-contract reviewer found three source-backed
documentation/scenario inaccuracies: definitive appearance observation order,
mixing lower-layer fixture completion with CLI signal propagation, and omitted
combined-error precedence. Each is valid and actionable, not a product defect.
Corrections use the exact unchanged source and existing acceptance/unit seams;
they change no target design. No second reviewer is materialised.

The two existing settling unit cases for combined cancellation/result/cleanup
failure and cancellation/group diagnostics passed after selecting the executable
target correctly. The first library-only filter selected no cases and supplies
no evidence. Production source and expectations remain unchanged.

The first comparison is the result-bearing job's consumer protocol, because it
crosses the required full-session path and constrains reuse of a capability
inside `ravel`. Its explicit continuation is `result-job-boundary-k4`; the
human book/structure agreement is `book-structure-k5`. Neither selects a package,
runtime replacement or implementation/editorial tree. The root remains live.

The obligation/source mapping and corrected diagram order were replayed against
the named unchanged functions. Browser checks confirmed all three rendered views,
a fresh composition deep link, and desktop/narrow-width navigation; narrow
diagrams use readable horizontal panning. Dark appearance remains uninspected.
The scoped mechanical check resolved 92 local links in nine documents and
validated diagram manifest sources/IDs. These checks support source fidelity
and delivery, not human comprehension. No new spec/ADR or tree review is needed
for this current-contract explanation and bounded question; candidate judgment
and the human book/reader agreements remain live continuations.
