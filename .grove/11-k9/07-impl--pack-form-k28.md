# pack-form-k28

## Goal

Move the five merged skill packs to the one skill form. Their hooks, such as
those of the writing rules, move into the metadata form.

## Context

- The reworked spec `docs/specs/skill-discovery.md`.
- The packs that `skill-packs-merge-k16` brought into the Ravel repository.

## Done when

- Each skill of the five packs uses the one authoring form and, where it needs
  one, the metadata form. No harness-specific manifest remains in a pack.
- The hooks of the writing rules come from the metadata form, and they run on
  each supported harness through Ravel's adapters.
- Each converted skill keeps its text and meaning. Readability edits belong to
  the corpus review.
- The checks of each pack pass in Ravel.
- The bare names that Grove's prompts use still resolve to the converted
  skills.

## Notes

- This conversion is mechanical. Record each case where the form cannot carry
  what a skill needs. If the case changes behaviour, stop and ask.
- DesignSkills converts in `design-skills-merge`, after its merge. Grove's own
  skills stay in their current form in this grove.
- For Gemini CLI, the command-line seam shows that the hooks run. The root leaf
  `gemini-cli-trial` owns the real trial, and it does not gate this leaf.
