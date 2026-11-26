# first-merge-stage-k3 — brief

## Goal

Bring the first merge stage into the Ravel repository. Five skill packs,
UniversalGrammar and UniversalGrammar-Visualiser arrive with their history, and
their checks pass in Ravel. The stage is the first step of one Ravel product.

## Done when

- RequirementsSkills, EngineeringSkills, ResearchSkills, WritingSkills and
  PersonalSkills are in the Ravel repository with their history.
- UniversalGrammar and UniversalGrammar-Visualiser are in the Ravel repository
  with their history.
- The checks of each merged repository pass in Ravel, through Ravel's
  Taskfile.
- The 12 abandoned Visualiser changes are gone from jj, and the `gh-issue-1`
  workspace of the Visualiser is retired.
- The owner decided the publication of the private Visualiser history before
  any push of the Ravel repository.

## Decomposition

This stage is a working increment of its own. It needs nothing from the Lua or
skill work, and it leaves the product working. It comes first because the
skill form, the pack conversion and the corpus review all work on the merged
packs. Its leaves share context that no other leaf needs: the merge method, the
source repositories and the Visualiser cleanup.

1. `merge-terms`: the owner settles the publication of the Visualiser history
   and the terms of the merge.
2. `skill-packs-merge`: the five packs arrive, and their checks pass.
3. `universal-grammar-merge`: the Visualiser cleanup, then the two
   UniversalGrammar repositories arrive, and their checks pass.

DesignSkills is not in this node. It waits for Grove's leaf
`relocate-decomposition-design-k1028`, and a blocked leaf here would stop the
walk before the Lua work. The root leaf `design-skills-merge` holds it.

## Pointers

- The source repositories are siblings of this workspace, under
  `~/Development/AgenticOrchestration/`.
- Grove's formula ships the packs today. The move of that shipping to Ravel's
  formula is the root leaf `pack-shipping`, not this node.

## Notes

- Nothing in this stage merges into or from Grove or Ravel-Strategies.
- Never publish the owner's configuration or secrets as fixtures.
