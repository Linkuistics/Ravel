# ravel-book-k197

## Goal

Make the structure change that the node brief describes: the book without
chapter 10. Write the text that explains it. Change the contract first. Then
change the book, the README and the inbound links of the Markdown documents.

## Context

- The node brief `ravel-book-without-chapter-ten-k196`, and the log of
  `component-work-scope-k57`.
- These places mention chapter 10, reuse or component consumers at commit
  `dbec471f`. They come from a search for “chapter 10”, “extracted component”
  and “component consumer”. Search again, because lines can move.
  - `docs/book/contract.md`:
    - the reader outcome (line 65)
    - the role “component consumers” (line 133)
    - the section plan (lines 136–176)
    - the inventory note (line 333)
    - the reader checkpoint for chapter 10 (line 620)
    - the names of chapters 11 and 12 (lines 168–170, 395 and 410–412)
  - `README.md`, lines 38–47.
  - Chapter 1 (lines 254–257), chapter 6 (line 1072), chapter 7 (lines
    220–222), chapter 9 (lines 566–570), chapter 11 (line 12) and chapter 12
    (lines 603–604).
  - `docs/book/book.toml`: its description names “reusing”.
- Links into the two renamed files are in chapters 1, 3, 4 and 12, in
  `SUMMARY.md` and in `README.md`. Three evidence documents also link to them:
  `docs/component-evidence.md`, `docs/publication-evidence.md` and
  `docs/caller-references.md`.
- The root brief's Notes name chapter 11 (its 22 links to `main`) and chapter
  12 (the setup of Pages). Change those numbers too.
- The release-script messages and Taskfile comments belong to
  `release-script-chapter-names-k199`, after this leaf. Do not edit them here.

## Done when

- The node brief's conditions for the contract, `SUMMARY.md`, the file names,
  the Markdown references and links, and the passages about chapter 10 hold.
- `task docs:book:build`, `docs:examples`, `docs:excerpts`, `docs:figures`,
  `docs:ste -- --strict` and `docs:breaks -- --strict` pass.
- This leaf's last act cuts the copy-edit under the node, with the stem
  `ravel-book`. It lands after `release-script-chapter-names-k199`.

## Notes

- Ravel's statements must not grow. Say what the owner decided. Keep each
  limit that the book states now: the persistent runner, the evidence limits
  and native x86-64 Linux.
- Chapter 1's route for reuse is a reader's entry. Give that reader an
  answer. Ravel supports no separate component, and a program uses the
  `ravel` CLI as a child process.
