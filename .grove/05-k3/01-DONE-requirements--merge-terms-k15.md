# merge-terms-k15

## Goal

Settle the terms of the first merge stage with the owner, before any
repository arrives. The decisive question is the publication of the
UniversalGrammar-Visualiser history. That repository is private, and Ravel is
public.

## Context

- The running log of `plan-k1`, from "Merge stages" to "The Visualiser's
  changes are garbage".
- The owner's instruction on the Visualiser changes stands. This leaf does not
  ask it again.

## Done when

The running log records the owner's answer to each question below.

- Does the Visualiser history come across in full, in part, or as one
  squashed change? A merge into Ravel publishes whatever comes across.
- Where does each merged repository go in the Ravel tree?
- How do the checks of each repository join Ravel's Taskfile?
- After the merge, which copy of each pack is authoritative while Grove's
  formula still ships the pack repositories? What happens to each source
  repository: does it stay, get archived, or get a pointer to Ravel?
- Does the push of the merged history wait for a later point, such as the
  release?

## Notes

- Bring a recommendation and its evidence to each question.
- The merge of each repository with its history must not rewrite Ravel's own
  history.
- If the answers leave the two merge leaves unchanged in scope, add nothing to
  the tree.

## Decisions (running log)

**The launcher is one part of the product, without privilege.** On 2026-11-26
the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> Even though the final deliverable will be called Ravel, it will not be the
> launcher that is currently the focus of this repo - that will only be one
> part of it, not priviledged.
<!-- tc: on -->

The product takes the name Ravel. The session reads "the launcher" as the
`ravel` program, with its crates, documents and release tooling. The launcher
is one part of the product, and it has no privilege over the other parts.
Today it owns the root of the repository. So the placement question also
covers the place of the launcher.

This statement conflicts with `CONTEXT.md`, which uses "Ravel" for the
launcher. The glossary changes when the owner settles the name of the
launcher.

**One licence for everything.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> Everything can have the same Apache 2.0 license, with Linkuistics copyright
<!-- tc: on -->

Every merged repository takes the Apache License 2.0, with Linkuistics as the
copyright owner. Ravel, the five packs and UniversalGrammar already carry
that licence, with the line "Copyright 2026 Linkuistics". Only the Visualiser
changes. Its Cargo manifests declare `MIT OR Apache-2.0`, and it has no
licence file. Third-party files keep their own licences. An example is the
IBM Plex Mono font of the Visualiser page, which has its own licence file.

**PersonalSkills stays out of the merge.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> PersonalSkills should not be included here.
<!-- tc: on -->

The session reads "here" as the merge into the Ravel repository. PersonalSkills
keeps its own repository. The first merge stage brings four packs:
RequirementsSkills, EngineeringSkills, ResearchSkills and WritingSkills. The
corpus review edits only merged packs, so it excludes the four skills of
PersonalSkills: `guardrail`, `authoring-conventions`, `machine-environment` and
`using-jujutsu`. At their trunks today, the four merged packs hold 29 skills.
The root brief counts 32 skills in five packs, and today the five trunks hold
33.

The decision changes the scope of the leaf `skill-packs-merge` and of the
corpus review. The briefs and that leaf change after the owner agrees that
the understanding is shared.

**History is no requirement of this merge.** The session asked whether the
Visualiser history enters Ravel in full, in part or as one squashed change.
On 2026-11-26 the owner answered:

<!-- tc: off because the owner's words are quoted verbatim -->
> I don't care about history in this merge.
<!-- tc: on -->

The session reads the answer as indifference for every repository of this
merge, not only for the Visualiser. Under that indifference, the session
proposes one snapshot per repository. A snapshot is one commit that holds the
files of the source trunk. Its message names the source repository and
commit.

Publication cannot be undone, and a snapshot publishes no past revision. So
the private Visualiser history stays private, and the option to publish it
stays open. The source repositories then hold the only copy of that history.
The question on their fate decides whether it stays available. The done
criteria "with their history" in the root brief and the node brief change
after the owner agrees that the understanding is shared.

**Every tool comes from mise.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> Use mise for all tooling, never rely on global availability
<!-- tc: on -->

Each component declares in its own `mise.toml` every program that its tasks
call, and its tasks run through mise. The session reads "all tooling" as every
program that a task calls. A declared tool is never absent, so no task skips
because a tool is missing. UniversalGrammar and the Visualiser already have a
`mise.toml`. The launcher and the four packs have none.

The launcher takes Rust through rustup from `rust-toolchain.toml`. The pack
tasks call `task`, `git`, `bash` and `claude` from the machine, and
WritingSkills also calls `python3` and `shellcheck`. The scripts that those
tasks run can call more. The plugin check of each pack skips when Claude Code
is absent. So the merge leaves add a `mise.toml` to each pack, and the
launcher needs one too.

Two mise facts serve the checks question. `mise exec -C <directory>` runs a
command in the tool environment of that directory. mise also sets
`RUSTUP_TOOLCHAIN`, which rustup ranks above any `rust-toolchain.toml` in a
parent directory.

**The layout of the product.** The session proposed peer directories, with the
launcher in `launcher/` and the packs in `packs/<name>/`. On 2026-11-26 the
owner amended the proposal:

<!-- tc: off because the owner's words are quoted verbatim -->
> universal-grammar should have core and visualiser subdirs
<!-- tc: on -->

The owner changed only the place of the two UniversalGrammar repositories, so
the session reads the rest of the proposal as accepted:

- `launcher/` holds the launcher.
- `packs/requirements/`, `packs/engineering/`, `packs/research/` and
  `packs/writing/` hold the four packs. Each directory takes the source ID of
  its pack. DesignSkills goes to `packs/design/` at its own merge.
- `universal-grammar/core/` holds UniversalGrammar.
- `universal-grammar/visualiser/` holds UniversalGrammar-Visualiser.
- The root holds the product's `README.md`, `LICENSE` and `Taskfile.yml`.

Each component keeps its own files, including its Taskfile and its
`mise.toml`.

The decision meets the bar for a decision record. A later move breaks each
link into the moved files, so it is hard to reverse. A repository called Ravel
with the `ravel` program in `launcher/` surprises a reader without context.
The rejected alternatives keep the launcher at the root, or put each pack at
the top level. The owner's statement about the launcher restates the
principle, and the amendment above states the one change. The first leaf that
creates this layout writes the record.

The owner's word "launcher" now names the component. The session recorded
**Launcher** in `CONTEXT.md`, and it flagged there that "Ravel" alone means
the product.

**The launcher moves now.** The session asked when the launcher moves into
`launcher/`. On 2026-11-26 the owner answered:

<!-- tc: off because the owner's words are quoted verbatim -->
> We will rename the launcher at some point, maybe `ravel` becomes a top level
> dispatcher with `ravel launch` being the subcommand.
>
> move launcher now
<!-- tc: on -->

The launcher moves into `launcher/` in a leaf of its own, before the merge
leaf of the packs. That leaf also adds the launcher's `mise.toml`, and it
writes the decision record for the layout.

The session reads the rename as a later intent, not a requirement of this
grove. One possible form makes `ravel` a dispatcher at the top level, with
`ravel launch` as the subcommand of the launcher. The glossary keeps
**Launcher** until the rename settles.

**The packs are separate only for clarity.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I'm not concerned about shipping the packs independently - I only want them
> separated for cognitive/architectural clarity.
<!-- tc: on -->

The packs keep separate directories under `packs/`, for clarity. No pack needs
a release, a formula or a version of its own. The session reads this as
permission for the root leaf `pack-shipping` to ship the packs together.

**The session decides the checks.** The session asked whether the root
`check` runs each component's own check as a separate run. On 2026-11-26 the
owner answered:

<!-- tc: off because the owner's words are quoted verbatim -->
> As far as check is concerned - you can decide that
<!-- tc: on -->

The session decides as follows:

- The root `Taskfile.yml` has one `check` task. It runs the `check` task of
  each component as a separate run, with
  `mise exec -C <component> -- task check`.
- The components are `launcher`, the four directories under `packs/`,
  `universal-grammar/core` and `universal-grammar/visualiser`. Each leaf that
  adds a component also adds it to the root `check`.
- Each component keeps its Taskfile unchanged, and it stays runnable alone in
  its directory.
- The root `mise.toml` declares only `task`. The four packs share one
  `packs/mise.toml`, because they call the same tools and mise reads the files
  of parent directories. This replaces the earlier note of one `mise.toml` per
  pack. UniversalGrammar and the Visualiser keep their own `mise.toml`.

The reason is a test of Task 3.54.0. In an included Taskfile, Task resolves
`{{.ROOT_DIR}}` to the product root, so an included pack would resolve its
paths there. Its tasks would also run with the tools of the root's run. A
separate run keeps each check exactly as it runs today.

**Each merged source repository ends archived.** The session asked whether
each source repository ends archived, with a README pointer to its place in
Ravel, once Ravel's copy is public. On 2026-11-26 the owner answered:

<!-- tc: off because the owner's words are quoted verbatim -->
> Eventually I want a single 'Ravel' package to ship that includes everything.
>
> yes, archive the merged repos so we don't accidentally commit changes to them
> Please delete the local skill repos dirs when you merge them into this so we
> don't have divergence.
<!-- tc: on -->

- After its snapshot lands, Ravel's copy of each merged repository is the only
  copy that changes.
- Once Ravel's copy is public, each of the six source repositories gets a
  README pointer to its place in Ravel. Then the publishing leaf archives it
  on GitHub. The six are the four packs, UniversalGrammar and the Visualiser.
- The merge leaf of the packs deletes the four local pack directories under
  `~/Development/AgenticOrchestration/` after their snapshots land. The
  session reads "the local skill repos dirs" as those four directories only.
- Before each deletion, the leaf shows that the local repository holds
  nothing that GitHub lacks. Today `main` equals `main@origin` in each of the
  four, and each working copy is empty.
- No check reads those directories. Grove names them only in 10 files under
  `docs/`: records, evaluation logs and evaluation scripts. The owner's Ravel
  settings name none of them, and sessions read the packs from the Grove keg.
- Until the shipping of the packs moves, Grove's formula ships the frozen pack
  commits.

The session reads the single Ravel package as a later intent. It sets the
direction of the root leaf `pack-shipping`: Ravel's formula becomes the one
package that ships every component.

**The open Grove workstream stays within DesignSkills.** On 2026-11-26 the
owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> The open grove workstream *must* be limited to DesignSkills, that we are not
> merging in this round.
<!-- tc: on -->

Among the packs, the open Grove workstream writes only into DesignSkills. So
the four merged packs have no writer outside Ravel. Their archive and the
deletion of their local directories collide with no open work.

**The merge stage lands on main.** The session asked whether the merge stage
lands on `main` as soon as it passes the root `check`. On 2026-11-26 the owner
answered:

<!-- tc: off because the owner's words are quoted verbatim -->
> yes, land it on main. Once the two UG repos are merged we can delete the
> local checkouts.
<!-- tc: on -->

- A last step of the merge stage lands the stage on `main` once the root
  `check` passes. Then it adds the six README pointers and archives the six
  source repositories.
- The two local UniversalGrammar checkouts also go, once their merge lands.
  This answer replaces the session's reading of "the local skill repos dirs"
  as the four pack directories only.
- Each pack checkout holds only regenerable ignored files: Grove checkouts in
  `.cache/`, a `ruff` cache and jj metadata. The Visualiser checkout holds only
  build output, Node packages and generated page files.
- The UniversalGrammar checkout also holds ignored personal data that no
  snapshot carries. `inputs/` holds an export of the owner's Zotero library.
  `data/` holds the artifacts that the pipeline derived from that library, and
  two of its stages call a model. `reproductions/` holds one run of the
  pipeline from 2026-11-18. The fate of these directories is the next
  question.

**UniversalGrammar's personal data moves with it.** The session asked where
`inputs/`, `data/` and `reproductions/` go before the deletion of the
UniversalGrammar checkout. It recommended a move into `universal-grammar/core/`
of the main checkout. On 2026-11-26 the owner answered: "Your recommendation".

The step that lands the merge on `main` updates the main checkout
`~/Development/AgenticOrchestration/Ravel` to the new `main`. Then it moves
the three directories into `universal-grammar/core/` there. Only after that
does it delete the old checkout. The pipeline keeps its default paths. The
snapshot's `.gitignore` ties `/inputs/`, `/data/` and `/reproductions/` to
its own directory, so the data stays out of every commit.

**The owner agrees that the understanding is shared.** The session summarised
the agreement and the tree changes. On 2026-11-26 the owner answered: "yes".
The summary held each decision above.

The summary also gave the later DesignSkills merge the same terms. Those are a snapshot in `packs/design/`, the deletion of
its local checkout and its archive once Ravel's copy is public. These terms are
the session's reading, and the owner's answer covers them.

**The placement of the new work.** Two leaves join the merge node as flat
siblings, because they share its purpose and its context. `product-layout-k34`
comes before the pack merge, and `merge-landing-k35` comes after the
UniversalGrammar merge. Neither needs a node of its own.

**No adversarial review of these terms.** This is the session's judgement, not
the owner's decision. The owner agreed to each decision live, in the owner's
own words. The leaves that carry out the terms test them against the
repositories and the root `check`.
