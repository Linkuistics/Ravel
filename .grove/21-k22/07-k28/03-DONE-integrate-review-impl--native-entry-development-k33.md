# native-entry-development-k33

**Integrates:** native-entry-development-k32

## Goal

Triage and integrate native-entry-development-k32's findings before native
qualification builds on the development artifact.

## Context

Read that review through its stable handle, k28's complete contract and
docs/native-entry-development-evidence.md. The review is source evidence,
not runtime reproduction. This leaf and k31 share P≤100/E≤70/RC≤2, D=X=0;
the D≤20 reserve is exclusively phase closure. After k32's conservative RC≤5
charge, cumulative bounds are D≤115/RC≤72/P≤225/E≤60/X=0 under caps
D≤135/RC≤74/P≤325/E≤130/X=0. Charge only new actual work and pass balances on.

## Done when

- Classify every finding from the named review and apply the actionable ones,
  with meaningful failing controls and affected/protected verification through
  the existing Taskfile entry points.
- Preserve k28's full contract, fixed matrix and approved API/EPERM amendments;
  distinguish physical operations, substituted reports and source reasoning.
- Leave the complete qualified campaign and supported-target remainder live
  in k31/k22. Update evidence and brief-chain balances without recharging k30,
  k29 or k32. No parent/root completion follows from this integration.

## Notes

K28's in-session reviewer allowance remains spent. This integration consumes
the named review rather than creating another producer/review campaign.

## Decisions (running log)

- Finding P1 (runner cancellation after its last latch read) is **a real issue**.
  Source confirms `finalise` read `take_interrupt()` before `try_block_and_check`,
  the runner handler writes only its own latch, and a consumed signal is absent
  from the pending set. T8 already specifies the blocked latch snapshot, so no
  contract edit. The review's other limits (no further report-loss finding, not
  exhaustive) are taken as stated; nothing else to triage.
- Fix: attempt the final block, then consume the runner latch, on both the
  confirmed and failed-block paths; precedence unchanged. Two private controls
  (confirmed block and failed block) raise TERM at the acknowledged boundary after
  a real launch leaves the runner handlers installed. Both failed on the unfixed
  order at the signal assertion and pass corrected. No reviewer spawned.
- Verification through `task check`, `task test:native-entry` and
  `task test:terminal-recovery` passed. Evidence is in
  docs/native-entry-development-evidence.md; k31's matrix, E06/E07 joins and
  supported-target remainder stay live.
- Conservative charge RC≤1/P≤10/E≤5, D=X=0, including retirement and commit.
  Cumulative bounds D≤115/RC≤73/P≤235/E≤65/X=0; remaining D≤20 exclusively phase
  closure, RC≤1/P≤90/E≤65/X=0 for k31. k30/k29/k32 are not recharged.
