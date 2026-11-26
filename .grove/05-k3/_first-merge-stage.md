# first-merge-stage-k3 — brief

## Goal

Bring the first merge stage into the Ravel repository, in the layout of the
product. The launcher moves into `launcher/`. Four skill packs,
UniversalGrammar and UniversalGrammar-Visualiser arrive as snapshots, their
checks pass through the root `check`, and the stage lands on `main`. The stage
is the first step of one Ravel product.

## Done when

- The launcher is in `launcher/`, and the root holds only the product's files.
- RequirementsSkills, EngineeringSkills, ResearchSkills and WritingSkills are
  in `packs/<name>/`, each as one snapshot of its trunk.
- UniversalGrammar and UniversalGrammar-Visualiser are in
  `universal-grammar/core/` and `universal-grammar/visualiser/`, each as one
  snapshot of its trunk.
- The root `check` runs the check of each component in that component's
  directory and mise environment, and every check passes.
- The stage is on `main`. Each of the six source repositories has a README
  pointer to its place in Ravel, and each is archived on GitHub.
- The six local checkouts are gone. UniversalGrammar's ignored personal data
  is in `universal-grammar/core/` of the main checkout, and no commit holds it.

## Decomposition

This stage is a working increment of its own. It needs nothing from the Lua or
skill work, and it leaves the product working. It comes first because the
skill form, the pack conversion and the corpus review all work on the merged
packs. Its leaves share context that no other leaf needs: the terms of the
merge, the source repositories and their retirement.

1. `merge-terms`: the owner settles the terms of the merge.
2. `product-layout`: the launcher moves into `launcher/`, and the root becomes
   the product's.
3. `skill-packs-merge`: the four packs arrive, their checks pass, and their
   local checkouts go.
4. `universal-grammar-merge`: the two UniversalGrammar repositories arrive,
   and their checks pass.
5. `merge-landing`: the stage lands on `main`. The six source repositories get
   their pointers and their archive, and the two UniversalGrammar checkouts
   go.

The launcher moves first, because every later leaf of this grove changes the
launcher. Each later leaf then writes its final paths. The landing comes last,
because a pointer helps a reader only when Ravel's copy is public.

DesignSkills is not in this node. It waits for Grove's leaf
`relocate-decomposition-design-k1028`, and a blocked leaf here would stop the
walk before the Lua work. The root leaf `design-skills-merge` holds it.

## Pointers

- The running log of `merge-terms-k15` holds each term of the merge, beside the
  owner's words.
- The source repositories are siblings of this workspace, under
  `~/Development/AgenticOrchestration/`.
- Grove's formula ships the packs today. The move of that shipping to Ravel's
  formula is the root leaf `pack-shipping`, not this node.

## Notes

- Nothing in this stage merges into or from Grove or Ravel-Strategies. Among
  the packs, the open Grove workstream writes only into DesignSkills.
- PersonalSkills keeps its own repository and formula.
- Never publish the owner's configuration or secrets as fixtures.
