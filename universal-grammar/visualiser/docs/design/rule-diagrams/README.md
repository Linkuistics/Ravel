# Rule diagrams: design presentation

This directory holds the visual design of the spec `rule-diagrams`. The spec holds the decisions, and these views show them.

- `index.html` is the static viewer of the Grove design skill. `diagrams.json` orders its topics and holds the current discussion.
- `mockups.py` parses the example grammars, lays out each rule by the method of the spec, and draws each `mock-*.svg` file. It also writes `setup.diagram.kdl`, the example of the diagram KDL in the spec. Edit the script, and then draw the files again.
- `grove-sessions.ebnf` is a guessed grammar of the kinds of grove sessions. It shows how a workflow grammar would look, and it is not a decision.
- Each `.mmd` file is a Mermaid diagram. The viewer draws it in the browser with Mermaid 12.0.0 from jsDelivr, so it needs network access.

The script is a sketch of the layout for the discussion. The layout of the core is the work of `diagram-layout-k13`, and its tests compare text snapshots of coordinates.

Run the tasks of the repository Taskfile:

- `task design:mockups DESIGN=rule-diagrams` draws the SVG files again.
- `task design:serve DESIGN=rule-diagrams` serves this directory at `http://127.0.0.1:8765/`. Set `PORT` to use another port.

The mock-ups name IBM Plex Mono first, and they fall back to the monospace font of the system. They follow the light or dark scheme of the reader.

## Reading order

In a mock-up, follow the numbered circles and the notes beside them. In a graph, start at the dark node whose label starts with "Start here".
