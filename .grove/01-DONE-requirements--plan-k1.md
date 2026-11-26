# plan-k1

## Goal

Establish what this grove builds, in the owner's own words. The owner's
statement is GitHub issue Linkuistics/Ravel#6, "Unify Lua configuration and
portable, selectively applied skills". Settle its open requirement questions
with the owner, one at a time, and agree the test seams. Then record the
result in the root brief, and add the leaves that the work needs next.

## Context

- Linkuistics/Ravel#6 is the requirements statement of this grove. It
  coordinates #4 and #5.
- Linkuistics/Ravel#5 replaces Rhai with Lua. It puts repository consolidation
  before the Lua migration.
- Linkuistics/Ravel#4 makes Ravel a universal skill provider that configures
  each harness.
- Linkuistics/grove#28 merges Ravel, the skill packs, Grove and
  Ravel-Strategies into one repository. It starts after Linkuistics/grove#25
  completes.
- Linkuistics/grove#25 separates the skill repositories. It is still open,
  although its pack repositories exist and Ravel 2.0.0 is released.
- Issue 6 reopens these decision records: `configuration-script-uses-rhai`,
  `skill-preparation-precedes-evaluation`, `skill-sources-are-local-directories`
  and `installed-skill-sources-are-the-default`. A new Ravel major also meets
  `a-caller-transition-publishes-a-versioned-formula-first`.
- The specs in the area are `docs/specs/configuration-evaluation.md` and
  `docs/specs/skill-discovery.md`.

## Done when

- The owner agrees that the session reached a shared understanding of the
  requirements.
- The running log below holds each settled decision. At each agreement point,
  it also holds the owner's restatement.
- The root brief states the goal, the done criteria and the pointers.
- The tree holds the next leaves. A `planning` leaf decomposes the work if it
  is too large to plan in this session.

## Notes

- The grilling procedure applies, because at least three open questions have
  interdependent answers.
- Never publish the owner's configuration or secrets as test fixtures.

## Decisions (running log)

**Topic 1, the order against repository consolidation.** On 2026-11-26 the
owner chose option A. This grove does the requirements, design, prototype and
implementation of the work in the Ravel repository now. The skill corpus
review, step 7 of issue 6, waits for the repository merge, because it edits the
skill packs. This order departs on purpose from issue 5, which put the merge
before the Lua migration. The Ravel work moves with its history, but corpus
edits before the merge would land in up to seven repositories with their pins.

**External users need migration notes.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> we now have at least one external user, so we will need migration notes for
> a new grove and ravel release.
<!-- tc: on -->

The premise of the clean cutover in 2.0.0 no longer holds. That premise was
that all known configuration scripts were the owner's. Each release that this
work causes, of Ravel or of Grove, carries migration notes.

**The merged repository is a unified product.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I want to merge Ravel, Ravel-Strategies, Grove, and the Skill repos into one
> mono-repo, as a unified product, whilst maintaining the ability for the use
> of Ravel in particular as an independent tool, and for users to manage their
> skills so that they can author them one and have them applied over arbitrary
> harnesses routed via Ravel.
<!-- tc: on -->

"Author them one" means "author them once". Two requirements bind this grove
too. Ravel stays usable as an independent tool, without Grove. A user authors
a skill once, and Ravel delivers it to every harness that Ravel routes. The
place of the merge itself is still open: this grove, or a grove of its own.

**Merge stages, open workstreams and the product name.** On 2026-11-26 the
owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> We have workstreams open in grove and Ravel-Strategies, and I don't want to
> merge into or from those until those are finished. The grove workstream in
> particular has been going for months. I forgot about UniversalGrammar and
> UniversalGrammar-Visualiser, which also need to be merged, the later of which
> is still in progress. Maybe Ravel and the Skills repos can be merged first
> (the product name will in the end be Ravel).
<!-- tc: on -->

"The later of which" means UniversalGrammar-Visualiser. So the merge comes in
stages. No stage merges into or from a repository with an open workstream
until that workstream finishes. Today Grove and Ravel-Strategies have open
workstreams, and UniversalGrammar-Visualiser is still in progress.

The merged repository also takes UniversalGrammar and
UniversalGrammar-Visualiser, and the product name is Ravel. The owner proposes
a first stage of Ravel and the six skill packs. Whether this grove carries
that stage is open.

**This grove carries the first merge stage.** On 2026-11-26 the owner chose
this grove for the first stage. The five packs without an open writer come into
the Ravel repository: RequirementsSkills, EngineeringSkills, ResearchSkills,
WritingSkills and PersonalSkills. DesignSkills follows when the Grove leaf
`relocate-decomposition-design-k1028` lands, because that leaf writes into
DesignSkills. This grove cannot finish before then, unless the owner moves
DesignSkills to a later stage. The corpus review runs in this grove after the
packs arrive.

**Installed skills keep their location.** On 2026-11-26 the owner wrote: "I
think share/ravel/skills can remain the location of the skills/symlinks." The
packaged skills stay as aliases in `share/ravel/skills`, the default skill
directory of the installation, whichever formula ships them.

**The UniversalGrammar repositories can probably join the first stage.** On
2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I might abandon the UniversalGrammar-Visualiser work, so those two UG repos
> can probably me merged as well.
<!-- tc: on -->

"Probably me merged" means "probably be merged". If the owner abandons the
Visualiser workstream, UniversalGrammar and UniversalGrammar-Visualiser join
the first stage in this grove. Until then, the rule about open workstreams
applies to them.

UniversalGrammar-Visualiser is a private repository, and the Ravel repository
is public. A merge into Ravel would publish the whole Visualiser history. The
merge stage therefore settles that question with the owner before any push.
UniversalGrammar is public, under the owner's personal account.
UniversalGrammar-ChatGPT is a folder of resources, not a repository, so the
statement does not cover it.

**Topic 2: one skill form, which Ravel makes work on every supported
harness.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I really want to have a single skill authoring and metadata form that Ravel
> is responsible for making work as identically as possible across all
> supported harnesses.

> Skill unification is a major feature of Ravel, because it's raison d'etre is
> abstraction over subscription-billed harnesses
<!-- tc: on -->

This rejects guidance only as the end state. It meets the bar for a decision
record, and the owner's words above are the restatement. The session reads the
decision as follows, and the owner's final agreement in this session covers
the reading:

- A skill has one authoring form and one optional metadata form. Its author
  writes no manifest for any harness.
- Ravel adapts that form to each supported harness, and Ravel owns that
  machinery. For skill delivery, Ravel is no longer neutral about harnesses, as
  it was in 2.0.0.
- The form also carries what a skill needs beyond its text. So the hooks of the
  writing rules move into the form, and Ravel applies them on each supported
  harness.
- "As identically as possible" admits a documented gap where a harness lacks a
  capability. There, issue 6 requires an explicit alternative or an actionable
  refusal.
- The set of supported harnesses is still open.

**Topic 2: the supported harnesses.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> At least Claude Code, Codex and Gemini CLI, because they are ones with
> subscription-billing models specific to the harness. Pi, Hermes etc are
> somewhat more generic. It might be that ravel can deliver the skills via the
> prompt for every harness, or maybe most harnesses in a generic fashion - we
> need to research that.
<!-- tc: on -->

The supported harnesses are at least Claude Code, Codex and Gemini CLI. The
criterion is a subscription-billing model specific to the harness. Generic
harnesses, such as Pi and Hermes, are outside that set.

A research question follows for the plan. Can Ravel deliver skills through the
prompt, in one generic way, to every harness or to most of them? Its answer
comes before the adapter design, because it decides how much each adapter must
do. It also decides whether generic delivery satisfies issue 4 on Pi. Gemini
CLI is not installed on this machine, so its real trials need an installation
and a sign-in first.

**Topic 3: Rhai coexists in 2.x, and a later major removes it.** On 2026-11-26
the owner accepted the recommendation and wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I agree, but let's make sure that the configuration mechanism is modular so
> that it's easy to remove Rhai and the code doesn't entangle.
<!-- tc: on -->

- A 2.x release reads both Lua and Rhai. Lua is the primary language.
- Rhai keeps its 2.0 behaviour exactly. The new skill features need Lua.
- A later major removes Rhai. It ships together with a Grove release that
  accepts that major.
- The configuration mechanism is modular, and Rhai code does not entangle with
  Lua code or with the rest of Ravel. The later change removes the Rhai module
  and its tests. Apart from that, it changes only the one place that registers
  the language.

This decision meets the bar for a decision record, and it reworks
`configuration-script-uses-rhai`. The owner's words above are the restatement.

**Topic 4: every skill identifier is namespaced.** On 2026-11-26 the owner
rejected the recommendation, a precedence rule for repeated names, and wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> Namespaced identifiers is a universal design rule I follow. The fact that two
> names are the same is always IME due to lack of context or scoping. Scoping
> (namespaces) is a universal strategy.
> Overriding is a separate issue and using name-collision precedence to
> implement that is *always* a design error.
<!-- tc: on -->

- Every skill identifier is namespaced, the owner's own skills included. Two
  skills never share an identifier, so Ravel needs no precedence rule between
  colliding names.
- The option proposed the source as the namespace of an installed skill. The
  design settles that form, and it decides how a skill in the owner's
  directory gets its namespace.
- An override of an installed skill is a separate concern. If Ravel ever offers
  one, it is an explicit mechanism, never a precedence rule between colliding
  names. No requirement for an override exists yet.
- Grove's prompts name a skill by its bare name today. The design decides how a
  bare name resolves to a namespaced identifier.

**Topic 5: the description of an available skill.** On 2026-11-26 the owner
accepted the recommendation: "I agree". A description in the skill's metadata
tells the agent when to read an available skill. Without one, the first
paragraph of the skill serves as its description. A standalone file therefore
needs no metadata. Inspection shows the description of each available skill,
so that its author sees what the agent sees.

**Topic 6: the inspection commands.** Issue 6 already states what inspection
must show. The design session proposes the commands in its spec, and the owner
agrees them there.

**Topic 7: the scope of the corpus review.** On 2026-11-26 the owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> I agree with your recommendation. Let's capture the grove-core and
> design-skill language review in a GH issue.
<!-- tc: on -->

- This grove's review edits 32 skills. They belong to the five packs that merge
  without an open writer: requirements, engineering, research, writing and
  personal.
- The language review of Grove's core and of the DesignSkills pack moves to
  GitHub issue Linkuistics/Ravel#7. Both sources depend on the open Grove
  workstream. This narrows the recommendation, which included DesignSkills.
- The review only reads TestAnyware and the two third-party sources,
  superpowers and Quint. It records its findings for a later stage.
- A requirements leaf settles the method of the comparisons with the owner,
  just before the review starts.

**Topic 8: the test seams.** On 2026-11-26 the owner agreed four seams: "yes".

- The command-line seam, as today. Tests run the real `ravel` commands against
  a sandbox home, with fake harness programs on a private `PATH`. This grove
  adds fakes for Claude Code, Codex and Gemini CLI, and one fake that accepts
  only a prompt. Each fake records its arguments, environment, input and the
  files that it can read. Almost all acceptance tests run here.
- Real harness trials, extended from 2.0.0. Opt-in Taskfile tasks run a trial
  set on each supported harness, in a sandbox home, with the owner's sign-in.
  Their reports stay separate from the simulated tests.
- A comparison of the owner's configuration, on the owner's machine only. A
  Taskfile task runs the real `config.rhai` and its Lua translation through the
  command-line seam, for every configured kind and an unknown kind. It writes
  its results outside the repository.
- A build without Rhai. A check builds Ravel without its Rhai module and runs
  the Lua tests at the command-line seam.

Three existing checks continue. The engine unit tests supplement the seams. The
checks of each merged pack must pass in the Ravel repository. A release smoke
shows that the executable needs no separate Lua installation.

**The UniversalGrammar repositories join the first stage.** On 2026-11-26 the
owner wrote:

<!-- tc: off because the owner's words are quoted verbatim -->
> The two non ChatGPT UniversalGrammar repos can be merged here - they have no
> active work
<!-- tc: on -->

UniversalGrammar and UniversalGrammar-Visualiser therefore join the first
merge stage in this grove, with no condition. UniversalGrammar-ChatGPT is not
a repository, and it is not part of the merge.

The Visualiser workspace `UniversalGrammar-Visualiser.gh-issue-1` still holds a
live grove tree. The repository also holds 12 unmerged changes from that
workstream, and no remote bookmark tracks them, so they exist only on the
owner's machine. The merge stage asks the owner whether those changes come
across. It also settles the publication
of the private Visualiser history before any push.

**The Visualiser's changes are garbage.** On 2026-11-26 the owner wrote: "The
Visualiser work was abandoned. jj needs to gc those commits." So the 12
unmerged changes do not come across. The merge stage abandons them in jj and
collects the garbage before the merge, and it retires the `gh-issue-1`
workspace. This session leaves that repository untouched, because the removal
cannot be undone and lies outside this leaf.

**The owner agrees that the understanding is shared.** On 2026-11-26 the owner
wrote: "Your plan matches mine." The summary that the owner accepted is the set
of decisions above, with the two UniversalGrammar repositories in the first
stage.

**No adversarial review of these requirements.** This is the session's
judgement, not the owner's decision. The owner agreed to each decision live, in
the owner's own words. The design spec turns these decisions into contracts, so
the spec is the artifact that earns an independent challenge.
