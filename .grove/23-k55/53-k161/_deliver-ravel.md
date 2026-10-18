# deliver-ravel-k161 — brief

## Goal

Chapter 12 (`docs/book/src/12-deliver-ravel.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
handle versions and identity, install and smoke tests, archives and platform
floors, the formula and release tasks, and the supported targets with their
limits. It is the last chapter of the book and carries inventory 5 for the
release tasks.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
The seam is the point where a reader can build and check unreleased archives on
a machine, without cutting a release. Part B starts from that and cuts nothing
out of part A.

- **Part A: versions, identity, archives and the checks before a cut.** The one
  Ravel version and the source identity that the Ravel CLI and the worker must
  share (and the refusal when they differ); the archive layout and its manifest;
  `task release:archives`, `release:smoke` and `release:test`; the platform
  floors and how each is measured (native arm64 macOS, arm64 Linux in Docker,
  x86-64 Linux under emulation at its CPU floor, glibc 2.17); the installed-layout
  smoke seam, direct and through a symlink; `task install` and `task smoke`
  from chapter 11 as the first rung; and what is measured and what is not. The
  packages, worker and SDK in an installation reuse the chapter 6 figure.
- **Part B: the formula, the release tasks and recovery.** The Homebrew formula
  (`task release:formula`, the template in `scripts/templates/`, the tap
  `linkuistics/taps/ravel`); `task release:doctor`, `release:notes`,
  `release:patch`, `release:minor` and `release:major`, and `release:lifecycle:test`;
  the version cut with `cargo release` (`release.toml`) and what it writes; the
  version line of the book (below); the activity figure for how a release is
  built and checked; the supported targets with their stated limits; what goes
  to GitHub Pages with a release and what does not exist before the first
  release that carries the book; and “Recover a partial release without
  another bump”.

## Context

- Needs chapters 1–3, 5 and 7, and chapter 11 for the tasks it teaches. The
  activity module is `book-figure-activity-k153`; the release figure is drawn by
  this unit's art stage with it.
- **Sources** (verify against the code and the scripts, not the guide):
  `docs/RELEASING.md` (all sections), `release.toml`, `CHANGELOG.md`,
  `Taskfile.yml` (the `release:*` tasks), `scripts/release-*.sh` and their
  tests, `scripts/templates/ravel.rb.tmpl`, `scripts/installed-check.sh`,
  `crates/ravel/scripts/installed-smoke.sh`, `docs/specs/session-execution.md`
  (“Installed boundary”, “Archives and platform floors”), `docs/runtime-evidence.md`,
  `docs/archive-evidence.md`, `docs/publication-evidence.md`,
  `docs/release-lifecycle-evidence.md` and the notices under
  `crates/ravel/notices/`. The evidence documents stay outside the book.
- **`docs/RELEASING.md` is stale at its start.** It says that development is
  0.0.0 and that the first minor release is planned at 0.1.0. The repository
  holds tags `v0.1.0` and `v1.0.0`, and the workspace version is 1.0.0. Check
  every statement of the guide about versions, tags and baselines against the
  current state, and write what is true.
- **The release writes the version into chapter 3 (R17).** The node brief of
  the book (“Handed forward to the second half”, the bullet for chapter 12)
  records the mechanism, which `book-release-version-k145` built: the single
  version line, the second `pre-release-replacements` entry in `release.toml`,
  and the two guards in `scripts/release-book-version.sh` (`ready`, run by
  `task release:doctor`, and `released vX.Y.Z`, run by the release task after the
  cut and before the build), with its test `scripts/release-book-version.test.sh`.
  Teach it: what a checkout between releases shows, what the cut writes, what
  each guard refuses, and the recovery. Verify it against the files.
- **The README's two release paragraphs** under “Earlier project” (the matched
  layout, `task release:smoke`, `task release:test`, `task release:formula`) and
  its notices line (checkout path versus installation path) are covered by this
  chapter. The log lists the destination of each sentence.
- Do not run `task release:patch`, `release:minor` or `release:major`, and do
  not push or tag. They publish. Use `release:doctor`, `release:lifecycle:test`
  and the dry runs the scripts provide. State in the chapter which release tasks
  were run for it, which were not, and why.
- **Support statement** (contract): aarch64 macOS and aarch64 Linux are
  measured; x86_64 Linux is built and run only under emulation, and native
  x86-64 qualification is tracked in Linkuistics/Ravel#1. Native x86-64 Linux
  is unqualified. State this once, plainly, here, and link from other chapters.
- The book on GitHub Pages: the release procedure puts it there with each
  release; the first publication happens with the next release, outside this
  node. Use the words “the book on GitHub Pages”, as `docs/book/ste.md`
  directs, and say that no published site exists before that release.
- Chapter 10 is not written. Name it in plain text if a release has to carry an
  extracted component.

## Done when

- Chapter 12 exists, is linked from `SUMMARY.md`, and has passed all four
  stages.
- The chapter claims inventory 5 for delivery: every `release:*`, `install`,
  `smoke` and archive task of `task --list` is covered here or in chapter 11,
  enumerated again at drafting and at proof, and the log reports what is
  missing.
- Every statement about a version, a tag, a floor or a target was checked
  against the repository at the checkout, and the chapter says what no check
  measures.
- Each statement of `docs/RELEASING.md` that this chapter replaces is listed in
  the log with its destination (or with the reason it stays outside the book),
  so that the migration leaf can verify it.
- The chapter teaches the release writing of the version line and the two
  guards (R17), consistent with chapter 3's placeholder text.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem `deliver-ravel` and part B's scope, then cut copy-edit
  under the family rule (it lands after part B). Part B's session cuts nothing
  if copy-edit already stands.
- Part A settles the heading structure for the whole chapter now and writes
  down in its log which headings part B owns. Do not emit empty headings.
- Read the tasks' real output where it is cheap (`task release:doctor`,
  `task release:lifecycle:test`, `task release:test`). `task release:smoke`
  needs Docker, Zig and the cross targets; run it only when the host has them,
  and say which host you used.

## Handed forward

- Art: chapter 12 links to chapter 6's installed-layout view in “Check the
  archive contents”. Reuse that figure as the contract requires. Part B supplies
  the release procedure for the activity figure. The platform table distinguishes
  four execution routes and their evidence limits.

## Settled chapter order

Part A writes “Keep the version and pair together”, “Build archives before a
release”, “Check the archive contents”, “Test the installed archive”, and
“Read the platform checks correctly”. Part B then adds “Prepare and test the
Homebrew formula”, “Prepare a version cut”, “Cut and deliver a release”,
“Keep the book with its release”, and “Recover a partial release without
another bump”. “Checks made for this chapter” stays last and takes both parts'
dated results. Part B preserves part A's support statement and extends its
teaching without a duplicate support section.
