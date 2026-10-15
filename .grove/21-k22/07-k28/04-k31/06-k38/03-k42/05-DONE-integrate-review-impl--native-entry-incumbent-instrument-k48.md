# native-entry-incumbent-instrument-k48

**Integrates:** native-entry-incumbent-instrument-k47

## Goal

Triage the committed findings of native-entry-incumbent-instrument-k47 against
the incumbent instrument and its contract. Apply the findings that survive
triage and verify the resulting complete instrument before k43's freeze.

## Context

Read the review through its stable handle and commit, the k42 brief, and the
k44–k46 implementation commits it identifies. The findings live on the review;
this charter does not predetermine their acceptance. The baseline build and
Taskfile entry points are identified by the review and k42's brief.

## Done when

- Every review finding has an explicit evidence-backed disposition.
- Accepted changes are verified at their actual seams, affected subjects and
  rows are rerun and individually reconciled, and the plan and development
  evidence reflect only the resulting supported classifications.
- Preserve all 153 identities, private-seam fidelity and the incumbent revision;
  distinguish substituted reports, physical operations and fixture cleanup.
- Record new charges under the standing k41 direction. Check k42's Done when
  before any close; k43 still owns the immutable campaign, documentation
  reconciliation and support continuation, and all parent/root work stays live.

## Notes

This is the tree-owned integration immediately following k47 inside k42.
No immutable campaign was taken by the review. Historical budgets no longer
gate required work; X=0 and the agreed scope and qualification obligations bind.

## Decisions (running log)

Triage of the four k47 findings, each reproduced from the committed source before
classification (the findings were read from k47's commit `496939a`, not from this
body). No in-session reviewer is spent; none of the four is redesign.

- **P1 worker fixture cleanup (`s03_run`) — real issue, fix.** The source
  counterexample holds: `Child::try_wait` reaped the role, then a 300 ms sleep, an
  existence probe `kill(-pid, 0)` and `kill(-pid, SIGKILL)` addressed a numeric
  group nothing reserved any more. Verified on this host that an unreaped group
  leader keeps the group addressable: a group holding only the zombie leader
  answers `EPERM` to signal 0, one with a live member answers success, and `ESRCH`
  comes only after the reap. The fix is to observe the role's exit with
  `waitid(WEXITED|WNOHANG|WNOWAIT)`, probe and kill the group while the leader is
  unreaped, and reap last. Affects S03 (8), F03 (6), paired F03 and their controls
  through the one helper; those rows are rerun individually.
- **P2 watchdog (`s03_run` timeout branch) — real issue, fix.** `child.wait()`
  after the group kill was unbounded and the kill result ignored. Replace with a
  bounded (2 s) unreaped settlement, report the kill errno, and panic without
  waiting if the role is still unconfirmed. Same rows as above.
- **P1 paired F05 Started — real issue, fix by hosting at the front.** Source
  confirms the runner role's observer evaluated and discarded `Err(EIO)`, so the
  recorded `primary-retained` failure was the fixture's choice. The front's own
  `restore_mask_at("Started-parent-restore")` is already the hosted operation of
  the two single F05 Started rows. The pair is now one real front run: that
  restore omitted at the front, and the runner's terminal-handback report
  substituted after the physical handback through a `native-faults`-only seam in
  `recovery_report` (it adds no check, report or repair). The runner role's
  Started stand-in is removed so no fixture callback can swallow an error again.
  The verdict is whatever the real run shows; it is rerun and filed from it.
- **P2 F07 TERM-default (2 rows) — real issue, fix by hosting.** Source confirms
  the incumbent performs the operation on the propagation path: `Handlers::drop`
  reinstates the TERM entry action (`SIG_DFL` in the default-entry state) just
  before `reraise`, and the role already calls `drop(handlers)`. F04's rows fault
  only INT's restore, so the TERM restore was unhosted, not covered. The premise
  rule (an operation the incumbent performs on the row's path) therefore hosts
  them. The plan's rejected-premise pattern is removed; both rows become observed
  and take the verdict of a real run, with the existing custom-handler control
  retained (it still shows `reraise` itself performs no `sigaction`).

- **Applied and verified.** The fixes are in `scripts/native-incumbent/` (overlay
  files, `seams.patch`, `instruments.patch`) and the plan; the repaired product
  and the repaired instruments are untouched. Verification at the seams:
  `native-incumbent.sh build` from the committed patches (the patches apply with
  `--fuzz=0`); `task test:native-incumbent-controls` and
  `task test:native-campaign-controls` pass; two new fixture controls, each seen
  to hold on this host (an unreaped leader keeps its group `EPERM`, not `ESRCH`;
  the bounded wait returns at its deadline), and an extended F07 control (the
  omitted TERM restore changes the ending from signal death to `exit(143)`).
- **Rerun and reconciled row by row.** The incumbent as k47 reviewed it was
  rebuilt from `HEAD`'s patches in a scratch root and run beside the corrected
  one, on the same host. 121 observed rows before, 123 after; every one of the
  121 kept its verdict, the 45 rows whose notes carry named claims kept identical
  unmet/held lists, and the two new rows are the F07 TERM-default fails. The paired
  F05 Started row is still a fail on the one claim `primary-retained`, now at the
  real front restore. A development campaign (`task test:native-campaign`)
  executed 112 tests, none failed, digested 327 subjects unchanged, and
  reconciled 153 of 153 with no baseline problem. `task check` passed 338/46.
- **Result: 153 of 153 by identity: 67 fails, 56 already correct, 30 rejected
  premises** (k46's 65/56/32 corrected by the two hosted rows). The remaining 30
  rejected premises were not re-opened beyond the two F07 rows, and the six rows
  k47 took from the evidence note (E01–E05, E06's other twelve, E08's two
  selection-success rows) were not re-derived from source here either. The
  distinction between substituted reports, physical operations and fixture
  cleanup is kept: the substituted handback report and the substituted TERM-default
  report are reports at operations the incumbent performs; the cleanup is the
  fixture's own and is now owned and bounded.
- **Charge** (conservative bounds, not measurements): P<=35/E<=10/RC<=3, D=X=0,
  no in-session reviewer spawned. Cumulative D<=115/RC<=99/P<=498/E<=172/X=0
  against the historical caps RC<=88/P<=467/E<=177; RC and P exceed them and,
  under the standing k41 direction, no funding leaf is cut. K42's Done when is
  met (build reproducible and digested; every row classified by identity; the
  driver's selftests reject a missing, unclassified and reasonless row; charges
  recorded). K42 therefore closes as a node; k43 still owns the immutable
  campaign, the documentation reconciliation and the support continuation, and
  k38, k31, k28, k22 and the root all stay live.
