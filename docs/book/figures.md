# Figures: layout, renderers and regeneration

This note settles how a figure sits on disk and how it is regenerated, so the
art stage and every drafting session use one layout. The contract
(`contract.md`, "Format and media") owns *which* notations and renderers the
book uses; this note owns the mechanics. It sits beside the contract and is not
part of the published book.

## Layout on disk

```text
docs/book/src/figures/
  system-context.puml      editable source (PlantUML)
  system-context.svg       what the source renders to, committed
  session-flow.d2          editable source (d2)
  session-flow.svg
  _style.puml              shared include, never rendered on its own
```

- One diagram per source file, flat in `docs/book/src/figures/`. The SVG has the
  source's name with `.svg`. A `.puml` and a `.d2` with one name are refused.
- Name a figure for its subject, not its chapter: several chapters reuse one
  figure (the contract's figure list puts the system context in chapters 1 and 2
  and the container view in 2 and 7).
- A file whose name starts with `_` is a shared include (`!include _style.puml`
  in PlantUML, `...@_style` in d2). It is not a figure and gets no SVG. This is
  where the art stage's visual vocabulary lives once it exists.
- The directory holds `.puml`, `.d2` and `.svg` files only. Anything else, a
  subdirectory, or an SVG with no source is refused, so the compare covers every
  file there.
- A figure made only of the page's own text, such as a table, needs no file.

## How a page uses a figure

```markdown
![A caller starts a session; Ravel launches the agent program](figures/system-context.svg)
```

The path is relative to the chapter, which sits in `docs/book/src/`, so it
resolves in the source tree (GitHub's file view, an editor preview) and in the
mdBook output alike. The alt text is written in the page, not in the SVG, and
says what the figure shows. mdBook copies every non-Markdown file in `src/`
into the built book, so the editable sources are published beside the SVGs.

## What is recorded, and where

| Fact | Recorded in | Kept honest by |
|---|---|---|
| Renderer version | `scripts/figures.sh` (one constant and one digest per tool); the contract names the versions | each SVG carries the renderer's own stamp (`<?plantuml 1.2026.8?>`, `data-d2-version="v0.9.0"`), which `task docs:figures` checks on the fresh and the committed file |
| Layout settings | the source itself | the lint below refuses a source that does not declare them |
| Regeneration command | here: `task docs:figures:update` rewrites every SVG; `task docs:figures` regenerates into a scratch directory and fails on any difference | the task is the command |

Recording the version once, in the script, and letting each SVG report it avoids
a per-figure copy that would rot. A figure rendered by another version fails the
compare and says so by name.

### Layout settings each source must declare

- **PlantUML**: exactly one bare `@startuml` ... `@enduml`, and
  `!pragma layout smetana` on a line of its own. Without it PlantUML uses
  whichever Graphviz the host has, which changes the output (seen on
  2026-10-16: the same state diagram differs with and without Graphviz on the
  path). No `!includeurl` or `!include https://...`: the figure must not depend
  on the network. `<C4/...>` and other `<...>` standard-library includes are
  bundled in the jar.
- **d2**: a `vars: { d2-config: { ... } }` block setting `layout-engine`
  (`dagre` or `elk`, the two bundled open engines; TALA is a separate
  proprietary install), `theme-id` and `sketch: false`. d2 lets `D2_*`
  variables and flags override the file, which is why the renderers run in an
  empty environment as well.

## Renderers and what makes them reproducible

| Tool | Pin | Fetched by `task setup` |
|---|---|---|
| PlantUML | 1.2026.8, the GPL jar `plantuml-1.2026.8.jar`, bundling C4-PlantUML 2.13.0 and Smetana | into `target/tools`, SHA-256 checked |
| d2 | 0.9.0, the host's release archive (macOS arm64, Linux arm64, Linux amd64) | into `target/tools`, SHA-256 checked |
| DejaVu fonts | 2.37, upright Sans, Sans Bold, Sans Mono and Sans Mono Bold | into `target/tools/fonts/dejavu-2.37`, archive and each face checked |
| Java runtime | 11 or later; not fetched | found on PATH, `JAVA_HOME`, `FIGURES_JAVA` or Homebrew's `openjdk` |

Everything that could make two runs differ, and what was done about it
(seen on 2026-10-16 unless marked otherwise):

- **Timestamps and ids**: none. PlantUML's SVG has no date and its element ids
  are sequential; d2 derives its class hash from the diagram. Two renders on one
  host are byte-identical, and so is a render from another directory under
  another file name.
- **Embedded source**: PlantUML appends an encoded copy of the source unless
  told not to; `-nometadata` removes it.
- **Fonts**: PlantUML measures text with the Java runtime's fonts, so the same
  source gave different widths on macOS and Ubuntu. The renderer is run with
  Java's font path set to the four pinned DejaVu faces and
  `defaultFontName`/`monospacedFontName` set to them; with that, macOS arm64
  (OpenJDK 27), Ubuntu arm64 (OpenJDK 21) and Ubuntu amd64 under QEMU user-mode
  emulation (OpenJDK 21) rendered byte-identical SVGs from the same sources,
  with the d2 archive for each host. The amd64 run is emulated, not a native
  x86-64 kernel. The Oblique
  faces are deliberately not pinned: with them the two hosts measured one italic
  label 55.945 and 55.816 wide, and without them Java slants the upright face
  and both agree. d2 embeds its own font and is unaffected.
- **Font fallback**: PlantUML names `DejaVu Sans` with no generic family, so a
  reader's browser without it would fall back to its serif default. The script
  appends `sans-serif` or `monospace` to those attributes after rendering, the
  same way for check and update.
- **Environment**: both renderers run under `env -i` from a private working
  directory, so `D2_*`, `GRAPHVIZ_DOT`, `PLANTUML_*`, `_JAVA_OPTIONS`, locale and
  a `plantuml.skin` in the caller's directory cannot reach a figure.

## Diagram kinds checked under Smetana

With no Graphviz on the path (`PATH` without `dot`, `GRAPHVIZ_DOT` pointing
nowhere), PlantUML 1.2026.8 rendered a C4 context, a C4 container, a C4
component, a UML state machine, a UML activity, a UML deployment, a UML
sequence and a UML timing diagram. Sequence and timing diagrams do not use a
layout engine at all. No kind in the contract's figure list needed Graphviz, so
there is no Smetana limitation to escalate. Whether a larger figure lays out
legibly under Smetana is the art stage's to judge on the real figures.

## Not yet verified

- GitHub's own rendering of a relative SVG link: the book is not on GitHub. The
  path was confirmed to resolve beside the page and in the mdBook output.
- Native x86-64 Linux hardware, and any Windows or macOS x86-64 host (not
  pinned hosts); other Java runtimes than OpenJDK 21 and 27.
- A reader's browser without DejaVu Sans: the generic fallback is in the SVG,
  but the rendering in such a browser was not inspected.
- How the figures look in mdBook's dark themes: PlantUML's SVG has a white
  background. The art stage inspects the declared viewing environments.

## Checking the check

`task docs:figures:test` runs the controls: the fetch refusals offline, two
renders compared, a hostile environment, and each of changed source, changed SVG,
missing SVG, orphan SVG, another renderer version, a changed layout pin, a
changed shared include, a removed font pin and every lint refusal made to fire
against a subject known to be wrong. Its fixtures live in
`scripts/figures-fixtures/`, outside the book. `task check` does not run it and
does not need PlantUML, d2 or Java.

## Changing a pin

A renderer or version change is a change to the contract. Re-read the release's
digests, edit the constants in `scripts/figures.sh`, run `task setup`, then
`task docs:figures:update` for the fixtures and the book's figures, and read the
diff of every figure before committing it.
