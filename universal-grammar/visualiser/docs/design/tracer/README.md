# Tracer: design presentation

This directory holds the visual design of the spec `tracer`. The spec holds the decisions, and these views show them.

- `index.html` is the static viewer of the Grove design skill. `diagrams.json` orders its topics and holds the current discussion.
- Each `.mmd` file is a Mermaid diagram. The viewer draws it in the browser with Mermaid 12.0.0 from jsDelivr, so it needs network access.

Run `task design:serve DESIGN=tracer` to serve this directory at `http://127.0.0.1:8765/`. Set `PORT` to use another port.

## Reading order

In a graph, start at the dark node whose label starts with "Start here". A sequence diagram reads from its first message.
