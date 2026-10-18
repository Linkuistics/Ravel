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
