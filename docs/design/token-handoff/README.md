# Token handoff views

Run `task docs:token-handoff`, then open <http://127.0.0.1:8779/#discussion>.
If that port is in use, run `task docs:token-handoff PORT=8885` and open
<http://127.0.0.1:8885/#discussion>. Read each graph from its dark node,
labelled `Start here`. Read the sequence from its first message. The sources
remain editable beside the viewer.

The [token handoff specification](../../specs/token-handoff.md) is the
authority for these views. They show the contract of the
`skill-directory-delivery` line, not the behaviour of Ravel `main`.

- [Solution → From the owner's token to the confined Claude Code](index.html#diagram-route) — Added the skill-source protection, the runner's descriptor list and the closed descriptor.
- [Checks and the read → Check, read and send in one confined launch](index.html#diagram-launch) — Added the protected paths, the open-file checks and the runner's list.
- [Delivery through a socket → Claude Code reads a socket directly and a pipe by its path](index.html#diagram-reader) — Corrected the direct read, which closes the descriptor.
- [Rejected routes → Four rejected routes and the agreed route](index.html#diagram-rejected) — Added the routes that the decision record rejects.
- [A confined harness receives an owner-minted token](../../adr/a-confined-harness-receives-an-owner-minted-token.md) — Recorded the decision and the rejected routes.

The viewer loads Mermaid 12.0.0, so these views need network access. No
generated SVG files are part of these views. A rendered view proves diagram
delivery. It does not prove executable behaviour.
