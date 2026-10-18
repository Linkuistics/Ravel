# develop-ravel-k180


## Goal

Read the whole of chapter 11, `docs/book/src/11-develop-ravel.md`, after
copy-edit k179. Own its figures, its tables as visual breaks, and their
adjacent prose under `docs/book/contract.md`, `docs/book/figures.md` and
`docs/book/ste.md`.

## Context

- The chapter has no figure now. Its prose runs are broken by tables and code
  blocks, and strict `task docs:breaks` passes. Do not skip art because of
  this. Decide each question below, and record the decision and its reason.
- **Source layout.** “Find the source for a change” maps paths to their
  contents in a table. Chapter 6 holds `source-components` (C4 component
  view) and `installed-files` (UML deployment view). The second-half brief
  says chapters 7 and 12 reuse the figure for packages, worker and SDK.
  Decide whether this section embeds `source-components`, links to it, or
  keeps only its table.
- **Install section.** “Install and test the installed pair” links to chapter
  6's installed layout. Decide whether `installed-files` belongs beside the
  `PREFIX/bin/ravel` and `PREFIX/libexec/ravel/` sentence.
- **Test seams.** The seam table has seven rows: unit, command boundary,
  runner subprocess, private PTY, hostile runtime, the Ravel package and the
  workspace.
  Decide whether a figure that shows which processes each seam runs teaches
  more than the table. If you draw one, the runner is inside the Ravel CLI,
  and the fixture role is a separate run of the same test binary.
- **Isolation guards.** “Keep tests separate from live sessions” has a guard
  table and a list of nine control names. Decide whether a figure of the
  enclosing session, the Cargo controls and the fixture environment adds
  information. An empty value disables a control and does not redirect it.
- **Wide tables.** Check at the 750-pixel column: the instrument table in
  “Read the evidence for a change”, the evidence table and the task
  inventory. Decide whether any cell is too long to read there.
- Copy-edit k179 changed no table structure, fragment or dated observation.
  Its log has the reviewer's findings and their classification.

## Done when

- Each question above has a recorded decision. Each new or reused figure
  follows `docs/book/figures.md`, with STE caption and alt text.
- Build, figures, examples, excerpts, strict STE and strict prose breaks pass
  on the chapter after the last edit.
- Last act: add the proof leaf with bare stem `develop-ravel`, unless a live
  later sibling already holds it.

## Notes

Dense figure text and the book's total figure size stay with whole-book art
k101.

## Decisions (running log)

- Read the whole chapter, the brief chain, glossary, three cited ADRs and
  the book's prose and figure contracts. The working copy began clean.
- Source layout: keep the path table and add a link to chapter 6's source
  view. The table answers where to edit, including tests and scripts that
  the component view omits. The optional view answers process membership.
  Declining an embedded copy is an editorial judgement, not a contract
  restriction or a limit of the medium.
- Installation: keep the link to chapter 6's installed layout. The local
  sentence gives the two destinations, and the task table compares what
  install, smoke and probes leave. The deployment view would repeat that
  context with SDK and archive detail this section does not need. This is
  an editorial judgement, not a contract or medium limitation.
- Test seams: retain the seven-row comparison. A process diagram spanning
  unit tests, PTY roles, hostile probes and aggregate suites would conflate
  a test boundary with a selection of many tests. Add a small page-native
  table at the foreground example instead: parent, launcher role and child,
  with the two statuses explicitly assigned. The separate launcher is a
  run of the same test binary, with the runner linked inside it. This
  editorial choice uses a medium that the contract already admits.
- Isolation: keep the boundary/guard/source table and the nine-name list.
  They already map every named boundary to its guard and identify the
  controlled names. An enclosing-session diagram would add no relation and
  could suggest that the guards form one mandatory chain. The sentence
  distinguishing disabling from redirecting remains. The declined diagram
  is an editorial judgement, not a contract or medium limitation.
- Wide tables: Chrome's local light mdBook shows a 750 px main column in a
  1227 × 919 viewport. All fourteen original tables fit that width. The
  evidence table's nine rows remain legible, so keep its structure. Long
  instrument names split at hyphens in the instrument table and inventory.
  Put each native-instrument name on its own line in those cells. Keep the
  compact short-name rows in the inventory.
- Each table needs adjacent role text. Add short introductions where the
  paragraph names the topic without explaining how to use the comparison.
  No fragment, dated observation, heading, task name or task destination
  needs to change.
- Source check for the new table: Tier 2 graph project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T11:09:26Z`. The exact `in_pty`, `role_command` and `launcher`
  snippets in `crates/keyed-launch/tests/job.rs` confirm separate process
  execution, the same test executable, the linked `run` call and the child
  status in the report. The outbound trace has 17 entries with no further
  page. Spurious name matches in that trace are not evidence. All eighteen
  checked source/context paths report no recorded coverage issue and
  matching metadata. This is bounded source evidence, not a runtime result.
- The first line-break trial made the instrument table's task column narrower
  and split more names. Group each instrument family in the first column,
  with its task names, unchanged purpose and evidence link in the wide second
  column. The inventory retains its two columns. No custom CSS or smaller
  type is needed.
- The first strict STE run found a seven-sentence paragraph beside the book
  check table. Put that table's role sentence in its own paragraph.
- The complete check pass found one more STE sentence-length violation in
  the reorganised instrument cell. Split the fixture-build instruction from
  the test instruction. Build, 13 figures, 14 examples, eight excerpts and
  prose breaks passed on that intermediate input. All 215 digested source
  inputs stayed unchanged throughout the run. Rerun after this final edit.
- The PTY seam row now says "linked runner in a fixture process", consistent
  with the new table and the checked source. This is a table clarification,
  not a new test arrangement.

## Verification and limits

- After the final chapter edit, all six commands exit 0:
  `task docs:book:build`, `task docs:figures` (13 matches),
  `task docs:examples` (14 passed, none failed or skipped, no page errors),
  `task docs:excerpts` (eight passed), `task docs:ste -- --strict` on chapter
  11 (no problems), and `task docs:breaks -- --strict` on chapter 11 (no long
  run). The example cases match the prior run item by item.
- SHA-256 digests match before and after for 215 tracked inputs under
  `docs/book/`, `scripts/`, `crates/` and `.cargo/`, plus the root Cargo
  manifests, Taskfile, glossary and README. The final chapter digest is
  `531bf49674cce82d9c529b1987b58f9d1b5dd30a39f527ea4bed2fbbdf562c45`.
  Scratch manifests and logs are under `/tmp/ravel-k180-checks/` and are not
  durable evidence artifacts. The generated tools' pin checks passed as part
  of the figure and build commands.
- Direct comparison with the parent revision confirms all nine fenced
  blocks, fifteen headings and 57 task names in the inventory are unchanged.
  The inventory keeps the same order. This checks preservation, not a fresh
  enumeration of Taskfile coverage, which belongs to proof.
- The new chapter 6 link resolves to its generated `installed-layout`
  section, with the source view and SVG present. Existing source and
  installation views were read but not changed or embedded here.
- Chrome's final DOM has fifteen tables, each 750 px wide with a 750 px
  scroll width. Screenshots cover the complete wide tables, the new process
  table, source map, seam table, installation table and isolation guards.
  The dated viewing record in `docs/book/figures.md` states the environment
  and limits. No editor preview, other theme, narrow viewport, GitHub or
  GitHub Pages view was checked. No reader-comprehension or new runtime or
  platform claim follows.
- No in-session reviewer was needed. The new table maps the existing example
  to exact fixture source, and no architectural decision changed. No defect
  is handed forward. Whole-book art k101 retains dense shared figure text
  and figure-set size. The document node remains open through proof.
