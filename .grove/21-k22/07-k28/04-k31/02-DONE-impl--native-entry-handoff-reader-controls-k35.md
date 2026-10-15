# native-entry-handoff-reader-controls-k35

## Goal

Complete the families that need the k34 controller and a real PTY or escaped
reader: E04's managed ignore-only mutant, E05's four direct/terminal INT handoff
cases and E08's four complete/escaped-reader cases.

## Context

Build on the committed k34 controller; do not create another one. Read the k31
brief, the design's E04/E05/E08 rows and the evidence note. Charges come from the
k31 balance after k34's actual charge.

## Done when

- E04 managed row has its independent ignore-only broken control observed to fail;
  the raw exec-flag reachability row stays separate and is never relabelled.
- E05's four cases record SID/PGIDs/tcgetpgrp and acknowledgements; direct INT
  cancels, after-handoff terminal INT kills the default child, and publish-then-
  signal distinguishes child status from invented launcher cancellation.
- E08's four cases record the reader's actual mask, creator before/after mask, a
  bounded collect deadline and reader state; the controller owns and kills/waits
  the escaped holder; broken shielding fails the eligibility receipt.
- Protected checks still pass; PTY and holder cleanup is bounded on every path.

## Notes

Development green is not the immutable campaign (k38).

## Decisions (running log)

- E04 managed's independent mutant lives at the runner, not the ravel fixture:
  the fixture links keyed-launch in non-test form, so the ignore-only repair is a
  `cfg(test)` seam in `restore_child_watching` driven by a role process under the
  k34 controller. The runner's own query records the flags it saw immediately
  before watching. Repaired: confirmed child status 7 with the flag cleared;
  mutant: flag left set and the wait fails ECHILD. The ravel-level managed row
  stays the end-to-end positive; the raw reachability row is separate and
  untouched.
- E05 runs at the runner on a launcher role with a PTY of its own, under the same
  controller. Before handoff is the last parent act before the spawn; after
  handoff is the Started start check. The child publishes through the channel
  and the controller waits for publication through acknowledged probes, never a
  sleep. All four rows conformed, with the launcher's latch empty exactly for
  terminal INT after handoff. The invented-cancellation mutant (child SIGINT
  death reported as launcher cancellation) is detected only there. The runner has
  no admission check, so before-handoff rows still spawn and cancel; the front's
  join is E06's. Recorded, not hidden.
- Instrument defect, kept: a role exiting onto an undrained PTY blocks in tty
  close beyond SIGKILL. The controller correctly failed the row (watchdog expiry,
  unresolved identity, 30 s) and the row now drains its master on a bounded
  thread joined on every path.
- E08 uses a private worker role (same executable) rather than the real Bun
  worker, because the controller can only own the escaped holder if the worker
  hands it the descriptor: the worker sends its stdout over a private socket
  with SCM_RIGHTS and the controller spawns and owns the holder (its own group)
  before acknowledging. The holder holds stdout only, so stdout's reader stays
  blocked and stderr's completes. Reader, mask and finalisation behaviour does
  not depend on the worker implementation; the real worker remains under E01–E07.
- The finalisation barrier gained one name, `before-finalise`, at the existing
  common site, so success and refusal rows share it. At it the runner's handlers
  own the signal on success rows (front latch empty) and the front's handler,
  recorded with a test-only handler-thread store, on refusal rows. Main-thread
  ownership is therefore claimed only for refusal rows.
- Broken shielding reports success without blocking: readers' own masks record
  no handled signal blocked, the creator mask is unchanged and cancellation is
  still observed (the latch is process-wide), so only the eligibility receipt
  fails, in all four rows.
- No product defect was found; the repaired rows conformed. Production semantics
  are unchanged: every new seam is `cfg(test)` and the non-test paths are
  textually the same decisions.
- Verification after the final edit: `task test:native-entry` (26 + 6 + 2),
  `task check` (320 run/36 skipped; the only intervening failure was a clippy
  byte-char lint in the new instrument, fixed), `task test:terminal-recovery`.
  R01 by identity: all 310 prior identities present, ignored flags unchanged, 46
  new. E04/E05/E08 passed five further unchanged repeats (flake detection only).
- Charge only new work: P≤25/E≤10/RC=0, D=X=0; no reviewer was spawned. Cumulative
  bounds D≤115/RC≤74/P≤290/E≤90/X=0 under caps D≤135/RC≤74/P≤325/E≤130/X=0;
  remaining D≤20 exclusively phase closure, RC=0/P≤35/E≤40/X=0 for k36–k38.
