# develop-ravel-k194

## Goal

Proof chapter 11 after the correction run. Check the added statement about
which tests need the built worker against the source and the Taskfile, and check
that inventory 5 still holds.

## Context

- Added by `develop-ravel-k192`. Origin: `book-architecture-fold-in-k164`, which
  stays live until this run ends.
- Enumerate `task --list` again. Do not remove the `docs:components` entry from
  the chapter: the fold-in removes that task and edits chapter 11 itself when it
  runs again.

## Done when

- The statement is true on a fresh checkout, or the log says what you could not
  check. The usual gates pass: build, figures, examples, excerpts, strict STE,
  strict prose breaks and `task check`, on subjects with equal digests before
  and after.
- The chapter's handoff list is clear.

## Notes

The fold-in `book-architecture-fold-in-k164` runs again after this leaf. It
needs chapter 11 stable, because it edits the task list and the evidence table.

## Decisions (running log)
- Read the whole chapter against the document contract, shared word decisions
  and glossary. The two added paragraphs agree with the seam table, build
  section and later probe explanation. No prose or figure correction is needed.
- The Taskfile selects Rust tests without building the worker or probes.
  `test:session` selects all of package `ravel`; `test` selects the workspace.
  Both include CLI and hostile tests. The library-only transport and runner
  commands select worker-free seams. `build.rs::main` hashes worker inputs
  without compiling the worker. `worker::locate` returns `worker_missing`
  before policy evaluation when the executable-relative worker is absent.
  The CLI fixture runs Cargo's executable with a cleared environment;
  hostile helpers require the checkout's shipped worker and separate probes.
  This source read supports the fresh-checkout explanation. K192's intentional
  absent-worker trial is corroborating evidence, not a fresh trial by k194.
- Retain `docs:components`: its removal and corresponding chapter edits belong
  to fold-in k164. No ADR decision changes and no handoff entry remains.
- Fresh inventory 5 enumeration from `task --list` finds 57 public tasks.
  Each has one destination in the final chapter table: no omissions, extras
  or duplicates. Removing a name and adding a false name each fail equality.
  Chapter 12 owns the ten release tasks, and its existing teaching remains
  linked by the chapter's destination rows. The guide destinations recorded
  by proof k181 remain valid: README prerequisites/setup/build/typecheck map
  to the first two sections; tests and check/logs to the seam and handoff
  sections; install/smoke to the installation section; the private-channel
  paragraph to the source section; authority guards and legacy exceptions
  to the isolation section and evidence section. No guide is removed here.
- Tier 2 graph verification uses project
  `Users-antony-Development-Ravel.simplify-and-componentize`, coverage generation
  `2026-10-18T13:38:55Z`. Searches have no remaining pages. Exact snippets
  establish worker location, the Cargo build script and hostile worker/probe
  requirements. The inbound trace links location to `choose_prepared`.
  Coverage metadata matches with no recorded gap for the cited code, Taskfile,
  chapter, contract, glossary, word decisions, check script and ADRs.
  `.cargo/config.toml` is excluded and was read directly. Coverage is a
  best-effort signal, not proof of completeness.
- All seven gates passed on macOS 27.0.1 arm64, 19 October 2026:
  book build without warnings; 14 figure comparisons; 14 complete examples
  with zero failures or skips (27 labelled fragments not run); 8 excerpts;
  strict STE with zero problems in 28 files; strict prose breaks with no
  long run in 11 chapters; and `task check`, including all eight preliminary
  stages and 339 passed tests, 46 skipped. The complete check took 113.95
  seconds. This is a local development check, not new platform qualification.
- One writer ran these gates sequentially. All 3,536 tracked files outside
  `.grove/` had equal SHA-256 maps before and after. This includes the code,
  manifests, Taskfile, Cargo controls, scripts, documentation, fixtures,
  evidence notes and receipts. Generated outputs, installed tools,
  dependencies and host state are outside this map. Logs, the exact input
  list, digest maps, inventory and per-command statuses are under
  `/tmp/ravel-k194-checks/`. The SHA-256 of `before.json` is
  `bd3de4e8b0c34b3e1f9f9bcd13af6b5b8f9393b051e398840ac40a3fb0c5cd90`.
- All 46 local link occurrences in the rendered chapter resolve, including
  heading self-links and the new diagnostic link. Missing-file and
  missing-anchor controls both fail. Raw Markdown supplied the whole-document
  read, and built HTML supplied rendered text and link checks. The in-app
  browser was unavailable. Chrome's URL policy rejected the local file URL
  because it permits only HTTP and HTTPS, so no fresh browser visual check
  was obtained. Editor preview, other themes, narrower widths, GitHub and
  public Pages were not inspected. Prior visual observations remain dated
  evidence, not a new view. No reader-comprehension result follows.
- No chapter edit, correction run, ADR change or in-session reviewer was
  needed. The document node's handoff list is clear. Its original Done when
  holds with k181's guide map and this fresh inventory and source check.
  Retiring this leaf closes `develop-ravel-k159` again. Promote the correction
  result to the book brief; k164 can resume its independent fold-in work.
  The book node retains live leaves and does not close. There are no live
  reviews naming this leaf or its document node that need Creator evidence.
