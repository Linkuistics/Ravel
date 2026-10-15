# native-entry-fault-families-k36

## Goal

Construct the unbuilt fault families on the k34 controller: F02 (6), F04 (14),
F05 (20), F06 (6), the four paired failures and the two queued-TERM variants,
each with its matching broken control.

## Context

Read the k22 brief's frozen F/paired/cancellation table verbatim and the design.
Mutation rows keep both the omitted and the performed+substituted modes; query
rows use the substituted report. Pre-exec controls stay fixed and allocation-free.
Never rewrite a report to model an irreversible physical operation.

Funding (k39): k36–k38 share RC≤10/P≤112/E≤84, D=X=0, with D≤20 reserved
exclusively for phase closure. This leaf's projection, not allocation, is
P≤58/E≤31/RC≤2: 38 owner-site rows (F02, F04, F05 finalisation) and 14
live-child/PTY rows (F05 Started, F06, paired, queued-TERM). Charge actual work
and pass the balance forward; an overrun stops at a precise continuation.

## Done when

- All 46 F rows plus four paired and two cancellation cases run on the controller
  with actual operation versus substituted report recorded separately.
- A failed pending query never becomes empty cancellation; the Started restore
  row uses a live real child/group and PTY lease; original and secondary reports
  survive under cancellation precedence.
- Each claimed repair has a failing broken control; already-correct baselines are
  stated, not invented.

## Notes

If this exceeds one session, decompose at F04/F05/F06.

## Decisions (running log)

- One private fixed-site hook per crate (`native_entry_tests::faults` in ravel,
  `cfg(test)` only): a mutation site either omits the physical operation or
  performs it and then reports EIO; a query site always performs the physical
  query and substitutes only the report. Each row writes one `fault-receipt.json`
  with the operation performed, the actual result/errno, the reported EIO and the
  actual state after the site (handler owners, mask, pending, SIGCHLD flags). The
  one broken subject per row ignores that failure at its site. Runner-side rows
  (F02 managed, F04 runner install, F06) need the keyed-launch role pattern
  because the ravel fixture links keyed-launch in non-test form.
- F05's queued-TERM rows (pending-query, TERM-membership) found a product defect
  the broken control could not have shown: after a failed final pending query the
  incumbent finalisation still restored the mask, so the queued TERM killed the
  launcher under its default action before the original failure or any secondary
  report was written (exit by SIGTERM, empty diagnostics, identical to the
  ignore-fault subject). Repaired: unknown pending state keeps the handled signals
  blocked through settlement and ends on the reported failure. This does not
  claim a cancellation (the signal is unknown); it preserves every report.

- All 46 F rows, the four paired failures and the two queued-TERM variants now
  have development controls with a failing broken subject each (evidence note,
  "Fault families, paired and queued-TERM controls (k36)"): ravel real-main
  fixture and PTY-lease rows for F02 worker, F04 selection and INT final
  restore, F05 and the front-level queued variants; keyed-launch role rows for
  F02 managed, F04 runner install, F06 and the runner-side pairs. All run under
  the shared 30/5/5 controller except the F03 paired row, which uses the older
  worker admission helper; routing that helper and C4's is k37's, per its brief.
  F06's forked-child sites reach the front's fixture through the runner's
  off-by-default `native-faults` feature, enabled only by the isolated fixture
  target. The paired F05 Started + recovery report is asserted at the runner's
  API, where the C4 report seam exists. These are development controls on
  aarch64-apple-darwin, not the immutable campaign, and no family is accepted.
- Charge P≤52/E≤24/RC=0, D=X=0: seams and rows, one product repair, the broken
  controls, two `test:native-entry` and two `check` runs, the R01 identity
  reconciliation (310 present, ignored flags unchanged, 15 new identities here)
  and the mutation control for the repair. No reviewer was spawned; the repair
  is covered by executable controls that fail without it, and the one in-session
  allowance stays spent. Cumulative bounds are D≤115/RC≤77/P≤342/E≤114/X=0 under
  caps D≤135/RC≤87/P≤402/E≤174/X=0; remaining D≤20 exclusively phase closure,
  RC≤10/P≤60/E≤60/X=0, shared by k37, k38 and an optional RC≤5 tree-owned review.
  Projections, not allocations: k37 P≤27/E≤18/RC≤1, k38 P≤27/E≤35/RC≤2 (sum
  P≤54/E≤53 within the balance). Review integration, Rust 1.85/Linux and a
  returning host remain unfunded; no node closes.
