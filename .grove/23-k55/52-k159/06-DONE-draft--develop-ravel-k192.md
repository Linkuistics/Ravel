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

- The structure brief is `docs/book/contract.md`, named by the document
  node's goal. It names the reader as a “16-year-old software engineer with
  no background in Ravel”. Its ordered section plan assigns chapter 11
  “set up a checkout, build, use the test seams and `task check`, add a test”.
  Its emphasis includes “Validating with fake agents”, and its exclusions
  keep evidence campaigns outside the book. This correction fits the current
  “Choose a test seam” section without changing that plan.
- The task definitions select Rust tests without a worker-build dependency.
  `test:session` selects package `ravel`, including CLI and hostile tests.
  `test:runner` selects `keyed-launch`. The transport-only command is
  `task test:session -- --lib`. Keep this distinction explicit: the whole
  package is not worker-free, nor is every CLI unit test worker-free.
- `crates/ravel/build.rs::main` hashes worker sources but does not compile
  a worker. `worker::locate` resolves `../libexec/ravel/ravel-policy` from
  the executable and returns `worker_missing` if absent. CLI tests run
  Cargo's checkout executable, not an installed launcher. Hostile tests use
  that pair and `support::direct::{shipped_worker,probe_build}`. Missing
  assets fail assertions rather than skip tests. Explain the first-build
  failure and the separate probe prerequisite in the added prose.
- Graph evidence uses Tier 2, project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T13:29:37Z`. Relevant searches and traces are untruncated.
  Coverage reports no recorded issue and matching metadata for the cited
  code, manifests and chapter. `.cargo/config.toml` is excluded from the
  graph and was read directly. Coverage is a best-effort signal only.
- Add prose only. The existing copy-edit and proof siblings discharge the
  successor rule. No figure or table change calls for an art leaf.
- The whole chapter was read. Two paragraphs after the seam table supply
  the missing dependency and first-build explanation. No heading, table,
  fragment, inventory item or other chapter text changes. The final wording
  explicitly applies the probe prerequisite to `test:session` and `test`,
  because both include the hostile-runtime tests.
- A temporary, initially empty Cargo target directory tested the dependency
  without removing this checkout's worker. With
  `CARGO_TARGET_DIR=/tmp/ravel-k192-no-worker.gePmiY`, `task test:session`
  compiled the Rust targets, then failed as expected: 25 of 242 selected
  tests ran, 12 passed, 13 failed and nextest stopped further starts.
  Its report also lists 29 skipped cases. CLI failures include
  `worker_missing` at the temporary target's `libexec/ravel/ravel-policy`.
  The cancellation case instead timed out before its readiness file appeared.
  Task exited 201 after nextest exited 100. This was an intentional absent-
  worker trial, not a product regression or a complete suite result.
- In the same target directory, with no worker or probes added,
  `task test:session -- --lib` passed all 9 tests, with none skipped.
  `task test:runner` passed all 97 selected tests, with 17 skipped fixture
  or instrument cases. This checks the worker-free commands on this host.
  It is not a full fresh-checkout setup or platform qualification.
  Logs are in `/tmp/ravel-k192-checks/` as
  `session-without-worker.log`, `transport-without-worker.log` and
  `runner-without-worker.log`.
- The first validation passed build, all 14 complete examples, all 8 excerpts,
  strict STE (28 files) and strict prose breaks (11 chapters).
  The 3,532 input digests stayed equal before and after that run and the
  temporary-target trials. Inputs were tracked files under `docs`, `crates`
  and `scripts`, plus Taskfile, Cargo manifest/lockfile, glossary, README,
  `.config` and the directly read `.cargo/config.toml`. Digest-list SHA-256:
  `92ca3ec50905f8c43f3aa2caeb49533f0c091731e12848c411034e602011b274`.
  The probe-prerequisite clarification followed this measurement, so the
  document gates run again on the final text.
- Final validation on macOS 27.0.1 arm64, 19 October 2026, passes:
  `task docs:book:build` without warnings, `task docs:examples` with 14
  passed, zero failed or skipped and 27 labelled fragments not run,
  `task docs:excerpts` with 8 passed, strict STE with zero problems in
  28 files, and strict prose breaks with zero long runs in 11 chapters.
  The same 3,532 input digests match before and after the final run.
  Final digest-list SHA-256:
  `8583584858a8f35d8ac3089105e764157d3f2207f1fb6dd8b7ebe027e89146dd`.
- The diff contains only the requested chapter passage and this task log.
  No ADR decision changes, and no new fragment or test inventory is added.
  No in-session reviewer was used. Copy-edit k193 and proof k194 remain
  live under `develop-ravel-k159`, so no node closes and no successor is
  cut. The guide fold-in k164 remains live for its later removal work.
