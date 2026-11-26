# Ravel.gh-issue-6 — brief

## Goal

Make Ravel a universal entry point for sessions on subscription-billed
harnesses. A caller asks for a session kind, and the owner's Lua configuration
selects the harness, the model, the effort and the skills. A skill has one
form, and Ravel makes it work as identically as possible on every supported
harness. The owner's statement is Linkuistics/Ravel#6, which coordinates #4 and
#5.

This grove also runs the first stage of one Ravel product. It merges four skill
packs, UniversalGrammar and UniversalGrammar-Visualiser into the Ravel
repository, beside the launcher. DesignSkills follows when it is free.

## Done when

Each condition traces to a decision in the running log of `plan-k1`.

- **Lua configuration.** Ravel embeds a pinned standard Lua as its primary
  configuration language, and it needs no separate Lua installation.
- **Selection.** A Lua script selects the harness route, the applied skills
  and the available skills for each kind, including an empty selection. A
  missing selected skill fails with a clear refusal.
- **Rhai.** A 2.x release reads Lua and Rhai. Rhai keeps its 2.0 behaviour
  exactly and gets no new feature. A build without the Rhai module passes the
  Lua tests.
- **Skill form.** A skill has one authoring form and one optional metadata
  form. Every skill identifier has a namespace, and no precedence rule settles
  equal names. The owner's skills live in `~/.config/ravel/skills`, and
  installed skills stay as aliases in `share/ravel/skills`.
- **Descriptions.** An available skill takes its description from its
  metadata, or else from its first paragraph. Inspection shows each
  description.
- **Supported harnesses.** Adapters that Ravel owns make skills behave as
  identically as possible on Claude Code, Codex and Gemini CLI. That includes
  launch-time hooks, such as those of the writing rules.
- **Other harnesses.** Every other harness receives skills through the prompt.
  Each gap has a documented explicit alternative or an actionable refusal.
- **Inspection.** Static inspection and an evaluated preview show the route,
  the selected skills, the delivery mode, the limitations and the exact prompt.
  That prompt includes the unmodified caller task. Ordinary logs show no prompt
  body by default.
- **Protected paths.** The skill directory inside `~/.config/ravel` works. No
  link or overlap exposes the configuration or a result channel.
- **First merge stage.** The launcher is in `launcher/`, and the root belongs
  to the product. RequirementsSkills, EngineeringSkills, ResearchSkills,
  WritingSkills, UniversalGrammar and UniversalGrammar-Visualiser are in the
  Ravel repository, each as one snapshot of its trunk. Their checks pass there
  through the root `check`. The stage is on `main`, and their source
  repositories are archived. DesignSkills joins after Grove's leaf
  `relocate-decomposition-design-k1028` lands.
- **Corpus review.** The review covers the 29 skills of the four merged packs
  for readability. It records their intended semantics, and the owner decides
  each substantive change of policy. The original material and its provenance
  stay available.
- **Tests.** The acceptance tests run at the four agreed seams below. Reports
  of real harness trials stay separate from simulated tests.
- **Releases.** Each Ravel or Grove release that this work causes ships with
  migration notes for installed users.
- **Documentation.** The book, specs, decision records, examples, packaging and
  third-party notices match the delivered contract. A record states how the
  outcome relates to #4, #5 and #7.

## Decomposition

The requirements leaf `plan-k1` settled the requirements with the owner. The
planning leaf `unification-k2` cut the tree. The walk visits the entries in
this order, and each increment leaves the product working.

1. **First merge stage** (node `first-merge-stage`). The launcher moves into
   `launcher/`. Then the four packs and the two UniversalGrammar repositories
   arrive, and the stage lands on `main`. It needs nothing from the other work,
   and later leaves work on the merged packs.
2. **Harness research** (`harness-delivery`, a survey pair and its combiner).
   It answers the first research question for the prototype and the skill
   design. Two corpora pay off here, because the supported harnesses come from
   different vendors.
3. **Configuration prototype** (`configuration-experience`). The owner reacts
   to mock-ups of `config.lua`, a skill file and the inspection output. Both
   designs use that reaction.
4. **Lua routing** (node `lua-configuration`). The second research question,
   the routing design, its implementation and the parity checks. A user can
   adopt Lua for routing before any skill feature exists.
5. **Skill delivery** (node `skill-delivery`). The skill design comes first.
   Then come the skill sources and the selection with prompt delivery. One
   adapter per supported harness and the conversion of the packs follow.
6. **Corpus review** (node `corpus-review`). The owner settles the method, and
   a planning leaf then cuts the review leaves.
7. **DesignSkills** (`design-skills-merge`). It sits late, because it waits for
   Grove's leaf `relocate-decomposition-design-k1028`. A blocked leaf earlier
   would stop the walk.
8. **Pack shipping** (`pack-shipping`). The owner decides the formula move and
   the releases. The owner's direction is one Ravel package that ships every
   component.
9. **Gemini CLI trial** (`gemini-cli-trial`). The real trial waits for the
   owner to install Gemini CLI and sign in. It sits after the work that does
   not use its result, and before the book and the release, which do.
10. **Book and release** (`book`, `release`). The book follows its editorial
    pipeline, and the release ships migration notes.

A node holds an increment whose leaves share a subgoal, a joint completion
condition and context that the other leaves do not need. The research pair,
the prototype and the tail leaves serve several increments, so they stay flat
at the root. The design leaves cut their own review chains. The corpus review
waits for its method before its leaves exist.

The owner chose this grove for every increment above. So they stay in one
grove, although each could form a separate grove. A split needs the owner's
confirmation.

## Pointers

- Requirements statement: Linkuistics/Ravel#6, with #4 and #5. The running log
  of `plan-k1` holds each decision beside the owner's words.
- This work reworks four decision records:
  - `configuration-script-uses-rhai`
  - `skill-preparation-precedes-evaluation`
  - `skill-sources-are-local-directories`
  - `installed-skill-sources-are-the-default`
- Before any major release, read the decision record
  `a-caller-transition-publishes-a-versioned-formula-first`.
- Specs: `launcher/docs/specs/configuration-evaluation.md` and
  `launcher/docs/specs/skill-discovery.md`.
- Configuration terms in play: **Configuration script**, **Rhai worker**,
  **Original prompt** and **Prepared prompt**.
- Skill terms in play: **Skill source**, **Default skill directory**, **Skill
  identifier**, **Applied skill**, **Available skill** and **Supported
  harness**.
- Product term in play: **Launcher**.
- Related issues: Linkuistics/Ravel#7 holds the review of Grove's core and
  DesignSkills. Linkuistics/grove#28 holds the whole merge, and
  Linkuistics/grove#25 separated the skill repositories.

## Test seams

The owner agreed these four seams in `plan-k1`.

- **The command-line seam, as today.** Tests run the real `ravel` commands
  against a sandbox home, with fake harness programs on a private `PATH`. Fakes
  stand in for Claude Code, Codex and Gemini CLI, and one fake accepts only a
  prompt. Each fake records its arguments, environment, input and the files
  that it can read. Almost all acceptance tests run here.
- **Real harness trials.** Opt-in Taskfile tasks run a trial set on each
  supported harness, in a sandbox home, with the owner's sign-in.
- **The comparison of the owner's configuration.** A Taskfile task on the
  owner's machine runs the real `config.rhai` and its Lua translation through
  the command-line seam. It covers every configured kind and an unknown kind,
  and it writes its results outside the repository.
- **A build without Rhai.** A check builds Ravel without its Rhai module and
  runs the Lua tests at the command-line seam.

Three existing checks continue. They are the engine unit tests, the checks of
each merged pack, and a release smoke without a separate Lua installation.

## Out of scope

- The removal of Rhai. A later major removes it, together with a Grove release
  that accepts that major.
- The language review of Grove's core and of DesignSkills, in
  Linkuistics/Ravel#7.
- The merge of Grove and Ravel-Strategies, after their workstreams finish.
- Hooks on generic harnesses such as Pi, beyond what prompt delivery gives.
  Also the executable plugins for interface features that #4 names. Issue 4
  stays open for those.
- An override of an installed skill. No requirement for one exists. If one
  arrives, it is an explicit mechanism, never a precedence rule between equal
  names.
- PersonalSkills. It keeps its own repository and formula, and the corpus
  review leaves it out.

## Notes

- **Open workstreams.** No merge goes into or from a repository with an open
  workstream until that workstream finishes. Today that covers Grove and
  Ravel-Strategies. Among the packs, the open Grove workstream writes only
  into DesignSkills.
- **Product layout.** Ravel names the product, and the launcher is one of its
  components, with no privilege over the others. The launcher lives in
  `launcher/` and the packs in `packs/<name>/`. The UniversalGrammar
  repositories live in `universal-grammar/core/` and
  `universal-grammar/visualiser/`. The leaf `product-layout` writes the
  decision record.
- **Tools.** Every tool comes from mise, and no task relies on a globally
  installed program. Each component declares its tools in its own
  `mise.toml`, and the packs share `packs/mise.toml`. The root `check` runs the
  check of each component in that component's directory.
- **The Visualiser.** Its work is abandoned. On 2026-11-26 one operation
  forgot its `gh-issue-1` workspace, and four operations abandoned its
  unmerged changes. Its snapshot brings today's files only, so its private
  history stays private. The deletion of its local checkout removes the
  abandoned changes.
- **The DesignSkills archive.** Once `main` holds DesignSkills, its source
  repository gets a README pointer to `packs/design/`. Then the leaf that
  lands it archives the repository on GitHub.
- **Modular configuration.** The later removal of Rhai removes the Rhai module
  and its tests. Apart from that, it changes only the one place that registers
  the language.
- **Fixtures.** Never publish the owner's configuration or secrets as test
  fixtures.
- **Gemini CLI.** The owner's machine has no Gemini CLI yet. Its trials need an
  installation and a sign-in first.
- **Research to commission.** Two questions need answers before the design.
  First, how far can one generic prompt delivery reach across harnesses, and
  what can one launch configure on each supported harness? Second, which Lua
  runtime and bindings should Ravel embed? Issue 6 names Lua 5.4 and `mlua` as
  candidates, not choices. That research covers limits, native calls and worker
  termination, and it does not assume that an embedded runtime is a sandbox.
- **Design questions.** The design settles how the owner's own skills get
  their namespace, and how a bare name in a Grove prompt resolves. It settles
  the metadata syntax and how a skill declares a hook. It settles the rule when
  `config.lua` and `config.rhai` both exist, and the protection of the
  configuration directory. It settles how applied and available skills reach a
  session, with prompt size, compaction, delegation and stable content for
  continuations. Its spec proposes the inspection commands, and the owner
  agrees them there.
- **Owner decisions still open.** A requirements leaf settles the method of
  the corpus review comparisons with the owner, just before the review. The
  owner also decides when the shipping of the packs moves from Grove's formula
  to Ravel's formula. That decision covers how the Grove release relates to the
  open Grove workstream.
