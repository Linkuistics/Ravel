# native-entry-barrier-controller-k34

## Goal

Build the one owner-local, qualified acknowledged-barrier controller that every
remaining native family runs on, validate it with its three failing controls, and
use it to complete E06 (15 boundary joins) and E07 (4 first-consumption cases)
with their individual broken joins. This is k31's first working increment; the
whole k28 acceptance contract is unchanged and stays live in the nodes above.

## Context

Read the k31 brief (its Done when and funding balance), the k28 and k22 briefs,
`docs/native-entry-transition-design.md` (T0–T8, the E06/E07 rows, the
controller envelope), `docs/native-entry-development-evidence.md` and
`docs/native-entry-cases.json`. The existing fixture in
`crates/ravel/src/native_entry_tests.rs` has an ad hoc polling controller; this
leaf replaces it with the barrier protocol rather than adding a second one.
Charges come from the k31 balance D≤20 closure only/RC≤1/P≤90/E≤65/X=0; charge
only this leaf's actual new work.

## Done when

- A private fixed-enum boundary barrier exists (cfg(test) only, no installed
  switch): the fixture writes a ready record with owner/mask/pending/consumed
  facts over a private pipe, the controller delivers its signal or fixed fault,
  the fixture acknowledges consumption, the controller releases. Every wait has
  the 5-second acknowledgement bound inside the 30-second outer case and a
  reserved final 5 seconds of cleanup. No sleep orders anything.
- The three controller controls are observed to fail where they should: watchdog
  expiry, blocked acknowledgement with failed release, and an escaped pipe-holder
  killed and waited by its owner. A timeout or unresolved fixture is a failed
  receipt; no reaped or released identity is ever signalled.
- E06's 15 rows (five boundaries × INT/TERM/HUP) run with the actual signal at
  the named boundary, recording handler owner, masks, pending/consumed facts,
  child/reap/resource state and zero result, each with its corresponding join
  removed (or first latch overwritten) and observed to fail.
- E07's four rows acknowledge the first consumption before the second delivery
  and survive outcome/error and late latch; the store-overwrite mutant fails.
- Existing development identities and every prior expectation still pass through
  `task test:native-entry`, `task check` and `task test:terminal-recovery`.
  Record decisions at the moment they settle; charge only actual new effort.

## Notes

Not covered here, and still owed: reader masks (E08, k35); the E04 managed
ignore-only mutant and the PTY/INT handoff (E05, k35); all F, paired and
cancellation rows (k36); S completions (k37); frozen, digested, immutable
receipts and R01 per-task runs (k38). Routing the older worker/terminal fixture
helpers through the controller belongs with the family that uses them.

Development-time green runs are not the immutable campaign; that is k38. Host-only
(aarch64-apple-darwin) observations qualify only that host.

## Decisions (running log)

- Decompose k31 at its family seams: this controller leaf, handoff/reader
  controls (k35), fault families (k36), settlement controls (k37) and the
  campaign/receipts/support leaf (k38). The whole k28 contract and the unchanged
  matrix stay live in the k31 and k28 nodes; no child is accepted separately.
- One controller, one file: `crates/keyed-launch/src/native_barrier.rs`,
  `cfg(test)` only, included into the ravel test binary by `#[path]`. keyed-launch's
  own `cfg(test)` seams are invisible to ravel's separately built fixture, so
  E06 (front joins) runs at the front and E07 (runner latch) at the runner, each
  under this controller. FIFOs rather than inherited descriptors, so no managed
  child or worker can hold a barrier end.
- The ready record is bounded by the 25-second work deadline, not 5 seconds: it
  covers the fixture's own preceding work. A probe and a release are each bounded
  by 5 seconds and the controller owns the final 5. The design's wording does not
  distinguish them; record this reading.
- E06 boundary mapping, chosen and recorded rather than discovered: the
  *after confirmed reap* boundary is the observer's `Reaped` event; the
  *no-ending* row reaches the spawn failure with a missing interpreter (ENOENT at
  exec, resolved at selection); *selection failure* is the owner policy's own
  `refused` status. Both finalise rows place the barrier after cleanup and
  immediately before `Handlers::finalise`.
- Instrument defect: the E07 handler counter first advanced before the latch
  write, so an acknowledged probe could read an empty latch. Corrected to count
  after the latch and each break now asserts that the *later* signal won, so a
  race cannot pass as a broken control.
- No product defect was found by these controls; every repaired row conformed.
  Production semantics are unchanged: the only non-`cfg(test)` source change is
  that the start check and admission result are bound to a local before use.
- Verification: `task test:native-entry` (25 + 4 + 2 cases), `task check` (318
  run/33 skipped, formatting, clippy, worker, PTY recovery) and
  `task test:terminal-recovery` passed after the final edit. R01 reconciled by
  identity: all 310 prior identities present with unchanged ignored flags; 41 new
  identities. E06 passed on five further unchanged repeats (flake detection only).
- Charge only new work: P≤30/E≤15/RC≤1, D=X=0, including decomposition, evidence
  and closure. Cumulative bounds D≤115/RC≤74/P≤265/E≤80/X=0 under caps
  D≤135/RC≤74/P≤325/E≤130/X=0; remaining D≤20 exclusively phase closure,
  RC=0/P≤60/E≤50/X=0 for k35–k38. k30/k29/k32/k33 are not recharged.
