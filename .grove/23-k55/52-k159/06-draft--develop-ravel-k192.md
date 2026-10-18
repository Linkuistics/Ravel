# develop-ravel-k192

## Goal

Add one missing contributor fact to chapter 11, `docs/book/src/11-develop-ravel.md`,
in "Choose a test seam" or the nearest section that fits. State which tests need
the built worker and which do not.

## Context

- Source of the defect: the fold-in `book-architecture-fold-in-k164` classified
  the sentences of `docs/session-channel.md` (lines 64–67). A fresh reviewer found
  that no chapter states them. They are, in the guide's words: the private
  transport and runner tests need no policy worker; the complete package suite
  includes the command tests and needs the built worker; none of these runtime
  tests uses a host Bun or Node, a model account or an installed launcher.
- What chapter 11 says now: the seam table says `cli.rs` runs "with real workers"
  and "After a worker change, run `task build` before command-boundary tests". It
  does not say that `task test:session` (all tests of package `ravel`) needs the
  built worker, or that `task test:runner` (package `keyed-launch`) needs none.
  The chapter's opening says the tests need no model account.
- Verify the fact against the source, not against the guide. Read the `Taskfile.yml`
  tasks `test:session`, `test:cli`, `test:runner` and `test:hostile`, how the
  `ravel` tests locate the worker, and what a missing worker does. Fresh-checkout
  behaviour matters: say what a contributor sees if they run `task test:session`
  before `task build`, and verify it in a scratch copy if the answer is not in
  the source.
- The guides are still in the tree until k164 finishes. Read them at
  `docs/session-channel.md` and `docs/local-runner.md`.
- Add no other change. Do not add a test inventory: the module comments and names
  of the files under `crates/keyed-launch/tests/` hold the coverage list.

## Done when

- The chapter states, in the book's words, which seams need the built worker and
  which do not, and the host-runtime and model-account facts if the source
  supports them. The statement names the task for each seam.
- Strict STE, strict prose breaks, `task docs:book:build`, `task docs:examples`
  and `task docs:excerpts` pass. Any fragment you add is real or labelled.
- Last act: none. The copy-edit and proof leaves already stand beside this leaf
  (a correction run, `references/editorial.md`).

## Notes

The run omits `art`: the change is one prose passage, with no figure and no new
table. If your change adds a table row or a figure, say so in the log, and cut
an `art` leaf before the copy-edit leaf with `leaf-insert`.

## Decisions (running log)
