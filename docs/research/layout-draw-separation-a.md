# Separating figure layout from drawing — survey (a)

Leaf `layout-draw-separation-k119`, research-a, 2026-10-17. This survey answers
the owner's request, made while reviewing the hand-drawn prototype
(`book-figure-sketch-style-k110`): "investigate d2, excalidraw and any other
libraries that can do pure drawing (without layout), and completely separate
the process of text->diagram/layout and the subsequent styling." It feeds the
figure ports `book-figure-sequences-k111` through
`book-figure-settled-exit-status-k116`.

## The answer in brief

The tools the owner named, and the other mainstream diagram tools with a
hand-drawn look, all draw it with one small library, rough.js, or a port of it.
d2's `sketch` mode is a Go port of rough.js 4.6.6, and Excalidraw, draw.io and
Mermaid's `handDrawn` look use rough.js directly. (PlantUML's `handwritten`
option and the Typst package chalks have their own algorithms.) So "a library
that can do pure drawing" exists and is easy to run on its own: rough.js takes
coordinates and returns sketchy paths, with no layout and no browser, and it ran
under the Bun the repository already pins.

What does *not* separate cleanly is **text**. A layout is only correct for the
font that measured it, so the stage that draws the labels has to be the stage
that measured them, with the same font. Excalidraw measures text through a
browser canvas and offers only its own fonts (DejaVu Sans is not one of them).
d2 measures and draws text with its own embedded fonts inside one command and
has no command-line way to hand its layout out as data. Taking either as the
drawing stage would give up the book's pinned DejaVu text, measured layout, or
offline byte-reproducible render.

The recommendation is therefore to make the separation **inside the Typst
toolchain the book already has**. Each figure's layout module returns a *layout
record*: plain data giving positions, sizes, routes and label boxes. One shared
drawing module turns a record into the figure under a named style, such as
crisp or sketch. Typst can print that record as JSON (checked below), so it can
be committed and compared. Then a restyle can be shown not to move anything,
and a different drawing backend, rough.js included, can be attached later
without touching any layout. A three-stage chain was run end to end:
Typst layout → JSON → rough.js on Bun → Typst drawing. It was byte-identical
across runs, and its layout JSON was identical at every roughness level.

The owner's questions are at the end.

## The problem in the book's terms

**Layout** decides where things go: it measures every label in the pinned font,
sizes each box around its text, places boxes, routes lines and places line
labels so no text touches a line. Its natural output is an *intermediate*:
for each element, a role, a box or point list, and the text with the width it
was measured at.

**Drawing** (styling) decides how those things look: stroke weight and colour,
fills, arrowheads, dashes, and whether a line is ruled or wobbly. It should be
able to change for every figure at once without moving anything.

**What the book has now.** The Typst/CeTZ engine already separates the two by
module discipline, but not as an interface:

- Every stroke goes through the `draw-*` primitives in
  `docs/book/src/figures/_style.typ`, and a lint stops a figure calling CeTZ
  itself (`docs/book/figures.md`, "Draw through the primitives").
- `_sequence.typ` computes a full list of positioned `items` (heads, messages,
  self-calls, dividers, notes) before its canvas draws anything
  (`_sequence.typ`, the "vertical layout" and "draw" sections). That list is a
  layout intermediate, but it stays inside the function.
- k110's roughness switch lives inside the primitives. Roughness 0 renders
  byte-identically to the crisp figure, and no layout code reads the switch
  (k110 running log). So a restyle already leaves layout untouched in practice.
  The only proof is reading the code, though: there is no artifact to compare
  and no seam where another drawing library could attach.

**Constraints that stay** (`docs/book/figures.md`, k119 brief): offline, pinned
and byte-reproducible output; text laid out from measured metrics in pinned
DejaVu Sans (the owner kept DejaVu in k110), so no label touches a line; no JVM
or network at render time unless admitted; one editable source per figure; a
regenerate-and-compare check. The owner also chose roughness 2, crisp key boxes
and tables, and deferred the shipped default to this survey (k110 log).

## Findings at a glance

"Run apart" asks whether layout and drawing can be invoked separately with an
inspectable intermediate. "Walk-away" asks what is still legible with the tool
uninstalled.

| Candidate | Layout | Drawing | Run apart? | Licence | Weight | Walk-away |
|---|---|---|---|---|---|---|
| d2 0.9.0 | dagre (Go port), ELK (Go port), TALA (now bundled) | own SVG renderer; sketch = rough.js Go port | not from the CLI; only through its Go library | MPL-2.0 | 40 MB binary, already pinned | `.d2` text; layout only inside the SVG |
| Excalidraw | none (positions are stored in the scene) | rough.js, own fonts | yes: the scene JSON is the intermediate | MIT | React app; headless export needs jsdom+node-canvas or a browser | `.excalidraw` JSON: coordinates and text |
| rough.js 4.6.6 | none | sketchy paths from coordinates | it *is* only drawing | MIT | 31 kB self-contained bundle | none of its own; output is plain paths |
| svg2roughjs, graphviz-rough, Sketchviz | inherits the input SVG's | re-draws an SVG with rough.js | post-process only | MIT / Apache-2.0 / not stated | DOM or JVM | the input SVG |
| Mermaid 12.2.0 | ELK (default) or dagre | `look: classic / handDrawn / neo`, via rough.js | layout and look are separate settings; one render | MIT | headless Chromium via mermaid-cli | source text; excluded by the book's brief |
| PlantUML 1.2026.8 | Smetana / Graphviz | `handwritten` option | no | GPL jar | JVM | source text |
| Graphviz | dot, neato, ... | own renderers; no sketch | yes: `-Tjson` gives positions and splines | EPL (not re-checked) | native binary | DOT text; JSON layout |
| diagraph-layout (Typst) | Graphviz engines via WASM | none ("no means of rendering") | yes, returns a Typst dictionary | MIT | 451 kB package | Typst source |
| ELK / elkjs 0.12.0 | layered and others | none | yes: JSON graph in, JSON layout out | EPL-2.0 or GPL-3.0+ | 8 MB package; runs on Bun/Node | JSON graph |
| tldraw | none | own "draw" style | n/a | licence key required in production | browser editor | `.tldr` JSON |
| draw.io | own layouts or hand-placed | sketch style via rough.js | geometry and style are separate in the file | Apache-2.0 (not re-checked) | Electron for CLI export | XML with geometry and style strings |
| Typst + CeTZ (current) | the book's own modules, from measured text | CeTZ / native `curve` | yes, if layout returns data; `typst eval` prints it as JSON | Apache-2.0 / LGPL-3.0+ | already pinned | `.typ` source; layout JSON if committed |
| chalks (Typst) | none | Typst-native sketch engine (Rust→WASM) | it is only drawing | MIT | young: 0.1.1, Aug 2026 | Typst source; opaque WASM |

## The candidates

### d2 0.9.0

**Layout.** `d2 layout` (pinned binary, run 2026-10-17) lists three bundled
engines. dagre is "implemented natively in Go by Dagro", a port of Dagre 3.1.1.
ELK is "bundled through the native Go elk-go port" of ELK.js 0.12.0 Layered.
TALA is "D2's native layout and edge-routing engine". TALA's option
`--tala-seeds` gives "random seeds for deterministic TALA layout attempts". TALA
is new here: the
[v0.9.0 release notes](https://github.com/terrastruct/d2/releases/tag/v0.9.0)
say "TALA is now open source and bundled with D2 … No separate plugin
installation or license key is required." The archive's `LICENSE.txt` is
MPL-2.0. Its `THIRD_PARTY_NOTICES.txt` has no separate TALA entry, which fits
TALA being part of d2's own code. That TALA's licence is MPL-2.0 comes only from
[secondary coverage](https://runtimewire.com/article/terrastruct-open-sources-tala-alexander-wang-winds-down-business),
not from a primary source. d2 moved to Hack Club as a non-profit project
([D2 blog, 2026-09-05](https://d2lang.com/blog/tags/community/)). One
consequence: `docs/book/figures.md` still says TALA "is a separate proprietary
install". That is stale for d2 0.9.0 and belongs to the leaf that rewrites that
section (`book-figure-retire-plantuml-k117`).

**Drawing and sketch.** The pinned archive's `THIRD_PARTY_NOTICES.txt` says:
"Rough-go is a native Go compatibility port of the Rough.js 4.6.6 rendering
surface used by D2." d2's sketch mode is rough.js, reimplemented. A small sketch
diagram rendered twice (ELK, `env -i`) gave byte-identical SVGs. The `--font-*`
flags take TTF files (`d2 --help`), so DejaVu could be supplied. Whether d2 then
measures with it exactly as Typst does was not checked. k110 found the sketch look
"heavy, hatched … with d2's own font and layout".

**Layout as data.** The CLI writes svg, png, pdf, pptx, gif or txt (`d2 --help`)
and has no JSON layout output. The Go library does have one: in
[`d2target.go`](https://raw.githubusercontent.com/terrastruct/d2/master/d2target/d2target.go)
a `Shape` carries `pos`, `width`, `height`, `labelWidth`, `labelHeight`, and a
`Connection` carries `route` points, all with JSON tags. A small Go program
could therefore emit d2's layout as JSON. In principle it could also feed its
own positions to d2's renderer. That second use was not tried, and either one
adds a Go build to the book's toolchain.

**Walk-away.** The `.d2` source stays legible. The computed layout exists only
inside the SVG (a third-party tool,
[d2vision](https://pkg.go.dev/github.com/grokify/d2vision), parses it back out
of the SVG).

### Excalidraw

**Format.** A scene is JSON with `type`, `version`, `source`, `elements`,
`appState` and `files`
([JSON schema docs](https://docs.excalidraw.com/docs/codebase/json-schema)).
Each element carries its own geometry and style
([`types.ts`](https://raw.githubusercontent.com/excalidraw/excalidraw/master/packages/element/src/types.ts)):
`x`, `y`, `width`, `height`, `angle`, `strokeColor`, `backgroundColor`,
`fillStyle`, `strokeWidth`, `strokeStyle`, `roughness`, `opacity` and a `seed`
("Random integer used to seed shape generation so that the roughjs shape …").
Arrows carry `points` and `startBinding`/`endBinding`. The scene is a laid-out
drawing with style per element, which makes it a real layout/style intermediate.
It has no layout engine: positions come from a person in the editor or from a
converter such as
[mermaid-to-excalidraw](https://github.com/excalidraw/mermaid-to-excalidraw)
(MIT). That converter's README does not say how it derives positions; no
primary source on that was found.

**Sketch look.** Roughness has three named levels, architect 0, artist 1,
cartoonist 2
([`constants.ts`](https://raw.githubusercontent.com/excalidraw/excalidraw/master/packages/common/src/constants.ts)),
drawn by rough.js (`roughjs` is a dependency of `@excalidraw/utils`, npm).

**Text and fonts: the blocker.** `fontFamily` is an enumeration: Virgil,
Helvetica, Cascadia, Excalifont (the default), Nunito, Lilita One, Comic Shanns,
Liberation Sans, Assistant (`constants.ts`). DejaVu Sans cannot be named. Text
is measured with the browser canvas, `document.createElement("canvas")` and then
`measureText`
([`textMeasurements.ts`](https://raw.githubusercontent.com/excalidraw/excalidraw/master/packages/element/src/textMeasurements.ts)).
SVG export builds nodes with `document.createElementNS` and inlines fonts as
`@font-face`
([`export.ts`](https://raw.githubusercontent.com/excalidraw/excalidraw/master/packages/excalidraw/scene/export.ts)).

**Headless export** exists only as third-party tools.
[excalidraw-to-svg](https://github.com/JRJurman/excalidraw-to-svg) emulates the
DOM with jsdom. [excalidraw_export](https://github.com/Timmmm/excalidraw_export)
depends on the native `node-canvas`, and by default its fonts load from
excalidraw.com unless `--embed-fonts` is given.
[excalidraw-brute-export-cli](https://github.com/realazthat/excalidraw-brute-export-cli)
drives headless Firefox through Playwright, whose authors say this avoids "bugs
that happen with other projects that attempt to export by emulating the DOM".
Weight: `@excalidraw/excalidraw` 0.18.1 unpacks to 47 MB and `@excalidraw/utils`
0.1.5 to 97 MB (npm registry, 2026-10-17). Licence MIT.

**Walk-away.** Good: the JSON keeps every coordinate and string, and anyone can
redraw it. That is Excalidraw's most borrowable idea: per-element `roughness`
and `seed` stored *beside* the geometry.

### rough.js 4.6.6

The library itself: "a small (<9 kB) graphics library … defines primitives to
draw lines, curves, arcs, polygons, circles, and ellipses. It also supports
drawing SVG paths" (package README); MIT
([GitHub](https://github.com/rough-stuff/rough)). Its `RoughGenerator` returns
drawables without a canvas or SVG element, and `toPaths()` returns SVG path
data (`bundled/generator.d.ts`). The
[options wiki](https://github.com/rough-stuff/rough/wiki) documents `seed`
("useful for creating the exact shape when re-generating with the same
parameters … If seed is not defined, or set to 0, no seed is used"),
`roughness` ("A rectangle with the roughness of 0 would be a perfect
rectangle"), `bowing`, `disableMultiStroke` and `preserveVertices`.

**Observed here.** The npm tarball's `bundled/rough.cjs.js` is 31 kB and has no
`require()` (its four dependencies are bundled in). Under the pinned Bun 1.4.2
with `env -i`, no DOM, a fixed seed gave byte-identical output on three runs.
See the evidence section. At default settings each edge is drawn twice
("multi-stroke"), so the look is sketchier than k110's single wobble;
`disableMultiStroke` gives a single pass.

**Walk-away.** Nothing of its own to read; it leaves plain SVG paths.

### SVG post-processors: svg2roughjs, graphviz-rough, Sketchviz

[svg2roughjs](https://github.com/fskpf/svg2roughjs) (MIT, 3.2.3) re-draws an
SVG's shapes with rough.js. It has a numeric `seed` "for reproducible output",
`roughConfig`, and a `fontFamily` option that defaults to
`'Comic Sans MS, cursive'` and keeps the original font when set to `null`. Every
example in its README runs in a browser. No source states that it runs headless,
or how it treats `<use>` elements. The Java
[graphviz-rough](https://github.com/nidi3/graphviz-java) `Roughifyer` (Apache-2.0)
and the [Sketchviz](https://www.johndcook.com/blog/2019/11/13/hand-drawn-graphviz-diagrams/)
site do the same for Graphviz output.
[sketchviz-docker](https://github.com/David-Mao/sketchviz-docker) is "a script
using RoughJS to 'draw' the graphs".

**Why this fits the book badly.** Typst 0.15.1 writes every glyph as an outline:
a figure SVG from the k107 trial contains `<use>` elements that point at glyph
`<symbol>`s, plain `<path>` strokes, and no class or group marking which is
text (inspected 2026-10-17). A post-processor has to guess which paths are
letters. It also sees only paths, not roles, so a rule such as "key boxes stay
crisp" (the owner's k110 choice) cannot be expressed. And it ties the style
stage to Typst's SVG layout, which is not a stable interface.

### Mermaid

Mermaid's configuration separates the two settings, `layout` (ELK by default,
dagre available) and `look` (`classic`, `handDrawn`, `neo`). `handDrawnSeed`
"Defines the seed to be used when using handDrawn look" and its default 0 "gives
a random seed"
([config schema](https://mermaid.js.org/config/schema-docs/config.html),
[layouts](https://mermaid.js.org/config/layouts.html)). The `mermaid` 12.2.0
package is MIT and depends on `roughjs ^4.6.6` (npm registry, 2026-10-17). It
renders in a browser: `@mermaid-js/mermaid-cli` 12.0.0 has a peer dependency on
`puppeteer` (npm). The book's node brief already rules it out ("No figure
relies on Mermaid", `_ravel-book.md`), so it counts only as prior art: a
mainstream tool made layout and look independent settings, and seeded its
randomness.

### PlantUML (from k110)

k110 tried `skinparam handwritten true` on real figures. It wobbles every shape
and keeps colours and DejaVu, but its roughness is fixed, its curved edges come
from its layout, the key wobbles too, and it needs Java. Layout and style cannot
be separated. Nothing to add.

### Graphviz, and Graphviz inside Typst

Graphviz writes layouts as data: `-Tjson` "contains the same information
produced by -Txdot", including node `pos`, edge spline `pos`, the bounding box
and drawing operations
([output formats](https://graphviz.org/docs/outputs/json/)). It has no sketch
renderer of its own; the search found only the third-party post-processors
above.

Inside Typst, [diagraph](https://typst.app/universe/package/diagraph) (0.3.7,
MIT) runs Graphviz as a WebAssembly plugin and lets Typst typeset node labels.
A related package, [diagraph-layout](https://typst.app/universe/package/diagraph-layout)
(0.0.1, MIT, 451 kB), "only exposes the layout engines and provides no means of
rendering the graphs itself". It takes nodes with sizes in points and returns
each node's `x`, `y`, `width`, `height` and each edge's control `points` and
label positions. It warns that "The API is not stable yet" and that output order
"is not guaranteed to be the same as the inputting order". This is a pure layout
engine running inside the pinned Typst, fed with sizes Typst has measured, which
is the shape the book wants. The engine is young.

### ELK and dagre

[elkjs](https://github.com/kieler/elkjs) "is a graph layout engine *only*" and
"computes positions for the elements of a diagram". It runs in Node without a
browser. The npm package (0.12.0) is "EPL-2.0 OR GPL-3.0-or-later" and unpacks
to 8 MB. Its [JSON format](https://eclipse.dev/elk/documentation/tooldevelopers/graphdatastructure/jsonformat.html)
gives nodes, ports and labels a location and size, and edges `sections` with
`startPoint`, `endPoint` and `bendPoints`. The format page says: "layout
algorithms generally don't perform any size estimation of the text. Therefore
you should specify a reasonable width and height". So label sizes have to come
from the stage that owns the font, Typst here.
[dagre](https://www.npmjs.com/package/@dagrejs/dagre) 3.1.1 is MIT and also
layout-only.

### tldraw

The tldraw SDK "requires a license key to work in production", and without one
it "stops rendering the editor" after five seconds
([license key](https://tldraw.dev/sdk-features/license-key)). It is a browser
editor without a layout engine; its licence and runtime rule it out.

### draw.io

draw.io's sketch style "uses rough.js" and applies "to all shapes and
connectors" when chosen with nothing selected, or through the `rough=1` URL
parameter ([blog](https://www.drawio.com/blog/rough-style),
[manual](https://www.drawio.com/docs/manual/styles/rough-style/)). Its file
stores each cell's geometry apart from its style string, another instance of
the Excalidraw idea. CLI export needs the Electron desktop app (not re-checked
here), which rules it out for the book's render.

### Typst-native pieces

- **Layout out as data.** `typst eval` "can be used to introspect the document"
  ([query docs](https://typst.app/docs/reference/introspection/query/)). A
  figure that puts `metadata(...)` with a label in its output can have that
  value printed as JSON:
  `typst eval 'query(<layout>).first().value' --in fig.typ`. Run here with the
  pinned Typst and DejaVu, offline, it printed measured widths
  (`"w":188.91796875`) byte-identically twice. (Typst 0.15.1 marks
  `typst query` deprecated in favour of `eval`.)
- **Drawing given paths.** Typst's native
  [`curve`](https://typst.app/docs/reference/visualize/curve/) has `move`,
  `line`, `quad`, `cubic` and `close`, which map one to one onto rough.js's
  `move`, `lineTo` and `bcurveTo` operations. CeTZ 0.5.0 added `svg-path`, an
  element "that accepts a subset of SVG commands"
  ([CHANGES.md](https://raw.githubusercontent.com/cetz-package/cetz/master/CHANGES.md)).
  The pinned 0.4.2 does not have it (checked in the package cache). No CeTZ
  release mentions a sketch or random decoration; its decorations remain brace,
  zigzag, wave and coil.
- **[chalks](https://typst.app/universe/package/chalks)** (0.1.1, MIT, Typst
  ≥0.14, first released 2026-08-07) is a Typst-native hand-drawn engine. Its
  shapes are perturbed by a bundled Rust→WASM plugin into "sketchy,
  variable-width filled outlines", and seeds are "derived deterministically from
  input geometry". It draws its own primitives only, not existing CeTZ output.
  It is very young
  ([27 commits, 8 stars](https://github.com/GiggleLiu/chalks)), and its
  strokes become filled outlines, which changes the book's stroke-weight rules.
- **[fletcher](https://typst.app/universe/package/fletcher)** (0.5.8, MIT) and
  **[chronos](https://typst.app/universe/package/chronos)** (0.3.0, Apache-2.0)
  are CeTZ-based diagram packages, for node-and-arrow diagrams and sequence
  diagrams. Each does layout and drawing in one call, like the current engine,
  so neither adds a separation.

### Prior art for "style as a filter on every path"

These are not candidates, but they show that the k110 design (one jitter
applied to every stroke at the drawing layer) is a well-trodden pattern:

- TikZ's `random steps` decoration: "each step is randomly shifted a little
  bit" ([PGF manual](https://tikz.dev/library-decorations)).
- matplotlib's `set_sketch_params(scale, length, randomness)`: "The amplitude of
  the wiggle perpendicular to the source line" applied to any artist
  ([docs](https://matplotlib.org/stable/api/_as_gen/matplotlib.artist.Artist.set_sketch_params.html)).
- SVG's own `feTurbulence` + `feDisplacementMap` filter, seeded by the `seed`
  attribute ([MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/seed)),
  applied by the viewer at display time.

And two designs that separate content from presentation at the *model* level:

- [Penrose](https://penrose.cs.cmu.edu/docs/ref) separates Domain, Substance
  and Style, but its Style "specifies shapes and diagram layout"
  ([Style overview](https://penrose.cs.cmu.edu/docs/ref/style/overview)). Its
  layout lives in the style program, which is the coupling the owner wants to
  remove.
- [Structurizr](https://docs.structurizr.com/dsl/cookbook/themes/) separates
  the model, views and styles/themes, but its styles "may not be fully supported
  by the PlantUML, Mermaid, etc export formats". Layout remains the exporter's.

## Architecture options

Each option is judged against the constraints above and by what it changes in
the port leaves. "Record" means the layout record described in option B.

**A. Keep one pass; the style switch stays inside the primitives (k110 as
built).** Restyling already happens in one file and leaves layout alone in
practice. There is no layout artifact, so "nothing moved" is shown only by
reading code, and no other drawing library can attach. *Ports:* unchanged; each
leaf draws through `_style.typ`. *Cost:* none. *Falls short of* "completely
separate the process".

**B. Two stages inside Typst, joined by a layout record.** Each layout module
(`_sequence.typ` and the coming `_c4.typ`, `_state.typ`, `_deployment.typ`)
returns plain data. That data gives the canvas size and, per element, an id, a
role (`ravel`, `external`, `note`, `key`, `relation`, `result`, ...), a box or
point list, dash and arrowhead, and each label's text, anchor point and measured
width. A single `_draw.typ` renders any record under a named style: `crisp`, or
`sketch` with a roughness. Style rules live there and nowhere else, such as
"keys and tables are never sketched" (the owner's k110 choice). Typst prints a
record as JSON through `typst eval`, so the figures task can also write and
compare `<figure>.layout.json`.

- *For:* one tool, so text is still measured and drawn by the same typesetter
  in DejaVu. Offline and pinned exactly as today. A restyle provably leaves
  every layout JSON unchanged. Layouts become reviewable as data diffs. The
  record is the seam where a different drawing backend (option C) or layout
  engine (option E) can attach later.
- *Against:* one more module and a small schema to keep stable. The lint rule
  "only `_style.typ` calls CeTZ" becomes "only `_draw.typ` draws". The figures
  directory gains JSON files if the records are committed.
- *Ports:* k111 moves `_sequence.typ`'s draw loop into `_draw.typ` and returns
  its `items` as a record (the loop is already separate, so this is mostly a
  move). k112–k114 write their modules as record producers. k115 and k116
  produce records whether placed by hand or computed. k117 documents the record
  and the JSON check in `figures.md`. k118 is unchanged. Defining the record and
  `_draw.typ` is a small up-front piece, either folded into k111 or a leaf
  before it.

**C. B plus an external drawing stage: rough.js on Bun.** The record goes to a
small Bun script that asks rough.js for each element's sketch operations, and
Typst draws those curves and the text. The chain ran end to end (evidence
below).

- *For:* the widely used rough.js look (the same algorithm as d2, Excalidraw
  and draw.io), with hachure fills available. Bun is already pinned and fetched
  by `task setup`, and the worker already installs with
  `bun install --frozen-lockfile --ignore-scripts`.
- *Against:* two runtimes and an extra intermediate per figure. A new lockfile
  and its controls. It is a different look from the one the owner approved in
  k110: two passes per edge at default settings. Cross-host byte-identity of
  rough.js's floating-point output under JavaScriptCore was not checked; the
  same caveat applies to k110's Typst `calc.cos` on Linux, which is also
  unchecked.
- *Ports:* as B, plus a figures-pipeline leaf for the Bun stage.

**D. Render crisp Typst SVG, then post-process it into a sketch.** Rejected.
Text arrives as glyph outlines with no role marking, so a post-processor must
guess what is a letter. Roles such as "key" are lost. svg2roughjs needs a DOM
according to its README. The approach depends on Typst's SVG structure, which is
not an interface.

**E. An external layout engine for graph-shaped views, feeding the same
record.** ELK through elkjs on Bun, or Graphviz through diagraph-layout inside
Typst, given sizes Typst has measured, returns positions and routes. This is
orthogonal to the style question and answers k115's open choice ("hand-place
coordinates or compute a simple layered layout") with concrete engines. *Ports:*
only k115 and k116, and only if hand placement proves costly. *Against:*
diagraph-layout is 0.0.1 and unstable. elkjs brings EPL code and the Bun stage
of option C. Both need edge-label placement checked against the "no label
touches a line" rule.

**F. d2 lays out; Excalidraw or rough.js draws.** Rejected for this book. d2's
layout is reachable as data only by writing a Go program against its library. An
Excalidraw render needs a DOM and a canvas, or a headless browser, and cannot
use DejaVu. Excalidraw text would be re-measured by a browser canvas, breaking
"laid out from measured metrics". The notations the book uses (UML sequence and
state, C4) are not d2's strength.

**G. Excalidraw files as the editable source.** Rejected. The layout would be
placed by hand in an editor, with no text-to-diagram step, no DejaVu and no
regenerate-and-compare. It is the opposite of the separation asked for.

**H. A view-time SVG filter.** Rejected. Each reader's renderer would apply its
own distortion, so the reproducible artifact is no longer what readers see. The
filter distorts text unless scoped to stroke groups that Typst does not emit.
Its rendering on GitHub and in editor previews is unverified.

## Recommendation

Adopt **B**. Keep the style stage in Typst, carrying the owner's approved look:
k110's jitter at roughness 2, DejaVu, keys and tables crisp. Commit each
figure's layout record as JSON so the figures check can show that a restyle
moves nothing. Keep **C** in reserve: the record makes it a drop-in backend,
and the chain has been shown to work. Adopt it only if the owner, having seen
rough.js on a real figure, prefers that look. Use **E** in k115 only if the
first state view proves expensive to place by hand.

Why B over C now: B delivers the separation the owner asked for (an explicit
layout artifact and one place that draws) without a second runtime, and it keeps
the look the owner has already chosen. C's only gain is a particular sketch
algorithm, and B leaves that choice open rather than closing it.

## Questions for the owner

1. **How far to separate.** B (one tool, two stages, a layout record between
   them) is recommended. The alternatives are C (also hand drawing to rough.js
   on Bun) and A (keep k110's switch inside the primitives, with no record).
2. **Commit the layout records?** Recommended: yes. Each figure gets a
   `<figure>.layout.json` beside its SVG, compared by `task docs:figures`, so a
   restyle's diff shows SVG changes and no layout changes. The cost is one more
   file per figure and a change to the figures directory's rules.
3. **Which sketch look, and the shipped default.** Recommended: k110's jitter at
   roughness 2, the look already approved. Offered: a side-by-side of rough.js
   on the two trial figures before deciding. Still open from k110: whether the
   book ships crisp or sketched by default.
4. **How the ports pick this up.** Recommended: a short planning leaf ahead of
   k111 that defines the record and `_draw.typ` and re-cuts k111–k116 against
   it. The alternative is to fold that into k111. Either way, k111–k116 should
   not start under the old shape.

**The owner's answers, 2026-10-17:** all four as recommended. B; commit and
check `<figure>.layout.json`; k110's jitter at roughness 2, shipped sketched by
default, keys and tables crisp; a planning leaf ahead of k111
(`book-figure-layout-record-k120`).

## Evidence: local observations (2026-10-17, macOS arm64)

All runs used the pinned tools under `target/tools` with `env -i`. Scratch
files lived in the session scratchpad and are not committed; the scripts are
reproduced below so the observations can be repeated.

| Observation | Result |
|---|---|
| `d2 --version`, `d2 layout`, `d2 layout tala/elk/dagre`, `d2 --help` | v0.9.0; three bundled engines as quoted; no JSON output format |
| d2 archive `d2-v0.9.0-macos-arm64.tar.gz`: `LICENSE.txt`, `THIRD_PARTY_NOTICES.txt` | MPL-2.0; Dagro (MIT), Rough-go port of Rough.js 4.6.6 (MIT), elk-go (EPL-2.0); no TALA entry |
| d2 sketch diagram (ELK, `sketch: true`) rendered twice | byte-identical SVGs |
| `typst eval 'query(<layout>).first().value'` on a measuring figure, twice | identical JSON; `"w":188.91796875` for "Ravel CLI starts the worker" at 14pt DejaVu |
| rough.js 4.6.6 tarball | SHA-256 `373873b2c3f1b7ddba878525c774816057c5818029780c7fba081f06018ba78c`; `rough.cjs.js` 31 kB, no `require(` |
| rough.js generator under Bun 1.4.2, seed 42, three runs | output SHA-256 `602cd156…2067` each time |
| Three-stage chain, roughness 1, run twice | layout JSON `42e321f3…b866`, rough JSON `fbb96ecd…ea8e`, SVG `2634d496…09f7` both times; SVGs `cmp`-identical |
| Same chain at roughness 0 and 2 | layout JSON unchanged (`42e321f3…b866`); only rough JSON and SVG change |
| Typst 0.15.1 SVG (`trial-typst/out/process-authority-long.svg`) | text is `<use>` references to glyph outlines held as `<symbol>` paths in `<defs>`; drawn strokes are top-level `<path>`s; the file has no `class` attribute |
| CeTZ 0.4.2 package cache | no `svg-path`; path primitives `line`, `bezier`, `catmull`, `hobby`, `merge-path` |

**Not established.** The chain ran only on macOS arm64. It drew two boxes and
one line without arrowheads, enough to prove the joins, not a figure port.
Cross-host identity of rough.js output, svg2roughjs headless use, d2's renderer
fed external positions, mermaid-to-excalidraw's positioning, and the licences
marked "not re-checked" were not verified.

**Searches that found nothing** (recorded so they are not re-run): a primary
source naming TALA's licence; a CeTZ or Typst sketch mode other than chalks; a
d2-to-Excalidraw converter; a Graphviz built-in sketch renderer; any statement
of how mermaid-to-excalidraw computes positions.

<details>
<summary>The three-stage feasibility scripts</summary>

Stage 1, `layout.typ` (layout only; emits geometry as data):

```typst
#set text(font: "DejaVu Sans", size: 14pt)
#context {
  let pad = 11
  let size(t) = { let m = measure(box(t)); (w: m.width / 1pt + 2 * pad, h: m.height / 1pt + 2 * pad) }
  let a = size([Ravel CLI])
  let b = size([Policy worker])
  let gap = 160
  let boxes = (
    (id: "cli", role: "ravel", x: 0, y: 0, w: a.w, h: a.h, label: "Ravel CLI"),
    (id: "worker", role: "external", x: a.w + gap, y: 0, w: b.w, h: b.h, label: "Policy worker"),
  )
  let arrows = ((id: "eval", pts: ((a.w, a.h / 2), (a.w + gap, b.h / 2)), label: "evaluate policy", lx: a.w + gap / 2, ly: a.h / 2 - 6),)
  [#metadata((width: a.w + gap + b.w, height: calc.max(a.h, b.h), boxes: boxes, arrows: arrows)) <layout>]
}
```

Stage 2, `sketch-stage.js` (style only; seeds from element ids), run as
`bun sketch-stage.js rough.cjs.js layout.json <roughness>`:

```js
const rough = require(process.argv[2]);
const layout = JSON.parse(require("fs").readFileSync(process.argv[3], "utf8"));
const roughness = Number(process.argv[4] || 1);
const seedOf = s => { let h = 7; for (const c of s) h = (h * 31 + c.codePointAt(0)) % 2147483629; return h || 1; };
const g = rough.generator();
const ops = d => d.sets.map(s => ({ type: s.type, ops: s.ops.map(o => [o.op, o.data]) }));
const out = { boxes: {}, arrows: {} };
for (const b of layout.boxes) out.boxes[b.id] = ops(g.rectangle(b.x, b.y, b.w, b.h, { seed: seedOf(b.id), roughness, fillStyle: "solid", fill: "#000" }));
for (const a of layout.arrows) out.arrows[a.id] = ops(g.linearPath(a.pts, { seed: seedOf(a.id), roughness }));
process.stdout.write(JSON.stringify(out));
```

Stage 3, `draw.typ` (draws the style stage's curves, and the text at the
layout's measured positions):

```typst
#let L = json("layout.json")
#let R = json("rough.json")
#let pal = (ravel: (fill: rgb("#1E4F78"), text: white, line: rgb("#163A59")), external: (fill: rgb("#E8EDF2"), text: rgb("#172B3A"), line: rgb("#64748B")))
#set page(width: auto, height: auto, margin: 12pt, fill: white)
#set text(font: "DejaVu Sans", size: 14pt)
#let path-of(sets, kind) = {
  let parts = ()
  for s in sets.filter(s => s.type == kind) {
    for (op, d) in s.ops {
      if op == "move" { parts.push(curve.move((d.at(0) * 1pt, d.at(1) * 1pt))) }
      else if op == "lineTo" { parts.push(curve.line((d.at(0) * 1pt, d.at(1) * 1pt))) }
      else if op == "bcurveTo" { parts.push(curve.cubic((d.at(0) * 1pt, d.at(1) * 1pt), (d.at(2) * 1pt, d.at(3) * 1pt), (d.at(4) * 1pt, d.at(5) * 1pt))) }
    }
  }
  parts
}
#box(width: L.width * 1pt, height: L.height * 1pt, {
  for b in L.boxes {
    let k = pal.at(b.role)
    place(top + left, curve(fill: k.fill, stroke: none, ..path-of(R.boxes.at(b.id), "fillPath")))
    place(top + left, curve(stroke: 1.5pt + k.line, ..path-of(R.boxes.at(b.id), "path")))
    place(top + left, dx: b.x * 1pt, dy: b.y * 1pt, box(width: b.w * 1pt, height: b.h * 1pt, align(center + horizon, text(fill: k.text, b.label))))
  }
  for a in L.arrows {
    place(top + left, curve(stroke: 1.5pt + rgb("#334155"), ..path-of(R.arrows.at(a.id), "path")))
    place(top + left, dx: a.lx * 1pt - 70pt, dy: a.ly * 1pt - 20pt, box(width: 140pt, align(center, text(size: 12pt, a.label))))
  }
})
```

</details>
