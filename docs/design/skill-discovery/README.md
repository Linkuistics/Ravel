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

- [Default skill directory → Homebrew fills the directory](index.html#diagram-homebrew-directory) — The keg now holds a private payload and a source alias. Another keg leaves the alias one entry.
- [Packaging a default source](../../specs/skill-discovery.md#packaging-a-default-source) — Added the packaging rule and why a real source directory under `share` fails.
- [Packaging record of a default source](../../skill-discovery-design-evidence.md#packaging-record-of-a-default-source) — Recorded the Homebrew source reading and the measurement behind the rule.

The viewer loads Mermaid 12.0.0, so these views need network access. No
generated SVG files are part of these views. A rendered view proves diagram
delivery. It does not prove executable behaviour.
