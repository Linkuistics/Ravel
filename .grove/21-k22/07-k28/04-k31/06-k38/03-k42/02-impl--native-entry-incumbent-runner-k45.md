# native-entry-incumbent-runner-k45

## Goal

Record a classified `baseline` result, or a verified rejected premise with its
missing mechanism, for every runner-, worker- and terminal-level row: E05 (4),
E07 (4), S01 (8), S02 (7), S03 (8), S04 (7) and S05 (2), 40 rows. (E04 managed
was hosted by k44 through the front.)

## Context

Second child of k42; builds on k44's incumbent tree, patch mechanism, baseline
class and driver (`scripts/native-incumbent.sh`, its `seams.patch` and
`files/`; edit the built tree and run `native-incumbent.sh patches` to
regenerate the patches, then `task test:native-incumbent-controls`). The runner
and worker instruments are separate modules from the front's, so hosting them
means porting their hook sites into the incumbent's `keyed-launch` and worker
the same way k44 did the front's, and deciding per row whether the premise
exists. Read the k42 brief's premise rule first. Source read at
decomposition predicts: S01, S02 and S05 start from a *failed Started check* and
the incumbent's Started is an infallible notification, so the checked
settlement whose stops, observations and waits would be faulted does not exist;
verify that against source, and host a row only if its path and faulted
operation exist (S03's second-reader-create teardown, S04's unknown-status
`Lease::reclaim(None, false)` reached through an observation-then-failed-reap,
E05's
transparent-SIGINT path, and E07's first-consumed latch, which the incumbent
overwrites with a plain store).

## Done when

- Each listed row has a `baseline` record or a rejected premise naming the exact
  missing operation, reconciled by identity.
- The runner-level baseline instruments judge by the same named claims as the
  repaired ones wherever the incumbent exposes the fact; where a claim's fact
  cannot exist (no operation report channel), that is recorded as the reason, not
  as a baseline failure the incumbent was never observed to commit.
- Charges and balances recorded in the brief chain.

## Notes

Do not add a start check, an operation-report channel or a bounded settlement to
the incumbent to make a row hostable: that widens the incumbent and turns it
into a different subject. Record the missing mechanism instead.

## Decisions (running log)
