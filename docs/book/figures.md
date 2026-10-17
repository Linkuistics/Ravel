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
