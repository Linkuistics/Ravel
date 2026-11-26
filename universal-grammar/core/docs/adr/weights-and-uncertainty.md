# Weights and uncertainty stay separate

Reports apply book and section weights after analysis, and the reference report is an equal-book baseline. Preference weights, relevance assessments, extraction uncertainty and interpretation uncertainty remain separate inputs. URG never combines them into one confidence or quality score.

Reweighting recomputes the prevalence or prominence of patterns that a discovery run already found. It does not show what a discovery run under the new weights would find. Each report states which of the two it is. Each discovery run records its settings, so a later run can repeat discovery under other weights.

## Trade-off

Separate dimensions resist a one-number summary. In return, each dimension stays visible, and a change to one can be traced through the reports.

## Rejected alternative

One composite score per book or passage. It would hide why a source counts, and a change to one input could not be traced.

Source: decisions 5 and 6 of the human's request in `docs/milestones/vertical-slice.md`.
