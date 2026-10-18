# develop-ravel-k178


## Goal

Complete part B of chapter 11, `docs/book/src/11-develop-ravel.md`, under
`docs/book/contract.md`. Start from part A's tested contributor workflow and
append the other workflows, isolation guards, evidence guide and complete
task inventory. Preserve part A's useful boundary and heading order.



## Context

- The structure brief is `docs/book/contract.md`. The document node defines
  part B's scope. `develop-ravel-k160` records every task from `task --list`
  and every README Develop statement with its destination or remaining owner.
- Append these headings in order: “Install and test the installed pair”,
  “Check the book”, “Keep tests separate from live sessions”, “Read the
  evidence for a change”, and “Find a development task”. Part A already owns
  the six preceding sections. No empty headings were emitted.
- Teach `task install`, `task smoke` and `task probes`. Explain the book tasks,
  their controls and what they do not establish: book build/serve, figures
  and update/test, examples and test/mutants, excerpts and tests, STE and
  tests, prose breaks and tests, and book integrity. Verify claims against
  the actual scripts and Taskfile. Exercise commands in a scratch checkout or
  temporary prefix. Never replace a personal installation as a documentation
  trial.
- Absorb `docs/authority.md`: forced-empty Cargo controls, scrubbing helpers,
  `env_clear` fixtures, installed smoke's `env -i`, and the `GROVE_*` and
  `HARNESS_DISPATCH_*` extraction-safety exceptions. The later fold-in leaf
  removes the guide and README content after set proof. Do not remove them now.
- Guide the reader through the evidence document families named in the node
  brief. Say which task or script produces each and when a contributor reads
  it. Name native-entry, incumbent and Linux instruments and link their
  evidence without teaching campaign execution or repeating campaign results.
- Enumerate `task --list` again. Part A's log assigns all 57 public names at
  its source snapshot. Add the complete reader-facing inventory with a
  destination for each name. Chapter 12 owns release tasks. It does not yet
  exist, so use its number/title rather than a broken file link. The set
  proof makes that link when the chapter exists. `docs:components` still
  exists until guide migration decides its disposition.
- Part A paraphrases task meanings, so it leaves Taskfile descriptions
  unchanged. If part B quotes descriptions, replace their “front” wording
  with “Ravel CLI” in the Taskfile as the node directs.
- The four test fragments were compiled and run in a scratch workspace.
  `docs/evidence/develop-ravel-k160/` preserves the patch and logs. The first
  full check failed at a hostile-worker fixture time limit. An unchanged
  isolated case and full rerun passed. Do not convert that into a repaired
  defect or erase the failed observation. The source freeze and limits are
  in the evidence note. Part A's one reviewer found no material error in
  its explanation of `task check`.

## Done when

- Part B delivers its five sections, and chapter 11 as a whole passes
  `task docs:book:build`, strict STE, strict prose breaks, complete examples
  and excerpts. Any new command or source fragment names its real source/run
  and what it omits.
- The task log accounts for all current public tasks, every remaining README
  Develop statement, the two authority paragraphs and each evidence family.
  Missing or unavailable checks remain explicit. No human reader result or
  new platform qualification is inferred.
- The complete chapter is ready for the already-live copy-edit sibling
  `develop-ravel-k179`. Do not cut another copy-edit leaf.

## Notes

Part A has no unresolved chapter-local correction for part B. The remaining
scope is the planned second half. Copy-edit then cuts art, and art cuts proof,
under the editorial family rule.

## Decisions (running log)

- `docs/book/contract.md` supplies the structure precondition: the reader is a
  “16-year-old software engineer with no background in Ravel”, chapter 11
  teaches checkout, build, test seams and evidence, and the ordered chapter
  plan places it before delivery. Emphasis requires fake programs and stated
  support limits. Exclusions keep campaign execution and results outside the
  book. Part B follows the five headings reserved by part A.
- The chapter paraphrases task purposes rather than quoting descriptions.
  Taskfile commands and descriptions therefore remain unchanged. The complete
  inventory gives release tasks a chapter 12 number/title pointer until that
  chapter exists. The existing copy-edit leaf k179 is the successor.
- Installation trials use `/tmp/ravel-k178-checks/install-prefix`; smoke uses
  its own disposable prefix. No personal installation is a trial subject.
  Evidence notes remain hand-authored interpretations of dated runs, not
  generated task output. The chapter separates those notes from their logs.

## Source and coverage checks

Tier 2 verification used graph project
`Users-antony-Development-Ravel.simplify-and-componentize`. The coverage
generation is `2026-10-18T10:50:17Z`. Exact snippets identify the runner's
`controls` and `scrub` helpers and the CLI and hostile fixture commands.
The inbound `scrub` trace reaches its authority test and command helper.
The relevant searches and trace have no remaining pages. A first path filter
returned no matches; the broader name search found the helpers. No absence
claim rests on that empty result.

Coverage was checked for the source, scripts and documents used. Cargo's
configuration is excluded and was read in full. `scripts/examples.test.sh`
has partial parse coverage at lines 1–788; that whole range was read directly.
Other checked paths had no recorded issue. This signal does not establish
graph completeness. Taskfile, script bodies, conventions and dated evidence
sections supply the claims about commands and evidence. No architectural
decision changes, and the three ADRs remain unchanged.

## Contributor workflow inventory

Fresh `task --list` output contains 57 public names. The chapter's final table
names each exactly once and assigns a section or chapter 12 destination.
An extracted set comparison reports no missing or extra name. A removed-name
control fails that comparison. The internal `release` helper is not public.
The release rows name chapter 12 rather than a nonexistent file. Set proof
owns the later link. `docs:components` remains named and live.

Part A's complete README Develop destinations remain in k160. This continuation
closes every remaining row:

| Remaining README item | Destination |
|---|---|
| `task install PREFIX=…` | Install and test the installed pair: explicit disposable prefix, default prefix, layout, release build and installed checks |
| `task smoke` | Same section: fresh prefix, cleanup, direct and relative-symlink execution, tested behaviours and limits |
| `task probes` | Same section: four test workers, removed controls, identities, output location and hostile-test prerequisite |

Part A stays byte-for-byte unchanged, including its private-channel paragraph
and failed-then-passing check observation. No README statement loses its mapped
destination, and no guide is removed by this draft.

| Authority note content | Destination in Keep tests separate from live sessions |
|---|---|
| Tests cannot act as the enclosing session; forced-empty Cargo controls | Opening explanation, guard table and complete nine-name list |
| Empty disables authority rather than redirecting it | Paragraph after the name list |
| Direct runner scrubs | Guard table and direct-invocation explanation |
| CLI/hostile `env_clear`, temporary HOME/TMPDIR and fixture PATH | Guard table and new-test guidance |
| Installed smoke `env -i`; no personal policy or real completion path | Guard table and final fixture instructions |
| Legacy Grove/dispatch names are intentional extraction exceptions | Paragraph before the caller-reference link |
| Provenance paths and retired project are attribution; no task-tree dependency | Same paragraph, plus the evidence section's one-line retired-project pointer |

| Evidence family | Destination in Read the evidence for a change |
|---|---|
| `runtime-evidence.md` | Runtime-upgrade row: probes, hostile, integrity and smoke |
| `archive-evidence.md` | Archive row: release smoke, manifest and formula checks |
| `publication-evidence.md` | Dated release/delivery row: release tasks, downloads and Homebrew checks |
| `release-lifecycle-evidence.md` | Procedure row: isolated lifecycle controls and delivery checks |
| `result-job-source-evidence.md` | Resource/failure row: source read at a named revision |
| `completion-failure-design-evidence.md` | Repair row: focused tests, paired terminal check and full check |
| `native-entry-transition-design.md` | Design row, explicitly separate from execution |
| `native-entry-development-evidence.md` | Native row and instrument-family table, with current Linux section link |
| `docs/evidence/` | Receipt-directory row and explanation of note/command/source ownership |
| `caller-references.md`, `retired-project.md` | Isolation section's classification link and one-line historical pointer |

The book names instrument families and their scripts without campaign recipes
or campaign results. The only support statement is the contract's aarch64
macOS/Linux measurement and x86_64 emulation limit. Chapter 12 retains release
teaching. No new platform qualification or human reader result is inferred.

## Validation and handoff

- The whole-chapter read preserves part A's contributor sequence and adds the
  five reserved part B sections in order. Each new shell fragment comes from
  the recorded command sequence and names its omitted setup/output. No source
  fragment or product code changed. The chapter's evidence tables distinguish
  dated source/design notes from executed checks and their generated records.
- `docs/evidence/develop-ravel-k178/README.md` records the host, commands,
  timings, source freeze and limits. Install to a disposable prefix, smoke and
  probes passed. Sixteen recorded tasks passed, including the book build,
  thirteen figure comparisons, fourteen complete examples (zero skipped),
  eight excerpts, strict STE and strict prose breaks, all named checker
  controls and mutants, and offline book integrity. The 264-file source map
  stayed identical before and after. Generated outputs, host tools/caches and
  dependencies are explicit unmeasured inputs.
- The served chapter returned HTTP 200 with part B's boundary headings.
  This is not a browser inspection or change-triggered rebuild trial.
  `docs:figures:update` ran in the existing part A scratch checkout. All 26
  output files match the primary checkout, whose figures stayed unchanged.
- A final first-use policy-context link was added after the measured sequence.
  Build, scoped strict STE and prose breaks passed again, with the chapter's
  before/after digest unchanged. All 45 local link occurrences resolve.
  Missing-file and missing-anchor controls fail. The 57-name inventory was
  compared again, and a removed-name control fails. Part A still equals its
  parent revision byte for byte.
- Final evidence coverage reports generation `2026-10-18T10:56:00Z`. The new
  logs and text records lack tracked freshness metadata, so their contents and
  statuses were read directly. No conclusion rests on graph completeness or
  the trace's unrelated same-name `Validator.new` edge.
- No in-session reviewer was used. Source paraphrases and the executable
  controls provide this draft's evidence; no new architecture or product
  behaviour is proposed. Copy-edit k179 is already live, so the editorial
  successor rule requires no new leaf. Art and proof follow it. The document
  node and book node remain open. No chapter-local defect is handed forward.
- No release, Linux campaign, personal installation change or human reader
  test ran. The first part A full-check failure remains recorded with its
  unknown cause. README/authority removal, chapter 12 links and whole-book
  viewing remain with their existing later owners.
