# rhai-policy-increments-k6

**Reviews:** rhai-policy-increments-k3

## Goal

Read the planned subtree adversarially, and report findings without fixes. The subtree holds every root entry after this leaf, from `rust-2024-toolchain` to `owner-policy-conversion`. Judge whether the cut gives the smallest independently useful increments in dependency order, and whether each one leaves main working.

## Context

- Find the commit of `rhai-policy-increments-k3` by its handle, and read its diff. It adds the whole subtree.
- The root brief states what the workstream must deliver. The running log of `plan-k1` holds the settled requirements. GitHub issue 2 binds through its "Required architecture" and "Acceptance criteria".
- The design is `docs/specs/policy-evaluation.md`, the ADR set and `CONTEXT.md`. Its review is `rhai-policy-k4`, and the integration is `rhai-policy-k5`.
- The code is the Ravel workspace in this checkout. Check a claim about the code against the source.

## Done when

- Each finding names the entry, the defect and the evidence.
- The review checks every requirement scenario of the spec for an owner leaf that tests it at the CLI seam.
- The review checks every acceptance criterion of issue 2 for a leaf that delivers it.

## Notes

These doubts are the producer's own. Read each one against the source.

- Main keeps both policy paths until `typescript-removal`. Check that every earlier leaf can leave the TypeScript path and all its tests green. Check the interim refusal of `load_context` in `rhai-policy-worker`.
- The book changes in two nodes, before and after the removal. Check that the split keeps `task docs:examples` and `task docs:excerpts` passing at every handoff, and that the contract's reader checkpoints still fit.
- Check that no leaf is too big for one session. The candidates are `rhai-policy-worker`, `rhai-llm-anthropic`, `rhai-book-use` (first draft), `typescript-worker-cutover` and `typescript-delivery-removal`. Decide whether any of them needs to be a node now.
- Check the order inside `rhai-local-capabilities` and `rhai-llm-calls`. Look for a leaf that uses something a later leaf creates.
- The release leaves carry three consequences. The upgrade keeps Ravel 1.1.0 and `policy.ts` until the driver restarts. Ravel 1.1.0 refuses new keys in the shared `settings.json`. `task release:major` refuses outside the default workspace. Check that each consequence reaches the leaf that it affects, and that no leaf violates one.
- Check that the leaves hand a verifiable result to the next one, and that each body names its input.

## Decisions (running log)

**Review boundary.** The producer is commit `8372228eefb7`, found by
`rhai-policy-increments-k3`. The working copy was clean at entry. This review
reads the planned subtree against issue 2, the settled requirements and the
integrated design. It changes no implementation or plan, and runs no build,
test, lint or format command.

**Source evidence.** The code graph could not start. `list_projects` refused
because a pre-coordination or unverified generation is active. No graph project,
generation or coverage result is available. This review therefore uses direct
source reads for its bounded claims. It does not infer completeness from the
unavailable graph.

**Scenario ownership.** Every named scenario in the policy-evaluation spec has
an owner leaf at the real CLI seam. The findings concern the safety and
verification of intermediate increments, and completion conditions that weaken
the agreed contract. The coverage tables below distinguish ownership from
executed evidence.

## Findings

### F1 — Keep the book checks green through the removal (P1)

**Entries:** `typescript-worker-cutover-k31`,
`typescript-delivery-removal-k32`, `typescript-removal-k29` and
`rhai-book-inside-k35`.

The removal closes before the remaining executable book examples are migrated.
The delivery leaf explicitly permits failing book checks at line 25, and its
parent permits the same at line 38. This contradicts the root's rule that each
increment leaves main working and fixes a broken book check in the same leaf.

This is a concrete dependency, rather than a possible prose discrepancy.
`docs/book/src/09-jobs-signals-and-the-terminal.md:111` contains a complete
example that writes `policy.ts`, invokes it at line 150, and expects exit 0 and
`finished`. The first book node excludes chapters 7 to 11
(`rhai-book-use-k26`, line 35). The cutover then refuses that command with exit
3. Its repair waits for `rhai-book-inside-k35`, after the entire removal node.
An unused installed worker cannot make the command work once the CLI refuses
the format.

The checker itself has the same dependency. Before running any example,
`scripts/examples.sh:76` invokes `crates/ravel/scripts/dispatch.sh build-id`
and requires `libexec/ravel/ravel-policy`. The delivery leaf deletes both.
`scripts/examples.test.sh:445` also quotes the SDK file that this leaf deletes.
The listed consumer inventory does not name these files, and the permission to
record failures allows the leaf to retire without reconciling them. Even a
check restricted to a newly migrated page cannot pass that freshness check.

The plan needs a handoff in which the executable chapter example and checker
consumers work at the cutover and delivery boundaries. A later prose pipeline
does not establish that handoff.

### F2 — The first LLM call lacks its supervisor memory boundary (P2)

**Entries:** `rhai-llm-anthropic-k16` and `rhai-llm-bounds-k17`.

The first leaf exposes the real HTTP capability and explicitly leaves response
bodies unbounded (`rhai-llm-anthropic-k16:33`). The next leaf supplies the
transport bound. The first leaf therefore cannot independently preserve the
isolation property on which the design relies: an endpoint can make the Ravel
CLI collect an arbitrarily large body before the worker's message or heap
limit can apply. The response is collected in the supervisor, not in the
killable worker.

This is the same defect that design review F2 identified and
`rhai-policy-k5` accepted. The spec requires the bound before Rig decodes a
body (`docs/specs/policy-evaluation.md:614`). The bounds leaf itself records
the unbounded `bytes()` and `text()` collection paths in its input evidence.
The requirement to leave the adapter extensible does not bound its first live
call. Passing the scripted success cases and archive smoke tests would not
establish that property.

The safe first-call boundary needs to include bounded response collection.
Additional hostile cases and accounting can still have distinct owners. No
release is planned between these leaves, which limits the exposure but does
not make the first increment independent of its safety sibling.

### F3 — Synthetic fixtures cannot discharge the real-exchange proof (P2)

**Entry:** `rhai-llm-upgrade-proof-k21:19`.

The leaf permits hand-authored recordings when credentials are unavailable,
then permits completion with an upgrade task that replays those recordings.
The integrated spec requires one recorded **real** exchange for each API type
(`docs/specs/policy-evaluation.md:889`). Synthetic provider documents already
belong to the scripted fake layer. A replay of another invented document cannot
establish that the adapter matches a wire exchange from the service.

The leaf correctly makes new recordings opt-in, but it assigns no remaining
work when all recordings are synthetic. It can therefore retire with the
required replay evidence absent. Lack of credentials must remain an explicit
unmet deliverable, or become an owner-approved change to the contract. A note
that calls a synthetic fixture synthetic does not supply the missing evidence.

### F4 — The plan makes contracted reader checkpoints optional (P2)

**Entries:** `rhai-book-use-k26:36` and `rhai-book-inside-k34:34`.

Both briefs let proof omit the reader session when the owner answers that the
revision needs no read. The current contract requires Read 1 after proof of
Parts I and II, its corrections before dependent drafting, and Read 2 after
proof of Parts III and IV (`docs/book/contract.md:667` and `:683`). It does
not make either checkpoint conditional on the size of a revision.

The first draft does own a contract change with the owner, but its specified
questions cover chapter placement, prerequisites and Rhai teaching. They do
not settle whether these checkpoints remain obligations. Neither brief
requires an agreed waiver to update the contract. A proof can therefore close
its node under this plan while the contract still requires a reader session.
The plan must retain those checkpoints or explicitly carry the contract
decision and its record. Mechanical checks and model proof do not substitute
for the reader's recorded results under that contract.

### F5 — A new test seam conflicts with the agreed CLI seam (P2)

**Entry:** `rhai-llm-structured-output-k20:29`.

The leaf explicitly permits unit tests of the schema validator's edge cases.
The owner settled that every test of this work runs the real `start-session`
in a sandbox home (`plan-k1`, "Test seams"). The integrated spec repeats that
rule and excludes a second in-process seam
(`docs/specs/policy-evaluation.md:1126` and `:1140`). Pure functions are tested
by policies through that same CLI seam.

This note gives the implementer a conflicting instruction about where to prove
the schema subset. The CLI cases in the leaf are otherwise appropriate. They
need the edge cases at the agreed seam too, unless the owner changes that
decision. This finding does not ask to remove existing unit tests of the 1.x
codebase.

## Requirement-scenario ownership

The table covers each named scenario in the current spec. An owner means that
the leaf explicitly assigns CLI tests for the scenario. It does not mean that
the test exists or passed. The references use stable handles because positions
will shift when the integration leaf is inserted.

| Spec scenario | Owner at the CLI seam | Assessment |
|---|---|---|
| Route table | `rhai-policy-worker-k9` | Explicit route and notice assertions. |
| LLM-assisted selection | `rhai-llm-anthropic-k16` | Explicit fake endpoint, selection and no harness-run case. |
| Another purpose | `rhai-llm-anthropic-k16` | Explicit `load_context` summary case. |
| A read in select | `rhai-measured-reads-k12` | Explicit phase, bytes and digest. |
| A FIFO as a source | `rhai-measured-reads-k12` | Explicit nonblocking refusal and blocking-open firing control. |
| Re-entry attempt | `rhai-harness-run-k23` | Explicit early refusal. |
| Completion attempt | `rhai-harness-run-k23` | Explicit separate channel and returned value. |
| Unknown field | `rhai-policy-worker-k9` | Explicit malformed-selection case with location. |
| Wrong role | `rhai-llm-anthropic-k16` | Explicit invalid host-request case. |
| Syntax error in a module | `rhai-measured-reads-k12` | Explicit module source and parse failure. |
| Rate limit and fallback | `rhai-llm-anthropic-k16` | Explicit script-owned second call. |
| Unsupported capability | `rhai-llm-structured-output-k20` | Explicit no request sent and no debit. |
| Oversized successful body | `rhai-llm-bounds-k17` | Explicit 9 MiB metadata without a declared length and early close; F2 concerns the earlier leaf. |
| Large failure body | `rhai-llm-bounds-k17` | Explicit bounded 429 body with retained failure class. |
| Missing program | `rhai-harness-run-k23` | Explicit unavailable result and no debit. |
| A native call that does not yield | `rhai-engine-limits-k10` | Explicit sleep and pad cases with controls. |
| Stalled endpoint | `rhai-llm-bounds-k17` | Explicit whole deadline and connection close. |
| Descendants of a harness run | `rhai-stop-order-k24` | Explicit TERM and absence of parent and child afterwards. |
| A signal after the deadline | `rhai-stop-order-k24` | Explicit deadline, TERM during grace, final signal and original failure. |
| Caught budget breach | `rhai-llm-budgets-k18` | Explicit ninth call within `try`. |
| Memory | `rhai-engine-limits-k10` | Explicit growth case and survival of the CLI. |
| Leak probes | `rhai-llm-anthropic-k16` | Explicit credential-header and reachable-value probes. |
| A flag grant of the credential | `rhai-settings-discovery-k14` | Explicit pre-worker `excluded_grant`. |
| A credential under a base name | `rhai-settings-discovery-k14` | Explicit pre-worker `settings_invalid`. |
| The owner-shaped policy | `rhai-example-policies-k25` | Explicit secondary jj workspace and argument equivalence; `rhai-equivalence-fixtures-k30` preserves the result. |
| Both files | `rhai-policy-worker-k9`, then `typescript-worker-cutover-k31` | Explicit Rhai precedence; the final entry matrix arrives at cutover. |

## Issue 2 acceptance coverage

The contract is the current [issue 2](https://github.com/Linkuistics/Ravel/issues/2),
including its Required architecture section. These are delivery assignments,
not claims of measured conformance.

| Acceptance criterion | Delivering leaves | Assessment |
|---|---|---|
| Deterministic harness, model and effort without an LLM | `rhai-policy-worker-k9` | Assigned, with no endpoint needed. |
| Script-built messages, generic Rig call, script interpretation and selection or refusal without a coding harness | `rhai-llm-anthropic-k16`, `rhai-llm-openai-types-k19`, `rhai-example-policies-k25` | Assigned. The existing refusal CLI test is also ported by `typescript-worker-cutover-k31`. No specialised selection oracle is planned. |
| Bounded neutral discovery and context with provenance | `rhai-measured-reads-k12`, `rhai-data-helpers-k13`, `rhai-settings-discovery-k14` | Assigned. The reader-thread limitation is carried accurately. |
| Bypass of selection, refused re-entry and separate outer completion authority | `rhai-harness-run-k23` | Assigned, including replacement of an enclosing result channel and scrubbing the marker from the final process. |
| Typed conversions, malformed fields, defaults, abstention, refusals and script, provider and process failures | `rhai-policy-worker-k9`, `rhai-measured-reads-k12`, `rhai-settings-discovery-k14`, LLM leaves `k16` to `k20`, `rhai-harness-run-k23`, `typescript-worker-cutover-k31` | Behaviour and CLI cases have owners. Existing selection and context validation tests survive until their planned port. F5 concerns the extra seam authorised for schema edge cases. |
| Deadlines, cancellation, exhaustion, descendants, credentials and logs/results | `rhai-engine-limits-k10`, `rhai-settings-discovery-k14`, `rhai-llm-anthropic-k16`, `rhai-llm-bounds-k17`, `rhai-llm-budgets-k18`, `rhai-harness-run-k23`, `rhai-stop-order-k24` | Final ownership exists. F2 identifies an intermediate exposure. |
| Credential-free deterministic fakes and separate opt-in real-provider and real-harness smoke tasks | LLM leaves `k16` to `k21`, `rhai-harness-run-k23`, `rhai-stop-order-k24` | Assigned. F3 concerns the additional real-recording obligation of the agreed design. |
| Equivalent representative translations and matching installation/migration documents | `rhai-example-policies-k25`, book nodes `k26` and `k34`, removal node `k29`, release/upgrade/conversion leaves `k37` to `k39` | Assigned, including freeze before deletion. F1 breaks intermediate book verification; F4 weakens the reader contract. |

The required architecture is preserved in the proposed final shape. Scripts
own branching, prompts, interpretation, retries and fallback. The adapter owns
generic transport; discovery reports owner configuration. The host owns
credentials, limits, execution and cleanup. LLM calls and harness runs bypass
selection, and harness programs come from settings while scripts supply their
arguments. No caller-specific Grove decision is moved into Ravel.

## Disposition of the producer's named doubts

**TypeScript coexistence.** The temporary refusal of Rhai `load_context` is
limited to the Rhai path, and `rhai-measured-reads-k12` explicitly removes it.
The old path and tests remain until cutover. The final format refusal has the
right owner. F1 is a remaining TypeScript consumer at that boundary.

**Book split and reader checkpoints.** Parts I and II can teach Rhai while the
TypeScript path remains. The split does not cover the executable chapter 9
consumer or checker implementation at removal (F1). Both reader checkpoints
need the contract reconciliation in F4.

**Session size and vertical slices.** The worker and first Anthropic leaf
cross several substantial seams: re-execution, supervision and conversion for
the worker; transport, Rig mapping, target builds and fakes for Anthropic. Each
has an explicit instruction to decompose if the work exceeds one session.
Source inspection does not establish a duration that warrants a separate
finding purely about size. However, the suggested fallback in
`typescript-worker-cutover-k31:14` must not be taken as permission to land
Rust-only removal before dependent test migrations. The existing CLI tests
still execute TypeScript, for example `crates/ravel/tests/cli.rs:130` and
`:167`. The same leaf owns their migration. Any decomposition must preserve
that joint green boundary. F1 and F2 identify concrete seams that already need
attention, independently of session estimates.

The first book draft `rhai-book-use-k27` and delivery removal `k32` are also
broad. Their bodies already name a possible decomposition, so this review does
not require speculative extra nodes solely from file size. Splitting the first
draft by migration chapter and owner-policy chapter can preserve independently
checkable pages. Splitting delivery by build and release machinery must keep
each executable consumer working when its inputs disappear; F1 demonstrates
one such consumer that the current handoff misses.

**Local-capability order.** Reads and imports precede the data helpers and the
wrapper test. Settings/discovery follows the environment capability and owns
credential exclusion before any LLM call exists. Program lookup explicitly
uses the reader thread. No additional prerequisite inversion was found in
these scoped task bodies.

**LLM order.** Plain calls precede the other wires and schema support. The
budget leaf mentions `unsupported_capability` before schema requests are
enabled, but `rhai-llm-structured-output-k20` explicitly owns the executable
no-send/no-debit test when that failure becomes reachable. This is a later
extension, not a prerequisite of the earlier unknown-endpoint budget test.
The response-bound ordering remains F2.

**Release and local conversion.** All three operational constraints reach their
owners. `Taskfile.yml` restricts release to the default colocated workspace,
and `release-ravel-2-0-0-k37` requires the owner's landing decision. That leaf
and `homebrew-upgrade-k38` preserve 1.1.0 with
`HOMEBREW_NO_INSTALL_CLEANUP`. `owner-policy-conversion-k39` preserves
`policy.ts` and the shared settings file. The source supports the latter
constraint: `crates/ravel/src/settings.rs:85` refuses unknown keys. These leaves
use sandbox homes and fake harnesses for local-policy checks. No operational
finding is added.

**Inputs and verifiable handoffs.** The task bodies name the spec sections,
earlier leaf logs or concrete source inputs they consume. The TypeScript
equivalence results freeze before deletion. The two book nodes finish before
publication. F1, F2 and F3 identify handoffs whose present completion conditions
do not establish the promised result.

## Verification limits

This was an inspection-only review of producer commit `8372228eefb7`, its
individual task bodies and brief changes, the current spec, five ADRs,
glossary, requirements log, design review and integration, book contract and
the source paths cited above. Issue 2 was read through `gh issue view`.

No build, test, lint, formatter, archive, real endpoint, harness, release or
personal-configuration operation ran. No subagent was used. Dependency-source
observations cited from the accepted design review and integration are recorded
evidence, not a new audit of those crates. The code graph and its coverage
check were unavailable as recorded above. The integration owns triage, all
plan changes and any resulting verification.
