# rhai-book-inside-k35

## Goal

Draft the changes to chapters 7, 8 and 9 of the Ravel book. A reader learns the parts of Ravel 2.0.0, follows a session through its code, and understands jobs, signals and the terminal.

## Context

- The structure brief is `docs/book/contract.md`. The brief of this node lists the other inputs.
- Chapter 7 explains the CLI and its separate worker. It describes a Bun worker now. Rewrite it for the worker as Ravel's own binary, the host bridge, the reader thread, the LLM adapter and the harness runs.
- Chapter 8 follows a session through the code with 8 exact excerpts. The code has changed, so choose new excerpts of the Rhai path. Follow the successful, refused and cancelled paths.
- Chapter 9 covers jobs, signals and the terminal. The runner is unchanged, so edit only what the new host calls change: the harness runs and the ordered stop.
- Every complete example runs through the CLI test seam. Every excerpt is whole lines of a tracked file. Read `docs/book/examples.md` and `docs/book/excerpts.md`.
- If the leaf proves too big, decompose it by chapter.

## Done when

- Chapters 7, 8 and 9 describe the code that exists. Each claim is checked against the source.
- The excerpts of chapter 8 follow the Rhai path from `main` to the result write, and `task docs:excerpts` passes.
- Chapter 9 shows the ordered stop with a harness run in progress, and the precedence of a signal over the deadline.
- The leaf names the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- The figures wait for the art stage. Record which figure each chapter needs changed.
- Chapter 9 may need no change. State the check, and the result, in the running log.
