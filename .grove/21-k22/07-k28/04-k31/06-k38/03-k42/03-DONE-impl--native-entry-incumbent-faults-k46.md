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

- **Hosting rule applied.** A row is hosted when the operation its fixture drives
  is one the incumbent performs under the row's own state, and the seam only
  injects the report there (after the physical operation, or omitting it); a
  row naming an operation the incumbent never performs is a rejected premise
  with a control on the same fixture. 45 of 67 are hosted, 22 rejected. No seam
  adds a check, a report or a repair.
- **Source verification (k41's list, checked, not copied).** Confirmed: F02
  worker (`worker.rs` never names SIGCHLD), F02 managed repair (the runner
  repairs `SIG_IGN` only), F05's final block/pending/membership (12) and its
  policy-refusal restore (2, the refusal path never restores the mask), F07
  TERM-default (`reraise` performs no `sigaction`). Not confirmed: k41 predicted
  F04 runner final restore (2) as rejected; the incumbent's `Handlers::drop`
  does release over the runner's handler after a completed launch, so both are
  hosted (fails). Also hosted, not predicted either way: all F06 (the TERM
  restore and mask build unchecked, the mask restore checked), F07's unblock
  and raise, F03, F01 and the paired F03/F06/F05-Started rows.
- **F01 by re-running the incumbent's own `record()`.** The initializer ran
  before `main`, so the substituted report is applied by calling the same
  function again with one report replaced, keeping the real record for every
  signal but TERM so SIGPIPE (since ignored by the Rust runtime) is not handed
  to the managed child. The mask query is the incumbent's one checked path
  (already correct); the other two are read as "not ignored"/"not blocked".
- **Restated predicates.** F05's `final mask release` and F04's
  `disposition release` are the repaired product's labels; both are judged as
  "the report names the failed operation", as k45 judged S02. The F05 final
  restore rows fail on that claim alone and would be already correct under a
  weaker reading, so this is flagged to the review (k47). F04's "handled
  signals blocked at finalisation" is a retained-protection claim the incumbent
  does not have (it blocks only in the pre-launch `block_and_check`); it carries
  the five F04 selection rows' failures with the TERM rows' partial subset.
- **F06 in the front fixture** needed the repaired runner's own off-by-default
  `native-faults` feature in the incumbent's two crates and the fixture build;
  no installed build enables it, and the incumbent's non-test and
  feature-on/off builds were built.
- **Paired and queued rows.** Paired F05-Started and F06 are hosted at the
  runner with the incumbent's own C4 handback substitution; for F05 the
  observer stands in for the front's swallowed restore. Paired F03 is S03's
  wait-EIO path (one incumbent path, filed under the paired identity). The two
  queued-TERM rows are hosted end to end at the front.
- **Failure found in the build, not in a row.** A first rebuild from nothing
  failed because a closure bound in `shielded` was only valid in the test
  configuration; the non-test branch was rewritten to the incumbent's original
  calls. This is why the script builds both configurations.
- **Review.** This leaf spends no in-session reviewer: the whole incumbent
  instrument (k44-k46, 153 rows) earns one tree-owned `review-impl` leaf (k47,
  under k42, before k43's freeze), whose first reads are k44's barrier placement
  and the restated predicates above. It cuts an integration only if it has
  findings worth acting on.
- **Result.** 67 of 67 classified: 41 fails, 4 already correct (F01 mask query,
  F06 mask restore x2, F07 signal-death control), 22 rejected. Node: 65 fails,
  56 already correct, 32 rejected, 153 of 153 by identity, 0 unclassified or
  unobserved. Development campaign: 109 tests (59 repaired, 50 incumbent), none
  failed, 327 subjects unchanged after, 153 of 153 reconciled; `task check`
  338/46; incumbent and campaign selftests pass (22 files differ from git's
  export). No product defect, production code unchanged.
- **Charge** P<=24/E<=6/RC<=1, D=X=0, no in-session reviewer. Cumulative
  D<=115/RC<=86/P<=463/E<=162/X=0 under caps D<=135/RC<=88/P<=467/E<=177/X=0;
  remaining D<=20 (phase closure), RC<=2/P<=4/E<=15/X=0 for k47 and k43. As
  with k44 and k45 these are recorded, not gates (standing direction, k41); the
  P remainder is below k43's projection and is stated, not narrowed around.
