# rhai-book-use-k116

## Goal

Before a procedure in chapter 5 asks the reader to act, make it supply its example and files.
It also supplies its context and complete command first.
Reconcile chapter 5 with the renamed configuration interface.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Draft `rhai-book-use-k115` applied the setup pattern to chapters 3 and 4. Use its words and order. Its running log records its choices.
- Chapter 5's “Run the examples of this chapter” section has the shape that F17 found in chapter 4.

## Done when

- Each chapter 5 procedure names its example and shows its workspace, files, purposes and contents before any action.
  It distinguishes the reader's files from what the command creates, and shows a complete command before its parts.
- Files, arguments, settings, fields, diagnostics and help in chapter 5 agree with the implementation.
  Re-run each changed example whose real-run fragment records its output (L64).
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict prose and break checks pass for chapter 5.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change other chapters, except for a link or title that this draft breaks.
