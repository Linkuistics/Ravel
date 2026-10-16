# book-toolchain-k58

## Goal

Give the book a skeleton that builds and serves locally, with mdBook 0.5.4
pinned and fetched by `task setup` the way Bun is, so every later drafting
session starts from a book that already builds. The product is unchanged and
`task check` is unaffected.

## Context

- Contract, "Format and media": `docs/book/contract.md`. Markdown chapters under
  `docs/book/src/`, ordered by `docs/book/src/SUMMARY.md`, with
  `docs/book/book.toml`; links between chapters are relative `.md` links that
  work in mdBook and on GitHub; a `docs:book` task builds and serves locally.
- The pin pattern to follow is `setup_bun` in `crates/ravel/scripts/dispatch.sh`:
  a per-platform archive fetched into `target/tools`, its SHA-256 pinned in the
  script and checked before use, the digest recorded beside the version, a
  corrupt download refused. `scripts/runtime-integrity.test.sh` shows how that
  pattern is controlled offline. The development targets are darwin arm64,
  linux arm64 and linux x86_64.
- `Taskfile.yml` already has `docs:components` and `docs:result-job` serving
  editable views; `docs:book` sits beside them. `task setup` is the entry for
  fetched tools.
- mdBook 0.5.4 was current on 2026-10-16. Recheck that the release and its
  per-platform archives exist, read each archive's digest from the download,
  and record the date.

## Done when

- `task setup` fetches and verifies mdBook 0.5.4 for the host platform, beside
  Bun, from a digest pinned in the repository; a second run reuses the verified
  archive; a corrupt or wrong-digest download is refused. Each refusal is seen
  to fire against a deliberately wrong digest, not assumed from the code.
- `docs/book/book.toml` and `docs/book/src/SUMMARY.md` exist. SUMMARY lists all
  twelve chapters in the contract's order and four parts. Chapters not yet
  drafted are mdBook draft entries (a title with no file and no link), so no
  placeholder prose is ever built. The file names Parts I and II will use are
  fixed here and recorded in this leaf's log, because drafting sessions link
  to them before the target exists: `01-what-ravel-is-for.md`,
  `02-actors-and-words.md`, `03-a-complete-session.md`,
  `04-writing-an-owner-policy.md`, `05-running-sessions.md`, `06-reference.md`.
  Change a name only with the reason recorded.
- `task docs:book` builds then serves the book on 127.0.0.1 with a PORT
  variable like its neighbours, and a non-serving build task exists for checks.
  The build output goes under `target/`, not the tree. Verified by building
  both with every chapter a draft entry (if mdBook accepts that) and with one
  real page, and by fetching the served page. If mdBook rejects an all-draft
  book, ship chapter 1 as a one-heading file and say so here.
- A deliberate break (a link to a missing chapter file, a malformed SUMMARY)
  is seen to fail or warn the build, so the build check is not a vacuous pass.
- `task check`, `task smoke` and the release manifest are untouched: nothing
  here enters `crates/`, the installed layout or the archives.
- A decision log records what was fetched, its digests and the date.

## Notes

- Keep the book toolchain out of `task check`. A contributor without it must
  still be able to run `task check`; the book's own tasks are separate.
- Output path, title and `language = "en-GB"` are this leaf's to set against
  mdBook 0.5.4's actual configuration schema; read the 0.5.4 documentation, not
  memory of 0.4.
- One in-session reviewer is allowed by the spine's rule.
