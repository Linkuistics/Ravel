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

## Decisions (running log)

- The structure brief is `docs/book/contract.md`. “The reader” names the
  reader, and “What the reader can do afterwards” gives the outcomes.
  “The section plan” gives the order and purpose. “Emphasis” and “Exclusions”
  state the scope. The owner's k57 decision changes this existing contract.
- The preparatory change converts the touched contract sections and affected
  prose. It preserves the old structure until the task change follows.
  Untouched contract sections carry `pending` markers. Technical departures
  distinguish input checks, cleanup confirmation and immutable bounds.
- The book now teaches CLI use from another program as the answer to reuse.
  It supports no separate component. The private runner's process-wide
  signal effects explain the limit. No extraction plan or persistent-runner
  promise follows.
- Current chapter references move with the two renamed files. Dated evidence
  keeps its historical chapter numbers. Links into the renamed chapters change, including the specification and
  evidence records. The caller-reference inventory names the new file.
- Release-script messages and Taskfile comments remain with k199. The next
  copy-edit follows that leaf and reads the whole book.
- The wider search found one old range in `docs/book/excerpts.md` and two
  checker-script comments. The convention now says chapters 7–11. The k199
  body names both comments, which its existing completion condition covers.
  The convention's verb conversion has its own preparatory commit.
- The whole-book read found no further structure change for this draft.
  The eleven chapters keep the learning order and the caller, policy and
  contributor routes. The reuse route gives the CLI answer at its entry.
  The k191 log reports no chapter 9 finding for the later proof.
- The private-package claim agrees with `crates/keyed-launch/Cargo.toml`
  and `src/lib.rs` in that package. The signal-state explanation agrees with
  `src/run.rs`, including `restore_child_watching` and `run_with_output`.
  The owner's k57 decision supplies the supported reuse policy.
  Existing decision records need no change for this book correction.
- Graph verification used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, at Tier 2.
  Its recorded generation was `2026-10-18T14:48:50Z`. Coverage had no recorded
  source gap for the runner paths. Changed documents needed direct reads.
  The reference search read the files directly, including graph-excluded
  figure assets. It did not infer completeness from the graph.
- The reference check enumerated chapter references across the README and
  `docs/`. Current references use the new numbers. Historical evidence,
  figure records and captured command output retain their original labels.
  A scratch stale reference was detected. Its corrected form passed.
  The same search still detected historical names and k199's pending names.
- Final validation passed on 2026-10-21: book build, 14 complete examples,
  eight excerpts, 14 figures, strict book STE and strict prose-break checks.
  No example failed or skipped. The example results matched the earlier run
  item by item. The separate prose checker reported no hard findings in
  the affected prose and task files. Its advisory findings remain visible.
- Local link validation checked 1,598 links across 15 built HTML pages and
  16 inbound Markdown links to the renamed chapters. No local link broke.
  The checker rejected an obsolete chapter path and a missing anchor.
  It accepted the new development chapter's `prepare-a-checkout` anchor.
  This run made no claim about external websites or visual rendering.
- Before and after the final checks, SHA-256 digests matched for 4,642 inputs.
  These were tracked files outside `.grove/`, checkout tools and their assets,
  worker dependencies, the prose checker and the scratch link checker.
  Both manifests have digest
  `2ba237524845a0732712e04721d2993238453674b34380ef53f14cd3d6303d14`.
  Scratch logs and manifests are under `/tmp/ravel-k197-*`.
  The build outputs were outputs, not frozen inputs.
- The preparatory commits are `8b179b5c` for affected book prose and
  `77afe1c3` for the excerpt convention. The task commit carries the structure
  change, the copy-edit leaf and this leaf's retirement. The parent stays
  open for k199 and the remaining editorial stages.
