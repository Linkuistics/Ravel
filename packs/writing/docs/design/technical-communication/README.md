# technical-communication — design views

The [spec](../../specs/technical-communication.md) is the contract for the
always-on `technical-communication` skill. These views show its layers, its
four events, the precedence order with its markers, and lazy conversion. They
also show the switch, the reply log and the layers for each harness. When a
view and the spec disagree, the spec wins, and the view is fixed.

Run `task design:technical-communication` from the repository root, then open
[the viewer](http://127.0.0.1:8770/). The server serves only this directory, and
`PORT=...` chooses another port. The `.d2` files and `diagrams.json` are the
editable sources. The `.svg` files are exports from D2 0.9.0 with the ELK
layout. After you edit a source, run `task design:technical-communication:render`
to export the views again. The viewer needs no network access.

Each graph view starts at a dark **Start here** node, and numbered edge labels
give the order of the steps. The sequence view starts at its first message.

| View | What it shows |
|---|---|
| [Layers and where they live](http://127.0.0.1:8770/#diagram-layers) | One skill directory, thin entries for each harness, the hook core, the checker and the state directory |
| [The four events](http://127.0.0.1:8770/#diagram-events) | What each event sends back to the harness, and why a reply's hook prints nothing |
| [Precedence and its markers](http://127.0.0.1:8770/#diagram-precedence) | The order, and the marker that records each departure from it |
| [Lazy conversion](http://127.0.0.1:8770/#diagram-conversion) | The never-convert classes, the large-document fallback and the `jj split` step |
| [What the off file reaches](http://127.0.0.1:8770/#diagram-switch) | The layers that stop at their next event, and the text that no hook can reach |
| [Reply log records](http://127.0.0.1:8770/#diagram-log) | One file for each session, with start and reply records and no reply text |
| [Layers per harness](http://127.0.0.1:8770/#diagram-harness-map) | The layers that Claude Code, Codex and Pi get, and Gemini CLI's skill-only scope |
