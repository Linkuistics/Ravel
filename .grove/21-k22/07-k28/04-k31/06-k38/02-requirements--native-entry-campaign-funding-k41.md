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
