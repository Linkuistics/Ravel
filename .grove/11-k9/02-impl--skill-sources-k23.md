# skill-sources-k23

## Goal

Read skills in the one form from `~/.config/ravel/skills` and from
`share/ravel/skills`, as the spec `skill-discovery` states it. Static
inspection lists each skill with its namespaced identifier and description.

## Context

- The reworked spec `launcher/docs/specs/skill-discovery.md` and its decision records.

## Done when

- A standalone Markdown file in the owner's skill directory is a skill, with
  no manifest and no per-harness installation.
- The optional metadata form is read, and its errors name the file and the
  field.
- Every identifier has its namespace. Two sources never yield one identifier.
- Each available skill has a description from its metadata or its first
  paragraph.
- The skill directory inside `~/.config/ravel` works. Tests show that no link
  or overlapping path exposes the configuration or a result channel.
- The existing installed skills, in their current form, still work as the
  spec's transition states.
- Static inspection shows each skill, its identifier, its source and its
  description. Tests at the command-line seam cover it.
