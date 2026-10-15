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

- **Premise verification (source, with controls).** Counted in `git show
  <rev>:<path>` of the incumbent `0515419edd29` and of the repaired `HEAD`, the
  same patterns on both: `start_check` 0/13 and `stop_reports` 0/5 in
  `keyed-launch/src/run.rs`, `ERROR_SETTLEMENT` 0/9, `operation_reports` 0/2;
  `thread::Builder` 0/1 and `thread::spawn` 1/0 in `ravel/src/worker.rs`. The
  repaired counts are the positive control; an earlier run of the loop printed
  0 for every pattern in *both* trees because `$rev:crates` is a zsh history
  modifier, and was discarded for exactly that reason. The incumbent has no
  `catch_unwind`, `set_hook` or `panic =` setting anywhere in the front, runner
  or worker, so a failed `thread::spawn` panics the process.
- **Premise classification, as first drafted and as settled.** The first draft
  rejected S01 (8), S02 (7), S03 (8) and S05's runner row because every one
  begins at a failed Started check or a failed reader creation that the
  incumbent cannot reach as a checked settlement. The single in-session reviewer
  (spent; one fresh Explore context, given only the artifact, the premise rule
  and the source paths) found that this contradicted what the same session had
  already hosted: E07's runner-wait-error substitutes an errno at the incumbent's
  own `waitid`, so saying S02's observation rows "would measure a settlement the
  incumbent does not have" was inconsistent, and the human's standing direction
  is that only a row whose operation the incumbent never performs is rejected.
  Findings classified: (1) S02 observation rows (3) and wait rows (3): valid and
  actionable, rehosted on the incumbent's own error paths. (2) S03: valid, but
  the reviewer's cheapest form (inject the secondary on an ordinary trigger)
  would change the row's primary; hosted instead by injecting the primary at the
  incumbent's `thread::spawn`, where std's documented result is a panic, with the
  secondary armed at the teardown's own stop, kill and `try_wait` so "never
  reached" is observed, plus a control that the armed secondary fires when the
  primary is absent. (3) S04 and S05 terminal substituted `None` after a
  successful reap: valid fidelity finding, a combination the incumbent cannot
  reach; rehosted through observation failure then failed reap, its real unknown
  status path. The own-foreground precondition stays a fixture-placed state
  (a visible trade-off, stated in the evidence). (4) "unbounded waitid loop" and
  "its own C4 tests drive it": valid wording corrections (a `WNOHANG` poll with
  500 ms sleeps; one unit test reaches `reclaim(None, false)` without a PTY).
  (5) S01 kept rejected, as the reviewer allowed: its one named claim reads a
  per-attempt stop report and the incumbent has no operation report channel,
  which the leaf's Done when names as a reason rather than a failure. (6) S02
  always-running and S05's runner row kept rejected: the incumbent enters no
  error path for a child that is merely still running.
- **Why S02's claims are judged by meaning, not by the repaired string.** The
  repaired claims match its own message text ("settlement observation:"). The
  incumbent says "cannot wait on the launched child: ... (os error 5)". Each
  such predicate was restated as "names the failed operation and its errno";
  `start-check-source-retained` and `ownership-released` have no fact to read
  and are not judged. The EINTR rows fail only because the incumbent treats the
  first EINTR as fatal and kills at once; that is stated in the record, not
  presented as a physical EINTR.
- **Instrument faults found and fixed before measurement.** libtest prints its
  own failure text into the role's redirected result descriptor, so "empty
  result" is judged by the absence of a selection result; the incumbent's
  worker survives the role's panic, so the fixture observes that, ends the
  group and asserts it is gone, leaving nothing behind; and an assumed "teardown
  ran" marker was one this build does not print, so S03's reached/not-reached
  reading uses the line the teardown does print and the armed-secondary control
  proves it fires.
- **Result.** 40 of 40 classified from real runs: 30 observed (14 already
  correct, 16 fails) and 10 rejected premises (S01 x8, S02 always-running,
  S05 runner). With k44: 24 fails, 52 already correct, 10 rejected, 67
  unclassified for k46. The development campaign (90 tests: 59 repaired, 31
  incumbent) digested 321 subjects, unchanged after, and all 153 repaired
  identities reconcile; `task check` passed 338/46; the incumbent and campaign
  selftests pass (17 files differ from git's export). No product defect.
- **Charge** P<=20/E<=8/RC<=1 (the one reviewer), D=X=0. Cumulative bounds
  D<=115/RC<=85/P<=439/E<=156/X=0 under caps D<=135/RC<=88/P<=467/E<=177;
  remaining D<=20 (phase closure), RC<=3/P<=28/E<=21/X=0 for k46 and k43. A
  projection, not an allocation; per the standing direction these are recorded,
  not gates. K42 stays a node: k46 (F01-F07, paired, cancellation: 67 rows)
  remains live.
