# RequirementsSkills

RequirementsSkills is the requirements pack of
[Grove](https://github.com/Linkuistics/grove). It ships Grove's requirements
session kinds as agent skills. Grove's core supplies the spine that each kind
loads.

## Kinds

| Kind | Skill | Purpose |
| --- | --- | --- |
| `requirements` | `grove-requirements` | Establishes what to build, in the human's own words, and decides whether the open questions need a full interview |
| `review-requirements` | `grove-review-requirements` | Reads one requirements artifact adversarially and writes findings, not fixes |
| `integrate-review-requirements` | `grove-integrate-review-requirements` | Triages the findings of one `review-requirements` leaf and applies the real ones |

`grove-requirements/grilling.md` holds the interview procedure. It is a
resource of the `grove-requirements` skill, not a kind.
`grove-requirements/reader-test.md` holds the procedure of a reader
checkpoint. The skill reaches it only when the leaf holds a reader
checkpoint. It is not a kind either.

The review and integration kinds belong to families whose members ship in
several packs. Their family procedures therefore stay in Grove's spine, as
`references/review.md` and `references/integrate-review.md`. Each of the two
kinds loads the `grove` skill and then reads its family file there.

The ADR, spec and glossary formats also stay in Grove's core, in the spine's
`ADR-FORMAT.md`, `SPEC-FORMAT.md` and `CONTEXT-FORMAT.md`. The moved text names
those files, and each route resolves them in the `grove` skill.

## Layout

The source directory is `plugins/requirements`. It is the Claude Code plugin
root and the Ravel source directory. Its only root is `skills`, which holds the
three kind skills. The repository root holds the files that the pack does not
deliver: this README, the licence and the Taskfile.

Grove's external pack contract, `docs/specs/external-pack-contract.md` in the
Grove repository, states the layout, the references between skills and the
conformance rules.

## Install

Install the `grove` formula with Homebrew:

```sh
brew tap linkuistics/taps
brew trust linkuistics/taps
brew install grove
```

Homebrew 6.0.0 and later do not trust a tap that `brew tap` adds. Grove's
formula loads its dependency `ravel` from the same tap, so trust the tap
before the first install. [Install Grove](https://github.com/Linkuistics/grove#install-grove)
in Grove's README gives Homebrew's rules.

Grove's formula, from the release after 22.8, carries Grove's core and the five
domain packs, and this pack is one of them. Core supplies the `grove` skill
that each kind loads. Each pack is at the commit that Grove's pack check
accepted, so a change to this pack reaches users through a Grove release. The
keg holds the source directory `plugins/requirements` at
`share/ravel/skills/requirements`. Homebrew links it into
`$(brew --prefix)/share/ravel/skills/`.

Ravel 2 delivers the skills. Ravel 2.0.0 reads that directory when its settings
name no sources, and it gives each session that it starts the skills of
the root `skills`. `brew upgrade` keeps the pack current. No harness finds the
skills in the keg alone, and this pack has no native marketplace entry or
installer step.

Ravel 1.x delivers no skill. While the unversioned `ravel` formula is
Ravel 1.x, use the keg-only `ravel@2` for each Grove invocation.
[Skills](https://github.com/Linkuistics/grove#skills) in Grove's README gives
the commands and the upgrade order.

### Development

A checkout of this repository can serve as a trial source. Name
`<checkout>/plugins/requirements` with the root `skills` in the `skills` object of the
Ravel settings. Ravel captures the uncommitted changes of the checkout. It
does not fetch or update the checkout.

## Checks

Run every check of the pack in this directory. mise supplies each program
that the checks call, from `packs/mise.toml`:

```sh
mise exec -- task check
```

The checks use Grove's conformance runner from a plain Git checkout of Grove,
under `.cache/grove/`. The `Taskfile.yml` variable `GROVE_COMMIT` names that
Grove commit. The variable `GROVE_REPOSITORY` names the public Grove
repository.

- `task check:conformance` checks the pack alone, then the aggregate of
  Grove's core and this pack.
- `task check:plugin` checks the Claude Code plugin with
  `claude plugin validate`.

`GROVE_COMMIT` names Grove commit `ca7f6c685209da7d0d86a4f8b51fad06114115ef`.
That commit adds the document contract and the reader checkpoint to Grove's
spine, and `grove-requirements` reads them. It is later than the commit that
removed Grove's own copies of these skills. The pack passes alone, and
the aggregate reads each requirements skill and each moved rule once.

## Provenance

The skills come from Grove commit `6cdab3a9883342e68fe51b3913d8c72ddb26e3bf`,
at these paths:

- `plugins/grove/skills/grove-requirements/`, with `SKILL.md` and
  `grilling.md`
- `plugins/grove/skills/grove-review-requirements/`
- `plugins/grove/skills/grove-integrate-review-requirements/`

Grove keeps the full history of these files. This repository imports none of
it. The move changed the opening paragraph of each kind skill and changed no
other text.

Grove commit `302f85429e4b8f8ada02be594687155109dfbf6b` removed Grove's copies
of these files. `GROVE_COMMIT` names a later commit, which keeps that removal,
so the aggregate check reads each requirements skill once.

`grove-requirements` adapts and bundles passages of
[mattpocock/skills](https://github.com/mattpocock/skills), which uses the MIT
licence. `grove-requirements/LICENSES/mattpocock-skills.LICENSE` holds that
licence text. It is a copy of the file in Grove's spine, so the skill's
pointers resolve inside the skill.

## Licence

RequirementsSkills uses the Apache License 2.0, as Grove does. The `LICENSE`
file holds the full text.
