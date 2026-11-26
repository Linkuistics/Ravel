# Layout in the core

The Rust core computes the position of every box and every edge, and writes the coordinates into the diagram KDL. D3 in the browser only draws. A box takes its size from its children, so an edit inside one part of a rule never moves the order of the rest. Text widths come from one bundled monospace font, by the record `bundled-monospace-font-for-diagram-labels`. The spec `rule-diagrams` gives the method and its constants. People cannot drag boxes, because a dragged position would conflict with the derived one.

## Trade-off

The core needs its own layout code and font metrics, and people lose manual placement. In return, every host gets the same coordinates. Tests compare text snapshots of coordinates, not screenshots. Playwright warns that "Browser rendering can vary based on the host OS, version, settings, hardware" (https://playwright.dev/docs/test-snapshots).

## Rejected alternative

elkjs or dagre in the browser. elkjs is 8.2 MB unpacked at version 0.12.1, and its licence changed to "EPL-2.0 OR GPL-3.0-or-later" at version 0.12.0 (https://github.com/kieler/elkjs). A layered engine helps only with minimised automata, which wait until after the slice.

Source: proposals that the human agreed on 2026-11-18, recorded in `docs/milestones/grammar-workbench.md`, and the layout-interaction survey.
