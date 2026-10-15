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
