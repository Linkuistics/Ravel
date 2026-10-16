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

## Visual vocabulary

The opening unit establishes these conventions in `src/figures/_style.puml`.
Include it after the bundled C4 library. Later units reuse the conventions and
extend them only for a new reader question; explain each extension beside its
first figure.

| Convention | Meaning |
|---|---|
| C4 system-context view | Ravel as one system, surrounded by its caller, owner and selected program |
| C4 container view | Ravel's running units and temporary store; not OS containers or simultaneous process lifetimes |
| Person shape | A human role; the caller uses a labelled box because it can be a person or a program |
| Blue box (`#1E4F78`) | Ravel system or process, at the level the view names |
| Grey box (`#E8EDF2`) | External participant, such as caller or managed process |
| Gold cylinder (`#FFF1CF`) | Temporary result channel; the cylinder does not imply a database or persistent store |
| Dashed enclosing outline | Software membership, not authentication, a sandbox or a process-group boundary |
| Directed arrow | The action or information named on it; it does not imply a function call or chronology |
| Green arrow (`#135E50`) | Opaque-result path, also identified by its label |
| Numbered relations | Only the ordering explicitly stated in the key; not a complete sequence diagram |
| Angle-bracket labels | C4 element types and the `temporary` tag |
| Square-bracket labels | Technology, interface or data, as the words specify |

The runner is text inside the native front's box: it executes there and has no
separate process box. The policy response and opaque-result paths have separate
arrows. Worker completion, reap and private-directory removal precede the
managed launch; the container view's key states that admission rule explicitly.

Each figure has an adjacent sentence stating what the reader should learn,
local notation teaching, a key inside the image, meaningful Markdown alt text
and a full-size SVG link. Colours supplement labels. Keep a white canvas,
dark legend text and explicit light legend fill so the SVG is self-contained
when displayed on a dark page. The chapter's text remains the alternative
when the image cannot be read.

Chapter 3 extends this vocabulary with UML sequence and state-machine views.
`_uml-style.puml` reuses `_style.puml` and adds notation-specific settings; its
C4 include supplies the shared style macros without drawing C4 elements.

| Convention | Meaning |
|---|---|
| UML sequence: dashed vertical line | A participant's lifeline, beginning when introduced; the front creates the worker, managed process and result channel where their heads appear. Order runs down the page, without a time scale |
| UML sequence: solid, dashed or bent arrow | An action, a reply or work within one participant, respectively |
| UML sequence: X | End of the pictured lifetime after confirmed reap or temporary-directory removal, not the instant of process exit |
| UML sequence: labelled horizontal bar | A group of related steps, not a process or boundary |
| UML state machine: solid dot | Start of the session |
| UML state machine: blue rounded box | A phase of Ravel's work, not a process |
| UML state machine: grey rounded box | An outcome, with no further transition shown |
| UML state machine: arrow and square-bracket text | A transition and its condition, respectively |
| UML state machine: green route | The route to successful opaque-result delivery |

The sequence is `successful-session`; the reusable outcome view is
`session-outcomes`. Their captions, notation teaching and prose alternatives
live beside them in chapter 3. Its state machine groups operations and names
settled outcomes; the full precedence algorithm belongs to chapter 8.

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

## Viewing checks

Chapter 3's art stage inspected `successful-session` and `session-outcomes`
in the locally served mdBook through native Chrome on 2026-10-16, with Auto
displaying a light page and with Navy. Both figures, their notation, captions
and prose alternatives were legible at the book's chapter width; the sequence
and the outcome view's first layout were also inspected through their
full-size SVG links. The outcome view was then simplified to improve its
chapter-width labels and inspected again in both themes. The raw chapter and
PlantUML sources were read alongside the rendered book. Chapter 3's editor
preview, narrower viewports, GitHub rendering and published site were not
inspected. This is a visual inspection, not a human comprehension result.

An isolated macOS TestAnyware guest loaded the chapter's accessibility tree,
but its VNC capture was black and its native screen capture failed. It supplied
no visual evidence and was stopped; the checks above used host Chrome. The
in-app browser was unavailable.

Opening-unit art inspected both SVGs in the locally served mdBook through
native Chrome on 2026-10-16, with Auto displaying a light page and with Navy.
The white canvases, labels, arrows and legends remained legible in both.
Chapter 2's nested-publication, worker-directory sequence and information
tables were inspected in Navy. Its container figure and those tables also
rendered legibly in VS Code's Markdown preview, in Restricted Mode on a light
background. The raw chapter and PlantUML sources were read alongside them.

Opening-unit proof independently inspected both figures and their surrounding
notation in the locally served mdBook through native Chrome on 2026-10-16,
on a light page. Chapter 2's authority, nested-publication, worker-directory
sequence and information tables were legible there too. Both chapters and
their figure sources were read in raw form. These checks do not extend the
earlier art inspection to additional themes, editor previews or viewport sizes.

The computer-use browser inventory was empty and the in-app browser unavailable;
native app access supplied these checks. No human comprehension result follows
from an art-stage visual inspection.

## Not yet verified

- GitHub's own rendering of a relative SVG link: the book is not on GitHub. The
  path was confirmed to resolve beside the page and in the mdBook output.
- Native x86-64 Linux hardware, and any Windows or macOS x86-64 host (not
  pinned hosts); other Java runtimes than OpenJDK 21 and 27.
- A reader's browser without DejaVu Sans: the generic fallback is in the SVG,
  but the rendering in such a browser was not inspected.
- mdBook's Rust, Coal and Ayu themes, narrower viewports, and chapter 1's editor
  preview. The checks above cover only the named pages and environments.
- The published site, which is unavailable before the book's first release.

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
