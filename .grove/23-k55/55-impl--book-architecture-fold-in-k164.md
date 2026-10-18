# book-architecture-fold-in-k164

## Goal

Remove the three architecture guides that chapters 7, 8 and 9 replace,
`docs/components.md`, `docs/local-runner.md` and `docs/session-channel.md`,
together with the editable Mermaid views under `docs/design/components/` that
the contract says become figure sources. Redirect every live inbound link, and
keep what is evidence outside the book. This follows the second half's set
proof (`book-second-half-k163`), so that “absorbed” is a statement about proved
chapters.

## Context

- Contract: `docs/book/contract.md` ("Relation to other documents"): the book
  replaces the component guide, the local runner guide and the result-channel
  guide, each removed once the chapters that absorb it land, with every inbound
  link redirected. The evidence documents, the caller-reference inventory and
  the retired-project record stay outside the book.
- The first half did the same for two guides in `book-guide-fold-in-k68`. Read
  its log for the method (the sentence-level classification, the table in the
  log, the search for inbound links with controls) and follow it.
- **Input: the units' logs.** Chapters 7, 8 and 9 each record the destination
  of the guide sentences they replace, and the set proof records which
  sentences no unit placed. Use them, then classify every sentence group of the
  three guides yourself from the proved chapters, not from the lists alone.
  Each group lands in one of: a chapter (name the section), an evidence
  document that stays outside the book, or deletion with the reason (stale,
  duplicate, history). A sentence with no home is not dropped: send it to the
  chapter that owns its subject by a correction run (`leaf-add` the owning
  stage and `proof` under that unit's node, or a `draft`, `copy-edit` and
  `proof` run, in pipeline order), remove nothing, and leave this leaf live so
  that it runs again after that run. A second send-back of one stage against
  one document goes to the owner.
- **Evidence in the guides.** `docs/components.md` ends with “Source and
  observation evidence” (E1–E7, the observed scenarios and their limits), and
  `docs/local-runner.md` ends with “Source provenance” (the source project's
  revision and URLs). Both are evidence, not teaching. Keep them in an evidence
  document outside the book (a new file such as `docs/component-evidence.md`, or
  an existing evidence document that fits), with a title and an opening that say
  what it is. `docs/caller-references.md` has a row that names
  `docs/local-runner.md` as historical attribution; update that row to the new
  home, and re-run the inventory's instrument with its controls as the file
  says.
- **Inbound references.** Enumerate them with a search that you first see find
  known hits (`README.md`, `docs/specs/session-execution.md`,
  `docs/adr/the-launched-child-is-a-job.md`, `docs/adr/ravel-supervises-the-session.md`,
  `docs/design/result-job/README.md`, `Taskfile.yml`). Classify each:
  - **live prose or link**: redirect to the chapter and anchor that teaches the
    subject, or to the evidence document (the specification and the ADRs are
    normative: change a link target only, never a statement; `ADR-FORMAT.md`
    says how to chase citations);
  - **frozen receipt** (the `subjects-*.json`, `source-paths.txt` and
    `*-checks.txt` files under `docs/` and `docs/evidence/`): these record
    digests and paths of files as they were. Do not edit them. State in the log
    that a frozen receipt names the removed files and why that is correct. Check
    that no script, test or task reads those names as live input (the earlier
    search found none in `scripts/`, `crates/` and `Taskfile.yml`; confirm it).
- **The Mermaid views.** The Taskfile task `docs:components` serves
  `docs/design/components/`. Once the chapters hold their own figures, the views
  have no reader. Remove the directory and the task, and make sure that
  `task --list` and the README no longer mention them. The sibling
  `docs/design/result-job/` views and the `docs:result-job` task are evidence
  views and stay.
- The README's reference paragraph for the unwritten Parts III and IV (it names
  `docs/components.md`, `docs/local-runner.md`, the specification and the runtime
  evidence) is the next leaf's, together with the rest of the README. Redirect
  only the links that this leaf breaks, and leave a working README.
- **Chapter 10.** Content that only a supported extracted component will use
  is not placed in a chapter. If a guide holds such content, keep it in the
  evidence document with a pointer in the log, and do not write chapter 10.

## Done when

- The three guides and `docs/design/components/` are removed, the
  `docs:components` task is removed, and no live document or task points at
  them. The log holds the classification of every sentence group, the list of
  inbound references with their classes, and the controls that were seen to
  fail (a planted dead link to a removed file is found by the instrument).
- Every redirected link resolves, and a control with a broken anchor fails.
  `task docs:book:build`, the strict STE and prose-break checks on the files
  changed, `task docs:figures`, `task docs:examples`, `task docs:excerpts` and
  `task check` pass on subjects whose digests are the same before and after.
- The evidence document (if any) is in STE, states what it is for, and is
  linked from chapter 11's guide to the evidence documents.
- The log says which statements of the three guides survive, where, and which
  were deleted with the reason.

## Notes

- Removal is the point of this leaf. Do not rewrite chapters 7, 8 or 9 here,
  except for a link that the move makes wrong.
- Do not run any task that publishes or tags.
