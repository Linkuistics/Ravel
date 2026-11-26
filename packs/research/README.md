# ResearchSkills

ResearchSkills is the research pack of
[Grove](https://github.com/Linkuistics/grove). It ships Grove's research
session kinds as agent skills. Grove's core supplies the spine that each kind
loads.

## Kinds

| Kind | Skill | Purpose |
| --- | --- | --- |
| `research-a` | `grove-research-a` | One half of a citation-disciplined survey of literature and prior art |
| `research-b` | `grove-research-b` | The other half of the survey pair, with the same brief |
| `combine-research` | `grove-combine-research` | Unites the two surveys and flags each disagreement |

The `research-family` skill holds the procedure that `research-a` and
`research-b` share. It is not a kind. Each half loads it after Grove's `grove`
skill.

## Layout

The source directory is `plugins/research`. It is the Claude Code plugin root
and the Ravel source directory. Its only root is `skills`, which holds the
three kind skills and the family skill. The repository root holds the files
that the pack does not deliver: this README, the licence and the Taskfile.

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
keg holds the source directory `plugins/research` at
`share/ravel/skills/research`. Homebrew links it into
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
`<checkout>/plugins/research` with the root `skills` in the `skills` object of the
Ravel settings. Ravel captures the uncommitted changes of the checkout. It
does not fetch or update the checkout.

## Checks

Run every check of the pack:

```sh
task check
```

The checks use Grove's conformance runner from a plain Git checkout of Grove,
under `.cache/grove/`. The `Taskfile.yml` variable `GROVE_COMMIT` names that
Grove commit. The variable `GROVE_REPOSITORY` names the public Grove
repository.

- `task check:conformance` checks the pack alone, then the aggregate of
  Grove's core and this pack.
- `task check:plugin` checks the Claude Code plugin with
  `claude plugin validate`. It skips when Claude Code is not installed.

## Provenance

The skills come from Grove commit `6eb881035a8eea1f4bb91ca1394dcbe7ddf1ec2e`,
at these paths:

- `plugins/grove/skills/grove-research-a/`
- `plugins/grove/skills/grove-research-b/`
- `plugins/grove/skills/grove-combine-research/`
- `plugins/grove/skills/grove/references/research.md`, which is now
  `plugins/research/skills/research-family/SKILL.md`

Grove keeps the full history of these files. This repository imports none of
it. The move changed the opening paragraph of each kind skill and added the
frontmatter of the family skill. It changed no other text.

Grove commit `57414e00cd876a9c0e1f8021786384ba7e368580` removed Grove's copies
of these files. `GROVE_COMMIT` names that commit, so the aggregate check reads
each research skill once.

## Licence

ResearchSkills uses the Apache License 2.0, as Grove does. The `LICENSE` file
holds the full text.
