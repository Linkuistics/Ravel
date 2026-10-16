# ravel-book-first-half-k56

## Goal

Plan the first half of the book, Parts I and II (chapters 1–6), into the
smallest independently useful increments, and cut them under this node with
read 1 after their proof. Only the first half is cut here.

## Context

- The book contract `docs/book/contract.md` and this node's brief.
- What the first half absorbs and draws on: `README.md`, `docs/session-cli.md`,
  `docs/owner-configuration.md`, `docs/authority.md`,
  `docs/specs/session-execution.md`, the CLI's help and argument definitions,
  the SDK, static example and sample under `crates/ravel/worker/`, and the
  CLI tests' real-policy/fake-program fixtures in `crates/ravel/tests/cli.rs`.
- `book-structure-k5`'s decision log records the owner's answers behind the
  contract.

## Done when

- The scaffold drafting needs first is planned as its own increment, each task
  verified before anything relies on it: `docs/book/book.toml` and
  `docs/book/src/SUMMARY.md`; mdBook 0.5.4 pinned and fetched by `task setup`;
  the `docs:book` build-and-serve task; the figure regenerate-and-compare task
  with PlantUML 1.2026.8 and d2 0.9.0 pinned; and the complete-example check
  through the real-policy/fake-program seam. The source-excerpt check serves
  the second half and may wait for it.
- The drafting units for chapters 1–6 are chosen and justified by session size
  and by which chapters depend on which. Each unit is a document that takes the
  pipeline draft → copy-edit → art → proof, and its document node brief or
  draft leaf body names `docs/book/contract.md` by path as its contract.
- The guide migration for the first half is planned: which guide content lands
  in which chapter; when the session CLI guide, the owner configuration guide,
  the authority note and the README's user-facing parts are removed or reduced,
  with their inbound links redirected; and where the owner's configuration
  handoff record is kept.
- The empty `--policy-env` description in `ravel start-session --help` is
  placed: a small fix leaf, or handed on with its reason.
- Read 1 is cut as a `requirements` leaf after the first half's proof,
  carrying the contract's read 1 tasks, entry surfaces and viewing environment.

## Notes

- Every leaf for the book goes under this node, never at the grove root, so the
  book runs before `component-work-scope-k57`.
- Chapter 4's routes to Claude Code and Codex need each agent's flags checked
  against its own help at a recorded version and date; Claude Code 2.1.301 and
  codex-cli 0.162.1 were installed on 2026-10-16.
- Parts I and II describe the compatibility-protected CLI, configuration and
  session behaviour, so their content does not wait on any component decision.
