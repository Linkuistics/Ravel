# native-entry-campaign-funding-k41

## Goal

Put one precise funding question to the human before the incumbent subject
(k42) starts: whether to renew the phase caps by exactly the shortfall the
remaining campaign needs, computed from actual spend through k40. Do not
allocate, spend or narrow anything in this leaf.

## Context

Read the k38 node brief (decomposition and Done when), the k31, k28 and k22
briefs and `docs/native-entry-development-evidence.md`, whose "Campaign tooling
and row identity (k40)" section says what now exists and what does not.

K40 charged P<=25/E<=10/RC<=1, D=X=0. Cumulative bounds stand at
D<=115/RC<=79/P<=397/E<=142/X=0 under caps D<=135/RC<=87/P<=402/E<=174/X=0,
leaving D<=20 exclusively phase closure and RC<=8/P<=5/E<=32/X=0 for k42, k43
and the optional RC<=5 instrument review. The producer recomputes from actuals;
nothing below is an allocation.

**Projection made at k40.** The measurement tooling is cheap to run: the whole
native selection is about 90 seconds and 59 tests, and the full development
campaign about 90 seconds more with builds. What costs P is the incumbent:

- *Incumbent subject and baseline records (k42), about P<=30/E<=12.* The
  pre-repair product is `0515419edd29`, the parent of k30. It lacks the Started
  check API, the capture result and `native_receipt` seam, the barrier
  controller, the fault hooks and the main-thread role dispatch; the diff from it
  to the instruments is about 8,400 lines. Before pricing, read which families'
  fixtures an incumbent can host with only private test seams added (the E
  families and S01-S03 look hostable; F02, F04-F07 and the paired cases have no
  fault site in the incumbent at all and are candidates for explicit
  rejected-premise records), because that is the main price lever.
- *Frozen campaign, R01 per Taskfile task, docs reconciliation and the support
  continuation cut (k43), about P<=15/E<=20.*
- *Coordination.* This session's own RC<=3 is charged inside the renewal it asks
  for; closure RC<=2. The optional RC<=5 review of the new instruments is unfunded
  unless the human wants it.

That is about P<=45/E<=32/RC<=5 (plus this session's RC<=3) against
RC<=8/P<=5/E<=32: a shortfall of roughly P<=40, no D/E/X increase.

## Done when

- The question is asked with its three parts: the trade-off (renew by the exact
  shortfall; renew less and accept that the incumbent baseline covers only the
  families it can host, with the rest recorded explicitly as rejected fixture
  premises; or stop and keep the matrix unmet), a recommended answer, and the
  evidence behind it. Acceptance is not narrowable by the producer and the D<=20
  closure reserve is not borrowable.
- The answer is recorded verbatim, the caps and balances updated through the
  brief chain, and k42-k43 left live with their charters unchanged.
- If renewal is declined, the leaf records exactly which rows and obligations
  would remain unmet and cuts no producer that cannot finish.

## Notes

K39 is the precedent for the shape of this question and for charging this
session's RC inside the renewal. No product or instrument work.

## Decisions (running log)

- One funding decision with variants, not three or more interdependent open
  questions: matrix, seams, envelope and amendments were settled by k22/k26/k28
  and the k38 decomposition. Below the grilling threshold. No new test seam: the
  incumbent's private seams are instrument internals under k22's frozen rules
  (private, owner-local, fixed data, no installed switch).
- Actual charges verified in the k36/k37/k40 task files (P≤52/E≤24/RC=0,
  P≤30/E≤18/RC≤1, P≤25/E≤10/RC≤1). Cumulative D≤115/RC≤79/P≤397/E≤142/X=0
  under caps D≤135/RC≤87/P≤402/E≤174/X=0; balances D≤20 exclusively phase
  closure, RC≤8/P≤5/E≤32/X=0.
- Hostability read of `0515419edd29` (source only, family level; k42 owns and
  verifies the per-row classification). "No fault site" there means no seam,
  not no operation: the incumbent performs most faulted operations, usually
  unchecked. Three classes. (1) The 54 E rows need observation/boundary hooks
  only. (2) About 75 rows need a private fault seam ported to an operation the
  incumbent performs: all 32 S rows (its kill/waitid/reap/try_wait, blocking
  waits and terminal recovery) and about 43 F/paired/cancellation rows (the
  initializer queries, `shielded` and `thread::spawn`, both handler sets, the
  front's final and Started `restore_mask`, pre-exec `reinstate`, `reraise`, the
  runner's SIGCHLD query). (3) About 24 rows target an operation the incumbent
  never performs and are rejected premises whatever the funding: F02 worker
  watching (3) and managed repair under default+SA_NOCLDWAIT (2; it repairs
  only an inherited ignore), F04 runner final restore (2; runner handlers are
  never restored), F05 finalisation-time block/pending/membership at all three
  boundaries (12; its only block is pre-launch `block_and_check`) and the
  policy-refusal restore (2), F07 explicit TERM default (2; `reraise` relies on
  the handlers' `Drop`) and the paired policy-refusal restore (1). Source
  predicts both outcomes inside class 2: checked operations (selection handler
  query/installs, final `restore_mask`, pre-exec mask set, capture-mask failure
  via `recorded()`) likely `already-correct`; unchecked ones (runner installs,
  Started restore, handler `Drop`, `shielded`, reinstate actions) likely `fails`.
- Recomputed price (proposal, not approval), from k36/k37's actual ~1 P per row
  for seam-plus-instrument construction with about a third reused when porting,
  plus fixed incumbent work. K42: fixed P≤13/E≤3 (frozen incumbent workspace and
  digests, compile shim for the shared instruments, `baseline` class with three
  failing selftests, recording); E hooks P≤14/E≤4; S seams P≤10/E≤4;
  F/paired/cancellation seams P≤16/E≤4; premise records P≤2 (P≤4 if the 43 F
  rows are recorded instead of hosted). K43 keeps k40's P≤15/E≤20. RC≤8: this
  session 3, k42/k43 coordination and closure 5. Full hostable baseline:
  P≤70/E≤35/RC≤8, exact shortfall P≤65/E≤3, D=RC=X=0, caps
  D≤135/RC≤87/P≤467/E≤177/X=0. E/S-only baseline: P≤56/E≤31/RC≤8, shortfall
  P≤51, caps D≤135/RC≤87/P≤453/E≤174/X=0; its 43 hostable F rows would carry
  "seam unfunded" records, which k42's "cannot host" definition does not fit.
  Least certain: the compile shim and the pre-exec/thread-creation seam ports.
  The optional RC≤5 instrument review stays unfunded in every option. This
  exceeds k40's pre-read P≤40 because more rows are hostable than it assumed.
- The human answered verbatim **“I'm not actually concerned about the budget -
  just do what is required.”** Read for this question as the full required
  scope: k42 measures every row the incumbent can host with private test seams,
  and only rows whose operation it never performs become rejected premises; no
  narrowing. Read also as a standing direction: budget caps no longer gate
  required work, so no further funding-question leaf is cut for it, and
  sessions still record actual charges where their charters ask. X=0/zero
  production disruption is not a budget and still binds. The optional
  instrument review is not required; k42 may cut it before k43's freeze only if
  it judges an adversarial read necessary.
- Charge conservative RC≤4, D=P=E=X=0 (one above the projection, for the
  heavier hostability read), including verification, the agreement, recording,
  retirement and commit. Bookkeeping only: renew P≤65/E≤3/RC≤1 so that balances
  equal the recomputed remaining price. Caps D≤135/RC≤88/P≤467/E≤177/X=0;
  cumulative D≤115/RC≤83/P≤397/E≤142/X=0; remaining D≤20 (phase closure),
  RC≤5/P≤70/E≤35/X=0 for k42 and k43.
- No leaf is cut: k42 and k43 keep their charters, and k42's context now points
  here for the hostability read. Recorded in the k38, k31, k28, k22 and root
  briefs and the evidence note. K38, k31, k28 and k22 keep live work, so no node
  closes. Rust 1.85/Linux, returning host, adoption, K4/k6/G2–G5/R-W and all
  root/book obligations remain live.
