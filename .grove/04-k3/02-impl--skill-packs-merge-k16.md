# skill-packs-merge-k16

## Goal

Merge RequirementsSkills, EngineeringSkills, ResearchSkills, WritingSkills and
PersonalSkills into the Ravel repository, with their history, on the terms
that `merge-terms` settled.

## Context

- The running log of `merge-terms-k15` holds the layout, the history method,
  the Taskfile integration and the push rule.

## Done when

- Each pack is in its agreed place, and its history is reachable from Ravel's
  history.
- Ravel's Taskfile runs the checks of each pack, and they pass.
- Ravel's own checks still pass.
- No change goes into or from Grove or Ravel-Strategies.

## Notes

- If the five packs do not fit one session, finish whole packs and leave the
  rest in a live sibling. Each merged pack is a working state.
- Check that the merged packs hold no owner configuration or secrets.
