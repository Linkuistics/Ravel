# rhai-book-use-k117

## Goal

Read chapters 1 and 2 against the full contract and lessons, especially the setup pattern and the renamed configuration interface.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Drafts `rhai-book-use-k115` and `rhai-book-use-k116` applied the setup pattern to chapters 3–5. Use their words.
- Chapters 1 and 2 have no complete example. They show real-run fragments and a configuration script that the reader reads but does not run.

## Done when

- Each fragment is visibly a fragment and says what it omits. Nothing in these chapters reads as a procedure that lacks its setup.
- Each recorded fragment agrees with a fresh run of its example, where its inputs changed (L64).
- Files, arguments, settings, fields, diagnostics and help agree with the implementation. No old literal-name explanation remains.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict prose and break checks pass for chapters 1 and 2.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change other chapters, except for a link or title that this draft breaks.
