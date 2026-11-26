# unification-k32

**Integrates:** unification-k31

## Goal

Triage the findings from `unification-k31` and apply the accepted repairs to
the planned subtree produced by `unification-k2`.

## Context

- Read the findings from the review's own commit, found by its stable handle.
  The review records its producer commit and uses that commit's coordinates.
- The root brief and the running log of `plan-k1` are the requirements. The
  producer task `unification-k2` and the `grove-planning` skill state the
  planning contract.

## Done when

- Each finding is verified against its source and classified as an unclear
  contract, a real issue, an accepted trade-off or noise, with the reason.
- Accepted repairs meet the agreed acceptance and material-risk threshold.
  The tree, node briefs and root decomposition agree after any repair.
- Every binding requirement retains an owner. Unfinished acceptance stays
  live, and any change to the owner's agreed scope has explicit confirmation.

## Notes

- This leaf owns triage, not an obligation to accept every finding. Do not
  copy the findings into this body's completion criteria.
- The review was inspection-only; it made no production or test changes and
  ran no test, build, lint or formatting commands.
