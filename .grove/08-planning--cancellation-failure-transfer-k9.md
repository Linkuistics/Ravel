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
