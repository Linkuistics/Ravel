# book-figure-engine-k109

## Goal

Admit Typst/CeTZ as a figure renderer beside PlantUML, with pinned tools and one
shared style module, so each later leaf can replace one figure's `.puml` with a
`.typ` and leave `task docs:figures` green. Port no book figure here.

## Context

- Plan and decisions: the `## Plan` section of `book-figure-migration-k108`.
  Read it first; it settles the ban on two sources, the SVG text question and
  the `_*` includes.
- Trial to promote: `docs/book/trial-typst/` (`style.typ`, `seq.typ`, README
  pins and measurements). Current mechanics: `scripts/figures.sh`,
  `scripts/figures.test.sh` with `scripts/figures-fixtures/`, the `docs:figures*`
  and `setup` tasks in `Taskfile.yml`, `docs/book/figures.md`.
- Existing pins style to follow: per-host SHA-256 for d2 in `scripts/figures.sh`,
  the DejaVu face digests, and the controls seen to fail in `figures.test.sh`.

## Done when

- `task setup` fetches a pinned Typst (0.15.1 unless a recheck on the day says
  otherwise; record the date) for Darwin/arm64, Linux/arm64 and Linux/amd64 with
  SHA-256 checks, and the pinned CeTZ 0.4.2 and its dependency oxifmt 1.0.0 from
  the package registry with SHA-256 checks, into `target/tools`. Check the CeTZ
  licence and record whether fetching or vendoring is right; the plan's default
  is fetching, because nothing else the book needs is vendored.
- Renders are offline once `task setup` has run: Typst gets
  `--package-cache-path` pointing at that directory, `--ignore-system-fonts`
  and `--font-path` for the pinned DejaVu faces, and an empty environment like
  the PlantUML render. A control with an empty package cache fails with a
  clear message instead of reaching the network.
- `scripts/figures.sh` treats `*.typ` files not starting with `_` as figure
  sources next to `*.puml`/`*.d2`. The one-source-per-stem rule extends to
  `.typ`. Underscore `.typ` files are shared modules and get no SVG. The check
  verifies the Typst version up front. Typst SVG carries no renderer stamp, so
  decide how the check names a version mismatch (the plan prefers a clear message
  from the up-front version check over editing the SVG) and record the choice.
- `docs/book/src/figures/_style.typ` (palette roles, scale, strokes, arrowhead,
  cylinder, key box, page setup) and `_sequence.typ` are promoted from the trial.
  Every stroke and outline goes through a small set of primitives (`line`, `box`,
  `cylinder`, arrow) in `_style.typ`, so the sketch leaf can change how they are
  drawn in one place. Do not draw with raw CeTZ calls in figure sources.
- Decide the stereotype/technology lines: the trial used upright 12pt, the
  PlantUML C4 output used italics. Say whether to pin DejaVu Oblique (the pin was
  avoided only because Java measured it differently per host) or keep upright.
- `scripts/figures.test.sh` has fixtures for a passing `.typ`, a changed source,
  a changed SVG, a tool version mismatch, a missing package cache and a
  `.puml`+`.typ` stem clash, each seen to fail before the pass is credited.
- Record in `docs/book/figures.md` what is new (the pins, the offline rule, the
  Typst reproducibility controls, md5-identical renders on this host; Linux is
  unchecked unless run). Leave the PlantUML text for the retirement leaf.
- `task docs:figures`, `task docs:figures:test` and `task check` pass.

## Notes

Leave `docs/book/trial-typst/` in place; the port leaves consume its figure
sources, and `book-figure-c4-context-container` removes it.

## Decisions (running log)

2026-10-17 — Pins rechecked on the day. Typst 0.15.1 is still the latest release
(published 2026-07-17); GitHub's listed digests for the macOS arm64, Linux arm64
and Linux amd64 archives equal what was recomputed from the downloads. CeTZ
0.4.2 and oxifmt 1.0.0 are pinned as planned. CeTZ 0.5.2 exists (needs Typst
0.14 or later, which 0.15.1 satisfies); not adopted, because moving up is a new
trial of the drawing primitives and no figure needs it. The registry lists no
digests, so the tarball digests were computed from the downloads, and the
unpacked trees were diffed against the copies Typst fetched on its own.

2026-10-17 — Fetch, not vendor. CeTZ is LGPL-3.0-or-later and ships a
WebAssembly plugin; the repository is Apache-2.0 and nothing else the book needs
is vendored. The SVGs carry paths and text, not CeTZ code. Recorded as an
engineering choice, not legal advice. `docs/book/figures.md` carries it.

2026-10-17 — Offline is enforced twice. `require_tools` checks the pinned Typst
version and every package by an unpacked-tree digest (and refuses extras), so a
missing cache fails with "run 'task setup'". The render also points its proxy at
a refused port: without it Typst downloaded a missing `@preview` package on its
own (seen), so an unpinned import is an error and not a fetch. A lint refuses
any `@ns/name:ver` in a `.typ` that is not a pinned package, `@local` included.
`--package-path` was left out: the empty environment and the private HOME leave
no local packages.

2026-10-17 — Fonts. Typst embeds Libertinus Serif, New Computer Modern and DejaVu
Sans Mono and treats an unknown family as a warning that still exits 0 (seen:
with the stderr rule removed, a nonexistent family rendered). So the render adds
`--ignore-embedded-fonts` and any stderr output fails the figure.

2026-10-17 — Italics: stay upright. DejaVu Oblique shares a directory with
Java's font path until PlantUML is retired, and pinning it would bring back the
host-dependent italic measurement into the nine committed PlantUML SVGs. Typst
draws a missing italic upright (no skew transform in the SVG). The 12pt step
and the «» and [] marks carry the distinction. The retirement leaf may reopen it
(one pinned face and a `style: "italic"`).

2026-10-17 — Version mismatch: no SVG edit and no stamp. The up-front version
check names a wrong Typst; a committed SVG made by another version surfaces as a
difference whose message names the pinned Typst and CeTZ. The check cannot tell
that cause from a changed figure; regenerating is the remedy for both.

2026-10-17 — Primitives. `_style.typ` has `draw-line`, `draw-arrow`, `draw-poly`,
`draw-box`, `draw-cylinder` and `draw-text` (placing text, no stroke), and
`canvas`. They take y-down coordinates, so the flip lives in one place. Only
`_style.typ` imports CeTZ; `_sequence.typ` draws through the primitives; the lint
refuses a figure source that imports a package. The sequence figure rendered
through them is byte-identical to the trial's (md5 equal for base and long).

## Outcome

- `scripts/figures.sh`: Typst 0.15.1 per-host archive digests, CeTZ and oxifmt
  tarball and tree digests, `.typ` figure sources and `_*.typ` modules, `.typ`
  in the stem rule and the orphan rule, `lint_typst`, a Typst render under an
  empty environment, `require_packages`.
- `docs/book/src/figures/_style.typ` and `_sequence.typ` promoted; no book figure
  ported, so `task docs:figures` still checks the nine PlantUML SVGs.
- `scripts/figures.test.sh`: fixtures `layers.typ`, `_shapes.typ`, `layers.svg`;
  the six required controls (passing `.typ`, changed source, changed SVG, tool
  version mismatch, missing package cache, `.puml`+`.typ` clash) plus the others
  listed in `figures.md`. Each safeguard was removed from a copy of the script and
  its control failed: `--ignore-embedded-fonts`, the proxy, the stderr rule, the
  lint call, the no-import rule, `require_packages`, the version check, the
  symlink rule and the `.typ` source glob.
- Checked here: macOS arm64 only. The Linux archives are pinned and not run.
- At handoff, run on this host: `task check` exit 0 (339 tests passed), `task docs:figures` (9 figures match), `task docs:figures:test` passed in full.
- Not run: Linux (arm64, amd64) Typst renders; a CeTZ 0.5.x trial.
