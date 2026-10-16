# PlantUML metadata discrepancy: retained evidence

On 2026-10-16, chapter 5's art correction reproduced a figure comparison
failure and retained its bytes. The correction makes the pinned PlantUML
process files with one worker. Thirty subsequent full checks matched all
seven committed SVGs. No chapter, figure source, SVG, renderer version or
font pin changed.

## Observed failure

The starting change was `2a386e85` (`running-sessions-k91`). The ordinary
`verify_figures check` was called repeatedly with an externally allocated
`WORK` directory. That bypasses scratch deletion, not rendering, linting or
comparison. Eight checks passed; the ninth exited 1 and stopped the loop.
[The per-run log](parallel.log) includes every completed attempt, including
the failure. All seven figures were compared in each completed check.

The failed files are retained exactly as the checker compared them:

- [session-outcomes.svg](parallel-failure/session-outcomes.svg)
- [successful-session.svg](parallel-failure/successful-session.svg)

Each differs from its committed counterpart only by one
`<?plantuml-src ...?>` processing instruction before the closing groups.
Removing that instruction **for diagnosis only** makes each byte-identical
to the committed figure. Neither geometry nor visible text differs. The
production checker continues to compare all bytes, including metadata; it
does not filter the discrepancy or retry a failed check.

The [parallel input inventory](parallel-inputs.sha256) was captured before
the first run and checked after the failure, with every entry unchanged.
It covers the book, scripts, crate files outside `node_modules` and `target`,
workspace manifests, Taskfile, PlantUML jar and pinned font faces. Its paths
are relative to the repository root. The later script and documentation changes
intentionally make some entries differ from the final tree; it is a receipt
for the original subject, not a check of today's tree.

The earlier k91 failure named `selection-cancellation.svg`, but deleted its
scratch output. That failure's exact bytes and cause remain unknown. The new
failure establishes a defect at the same shared delivery seam; it cannot
retroactively establish that k91 saw the same discrepancy.

## Mechanism and correction

The pinned jar's `javap -c -p` output was checked against the release-tagged
upstream source:

- [`Run.processInParallel` and `getSourceFileReader`](https://github.com/plantuml/plantuml/blob/v1.2026.8/src/main/java/net/sourceforge/plantuml/Run.java#L325)
  dispatch files to a shared options object's worker pool.
- [`CliOptions.getFileFormatOption`](https://github.com/plantuml/plantuml/blob/v1.2026.8/src/main/java/net/sourceforge/plantuml/cli/CliOptions.java#L276)
  checks and publishes a plain shared field without synchronisation, then
  applies `hideMetadata()`. Its default worker count is the number of
  available processors; `--threads 1` selects one worker.
- [`FileFormatOption`](https://github.com/plantuml/plantuml/blob/v1.2026.8/src/main/java/net/sourceforge/plantuml/FileFormatOption.java#L74)
  defaults to metadata enabled and copies that option when deriving another
  format option. `hideMetadata()` mutates the original boolean.

Concurrent initialisation can therefore expose an option with metadata still
enabled. This is a source-supported explanation of the observed metadata-only
discrepancy, not a trace of the failed JVM's thread interleaving. Serial file
processing removes that concurrent initialisation from this command. The
script still uses the same jar, Smetana settings, fonts, `-nometadata`, source
files and exact comparison. A separate JVM per file would also isolate the
options but adds unnecessary startup; output filtering would conceal the
underlying fault.

## Bounded verification

The host was Darwin arm64, Homebrew OpenJDK 27, build 27. This pass adds no
Linux, other-Java or remote-rendering evidence.

The [serial input inventory](serial-inputs.sha256) covers the corrected script,
every book figure source/include/SVG, the jar and all font faces. It was
captured before the serial run and verified afterwards, with every entry
unchanged. [All thirty per-run results](serial.log) match every one of the
seven committed figures, including the originally reported selection figure.
This bounds the observation; it is not proof that arbitrary future inputs or
every Java runtime produce identical bytes.

[The existing figure controls](controls.log) also passed: actual renders,
hostile environment, changed source/SVG/include/font/layout/version, missing
and orphan assets, tool/fetch refusals, idempotent update and mdBook delivery.
The failed ordinary check supplied the red observation; the correction uses
the real renderer and existing comparisons rather than a mock assertion of
one flag. Shellcheck and `shfmt -d -i 2 -ci scripts/figures.sh` passed.

The final required tasks all exited 0: [figure comparison](final-figures.log)
(seven matched), [complete examples](final-examples.log) (13 passed, none
failed or skipped; six labelled fragments not run), and
[mdBook build](final-book.log) (no warnings). The
[final input inventory](final-inputs.sha256) covers the book, scripts, crate
files outside dependency/build directories, workspace manifests, Taskfile,
renderer jar, fonts, d2 and mdBook. Every entry matched after those tasks.
No measured subject was edited during a run. This evidence directory and
the Grove handoff notes were written outside that inventory.

The whole chapter, its three PlantUML sources and their adjacent explanations
were read as raw files. No new figure was needed: the existing sequences,
settlement view and tables already carry the chapter's relations. This is
an editorial judgement, not a limitation of the admitted medium. No visible
output changed, and this correction adds no browser or editor viewing claim.
The art and proof viewing records in [figures.md](../../book/figures.md)
remain intact, with their theme, viewport, editor and unpublished-site limits.
No human-comprehension, real-agent or terminal-interaction evidence follows.

## Retain a future failure

For a diagnostic run, allocate a private directory before sourcing the checker:

```sh
retained=$(mktemp -d)
bash -c 'source scripts/figures.sh; WORK="$1"; verify_figures check' bash "$retained"
```

The exit status is still the check's status. Inspect `d.*/in`, `d.*/out` and
the renderer logs below that directory before removing it. Ordinary
`task docs:figures` remains the required reusable check and deletes its scratch
directory on exit. A future mismatch must be investigated, not accepted by
regenerating the committed figure or by retrying until green.
