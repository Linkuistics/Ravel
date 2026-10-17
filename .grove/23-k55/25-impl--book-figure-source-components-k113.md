# book-figure-source-components-k113

## Goal

Port the C4 component view and report what its hand-placed layout cost, so the
owner can see the effort the remaining unproven views are likely to take.

## Context

- Figure: `source-components`. Size: 3 entities and 1 external system, 3
  relations, one dashed boundary around two components, and an 8-line key. The
  `Lay_D` hint kept the worker above the managed process in PlantUML.
- It reuses `_c4.typ`. The new feature is a boundary that encloses some entities
  and not others, with a stereotype line; if `_c4.typ` does not support it, extend
  the module.
- Preserve: the Ravel CLI's checking code and the linked runner share one
  process, the worker is separate, the boundary means process membership not
  confinement, no third runner executable is installed.

## Done when

- `source-components.puml` is replaced by a committed `.typ` and regenerated SVG.
- The decision log records the layout cost as numbers: entities, relations placed
  by hand, label moves, renders needed to settle, minutes of work, and anything
  that was clumsy. Compare with the estimate in `book-figure-migration-k108`'s
  cost table and say whether the table was right, so the owner can judge the
  rest before they are spent.
- Facts and key lines are preserved; differences are listed.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
