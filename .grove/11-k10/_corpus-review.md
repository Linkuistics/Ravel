# corpus-review-k10 — brief

## Goal

Review the 32 skills of the five merged packs for readability. Record their
intended semantics, simplify their language and remove redundant machinery.
The owner decides each substantive change of policy.

## Done when

- Each of the 32 skills has a review that records its intended semantics.
- The comparisons that the agreed method requires ran, and their limitations
  and regressions are reported.
- The owner decided each substantive change of policy.
- The original material and its provenance stay available.
- The review recorded its findings on TestAnyware, superpowers and Quint for a
  later stage, and it changed none of them.

## Decomposition

The review comes after the merge and after `pack-form`. It then edits one
repository, and it reads the skills in their final form. That form shows which
harness mechanics Ravel now owns, so the review can remove them from the text.

1. `review-method`: the owner settles the method of the comparisons.
2. `review-plan`: a planning session cuts the review leaves from that method.

The method decides the shape of the review: the comparisons, the kinds and the
order of the packs. So the review leaves wait for it, and this node holds that
context for them.

## Pointers

- Issue 6, step 7 and its acceptance criterion on representative skills.
- Linkuistics/Ravel#7 holds the review of Grove's core and DesignSkills. This
  node does not review them.
