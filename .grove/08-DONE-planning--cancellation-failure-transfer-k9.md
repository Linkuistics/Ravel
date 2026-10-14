# cancellation-failure-transfer-k9


## Goal

Plan the next narrow, independently useful incumbent correction: retain a
runner-consumed cancellation across observation/reap failure and carry it into
the front's no-ending settlement with original/recovery diagnostics intact.
Deliver one executable-ready C1 vertical slice and price its construction/checks
within the remaining correctness-phase caps. This is planning, not repair or a
reopened component comparison.



## Context

- `completion-failure-contract-k8` owns the new design and reconciliation.
  `docs/specs/result-job-completion.md#required-failure-settlement` binds the
  required outcome; `docs/completion-failure-design-evidence.md` gives exact
  source/enabling points, alternative costs and discriminating checks.
  `completion-failure-scope-k7` settled consumer lifetime, scope and seams.
  Do not re-interview that agreement or reconstruct a new architecture.
- C1's consumers are the Ravel front and existing runner callers. The runner
  owns a signal it consumed; the front owns channel/launch association, outer
  cleanup and its final write. Candidate ownership is the existing opaque
  `LaunchError` plus internal `Failed`/`watch`/`supervise`, then front `run` and
  `settle`. Success APIs, spawn errno and CLI classification remain compatible.
  No relatching, fabricated `Ended`, public test switch or new crate is selected.
- Existing acceptance enabling points are `Process`/`FakeProcess` in
  `crates/keyed-launch/tests/internal/wait_events.rs` and front `settle` with
  an absent ending; public CLI tests use real TypeScript policy and fake jobs.
  Keep latch tests serialized. Fault injection changes reported outcomes only;
  any real fixture child/group is actually waited for by its controller.
- Fixed product/preservation reference remains
  `0bdcb1f9ae24db5e9dd499308dd791744e908eb0`. K8 changed no product source and
  built/ran no behavioral instrument. Its Tier 2 project is
  `Users-antony-Development-Ravel.simplify-and-componentize`, initially
  generation `2026-10-14T05:10:00Z`; source/trace/coverage provenance is in the
  evidence document. Refresh graph generation/coverage before code discovery.
- K4 remains closed: one iteration, seven attempts, R=1/8, F=4/4, N=3/6,
  charged D≤45/RC≤15 minutes, P=E=X=0. K6 separately charged D≤45/RC≤10,
  P=E=X=0. These do not refund or reset; G2–G5 and R-W debt remain below.

## Done when

- Establish the smallest C1 change set: retained consumed cancellation in
  error outcomes, first-signal precedence on both error exits and the
  success-then-recovery-error boundary, and front joining before error data is
  discarded. Keep error errno, original observation/reap failure and all obtained
  emergency-reap/recovery diagnostics. Do not expand into C2/C3/C4 repair.
- Name a vertical implementation slice with responsibility/file ownership,
  consumers, compatibility and verification. Its useful stopping state is
  a working incumbent with C1 corrected and discriminating evidence, while
  worker reap admission, partial-directory reports, failed-exec/checked terminal
  recovery and unchecked entry-transition premises remain explicitly owed.
- Keep the expected outcome outside the implementation under judgment. Cover
  interruption then observation error with successful/failed emergency reap;
  interruption then normal-exit observation/reap error; a second signal at
  failed reap; front no-ending settlement with earlier front signal and late
  runner signal; and no-cancellation controls. Reaped fires only on confirmation,
  no ending/group absence is invented, all cleanup reports survive and no bytes
  reach result stdout on failure/cancellation. Existing group-before-reap order,
  exact bytes/first-writer/nesting, configuration, terminal recovery and final
  CLI write expectations remain protected. Nominal CLI cancellation does not
  substitute for private injected wait-error cases.
- Write the later implementation mandate before growing it, with a concrete
  P≤45/E≤25-minute allocation or a smaller justified one. It must name relevant
  Taskfile checks (`test:session`, `test:cli`, workspace/runner checks as needed),
  before/after failing controls, per-case logs and real fixture cleanup. Keep
  checks appropriate to the touched surface. No behavior is executed in planning.
- Grow only that next supported implementation slice before `book-structure-k5`.
  Carry aggregate spend/remaining budgets and future C2/C3/C4/entry work forward;
  do not prematurely reach the book agreement while consequential failure
  work remains. Stop at a cap with a precise funded remainder rather than reset.
- Preserve G2 six-month upkeep/support and route cost, G3 native delivery/use
  beyond entry, G4 unread alternatives, G5 unexecuted runtime domains and R-W
  compatible-runtime/new-lifecycle debt. No simplification, target/migration or
  independent-capability verdict follows. Root capability delivery, unchanged
  configuration/complete sessions, human book structure and the practical
  post-proof reader checkpoint remain required.

## Notes

The correctness phase has caps D≤90 (including 20-minute closure reserve),
RC≤30, P≤240, E≤120 minutes, X=0 and zero production disruption. K8 conservatively
charges D≤40 and RC≤10, P=E=X=0, including its closure/commit. Therefore this
leaf starts with remaining D≤50 (20 still reserved for phase closure), RC≤20,
P≤240/E≤120, X=0. Its own prospective allocation is D≤10/RC≤5, P=E=X=0,
one session; any fresh reviewer is subject to Grove's picked-leaf allowance.

P≤45/E≤25 are earmarked for the future C1 implementation/check slice, **not
charged or to be spent by this planning leaf**. They leave P≤195/E≤95 available
for later gaps if consumed in full; unused reservations are not fictional
implementation spend. After this planning ceiling, remaining D would be≤40
and RC≤15, with the phase closure reserve retained. Record actual conservative
charged bounds and pass totals onward. At the first cap with unfinished work,
preserve a coherent handoff and seek renewed human scope/funding.

New comparative iterations/attempts, variants and catalogue/routing reads are
zero. Keep the incumbent. The one-session CLI contract excludes a returning
host's arbitrary restoration and concurrent embedding. C4's proposed
checked-holder mechanism requires explicit exclusive-lease/identity premises;
CLI lifetime alone cannot establish them. Started is spawn-reported success,
not confirmed exec. Those distinctions and checks survive this C1 slice.

## Decisions (running log)

- Keep C1 as one implementation leaf before the book agreement: error-carried
  cancellation, runner failure transfer and front settlement are one working
  increment. Landing an unused error accessor separately would not correct the
  consumer. No node, package, comparison or C2/C3/C4 implementation is needed.
- Tier 2 refresh found generation `2026-10-14T05:42:04Z`. The exact-name search
  returned all 15 matches without another page. Both-direction depth-one traces
  of runner `supervise` and front `settle`, including tests, were untruncated;
  their exact snippets and runner `watch` were read. Coverage of the four source
  paths, contract/evidence, Taskfile and bootstrap documents reported matching
  metadata and no recorded issue, a best-effort signal only. Direct reads of
  the error type, front conversion/settlement and private fixture confirm the
  planned enabling points. No behavioral instrument was built or run.
- Runner error exits must sample at their final wait attempt before observers
  or recovery, retaining the first consumed signal and draining the later
  sample. The emergency-reap error text must survive alongside the observation
  error; today's `.ok()` loses it. Front must capture the outcome's signal
  before converting its error and report that original operational diagnostic
  when cancellation supersedes it. No `Ended` or group fact is manufactured.
- Recovery-error composition is part of C1's information contract, while actual
  checked terminal recovery remains C4. A narrow private outcome/report control
  at existing supervision recovery may exercise successful watch followed by
  a reported recovery error; production's unchecked reclaim remains explicitly
  unproved. No new public switch or full syscall abstraction is funded.
- Price the complete repair at P≤45/E≤25 minutes, allocated prospectively only.
  Preserve the external case expectations in its mandate before growth. On
  completion, cut the next precise failure-work continuation before the book;
  C1 cannot discharge the other gaps or phase closure. At a bound preserve the
  exact remainder and obtain renewed human funding without resetting totals.
- The bounded planning critique confirmed that error storage and front transfer
  must land together, and that a reported recovery-error control cannot prove
  checked terminal recovery. It also reconciled the Taskfile: `test` has no
  selection parameter, so k10 adds/exercises a focused runner task; `check`
  already runs the locked full-workspace nextest command. Required checks are
  completion criteria, not optional successes inferred at the budget bound.
- Closure conservatively charges this leaf D≤10/RC≤5 minutes including tree
  verification/retirement/commit, P=E=X=0; no fresh reviewer was materialised.
  Correctness total charged bounds are D≤50/RC≤15/P=E=X=0. Remaining ceilings
  are D≤40 (20 reserved for phase closure), RC≤15/P≤240/E≤120/X=0. The P≤45/E≤25
  reservation is handed to k10 without spending it here. Its complete mandate
  was written before insertion and the root carries the unresolved repairs and
  next-continuation obligation. No ADR or glossary meaning changed.
