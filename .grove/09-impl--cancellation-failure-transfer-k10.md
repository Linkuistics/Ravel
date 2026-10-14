# cancellation-failure-transfer-k10

## Goal

Correct C1 on the incumbent in one complete runner-to-front slice: consumed
cancellation survives observation/reap errors and reaches no-ending settlement,
with original and secondary diagnostics intact. Leave the working launcher
compatible, with discriminating checks. This is correctness work, not a selected
component target, extraction or comparison restart.

## Context

Read `completion-failure-scope-k7`, `completion-failure-contract-k8` and
`cancellation-failure-transfer-k9` beyond the brief chain. The governing contract
is `docs/specs/result-job-completion.md#required-failure-settlement`; source and
enabling-point rationale are in `docs/completion-failure-design-evidence.md`.
Keep the fixed product/preservation reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0` and its existing expectations.
K9 observed source only, not runtime conformance. Refresh graph generation and
coverage before code discovery; its Tier 2 refresh was generation
`2026-10-14T05:42:04Z` in project
`Users-antony-Development-Ravel.simplify-and-componentize`.

## Working increment and ownership

This leaf owns the complete correction and its tests/documented observations:

- `crates/keyed-launch/src/error.rs`: retain an optional consumed cancellation
  with a typed public read accessor on existing opaque `LaunchError`. Keep
  Display/Debug useful, preserve primary errno, and default unrelated errors
  to no consumed signal. Annotating errors/adding diagnostics must not replace
  an earlier signal. No requeue into the process-global latch.
- `crates/keyed-launch/src/run.rs`: carry the observation through `Failed`, both
  `watch` error exits and `supervise`. At every final wait attempt, success or
  failure, sample/drain the latch before observer/recovery, retaining the first
  consumed signal over that sample. Keep original observation/reap failure and
  actual emergency-reap error text; do not reduce the latter to `.ok()`.
  Preserve group-before-reap signaling and existing recovery ordering. Emit
  Reaped only for a confirmed reap; no invented ending or group absence.
- `crates/ravel/src/run.rs`: capture cancellation from either runner outcome
  before error conversion. Join front signal, outcome signal, then late runner
  signal. Keep original supervision diagnostics when cancellation wins, plus
  every available outer-cleanup/recovery report via existing stderr reporting,
  including JSON mode. Preserve existing codes/stages/statuses without
  cancellation and spawn ENOENT/other-start classifications (127/126).
- `crates/keyed-launch/tests/internal/wait_events.rs` and front private tests:
  exercise these paths using existing Process/FakeProcess and no-ending settle
  seams. A small production-used private conversion/join helper is acceptable
  to make error transfer testable; testing a parallel unused helper is not.
  Keep global-latch tests serialized and leave their latch clean on exit.
- Update the existing runner/error contract documentation and
  `docs/completion-failure-design-evidence.md` with exact checked observations,
  limitations and remaining debt. Adjust obsolete current-source descriptions
  in `docs/specs/result-job-completion.md` and linked component/source guides
  only where C1 changes them. Required behavior remains the independent oracle.

Consumers are the existing launcher CLI and existing runner callers. Success
APIs, packaging, policy/SDK/settings/discovery, command argv and notices remain
compatible. No relatching, public fault flag, owner-policy convention or crate
is selected. The useful stopping state is a working incumbent with C1 corrected;
error-only storage without its consumer is not a separately useful increment.

## Fixed acceptance cases

These expectations are written before implementation; do not derive them from
the implementation's output or soften them to make a check pass.

| Case | Required observation |
|---|---|
| HUP consumed, observation error, emergency reap succeeds | Returned error retains HUP and original observation text; one confirmed Reaped before recovery; no fabricated successful ending. |
| HUP consumed, observation error, emergency reap fails | HUP, distinct original and emergency-wait diagnostics survive; no Reaped; recovery gets unconfirmed status. |
| HUP consumed, exit observed, final reap fails | HUP retained; group-kill precedes wait, wait error retained; no Reaped or group-Gone assertion. |
| Each failed-reap route with TERM arriving during reap | HUP still wins; TERM sampled/drained before recovery. Repeat without earlier HUP: TERM is retained. |
| Successful watch carrying HUP, then reported recovery error | Error retains HUP and the recovery diagnostic. A signal arriving only after the reap remains late, with no rewrite of the completed watch. |
| Front error with no ending, earlier front INT, runner HUP, late TERM | INT wins; supervision and every cleanup report remain diagnostic; no result. Remove INT: HUP wins; remove HUP: late TERM wins. |
| Matching no-cancellation controls | Same original operational failure/classification and diagnostics; absent cancellation accessor; no success invented. Include missing/other executable start errors and an ordinary success control. |

The successful-watch/recovery-error case may use a narrow private report control
at the existing reclaim boundary, with production forwarding its current unit
reclaim as successful. This checks outcome composition only: it neither detects
real terminal syscall failures nor implements C4. Prefer exercising the same
composition function production uses, without a general syscall framework.
Keep successful reap versus failed-reap sampling and late-caller boundary
explicit. Never signal an uncertain/already-reaped group identity anew.

Front assertions must cover the actual production error-conversion/join path,
then no-ending settle, rather than merely passing a prechosen signal to settle.
Assert operational diagnostics when cancellation supersedes a primary error,
and all available cleanup reports; captured policy output remains separate.
Pre-write failures/cancellation produce no result stdout bytes. Preserve the
existing accepted possible prefix on a failed final write; do not turn it into
success. Preserve exact empty,
Unicode and multiline values, first-writer, nesting, published-result/child-error
precedence, descendants, group/reap order, terminal recovery and final CLI write.
Nominal public cancellation alone cannot establish injected error behavior.

## Construction and verification

Allocation is P≤45 minutes (up to 10 for fixtures, 25 for runner/front changes,
10 for documentation/closure) and E≤25 minutes (up to 10 for discriminating
red/green controls, 15 for affected regressions). Aggregate ceilings override
these estimates. No behavior was executed by k9. Use the root Taskfile for
reusable workflows; add a focused runner task only if the existing `test` task
cannot express the needed reusable selection.

1. Preserve reference expectations and record input/source digests before and
   after each measured run. Add the fixed cases first, show the uncorrected
   behavior fails for the intended signal/diagnostic reason (not compilation),
   then show the corrected behavior passes. An additive accessor may be needed
   to compile a control; temporarily remove the transfer while keeping that API
   to establish the red control. Restore the fix and rerun. Keep case-by-case
   command, status, assertion and cleanup records, not matching totals alone.
2. The current `test` task accepts no selection argument. Add and exercise a
   minimal `test:runner` task wrapping locked nextest for `keyed-launch`, with
   optional arguments, and use it for focused private runner tests. Run front
   tests with `task test:session CLI_ARGS='…'`. Verify selectors against listing.
   Check relevant errno/success and existing wait-event/order controls.
3. Run `task test:cli` for real TypeScript policies and fake jobs, then `task
   check` for formatting, lint and workspace tests (its script already runs the
   complete `task test` command, so a duplicate full run is unnecessary).
   Existing runner process/PTY regressions protect actual cleanup and recovery;
   no model service, production disruption or speculative cross-target campaign.
   These are completion checks; at the budget bound, record any not completed
   as owed and preserve a precise continuation. Do not infer a passing workspace
   from focused tests. A Taskfile change must itself be exercised.
4. Keep injected reported outcomes distinct from physical cleanup. FakeProcess
   uses no OS child. Any real fixture has a controller which actually kills as
   needed, waits every child, closes endpoints and bounds reader completion.
   A reported reap failure never excuses an actual orphan in the test harness.

Do not construct all future failure instruments now. Source changes and checked
evidence land together; stop at the first P/E or aggregate bound, name the
precise remainder and seek renewed scope/funding without resetting counters.

## Done when and continuation

C1 acceptance cases discriminate before/after, compatibility checks appropriate
to the touched surface pass, and their evidence/limits are recorded. No failure
diagnostic or consumed cancellation is discarded by the corrected transfer.
Follow Grove's review allowance; if review becomes tree-sized, externalize it.

Before retirement, grow only the next precise supported continuation ahead of
`book-structure-k5`, normally planning the worker-reap admission correction C2.
If C1 leaves material uncertainty, that precise correction/check comes first.
Do not enter the book agreement while known consequential failure work remains.
C2 worker admission, C3 partial acquisition/cleanup, C4 failed-exec/checked
terminal recovery, and native entry-transition checks remain mandatory.
C4's exclusive-lease/identity premises remain conditional; Started is
spawn-reported success, not confirmed exec. No persistent-host restoration is
established by the one-session launcher lifetime.

## Budget and preservation history

Correctness caps: D≤90 minutes including 20 reserved for phase closure, RC≤30,
P≤240, E≤120, X=0, zero production disruption. K8 charged D≤40/RC≤10; k9
conservatively charged D≤10/RC≤5 including closure/commit. Total charged bounds
are D≤50/RC≤15/P=E=X=0; remaining D≤40 (20 reserved), RC≤15/P≤240/E≤120/X=0.
This leaf may spend the prospective P≤45/E≤25 earmark, not fresh D/RC funding;
any review spends within remaining RC and must be recorded. Full earmark use
leaves P≤195/E≤95. Record actual conservative charged bounds and remaining caps
at closure; an allocation is not spent merely because it exists.

K4 stays closed: one iteration, seven attempts, R=1/8, F=4/4, N=3/6,
D≤45/RC≤15/P=E=X=0. K6 separately charged D≤45/RC≤10/P=E=X=0. No refunds,
new comparison iterations/attempts, variants or catalogue/routing reads. Preserve
G2 six-month upkeep/support and route cost, G3 native delivery/use beyond entry,
G4 unread alternatives, G5 unexecuted runtime domains and R-W compatible-runtime/
new-lifecycle debt. No simplification gain, supported target/migration or actual
independent-capability use follows. Root capability delivery, unchanged config
and complete sessions, human book structure, and practical post-proof reader
checkpoint remain required.
