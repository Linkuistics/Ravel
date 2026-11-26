# unification-k31

**Reviews:** unification-k2

## Goal

Try to break the decomposition that `unification-k2` cut. Report findings,
not fixes.

## Context

- The artifact is the task tree under `.grove/` as `unification-k2` committed
  it. It includes the node briefs, the leaf bodies and the Decomposition
  section of the root brief.
- The contract is the task file of `unification-k2`, its "Done when" list, and
  the `grove-planning` skill.
- The requirements are the root brief and the running log of `plan-k1`.

## Done when

The review reports on each doubt below, with evidence from the tree.

- Does every done criterion of the root brief have a live leaf that owns it?
  Is any criterion owned only in part?
- The driver walks the tree in pre-order. Can any live leaf block the walk
  before work that does not depend on it?
- Does each increment leave the product working at its last leaf?
- Does each node earn its level, by the placement test in the spine's
  decomposition procedure?
- Do the research briefs name the downstream questions that their consumers
  need? Is the shape of each research question justified?
- Does each owner decision that is still open have a leaf with the owner in
  the loop?
- Does any leaf body contradict a decision in the running log of `plan-k1`?

## Notes

- Read the tree before the Decomposition section of the root brief. That
  section states the producer's reasons, and the review must judge the tree
  without them first.
