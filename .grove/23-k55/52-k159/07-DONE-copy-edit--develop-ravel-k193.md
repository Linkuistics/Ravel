# develop-ravel-k193

## Goal

Copy-edit chapter 11 after the correction in `develop-ravel-k192`. Check the
added passage against `docs/book/contract.md`, its STE rules and
`docs/book/ste.md`, and check that it agrees with the sentences around it.

## Context

- The correction run is `develop-ravel-k192`, then this leaf, then
  `develop-ravel-k194`. It exists because the guide fold-in
  `book-architecture-fold-in-k164` found a contributor fact with no chapter home.
- Read the whole chapter, as `references/editorial.md` requires. Change only what
  the copy-edit charter owns.

## Done when

- The chapter follows the prose contract, and its technical facts are unchanged
  except for the added passage.
- Strict STE, strict prose breaks and `task docs:book:build` pass.

## Notes

This is a re-run leaf, so it needs no Creator line.

## Decisions (running log)
- The prose contract is section 2 of `docs/book/contract.md`, with the word
  decisions in `docs/book/ste.md` and the terms in `CONTEXT.md`. This session read
  the whole chapter. Only the two paragraphs from k192 changed.
- "command tests" became "command-boundary tests", the name in the seam
  table and in "Build the matched pair". "installed launcher" became
  "installed Ravel", because the chapter uses "launcher role" for a test
  process. "These runtime tests" became "All of these test tasks", because
  the old words could mean the hostile-runtime tests.
- The worker sentence now names `task test`. The same paragraph already said
  that `task test` runs the hostile-runtime tests. `Taskfile.yml` runs
  `cargo nextest run --locked --workspace` for it, which includes package
  `ravel`. This is a consistency repair inside the added passage. No other
  fact changed.
- The passage now links `worker_missing` as a diagnostic code to chapter 6
  (`06-reference.md#diagnostic-codes`). `crates/ravel/src/worker.rs::locate`
  returns it with stage `worker`. Chapter 6 lists it with exit status 5. The
  link lets the reader check the code. The built page holds the anchor.
- "probes" became "probe workers", the words of chapter 12. The fresh-checkout
  sentence is now condition-first.
- Checks on macOS 27.0.1 arm64, 19 October 2026: strict STE on the chapter
  and on all 28 default files, zero problems. Strict prose breaks on the
  chapter, zero long runs. `task docs:book:build` passed without a warning.
  The skill checker found no finding in the changed passage.
- This session used no in-session reviewer. Proof k194 already stands, so this leaf
  cuts nothing. The run omits art, and no table or figure changed. Nothing is
  handed forward.
