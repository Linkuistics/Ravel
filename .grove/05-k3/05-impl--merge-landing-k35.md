# merge-landing-k35

## Goal

Land the first merge stage on `main`. Then retire the six source repositories
and the two UniversalGrammar checkouts, on the terms of `merge-terms-k15`.

## Context

- The running log of `merge-terms-k15`, from "Each merged source repository
  ends archived" to "UniversalGrammar's personal data moves with it". There
  the owner authorised each step of this leaf.

## Warnings

- Warning: Do not push `main` before the root `check` passes. A push publishes
  the stage, and nobody can withdraw a published commit.
- Warning: Do not delete a checkout before you show that GitHub holds its
  work. A deleted checkout cannot be restored.
- Warning: Do not delete the UniversalGrammar checkout before its `inputs/`,
  `data/` and `reproductions/` are in the main checkout. They hold the owner's
  personal research data, and no repository holds a copy.
- Caution: Push each README pointer before you archive its repository. An
  archived repository refuses pushes until the owner unarchives it.

## Done when

- `main` on GitHub holds the merge stage, and the root `check` passes on it.
- The main checkout `~/Development/AgenticOrchestration/Ravel` sits on the new
  `main`. Its working copy was empty before it moved.
- UniversalGrammar's `inputs/`, `data/` and `reproductions/` are complete in
  `universal-grammar/core/` of the main checkout, and no commit holds them.
- The local checkouts `UniversalGrammar` and `UniversalGrammar-Visualiser`
  under `~/Development/AgenticOrchestration/` are gone.
- Each of the six source repositories has a README commit that points to its
  place in Ravel, and each is archived on GitHub. The six are
  RequirementsSkills, EngineeringSkills, ResearchSkills, WritingSkills,
  UniversalGrammar and UniversalGrammar-Visualiser.

## Notes

- A checkout holds nothing that GitHub lacks when its `main` equals
  `main@origin` and no other change holds work. The abandoned Visualiser
  commits and regenerable ignored files are the exceptions. The owner wants
  the abandoned commits gone.
- The Visualiser repository stays private when it is archived.
- PersonalSkills and DesignSkills are not part of this leaf. Grove and
  Ravel-Strategies receive nothing.
