# Output view: design presentation

This directory holds the visual design of the spec `output-view`. The spec holds the decisions, and these views show them.

- `index.html` is the static viewer of the Grove design skill. `diagrams.json` orders its topics and holds the current discussion.
- `mockups.py` draws each `mock-*.svg` file from a list of rows. Edit the rows, and then draw the files again.
- Each `.mmd` file is a Mermaid diagram. The viewer draws it in the browser with Mermaid 12.0.0 from jsDelivr, so it needs network access.

Run the tasks of the repository Taskfile:

- `task design:mockups` draws the SVG files again.
- `task design:serve` serves this directory at `http://127.0.0.1:8765/`. Set `PORT` to use another port.

The mock-ups use system fonts. They follow the light or dark scheme of the reader.

## Reading order

In a mock-up, follow the numbered circles and the notes beside them. In a graph, start at the dark node whose label starts with "Start here".

A grammar-view part of a mock-up is a sketch. The rule-diagram design sets the drawing of the grammar view.
