# skill-delivery-k9 — brief

## Goal

Make a skill one authored form that Ravel delivers to every harness. The Lua
configuration selects the applied and available skills for each kind. Ravel
delivers them through the prompt to every harness, and natively on Claude
Code, Codex and Gemini CLI.

## Done when

- A skill has one authoring form and one optional metadata form. Every skill
  identifier has a namespace, and no precedence rule settles equal names.
- The owner's skills live in `~/.config/ravel/skills`. Installed skills stay as
  aliases in `share/ravel/skills`.
- The skill directory inside `~/.config/ravel` works. No link or overlap
  exposes the configuration or a result channel.
- An available skill takes its description from its metadata, or else from its
  first paragraph. Inspection shows each description.
- A Lua script selects the applied and the available skills for each kind,
  including an empty selection. A missing selected skill fails with a clear
  refusal.
- Every harness receives skills through the prompt. Each gap has a documented
  explicit alternative or an actionable refusal.
- Adapters that Ravel owns deliver skills natively on Claude Code, Codex and
  Gemini CLI. They apply launch-time hooks, such as those of the writing rules.
- Static inspection and an evaluated preview show the route, the selected
  skills, the delivery mode, the limitations and the exact prompt. That prompt
  includes the unmodified caller task. Ordinary logs show no prompt body by
  default.
- The merged packs use the one form.
- Opt-in Taskfile tasks run a trial set on each supported harness. Their
  reports stay separate from the simulated tests. The real trials on Claude
  Code and Codex ran, with recorded results. The real Gemini CLI trial belongs
  to the root leaf `gemini-cli-trial`.

## Decomposition

The design comes first, because one skill form serves every later leaf. After
it, each leaf leaves the product working and adds one verifiable behaviour.

1. `skill-contract`: the design of the skill form, selection, delivery and
   inspection. The owner agrees its spec.
2. `skill-sources`: Ravel reads the one form from both skill directories, with
   namespaces, descriptions and the protected configuration directory. Static
   inspection shows the result.
3. `skill-selection`: the Lua configuration selects skills, and Ravel delivers
   them through the prompt to every harness. The evaluated preview shows the
   exact prompt. This is the smallest end-to-end path.
4. `claude-code-adapter`, `codex-adapter` and `gemini-cli-adapter`: native
   delivery and launch-time hooks, one supported harness each. The first two
   run their real trials. The Gemini CLI trial waits for a sign-in, so a root
   leaf runs it later.
5. `pack-form`: the merged packs move to the one form.

The leaves share the skill spec and the harness research. The Lua leaves need
neither, so this scope earns its own node.

## Pointers

- Decision records to rework: `skill-preparation-precedes-evaluation`,
  `skill-sources-are-local-directories`,
  `installed-skill-sources-are-the-default`.
- Also read `skill-resources-outlive-sessions` and
  `ravel-does-not-confine-a-session`.
- Spec: `docs/specs/skill-discovery.md`.
- Research: `docs/research/harness-delivery.md`, which the combined survey
  writes.
- Terms: **Skill source**, **Default skill directory**, **Skill identifier**,
  **Applied skill**, **Available skill**, **Supported harness**, **Original
  prompt**, **Prepared prompt**.
- Test seams: the command-line seam and the real harness trials, as the root
  brief states them.

## Notes

- Grove's own skills stay unconverted in this grove. DesignSkills stays
  unconverted until `design-skills-merge` converts it, late in the grove. Ravel
  still delivers both, so the form must accept them, or a documented
  compatibility reading must.
- Gemini CLI is not installed on the owner's machine. Its real trial needs the
  owner to install it and sign in first. So the root leaf `gemini-cli-trial`
  runs that trial, after the corpus review. A pending sign-in then blocks no
  work that does not use the result.
