# WritingSkills

WritingSkills is the writing pack of
[Grove](https://github.com/Linkuistics/grove). It ships Grove's editorial
session kinds, their shared procedure and two writing craft skills as agent
skills. Grove's core supplies the spine that each kind loads.

## Kinds

| Kind | Skill | Purpose |
| --- | --- | --- |
| `draft` | `grove-draft` | Produces a document from its sources and its human-authored structure brief, and owns structure and technical truth |
| `copy-edit` | `grove-copy-edit` | Corrects sentences, terminology and consistency with the document's prose contract, without restructuring |
| `art` | `grove-art` | Makes the document's figures, in the medium that its contract admits |
| `proof` | `grove-proof` | Reads the whole document a final time, with every class in charter |

The four kinds are the stages of a document's publishing pipeline. They are one
family with one procedure. The family skill `editorial-family` holds that
procedure. Each kind loads the `grove` skill and then the `editorial-family`
skill. The family skill is not a kind, because its name has no `grove-` prefix.

## Craft skills

| Skill | Purpose |
| --- | --- |
| `technical-communication` | Controlled-English writing rules, modelled on ASD-STE100, for all natural language that a model writes. Its always-on layers put the rules in front of the model in each session |
| `writing-code-walkthroughs` | Concept-ordered, source-exact code walkthroughs for Markdown books and subsystem guides |

A craft skill is guidance that a session of any kind can use. No Grove mandate
names it.

`technical-communication` holds more than its rules. Its directory also holds
these parts:

- `always-on/`: the instruction and the reminder that the hooks send.
- `scripts/`: the checker `ste-lint.py`, the hook core `tc_hook.py` and the
  hook guard `tc-hook`.
- `harnesses/`: the Codex hooks file and the Pi extension.
- `.claude-plugin/plugin.json` and `hooks/hooks.json`: the manifest and the
  hooks of the Claude Code hooks plugin.
- `.codex-plugin/plugin.json`: the manifest of the Codex hooks plugin.
- `LICENSES/`: the licence of the vendored checker.

## Hooks plugins

The always-on layers of `technical-communication` run as hooks. A hook is a
command that the harness runs at an event, such as the start of a session. Two
hooks plugins deliver them. Both plugins have the skill directory
`plugins/writing/craft/technical-communication` as their root, so each hook
runs `scripts/tc-hook` of that directory.

| Harness | Plugin | Manifest | Hooks file |
| --- | --- | --- | --- |
| Claude Code | `technical-communication-hooks` | `.claude-plugin/plugin.json` | `hooks/hooks.json`, which Claude Code reads by default |
| Codex | `technical-communication` | `.codex-plugin/plugin.json` | `harnesses/codex/hooks.json`, which the manifest names |

Neither plugin holds a skill. Ravel delivers the skill itself.

The Claude Code manifest lists `"skills": []`. Claude Code loads a plugin root
that holds `SKILL.md`, with no `skills/` directory and no `skills` key, as one
skill. The empty list stops that, so the hooks plugin adds no second
`technical-communication` skill. A Ravel user enables this plugin alone and
receives the hooks without a native copy of the skill.

The Codex manifest names its hooks file. Codex reads the default
`hooks/hooks.json` only when the manifest has no `hooks` field. The field
therefore keeps Codex away from the Claude Code hooks file in the same root.

The writing plugin lists `technical-communication-hooks` in its
`dependencies` and declares no hook. No route installs the writing plugin, so
its manifest is layout metadata. `task check:hooks` still checks that each
hook runs once in each combination of the two plugins.

Each harness adds a hooks plugin only from a marketplace. Grove's Homebrew keg
holds the marketplace files at its root, with the name `writing-hooks` for
both harnesses. Codex resolves a `local` source only inside its marketplace
root, so the file must lie above the skill directory in the keg. This
repository therefore holds no marketplace file. [Install](#install) gives the
commands that register the files.

## Layout

The source directory is `plugins/writing`. It is the Claude Code plugin root
and the Ravel source directory. It has two roots:

- `skills` holds the four kind skills and the family skill.
- `craft` holds the two craft skills.

The directory `craft/technical-communication` is also the root of the two
hooks plugins in [Hooks plugins](#hooks-plugins).

The plugin manifest lists `"skills": ["./craft/"]`. Claude Code scans `skills/`
by default and adds `craft/`. `plugins/writing/PROVENANCE.md` holds the sources
of the craft skills.

The repository root holds the files that the pack does not deliver. They are
this README, the licence, the Taskfile and the documents in `docs/`. The
scripts in `scripts/` are also there. They are the Pi load check and the hook
count.

Grove's external pack contract, `docs/specs/external-pack-contract.md` in the
Grove repository, states the layout, the references between skills and the
conformance rules.

## Documents

These documents state, decide, explain or check the craft skills. They came
from Grove with the skills, at the same paths.

| Path | What it is |
| --- | --- |
| `docs/specs/technical-communication.md` | The specification of `technical-communication` |
| `docs/TECHNICAL-COMMUNICATION.md` | The user guide of `technical-communication` |
| `docs/design/technical-communication/` | The design views of the specification. `task design:technical-communication` serves them |
| `docs/adr/the-off-file-is-the-only-switch.md` | The decision that one off file turns off every always-on layer |
| `docs/adr/documents-are-gated-replies-are-measured.md` | The decision that each document write is checked and each reply is only scored |
| `docs/adr/technical-communication-carries-no-standard-text.md` | The decision that the skill carries no text of the standard |
| `docs/adr/behavioral-acceptance-is-closed-unmet.md` | The decision that closes the behavioural acceptance of `writing-code-walkthroughs` as unmet |
| `docs/technical-communication-publication-review.md` | The check of the skill's text against the notice of the standard |
| `docs/research/harness-hooks-a.md` | The survey of what each harness allows for hooks |
| `docs/evaluations/writing-code-walkthroughs/` | The behavioural evaluations of `writing-code-walkthroughs` and their evidence |

The finished evaluation records keep their bytes. Four scripts in the
evaluation directory read the skill, and they now name its path in this pack:
the three `harness.sh` files under `enabled/` and
`refinement-regression/template-test.sh`. The template test also reads the
plugin manifest of this pack.

## Install

### Skills

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
domain packs, and this pack is one of them. Each pack is at the commit that
Grove's pack check accepted, so a change to this pack reaches users through a
Grove release. The keg holds the source directory `plugins/writing` at
`share/ravel/skills/writing`. Homebrew links it into
`$(brew --prefix)/share/ravel/skills/`.

Ravel 2 delivers the skills. Ravel 2.0.0 reads that directory when its settings
name no sources, and it gives each session that it starts the skills of both
roots. `brew upgrade` keeps the pack current. No harness finds the skills in
the keg alone, and this pack has no native marketplace entry or installer
step.

Ravel 1.x delivers no skill. While the unversioned `ravel` formula is
Ravel 1.x, use the keg-only `ravel@2` for each Grove invocation.
[Skills](https://github.com/Linkuistics/grove#skills) in Grove's README gives
the commands and the upgrade order.

### Hook adapters

Ravel installs no hook. Register the hook adapters of Grove's keg once in each
harness. These are the documented commands, which the formula's caveats also
print.

Claude Code, once:

```sh
claude plugin marketplace add "$(brew --prefix)/opt/grove"
claude plugin install technical-communication-hooks@writing-hooks
```

The installation enables the plugin. No option installs a plugin disabled, so
a user who wants the plugin off runs `claude plugin disable` next.

Codex, after each installation or upgrade of Grove:

```sh
codex plugin marketplace add "$(brew --prefix)/opt/grove"
codex plugin add technical-communication@writing-hooks
```

Then start `codex` once and choose "Trust all and continue" at its hook
review. Codex stores the path of the versioned keg and copies the plugin into
its cache. An upgrade therefore needs both commands and the review again.

Pi, once:

```sh
ln -s "$(brew --prefix)/share/ravel/skills/writing/craft/technical-communication/harnesses/pi/technical-communication.ts" ~/.pi/agent/extensions/
```

### Development

A checkout of this repository can serve as a trial source. Name
`<checkout>/plugins/writing` with the roots `skills` and `craft` in the
`skills` object of the Ravel settings. Ravel captures the uncommitted changes
of the checkout. It does not fetch or update the checkout.

## Checks

Run every check of the pack in this directory. mise supplies each program
that the checks call, from `packs/mise.toml`:

```sh
mise exec -- task check
```

The conformance checks use Grove's conformance runner from a plain Git checkout
of Grove, under `.cache/grove/`. The `Taskfile.yml` variable `GROVE_COMMIT`
names that Grove commit. The variable `GROVE_REPOSITORY` names the public Grove
repository.

- `task check:plugin` checks both Claude Code plugins with
  `claude plugin validate`: the writing plugin, with the craft skills of
  `craft/`, and the hooks plugin.
- `task check:hooks` runs `scripts/hook-count.py`. The script counts the hook
  declarations of three installations: the writing plugin alone, the hooks
  plugin alone, and both. For each plugin, it reads the default
  `hooks/hooks.json`, the manifest's `hooks` field and the dependencies, as
  Claude Code does. Each installation must give one declaration for each of
  the six hooks. The script also checks the empty `skills` list and the hooks
  field of the Codex manifest. Its controls then mutate scratch copies of the
  pack, for example with a second declaration, and each control must fail.
  The `grove` plugin is a dependency of the writing plugin, and the task reads
  it from the Grove checkout.
- `task check:tc-checker` runs the self-test of the checker `ste-lint.py`.
- `task check:tc-hook-lint` runs ShellCheck on the hook guard `tc-hook` and on
  the Pi load check.
- `task check:tc-pi` loads the Pi extension in a real Pi, with no model
  request. It checks that the extension writes a start record. Its controls
  check that the off file stops that record and that Pi reports an extension
  that cannot load. The script skips when Pi is not installed, but
  `packs/mise.toml` supplies Pi.
- `task check:conformance` checks the pack alone, then the aggregate of
  Grove's core and this pack. Conformance covers only the kind skills in
  `skills/`. It does not read `craft/`.

The three `check:tc-*` tasks are the checks of `technical-communication`. They
came from Grove's `scripts/check.sh` and need no Grove checkout.

`GROVE_COMMIT` names Grove commit `ca7f6c685209da7d0d86a4f8b51fad06114115ef`.
That commit adds the document contract, the figure contract and the house
style to Grove's spine, and the stage skills read them. It is later than the
commits that removed Grove's copies of the editorial skills, the craft skills,
their hooks and their documents. The pack passes alone. The aggregate reads
each editorial skill and each moved rule once.

## Tool requirements

| Skill or check | Tools |
| --- | --- |
| `technical-communication` | `python3`, version 3.9 or later, for the checker and the hook core. A POSIX `/bin/sh` for the hook guard `tc-hook` |
| `writing-code-walkthroughs` | None |
| The hooks plugins | `python3` and `/bin/sh`, as for `technical-communication`. Each hook runs `/bin/sh scripts/tc-hook` |
| `task check` | Bash, Git, Task, Python 3 and ShellCheck. Claude Code for `check:plugin`, and Pi for `check:tc-pi`. Each of those two skips when its tool is not installed |
| `task design:technical-communication:render` | D2 |

The scripts of the evaluation records name further tools, such as Codex, Ruby
and `jq`. They describe finished runs, and `task check` does not run them.

## Provenance

The editorial skills come from Grove commit
`f45c5a53796f2a5bf634fec340cb0d7aef63e184`, at these paths:

- `plugins/grove/skills/grove-draft/`
- `plugins/grove/skills/grove-copy-edit/`
- `plugins/grove/skills/grove-art/`
- `plugins/grove/skills/grove-proof/`
- `plugins/grove/skills/grove/references/editorial.md`, which is now
  `editorial-family/SKILL.md`

The craft skills and their documents come from Grove commit
`31ec9117c9dacfe993e80313addd432f2930d679`, at these paths:

- `plugins/linkuistics/skills/technical-communication/`, with every tracked
  file
- `plugins/linkuistics/skills/writing-code-walkthroughs/`
- The two sections of `plugins/linkuistics/PROVENANCE.md` for these skills,
  which are now in `plugins/writing/PROVENANCE.md`
- Each document in [Documents](#documents), at the same path
- `scripts/pi-extension-load.test.sh`

The two hooks plugins come from Grove commit
`31ec9117c9dacfe993e80313addd432f2930d679`, with these changes:

- `craft/technical-communication/hooks/hooks.json` is Grove's
  `plugins/linkuistics/hooks/hooks.json`. Its script path loses the part
  `skills/technical-communication/`, because the plugin root is now the skill
  directory.
- `craft/technical-communication/.claude-plugin/plugin.json` and
  `scripts/hook-count.py` are new.

Grove keeps the full history of these files. This repository imports none of
it.

Grove commit `31ec9117c9dacfe993e80313addd432f2930d679` removed Grove's copies
of the editorial files. Grove commit `4cfac62958815219ba5a71941674244a05ff9b56` removed Grove's copies of the
craft skills, the hooks and the moved documents. `GROVE_COMMIT` names a later
commit, which keeps both removals.

The move changed the first two paragraphs of each kind skill. The first
paragraph is now the opening of Grove's external pack contract, and the second
names the `editorial-family` skill as the family procedure. The family file
gained frontmatter. Its body has one change: the phrase
"`references/execute.md`'s" became "the spine's". The family skill loads no
other skill, so a path into the spine cannot resolve from it.

The move of the craft skills made these changes:

- `technical-communication/.codex-plugin/plugin.json` names this repository.
  Its description names the routes that install the skill from this pack.
- `docs/TECHNICAL-COMMUNICATION.md` links the skill at its path in this pack.
  It links Grove's README by its public URL.
- `docs/adr/behavioral-acceptance-is-closed-unmet.md` links
  `plugins/writing/PROVENANCE.md`.
- `scripts/pi-extension-load.test.sh` and the four evaluation scripts name the
  skill's path in this pack.
- `plugins/writing/PROVENANCE.md` has its own opening paragraphs. Two paths in
  the section of `technical-communication` start with `craft/`.

The commit that records Grove's removal corrected the claims that the moved
documents made about the delivery through Grove's `linkuistics` plugin and the
checks of Grove's `scripts/check.sh`:

- `docs/specs/technical-communication.md` states the layout in this pack. It
  names the hooks plugin as the Claude Code entry, Grove's
  `install.sh --pack` as the Pi route and the `check:tc-*` tasks as the checks.
- `docs/TECHNICAL-COMMUNICATION.md` installs the writing plugin or the hooks
  plugin, the Codex marketplace of this repository and Grove's installer with
  `--pack`. Its checker paths and its issue link name this repository.
- `docs/adr/the-off-file-is-the-only-switch.md` names the Claude Code hooks
  plugin in place of the `linkuistics` plugin.
- `docs/adr/technical-communication-carries-no-standard-text.md` names the
  routes from this repository and the earlier publication from Grove.

The commit for Homebrew installation removed the root Codex marketplace file
of this repository. That file came from Grove with the hooks plugins, with
the name `writing-hooks`. Grove's keg now holds the Codex marketplace file of
that name. The commit also made these changes:

- This README, the user guide, the specification and its harness map state
  Homebrew installation, Ravel delivery and the registration commands of the
  hook adapters.
- `docs/adr/technical-communication-carries-no-standard-text.md` names the
  routes from Grove's keg.
- The descriptions of the two hooks plugin manifests and a comment of the Pi
  extension name Ravel and the keg in place of Grove's installer.

The dated records keep their bytes. They are the publication review, the
harness survey and the evaluation records.

`technical-communication` vendors the checker of
[danyuchn/asd-ste100-skill](https://github.com/danyuchn/asd-ste100-skill),
which uses the MIT licence. Its own `LICENSES/` holds that licence text. The
editorial skills adapt no passage of a third party.

## Licence

WritingSkills uses the Apache License 2.0, as Grove does. The `LICENSE` file
holds the full text.
