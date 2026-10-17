# book-figure-installed-files-k114

## Goal

Port the UML deployment view, which needs nested directory enclosures that no
earlier figure has, and report its layout cost.

## Context

- Figure: `installed-files`. Host enclosure around `PREFIX`, which holds
  `bin/ravel` and a `libexec/ravel` directory of one executable and three file
  groups; one dashed discovery arrow; the PlantUML source stacks the file groups
  with hidden links. Directory tabs and outlines must not cross labels (a past
  art pass fixed a label that crowded an outline).
- New engine code: `_deployment.typ` (nested enclosure with tab, executable and
  file-group artifacts, dashed dependency). Keep it small; nothing else in the
  book uses it.
- Preserve: keep the CLI and worker from one build together; outlines group
  directories; a file group lists separate files relative to `libexec/ravel`;
  editing adjacent sources does not replace embedded modules; archive-only
  README and licence remain identified.

## Done when

- `installed-files.puml` is replaced by a committed `.typ` and regenerated SVG,
  with the key and file-path text unchanged except where a difference is listed.
- The decision log records the layout cost as numbers (see
  `book-figure-source-components`) and whether the cost table in
  `book-figure-migration-k108` held.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
