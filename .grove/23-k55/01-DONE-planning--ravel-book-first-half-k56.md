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

## Decisions (running log)

- **Scaffold is three impl leaves and one fix, flat under the book node**
  (`book-toolchain-k58`, `book-figures-k59`, `book-examples-k60`,
  `policy-env-help-k61`), each independently verifiable and each leaving the
  product and `task check` unchanged. The source-excerpt check serves chapter 8
  and waits for the second half. The book's tasks stay out of `task check` so a
  contributor without Java, PlantUML, d2 or mdBook can still run it; the owner
  may overrule this default.
- **`--policy-env` is a small fix leaf placed before drafting**, so chapters 5
  and 6 quote a real description and the contract's omission is removed rather
  than hedged around.
- **Units**: chapters 1–2 one unit (short, 2 defines 1's words, shared
  context figure); 3 alone (first complete example and sequence/state
  figures); 4 one document in two draft sessions (anatomy and validation, then
  context/settings/grants/real routes/copying); 5 alone; 6 one document in two
  draft sessions along inventories 1 and 2. The two-session splits are cut
  lazily by part A as its last act, ahead of copy-edit, so the pipeline order is
  preserved by `leaf-add`'s append position.
- **A set proof (`book-first-half-k67`) is cut now** after the last unit: a
  unit's proof reads only its unit, while entry routes, inventories and terms
  span them. Foreseeable and precisely stated, so a leaf rather than horizon.
- **Guide migration is two leaves after the set proof and before read 1**:
  `session-cli.md` and `owner-configuration.md` are removed (fully absorbed by
  chapters 1–6), the owner's handoff record moves to
  `docs/configuration-handoff-2026-10-12.md` outside the book; `authority.md`
  is reduced in place (its test guards belong to chapter 11) and the README
  reduced to a short entry (it ships in the release archives). Reduce-now for
  mixed guides is the contract's own "removed or reduced".
- **Forward links**: page-level to chapter 6, plain text to chapters 7–12 with
  the second half converting them; chapter file names fixed by the toolchain
  leaf. Chapters 7–12 are mdBook draft entries until written.
- **No `review-planning` leaf and no in-session reviewer spent.** The tree was
  checked against this leaf's Done-when and the contract's chapter, inventory
  and checkpoint text directly; a reader of the tree can still escalate.
- Claude Code 2.1.301 and codex-cli 0.162.1 are still the installed versions
  on 2026-10-16; PlantUML 1.2026.8 and d2 v0.9.0 are installed on this host;
  mdBook is not installed (the toolchain leaf fetches it).
