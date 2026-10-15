# native-entry-incumbent-baseline-k42

## Goal

Build the incumbent subject and record, for every manifest row, what the
pre-repair product does under the same claims the repaired product is held to:
fails them, already holds them, or cannot host the instrument. Invent no baseline
failure and do not reuse a broken subject as a stand-in for the incumbent.

## Context

Child of k38 after k40's tooling and k41's funding answer. Read the k38 brief,
k40's section of `docs/native-entry-development-evidence.md`,
`docs/native-entry-campaign-plan.json` and `scripts/native-campaign.sh`.

The incumbent is `0515419edd29` (the parent of k30). Build it as its own frozen
tree with its own digest manifest, outside the repaired tree, so nothing about it
moves a repaired subject. Rows are judged by the same named claims and recorded
through the same write-once recorder under the subject name `baseline`. The
driver and plan have no `baseline` verdict class yet: add one whose outcome is
observed and classified (`fails`, `already-correct`, or `rejected-premise` with
its reason) and never required to fail, and give it failing controls like the
other guarantees. Shared fixture code that has to change to compile against the
incumbent changes the repaired instrument digest: rerun the affected repaired
rows rather than reasoning about it.

Known: the E03 skip-repair mutant conforms on Darwin because exec clears the
private flag; the E04 worker row is the discriminator there. Host-only receipts
qualify only aarch64-apple-darwin.

## Done when

- The incumbent subject is built reproducibly from `0515419edd29` plus only
  private test seams, and digested before and after each run.
- Every one of the 153 rows has an explicit `baseline` classification with the
  recorded facts or the reason its fixture premise cannot be hosted, reconciled
  by identity, not by count.
- The driver rejects, in a selftest, a missing baseline record, an unclassified
  one and a `rejected-premise` without a reason.
- Charges and balances recorded in the brief chain; no immutable evidence is
  published here.

## Notes

If hosting a family costs more than priced, record it as a rejected premise with
the exact missing seam rather than widening the incumbent. A precise
continuation beats a silent narrowing.

## Decisions (running log)
