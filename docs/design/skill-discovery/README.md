# Skill discovery views

Run `task docs:skills`, then open <http://127.0.0.1:8778/#discussion>.
If that port is in use, run `task docs:skills PORT=8884` and open
<http://127.0.0.1:8884/#discussion>. Read each graph from its dark node,
labelled `Start here`. Read each sequence from its first message. The sources
remain editable beside the viewer.

The [skill discovery specification](../../specs/skill-discovery.md) is the
authority for these views. They show the contract of the
`skill-directory-delivery` line, not the behaviour of Ravel `main`. The
[design evidence](../../skill-discovery-design-evidence.md) separates source
observations from proposed acceptance cases.

- [Default skill directory → Choose the source set](index.html#diagram-source-set) — Added the view: explicit sources replace the default, and the installation record names the directory.
- [Default skill directory → Homebrew fills the directory](index.html#diagram-homebrew-directory) — Added the view of the record, the links and the upgrade interval.
- [Preparation → Prepare before the worker launches](index.html#diagram-preparation) — The settings box now names the default skill directory.
- [Settings](../../specs/skill-discovery.md#settings) — Made `sources` and `mandatory` optional, and defined the source set.
- [Default skill directory](../../specs/skill-discovery.md#default-skill-directory) — Added the installation record, the entry table, the roots and the changes under running sessions.
- [Installed skill sources are the default](../../adr/installed-skill-sources-are-the-default.md) — Recorded the decision, its costs and the conditions that would reopen each rejected option.

The viewer loads Mermaid 12.0.0, so these views need network access. No
generated SVG files are part of these views. A rendered view proves diagram
delivery. It does not prove executable behaviour.
