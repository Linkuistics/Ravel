# native-entry-incumbent-instrument-k47

**Reviews:** native-entry-incumbent-baseline-k42

## Goal

A fresh-context, inspection-only adversarial read of the whole incumbent
instrument before k43 freezes it: the incumbent build, its seams and overlay,
the baseline modules, and the plan's classification of all 153 rows (65 fails,
56 already-correct, 32 rejected premises). Produce findings and no fixes. The
question is whether any row's class is wrong, in either direction: a failure the
incumbent was never observed to commit, or a held claim read by an instrument
that cannot fail.

## Context

Read the k42 node brief, then `scripts/native-incumbent.sh`,
`scripts/native-incumbent/{seams.patch,instruments.patch,files/}`,
`docs/native-entry-campaign-plan.json` (the `baseline` entries) and the k44, k45
and k46 sections of `docs/native-entry-development-evidence.md`. The producer
leaves are k44, k45 and k46 under k42; their decisions logs carry each session's
judgement calls. The k45 session spent its one in-session reviewer on the runner
rows; k46 spent none, because this leaf is its scheduled review.

Where to look first, in order of what a wrong call would cost:

1. k44's placement of the refused-selection barriers (`before-finalise`,
   `selection-failure-before-finalise` in the incumbent's `run.rs`): the E06/E08
   refusal classification depends on it, and the F04 selection rows' unmet
   "handled signals blocked at finalisation" claim depends on the same boundary.
2. The F04 selection and final-restore rows (14) and F05's four final-mask-restore
   rows, where the one unmet claim in several rows is a repaired-design property
   (a retained block) or a label ("final mask release"), restated here as "the
   report names the failed operation". Is that restatement fair, or does it
   manufacture a failure? The evidence note lists these as judgement calls 1
   and 2.
3. The 32 rejected premises. For each, is the named missing operation really
   absent from the incumbent under the row's own state, and is a hosted
   alternative (a seam at an operation the incumbent does perform) being refused
   without cause? Especially F07 TERM-default (the incumbent's `Handlers::drop`
   restores the entry action; is that the "default-action transition" the row
   names?), F02 managed repair (the incumbent repairs `SIG_IGN` only), and S01.
4. F01: the substituted report is applied by re-running the incumbent's own
   `record()` after `main`, keeping the real record except for TERM. Does that
   measure the incumbent's initializer, or the seam?
5. The paired F05 Started row, hosted at the runner with the observer standing in
   for the front's swallowed restore.
6. Whether any seam adds behaviour. The incumbent's non-test build must be
   unchanged in effect; `cargo build -p ravel` and `-p keyed-launch` with and
   without `--features native-faults` were built, but read the `cfg(not(test))`
   branches rather than trusting that.

## Done when

- Every finding names the row or file, the claim it puts in doubt and a concrete
  failure scenario, or says the read found nothing.
- The review states which rows it re-derived from source and which it took from
  the evidence note.
- If it has findings worth acting on it cuts one `integrate-review-impl` leaf
  whose body carries this handle and not the findings.

## Notes

Inspection only: do not edit the incumbent tree, the plan or the instruments.
`task test:native-incumbent-controls` and `task test:native-campaign-controls`
reproduce the build and the driver's failing controls. A development campaign of
the finished state ran 109 tests with none failing and reconciled 153 of 153.

## Decisions (running log)

- The worker baseline's controller loses its group reservation before its
  orphan cleanup and has a blocking timeout wait. Record these as two concrete
  fixture defects, affecting S03, F03 and paired F03, rather than as incumbent
  product failures.
- The paired F05 Started observer substitutes the implementation of failure
  handling, not a report at the incumbent's restore operation. Record its
  baseline failure as unsupported by that instrument; the single end-to-end
  Started rows remain separate evidence.
- The two F07 TERM-default rejected premises overlook the TERM disposition
  restore in the incumbent's own Handlers::drop. Record this as a hostability
  finding; do not invent new measured verdicts for the unrun alternative.
- Cut one integrate-review-impl sibling carrying this review's handle, leaving
  all triage and fixes to it. K42 stays open and k43's freeze follows integration.
  Charge RC<=10 minutes including inspection, findings, bookkeeping, retirement
  and commit; D=P=E=X=0. Cumulative D<=115/RC<=96/P<=463/E<=162/X=0. The k41
  standing direction removes budget gates; no funding leaf or parent close.

## Findings

### P1 — Worker fixture cleanup can kill a reused process group

At `scripts/native-incumbent/files/crates/ravel/src/worker/native_tests.rs:469–492`,
`s03_run` reaps the role through `Child::try_wait`, waits another 300 ms,
probes its former numeric process group and sends SIGKILL to that number if the
probe succeeds. The probe establishes existence, not the group's identity or
the controller's ownership. The comment that only the incumbent's worker can
keep that group present is false once the last original member exits.

For example, a F03 mask-fault run or the primary-off S03 control can complete
its worker teardown normally. After the role is reaped, its now-unused PID/PGID
can be assigned to an unrelated process group before the delayed probe. The
probe then succeeds and line 492 kills that unrelated group. Even a genuinely
surviving worker can exit between probe and kill, opening the same reuse race.
The receipt can also mislabel that other group as an orphaned incumbent worker.

This affects all eight S03 rows, all six F03 rows, paired F03 and their controls
through the shared helper. It violates k28's no-signalling-reaped-identities
fixture requirement and the selected design's requirement that controllers own
independent child handles and never reuse a numeric group
(`docs/native-entry-transition-design.md:395–400`). Keep an owned reservation
through group cleanup, or separately own the surviving fixture processes; an
existence probe after reap is insufficient. This is a source counterexample,
not an observed kill of an unrelated process.

### P1 — Paired Started failure is manufactured by the fixture callback

For
`paired__case=F05-Started-restore-recovery-report__mode=omit-primary-report-secondary`,
`scripts/native-incumbent/files/crates/keyed-launch/src/run/baseline_faults_tests.rs:94–118`
calls the infallible `run_observed` API with a fixture observer that evaluates
and discards `Err(EIO)`. There is no parent-mask restore in that callback, no
mask to omit, no primary site record, and no call to the incumbent front's
restore/failure-handling code. `select(0, 0, ...)` selects no primary operation.
Only the terminal-handback secondary is faulted at a real incumbent operation.

At lines 514–526 the test requires the resulting runner error to contain
`Started parent transition failed`. That text cannot be produced from the
discarded expression. If the incumbent front were changed to check and retain
its actual Started restore error correctly, this row would still report the
same missing primary, because that front is never exercised. Thus the row
records the fixture's decision to swallow the error as an observed incumbent
failure. It also does not perform the row's named omitted primary operation.

This violates k42's rule that a seam substitutes a report at an operation the
incumbent actually performs, without supplying its response. Host the pair at
the actual front restore with an owner-local recovery-report seam, or otherwise
make the measurement execute the incumbent operation and its real handling.
The two single F05 Started rows already exercise the actual front and support
the separate claim that the incumbent swallows that transition; they do not
turn this synthetic paired run into an observation of both faults together.

### P2 — TERM-default rows reject an operation the incumbent performs earlier

The `F07__operation=TERM-default__mode=omit-EIO` and
`F07__operation=TERM-default__mode=perform-report-EIO` entries in
`docs/native-entry-campaign-plan.json:657–670` reject their premise because
`reraise` contains no `sigaction`. But the complete incumbent propagation path
does set TERM to default: the role itself calls `drop(handlers)` at
`scripts/native-incumbent/files/crates/ravel/src/cancellation/baseline_propagation_tests.rs:100–102`.
At revision `0515419edd29`, `Handlers::drop` in
`crates/ravel/src/cancellation.rs:134–139` restores the saved TERM action using
`sigaction`. In the row's default-entry state that action is SIG_DFL. This is
the transition on which the incumbent's own `reraise` contract relies.

Both modes can therefore be hosted at that existing TERM restore with the
first TERM consumed, handled signals blocked and no pending TERM, exactly the
state this role establishes. Omitting it leaves the front's TERM handler in
place; performing it and substituting EIO leaves default installed. The
incumbent can then proceed through its existing unblock and raise without a new
mechanism. Those outcomes still need to be measured before classification.

The custom-handler control at lines 103–113 installs a new handler *after*
the real default transition, so it proves only that `reraise` does not repeat
that operation. It cannot establish that the complete path lacks it. F04's
INT-release coverage does not cover a fault in this selected TERM operation.
Reconsider these rejected premises against the whole path, as k42 already did
when hosting S02 and S04 at equivalent incumbent operations outside the repaired
start-check helper. No test execution here establishes either replacement
verdict.

### P2 — The worker baseline's watchdog waits without a deadline

At `scripts/native-incumbent/files/crates/ravel/src/worker/native_tests.rs:467–475`,
the eight-second timeout sends group SIGKILL and then calls blocking
`child.wait()` before it can fail the test. There is no outer controller or
remaining cleanup deadline around that call; the result of `kill` is ignored.
A role that reaches the timeout while termination is unconfirmed can therefore
leave the baseline test blocked in the very cleanup path intended to bound it.
The normal path's two-second polling loop does not cover this branch.

The same S03/F03/paired-F03 identities are affected. This contradicts k28's
30-second controller envelope and bounded cleanup on timeout; the selected
design explicitly requires broken unbounded waits to be killed by their owner,
not awaited (`docs/native-entry-transition-design.md:395–405`). A successful
ordinary development run does not exercise this branch. Use bounded owned
settlement and qualify its timeout/failed-cleanup controls before freezing it.
This finding concerns the controller's software bound, not a promise of real-time
scheduling or that SIGKILL can force an uninterruptible kernel operation to end.

## Review evidence and limits

Reviewed the k44–k46 committed increment (`b91e1e861b7f5fdb335b7269f5d77cf5fa54c64d`,
`0333e560f0175bd1bc549d59a9fd978fb4706ce8`,
`26b286f2f9d832ac440a45a0d5ebba5bd2736190`) against incumbent revision
`0515419edd29`, the complete seam/instrument patches, the baseline overlays,
the build/driver/plan and k44–k46's recorded evidence and decisions. Original
incumbent source was also read directly from git. No intervening source change
was present at review entry.

Source re-derived the classification logic or hostability for the three E06
refusal boundaries, two E08 refusal rows, all four E07 rows, F01–F07, every
paired/cancellation row, S01's eight rejected report-channel premises, all
seven S02 rows, all eight S03 rows, all seven S04 rows and both S05 rows.
This means checking the instrument against the incumbent operation and
contract; it is not an independent runtime observation. The remaining E rows
(E01–E05, E06's other twelve and E08's two successful-selection rows) take their
recorded verdicts from the evidence note. Their shared build/seam surface was
inspected, but their complete runtime claims were not independently re-derived.
The reported 65/56/32 aggregate and 109-test development run are producer
evidence, not freshly verified results or a certification of all 153 rows.

No additional finding was established about the refused-selection barrier
placement: the incumbent choice really drops its handlers before returning
the failure to front settlement. A signal at that boundary can therefore
terminate it before its refusal is reported. F04's blocked/retained-subset
claims are explicit requirements of the repaired rows, so absence of that
protection can legitimately fail them even where the original EIO was checked.
F05's four final-mask-restore failures are narrower: the recorded incumbent
diagnostic is only the errno, whereas the predicate requires an operation label.
That supports the stated operation-identification failure; it must not be
presented as loss of the errno or original failure. The exact repaired string
is an instrument limitation, not evidence of a separate functional failure.

F01's rerun executes the incumbent's own `record()` and keeps all non-TERM
entry bits, which addresses the known post-runtime PIPE difference for these
fixed TERM-only faults. It remains a post-main report-substitution experiment,
not an observation of a physically failing loader-time initializer. F02 managed
repair is legitimately absent under its default-plus-SA_NOCLDWAIT precondition:
the incumbent repairs SIG_IGN only. The `cfg(not(test))` paths in the seam patch
retain the original operations; the opt-in `native-faults` paths select none by
default. No non-test behavioral change was established by this read.

Tier 2 graph evidence used project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-15T12:36:44Z`. Exact snippets and bounded traces were checked, with no
unread pages for the material queries. Coverage reported metadata matches and
no recorded gaps for the tracked instrument, plan, build, driver and contract
paths checked. That is best-effort coverage, not proof of completeness. Graph
cross-file edges do not establish runtime wiring; direct source supplies the
call and ownership arguments above. The ignored generated incumbent tree was
used for orientation, with material incumbent facts checked against the pinned
git source and committed patch rather than assumed from that tree.

No tests, builds, lint or formatting commands were run. No production/test
code, plan, instrument or evidence note was changed. Runtime reproduction,
triage, fixes, affected reruns and reconciliation belong to integration and
k43. No qualified campaign or supported-target claim follows from this review.
