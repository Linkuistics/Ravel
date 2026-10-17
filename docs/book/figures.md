# Figures: layout, renderer and regeneration

This note settles how a figure sits on disk, how it is drawn and how it is
regenerated, so the art stage and every drafting session use one layout. The
contract (`contract.md`, "Format and media") owns *which* notations and which
renderer the book uses; this note owns the mechanics. It sits beside the
contract and is not part of the published book.

Every figure is a Typst source drawn with the CeTZ package. The book used
other, notation-specific tools until the owner's decision of 2026-10-17 to
migrate every figure. The migration, its effort and what changed for each figure
are recorded under "Porting record" below, and the rules for the former tools
went with them; they are named only there, in the viewing records and in the
evidence directory it links.

## Layout on disk

```text
docs/book/src/figures/
  system-context.typ          the figure's only editable source
  system-context.svg          what the source renders to, committed
  system-context.layout.json  the layout record the source evaluates to, committed
  _style.typ                  shared modules, never rendered on their own
  _layout.typ
  _draw.typ
  _c4.typ  _sequence.typ  _state.typ  _deployment.typ
```

- One diagram per source file, flat in `docs/book/src/figures/`. The SVG and the
  layout record have the source's name with `.svg` and `.layout.json`. There is
  exactly one editable source per figure: a second source in any other notation
  beside a `.typ` is refused.
- Name a figure for its subject, not its chapter: several chapters reuse one
  figure (the contract's figure list puts the system context in chapters 1 and 2
  and the container view in 2 and 7).
- A file whose name starts with `_` is a shared module that figures import
  (`#import "_sequence.typ": *`). It is not a figure and gets no SVG and no
  layout record. This is where the book's visual vocabulary lives.
- The directory holds `.typ`, `.svg` and `.layout.json` files only. Anything
  else, a subdirectory, an SVG with no `.typ` figure, or a `.layout.json` with no
  `.typ` figure beside it (a module does not count) is refused, so the compare
  covers every file there.
- A figure made only of the page's own text, such as a table, needs no file.

## How a page uses a figure

```markdown
![A caller starts a session. Ravel launches the agent program.](figures/system-context.svg)
```

The path is relative to the chapter, which sits in `docs/book/src/`, so it
resolves in the source tree (GitHub's file view, an editor preview) and in the
mdBook output alike. The alt text is written in the page, not in the SVG, and
says what the figure shows: the SVG holds its text as paths (see "Text in the
SVG"), so the page's alt text, caption and prose are the text alternative.
mdBook copies every non-Markdown file in `src/` into the built book, so the
editable sources and layout records are published beside the SVGs.

## Visual vocabulary

All figures use the palette and scale in `src/figures/_style.typ`, and
`_draw.typ` turns each element's role into strokes with them. Individual figures
select semantic roles; they do not redefine fonts, borders or spacing.
Page-native tables keep mdBook's normal styling.

| Role | Fill / text / border |
|---|---|
| Ravel system, process, linked component or executable; Ravel work phase in a state view | Blue `#1E4F78` / white `#FFFFFF` / navy `#163A59` |
| External caller or managed process; other installed file; final outcome in a state view | Grey `#E8EDF2` / ink `#172B3A` / slate `#64748B` |
| Human owner in the context view | Slate `#475569` / white / dark slate `#334155` |
| Temporary result channel | Gold `#FFF1CF` / brown `#3D321A` / ochre `#8A6B26` |
| Notes, keys and host enclosure | Pale grey `#F8FAFC` / ink / slate |
| Ordinary relation and state entry marker | Dark slate `#334155`; relation text is ink |
| Opaque-result path or successful result-delivery route | Green `#135E50`, also identified by labels |
| Canvas and installation directories | White; directory labels are ink and borders slate |

| Property | Shared choice and notation-specific reason |
|---|---|
| Font family | Pinned DejaVu Sans, bold for titles and entity names; DejaVu Sans Mono for explicit code styling. These are the only faces on a render's font path. Typst writes every glyph as a path, so the SVG names no font and a reader's browser needs none. |
| Type scale, in SVG units | Body, relations, keys and notes 14; titles, participant and entity names and enclosure heads 16; C4 type and technology qualifiers 12. State names stay 14 because they are short work and outcome labels. |
| Main strokes | Entity, boundary, note, key and arrow shafts 1.5. Sequence lifelines are 0.5 to keep them behind messages, and paired separator rules 0.75. |
| Arrow forms | One arrowhead (a filled "stealth" head, 9 by 7) is shared by every relation, message and reply, in the line's colour. A reply and the deployment dependency are dashed; a C4 boundary's outline is dashed in longer dashes. Colour and shaft weight are shared. |
| Spacing | Each layout module measures its text in the pinned font and sizes boxes, columns and rows from it. A figure source names only gaps and widths where it needs them (`col-gap`, `row-gap`, `gap`, `pad`, `limit`, `max-label`, `min-w`); only the C4 layout takes positions (an entity's `cx`, `top` and width). Figures are checked at their final rendered size. |
| Canvas | White, with a 6-point margin and no shadows. Keys have an explicit pale fill and dark text, including when the image sits on a dark page. |
| Style | Sketched at roughness 2 (see "Layout, then draw"). Keys are never sketched; text is never moved. |

These sizes are source-space units. mdBook scales a wide SVG to the chapter
column; the full-size link preserves access to its original scale. Do not
shrink individual text to force a figure into the column.

Each figure has an adjacent sentence saying what the reader should take from
it, a book-specific key, meaningful Markdown alt text and a full-size SVG link.
Explain the book's meanings and limits, not generic diagram notation. Colours
supplement labels. Keep colour descriptions in the key, without a second copy
in the adjacent prose. Each key stays with its figure at full size. The
surrounding prose supplies an alternative to the image. Labels, keys, captions
and alt text follow `ste.md` and the contract's STE writing rules.

### What each view means

| View | Book-specific meaning to preserve |
|---|---|
| `system-context` | Ravel between a caller, such as an orchestrator, and a harness that runs an LLM session or another selected program; the owner controls command selection. The sent result asks Ravel to stop, and return follows confirmed cleanup. Responsibility boundaries are not security boundaries. |
| `process-authority` | Processes and temporary files across a session, not simultaneous lifetimes or OS containers. The worker response and opaque result use separate paths. Worker end, reap and private-directory removal precede the managed launch. The runner runs inside the Ravel CLI. |
| `successful-session` | The unconfined successful path. An X ends a pictured lifetime only after confirmed reap or directory removal, not process exit. The sent result precedes cleanup. Only the last result arrow delivers to the caller. Terminal recovery applies only when the terminal was lent. |
| `session-outcomes` | Work phases and final outcomes. Cleanup is attempted on every path; an attempt need not succeed. The only green route requires a valid result, no cancellation, confirmed cleanup and a successful final write. The diagram groups operations rather than giving the full precedence algorithm. |
| `settled-exit-status` | Outcome precedence for `start-session`, not implementation call order: cancellation, original failure, required cleanup, final write. Cleanup is attempted before reporting. An original failure keeps its status if later cleanup fails. The filename is retained for stable links. |
| `selection-cancellation`, `supervision-cancellation` | Direct INT delivery to the Ravel CLI in the examples, not keyboard delivery. Cleanup steps are attempts and no X promises confirmed removal. Cancellation suppresses the result. HUP/TERM use the same rule. |
| `source-components` | The Ravel CLI's checking/final-outcome code and linked runner share one process; the worker is separate. The enclosing boundary means process membership, not confinement. No third runner executable is installed. |
| `installed-files` | Keep the CLI and worker from one build together. Outlines group installation directories; a file group lists separate files. Adjacent source edits do not replace embedded modules. Archive-only README and licence remain identified. |

Chapters 1–6 also use page-native tables for the sent result through cleanup and
return, responsibilities, result versus
exit status versus work, platform evidence, nested result authority, worker
startup, policy evaluation order, policy-context delivery, environment grants,
confinement, streams, route choices, import resolution and lookup fields.
Their adjacent sentences state the comparison, mapping or sequence the reader
can use. They need no separate figure source.

## What is recorded, and where

| Fact | Recorded in | Kept honest by |
|---|---|---|
| Renderer versions | `scripts/figures.sh` (one constant and one digest per tool and package); the contract names the versions | the up-front version check (Typst stamps no version into its SVG, so this is the only place a different Typst is named), the package tree digests and the font digests, all checked before any render |
| Layout | the figure source and the layout module it calls | `<figure>.layout.json`, the record the source evaluates to, compared apart from the SVG |
| Style settings | `_style.typ` (palette, scale, weights) and `_draw.typ` (drawing, sketch); the shipped look is compiled in | the SVG compare: a restyle changes the SVG and not the record |
| Regeneration command | here: `task docs:figures:update` rewrites every SVG and record; `task docs:figures` regenerates into a scratch directory and fails on any difference | the task is the command |

Recording each version once, in the script, and checking it before every render
avoids a per-figure copy that would rot.

## Layout, then draw

A figure source builds a **layout record** with a layout module and passes it to
`render` in `_draw.typ`. The record is plain data: positions, roles and measured
text, and no colour, weight, font size or roughness, so a restyle cannot change
it. Its shape is documented in `_draw.typ`'s header. `render` draws it and
attaches it to the page as `metadata(record) <layout>`, which `typst eval
'query(<layout>).first().value' --in FIG.typ` prints as JSON. `scripts/figures.sh`
writes that print as `<figure>.layout.json`, pretty-printed so a change is a
readable diff, and compares it apart from the SVG, so a layout change (both
files change) and a restyle (only the SVG changes) read differently. A record
holding a colour, a length, a piece of content or a style key is refused, so the
committed record cannot drift with a restyle.

`_draw.typ` is the only module that draws and the only one that imports CeTZ, so
how every stroke is drawn can change in that one place. The lint refuses a
figure source that imports a package itself and any other module that imports
CeTZ; a layout or style module may import another pinned package.

**Styles.** `crisp` is roughness 0 and draws the exact geometry. `sketch`
wobbles every outline, line, arrow shaft and cylinder by up to 0.6, 1.2 or 1.8
points at roughness 1, 2 or 3. **The shipped default is sketch at roughness 2,
with keys and page tables crisp**, as the owner decided on 2026-10-17 ("Roughness
2 is good"; DejaVu stays, with no handwriting face; key boxes crisp). The wobble
is a pure function of the figure's name and each element's own geometry, never a
clock or a random source, so two renders are byte-identical, and
`scripts/figures.sh` passes only `--input figure=NAME`: the look is compiled in.
`--input style=crisp` or `--input roughness=N` select another for review and
are not used by the check.

**Held in reserve: rough.js on Bun.** The layout record is also the seam where a
different drawing backend could be attached. rough.js, run on Bun, which `task
setup` already fetches, would take the record's shapes and draw them in its own
hand-drawn style; the survey that proposed it is
[`docs/research/layout-draw-separation-a.md`](../research/layout-draw-separation-a.md).
It is not adopted: nothing in a figure needs a look `_draw.typ` cannot give, and
it would add a second renderer to pin and keep reproducible. The record needs no
change to allow it.

**Typst's markup trap.** A label that starts `2. ` is parsed as a numbered list
item, not text. In the trial it drew the label as a list. Write `2\.` in the
figure source (`[2\. Launches]`); the record holds the string `2. Launches`. The
two numbered relations in `process-authority` need it. Under the layout record
the mistake is loud: `_layout.typ` accepts only text, spaces and line breaks in a
label, and the render stops with `a label may hold only text, ... not
item(number: 2, ...)` (seen by removing the backslash from a scratch copy).

## Fetched, pinned and offline

| Tool | Pin | Fetched by `task setup` |
|---|---|---|
| Typst | 0.15.1, the host's static release archive (macOS arm64, Linux arm64, Linux amd64) | into `target/tools/typst`, archive SHA-256 checked, version checked before every run |
| CeTZ | 0.4.2 and its one dependency oxifmt 1.0.0, from the Typst package registry | unpacked into `target/tools/typst-packages/preview`, tarball SHA-256 and unpacked-tree digest checked |
| DejaVu fonts | 2.37, upright Sans, Sans Bold, Sans Mono and Sans Mono Bold | into `target/tools/fonts/dejavu-2.37`, archive and each face checked |

Setup needs only the host's `curl`, `tar` (with xz and bzip2 support) and a
SHA-256 utility; no language runtime is needed to set up, check or update a
figure.

**Why these versions, checked 2026-10-17.** Typst 0.15.1 (published 2026-07-17)
was the latest release. CeTZ 0.4.2 is what the trial drew with; 0.5.2 exists
and is not adopted, because moving up means re-trialling the primitives and is
a decision of its own. The Typst digests equal the ones GitHub lists. The
registry lists no digests, so the package digests were computed from the
download, and the unpacked trees matched the copy Typst had fetched itself. The
DejaVu release lists no digest either; the archive and face digests were
computed from the download on 2026-10-16.

**Fetched, not vendored.** CeTZ is LGPL-3.0-or-later and ships a WebAssembly
plugin; this repository is Apache-2.0, and nothing else the book needs is
vendored (mdBook, Bun and the fonts are fetched too). Fetching keeps the licence
text and the binary out of the tree and costs one pin. The SVGs carry none of
CeTZ's code, only paths. This is a recorded engineering choice, not legal advice.

**Offline once set up.** A render runs the pinned `target/tools/typst`, never
one on `PATH`, under `env -i` from a private directory, with
`--ignore-system-fonts`, `--ignore-embedded-fonts` and `--font-path` set to the
four pinned DejaVu faces, and `--package-cache-path` set to the pinned cache.
Before any render, the check verifies the Typst version, every pinned package
by its tree digest, that the cache holds nothing else, and every pinned font by
digest and that the font directory holds nothing else. A missing cache
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

**Stereotype and technology lines are upright.** The earlier C4 output set them
in italics. Typst draws a missing italic face upright (no skew in the
SVG), no Oblique face is pinned, and the 12-point step and the `«»` and `[]`
marks carry the distinction. `book-figure-retire-plantuml-k117` considered
pinning the face: the reason it had been avoided (the former renderer measured
the italic face differently per host) went with that renderer, so adopting it
would be one pinned face and a `style: "italic"` in `_style.typ`, but nothing a
reader asks of a figure turns on it, so it is not adopted.

**Version mismatch.** Typst's SVG carries no version stamp, and the script does
not edit the SVG to add one. A wrong Typst is named up front by the version
check (`Typst 0.15.1 is pinned, found '...'`). Because renders always use the
pinned binary, an SVG made by another version can only be a committed one, and
it surfaces as a difference whose message names the pinned Typst and CeTZ. The
check cannot tell that cause from a changed figure; regenerating is the remedy
for both.

**Reproducibility, observed on macOS arm64 (Linux not run).** Two renders, a
render from another directory, and a render under a hostile environment
(`TYPST_*`, `SOURCE_DATE_EPOCH`, locale, proxy and `XDG_*` set) are
byte-identical. The Linux arm64 and amd64 archives are pinned and fetched by the
same code, but no Linux run has been made, and Typst's output on Linux is
unchecked. The findings retained from the former renderer (the host-font
differences, the embedded-source race and the digests of its inputs) are in
[`docs/evidence/book-figures-k92/`](../evidence/book-figures-k92/README.md) and
are not rewritten.

**Text in the SVG.** Typst writes every glyph as a path: the text is not
selectable or searchable, and the files are 5-7 times larger than the former
renderer's. The contract never promised selectable figure text; the page's alt
text, caption and prose are the text alternative, and the SVG is a full-size
link and never the only place a fact is stated. The size budget (each SVG at
most 300 KB, the set at most 2 MB) was checked by `book-figure-viewing-check`
(see "Viewing checks"). The latest size measurement is in the dated viewing record below.

**Controls.** `task docs:figures:test` runs the checks of the check, each made to
fire against a subject known to be wrong, with its fixtures in
`scripts/figures-fixtures/` outside the book: the fetch refusals offline (a
fresh fetch, a corrupt cache, a corrupt download, a wrong pin, a failed
download); a fixture figure that matches and renders to the same bytes twice, in
a hostile environment too; a changed source, module, SVG and layout record; a
missing or orphan SVG and layout record; `update` writing and rewriting and then
changing nothing; a second editable source, a file of another kind and a
subdirectory; a restyle that changes the SVG and leaves the record alone, and a
record holding a colour, a length, a style key or a piece of content; a Typst of
another version; a missing, empty, changed, extended and symlinked package
cache and an extra package or namespace in it; an unpinned, other-version and
`@local` import; a figure importing a package itself; a missing font family; a
tampered, extra and missing pinned font; a render with an empty package cache or
an unpinned import that reaches only a refused connection; and a render with no
fonts that cannot fall back to an embedded one. The book's own four modules are
rendered through a figure with every kind of row, twice, with no bytes committed
for them (a style change is a change to every figure); the control checks that
the figure is sketch at roughness 2, that its key box is crisp, that two names
give two different sketches, and that a module other than `_draw.typ` importing
CeTZ is refused. One page with a figure is built through mdBook to confirm the
relative path and the alt text. When the Typst side was added, each safeguard was
also removed from a copy of the script, one at a time, and the control for it
failed: the embedded-font flag, the proxy, the stderr rule, the lint call, the
lint's no-import rule, the package check, the version check, the symlink rule
and the `.typ` source glob. `task check` does not run these controls and does not
need the figure tools.

## The layout modules

**On disk.** A `.typ` whose name does not start with `_` is a figure source, with
its SVG and layout record of the same name. A file starting with `_` is a module
that figures import. It is not a figure and gets no SVG; an SVG for one is
refused as an orphan. Seven modules hold the book's drawing. `_style.typ` holds
the palette roles, text scale, stroke weights, faces and text measuring, and
draws nothing. `_layout.typ` holds the constructors a layout module builds its
record with and the orthogonal ports and routes that `_c4.typ` and `_state.typ`
share. `_sequence.typ` is the sequence-diagram layout, `_c4.typ` the C4 layout,
`_deployment.typ` the deployment layout and `_state.typ` the state-machine
layout: each measures its text and returns a layout record. `_draw.typ` turns a
record into strokes (see "Layout, then draw"). `_style.typ` and `_sequence.typ`
were promoted from the k107 trial, which was removed once its findings were
recorded here (its sources stay in the history of the commit that added it,
`book-figure-renderer-k107`).

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
written in the source as `'` and drawn as `’` (typographic).

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

## Porting record

This is the history of the migration from PlantUML to Typst
(`book-figure-migration-k108`). The ports are `book-figure-sequences-k111`,
`book-figure-c4-context-container-k112`, `book-figure-source-components-k113`,
`book-figure-installed-files-k114`, `book-figure-session-outcomes-k115` and
`book-figure-settled-exit-status-k116`, and `book-figure-retire-plantuml-k117`
removed the former tools. It records the effort each figure cost and what changed
against the former output, with the reason. It describes the former renderer and
is not a rule for any figure. In the former output an apostrophe was straight;
in the Typst figures a source's `'` is drawn as the typographic `’`.

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
primitives. The book has no other renderer to fall back on, and rough.js on Bun
(see "Layout, then draw") is a drawing backend only: it would not lay out a
figure.

## Viewing checks

The records below are dated evidence of the artifacts each pass inspected.
They are not current style instructions; the shared conventions above govern
new and revised figures. Earlier references to notation teaching describe the
superseded contract. Every record dated before 2026-10-17 describes figures
drawn by the former renderer (PlantUML, with the `.puml` sources and the shared
`_style.puml` and `_uml-style.puml` includes it names), which the migration to
Typst has since replaced. They are kept as history, and none of their
observations is a statement about the present figures. Each later record names
the revision and environments that it checked.

### Read 1 text correction, 2026-10-18 (`book-first-half-k130`)

This pass changed the text of all nine figures to the STE writing rules and
the word decisions in `ste.md`. Result labels now use “send”. The colour
descriptions stay in each key, and the adjacent prose no longer repeats them.
The figures keep their facts, relations, lifetime markers and colour roles.
Their existing alt text and captions still describe them. The pins and shared
modules did not change.

All nine figures were inspected in the local book through Chrome on macOS,
on a light page at the 750-pixel chapter width. The viewport was 1227 by
919 pixels. Each full-size link opened the SVG itself, and screenshots of
the complete SVG supplied the inspection of figures larger than the viewport.
The text fits its boxes, and no changed label overlaps another label or a
relation. The page introductions still state the purpose of each figure.

The three dense figures retain the size issue recorded by k118. At chapter
width, the 14-unit body text is about 8.0 pixels in `successful-session`,
9.0 in `process-authority` and 9.8 in `supervision-cancellation`.
The full-size links give clear text. This issue stays with `ravel-book-k101`.
This pass does not claim that the chapter-width size issue is resolved.

The nine SVGs total 1,810,122 bytes. The largest is `process-authority`, at
260,683 bytes. Both remain within the 2 MB set and 300 KB individual budgets.
The sources, generated layout text and chapter alternatives were read
directly. No new figure was needed: the existing diagrams, lists and tables
carry the relations. This is an editorial judgement, not a medium limit.

This pass did not inspect an editor preview, other browsers, other themes
or narrower viewports. GitHub and the book on GitHub Pages remain unavailable
for this branch. The inspection supplies visual evidence, not evidence that
a new reader understands the figures.

### Migrated figures, 2026-10-17 (`book-figure-viewing-check-k118`)

All nine Typst figures were opened in the locally served mdBook (`task
docs:book`, mdBook 0.5.4) through Chrome on macOS, in a 1467-pixel-wide window,
so that the light page's chapter column is the 750 pixels earlier passes used.
Each figure was viewed on its page at chapter width in the light theme and again
in the Navy theme, and was rendered at natural size, unscaled, inside the book's
own origin. Each page's alt text, caption and key were read against the figure.
Every "full size" link resolves (HTTP 200, `image/svg+xml`).

*Size.* The nine SVGs total 1,803,456 bytes against the 2 MB budget. The largest
is `process-authority` at 259,237 bytes against 300 KB; the smallest is
`installed-files` at 160,152. The budget holds.

*Facts.* Each view's row in "What each view means" was checked against the
figure, and every item in it is drawn. As an independent test, the words in the
`<text>` of each former PlantUML SVG (the last one before its port) were looked
up in the strings of the new layout record: none of the nine is missing a word.
That comparison compares word sets, not positions or counts, and the record holds
the text to be drawn, not the pixels. The control was seen to fail: against
another figure's record, 113 of 143 and 119 of 145 words of two old figures were
missing, and a record with one word removed named that word. The viewing
confirms that the drawn text is the record's text. Every alt text, caption and
key still describes its figure.

*Edges.* Each SVG was rasterised at natural size and the first non-white pixel
found from each side. The margin is 7 to 8 pixels on all four sides of every
figure (the 6-point canvas margin), so nothing is clipped, except that
`system-context` has 35 pixels on its right and `process-authority` 22, a gap of
about 21 and 11 units that the layout leaves empty.

*Text size at chapter width.* The SVGs state their width in points, so a figure's
natural pixel width overstates its size. The 14-unit body text is
14 × min(4/3, 750 ÷ the viewBox width) pixels in the column, and for a PlantUML
figure 14 × min(1, 750 ÷ its width). Measured from the files, with the image's
rendered width confirmed as 750:

| Figure | viewBox width | Text now | Text in the PlantUML figure |
|---|---|---|---|
| `successful-session` | 1289 | 8.1 px | 10.5 px |
| `process-authority` | 1162 | 9.0 px | 9.7 px |
| `supervision-cancellation` | 1056 | 9.9 px | 13.8 px |
| `source-components` | 950 | 11.0 px | 14.0 px |
| `system-context` | 916 | 11.4 px | 12.4 px |
| `session-outcomes` | 852 | 12.3 px | 14.0 px |
| `selection-cancellation` | 777 | 13.5 px | 14.0 px |
| `settled-exit-status` | 750 | 14.0 px | 14.0 px |
| `installed-files` | 635 | 16.5 px | 14.0 px |

The Typst layouts are wider than the PlantUML ones for the same content, so a
reader sees smaller text, most in the three densest views. All are legible when
the screen is zoomed, none overlaps, and the full-size link shows each at its
drawn size. No source was changed. A narrower `successful-session` was tried
(`max-label` at its default of 170 gives 10.4 px, as the old figure had) and
reverted: labels written with line breaks then wrap again, "process-group" is
split at its hyphen, which the porting record gives as the reason these figures
pass 300 or 340, and the figure grows from 1185 to 1548 units tall. The same
setting on `selection-cancellation` left words alone on a line and "re-raise"
split, and on `supervision-cancellation` 240 or 260 gained only 1.1 or 0.8 px.
Recorded for the whole-book art pass `ravel-book-k101`, which can choose between
non-breaking hyphens with a smaller `max-label`, shorter label text and accepting
the size with the full-size link. Three figures have text under 10 px:
`successful-session`, `process-authority` and `supervision-cancellation`. Also
for that pass, in `successful-session`
the "Stop the managed process's group" label sits about 15 pixels below its
section divider, close but not touching, and the unused right-hand space of
`system-context` and `process-authority` could go.

*Not reached.* VS Code's Markdown preview: chapter 1 was opened with the `code`
command, but the window was in Restricted Mode and the editor in Vim normal mode,
the preview shortcut produced no preview, and the workspace was not trusted
because that is a security setting. The preview is unchecked, as in earlier
passes. A raw SVG document could not be captured by the browser extension, so
"full size" was viewed as the same file rendered unscaled in the page, plus the
link's resolution. Chrome only; the Rust, Coal and Ayu themes; narrower
viewports; GitHub's rendering of this unpublished branch; and the published site
were not viewed. This is visual evidence of the figures, not of how a reader
understands them: the owner's stand-in read, `book-read-one-k70`, follows.

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
- Typst on Linux (arm64 and amd64): both archives are pinned and fetched by the
  same code, and no Linux run has been made. Native x86-64 Linux hardware, and
  any Windows or macOS x86-64 host, are not pinned hosts.
- VS Code's Markdown preview, which did not open in the 2026-10-17 pass, and any
  browser but Chrome.
- mdBook's Rust, Coal and Ayu themes and narrower viewports. The light and Navy
  themes were viewed for the nine migrated figures; the records under "Viewing
  checks" cover only the named pages and environments.
- The published site, which is unavailable before the book's first release.

## Changing a pin

A renderer or version change is a change to the contract. Re-read the release's
digests, edit the constants in `scripts/figures.sh`, run `task setup`, then
`task docs:figures:update` for the fixtures and the book's figures, and read the
diff of every figure and layout record before committing it.
