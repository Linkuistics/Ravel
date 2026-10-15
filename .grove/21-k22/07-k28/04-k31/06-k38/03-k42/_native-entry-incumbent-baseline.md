# native-entry-incumbent-baseline-k42 — brief

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

K41's decisions log holds a family-level, source-only hostability read of the
incumbent and the human's funding answer: start from that read and verify each
row against the incumbent source.

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

## Decomposition (k42)

This brief now belongs to a node: k42 proved larger than one session. Its Done
when above is unchanged and is met only by the children together. Source reading
at decomposition (the whole incumbent runner, front, cancellation and signal
state, and the repaired instruments' hook sites) found three things the k41
family-level read had not priced:

- **No shared harness reaches the incumbent unchanged.** The instruments are
  `cfg(test)` modules that call about 35 hook sites in the repaired front and
  about 45 in the repaired runner (`native_hooks`, `terminal_tests`,
  `native_tests`, the `Process` seam with checked stops and bounded settlement).
  The incumbent has the same operations but none of those seams, so each family
  needs its own port of the hooks, and the runner-level instruments are written
  against repaired internals (`supervise_started`, `Settled`, `Ended::
  operation_reports`) that the incumbent does not have.
- **The premise rule.** A row is hosted when the operation path its fixture
  drives exists in the incumbent and the faulted or observed operation is one the
  incumbent performs there; the seam only injects a fault or records a fact. A
  row whose premise needs a product mechanism the incumbent lacks is a
  rejected premise naming that mechanism, never a seam that adds the mechanism.
  Source predicts, for k45 to verify: S01 (8), S02 (7) and S05 (2) all begin
  with a *failed Started check*, and the incumbent's Started is an infallible
  notification (`observer(LaunchEvent::Started)`, and the front's
  `let _ = signal_state::restore_mask()`), so no checked settlement exists whose
  stop, observation or wait attempts could be faulted; the Started restore
  failure itself is F05's row and is hosted there, where its consequence is
  observable as a swallowed failure. S03's reader-creation premise, S04's
  `Lease::reclaim(None, false)` path and the E rows exist in the incumbent.
- **Worker bytes.** The worker source (`crates/ravel/worker`, `scripts/
  dispatch.sh`, `build.rs`) is identical between the incumbent and the repaired
  tree, so the one built `ravel-policy` is a shared, digested helper rather
  than a second build; the build asserts that identity from the trees.

Build design (k44): `scripts/native-incumbent.sh` exports the incumbent from
git, applies a stored patch of private test seams to its product files, copies
the repaired instrument sources verbatim, applies an instrument patch that adds
only the baseline module and compile shims, and builds the private fixture in
its own target directory under `target/native-incumbent/`. Baseline tests read
row identities and parameters from the frozen manifest rather than re-listing
them, and record under the subject `baseline` through the same write-once
recorder; the plan states each row's baseline classification explicitly
(`observed` or a `rejected-premise` with its reason and missing mechanism) and
the driver derives `fails` or `already-correct` from the record.

Children, in order: k44 incumbent subject, `baseline` class and the front
E rows (E01–E04 front, E06, E08: 46 rows; landed); k45 runner, worker and
terminal rows (E05, E07, S01–S05: 40 rows, S01/S02/S05 verified as rejected
premises or hosted); k46 fault families (F01–F07, paired, cancellation: 67
rows). The last child to land records the by-identity reconciliation of all 153
rows, the charges and balances, and retires the node's closing obligations.

## Decisions (running log)

- K44 (first child) built the incumbent subject (`scripts/native-incumbent.sh`:
  git export of `0515419edd29`, `seams.patch`, the repaired instruments copied
  verbatim, `instruments.patch`, the baseline module and the shared compiled
  worker), added the plan's `baseline` class and the driver's classification
  with six failing controls, and filed 46 front rows from a real run: 39
  already-correct and 7 fails. Charge P<=22/E<=6/RC<=1, D=X=0. K45 owns E05, E07
  and S01-S05 (40 rows) and k46 the 67 fault rows; 107 rows stay unclassified
  and visible in the driver. No node closes.

- K45 (second child) hosted and classified the 40 runner, worker and terminal
  rows from real runs: E05 (4) already-correct; E07 (4) fails (the handler
  overwrites the latch); S02 observation EIO and wait ECHILD already-correct,
  the other four fail (EINTR is fatal and unretried, ECHILD is followed by a
  signal, a wait EIO is not retried); S03 (8) fail (the incumbent's
  `thread::spawn` panics at a refused second reader, so no teardown runs and
  its worker is left running); S04 six already-correct and one fails (a
  missing status is treated like an exit, so a foreign foreground is preserved
  without a report); S05's terminal row already-correct. Rejected premises:
  S01 x8 (no operation report channel), S02 always-running and S05's runner
  row (no error path for a merely running child). This corrects the
  decomposition's prediction that S03's reader creation exists as a checked
  operation: it exists only as an unchecked `thread::spawn`, and was hosted as
  std documents it. The reviewer allowance of this leaf is spent. Charge
  P<=20/E<=8/RC<=1, D=X=0. Cumulative D<=115/RC<=85/P<=439/E<=156/X=0;
  remaining D<=20 (phase closure), RC<=3/P<=28/E<=21/X=0 for k46 and k43. 67
  rows stay unclassified (k46); no node closes.

- K46 (third and last child) classified the 67 fault rows from real runs and
  controls: 45 observed (41 fails, 4 already correct) and 22 rejected premises,
  each with a control on the same fixture. With k44 and k45 the node stands at
  65 fails, 56 already correct and 32 rejected, 153 of 153 by identity with none
  unclassified (the driver's selftests reject a missing, unclassified or
  reasonless row). K46 spent no in-session reviewer and cut one tree-owned
  `review-impl` leaf (k47) for the whole incumbent instrument before k43's
  freeze; the node closes when it and any integration retire. Charge
  P<=24/E<=6/RC<=1, D=X=0. Cumulative D<=115/RC<=86/P<=463/E<=162/X=0 under
  caps D<=135/RC<=88/P<=467/E<=177/X=0; remaining D<=20 (phase closure),
  RC<=2/P<=4/E<=15/X=0 for k47 and k43. The evidence note section is "Fault-
  family baseline (k46)"; the frozen campaign, docs reconciliation and the
  support continuation stay in k43.
