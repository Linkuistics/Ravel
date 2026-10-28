# rhai-book-use-k118

## Goal

Read chapter 6 against the full contract and lessons.
Then run the mechanical gates for all of chapters 1–6, and close node `rhai-book-use-k93`.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Drafts `rhai-book-use-k115` to `rhai-book-use-k117` changed chapters 1–5. Chapter 6 quotes their examples in fragments.
- Derive the CLI and configuration inventories of contract §1 from the source and the built CLI.
- Draft `rhai-book-use-k116` named chapter 5's examples. The `--json` review command now belongs to the example `notices`.
  Its real-run fragment in chapter 5 now shows a new run in `/tmp/ravel-chapter-5/notices`, with `selectionMs` 3. The script bytes and digest did not change.
  Chapter 6 quotes the earlier run in `/tmp/ch5r`. Its values are still those of that run, but its paths and label differ from chapter 5's fragment.

## Done when

- Chapter 6's fragments agree with the current examples of chapters 1–5. Search the whole document for each changed example (L64).
- Each entry's files, arguments, settings, fields, diagnostics and help agree with the implementation and the inventories.
- Links and reference fragments agree across chapters 1–6.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass. Strict prose and break checks pass for chapters 1–6.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.
- The node `rhai-book-use-k93` closes, with copy-edit `rhai-book-use-k94` already standing.
