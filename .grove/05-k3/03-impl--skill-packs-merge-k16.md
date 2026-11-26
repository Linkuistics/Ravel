# skill-packs-merge-k16

## Goal

Bring RequirementsSkills, EngineeringSkills, ResearchSkills and WritingSkills
into the Ravel repository, each as one snapshot, on the terms of
`merge-terms-k15`. Then delete their local checkouts.

## Context

- The running log of `merge-terms-k15` holds the terms. They cover the
  snapshot, the layout, the mise rule, the root `check` and the deletion of
  the local checkouts.
- `product-layout-k34` runs first, so the root `Taskfile.yml` and `mise.toml`
  exist.

## Warnings

- Warning: Do not delete a pack checkout before you show that GitHub holds its
  work. A deleted checkout cannot be restored.

## Done when

- Each pack is in `packs/<name>/` as one commit that holds the files of its
  trunk. The names are `requirements`, `engineering`, `research` and
  `writing`. Each commit message names the source repository and the commit.
- `packs/mise.toml` declares every program that the pack tasks call. The
  plugin check of each pack no longer skips when Claude Code is absent.
- The root `check` runs `mise exec -C packs/<name> -- task check` for each
  pack, and each check passes. The launcher's check still passes.
- The root `README.md` lists the four packs, each with its directory.
- The four local checkouts under `~/Development/AgenticOrchestration/` are
  gone.
- No change goes into or from Grove or Ravel-Strategies, and PersonalSkills
  stays as it is.

## Notes

- A pack checkout holds nothing that GitHub lacks when its `main` equals
  `main@origin`, its working copy is empty and no other change holds work. Its
  ignored files are regenerable: Grove checkouts in `.cache/`, a `ruff` cache
  and jj metadata. On 2026-11-26 all four checkouts met that test.
- Each pack Taskfile builds its paths from `{{.ROOT_DIR}}`. A separate run in
  the pack's directory keeps those paths right. So leave each Taskfile as it
  is, apart from the skip in its plugin check.
- Each pack check fetches its own pinned Grove commit from GitHub, and each
  pack keeps its pin.
- If the four packs do not fit one session, finish whole packs, and leave the
  rest in a live sibling. Each merged pack is a working state.
- Check that the merged packs hold no owner configuration or secrets.
