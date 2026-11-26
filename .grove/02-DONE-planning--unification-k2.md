# unification-k2

## Goal

Decompose the work of this grove into leaves, from the requirements that
`plan-k1` settled with the owner. Find the smallest independently useful
working increments before you cut any leaf.

## Context

- The root brief states the goal, the done criteria, the agreed test seams, the
  out-of-scope work and the open questions.
- The running log of `plan-k1` holds every decision beside the owner's words.
- Linkuistics/Ravel#6 is the full requirements statement, with #4 and #5. Its
  suggested sequence is requirements and observable command-line behaviour,
  then a prototype of the Lua embedding and the delivery boundaries. Then the
  smallest coherent end-to-end path, then compatibility and the harness
  integrations. The corpus review and its comparisons come before the skill
  simplification counts as complete.

## Done when

- The tree holds leaves for every done criterion of the root brief, in
  dependency order. Each leaf has its kind and a body that names its specific
  case.
- Each research question in the root brief has a research leaf that names its
  downstream questions.
- Each owner decision that is still open has a leaf with the owner in the loop.
  These are the method of the corpus review comparisons, the publication of the
  Visualiser history and the agreement of the design spec.
- The Decomposition section of the root brief explains the chosen shape.

## Notes

- DesignSkills merges only after Grove's leaf
  `relocate-decomposition-design-k1028` lands.
- The corpus review comes after the merge of the five packs.
- The merge stage removes the abandoned Visualiser changes before the
  Visualiser merges.
- Nothing in this grove merges into or from the Grove or Ravel-Strategies
  repositories.

## Decisions (running log)

**One grove, increments as nodes.** The owner chose this grove for the first
merge stage, the corpus review and the product work. So the increments stay in
this grove, although the planning skill would cut a separate grove for each. A
split needs the owner's confirmation, and the final report offers it.

**The merge stage comes first.** It depends on nothing else, and the skill
design, the pack conversion and the corpus review all read the merged packs.
DesignSkills sits late at the root. The walk is pre-order, so a blocked leaf
inside the merge node would stop all later work.

**The harness question gets a survey pair, and the Lua question gets one
survey.** The supported harnesses come from different vendors, and the
owner's policy sends the two halves to different providers. The Lua question
has an obvious candidate, and `lua-routing` tests it empirically.

**A prototype collects the owner's reaction to the configuration experience.**
Issue 6 asks for configuration that people readily understand. A throwaway
mock-up is the cheapest way to test that, before either design. The technical
risks of the embedding go to the research and to `lua-routing`, not to the
prototype.

**Lua routing is an increment before skill selection.** A user can adopt Lua
for routing alone, and the skill design extends a contract that already
exists. The parity checks form their own leaf, because they are the
increment's acceptance.

**The corpus review waits for its method.** The owner settles the method
first, and a planning leaf then cuts the review leaves. The review comes after
`pack-form`, so it reads the skills in their final form.

**The book goes through the editorial pipeline.** The book has a document
contract, so its revision is one `draft` leaf that decomposes itself.

**This decomposition gets an adversarial review.** The tree carries months of
work. So `unification-k31` reviews it before the first increment starts.
