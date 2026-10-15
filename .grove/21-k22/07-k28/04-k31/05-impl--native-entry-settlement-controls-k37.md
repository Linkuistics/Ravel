# native-entry-settlement-controls-k37

## Goal

Finish the S01–S05 matching broken controls and remaining rows: S02's bound and
identity claims, F03/S03 report/deadline controls, S04's seven recovery rows and
both S05 first-HUP-then-TERM rows, preserving counts 8/7/8/7/2.

## Context

Read the design's S table and the evidence note's S sections. Runs on the k34
controller. ECHILD/reap/release disables numeric signalling; substituted reports
cannot restore released ownership or invent lifecycle facts.

## Done when

- Every S row has its failing broken control; deadlines are absolute and EINTR
  never restarts them; foreign foreground is preserved and unknown status reports
  uncertain foreground/modes.
- The 32 S identities are unchanged and individually reconciled.

## Notes

Development green is not the immutable campaign (k38).

## Decisions (running log)
