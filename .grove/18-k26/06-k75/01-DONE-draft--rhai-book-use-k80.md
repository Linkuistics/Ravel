# rhai-book-use-k80

## Goal

Apply the owner's Read 1 patterns to Part I, chapters 1–3.
Write the new explanations of chapter 1: Rhai, a selection, the routing notice and the path of `RAVEL_EXIT_FILE`.

## Context

- The node brief `rhai-book-use-k75` gives the six passages F1–F6, the Rhai addresses and the facts to confirm.
- Section 2 of `docs/book/contract.md` gives the ten patterns. The chapter 1 row of the section plan is new.

## Done when

- Chapters 1–3 follow each pattern in section 2 of the contract.
- Chapter 1 introduces Rhai where it first names it, with its characteristics, a short example and links to its documentation.
- Chapter 1 explains a selection, the routing notice and the path of `RAVEL_EXIT_FILE`, each with real values.
- The document node brief has a “Handed forward” list. It gives art the place and the question of each new figure.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–3.
- Each link into chapters 1–3 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Keep the release placeholder in chapter 3.
- Do not change chapters 4–11.

## Decisions (running log)

- **Chapter 1 uses fragments with real values, 2026-10-27.** Chapter 1 comes before the installation in chapter 3, so it has no complete example.
  Each fragment is real output of the checkout build (`ravel 1.1.0`, Rhai path) from a run on 2026-10-27 in a private scratch home.
  The review policy, its routing notice, the `selection` member of its JSON notice and the bare `end-session` failure come from that run.
  The nested run used `TMPDIR=/tmp`, so the paths are short. The fragment label says that the random parts change and that macOS shows `/private/tmp`.
- **Facts confirmed against the source, 2026-10-27.**
  The text notice is one `eprintln!` line before the launch, and `run.rs` keeps the JSON notice until after supervision (`crates/ravel/src/run.rs`).
  The JSON `selection` member has `provider`, `model`, `effort` and `reason`, but not `program` or `args`. The `resolution` member names the found file.
  A selection needs seven fields (`crates/ravel/src/policy.rs`). The channel is `$TMPDIR/ravel-session-*/signal-*` unless `--exit-dir` or confinement chooses the parent (`run.rs`, `completion.rs`, `keyed-launch/src/channel.rs`).
  The engine sets Rhai's operation, call-depth, expression-depth and size limits (`crates/ravel/src/rhai_worker/engine.rs`).
- **Rhai's characteristics come from its feature list, 2026-10-27.** The page gave status 200 and names the syntax like JavaScript and Rust, dynamic types, the sandbox and the protection against runaway scripts.
  The Rhai Book page names version 1.27.0, as `Cargo.lock` does. The chapter does not link to `/book/language/`.
- **Shape of “From a request to a result”, 2026-10-27.** An order table previews six steps. Six subsections follow the table, one for each actor's step, and a summary paragraph ends the section.
  The existing H2 anchors stay, because chapters 2–11 link to them.
- **Chapters 2 and 3 restate chapter 1, 2026-10-27.** Chapter 2 replaces the one-line Rhai description with a short introduction and a link to the Rhai Book.
  Chapter 2 explains a selection and the routing notice at line 55's old place, with the text notice as a fragment. It also shows how `RAVEL_EXIT_FILE` passes from parent to child.
  Chapter 3 explains the routing notice after the first session, adds the `status` and `reason` rows to the selection table and gives one real path.
  Each term in chapters 2 and 3 now has a short explanation at its first use on the page.
- **One claim narrowed, 2026-10-27.** This session did not verify that every harness starts a shell for each command.
  Chapter 1 now says that a harness runs each requested command in a child process, for example a shell. The inheritance needs only that.
- **No in-session reviewer, 2026-10-27.** Each new fact came from the source or a real run, and copy-edit, art and proof follow.
- **Handed forward, 2026-10-27.** The document node brief lists the two new figures for art `rhai-book-use-k77`, with place, question, notation and real values.
- **Checks, 2026-10-27.** Evidence class: mechanical.
  `task docs:examples`: 19 passed, 0 failed, 0 skipped, 33 fragments not run. `task docs:excerpts`: 9 passed. `task docs:figures`: 14 match.
  `task docs:book:build` had no error or warning. `task docs:ste -- --strict` and `task docs:breaks -- --strict` passed for chapters 1–3.
  A script compared 199 links into chapters 1–3, from all chapters and the README, with the ids of the built HTML. None failed. A planted bad anchor failed the same comparison.
- **Lessons, 2026-10-27.** `docs/documentation-lessons.md` has L52 and L53, and the applications of L04, L06 and L07 name chapter 1.
