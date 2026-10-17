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
- The directory holds `.puml`, `.d2`, `.typ`, `.svg` and `.layout.json` files
  only. A `.typ` figure also has a `<name>.layout.json`, the layout record it
  evaluates to, which `check` compares apart from the SVG and `update` rewrites.
  Anything else, a subdirectory, an SVG with no source, or a `.layout.json` with
  no `.typ` figure beside it (a `.puml`, a `.d2` or a module does not count) is
  refused, so the compare covers every file there.
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

All diagram types use the palette and scale in `src/figures/_style.puml`.
Include it after the bundled C4 library. `_uml-style.puml` includes that
library and the common style, then supplies sequence, state and deployment
settings. The library supplies style macros without drawing C4 elements in
UML views. Individual figures select semantic roles; they do not redefine
fonts, borders or spacing. Page-native tables keep mdBook's normal styling.

| Role | Fill / text / border |
|---|---|
| Ravel system, process, linked component or executable; Ravel work phase in a state view | Blue `#1E4F78` / white `#FFFFFF` / navy `#163A59` |
| External caller or managed process; supporting installed file; final outcome in a state view | Grey `#E8EDF2` / ink `#172B3A` / slate `#64748B` |
| Human owner in the context view | Slate `#475569` / white / dark slate `#334155` |
| Temporary result channel | Gold `#FFF1CF` / brown `#3D321A` / ochre `#8A6B26` |
| Notes, keys and host enclosure | Pale grey `#F8FAFC` / ink / slate |
| Ordinary relation and state entry marker | Dark slate `#334155`; relation text is ink |
| Opaque-result path or successful result-delivery route | Green `#135E50`, also identified by labels |
| Canvas and installation directories | White; directory labels are ink and borders slate |

| Property | Shared choice and notation-specific reason |
|---|---|
| Font family | Pinned DejaVu Sans; DejaVu Sans Mono for explicit code styling. The renderer adds generic browser fallbacks. |
| Type scale, in SVG units | Body, relations, keys and notes 14; titles, C4/sequence participant heads and deployment enclosure heads 16; C4 type/technology qualifiers 12. C4's heads come from its Creole heading markup. State names remain 14 because they are short work/outcome labels; deployment file listings use the 14-unit body scale. |
| Main strokes | Entity, boundary, note, key and arrow shafts 1.5. Sequence lifelines are 0.5 to keep them behind messages. The pinned renderer draws arrowhead outlines and some detail at 1, and paired separator rules at 0.75; those are notation glyphs, not competing semantic weights. |
| Arrow forms | C4's relationship heads, UML transition heads, sequence action/reply heads and the deployment dependency keep their notation's form and dash pattern. Colour and shaft weight are shared. |
| Spacing | C4/state/deployment layout uses `nodesep 40`, `ranksep 50` and `wrapWidth 220`. Sequence participant spacing is 20, participant padding 7 and message wrapping 55, with explicit line breaks for semantic groups. Titles use padding/margin 5; keys use padding 8 and margin 12. Layout engines interpret these settings differently; figures are checked at their final rendered size. |
| Canvas | White, with no shadows. Keys have explicit pale fill and dark text, including when the image sits on a dark page. |

These sizes are source-space units. mdBook scales a wide SVG to the chapter
column; the full-size link preserves access to its original scale. Do not
shrink individual text to force a figure into the column.

Each figure has an adjacent sentence saying what the reader should take from
it, a book-specific key, meaningful Markdown alt text and a full-size SVG link.
Explain the book's meanings and limits, not generic diagram notation. Colours
supplement labels. The surrounding prose supplies an alternative to the image.

### What each view means

| View | Book-specific meaning to preserve |
|---|---|
| `system-context` | Ravel between a caller, such as an orchestrator, and a harness running an LLM session or another selected program; the owner controls routing. Publication asks Ravel to stop, and return follows confirmed cleanup. Responsibility boundaries are not security boundaries. |
| `process-authority` | Processes and temporary files across a session, not simultaneous lifetimes or OS containers. The worker response and opaque result use separate paths. Worker end, reap and private-directory removal precede the managed launch. The runner runs inside the Ravel CLI. |
| `successful-session` | The unconfined successful path. An X ends a pictured lifetime only after confirmed reap or directory removal, not process exit. Publication precedes cleanup; only the last result arrow delivers to the caller. Terminal recovery applies only when the terminal was lent. |
| `session-outcomes` | Work phases and final outcomes. Cleanup is attempted on every path; an attempt need not succeed. The only green route requires a valid result, no cancellation, confirmed cleanup and a successful final write. The diagram groups operations rather than giving the full precedence algorithm. |
| `settled-exit-status` | Outcome precedence for `start-session`, not implementation call order: cancellation, original failure, required cleanup, final write. Cleanup is attempted before reporting. An original failure keeps its status if later cleanup fails. The filename is retained for stable links. |
| `selection-cancellation`, `supervision-cancellation` | Direct INT delivery to the Ravel CLI in the examples, not keyboard delivery. Cleanup steps are attempts and no X promises confirmed removal. Cancellation suppresses publication; HUP/TERM use the same rule. |
| `source-components` | The Ravel CLI's checking/final-outcome code and linked runner share one process; the worker is separate. The enclosing boundary means process membership, not confinement. No third runner executable is installed. |
| `installed-files` | Keep the CLI and worker from one build together. Outlines group installation directories; a file group lists separate files. Adjacent source edits do not replace embedded modules. Archive-only README and licence remain identified. |

Chapters 1–6 also use page-native tables for publication through cleanup and
return, responsibilities, result versus
exit status versus work, platform evidence, nested result authority, worker
startup, policy evaluation order, policy-context delivery, environment grants,
confinement, streams, route choices, import resolution and lookup fields.
Their adjacent sentences state the comparison, mapping or sequence the reader
can use. They need no separate figure source.

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

Known sources of differing renders, and the controls checked for them
(seen on 2026-10-16 unless marked otherwise; these observations are not a
guarantee across every Java runtime):

- **Timestamps and ids**: none. PlantUML's SVG has no date and its element ids
  are sequential; d2 derives its class hash from the diagram. Two renders on one
  host are byte-identical, and so is a render from another directory under
  another file name.
- **Embedded source**: PlantUML appends an encoded copy of the source unless
  told not to; `-nometadata` requests its removal. The pinned release can race
  while initialising shared format options when processing files concurrently,
  and fresh failed renders contained source metadata despite this flag.
  `--threads 1` avoids that concurrent initialisation. The exact comparison
  remains unchanged: the checker neither removes metadata nor retries failures.
  [The retained failure and bounded verification](../evidence/book-figures-k92/README.md)
  distinguish observed differing bytes from the source-supported race diagnosis.
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

## Typst figures (added beside PlantUML by `book-figure-engine-k109`)

The book is moving its figures from PlantUML to Typst with the CeTZ drawing
package (decided 2026-10-17, planned in `book-figure-migration-k108`). This
section describes the Typst side as it now stands in the toolchain. Until the
last `.puml` is ported, both renderers run, every rule above still holds for a
`.puml`, and the text above is left for the leaf that retires PlantUML to
rewrite (the last `.puml` was ported by `book-figure-settled-exit-status-k116`). The three sequence figures (`successful-session`, `selection-cancellation`
and `supervision-cancellation`, ported by `book-figure-sequences-k111`) and the two
C4 figures (`system-context` and `process-authority`, ported by
`book-figure-c4-context-container-k112`) are `.typ` figures with committed
`.layout.json` records, as are `source-components` (`book-figure-source-components-k113`)
and `installed-files` (`book-figure-installed-files-k114`) and `session-outcomes`
(`book-figure-session-outcomes-k115`) and `settled-exit-status`
(`book-figure-settled-exit-status-k116`); each port replaces a figure's `.puml` with
its `.typ` in one commit. All nine figures are now `.typ`: no `.puml` figure is
left, and the leaf that retires PlantUML removes the renderer and the text above.

**On disk.** A `.typ` whose name does not start with `_` is a figure source,
beside the `.puml` and `.d2` ones, with its SVG of the same name. The
one-source-per-name rule extends to it: `x.puml` and `x.typ` are refused, as
`x.puml` and `x.d2` are. A file starting with `_` is a module that figures
import. It is not a figure and gets no SVG; an SVG for one is refused as an
orphan. Seven modules hold the Typst side. `_style.typ` holds the palette roles,
text scale, stroke weights, faces and text measuring, and draws nothing.
`_layout.typ` holds the constructors a layout module builds its record with and
the orthogonal ports and routes that `_c4.typ` and `_state.typ` share.
`_sequence.typ` is the sequence-diagram layout, `_c4.typ` the C4 layout,
`_deployment.typ` the deployment layout and `_state.typ` the state-machine
layout (below): each measures its text and returns a layout record. `_draw.typ` turns a record
into strokes and is the only module that imports CeTZ; its header documents the
record's shape and the styles. `_style.typ` and `_sequence.typ` were promoted from
the k107 trial, which was removed once its findings were recorded here (its
sources stay in the history of the commit that added it,
`book-figure-renderer-k107`); the sequence figure rendered through the modules at
`--input style=crisp` was byte-identical to the trial's, for the base and the long
variant.

**What the sequence layout covers.** Messages and dashed replies, a self-call
(its label may be several lines), `create` (the participant's head drops to the
creating message and its lifeline starts there), `destroy` (the X), section
dividers, and notes spanning two or more participants. A participant may be a
cylinder (`db: true`); a message is `role: "result"` for the arrow that carries
the session result. Each label wraps at `max-label` points (default 170, a
parameter of `sequence`); the three book figures pass 300 or 340 so that their
long labels, with the line breaks written in each source, are not split at a
hyphen. Column gaps and row heights follow the measured labels, so adding a
participant or lengthening a label edits no coordinates: a scratch copy of
`supervision-cancellation.typ` with an `audit sink` participant and a
four-line message rendered with no other change. A label's apostrophe is
written in the source as `'` and drawn as `’` (typographic), as in the trial;
the old PlantUML figures drew a straight one.

**What the C4 layout covers.** `_c4.typ` serves the system-context, container and
component views: an entity in a semantic role (`ent`: «stereotype», name, [technology] and a
description, in `ravel`, `external`, `owner` or `temporary`, as a box or, with
`db: true`, the shared cylinder with its text inside), a boundary (`boundary`: a
dashed outline around named entities, with a name and a `[tag]`), a relation
(`rel`: a label and a technology line beside an orthogonal route, in role
`relation` or `result`, the green one) and a key. A figure source is the entity
list, the boundary list, the relation list and the key, handed to `c4(...)` and
then to `render`. **Placement is manual.** An entity has a centre `cx`, a `top` (or
`cy-of`, to sit level with another entity) and a width; its height is measured
from its text. A relation names two entities, the side of each it leaves and
enters by and an offset along the side; the module draws the straight, L or Z
route between the two ports and puts the label beside the longest segment (or
`seg`), at `t` of the way along it, `side` of it, nudged by `dx` and `dy`. It does
not choose positions or avoid collisions: a label that lands on a line or a
boundary is moved in the figure source and rendered again. Edge routes and label
positions were the cost in the trial and remain the cost; entities and the key
are not. A boundary is at least as wide as its own name and tag (centred on its
members when the name is wider) and the topmost element sits 14 points below the
title. Both came from a scratch sketch of `source-components` (one boundary around
two of four entities, with a tag) made to check that the third user fits; the
port of that figure then needed no further change to the module.

**What the deployment layout covers.** `_deployment.typ` serves `installed-files`,
and nothing else uses it. A figure source is a tree: `artifact` (a box with a page
icon, in `ravel` for an executable or `external` for a supporting file, holding
lines of text), `dir` (a directory: an outline with a tab sized to its name) and
`node` (the UML node: a box with a top and a right face, its name below the top
face), each enclosure holding children in a `row` or a `col` with a `gap` and a
`pad`; and `dep(from, to)`, a dashed arrow between two artifacts. **Placement is
automatic**, unlike the C4 layout, because an enclosure is a container: the module
measures every artifact's text, sizes each enclosure from the inside out and
places the boxes, so no figure source holds a coordinate. A row aligns its
children on the centre of the first artifact each one holds, which makes the
dependency from `bin/ravel` to `ravel-policy` a straight line although the worker
sits inside `libexec/ravel`; where the two centres are further apart than the
source's side allows, the arrow is a Z. A directory's name is inside its tab and
the tab is as wide as the name, so no outline crosses a name; the enclosure is
at least as wide as its tab. An artifact's width is its text plus room for the
icon, so a long first line never meets the icon. A line wider than an artifact's
`limit` wraps (the worker's limit is 210, which breaks it after "static", as the
old figure did). The record uses the shapes `_draw.typ` already had (polygons for
the enclosures and the icons, a box for an artifact), so the module is the only
code the figure needed; `fit`, the measured wrapping width, moved from `_c4.typ` to
`_layout.typ` to be shared, and the other eight records and SVGs came out
byte-identical.

**What the state layout covers.** `_state.typ` serves the state views, and
`session-outcomes` and `settled-exit-status` are its users. A figure source is the state list, the
transition list and the key, handed to `state-machine(...)` and then to `render`.
A `state` is a rounded box in the `phase` (blue) or the `outcome` (grey) role,
holding lines of text, and `initial` is the filled marker; a transition (`tr`)
names two states and carries an optional guard label and a role, `relation` or
`result` (the green route, with a green label). **Placement is by grid cell.** A
state has a `col` and a `row`, either of which may be fractional (`2.5` is halfway
between the centres of columns 2 and 3); each column is as wide as its widest
whole-cell state and each row as tall as its tallest, with `col-gap` and
`row-gap` (a number, or one per gap) between them, and a state's `min-w` widens
it past its text. No figure source holds a coordinate. A transition's route is the
shared one in `_layout.typ` (the C4 module's ports and routes, moved so both use
them): it leaves and enters by a side, straight, an L or a Z or U, with `jog` for the
middle run. The port "align" puts a port level with the other state's centre,
kept inside its own side, so a fan of transitions leaves one wide state one at a
time with no offset written. The label sits beside one segment (`seg`, `t`,
`side`, `dx`, `dy`, `w`, as in the C4 module) and its lines are centred. As in
the C4 layout, nothing avoids collisions: a label that lands on a line or a box
is moved in the figure source and rendered again. The record uses the shapes
`_draw.typ` has; two additions came with this module, a filled `circle` for the
initial marker, and the roles `phase`, `outcome` and `initial` (`_style.typ`),
the first two drawn with 12pt corners. A rounded box's sketched outline follows
the corner (a path through points on the arc), where a 3pt box's stays a
wobbly rectangle, so the other eight records and SVGs came out byte-identical.

**Effort and edit cost.** Counted when the figures were ported.
- `system-context`: 4 entities and 5 relations written; all 5 routes named by
  hand (one relation took the default facing sides, four named a side and offset
  so the two pairs run in parallel); label placement chosen for 3 of 5 (`side`
  for 2, a wrapping width for 2); no `dx`, `dy` or `t`; 2 renders to settle (the
  first was structurally right and needed only two vertical gaps tightened).
- `process-authority`: 5 entities, a boundary and 7 relations; 7 routes by hand;
  label placement chosen for 6 of 7 (2 `t`, 1 `seg`, 1 `dy`, 1 `dx`, 2 `side`, 5
  wrapping widths); 3 renders to settle (the long green label crossed the
  boundary, the publish label sat on the wrong segment, the launch label crossed
  the boundary's edge, and the worker's technology line wrapped). The first
  render also showed the boundary's dashes drawn as dots, a fix in `_draw.typ`
  (4pt dashes for the `boundary` role) and not in either figure.
- `source-components`: 4 entities (two inside the boundary, two outside) and 3
  relations written; no route named by hand (all three took the default facing
  sides, because every pair of boxes faces across a gap) and no `side`, `dx`,
  `dy`, `t` or `seg`; label placement chosen for 3 of 3, each a wrapping width; 2
  renders to settle. The first was structurally right and showed two faults, both
  placement: the long technology lines were wider than the gap between the boxes
  and ran into them, and one label ran across the boundary's edge. The second
  render widened the gap (one number, on the two right-hand entities) and wrapped
  three labels. The source is 39 lines, of which 8 are the key.
- `installed-files`: 5 artifacts, 3 enclosures (a node and two directories) and
  1 dependency written; 0 coordinates, 0 hand routes and 0 label moves, because
  the layout is automatic and the one arrow is straight; one wrapping width
  chosen (the worker's `limit: 210`); 2 renders to settle (the first was
  structurally right and wrapped "embedded" alone onto the worker's last line).
  The source is 29 lines, of which 8 are the key, and `_deployment.typ` is the new
  module (181 lines, 44 of them comments). A scratch copy with a fifth
  artifact of three long lines rendered with no other edit.
- `session-outcomes`: 9 states and an initial marker, and 11 transitions, written
  as grid cells (0 coordinates). Sides named by hand on 6 of 11 transitions (the
  other 5 took the facing sides), 0 `jog`; label placement chosen for 4 of the 10
  labelled transitions (3 `side: "left"`, and one `seg`, `side` and `t`), 0 `dx`,
  `dy` or wrapping widths; one row gap and one `min-w` (the cleaning-up state's 250,
  which spreads the fan so a label clears its neighbour's line); 5 renders to settle.
  The first was structurally right; the others fixed the rounded states'
  sketched outline (a change in `_draw.typ`, not in the figure), moved the green
  guard label (above its line it met the label above the state, below it the
  label beside the fan, and beside the long vertical it fits), widened one row gap
  and one state, and restored a line break the old source did not have (found by
  the comparison script, below). The source is
  48 lines, of which 10 are the key, and `_state.typ` is the new module (203
  lines, 56 of them comments).
- `settled-exit-status`: 9 states and an initial marker, and 9 transitions (8
  labelled), written as grid cells in two columns (0 coordinates): the four steps
  and the success outcome in one, the four other outcomes beside their steps.
  Sides named by hand on 2 of 9 transitions (`out: ("right", "align")` for the two
  that leave a step level with a tall outcome; the other 7 took the facing
  sides), 0 `jog`; label placement chosen for 0 of the 8 labels (every label took
  the default segment, side and position), 0 `dx`, `dy` or wrapping widths. Other
  numbers chosen: 1 `col-gap` (190, wide enough for the longest guard between a
  step and its outcome), 1 `row-gap` entry that differs from the rest (72, under
  the cleanup step, for the two-line green guard), 3 `limit` (320, on the three
  outcomes whose text is wider than the default 240), 0 `min-w`. The eight-line
  failure box needed nothing: the row grew to its height and the "align" port put
  the line level with its centre. 2 renders to settle: the first was structurally
  right, and the second widened the one row gap. The source is 47 lines, of which
  10 are the key; no module changed (`_state.typ`, `_layout.typ`, `_draw.typ` and
  `_style.typ` are as `session-outcomes` left them) and the other eight records and
  SVGs are byte-identical.
- The trial's extra container (`audit sink`), added to a scratch copy of
  `process-authority`: one entity (4 lines), one relation (2 lines, a left-to-left
  route given by `jog`) and one name added to the boundary's members; no legend
  offset, no coordinate edit; the first render was usable, where the trial needed
  an entity, an edge with a hand-chosen route, a legend offset and two retunes.

**Differences from the PlantUML C4 output, with the reason.** Every entity, tag,
technology line, relation label, number and key line is present: a script
compared each quoted string and key line in the three old sources with the text
in the new layout records and found none missing.
- Relations are orthogonal polylines, not splines: the route is data a person
  edits, and a spline needs a layout engine.
- Entity names and the title are bold 16pt (the old output set them in regular
  weight), matching the sequence figures; stereotype and technology lines are
  upright 12pt (see below).
- Apostrophes are typographic (`’`) where the source writes `'`, as in the
  sequence figures.
- The owner has no person icon: it was a sprite from the C4 library, and the
  `«person»` stereotype, the slate role and the key still say who it is.
- Text wraps at the box width less 24 points, measured in the pinned font, so a
  description breaks at different words than before.
- The figures are drawn sketched at roughness 2 with a crisp key, like every
  Typst figure.
- The component view's `Lay_D(worker, managed)` hint, which kept the worker above
  the managed process, is the figure source's two columns: the worker's and the
  managed process's `cx` are the same. Its boundary header names the
  boundary in regular weight with `[Container]` beneath, as the other boundaries
  do, and an embedded line break in the worker's description is kept as a line
  break.
- A Typst figure's SVG holds paths and no `<text>`: `process-authority.svg` is
  259 KB, `source-components.svg` 211 KB and `system-context.svg` 161 KB (the
  nine figures together are 1.46 MB, of which the two PlantUML ones are 52 KB).

**Differences from the PlantUML deployment output, with the reason.** Every
artifact line, enclosure name, the title and the eight key lines are present: a
script compared each quoted string, the title and each key line of the old source
with the text in the new layout record and found none missing (and, with one
string removed from the text it searched, reported exactly that string, so it can
fail).
- Enclosures are drawn by the module from the tree, not by a graph layout:
  the three file groups stack in one column because the column is the source's
  `col`, which replaces the old source's three hidden links.
- A directory's name sits in a tab sized to it, so no outline can cross a label
  (the past art pass fixed a PlantUML label that crowded an outline); the node's
  name sits below its top face. The page icon is a drawn polygon with a folded
  corner, in the artifact's colours, rather than PlantUML's.
- The artifact text is wrapped at a measured width (the worker's at 210 points)
  and drawn in the same 14-point body face; the title is bold 16pt, as in every
  Typst figure.
- The dependency is a dashed arrow in the shared arrowhead and stroke weight,
  without PlantUML's spline.
- The figure is drawn sketched at roughness 2 with a crisp key, like every Typst
  figure, and its SVG holds paths and no `<text>` (160 KB, where the PlantUML one
  was about 12 KB).

**Differences from the PlantUML state output, with the reason.** Every state name,
guard and transition label, the title and the ten key lines are present: a script
compared each quoted string of the old source with the text in the new layout
record and found none missing; it reported the string, and the key, when one was
removed from the text it searched, so it can fail. The two green transitions were
first missed by the script's own pattern (it did not read `-[#135E50]->`), which
showed when a green label was removed and nothing was reported; the pattern was
fixed and the removal then reported it.
- Transitions are orthogonal polylines, not splines, as in the C4 views, with the
  three phases in one column, the bypass from selecting to cleanup on the right
  and the outcomes in a row below, where PlantUML's graph layout curved them.
- Guards and labels are centred beside a line and wrapped at the line breaks the
  old source wrote; the long green guard is two lines, as in the source.
- The initial marker is a filled circle in the relation colour; there is no final
  marker, as in the old figure ("no further transition is shown").
- The figure is drawn sketched at roughness 2 with a crisp key, like every Typst
  figure, with 12pt corners on the states, and its SVG holds paths and no
  `<text>` (184 KB, where the PlantUML one was 22 KB).

**Differences from the PlantUML exit-status output, with the reason.** The nine
state texts (the eight-line failure box and the three-line cancelled box included,
each line), the title, the eight guards and transition labels and the ten key lines
are present: the same script (`python3 -I`) compared each quoted string of the old
source, taken from the previous commit, with the text in the new layout record and
found none missing, and reported the string when one was removed from a copy of the
record (a key line, an unbracketed label, a green label, a bracketed guard and an
exit-code line). Every exit status in the boxes was checked against
`crates/ravel/src`: the constants in `refusal.rs` for 2, 3, 5, 124, 126 and 127,
`128 + signal` in `cancellation.rs` for 129, 130 and 143, and chapter 6's status
table and the CLI help for 1; none changed.
- The steps run down one column and each outcome sits to its right, level with the
  step, as the old figure's `-right->` transitions asked; the success outcome is
  below the last step. The lines are orthogonal, not splines.
- The tall failure box sets the height of its row, so the row is taller than the
  others; PlantUML drew the same box and moved the rows to suit.
- Guards are centred beside a line and wrapped at the breaks the old source wrote.
  The initial marker is a filled circle, as in `session-outcomes`.
- The figure is drawn sketched at roughness 2 with a crisp key and its SVG holds
  paths and no `<text>` (208 KB, where the PlantUML one was 30 KB; the nine
  figures together are 1.80 MB, all Typst, under the 2 MB budget, and the largest
  is 259 KB, under the 300 KB limit for one).

**Typst's markup trap.** A label that starts `2. ` is parsed as a numbered list
item, not text. In the trial it drew the label as a list. Write `2\.` in the
figure source (`[2\. Launches]`); the record holds the string `2. Launches`. The
two numbered relations in `process-authority` need it. Under the layout record
the mistake is loud: `_layout.typ` accepts only text, spaces and line breaks in a
label, and the render stops with `a label may hold only text, ... not
item(number: 2, ...)` (seen by removing the backslash from a scratch copy).

**Extraction from PlantUML, tested in the trial, not adopted.** PlantUML's Smetana
SVG exposes node boxes (`rect x/y/width/height`) and link Béziers
(`<g class="link">` with `M… C…` paths), but the cylinder is path-only, edge-label
positions and roles were not checked for extractability, and the result would be
a one-way converter leaving PlantUML as a second editable layer. A PlantUML text
width agreed with Typst's measured width to a mean of 0.2 units over 240 runs
(maximum 2.7 on a 570-unit line), so a PlantUML-layout, Typst-final conversion
would not have moved text noticeably; it was still not built. Both C4 figures
are maintained Typst source with no `.puml`.

**Single-engine risk.** PlantUML's C4 and UML conventions (standard shapes,
`legend`, the standard library) exist now only as this directory's hand-written
primitives, and no PlantUML figure is left; any d2
diagram has no counterpart.

**Layout, then draw.** A figure source builds a record with a layout module and
passes it to `render` in `_draw.typ`. The record is plain data: positions, roles
and measured text, and no colour, weight, font size or roughness, so a restyle
cannot change it. `render` draws it and attaches it to the page as
`metadata(record) <layout>`, which `typst eval 'query(<layout>).first().value'
--in FIG.typ` prints as JSON. The figure is drawn sketched at roughness 2 unless
`--input style=crisp` (or `--input roughness=N`) says otherwise; key boxes are
never sketched. `scripts/figures.sh` passes `--input figure=NAME` so each figure
seeds its own sketch. Only `_draw.typ` calls CeTZ, so how all lines are drawn can
change in one place: the lint refuses a figure source that imports a package
itself and any other module that imports CeTZ. A module may import a pinned
package other than CeTZ.

| Tool | Pin | Fetched by `task setup` |
|---|---|---|
| Typst | 0.15.1, the host's static release archive (macOS arm64, Linux arm64, Linux amd64) | into `target/tools/typst`, archive SHA-256 checked, version checked before every run |
| CeTZ | 0.4.2 and its one dependency oxifmt 1.0.0, from the Typst package registry | unpacked into `target/tools/typst-packages/preview`, tarball SHA-256 and unpacked-tree digest checked |

**Why these versions, checked 2026-10-17.** Typst 0.15.1 (published 2026-07-17)
was the latest release. CeTZ 0.4.2 is what the trial drew with; 0.5.2 exists
and is not adopted, because moving up means re-trialling the primitives and is
a decision of its own. The Typst digests equal the ones GitHub lists. The
registry lists no digests, so the package digests were computed from the
download, and the unpacked trees matched the copy Typst had fetched itself.

**Fetched, not vendored.** CeTZ is LGPL-3.0-or-later and ships a WebAssembly
plugin; this repository is Apache-2.0, and nothing else the book needs is
vendored (PlantUML, d2, mdBook and the fonts are fetched too). Fetching keeps
the licence text and the binary out of the tree and costs one pin. The SVGs
carry none of CeTZ's code, only paths and text. This is a recorded engineering
choice, not legal advice.

**Offline once set up.** A render runs the pinned `target/tools/typst`, never
one on `PATH`, under `env -i` from a private directory, with
`--ignore-system-fonts`, `--ignore-embedded-fonts` and `--font-path` set to the
four pinned DejaVu faces, and `--package-cache-path` set to the pinned cache.
Before any render, the check verifies the Typst version, every pinned package
by its tree digest, and that the cache holds nothing else. A missing cache
fails with `run 'task setup'` instead of starting a download. As a second line,
a render's proxy points at a refused port, so an import of anything unpinned
fails with a refused connection and downloads nothing; without that, Typst
fetches a missing `@preview` package on its own (seen on 2026-10-17). A lint
refuses any `@namespace/name:version` in a `.typ` that is not a pinned
package, `@local` included.

**Fonts.** Typst embeds Libertinus Serif, New Computer Modern (and its Math face) and
DejaVu Sans Mono, and treats a missing family as a warning, not an error: with a family missing it
rendered with a fallback and exited 0. The render therefore ignores embedded
fonts, and any output on Typst's stderr fails the figure.

**Stereotype and technology lines are upright.** The PlantUML C4 output set
them in italics. Pinning DejaVu Oblique was avoided only because Java measured
the italic face differently per host, and that still holds while PlantUML is
here, since the faces share one directory that is also Java's font path. Typst
draws a missing italic upright (no skew in the SVG), and the 12pt step and the
`«»` and `[]` marks carry the distinction. The retirement leaf may reopen this:
it is one pinned face and a `style: "italic"` in `_style.typ`.

**Version mismatch.** Typst's SVG carries no version stamp, unlike PlantUML's
and d2's, and the script does not edit the SVG to add one. A wrong Typst is
named up front by the version check (`Typst 0.15.1 is pinned, found '...'`).
Because renders always use the pinned binary, an SVG made by another version
can only be a committed one, and it surfaces as a difference whose message names
the pinned Typst and CeTZ. The check cannot tell that cause from a changed
figure; regenerating is the remedy for both.

**Reproducibility, observed on macOS arm64 (Linux not run).** Two renders, a
render from another directory, and a render under a hostile environment
(`TYPST_*`, `SOURCE_DATE_EPOCH`, locale, proxy and `XDG_*` set) are
byte-identical. Linux arm64 and amd64 archives are pinned and fetched by the
same code, but this leaf did not run them, and Typst's output on Linux is
unchecked.

**Text in the SVG.** Typst writes every glyph as a path: the text is not
selectable or searchable, and the files are 5-7 times larger than PlantUML's.
The page's alt text, caption and prose are the text alternative. The size
budget (each SVG at most 300 KB, the set at most 2 MB) is checked by
`book-figure-viewing-check`.

**Controls.** `task docs:figures:test` adds: a passing `.typ` fixture and its
byte-identical re-render; a changed source, module and SVG; `update`; a Typst of
another version; a missing, empty, changed, extended and symlinked package cache
and an extra package or namespace in it; a `.puml`+`.typ` and a `.d2`+`.typ`
stem clash; an SVG for a module; an unpinned, other-version and `@local`
import; a figure importing a package itself; a missing font family; a render
with an empty package cache or an unpinned import that reaches only a refused
connection; and a render with no fonts that cannot fall back to an embedded one.
The book's own four modules are rendered through a figure with every kind of
row, twice, with no bytes committed for them (a style change is a change to every
figure); the control checks that the figure is sketch at roughness 2, that its key
box is crisp, that two names give two different sketches, and that a module other
than `_draw.typ` importing CeTZ is refused. Each safeguard was also removed from a copy of
the script, one at a time, and the control for it failed: the embedded-font
flag, the proxy, the stderr rule, the lint call, the lint's no-import rule, the
package check, the version check, the symlink rule and the `.typ` source glob.

## Diagram kinds checked under Smetana

With no Graphviz on the path (`PATH` without `dot`, `GRAPHVIZ_DOT` pointing
nowhere), PlantUML 1.2026.8 rendered a C4 context, a C4 container, a C4
component, a UML state machine, a UML activity, a UML deployment, a UML
sequence and a UML timing diagram. Sequence and timing diagrams do not use a
layout engine at all. No kind in the contract's figure list needed Graphviz, so
there is no Smetana limitation to escalate. Whether a larger figure lays out
legibly under Smetana is the art stage's to judge on the real figures.

## Viewing checks

The records below are dated evidence of the artifacts each pass inspected.
They are not current style instructions; the shared conventions above govern
new and revised figures. Earlier references to notation teaching describe the
superseded contract.

### First-half style correction, 2026-10-16 (`book-first-half-k99`)

This pass read all nine sources, both shared style files, the bundled
C4-PlantUML 2.13.0 definitions and PlantUML 1.2026.8's native skin, then
enumerated the rendered SVG attributes. C4 views use 12/14/16-unit text;
UML views use 14/16. Visible text uses DejaVu Sans with the script's sans-serif
fallback. Principal borders and arrow shafts are 1.5; sequence lifelines are
0.5, paired separator rules 0.75 and arrowhead/detail outlines 1. The black
sequence rectangles are fully transparent lifetime regions, not a visible
palette exception. Shared definitions now state these choices where the
renderer exposes them; notation-specific forms remain intact.

All nine final figures were inspected in locally served mdBook through Chrome
on macOS, at the light theme's approximately 750-pixel chapter column in a
1467 × 2048 viewport, and as full-size SVGs. Labels, keys, captions and nearby
prose were legible. The successful-session and process-authority views are
the densest at chapter width; their full-size links provide larger detail.
The process boundary heading was shortened to clear an incoming arrow, with
its software-boundary meaning kept in the key. The installed-file dependency
was made horizontal, with its discovery/pairing explanation in the key, to
clear the directory tab and outline. No labels remain across those outlines.

Raw Markdown, PlantUML and SVG files were read. A Linux testanyware guest
started and opened Firefox, but screen capture then failed with an early EOF;
it supplies no usable book-viewing evidence and was stopped. This pass did not
check editor previews, other themes or viewport widths, GitHub's rendering of
the unpublished branch, or the unpublished site. These are viewing limits,
not claims about how those environments render the figures. Earlier
cross-platform byte comparisons and the retained k92 failures remain evidence
of their original inputs; this style pass does not extend their scope or
identify the cause of the lost selection render.

### Earlier viewing records

The first-half set proof independently read all six chapters and the nine
figure sources with both shared style files on 2026-10-16. In locally served
mdBook through Chrome at the existing light-page chapter width, it inspected
the chapter 1 reading routes, chapter 5 caller-input tables, chapter 3 outcome
figure and adjacent explanation, and chapter 6's publication, status and
diagnostic entries. The sampled diagram, prose and tables were legible, and
the caller-route link opened its intended page. This samples direct reference,
status, error and figure entries; it is not a full visual consistency audit.

VS Code exposed a blank preview surface, so this pass supplies no editor-preview
evidence. Other themes and viewports, the full set of full-size diagrams,
GitHub rendering of the unpublished branch and the unpublished site were not
inspected. Raw files
were read. The owner changed the contract during this proof: assume familiar
diagram notation, unify colours/fonts/line weights and other visual properties,
and use plain prose for a 16-year-old software engineer, naming the command-line
program **Ravel CLI**. The queued copy-edit → art → proof run applies these
changes across the first half; a later whole-book art pass checks all chapters.
The present nine-figure comparison establishes source agreement, not compliance
with those new requirements or human comprehension.

Chapter 6's art stage inspected both new diagrams and their adjacent
explanations at chapter width in the locally served mdBook through Chrome on
a light page on 2026-10-16. Both SVGs were opened at full size. A deployment
arrow label initially crowded its directory boundary; after shortening it,
the regenerated diagram was checked again at full size and chapter width.
The installed-path table was legible alongside its explanation. The whole
chapter and both PlantUML sources were read as raw files.

This pass did not inspect an editor preview, other themes, narrower viewports,
GitHub's rendering or the unpublished site. The earlier guest-capture
limitation was not retried. These are visual/source checks and establish no
human comprehension, terminal or real-agent operation, or native x86-64
qualification. Proof performs its own whole-chapter read and viewing checks.

Chapter 5's final proof after the art correction independently inspected all
three diagrams, their notation and adjacent explanations, and all seven
page-native tables in locally served mdBook through Chrome on 2026-10-16.
The light page at the existing chapter width kept labels, arrows, table cells
and prose legible. The whole chapter and three PlantUML sources were also
read as raw files. This pass found no further chapter or figure correction.

VS Code exposed its older chapter 2 preview, but Quick Open for chapter 5 left
that view unchanged, so this proof adds no chapter 5 editor-preview evidence.
It inspected no other theme, narrower viewport or full-size SVG. GitHub's
rendering and the published site remain unavailable for the unpublished book.
These checks add no human-comprehension, real-agent, terminal-interaction or
platform-qualification evidence; the owner's stand-in read remains later.
The art correction's retained metadata-only failures and thirty matching
serial checks support its bounded renderer correction. They do not establish
the cause of the earlier discarded selection render or universal
reproducibility; the separate evidence below retains those limits.

Chapter 5's proof independently inspected all three figures and their adjacent
notation, explanations and prose alternatives at chapter width in the locally
served mdBook through Chrome on a light page on 2026-10-16. The settlement and
supervision-cancellation SVGs were also opened through their full-size links.
The caller-input, program-resolution, nested-result, confinement-comparison,
output-routing, exit-status and operational-limits tables were legible there.
The whole chapter and the three PlantUML sources were read as raw files.
Proof linked the first use of native front and changed the status table's
literal numbers from bold to code font; no diagram changed.

The browser extension was available for this pass. VS Code returned no usable
window (`cgWindowNotFound`), so there is no editor-preview evidence. Other
themes, narrower viewports, GitHub's rendering of this unpublished branch and
the unpublished site were not inspected. The earlier isolated-guest capture
limitation was not retried. These are source and visual checks, not evidence of
human comprehension, real-agent behaviour or interactive terminal operation.

Proof also found an intermittent regeneration discrepancy: its initial
`task docs:figures` passed, but a later run reported that
`selection-cancellation.svg` differed from its source's render. The failed
check removed its scratch output, so the differing bytes are unavailable.
The before/after digests of the checked book, script and crate sources and
workspace manifests were unchanged. Two direct renders retained for comparison
and six subsequent full checks all matched the committed SVGs. This did not
explain the failed run or establish reproducibility. The following art
correction retained a fresh failure in two other figures: each differed only
by embedded source metadata. Its evidence and the single-worker correction
are [recorded separately](../evidence/book-figures-k92/README.md). The lost
selection render cannot be assigned that cause retrospectively. The SVGs,
sources and renderer pins remain unchanged; the existing proof successor
reads the correction and its limits.

Chapter 5's art stage inspected the three new SVGs and their adjacent
explanations at chapter width in the locally served mdBook through Chrome on
a light page on 2026-10-16. The initial renders were also opened through their
full-size SVG links. After separating the cancellation-status labels and
clarifying two sequence labels, the regenerated views were checked again at
chapter width with the browser's cached assets refreshed. The four new
page-native tables were legible there too. The whole chapter, the PlantUML
sources and the shared styles were read as raw files.

This pass did not inspect an editor preview, other themes, narrower viewports,
GitHub or the unpublished site. The earlier isolated-guest capture limitation
was not retried. These are visual and source checks, not evidence of human
comprehension, real-agent behaviour or interactive terminal operation.

Chapter 4's proof independently inspected the selection/refusal and evaluation
tables, caller/loader cases and host reads, context diagnostic entries, settings
and all six bounds, environment exclusions and the worker/managed-process
comparison, import-resolution table and agent-argument table in the locally
served mdBook through Chrome on a light page on 2026-10-16. Their adjacent
explanations remained legible at chapter width. The whole chapter was also read
as raw Markdown. Proof added the missing context-stage `message_too_large`
entry and clarified that the protocol bound also covers failure reports.

VS Code did not open the requested chapter after Quick Open, so this pass
provides no editor-preview evidence. It did not inspect other themes, narrower
viewports, GitHub or the unpublished site. The earlier isolated-guest capture
failure was not retried. These are visual and source checks, with fake-program
validation recorded by the proof leaf; they establish no human comprehension,
real-agent execution, agent permissions or terminal-interaction result.

Chapter 4's art stage uses four page-native tables: the accepted evaluation
sequence with its stopping conditions, the caller/loader document combinations,
the policy-worker versus unconfined-managed-process environments, and the
resolution bases for imports and host reads. Their editable source and rendered
content are the chapter's Markdown itself; they need no SVG, diagram notation
or separate regeneration step. `task docs:book:build` renders them, while
`task docs:figures` continues to check the existing diagrams. Each table has an
adjacent reader takeaway. The environment comparison links back to chapter 2's
process-authority view instead of duplicating its process drawing.

Art inspected all four new tables and their adjacent explanations in the
locally served mdBook through native Chrome on a light page on 2026-10-16.
The existing selection/refusal, policy-context fields, host-read, settings,
bounds, excluded-name and agent-argument tables were also visible and legible
at chapter width. The caller-document heading was shortened after viewing
and inspected again. The whole chapter was read as raw Markdown. This pass
did not inspect an editor preview, dark themes, narrower viewports, GitHub or
the published site, and supplies no human comprehension evidence. The in-app
browser was unavailable; the earlier guest capture failure was not retried.

Chapter 3's proof independently inspected the example and transcript, both
figures at chapter width, and their surrounding notation, captions and prose
alternatives in the locally served mdBook through native Chrome on a light
page on 2026-10-16. The chapter and editable figure sources were also read raw.
VS Code's empty window did not respond to file-open or quick-access attempts,
so proof supplies no editor-preview evidence. It extends neither the art
stage's theme coverage nor its viewport coverage, and supplies no terminal
interaction or human comprehension result. GitHub and the published site
remain unavailable for this unpublished book.

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

Chapter 6's proof independently read the whole raw chapter and both PlantUML
sources on 2026-10-16. In locally served mdBook through Chrome on the existing
light page, it inspected the installed-path table, both figures and their
adjacent explanations, both full-size SVG links, the sample-choice comparison,
and the bounds and policy-field tables. Labels, relations, directory groups
and keys were legible on those surfaces; no figure correction was needed.
The first older localhost server returned an empty response; a fresh
`task docs:book PORT=8778` supplied the inspected build.

VS Code returned no usable window (`cgWindowNotFound`), so this proof supplies
no editor-preview evidence. Other themes, narrower viewports, GitHub's view of
the unpublished branch and the unpublished site were not inspected. Visual and
source checks establish neither human comprehension nor real-agent behaviour,
interactive terminal operation or native x86-64 qualification. The owner's
stand-in read remains later.

Chapters 1–2's domain-correction art pass inspected both corrected C4 figures,
their adjacent explanations and the publication/cleanup sequence and
responsibility tables in locally served mdBook through native Chrome on
2026-10-17. The light page at chapter width kept the labels and tables legible.
The process-authority SVG was also inspected at full size. Moving the managed
process to the right and shortening its result-arrow labels separated result
publication from result reading; the final layout was rechecked at chapter
width. Both chapters and the PlantUML sources were read as raw files.

This pass did not inspect an editor preview, other themes, narrower viewports,
the full-size system-context SVG, GitHub rendering or the unpublished site.
The in-app browser was unavailable; native Chrome supplied the visual checks.
These checks provide no human-comprehension or real-agent execution evidence.
The accepted Typst/CeTZ trial remains separate from these published figures.

Opening-unit proof on 2026-10-17 independently read the two corrected C4
sources, keys and alternatives. In a fresh locally served mdBook through
native Chrome on a light page, it inspected both figures at chapter width,
the publication/cleanup/return table, the result/status/work and fit tables,
the managed-process explanation and responsibility table, and the process
figure at full SVG size. Labels and keys were legible, with the full-size
process link supplying more detail. VS Code's light Markdown preview in
Restricted Mode also showed chapter 1's introduction, publication table,
system-context figure and complete key, and adjacent output teaching legibly.
Raw files supplied the complete chapter and source read. This proof did not
inspect chapter 2's editor preview, other themes, narrower viewports or the
full-size system-context link. GitHub and the published site remain unavailable;
the check establishes no real-agent execution or human comprehension result.

## Not yet verified

- GitHub's own rendering of a relative SVG link: the book is not on GitHub. The
  path was confirmed to resolve beside the page and in the mdBook output.
- Native x86-64 Linux hardware, and any Windows or macOS x86-64 host (not
  pinned hosts); other Java runtimes than OpenJDK 21 and 27.
- A reader's browser without DejaVu Sans: the generic fallback is in the SVG,
  but the rendering in such a browser was not inspected.
- mdBook's Rust, Coal and Ayu themes and narrower viewports. The checks above
  cover only the named pages and environments.
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
