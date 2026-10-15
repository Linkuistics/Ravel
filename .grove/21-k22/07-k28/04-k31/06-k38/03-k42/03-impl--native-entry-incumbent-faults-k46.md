# native-entry-incumbent-faults-k46

## Goal

Port the fault sites to the operations the incumbent performs and record a
classified `baseline` result, or a verified rejected premise, for the F families,
the four paired failures and the two queued-TERM variants: F01 (3), F02 (6), F03
(6), F04 (14), F05 (20), F06 (6), F07 (6), paired (4), cancellation (2), 67 rows.

## Context

Third child of k42; builds on k44 and k45 (the build, patch and baseline
mechanism is `scripts/native-incumbent.sh`; see k44's decisions). K41's source-only read, to verify per
row: unchecked operations (runner installs, Started restore, handler `Drop`,
`shielded`, reinstate actions, `reraise` relying on `Drop`) likely `fails`;
checked ones (selection handler query and installs, final `restore_mask`,
pre-exec mask set, capture-mask failure through `recorded()`) likely
`already-correct`; rejected premises likely F02 worker watching (3) and managed
repair under default plus SA_NOCLDWAIT (2), F04 runner final restore (2), F05
finalisation-time block, pending and membership at all three boundaries (12) and
the policy-refusal restore (2), F07 explicit TERM default (2) and the paired
policy-refusal restore (1). The incumbent's front block is only the pre-launch
`block_and_check`; confirm each against source rather than copying the list.

## Done when

- Every F, paired and cancellation row has a `baseline` record or a rejected
  premise with its missing operation, reconciled by identity.
- With k44 and k45 landed, all 153 rows carry an explicit baseline
  classification, reconciled by identity and not by count; the driver rejects a
  missing, unclassified or reasonless row in its selftest, and the development
  run shows zero unclassified rows.
- Charges and balances recorded in the brief chain, and the node's closing
  obligations (brief promotion, evidence note section) discharged.

## Notes

A seam injects a fault or records a fact at an operation the incumbent already
performs; it never adds the check, the report or the mechanism. If a row needs a
mechanism, record the rejected premise and say which.

As the last child of k42, decide whether the whole incumbent instrument earns
one tree-owned `review-impl` leaf before k43's freeze (the node brief leaves that
to k42). The first thing a reviewer should read is k44's placement judgement for
the refused-selection barriers, because the E06/E08 refusal classification
depends on it.

## Decisions (running log)
