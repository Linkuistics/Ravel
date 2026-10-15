# native-entry-settlement-controls-k37

## Goal

Finish the S01–S05 matching broken controls and remaining rows: S02's bound and
identity claims, F03/S03 report/deadline controls, S04's seven recovery rows and
both S05 first-HUP-then-TERM rows, preserving counts 8/7/8/7/2.

## Context

Read the design's S table and the evidence note's S sections. Runs on the k34
controller. ECHILD/reap/release disables numeric signalling; substituted reports
cannot restore released ownership or invent lifecycle facts.

Funding (k39): this leaf's projection, from the shared k36–k38 balance, is
P≤27/E≤18/RC≤1, including routing the older worker/terminal helpers through the
controller. As the last instrument producer, cut one tree-owned review of the
new instruments before k38's freeze only if you judge an adversarial read
necessary; RC≤5 is reserved for it, and integrating its findings is unfunded.

## Done when

- Every S row has its failing broken control; deadlines are absolute and EINTR
  never restarts them; foreign foreground is preserved and unknown status reports
  uncertain foreground/modes.
- The 32 S identities are unchanged and individually reconciled.

## Notes

Development green is not the immutable campaign (k38).

## Decisions (running log)

- Judge every row by named contract claims, never by the instrument's own
  summary. The same claim list judges the repaired product and the row's one
  named broken subject, and a broken run counts as detected only when it
  violates the claim it names; a timeout, crash or different claim is not a
  detection. Fixture cleanup is asserted on every row, repaired or broken.
- S02: three kinds of private break at the runner's owner seams (ignored report,
  bound outlived by 1 s, ECHILD keeps identity), each failing its named claim.
  A first draft named the wrong claim for the wait-ECHILD row (no Reaped event
  is true of the identity subject too); the row's discriminating claim is that a
  lost identity settles at once instead of waiting out the deadline.
- S03/F03: broken subjects are fixed suffixes on the worker case name
  (`-dropped`, `-slack`, `-unguarded`, `-ignored`), each failing its named claim
  through a new `detects` helper that requires the claim in the failure. The
  F03 reader rows gain the missing "failure ignored" subject; the claim is
  `no-managed-start`, because an ignored reader failure admits the managed child.
- S04/S05: one PTY role, unknown status after a failed Started check, with the
  fixture standing in for a child that left raw mode. S04 unrelated's two named
  defects (foreign takeover, dropped uncertainty report) are both checked.
- Route the worker driver and C4 `pty_case` through the shared 30/5/5
  controller. One accessor, `Controller::poll`, was added (non-blocking exit
  check, status recorded once). A role's own children (foreign holder, C4
  grandchildren) remain the role's bounded cleanup; only cooperative abort or
  the controller's group kill reaches them on a failed case.
- A protected `task check` exposed a k35 instrument race: E05's acknowledgement
  read the launcher latch right after an asynchronous signal and once saw it
  empty. The controller now polls acknowledged probes until the handler's run
  counter is seen (bounded window) where the launcher is the target. This is an
  instrument fix, not a product change.
- No tree-owned instrument review is cut. Every broken subject was seen failing
  the claim it names, the seams are `cfg(test)`-only and equivalent in non-test
  builds, the claims were taken from the frozen S table, and the one defect
  found by running (E05) was found by a protected check, not by reading. The
  reserved RC<=5 stays unspent for k38 or a precise continuation.
- Charge P<=30/E<=18/RC<=1, D=X=0 (the projected P<=27 was exceeded by the E05
  instrument fix and the controller routing). Cumulative bounds are
  D<=115/RC<=78/P<=372/E<=132/X=0 under caps D<=135/RC<=87/P<=402/E<=174/X=0;
  remaining D<=20 exclusively phase closure, RC<=9/P<=30/E<=42/X=0 for k38 and
  the optional review. Development controls on aarch64-apple-darwin, not the
  immutable campaign; no family is accepted.
