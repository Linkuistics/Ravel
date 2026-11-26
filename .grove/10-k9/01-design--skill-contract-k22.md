# skill-contract-k22

## Goal

Design the one skill form and its delivery. Rework the spec `skill-discovery`
and the three skill decision records that the node brief names. The owner
agrees the spec, including its inspection commands.

## Context

- The combined survey `docs/research/harness-delivery.md`.
- The owner's reactions in the running log of `configuration-experience-k7`.
- The Lua routing contract that `configuration-contract-k19` settled. Skill
  selection extends it.
- The running log of `plan-k1`, Topics 2, 4, 5 and 6.
- The merged packs in the Ravel repository show the forms in use today,
  including the hooks of `technical-communication`.

## Done when

The spec and the decision records settle each design question below.

- The authoring form, the optional metadata form and its syntax, and the
  supporting resources of a skill.
- How a skill declares a hook.
- The namespace of every skill identifier, including the owner's own skills.
  No precedence rule settles equal names.
- How a bare name in a Grove prompt resolves to a namespaced identifier.
- The description of an available skill: its metadata, or else its first
  paragraph.
- The Lua selection of applied and available skills for each kind: defaults,
  order, an empty selection and the refusal for a missing or unreadable skill.
- The reorder of the boundary: Lua chooses skills before Ravel builds their
  delivery. The original and the prepared prompts stay distinct.
- The protection of the configuration directory, which now holds the skill
  directory. No link or overlap exposes the configuration or a result channel.
- Delivery through the prompt to every harness, and native delivery on each
  supported harness. Both cover prompt size, file access and native name
  collisions. Both also cover compaction, delegation and stable content for
  continuations.
- The documented gap per harness, with its explicit alternative or actionable
  refusal.
- The inspection commands: static inspection and evaluated preview. Ordinary
  logs show no prompt body by default.
- How existing users keep their skills during the transition. Grove's skills
  and DesignSkills stay in their current form in this grove.

## Notes

- The spec is an agreement point. Record the owner's restatement in the
  running log.
- Prefer a bare-name resolution that needs no change in Grove. Grove has an
  open workstream. If a Grove change is unavoidable, say so, because it causes
  a Grove release.
- If the design is too large for one session, decompose it. A natural split is
  the skill form first, then delivery and inspection.
- The spec is load-bearing, and `plan-k1` judged that it earns an independent
  challenge. Cut a `review-design` sibling as the last act.
- Do not cut implementation leaves. The node already holds them. If the spec
  changes their slicing, cut a `planning` leaf instead.
