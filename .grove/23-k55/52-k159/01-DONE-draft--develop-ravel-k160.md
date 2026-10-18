# develop-ravel-k160

## Goal

Draft **part A** of chapter 11, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/11-develop-ravel.md`, linked from
`docs/book/src/SUMMARY.md`: setting up a checkout, building, the test seams,
`task check`, and adding a test. The headings for part B are not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 7 gives the
  reasons for the parts; this chapter is the contributor's map of the same
  source. Link back instead of re-teaching.
- Start from a clean checkout (a scratch `jj` workspace or a fresh clone of the
  working copy in the scratchpad, never the live tree) and follow your own
  chapter as the reader would: `task setup`, `task build`, `task typecheck`,
  the focused tests, `task check`. Record which of them you ran, on which host,
  with the elapsed time and what they need (disk, network for `task setup`).
  The first run of `task setup` fetches the pinned tools, so say what is
  fetched and what is verified.
- Add a test in each seam on a scratch copy to learn what a contributor sees:
  one runner test with a real process (and the PTY helper where it applies), one
  command-boundary test in `crates/ravel/tests/cli.rs`, and one unit test in
  the Ravel CLI. The fragment in the chapter is what you actually added, with
  the file it came from. Read `task test:list` for the filters.
- Settle the heading structure for the whole of chapter 11 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- The README's “Develop” section is a source, not a place to copy from.
  Its sentences are listed in the node brief. Rewrite them in the book's
  words, and verify each against the Taskfile and the scripts.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and, if the
  chapter holds an excerpt, `task docs:excerpts`.
- The log lists which tasks of `task --list` part A teaches and which remain
  for part B, and the destination of each README “Develop” sentence that part A
  covers.
- Last act: `leaf-add` a `draft` leaf (bare stem `develop-ravel`) whose body
  states part B's scope from the node brief, the headings reserved for it and
  anything part A learned that changes it; then `leaf-add` the `copy-edit` leaf
  if no live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claims of what `task check` runs and writes, because the chapter's reader
  trusts them without running them.

## Decisions (running log)

- Structure precondition: `docs/book/contract.md` names the reader as a
  “16-year-old software engineer with no background in Ravel”. Its ordered
  section plan gives chapter 11 the outcome “set up a checkout, build, use the
  test seams and `task check`, add a test, and know what the evidence documents
  are for”. Its Emphasis and Exclusions sections require fake programs and
  keep evidence campaigns outside the book. The document node names this
  contract and divides the chapter at the point where the reader can add a test.
- Whole-chapter heading order: part A owns “Prepare a checkout”, “Build the
  matched pair”, “Find the source for a change”, “Choose a test seam”, “Run
  the checks before handoff”, and “Add a test”. Under “Add a test”, the three
  subsections are “Test a real child process”, “Test the command boundary”,
  and “Test a local rule”. Part B owns, in order, “Install and test the
  installed pair”, “Check the book”, “Keep tests separate from live sessions”,
  “Read the evidence for a change”, and “Find a development task”. No empty
  part B headings go into part A.
- Measurements use the new jj workspace `/tmp/ravel-k160-checkout`, based on
  `df241c28`, with no copied target directory or worker dependencies. The host
  has macOS 27.0.1 arm64, Rust 1.99.1, Task 3.54.0 and nextest 0.9.148.
  Host Cargo and Bun download caches are not empty. This is a clean checkout,
  not a claim of an empty machine or minimum-Rust qualification.
- The scratch-only tests will teach an unsent child exit status, a result
  containing leading/trailing spaces, and parsing an option-looking result.
  They assert current behaviour. No production behaviour changes or duplicate
  permanent test cases are needed to teach these seams.
- One reviewer inspected the “Run the checks before handoff” section against
  the Taskfile, scripts and Cargo/nextest configuration. It found no material
  issue. The leaf-wide allowance is spent. No ADR decision changes.
- The first full scratch `task check` failed in the hostile package-json FIFO
  case at `hostile.rs:1315`: “the shipped worker stalled as well”. Its prior
  stages passed. The exact case passed alone, unchanged, in 7.57 seconds.
  The failure is retained rather than explained as contention without evidence.
  No product or test repair is inferred from the isolated pass.

## Source and structure verification

Tier 2 graph lookup used project
`Users-antony-Development-Ravel.simplify-and-componentize`, initially indexed
at generation `2026-10-18T10:34:37Z`. Searches located the CLI fixture and
runner process/PTY tests. Their exact snippets and the CLI fixture's outbound
trace were checked against source. Relevant searches had no remaining pages.
The trace's cross-module `args`, `path` and `write` matches are not relied on:
the actual fixture bodies establish the process and policy boundaries.
Coverage checks found no recorded gap in the scripts, manifests and source
files used. `.cargo/config.toml` is excluded from the graph and was read in
full. Later document coverage uses generation `2026-10-18T10:39:26Z`.
Coverage is a best-effort signal, not completeness proof.

`docs/book/contract.md`, the brief chain, glossary, all three ADRs, example and
excerpt conventions, and STE word decisions were read. The local STE PDF's
verb, procedure and descriptive-text rules were consulted without copying it
into the repository. The chapter order follows the contract's contributor
outcome. Tables compare tools, source locations and seams. The examples progress
from process behaviour through the public command to a local parser rule.

## Contributor workflow inventory

`task --list` enumerates 57 public tasks at this draft. The internal `release`
task is not in that list. The table assigns every enumerated name, including
tasks whose detailed teaching remains for part B or chapter 12. It does not
claim completed inventory 5 for part A alone.

| Owner | Tasks |
|---|---|
| Part A | `build`, `check`, `deps`, `setup`, `test`, `typecheck`, `worker`, `runtime:integrity`, `test:cli`, `test:hostile`, `test:list`, `test:runner`, `test:session`, `test:terminal-recovery` |
| Part B: installed pair and probes | `install`, `smoke`, `probes` (part A names probes as the hostile-test prerequisite) |
| Part B: book | `book:integrity`, `docs:book`, `docs:book:build`, `docs:breaks`, `docs:breaks:test`, `docs:examples`, `docs:examples:mutants`, `docs:examples:test`, `docs:excerpts`, `docs:excerpts:test`, `docs:figures`, `docs:figures:test`, `docs:figures:update`, `docs:ste`, `docs:ste:test` |
| Part B: views and evidence instruments | `docs:components`, `docs:result-job`, `native-entry:build`, `native-entry:front`, `native-entry:runner`, `native-entry:start`, `native-incumbent:build`, `native-linux:image`, `test:native-campaign`, `test:native-campaign-controls`, `test:native-entry`, `test:native-incumbent`, `test:native-incumbent-controls`, `test:native-linux`, `test:native-r01-controls` |
| Chapter 12, named and linked by part B | `release:archives`, `release:doctor`, `release:formula`, `release:lifecycle:test`, `release:major`, `release:minor`, `release:notes`, `release:patch`, `release:smoke`, `release:test` |

No public task lacks an assigned owner. Part B re-enumerates the list and
adds the complete reader-facing inventory. `docs:components` remains a live
task until guide migration k164 decides its disposition. Part A paraphrases
task meanings and quotes no old “front” description, so the Taskfile is unchanged.

## README Develop destinations

| Source statement or command | Destination |
|---|---|
| Rust floor, curl, tar, Task, ShellCheck and nextest prerequisites; tasks run from checkout | “Prepare a checkout” |
| `setup`, `build`, `typecheck` command lines | Setup fragment and “Build the matched pair” |
| `test`, `test:session`, `test:cli`, `test:hostile` | “Choose a test seam” |
| `probes` | Part A gives the hostile-test prerequisite; part B teaches the workflow |
| `runtime:integrity`, `check` | “Run the checks before handoff” |
| `smoke`, `install PREFIX=…` | Part B |
| Typecheck covers worker, SDK and generated declarations | “Build the matched pair”, with its declaration-generation prerequisite |
| Command-boundary tests use actual workers, temporary policies and fixture processes | “Choose a test seam” and “Test the command boundary” |
| Focused checks during edits, full relevant suite at handoff | “Choose a test seam” |
| Check rebuilds/typechecks the pair, checks format, denies Clippy warnings and runs the Rust suite | Ordered stage table and Clippy/default-selection explanation in “Run the checks before handoff” |
| Diagnostics under `target/check.*`, short summaries/failure details, committed Cargo lockfile | Same section, with the separate worker logs and generated outputs |
| macOS sandbox-exec and Linux bubblewrap requirements | “Prepare a checkout” |
| Checkout Bun pin, no host Bun/Node at run time | Setup pins and paragraph before “Build the matched pair” |
| No model account or other checkout | Chapter opening |
| Private channel combines completion signal and atomic result send; real subprocess checks; compiled evaluator | “Find the source for a change”, with chapter 7's channel link |

The README remains unchanged for the later fold-in leaf. Its release and
notices text belongs to chapter 12, and its retired-project pointer stays
outside the chapter's teaching.

## Validation and handoff

- The chapter exists and SUMMARY links it. The whole-document read checks the
  contributor order and each source claim. Setup pins, dependency installation,
  generated declarations, task filters, logs and the private PTY roles agree
  with the source. The four Rust fragments exactly match the scratch files.
  A changed function-name control fails each comparison. The two added runner
  tests cover an ordinary child and the PTY helper.
- `docs/evidence/develop-ravel-k160/README.md` records the host, elapsed times,
  disk observations, source freeze and limits. Its logs retain setup, build,
  typecheck, both lists, each focused test, the first failed full check, the
  isolated retry and the full successful check. Its patch reproduces the four
  scratch-only additions against `df241c28`.
- The successful full scratch check reports 343 passed and 46 skipped. A join
  of test names confirms all 339 names in the original default list appear in
  the passed set, with exactly the four added names left over. The first name
  extractor missed nextest's padded positions below 100. It was corrected to
  accept that padding before the comparison was credited. No failed run is
  counted as a pass, and no cause is assigned to the first timeout.
- All 3,662 tracked scratch files have identical digest manifests before the
  first full check and after each full check. The evidence note states the
  unmeasured host-cache and generated-output limits. No live production file
  changed.
- Final whole-book checks passed: `task docs:book:build`, `task docs:ste --
  --strict` (26 files), `task docs:breaks -- --strict` (10 chapters),
  `task docs:excerpts` (8 excerpts), and `task docs:examples` (14 passed,
  0 failed, 0 skipped). A digest over `docs/book`, `scripts`, `crates`
  (excluding node_modules and target), Taskfile, Cargo manifests/lockfile,
  Cargo/nextest configuration, glossary and README stayed unchanged through
  that run. After two first-use term links changed, build, scoped strict STE,
  prose breaks and the chapter's example syntax check passed again. Its seven
  fragments are not run by the example checker, as their labels state.
- The 57-name task inventory matches `task --list` with no unassigned or extra
  names. A removed-name control is rejected. Part B still owes the complete
  reader-facing inventory, installation, book checks, guards and evidence
  families. No existing guide is removed here.
- The one reviewer found no material error in the check section. No second
  review is required, no architectural decision changed, and the ADR set stays
  unchanged. This is a source and execution check on one host, not a human
  comprehension result. Copy-edit, art, proof and the owner's read remain live.
