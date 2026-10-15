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
