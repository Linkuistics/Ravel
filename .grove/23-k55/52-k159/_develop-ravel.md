# develop-ravel-k159 — brief

## Goal

Chapter 11 (`docs/book/src/11-develop-ravel.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
set up a checkout, build, use the test seams and `task check`, add a test, and
know what the evidence documents are for. It is the Part IV chapter for a
contributor, and it carries the book's inventory 5 (contributor workflows) for
development.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
The seam is the point where a reader can change code, run the right checks and
add a test. Part B starts from that and cuts nothing out of part A.

- **Part A: change code and check it.** Prerequisites and `task setup` (what it
  fetches and pins, and where); `task build`, `task worker` and `task
  typecheck`; the layout of the source (the two Rust packages, the worker, the
  SDK, the tests) as a map for a contributor, linking back to chapter 7 for the
  reasons; the test seams (runner tests with real processes and PTYs, the Ravel
  CLI's command-boundary tests with real workers, temporary policies and fixture
  processes, the hostile-runtime tests, the focused tasks `test:runner`,
  `test:session`, `test:cli`, `test:list`); `task check` and what it runs and
  writes (`target/check.*`); the focused-checks-while-editing and
  full-suite-at-handoff habit; and a worked example of adding a test (a
  fragment that is the smallest real test in each seam, verified to compile and
  pass in a scratch copy).
- **Part B: the other workflows, the guards and the evidence.** `task install`,
  `task smoke` and `task probes`; the book's own tasks (`docs:book`,
  `docs:book:build`, `docs:figures` and its update and test tasks, `docs:examples`
  and its tests, `docs:excerpts`, `docs:ste`, `docs:breaks`, `book:integrity`)
  and what each checks; the test-isolation guards and the legacy-name exceptions
  that `docs/authority.md` holds (forced-empty Cargo controls, scrubbing helpers,
  `env_clear` fixtures, installed smoke's `env -i`, the `GROVE_*` and
  `HARNESS_DISPATCH_*` extraction-safety exceptions); the evidence instruments
  that the book names and does not teach (the native-entry, incumbent and Linux
  measurement tasks); and a guide to the evidence documents, saying what each
  family is for and when a contributor reads it. It ends with the complete list
  of tasks (inventory 5).

## Context

- Needs chapters 1–3, 5 and 7, and the scaffold. Chapter 8 helps for “where a
  change belongs” and is linked from here. Chapter 12 owns the release tasks;
  this chapter names them and links.
- **Sources.** `Taskfile.yml` and `task --list` (derive inventory 5 from them
  again at drafting and at proof, not from this file); the README “Develop”
  section (kept until this chapter absorbs it, together with its “private
  session channel” paragraph and the contributor sentences that the first half
  moved there from `docs/session-cli.md`: the `task typecheck` description and
  list line, what `task test:cli` runs, “focused checks while editing, the full
  relevant suite at handoff”, and that `task check` rebuilds and typechecks the
  pair); `docs/authority.md`; `docs/caller-references.md`; `scripts/check.sh`;
  `.cargo/config.toml`; the tests under `crates/ravel/tests/` and
  `crates/keyed-launch/tests/`; `docs/runtime-evidence.md`,
  `docs/archive-evidence.md`, `docs/publication-evidence.md`,
  `docs/release-lifecycle-evidence.md`, `docs/result-job-source-evidence.md`,
  `docs/completion-failure-design-evidence.md`,
  `docs/native-entry-development-evidence.md`, `docs/native-entry-transition-design.md`,
  `docs/evidence/` and `docs/retired-project.md` for what each is.
- The guides use words that the book does not use (“front”, “admission”,
  “publication”, “settle”). Write in the book's words. The task descriptions in
  `Taskfile.yml` still say “front” in several places. If the chapter quotes a
  description, or lists the tasks with their descriptions, reword the
  descriptions in `Taskfile.yml` to “Ravel CLI” in the same change. That is a
  documentation edit only, and the task names and commands do not change.
- **Task coverage.** Every task in `task --list` is named in the chapter or in
  chapter 12, with a pointer to the chapter that teaches it. The native-entry,
  incumbent and Linux measurement tasks are evidence instruments: the chapter
  names them and points to the evidence they produce, and does not teach them.
- **Examples.** A command listing is a fragment that says what it omits, or a
  complete example that runs through `task docs:examples` when it can. The
  example check runs the build of the checkout through the real-policy and
  fake-program seam. Do not claim that the chapter's tasks were run end to end
  unless they were. Say which you ran and on which host.
- **Evidence documents.** They stay outside the book. The chapter says what
  each family of them is for, which task or script produces it, and when a
  contributor reads it, with a link to the file. It states no result from them
  except the support statement that the contract names (aarch64 macOS and
  aarch64 Linux are measured; x86_64 Linux is built and run only under
  emulation).
- Chapter 10 is not written. A contributor who adds a supported extracted
  component writes chapter 10 with it; say so in one plain sentence.

## Done when

- Chapter 11 exists, is linked from `SUMMARY.md`, and has passed all four
  stages.
- The chapter claims inventory 5 for development: every task in
  `task --list`, enumerated from the Taskfile at drafting and again at proof,
  appears in this chapter or chapter 12, and the log reports what is missing.
- The test-isolation guards and the legacy-name exceptions of
  `docs/authority.md` are in the chapter, so that the migration leaf can remove
  the file. The README's “Develop” section, its “private session channel”
  paragraph and the contributor sentences are each covered by the chapter, and
  the log lists the destination of each.
- Each fragment that shows a command, a test or a configuration file was taken
  from a real run or a real file, and says so.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem `develop-ravel` and part B's scope, then cut copy-edit
  under the family rule (it lands after part B). Part B's session cuts nothing
  if copy-edit already stands.
- Part A settles the heading structure for the whole chapter now and writes
  down in its log which headings part B owns. Do not emit empty headings.
- The README's “Develop” section is removed by the contributor and delivery
  fold-in leaf after the second half's set proof. This chapter does not edit
  the README except for what the node brief of the book says the README keeps.
- Read 2's task “build and test Ravel from a checkout” is this chapter's. The
  chapter must make it possible from a clean checkout and `task setup`.

## Handed forward

None yet.
