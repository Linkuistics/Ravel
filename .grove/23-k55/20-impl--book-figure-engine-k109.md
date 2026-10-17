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
