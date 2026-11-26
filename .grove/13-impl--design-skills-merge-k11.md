# design-skills-merge-k11

## Goal

Bring DesignSkills into the Ravel repository as one snapshot in
`packs/design/`, on the terms of `merge-terms-k15`. Then move it to the one
skill form, as `pack-form` did for the other packs.

## Context

- The node brief `first-merge-stage-k3` and the running log of
  `merge-terms-k15`.
- The body of `pack-form-k28` states the conversion.

## Warnings

- Warning: Do not delete the DesignSkills checkout before you show that GitHub
  holds its work. A deleted checkout cannot be restored.

## Done when

- Grove's leaf `relocate-decomposition-design-k1028` has landed in
  DesignSkills.
- DesignSkills is in `packs/design/` as one commit that holds the files of its
  trunk. The commit message names the source repository and the commit.
- `packs/mise.toml` declares the programs that its tasks call, and the root
  `check` runs its check.
- Its skills use the one form, and its checks pass in Ravel.
- Its local checkout under `~/Development/AgenticOrchestration/` is gone.

## Notes

- First, check whether `relocate-decomposition-design-k1028` landed. If it did
  not, ask the owner to choose. The owner can wait for it, or move DesignSkills
  to a later stage. A later stage changes the root brief's acceptance, so it
  needs the owner's explicit confirmation.
- This merge reads from DesignSkills only. Nothing goes into or from Grove.
- The language review of DesignSkills belongs to Linkuistics/Ravel#7, not here.
- The owner agreed these terms in `merge-terms-k15`, as the session's reading
  of the owner's answers for the first stage.
- Once `main` holds DesignSkills, its source repository gets a README pointer
  and goes into the GitHub archive. The root brief holds that obligation.
