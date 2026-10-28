# rhai-book-inside-k34 — brief

## Goal

Revise Parts III and IV of the Ravel book for Ravel 2.0.0, and prove the whole book. The chapters describe the Rhai worker, the host bridge, the new source layout, the new installation and the new release. The pipeline is draft, copy-edit, art and proof.

## Done when

- Chapters 7 to 11 describe the code and the delivery of Ravel 2.0.0. They name no Bun worker, no SDK and no `libexec` layout except in a statement of what was removed.
- Chapter 8 follows a session through the code that exists after the removal. Each excerpt is exact, and `task docs:excerpts` passes.
- The figures of these chapters match the text, and `task docs:figures` passes. The figures `installed-files`, `source-components`, `process-authority`, `session-code-path`, `selection-cancellation` and `release-delivery` need a check, and any other figure that names the worker.
- The book gets one complete example that shows the refusal of a lone `policy.ts`, as the removal makes it executable.
- Every complete example passes `task docs:examples`, and `task docs:ste` reports no problem.
- The proof stage has read the whole book, Parts I to IV, as one reader would. It finds no stale TypeScript, Bun or `libexec` statement, no broken link and no unresolved term.
- Chapters 7 to 11 follow the owner's Read 1 patterns in section 2 of `docs/book/contract.md`. The owner asked for the patterns in the entire book on 2026-10-27.
  The whole-book proof checks each pattern in all eleven chapters.
- The drafts apply all relevant rules from `docs/documentation-lessons.md`, including the later Part I findings F9–F16.
  Each new lesson preserves its source and gives its trigger, action, scope and a way to check the result for the future authoring skill.

## Decomposition

- `rhai-book-inside` (first draft) rewrites chapters 7, 8 and 9: the parts of Ravel, the walk through the code, and the jobs, signals and terminal.
- `rhai-book-inside` (second draft) rewrites chapters 10 and 11: develop Ravel and deliver Ravel.
- Copy-edit, art and proof follow. The draft stages cut them. The proof stage reads the whole book.

The two drafts share one stem, so name them by key. Chapter 9 changes least, because the runner is unchanged. The first draft checks that claim.

## Pointers

- Structure brief and prose contract: `docs/book/contract.md`, as updated by `rhai-book-use`. Its section 1 holds the reader, the section plan and the figure needs.
- Source of truth: the code after `typescript-removal`, `docs/specs/policy-evaluation.md` and `CONTEXT.md`. Check each claim against the source and the running binary.
- The running logs of the leaves `rhai-upgrade-task`, `rhai-harness-smoke` and `typescript-delivery-removal` hold the task names, the new layout and the delivery steps.

## Notes

- The owner completed Part I on 2026-10-28 and requested all lessons before Part II.
  The second correction run under `rhai-book-use-k26` ends at proof `rhai-book-use-k87` before Read 1 resumes.
  These later drafts inherit the current contract and lessons, including configuration terms, concrete examples and descriptive headings.
  Check shared figures against the corrected prose and keep exact interface tokens where the reader must use them.
- Parts I and II proof passed the final notice-path choice to `typescript-delivery-removal-k32`.
  After that leaf settles the installed path, update chapter 6's installed-layout table and `installed-files.typ` with its SVG, layout record and alternative.
  The whole-book proof checks those entries against the final archive and installed layout.
- `rhai-book-use-k72` updated the shared figures for Rhai in Parts I and II.
  Chapter 8 still embeds `selection-cancellation.svg` beside its old worker-signal example and alternative.
  Reconcile that example, alternative and surrounding explanation during the first draft.
  The figure now shows a fake evaluation harness sending INT, then harness cleanup before worker teardown.
  The shared process, source and installation views now show the same executable for the CLI and policy worker.
- Chapter 7 teaches why Ravel has a worker. The answer changes. The worker is Ravel's own binary in an internal role, and the CLI performs every host capability for it. The first draft teaches the reasons that the spec and the ADR give.
- Chapter 11 describes the release. The release publishes the book, so the book must be final before the release leaves run. Describe the rule that `brew upgrade` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. Do not describe the owner's machine.
- Section 4 of the contract places Read 2 after the proof of Parts III and IV. The read is a `requirements` leaf, and it is not optional. The proof stage cuts it, unless section 4 of the contract no longer requires it. The answer of the owner to the fourth question of `rhai-book-use-k27` decides this.
- Run the checks of the book with `task setup` done first.
