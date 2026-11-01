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

- [Preparation → Prepare before the worker launches](index.html#diagram-preparation) — Replaced the policy worker with the Ravel CLI and the Rhai worker, and moved resolution before the worker launch.
- [Prompt bounds and frames → Where the prompt travels and which bound applies](index.html#diagram-frames) — Added the prompt bound, the derived evaluate-frame bound and the script-result bound.
- [Lifetime → Keep the reader's directories](index.html#diagram-lifetime) — Renamed the resolver to the Ravel CLI, with no change to the recovery rules.
- [Permissions → Read resources and state the authority limit](index.html#diagram-permissions) — Added harness runs, which receive the same read grants and variable as the managed harness.
- [Request and configuration script](../../specs/skill-discovery.md#request-and-configuration-script) — Defined schema 4, the copy of one request for each phase and ordinary measured descriptor reads.
- [Frame bounds](../../specs/configuration-evaluation.md#frame-bounds) — Derived the evaluate-frame bound and turned an oversized frame into a refusal of the caller input.

The viewer loads Mermaid 12.0.0, so these views need network access. No
generated SVG files are part of these views. A rendered view proves diagram
delivery. It does not prove executable behaviour.
