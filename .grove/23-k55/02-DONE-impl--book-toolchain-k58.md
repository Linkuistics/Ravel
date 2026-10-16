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

## Decisions (running log)

- **What was fetched, on 2026-10-16.** mdBook v0.5.4 (release published
  2026-07-06, still the latest release) from
  `https://github.com/rust-lang/mdBook/releases/download/v0.5.4/`. Each digest
  below was computed from the downloaded archive and matched the digest GitHub
  lists for that asset. They are pinned in `scripts/book.sh`:
  - `mdbook-v0.5.4-aarch64-apple-darwin.tar.gz`
    `03e8a6d8b13a2971e0b3280affd03b388373c1485e26f73407c3a76b0b1838df`
  - `mdbook-v0.5.4-aarch64-unknown-linux-musl.tar.gz`
    `753e5c5c363ee8a56972344dcf91466f005a51db84a7aeffe427ae3ef83d6d44`
  - `mdbook-v0.5.4-x86_64-unknown-linux-musl.tar.gz`
    `5222beabd3e37dc5be0d18ff99b79058469354db5c220153a1b92db5ba12be89`

  Both Linux hosts use the static musl build, which needs no particular C
  library; the release also offers an x86_64 glibc build, not used. Only the
  host's Darwin archive was fetched and run here; the two Linux digests are
  read from the downloads and were not executed on a Linux host.
- **The fetch lives in `scripts/book.sh`, not in `dispatch.sh`.**
  `dispatch.sh`'s bytes are an input to the compiled worker's build identity
  (`source_files`), so adding a book tool there would have changed the product's
  identity for a non-product tool. `task setup` runs `scripts/book.sh setup`
  after Bun's setup. Same discipline as `setup_bun` and `pinned_runtime`: the
  digest is checked on a private copy, that copy is what is extracted, a cached
  archive that fails is fetched again once, a download that still differs is
  refused and nothing is installed or cached, and the installed tool's
  `--version` is checked on every use.
- **Refusals seen to fire.** `scripts/book-integrity.test.sh` (`task
  book:integrity`) drives the real `setup_mdbook` offline against private tool
  directories, emulating only `curl`: fresh fetch, verified-cache reuse (no
  fetch), corrupt-cache recovery (one fetch), corrupt download refused, corrupt
  cache plus corrupt download refused, a deliberately wrong pinned digest
  refusing the real archive, and a failed download reported. Three mutations of
  `book.sh` were each seen to fail it: the download digest check removed,
  the cached-archive check removed, and the cache never reused. A real
  `task setup` run twice on this host also reused the archive.
- **Chapter file names for Parts I and II, fixed here** and used by the drafting
  sessions: `01-what-ravel-is-for.md`, `02-actors-and-words.md`,
  `03-a-complete-session.md`, `04-writing-an-owner-policy.md`,
  `05-running-sessions.md`, `06-reference.md`. None was changed. `SUMMARY.md`
  lists all twelve chapters as draft entries (`- [Title]()`) under the four
  part headings, so a drafting session changes its entry to
  `- [Title](NN-name.md)` when it creates the file.
- **`book.toml` against the 0.5.4 schema** (read from the guide at tag v0.5.4,
  not from 0.4 memory): `[book]` title, description, `language = "en-GB"`
  (which sets `<html lang="en-GB">`, seen in the output), `src`; `[build]`
  `build-dir = "../../target/book"` (relative to the book root, under the
  ignored `target/`) and `create-missing = false`; `[output.html]`
  `git-repository-url`. `create-missing = false` matters: mdBook's default
  creates a file named in SUMMARY.md inside the source tree, so a typo would
  have written a stray chapter instead of failing. `authors` is omitted,
  not guessed.
- **mdBook 0.5.4 accepts an all-draft book**, so no one-heading chapter 1 was
  shipped. Consequence for serving: with no real page there is no
  `index.html`, so `/` answers 404 and `/toc.html` and `/print.html` answer 200
  until chapter 1 exists. With one real page, `/` and the page both answer 200
  with the title `What Ravel is for - The Ravel book`, served on
  `127.0.0.1` through `task docs:book PORT=…`.
- **Tasks.** `task docs:book` builds, then serves on 127.0.0.1 (`PORT`, default
  8775, beside 8773 and 8774); `task docs:book:build` builds without serving;
  `task book:integrity` runs the offline instrument above. None is a
  dependency of `task check`, so a contributor without the tool still runs it.
- **The build check had to be made stricter than mdBook's exit status.** Seen
  on 0.5.4 on 2026-10-16:
  - a SUMMARY entry naming a missing file fails (exit 101) because of
    `create-missing = false`;
  - an unterminated link in SUMMARY.md fails (exit 101) with a WARN and an ERROR;
  - a duplicate file in SUMMARY.md fails;
  - a `{{#include}}` of a missing file logs an ERROR and **exits 0**.

  So `book.sh build` captures the log and fails when any line starts with
  `ERROR` or `WARN`. Seen through `task docs:book:build` on the real tree for the
  missing SUMMARY file, the unterminated link and the missing include, each
  restored afterwards.
- **What the build does not check, handed to the proof stages.** A link to a
  missing chapter written inside a page (`[x](02-nope.md)`), a prefix chapter
  after the numbered ones and a mixed `-`/`*` list are all accepted without a
  warning. mdBook has no link checker; the book will link to chapters not yet
  written on purpose (the contract fixes the file names for that), so a hard
  link check would fail the drafting sessions. Each unit's proof and the set
  proof read the links; a non-failing report of unresolved relative links is a
  possible later addition and is not cut as a leaf here.
- **`task check` is not affected.** The one place it touches new files is
  `shellcheck scripts/*.sh`, which runs on `scripts/book.sh` and
  `scripts/book-integrity.test.sh`; the same command over the same globs exits 0
  (it exited 1 on four info-level findings in the first draft of the test, which
  were fixed rather than suppressed wholesale). No file under `crates/`, the
  installed layout, `dispatch.sh` or the release scripts changed. The native
  campaign driver digests `Taskfile.yml` and `scripts/` as subjects of whatever
  run it makes, so a future campaign's digest map will include these files; no
  recorded campaign is altered.
- **No in-session reviewer spent.** The claims that a compiler cannot check are
  the integrity cases, and each was seen to fail under mutation.
