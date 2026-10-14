# Component-contract views

Read the [component guide](../../components.md) first for the textual obligation
table, responsibility contracts, relation IDs and evidence. These views explain
the current implementation; they select no target architecture. Human usefulness
remains unassessed until the practical reader checkpoint.

Run `task docs:components`, then open
[the current question](http://127.0.0.1:8773/#discussion).
The server exposes only this presentation directory. Use `PORT=...` to choose a
different local port. The viewer renders editable `.mmd` sources using pinned
Mermaid 12.0.0 from jsDelivr, so rendering requires network access. The Markdown
guide and source files remain usable without that viewer. No generated SVG is
checked in; the renderer positions the nodes on each page load.
Sequence sources set compact actor spacing and wrapping using Mermaid's
[documented sequence configuration](https://mermaid.js.org/config/schema-docs/config-defs-sequence-diagram-config.html),
then verify the actual pinned renderer's output in the browser.

Graph views begin at a dark, bold node labelled “Start here”. Sequence views
begin at their first message. Package, process and logical responsibility labels
are explicit; an arrow's label gives its meaning rather than proximity/colour.
R1–R8 and O1–O10 refer to the guide. The captions state represented scope,
omissions and evidence limits. `diagrams.json` supplies topic order and stable
view IDs. Keep those IDs stable when revising sources and captions together.

The static viewer is copied from `grove-design/assets/diagram-viewer.html`.
Its narrow-screen CSS is adapted to keep diagram labels readable through
horizontal panning rather than shrinking a whole sequence into a phone width.
The presentation is checked by opening the served page and a fresh deep link,
inspecting rendered layouts and checking the depicted relations against the
guide. HTTP success alone is not rendering or source conformance evidence.
Desktop and a 390-pixel narrow viewport were inspected in Chrome; the current
light appearance was checked. Dark appearance follows the browser's media
preference but was not independently inspected. The book's final viewing
environment remains part of the human structure agreement.

Current additions:

- [Packages and processes → Current composition](http://127.0.0.1:8773/#diagram-composition) — Added package/process labels and typed authority/data relations.
- [Session lifecycle → Successful result](http://127.0.0.1:8773/#diagram-success) — Added worker wait and cleanup before exact result stdout.
- [Session lifecycle → Cancellation](http://127.0.0.1:8773/#diagram-cancellation) — Added selection/managed phases and the separate publication-before-cancellation observation.
