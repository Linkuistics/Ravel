# EngineeringSkills

EngineeringSkills is the engineering pack of
[Grove](https://github.com/Linkuistics/grove). It ships Grove's `impl` and
`prototype` session kinds, their review and integration steps, and nine
engineering craft skills as agent skills. Grove's core supplies the spine that
each kind loads.

## Kinds

| Kind | Skill | Purpose |
| --- | --- | --- |
| `impl` | `grove-impl` | Produces code, documents or tests that ship, and checks each framework decision against the source |
| `review-impl` | `grove-review-impl` | Reads one implementation adversarially and writes findings, not fixes |
| `integrate-review-impl` | `grove-integrate-review-impl` | Triages the findings of one `review-impl` leaf and applies the real ones |
| `prototype` | `grove-prototype` | Builds a cheap, throwaway artifact whose deliverable is the human reaction to it |
| `review-prototype` | `grove-review-prototype` | Reads one prototype adversarially and writes findings, not fixes |
| `integrate-review-prototype` | `grove-integrate-review-prototype` | Triages the findings of one `review-prototype` leaf and applies the real ones |

`grove-impl` adapts a passage of
[addyosmani/agent-skills](https://github.com/addyosmani/agent-skills). Its own
`LICENSES/addyosmani-agent-skills.LICENSE` holds that licence text.
`grove-impl` also names the spine's `references/execute.md`, which states the
leaf-wide review allowance.

The review and integration kinds belong to families whose members ship in
several packs. Their family procedures therefore stay in Grove's spine, as
`references/review.md` and `references/integrate-review.md`. Each of the four
kinds loads the `grove` skill and then reads its family file there.

## Craft skills

| Skill | Purpose |
| --- | --- |
| `coding-style-bash` | Bash coding standards |
| `coding-style-elixir` | Elixir coding standards |
| `coding-style-python` | Python coding standards |
| `coding-style-rust` | Rust coding standards |
| `coding-style-swift` | Swift coding standards |
| `coding-style-typescript` | TypeScript coding standards |
| `using-codebase-memory` | Queries a codebase knowledge graph through the `codebase-memory-mcp` command line |
| `simplify-project` | Simplifies a mature repository and keeps each contract that its consumers use |
| `doubt-driven-development` | Gives a non-trivial decision to a fresh-context reviewer that tries to disprove it |

A craft skill is guidance that a session of any kind can use. No Grove mandate
names it.

`doubt-driven-development` has a section, "Composition with Grove", that yields
to Grove's review allowance inside a Grove leaf. The doubt check below compares
that section with Grove's text.

## Layout

The source directory is `plugins/engineering`. It is the Claude Code plugin
root and the Ravel source directory. It has two roots:

- `skills` holds the six kind skills.
- `craft` holds the nine craft skills.

The plugin manifest lists `"skills": ["./craft/"]`. Claude Code scans `skills/`
by default and adds `craft/`. `plugins/engineering/PROVENANCE.md` holds the
sources of the craft skills. The repository root holds the files that the pack
does not deliver: this README, the licence, the Taskfile and the doubt check in
`scripts/`.

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
keg holds the source directory `plugins/engineering` at
`share/ravel/skills/engineering`. Homebrew links it into
`$(brew --prefix)/share/ravel/skills/`.

Ravel 2 delivers the skills. Ravel 2.0.0 reads that directory when its settings
name no sources, and it gives each session that it starts the skills of
both roots. `brew upgrade` keeps the pack current. No harness finds the
skills in the keg alone, and this pack has no native marketplace entry or
installer step.

Ravel 1.x delivers no skill. While the unversioned `ravel` formula is
Ravel 1.x, use the keg-only `ravel@2` for each Grove invocation.
[Skills](https://github.com/Linkuistics/grove#skills) in Grove's README gives
the commands and the upgrade order.

### Development

A checkout of this repository can serve as a trial source. Name
`<checkout>/plugins/engineering` with the roots `skills` and `craft` in the `skills` object of the
Ravel settings. Ravel captures the uncommitted changes of the checkout. It
does not fetch or update the checkout.

## Checks

Run every check of the pack in this directory. mise supplies each program
that the checks call, from `packs/mise.toml`:

```sh
mise exec -- task check
```

The checks use Grove's conformance runner and Grove's spine from a plain Git
checkout of Grove, under `.cache/grove/`. The `Taskfile.yml` variable
`GROVE_COMMIT` names that Grove commit. The variable `GROVE_REPOSITORY` names
the public Grove repository.

- `task check:plugin` checks the Claude Code plugin with
  `claude plugin validate`, which covers the manifest and the craft skills of
  `craft/`.
- `task check:doubt` checks the composition of `doubt-driven-development` with
  Grove. `scripts/doubt-composition.sh` holds the skill's half of seven
  assertions that Grove's tests moved to this pack. Each assertion also reads
  one matching phrase from Grove's checkout. Then
  `scripts/doubt-composition.test.sh` runs the controls. For each assertion,
  a mutated copy of the skill and a mutated copy of Grove's file must each
  fail that assertion alone.
- `task check:conformance` checks the pack alone, then the aggregate of
  Grove's core and this pack. Conformance covers only the kind skills in
  `skills/`. It does not read `craft/`.

`check:plugin` and `check:doubt` are the checks of the craft skills.

`GROVE_COMMIT` names Grove commit `f45c5a53796f2a5bf634fec340cb0d7aef63e184`,
which removed Grove's own copies of these skills. The pack passes alone, and
the doubt check passes. The aggregate reads each engineering kind and each
moved rule once.

The doubt check reads Grove's text at `GROVE_COMMIT`. A change of Grove's
wording therefore reaches this pack when the pack moves `GROVE_COMMIT` forward.

## Tool requirements

The skills name these tools. A session needs only the tools of the skill that
it uses.

| Skill | Tools |
| --- | --- |
| `coding-style-bash` | Bash, ShellCheck, shfmt and bats-core |
| `coding-style-elixir` | Elixir and Mix, with `mix format`, Credo and Dialyzer |
| `coding-style-python` | Python, uv, Ruff, pyright and pytest. mypy is acceptable in an existing project |
| `coding-style-rust` | Rust and Cargo, with rustfmt and Clippy |
| `coding-style-swift` | Swift and SwiftPM, with swift-format and SwiftLint |
| `coding-style-typescript` | TypeScript, Biome and vitest. Node.js, pnpm or Bun as the project configures |
| `using-codebase-memory` | `codebase-memory-mcp`. The skill names the version that its behaviour notes describe |

The checks of this repository need Bash, Git, Task and, for `check:plugin`,
Claude Code.

## Provenance

The skills come from Grove commit `d65914d57015d88924edf5dc9d22df467ff82264`,
at these paths:

- `plugins/grove/skills/grove-impl/`, with `SKILL.md` and
  `LICENSES/addyosmani-agent-skills.LICENSE`
- `plugins/grove/skills/grove-prototype/`
- `plugins/grove/skills/grove-review-impl/`
- `plugins/grove/skills/grove-integrate-review-impl/`
- `plugins/grove/skills/grove-review-prototype/`
- `plugins/grove/skills/grove-integrate-review-prototype/`
- `plugins/linkuistics/skills/coding-style-bash/`,
  `coding-style-elixir/`, `coding-style-python/`, `coding-style-rust/`,
  `coding-style-swift/` and `coding-style-typescript/`
- `plugins/linkuistics/skills/simplify-project/`
- `plugins/linkuistics/skills/using-codebase-memory/`, with `SKILL.md` and
  `references/reading-the-graph.md`
- `plugins/linkuistics/skills/doubt-driven-development/`, with `SKILL.md` and
  two files in `references/`

The four rows of `plugins/engineering/conformance/rules.tsv` come from Grove's
`plugins/grove/conformance/rules.tsv` at that commit. The doubt check holds the
skill's half of assertions from Grove's
`crates/grove-llm/tests/composition_guidance.rs` and
`crates/grove-llm/tests/session_kind_guidance.rs`.

Grove keeps the full history of these files. This repository imports none of
it. The move made these changes:

- Each kind skill has the bare-name opening of Grove's external pack contract.
  The kinds have no other change.
- `doubt-driven-development/references/measured-workstream.md` links Grove's
  `docs/candidate-lessons.md` by its public URL. That record stays in Grove.
- `doubt-driven-development/LICENSES/addyosmani-agent-skills.LICENSE` is new.
  It is a copy of the file of `grove-impl`.
- `plugins/engineering/PROVENANCE.md` holds the section of Grove's
  `plugins/linkuistics/PROVENANCE.md` for `doubt-driven-development`. The
  other eight craft skills have no section there.

The other craft skills have no change.

Grove commit `f45c5a53796f2a5bf634fec340cb0d7aef63e184` removed Grove's copies
of these files. `GROVE_COMMIT` names that commit, so the aggregate check reads
each engineering kind once.

`grove-impl` and `doubt-driven-development` adapt passages of
[addyosmani/agent-skills](https://github.com/addyosmani/agent-skills), which
uses the MIT licence. Each holds that licence text in its own `LICENSES/`.

## Licence

EngineeringSkills uses the Apache License 2.0, as Grove does. The `LICENSE`
file holds the full text.
