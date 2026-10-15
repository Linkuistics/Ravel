# native-entry-fault-families-funding-k39

## Goal

Put one precise funding question to the human before the fault families
(k36) start: whether to renew the phase caps by exactly the shortfall the
remaining native qualification needs, computed from the actual spend recorded
up to and including k35. Do not allocate, spend or narrow anything in this leaf.

## Context

Read the k31 and k28 briefs and `docs/native-entry-development-evidence.md`.
K34 charged P≤30/E≤15/RC≤1, D=X=0 for the controller, E06 and E07. Cumulative
bounds then stood at D≤115/RC≤74/P≤265/E≤80/X=0 under caps
D≤135/RC≤74/P≤325/E≤130/X=0, leaving D≤20 exclusively phase closure and
RC=0/P≤60/E≤50/X=0 for k35–k38. Take k35's actual charge from its task file and
subtract it before computing anything below.

**Projection, not allocation, made at k34.** The barrier rows cost about 1.6 P per
row including the one-off controller. Remaining after k34: E04 managed mutant,
E05 (4, PTY), E08 (4, escaped reader), F02/F04/F05/F06 (46) with the paired and
queued-TERM cases (6), the S02/S04/S05 remainder, the frozen campaign with
immutable receipts and R01 per task, and the support continuation. At a reuse
rate of 0.5–0.8 P per remaining row, remaining P is roughly P≈90–115, E≈55–70
(the campaign alone is E≈35: baseline, broken and repaired for 153 rows, plus
protected checks and affected reruns), and coordination RC≈6–9 (one per child,
this session's own, and the tree-owned review of the new instruments if the human
wants one). Against RC=0/P≤60/E≤50 that is a shortfall of roughly
P≈30–55/E≈5–20/RC≈6–9, no D/X. The producer who runs this leaf recomputes it
from k35's actual rate rather than trusting this paragraph.

## Done when

- The question is asked with its three parts: the trade-off (renew by the exact
  shortfall; stop and keep the matrix unmet; or renew by less and accept that the
  next child ends with a precise continuation rather than a finished family),
  a recommended answer, and the evidence behind it. Acceptance is not narrowable
  and the D≤20 closure reserve is not borrowable.
- The answer is recorded verbatim, the caps and balances updated through the
  brief chain, and k36–k38 left live with their charter unchanged.
- If renewal is declined, the leaf records exactly which families would remain
  unmet and cuts no producer that cannot finish.

## Notes

K29 is the precedent for the shape of this question and for charging this
session's own RC inside the renewal it asks for. No product or control work.

## Decisions (running log)

- One funding decision with interdependent variants, not three or more
  interdependent open questions: the contract, matrix, seams and envelope were
  settled by k22/k26/k28. Below the grilling threshold; no new test seam.
- Recomputed from actual charges (proposal, not approval). After k35 the caps
  D≤135/RC≤74/P≤325/E≤130/X=0 leave D≤20 exclusively closure, RC=0/P≤35/E≤40.
  K35 cost about 2.8 P/1.1 E per row against the projected 0.5–0.8 P; k34's
  owner-boundary rows cost about 1.0 P/0.6 E per row without the controller.
  Source has no fault seam for F02, F04 or F06 and only two partial F05 seams.
  Remaining: 38 owner-site rows (F02, F04, F05 finalisation) at 0.8 P/0.4 E;
  22 live-child/PTY rows (F05 Started, F06, paired, queued-TERM, S04 six, S05)
  at 1.7 P/0.8 E, reusing k35's PTY role and the C4 seams; 22 control-only rows
  (S02, F03, S03, S04's existing row) at 0.35 P/0.2 E; fixed child work.
  Projections: k36 P≤58/E≤31/RC≤2; k37 P≤27/E≤18/RC≤1, including routing the
  older worker/terminal helpers through the controller; k38 P≤27/E≤35/RC≤2,
  including receipts tooling, the incumbent subject, evidence/docs and the
  support-continuation cut; this session RC≤3; an optional RC≤5 ceiling for one
  instrument review cut lazily before k38's freeze. Price P≤112/E≤84/RC≤13.
  Exact shortfall P≤77/E≤44/RC≤13 (RC≤8 without the review ceiling), D=X=0:
  caps D≤135/RC≤87/P≤402/E≤174/X=0. Least certain: k36's live-child rows;
  review integration, Rust 1.85/Linux qualification and returning host excluded.
- The human answered verbatim **“Renew full shortfall (Recommended)”**: add
  RC≤13/P≤77/E≤44, no D/X increase, so caps become
  D≤135/RC≤87/P≤402/E≤174/X=0. Zero production disruption, X=0 and the D≤20
  phase-closure reserve remain binding. The renewal covers this session,
  k36–k38 at the unchanged 153-row matrix, EPERM/Ended amendments and 30/5/5
  envelope, and an RC≤5 ceiling for one tree-owned instrument review that k37
  cuts before k38's freeze only if it judges an adversarial read necessary. No
  new protocol, infrastructure, support qualification, review integration or
  narrower acceptance is authorised.
- Charge conservative RC≤3, D=P=E=X=0, for context verification, recomputation,
  the agreement, brief-chain/evidence/leaf recording, verification and
  retirement/commit. Cumulative bounds are D≤115/RC≤77/P≤290/E≤90/X=0;
  remaining D≤20 exclusively closure, RC≤10/P≤112/E≤84/X=0, shared by k36, k37,
  k38 and the optional review. Each charges actual work and passes the balance
  forward; the per-child figures above are projections, not allocations. Full
  use leaves only D≤20 reserved. An overrun stops at a precise continuation.
- No leaf is cut: k36–k38 already exist with their charters unchanged; their
  contexts now carry the funding. The record goes to the k31, k28, k22 and root
  briefs and the evidence note. K31, k28 and k22 keep live work, so no node
  closes. Rust 1.85/Linux, returning host, adoption, K4/k6/G2–G5/R-W and all
  root/book obligations remain live.
