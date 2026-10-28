# rhai-executable-book-examples-k100


## Goal

Make executable book examples and their freshness checker independent of the TypeScript worker before removing the runtime.



## Context

- `scripts/examples.sh` (`require_fresh_build`), `scripts/examples.test.sh`, and complete examples in book chapters 7–11. Chapter 9 is a known TypeScript-policy example.
- The parent brief preserves the exact example-edit restriction and checker requirements. Book-tool Bun is unrelated and stays.

## Done when

- Every complete example in chapters 7–11 that runs a TypeScript policy runs an equivalent Rhai script with the same behavior, exit code and output. Edit only its policy file and `--config` argument; the later book node owns prose.
- Record each example port for `rhai-book-inside-k35` and `rhai-book-inside-k36`.
- Freshness checks no longer need the worker digest or `libexec/ravel/ravel-policy`; an older-than-Rust-sources binary still refuses. Retain a firing stale-binary control and remove old pair controls.
- Re-anchor any moved excerpt. `task docs:examples`, `task docs:excerpts` and both checkers' tests pass, including the example mutants where affected. `task check` passes after script edits.

## Notes

Both runtimes still exist in this child. Keep changes mechanical, so the final child can delete the runtime without breaking examples.
