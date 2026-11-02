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

- [Preparation → Prepare before the worker launches](index.html#diagram-preparation) — Removed Git acquisition, so the Ravel CLI only captures local bytes.
- [Lifetime → Keep the reader's directories](index.html#diagram-lifetime) — Capture now reads the local tree, and the owner edits the source or its checkout instead of a pin.
- [Settings](../../specs/skill-discovery.md#settings) — Limited `type` to `local`, and stated the `settings_invalid` refusal of every other type.
- [Sources outside Ravel](../../specs/skill-discovery.md#sources-outside-ravel) — Replaced exact Git acquisition and offline reuse with the capture of a checkout that a tool outside Ravel installs.
- [Skill sources are local directories](../../adr/skill-sources-are-local-directories.md) — Recorded the decision, the withdrawn Git acquisition and the conditions that would reopen it.

The viewer loads Mermaid 12.0.0, so these views need network access. No
generated SVG files are part of these views. A rendered view proves diagram
delivery. It does not prove executable behaviour.
