# book-figures-k59

## Goal

Make a figure reproducible: a Taskfile task regenerates every committed figure
from its editable source with PlantUML 1.2026.8 and d2 0.9.0 and compares the
result byte for byte with the committed SVG, and the layout for a figure on
disk, with its recorded renderer version and regeneration command, is settled
once so the art stage and every drafting session use it.

## Context

- Contract, "Format and media" (figure notations, renderers, delivery) and
  "Figure needs": `docs/book/contract.md`. PlantUML with its bundled
  C4-PlantUML 2.13.0 and the built-in Smetana layout, so no separate Graphviz;
  d2 for structure and flow diagrams; every figure committed as editable text
  source beside the SVG it generates, with the renderer version, layout
  settings and regeneration command recorded; no reliance on GitHub's Mermaid;
  a figure made only of the page's own text needs no source.
- On the host used for this planning, `plantuml -version` reports 1.2026.8 and
  `d2 --version` reports v0.9.0 (checked 2026-10-16); PlantUML needs a Java
  runtime. Whether to fetch the pinned jar and d2 archive into `target/tools`
  with pinned digests (the `setup_bun` pattern in
  `crates/ravel/scripts/dispatch.sh`) or only to assert the installed versions
  is this leaf's decision. Record it with the reason. Either way a version other
  than the pin must refuse, because a different renderer is a different figure.
- Depends on `book-toolchain-k58` for `docs/book/src/` to exist and for the
  stable page names. Existing editable views under `docs/design/components/`
  are Mermaid and stay where they are; they become figure sources only when the
  second half's art stage redraws them.

## Done when

- A layout is recorded for a figure: where its source, its SVG and its metadata
  live under `docs/book/src/` (for example `figures/<name>.puml|.d2` beside
  `<name>.svg`), how the renderer version, layout settings (Smetana pragma, d2
  layout engine, theme, sketch off) and regeneration command are recorded for
  each, and how a page references the SVG with alt text.
- `task docs:figures` regenerates every figure into a scratch directory and
  fails on any difference from the committed SVG; `task docs:figures:update`
  (or the leaf's equivalent) rewrites the committed ones. Names follow the
  `docs:` family.
- Both renderers are proven reproducible: regenerate each sample twice and
  compare, and note anything non-deterministic (timestamps, ids) with how it
  is neutralised. A renderer or layout pin that changes the output is seen to
  fail the compare.
- A control that has been seen to fail: a changed source, a changed SVG and a
  wrong renderer version each make the task fail; an unchanged tree passes.
- A C4 context sample and a d2 sample exist only as the task's controls, kept
  under a test location that is not in the book, unless a book figure from the
  contract's figure list can serve as the first real one without pre-empting the
  art stage; if so, say which.
- The task and its prerequisites (Java, PlantUML, d2) are documented where a
  contributor looks (the task `desc` and a short comment), and `task check`
  does not require them.
- One page build is run with a figure embedded, in the mdBook build and as the
  raw Markdown path GitHub would use, to confirm the relative SVG path loads
  in both. The GitHub rendering itself is unchecked until a push; say so.

## Notes

- The contract says a change of renderer or version is a change to the
  contract. A needed deviation (a Smetana limitation that forces Graphviz, a
  rendering defect) is an escalation to the owner, not a quiet substitution.
- The visual vocabulary (palette, line weights, how a boundary or an authority
  is drawn) belongs to the art stage and to the first figures; this leaf only
  makes figures reproducible. Do not design the vocabulary here.
- One in-session reviewer is allowed by the spine's rule.

## Decisions (running log)

- **Fetch and pin, do not only assert (decided 2026-10-16).** `task setup` now
  fetches PlantUML 1.2026.8 (the GPL jar, the build Homebrew installs), d2 0.9.0
  and DejaVu fonts 2.37 into `target/tools` through `scripts/figures.sh`, each
  digest-checked on a private copy the way mdBook is. The reason is the
  leaf's own goal: a byte compare is only meaningful against one renderer, and
  asserting an installed version leaves the bytes to whichever package manager
  built it. The jar's digest equals the one Homebrew's formula installed
  (compared, 2026-10-16). Java 11 or later is a prerequisite and is not fetched;
  the jar's class files are major version 55. Version-other-than-pin refuses:
  the jar by digest, d2 by `--version`, and every SVG by the stamp the renderer
  writes into it (`<?plantuml 1.2026.8?>`, `data-d2-version="v0.9.0"`).
- **Layout on disk** (full text in `docs/book/figures.md`, which the contract's
  "Figure delivery" bullet now points to): flat `docs/book/src/figures/`,
  `<subject>.puml|.d2` beside `<subject>.svg`, one diagram per file, named for
  subject not chapter because chapters share figures, `_*.puml|.d2` shared
  includes for the art stage's vocabulary, nothing else allowed there. A page
  uses `![alt](figures/<name>.svg)`; the alt text lives in the page.
- **Where each fact is recorded.** Renderer version: once, in `scripts/figures.sh`
  and the contract, and self-reported by each SVG and checked per file (a
  per-figure copy would rot). Layout settings: in the source, enforced by lint
  (`!pragma layout smetana`, bare single `@startuml`, no remote include; d2
  `d2-config` with `layout-engine` dagre or elk, `theme-id`, `sketch: false`).
  Regeneration command: `task docs:figures:update`.
- **Tasks.** `docs:figures` (check), `docs:figures:update`, `docs:figures:test`
  (the controls). `task setup` also runs `scripts/figures.sh setup`. `task
  check` does not run or need any of it; the new scripts pass its shellcheck
  stage and the whole `task check` passed with them present.
- **Non-determinism found and how it is neutralised** (all seen on 2026-10-16):
  - *Fonts.* PlantUML measures text with the JVM's fonts. The same source gave
    different SVGs on macOS (Homebrew OpenJDK 27) and Ubuntu arm64 (OpenJDK 21).
    Fix: Java's font path is the four pinned upright DejaVu faces and
    `defaultFontName`/`monospacedFontName` name them. With that, macOS arm64,
    Ubuntu arm64 and Ubuntu amd64 under QEMU user-mode emulation (OpenJDK 21,
    with each host's d2 archive) rendered all four fixtures byte for byte alike.
    Pinning the Oblique faces too *broke* it (one italic label 55.945 against
    55.816), so they are deliberately absent and Java slants the upright face.
    The amd64 run is emulated, not an x86-64 kernel. A render with the font
    path pointing at an empty directory differs, so the pin is load-bearing.
  - *Graphviz.* Without the pragma PlantUML used the host's `dot` when present
    and gave different bytes; with it, no Graphviz is consulted. A C4 context,
    container and component, a state, activity, deployment, sequence and timing
    diagram all rendered under Smetana with `dot` absent from PATH, so no kind in
    the contract's figure list forces a Graphviz deviation; nothing to escalate.
  - *Embedded source.* PlantUML appends an encoded copy of the source to the SVG;
    `-nometadata` removes it.
  - *Environment.* A `D2_THEME` in the caller's environment changes d2's output
    (it overrides the file), and a `plantuml.skin` in the working directory is
    read (it broke PlantUML here). Both renderers run under `env -i` from a
    private directory. The Java probe also runs there: the first run of the
    hostile-environment control failed because `_JAVA_OPTIONS=-Xmx1m` made the
    probe think Java was broken.
  - *Timestamps and ids.* None; two renders agree, as does a render from another
    directory under another file name.
  - *Font fallback.* PlantUML names `DejaVu Sans` with no generic family, which
    would give a reader without it a serif face. A fixed text substitution
    appends `sans-serif` or `monospace`, identically in check and update.
- **Controls, each seen to fire** (`task docs:figures:test`, 39 numbered passes): the
  fetch function's fresh, cached, corrupt-cache, corrupt-download, wrong-pin and
  failed-download cases offline; unchanged fixtures pass and two renders are
  identical; a hostile environment changes nothing; a changed source, changed
  SVG, missing SVG, orphan SVG, wrong-version stamp (both renderers), changed d2
  engine, changed d2 theme, changed shared include, missing generic fallback,
  every lint refusal, duplicate stem, stray file, subdirectory, missing jar,
  tampered jar, d2 of another version, tampered font and extra font each fail
  with the named reason. The test itself was mutated seven ways (stamp check,
  compare, font path, d2 env clearing, pragma lint, orphan lint, jar digest) and
  each mutant was killed by a different control.
- **No real figure committed here.** The first real figure (the system context)
  is the art stage's: its visual vocabulary is not this leaf's to design, and a
  figure committed now would pre-empt it. `docs/book/src/figures/` does not exist
  yet, so `task docs:figures` reports zero figures and still verifies the tools.
  The samples are the controls' fixtures in `scripts/figures-fixtures/`, outside
  the book.
- **Page build.** A scratch book with three fixture figures built under mdBook
  0.5.4 with no warning, copied each SVG byte for byte, referenced it as
  `src="figures/context.svg"` with its alt text, and the three loaded in a
  browser over loopback (looked at the rendered page). mdBook also copies the
  `.puml` and `.d2` sources into the built book. The same relative path resolves
  beside the page in the source tree, which is what GitHub's file view follows.
  GitHub's own rendering is unchecked until a push.
- **Handed to the art stage and proof.** PlantUML's SVG has a white background,
  so figures sit as white panels in mdBook's dark themes; inspect each declared
  viewing environment. Authors use `""mono""` for code in labels; `<font:monospaced>`
  maps to the host's monospace, which matched across the hosts tried but is not
  pinned. A reader's browser without DejaVu was not inspected. Other Java
  runtimes than 21 and 27, and native x86-64 hardware, were not tried.
