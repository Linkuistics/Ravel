# book-contributor-delivery-fold-in-k165

## Goal

Finish the guide migration for chapters 11 and 12. Remove the release guide
`docs/RELEASING.md` and the authority note `docs/authority.md`, once chapters 12
and 11 absorb them, reduce the README to a short entry into the book, and
redirect every live inbound link. This follows the second half's set proof
(`book-second-half-k163`) and `book-architecture-fold-in-k164`, so that
“absorbed” is a statement about proved chapters.

## Context

- Contract: `docs/book/contract.md` ("Relation to other documents"): the book
  replaces the release guide and the authority note. The authority note is
  already reduced to the test-isolation guards and the legacy-name exceptions
  (`book-readme-and-authority-k69`), and “removed when chapter 11 absorbs
  those”. The contract's clause is edited in place then, to state the result.
- Method: the same as `book-guide-fold-in-k68` and `book-architecture-fold-in-k164`
  (sentence-level classification in the log, the search for inbound references
  with controls, correction runs for sentences with no home). Read both logs.
- **Input: the node brief of the book** (“Handed forward to the second half”)
  and the logs of chapters 11 and 12, which list the destination of each
  README “Develop” sentence, the “private session channel” paragraph, the two
  release paragraphs under “Earlier project”, the notices line, each section of
  `docs/RELEASING.md` and each guard of `docs/authority.md`. Verify each from the
  proved chapters. A sentence with no home goes to the owning chapter by a
  correction run, and this leaf stays live and removes nothing until the run is
  done.
- **The README.** It ships in the release archives and is the repository's door
  (`book-readme-k146` wrote the opening). After this leaf it holds what Ravel
  is (as now), the two verbs, where the book is and how to build it
  (`task setup`, `task docs:book`), the link to the book on GitHub Pages once the
  site exists (it does not exist before the next release: say so or omit it, and
  record which), the licence line and the pointer to the retired record. Remove:
  the “Develop” section (chapter 11), the paragraph on the private session
  channel, the two release paragraphs under “Earlier project” (chapter 12), and
  the reference paragraph that names the guides (replace it with a pointer to
  chapters 7–12 by number and title, as links). The notices line (checkout path
  versus installation path) is in chapter 12; verify that it is, then remove it
  from the README or keep one sentence if an archive reader has no book (a
  release archive does not hold the book). Rename the “Earlier project” heading
  so that it describes what is left under it. Keep the README in STE:
  `task docs:ste -- --strict README.md` passes.
- **Live references to the removed files.** Enumerate them with a search that
  you first see find known hits: `scripts/release-prepare.sh` (two messages that
  say to resume using `docs/RELEASING.md`), `scripts/release-smoke.sh` (the Zig
  message), the comment in `Taskfile.yml` near the release task, `README.md`,
  `docs/publication-evidence.md`, `docs/caller-references.md`, and
  `docs/book/contract.md`. A script message that tells a person where the
  recovery steps are must name the chapter 12 section and its path in a
  checkout (`docs/book/src/12-deliver-ravel.md`, with the anchor for “Recover a
  partial release without another bump”). The tests of those scripts may match
  the message text: run `task release:lifecycle:test`, `task release:test` and
  `task release:doctor` after the change, and update the tests with the message.
  Do not run a task that publishes or tags.
- **Frozen receipts** that name the removed files (the `subjects-*.json`,
  `source-paths.txt` and `*-checks.txt` files) are not edited, for the reason
  that `book-architecture-fold-in-k164` records. Confirm that no script or
  task reads those names as live input.
- `docs/caller-references.md` has a row for `docs/authority.md` and this
  inventory. Update the row and the inventory's account of the guards to point to
  chapter 11, and re-run the inventory's instrument with its controls.
- Chapter 10 is not written. Do not write it.

## Done when

- `docs/RELEASING.md` and `docs/authority.md` are removed and no live document,
  script or task points at them. The README is the short entry above. The
  contract's “Relation to other documents” states the finished migration in
  place.
- The log holds the classification of every sentence group, the list of inbound
  references with their classes, and the controls seen to fail.
- Every redirected link resolves, with a broken-anchor control that fails.
  `task docs:book:build`, the strict STE and prose-break checks on the files
  changed, `task docs:figures`, `task docs:examples`, `task docs:excerpts`,
  `task release:test`, `task release:lifecycle:test`, `task release:doctor` (it
  may report what the host lacks: record exactly what) and `task check` pass on
  subjects whose digests are the same before and after.
- The log says which statements of the removed files survive, where, and which
  were deleted with the reason.
- No guide of the contract's list remains except the ones the contract keeps:
  the normative specifications, the decision records and the evidence
  documents.

## Notes

- Removal is the point. Do not rewrite chapters 11 or 12 here, except for a link
  that the move makes wrong.
- The README is checked in a release archive by name, not by content. Confirm
  this in `scripts/release-archives.test.sh` and the manifest code before you
  change the README, and record the line that shows it.
