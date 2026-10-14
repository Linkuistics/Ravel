# worker-reap-admission-k11

## Goal

Plan the next complete incumbent correctness slice, C2: confirmed policy-worker
reap is a prerequisite to successful evaluation, resolution, selection and
managed launch. Preserve original failure classifications/cancellation and all
teardown/cleanup diagnostics. Deliver an executable-ready repair mandate with
prospective P/E pricing, without implementation or behavioral execution here.

## Context

Read `completion-failure-scope-k7`, `completion-failure-contract-k8` and
`cancellation-failure-transfer-k10` beyond the brief chain. The independent
oracle is `docs/specs/result-job-completion.md#worker-teardown-before-admission`;
C2 source/enabling-point rationale is in `docs/completion-failure-design-evidence.md`.
C1 checked receipts are there too: cancellation survives runner errors and
no-ending settlement. Its private recovery-report seam does not detect actual
terminal recovery failure. Keep fixed preservation reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0` and protected existing expectations.
Refresh the nearest graph project/generation and coverage before discovery.

Candidate ownership is `crates/ravel/src/worker.rs` evaluate/stop and their
private tests, with existing choice/front consumers and CLI/process fixtures.
Use exact graph/source evidence to settle the smallest enabling point. K8 found
that evaluation stores the worker stop/wait outcome but admits successful
conversation without checking it; choice resolves only after evaluation success.
K7 authorizes a private report substitution after actual physical teardown and
before admission. No public testing flag or general OS-wait abstraction is funded.

## Done when

- Establish the complete success gate: conversation success plus confirmed wait
  may admit evaluation; a reported wait failure becomes worker failure 5 and
  prevents resolution, Selected and every managed spawn. Any confirmed worker
  exit status satisfies reap, including nonzero; no false alive/gone inference.
- Retain refusal, timeout, protocol/output-bound and cancellation classifications
  when teardown also fails. Cancellation wins while original failure and every
  available teardown/directory cleanup report remain diagnostic. Keep captured
  policy stdout/stderr separate from operational reports. Do not claim “reaped”
  on unconfirmed wait. Worker identity, failed operation and recovery advice
  survive. Confirm physical endpoint-close/stop/kill/wait/capture/cleanup order.
- Specify the smallest private report control at evaluator teardown before the
  success gate. The fixture actually stops/waits its worker before substituting
  the reported failure, keeps the real wait receipt, drains bounded diagnostics,
  closes endpoints and removes its fixture directories. The front/choice path
  must prove no resolution/managed-start sentinel after unconfirmed reported
  reap. Do not construct all future C3/C4 instruments in this slice.
- Write fixed expected cases before the subsequent implementation: successful
  conversation + failed wait; policy refusal + failed wait; deadline + failed
  wait; protocol/output-bound failure + failed wait; cancellation + failed wait;
  confirmed nonzero worker exit; no-fault successful selection/complete session.
  Include teardown cleanup failure when available, no result stdout on failure,
  original diagnostics and no invented successful selection/reap. Identify any
  new consequential uncertainty before allocating construction.
- Price and grow one complete evaluator-to-admission implementation/check leaf
  before `book-structure-k5` from remaining P/E. Its artifact and actual consumer
  form one useful increment; a report-only helper with no admission gate is not
  useful on its own. Use root Taskfile selection, verify selectors by listing,
  demonstrate red for the intended admission/diagnostic reason then green, and
  record immutable input/source digests plus per-case receipts. Preserve real
  policy/fake-job, runner/process/PTY and C1 regressions. Required workspace
  checks remain explicit completion criteria rather than inferred success.

## Notes

This planning leaf is D≤10/RC≤5 minutes, one session, P=E=X=0; any review follows
Grove's picked-leaf allowance. It starts from cumulative correctness charges
D≤50/RC≤20/P≤30/E≤5/X=0 and remaining D≤40 (20 reserved for phase closure),
RC≤10/P≤210/E≤115/X=0. K10's own conservative charge includes retirement/commit.
A new leaf resets no cap. Record actual conservative bounds and remaining caps;
stop at the first leaf/aggregate bound with a precise supported remainder and
seek renewed funding rather than silently expand. Implementation reservations
must be named before constructing or executing a new control.

C3 checked partial acquisition/removal, C4 failed-exec/checked terminal recovery
and native entry transitions remain mandatory before the book agreement or new
component selection. C4's exclusive-lease/identity premises remain conditional;
Started is spawn-reported success, not confirmed exec. A one-session CLI is not
proof of arbitrary continuing-host restoration; the persistent runner is separate.
After C2, grow only the next precise supported continuation, or a narrower C2
remainder if checks expose material uncertainty, before the book agreement.

K4 stays closed: one iteration, seven attempts, R=1/8, F=4/4, N=3/6,
D≤45/RC≤15/P=E=X=0. K6 separately charged D≤45/RC≤10/P=E=X=0. No comparison
restart, new variant/catalogue/routing read, selected crate/package/target,
migration, independent-use result or simplification gain is authorized here.
Preserve G2 six-month upkeep/support and route cost, G3 delivery/use beyond
entry, G4 unread alternatives, G5 unexecuted runtime domains and R-W compatible
runtime/new-lifecycle debt. Root capability delivery, unchanged owner config and
complete sessions, human book structure and the practical post-proof reader
checkpoint remain required.

## Decisions (running log)

- The complete useful increment is evaluator teardown through choice/front
  admission, with its private controls and evidence in one implementation leaf.
  No helper-only or module-by-module stopping state is useful. Keep existing
  stop/kill/wait and bounded capture order; require confirmed wait and checked
  post-evaluation private-directory removal before success. Partial preparation
  cleanup remains C3; native waitability and actual terminal recovery remain owed.
- Tier 2 graph/source refresh at coverage generation `2026-10-14T06:08:15Z`
  confirms `evaluate` ignores wait on success, `evaluate_and_resolve` resolves
  only after evaluation, and `run` settles selection failure before launch.
  Exact source also shows `choice::overruled` replaces the original refusal on
  late cancellation. Therefore the C2 slice includes narrow preservation there,
  not only the evaluator gate: original classification/source/bound/remedy and
  all operational reports must survive late cancellation without contaminating
  captured policy stdout/stderr. No general reporting/OS-wait redesign is needed.
- K12 is prospectively allocated P≤50/E≤25/RC≤5 minutes, D=X=0, including
  repair/control construction, fixed red/green checks, regressions, evidence,
  coordination and retirement/commit. This reserves, rather than spends, P/E.
  No new reviewer is commissioned for this planning leaf; RC=0 here. The current
  D≤10 charge includes source/planning, tree verification and closure/commit.
- Closure retains the incumbent contract and existing ADR choices; the local
  reversible repair mandate does not earn a new ADR. No consequential new
  architectural question or fresh review was needed. The root and k12 carry
  conservative charged D≤60/RC≤20/P≤30/E≤5/X=0 and remaining D≤30 (20
  reserved)/RC≤10/P≤210/E≤115/X=0. K12's prospective P≤50/E≤25/RC≤5 is not
  spend; all remaining C3/C4/native and root/G2–G5/R-W obligations stay live.

## Verification and stopping state

`resolve` and `brief-chain` confirm k12 before the book and the unchanged root
brief chain. The book's SHA-256 matches its parent-commit body through renumbering:
`1ec62bb523ab56e47af33a7a3030e0605bc37924a6ecbfff88338f898b40d28a`.
`task --list-all` parsed the existing task entry points; no behavioral selector
was run. The fresh bounded `jj diff --summary crates Taskfile.yml` is empty;
the same diff instrument over `.grove` shows the intended task additions,
log/root changes and book rename. Full task-tree diff was read. Direct validation
passed task headers/sections/whitespace, book bytes/order, unchanged product/
Taskfile and shared budget account. Unsupported `jj diff --check` was replaced
with direct whitespace validation, not credited as a passing check.

Source discovery used Tier 2 in the nearest graph project with exact snippets,
completed evaluate both-direction pagination and direct choice/front/report/
capture source reads. Coverage checked every relied-on source/task/spec/ADR/
Taskfile/glossary path at generation `2026-10-14T06:08:15Z` with matching metadata
and no recorded gaps. The changed/new task artifacts and renamed book also had
matching metadata/no recorded gaps at generation `2026-10-14T06:10:38Z`, recorded
06:13:08 UTC. This is a best-effort signal, not completeness proof. These source
observations do not establish runtime failures or successful behavior.

The useful stopping state is one precisely funded implementation/check mandate
with fixed expectations and private-control/consumer boundaries. This session
constructed no executable instrument, changed no product code and ran no
behavioral checks (P=E=X=0). No live review declares k11 as its producer, so
no provider receipt is written. K12 and the book remain live; root acceptance
is incomplete, so no parent close or finish teardown is earned. D≤10/RC=0
includes planning, verification, retirement and commit.
