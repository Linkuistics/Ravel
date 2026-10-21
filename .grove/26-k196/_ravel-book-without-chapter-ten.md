# ravel-book-without-chapter-ten-k196 — brief

## Goal

Make the book agree with the owner's decisions of 2026-10-21 in
`component-work-scope-k57`. Ravel supports no separate component, so the book
has no chapter 10. “Develop Ravel” becomes chapter 10, “Deliver Ravel” becomes
chapter 11, and Part IV becomes “Develop and deliver Ravel”. The book says why
Ravel offers no separate component.

## Done when

- `docs/book/contract.md` states the new structure: the section plan, the
  reader outcomes, the roles, the entry routes, the coverage inventories and
  the reader checkpoints. No text in it plans a chapter 10 or names component
  consumers as readers.
- `docs/book/src/SUMMARY.md` has no chapter 10 entry, and Part IV is “Develop
  and deliver Ravel”. The two chapter files carry their new numbers.
- Each reference to chapters 10–12 names the right chapter. This covers the
  book, the contract, the README, the documents under `docs/`, the release
  scripts and the Taskfile. A search with a control that finds a planted stale
  reference shows that no stale reference remains.
- Each link to a renamed file resolves. A link check with a control that
  fails on a planted broken link shows this.
- Each passage that mentions chapter 10 says that Ravel supports no separate
  component, and why. These passages are in the README, chapters 1, 6, 7 and
  9, and the old chapters 11 and 12.
- The run went draft → copy-edit → art → proof under this node, and the proof
  read the whole book.
- These tasks pass: `task docs:book:build`, `docs:examples`, `docs:excerpts`,
  `docs:figures`, `docs:ste -- --strict`, `docs:breaks -- --strict`,
  `release:lifecycle:test` and `check`.

## Decomposition

- `ravel-book-k197` (draft) changes the contract and the structure of the
  book. It renames the two files, updates each reference and inbound link in
  the Markdown documents, and writes why there is no separate component. Its
  last act cuts the copy-edit under this node, after the impl leaf.
- `release-script-chapter-names-k199` (impl) changes the release-script
  messages and the Taskfile comments to the new chapter numbers. Code is
  outside the editorial charters, so it has its own leaf. It runs after the
  draft and before the copy-edit.
- Copy-edit, art and proof follow, and each stage cuts the next one under
  this node. No figure names chapters 10–12. The art stage still runs, because
  the stages are mandatory.

## Pointers

- The owner's decisions and their evidence are in the log of
  `component-work-scope-k57`, under question 2 and “The book without chapter
  10”.
- These are the facts that the book states in place of chapter 10:
  - Ravel's runner, `keyed-launch`, is a private package. Ravel made and
    measured it for a launcher that exits after one session. It can change the
    signal state of the whole process, and its return does not restore that
    state.
  - A program that needs supervised harness sessions starts the `ravel` CLI
    as a child process. Grove does this too (`Linkuistics/grove#22`).
  - No consumer needs a persistent runner. The term stays only as a limit,
    as the contract's “Undelivered contracts” requires.
  - Extraction can start again when a project needs Ravel's runner without
    Ravel's CLI. That work adds its own chapter and renumbers. The book does
    not present this as a plan.
- `adr-native-waitability-status-k191` runs before this node. If its log
  sends a chapter 9 finding to a whole-book proof, the proof of this node
  takes it.
- The writing rules are those of the `linkuistics:technical-communication`
  skill, with its checker `ste-lint.py`, and the book's own `task docs:ste`.
  At commit `dbec471f`, the contract has 6,102 words and 130 hard findings
  under `ste-lint.py`. Thus, a session that edits it first converts the
  sections that it touches, in its own commit, and marks the other sections
  `pending`. The chapters have up to eight hard findings each.
- The evidence documents under `docs/` are records. Change only the link
  targets in them, and do not convert them.

## Notes

- The first publication of the book comes with release 1.1.0, after the
  finish of this workstream (`Linkuistics/Ravel#3`). No published page or
  outside link uses the old chapter names yet.
- `Linkuistics/Ravel#2`, Rhai and Rig in place of TypeScript policy, is a
  separate workstream after 1.1.0. The book does not mention it.

## Handed forward

- **proof**, chapter 11, end of “Set up Pages for the first release”: the
  paragraph that says Ravel supports no separate component sits under the
  Pages setup heading. The draft put it where the earlier chapter-10
  sentence was. The copy-edit tied its first sentence to the book that a
  release carries, and did not move it. Judge whether the reader finds it
  there. A move is a structure change for a draft re-run.
