# book-contributor-delivery-fold-in-k165

## Goal

Finish the guide migration for chapters 11 and 12. Remove the release guide
`docs/RELEASING.md` and the authority note `docs/authority.md`, once chapters 12
and 11 absorb them, reduce the README to a short entry into the book, and
redirect every live inbound link. This follows the second half's set proof
(`book-second-half-k163`) and `book-architecture-fold-in-k164`, so that
“absorbed” is a statement about proved chapters.

## Context

- Contract: `docs/book/contract.md` ("Relation to other documents"): the book
  replaces the release guide and the authority note. The authority note is
  already reduced to the test-isolation guards and the legacy-name exceptions
  (`book-readme-and-authority-k69`), and “removed when chapter 11 absorbs
  those”. The contract's clause is edited in place then, to state the result.
- Method: the same as `book-guide-fold-in-k68` and `book-architecture-fold-in-k164`
  (sentence-level classification in the log, the search for inbound references
  with controls, correction runs for sentences with no home). Read both logs.
- **Input: the node brief of the book** (“Handed forward to the second half”)
  and the logs of chapters 11 and 12, which list the destination of each
  README “Develop” sentence, the “private session channel” paragraph, the two
  release paragraphs under “Earlier project”, the notices line, each section of
  `docs/RELEASING.md` and each guard of `docs/authority.md`. Verify each from the
  proved chapters. A sentence with no home goes to the owning chapter by a
  correction run, and this leaf stays live and removes nothing until the run is
  done.
- **The README.** It ships in the release archives and is the repository's door
  (`book-readme-k146` wrote the opening). After this leaf it holds what Ravel
  is (as now), the two verbs, where the book is and how to build it
  (`task setup`, `task docs:book`), the link to the book on GitHub Pages once the
  site exists (it does not exist before the next release: say so or omit it, and
  record which), the licence line and the pointer to the retired record. Remove:
  the “Develop” section (chapter 11), the paragraph on the private session
  channel, the two release paragraphs under “Earlier project” (chapter 12), and
  the reference paragraph that names the guides (replace it with a pointer to
  chapters 7–12 by number and title, as links). The notices line (checkout path
  versus installation path) is in chapter 12; verify that it is, then remove it
  from the README or keep one sentence if an archive reader has no book (a
  release archive does not hold the book). Rename the “Earlier project” heading
  so that it describes what is left under it. Keep the README in STE:
  `task docs:ste -- --strict README.md` passes.
- **Live references to the removed files.** Enumerate them with a search that
  you first see find known hits: `scripts/release-prepare.sh` (two messages that
  say to resume using `docs/RELEASING.md`), `scripts/release-smoke.sh` (the Zig
  message), the comment in `Taskfile.yml` near the release task, `README.md`,
  `docs/publication-evidence.md`, `docs/caller-references.md`, and
  `docs/book/contract.md`. A script message that tells a person where the
  recovery steps are must name the chapter 12 section and its path in a
  checkout (`docs/book/src/12-deliver-ravel.md`, with the anchor for “Recover a
  partial release without another bump”). The tests of those scripts may match
  the message text: run `task release:lifecycle:test`, `task release:test` and
  `task release:doctor` after the change, and update the tests with the message.
  Do not run a task that publishes or tags.
- **Frozen receipts** that name the removed files (the `subjects-*.json`,
  `source-paths.txt` and `*-checks.txt` files) are not edited, for the reason
  that `book-architecture-fold-in-k164` records. Confirm that no script or
  task reads those names as live input.
- `docs/caller-references.md` has a row for `docs/authority.md` and this
  inventory. Update the row and the inventory's account of the guards to point to
  chapter 11, and re-run the inventory's instrument with its controls.
- Chapter 10 is not written. Do not write it.

## Done when

- `docs/RELEASING.md` and `docs/authority.md` are removed and no live document,
  script or task points at them. The README is the short entry above. The
  contract's “Relation to other documents” states the finished migration in
  place.
- The log holds the classification of every sentence group, the list of inbound
  references with their classes, and the controls seen to fail.
- Every redirected link resolves, with a broken-anchor control that fails.
  `task docs:book:build`, the strict STE and prose-break checks on the files
  changed, `task docs:figures`, `task docs:examples`, `task docs:excerpts`,
  `task release:test`, `task release:lifecycle:test`, `task release:doctor` (it
  may report what the host lacks: record exactly what) and `task check` pass on
  subjects whose digests are the same before and after.
- The log says which statements of the removed files survive, where, and which
  were deleted with the reason.
- No guide of the contract's list remains except the ones the contract keeps:
  the normative specifications, the decision records and the evidence
  documents.

## Notes

- Removal is the point. Do not rewrite chapters 11 or 12 here, except for a link
  that the move makes wrong.
- The README is checked in a release archive by name, not by content. Confirm
  this in `scripts/release-archives.test.sh` and the manifest code before you
  change the README, and record the line that shows it.

## Decisions (running log)

- Method of `book-guide-fold-in-k68` and `book-architecture-fold-in-k164` followed:
  sentence-group classification against the proved chapters (read again in full,
  not taken from the units' lists), enumerate-then-classify of inbound references,
  controls seen to fail, gates on frozen digests.
- Every sentence group of `docs/RELEASING.md`, `docs/authority.md` and the README
  "Develop", "private session channel", "Earlier project" release paragraphs and
  reference paragraph has a home or a reason (tables below). No group lacks a home,
  so no correction run is cut and both files are removed in this run.
- Site link in the README: **omitted**. The README ships in the release archives,
  and the first book site appears only with the release that carries this README
  (and after an administrator enables Pages). Either "the site does not exist yet" or
  a link would be wrong in that archive. Chapter 12 "Keep the book with its release"
  already states the address and its unconfirmed status.
- Notices line: kept as one sentence in the README. A release archive carries the
  README and the notices but not the book; chapter 12 "Check the archive contents"
  also states both paths.
- README heading "Earlier project" renamed "Retired project" (no inbound anchor: the
  enumeration found none).
- The README is checked in the release archive by name only: `scripts/release-common.sh`
  line 37 (`README.md` in `archive_manifest`) and `scripts/release-build.sh` line 81
  (`cp ... "$REPO_ROOT/README.md"`). `rg -i readme` over `scripts/`,
  `crates/ravel/scripts/`, `crates/*/tests/`, `Taskfile.yml` and the Cargo manifests
  finds no other reader apart from `scripts/ste.sh` (STE set) and its test. In
  particular `scripts/release-archives.test.sh` holds no README content check.
- Script messages now name the chapter 12 section and its path in a checkout, with
  the anchor: `release-prepare.sh` (two messages: "Recover a partial release without
  another bump"), `release-smoke.sh` (Zig message: "Build archives before a release"),
  `release-doctor.sh` and `release-book.sh` (three messages: "Set up Pages for the
  first release"), and two `Taskfile.yml` comments in the `release` task. The
  scripts' tests match `'first release needs the one-time setup'`, `'No changes since v0.1.0'`
  and `'did not serve'`; all three phrases are kept, so no test needed an update.
- `docs/book/contract.md` "Relation to other documents" edited in place: the release
  guide and the authority note are removed, with where each went.
- `docs/book/figures.md` line 437 (a historical note on a scratch figure drawn from
  the release guide): kept as history, with "now folded into chapter 12" (the k68 method).

## Classification

Short forms: ch11 = `11-develop-ravel.md`, ch12 = `12-deliver-ravel.md`, ch7 = `07-the-parts-of-ravel.md`,
CE = `docs/component-evidence.md`. Dispositions: **A** a chapter teaches it, **D** the
statement survives in source, a script, `release.toml` or a surviving document, **X** deleted
with the reason given. Anchors are heading slugs.

### `docs/RELEASING.md` (367 lines, parent revision)

| Lines | Group | Disp. | Home or reason |
|---|---|---|---|
| 3–7 | One version and source identity, independent of any caller; "development is 0.0.0, first minor 0.1.0 planned"; recheck remote tags; retired daemon bookmark and `structured-plan-migration` tag are not baselines | A, X | ch12 "Keep the version and pair together" (independent version, identity); "Prepare a version cut" (absent-tag case; `structured-plan-migration` is not a baseline). "0.0.0 / planned 0.1.0" is X: stale, the tree holds `v0.1.0` and `v1.0.0`. The daemon bookmark is X here: `docs/retired-project.md` keeps it |
| 9–22 | Prerequisites and `rustup target add` recipe | A | ch12 "Build archives before a release" (prerequisites, the recipe fragment) |
| 24–29 | `release:smoke` and `release:archives` clear `target/archives`; final filename after manifest check; rerun a failed build | A | ch12 "Build archives before a release" |
| 31–36 | Bun runtime pins, source findings, upgrade rules; no host Bun or Node | A, D | ch12 "Build archives before a release", "Keep the version and pair together"; the pins are in `docs/runtime-evidence.md` (linked from ch11 "Read the evidence for a change") and `crates/ravel/scripts/dispatch.sh` |
| 38–48 | Docker linux/arm64, kernel 6.7, CentOS 7.9 userlands, QEMU, Ubuntu helper, glibc 2.17, binfmt_misc, CPU floors, SIGILL, `-cpu max`, vDSO, address bounds, no network, read-only inputs, no kernel-floor measurement | A, D | ch12 "Build archives before a release" (Docker, kernel 6.7, `binfmt_misc`), "Read the platform checks correctly" (every other item). The Ubuntu helper image and its digest: `scripts/release-smoke.sh` lines 106–108 (D) |
| 50–56 | Own tar, version check, installed-layout seam, direct and symlink, private `PATH` without Bun, Node, Grove, the seam's checks, no skip path | A | ch12 "Test the installed archive" (including the stale-context row: `installed-smoke.sh` line 79 is the "Missing session-result context" row) |
| 58–65 | Archive layout, manifest in `release-common.sh`, forbidden content | A | ch12 "Check the archive contents" |
| 67–73 | `release:test` controls; hostile suite probes; `task probes` | A | ch12 "Check the archive contents" |
| 75–91 | `release:formula`: three archives, digests, placeholders, Homebrew Ruby library, real install and test methods, temporary prefix, no keg, Homebrew prerequisite, versus `brew test ravel`, no model, no host runtime, no Grove dependency | A | ch12 "Prepare and test the Homebrew formula" |
| 93–97 | Four native probe workers substituted; `worker_identity_mismatch`; restored | A | ch12 "Prepare and test the Homebrew formula" (formula check table and the paragraph after it) |
| 99–104 | `target/archives/ravel.rb` is unpublished; do not copy to a tap; `ravel-lite.rb` intact; lifecycle tasks own cuts | A | ch12 "Prepare and test the Homebrew formula" (last paragraphs), "Cut and deliver a release" |
| 106–114 | Evidence: saved logs, what a handoff names, archives are local artifacts, rebuild after changes | A | ch12 "Cut and deliver a release" (delivery record), "Read the platform checks correctly" (rebuild after a change) |
| 116–122 | The three cut tasks; first cut 0.0.0 → 0.1.0; absent `v0.0.0`; recheck tags | A, X | ch12 "Prepare a version cut". "First cut is 0.0.0 → 0.1.0 with `release:minor`" is X: stale, done |
| 124–129 | Extra prerequisites; `RAVEL_TAP_DIR`; Homebrew Bash first; `task setup` | A | ch12 "Prepare a version cut" |
| 131–137 | Default colocated jj workspace; saved change ID; what preparation refuses | A | ch12 "Prepare a version cut" |
| 139–145 | The command block, ending with `task release:minor` | A | ch12 "Prepare a version cut" (fragment; the cut task is left out on purpose, with its public effects stated) |
| 147–154 | Lock, doctor, preflight, Pages remote check, preparation, second tap check, `task check`, preview, cut; cargo-release as the Git exception; only `ravel` owns the changelog replacement; replacement controls | A, D | ch12 "Cut and deliver a release" (step table, the `release.toml` paragraphs). The isolated tests of the replacement: `scripts/release-book-version.test.sh`, `release-lifecycle.test.sh` (D) |
| 156–166 | The book's version line; `release.toml` replacement; doctor refusal; `released` guard; the test with real cargo-release | A, D | ch12 "Keep the book with its release"; the test is `scripts/release-book-version.test.sh` (D) |
| 168–177 | Order: book from the tag, archives, formula, smoke before any push, GitHub release, book, tap | A | ch12 "Cut and deliver a release" |
| 179–184 | Install or upgrade, version of the Homebrew binary, assets; Grove and legacy installs untouched; `live` last | A | ch12 "Cut and deliver a release" (last paragraphs) |
| 186–224 | Pages: address, `gh-pages` layout, no workflow, the four `release-book.sh` modes, stage and identity file, no-force push, `check-remote`, `check-pages`, doctor policy | A | ch12 "Keep the book with its release" and "Build and deliver the tagged book" |
| 226–256 | First-release setup; the `gh api` call; Pages limits; about 7 MB per release; three open questions | A | ch12 "Set up Pages for the first release" and the paragraphs that follow it |
| 258–278 | Notes: preservation, fill from commits, `release:notes`, `--from-file`, refusals, commit through jj; "no Grove, session, model or credential launch"; missing `CHANGELOG.md` is an error | A, D | ch12 "Prepare a version cut". The missing-file error is `scripts/release-notes.sh` line 40 (`jj file show ... CHANGELOG.md` fails, D) |
| 280–288 | Never rerun a cut task; what to inspect; lock; `rmdir .jj/release-lock` | A | ch12 "Recover a partial release without another bump" |
| 290–292 | Recovery 1 | A | recovery table row 1 |
| 293–304 | Recovery 2: finish the current version; `cargo release replace` once; reconcile manifests and lockfile; `jj describe`, bookmark, `jj new`; missing tag only; never overwrite a tag | A, D | recovery table row 2 and the paragraphs after it. The commit message `chore: release v<VERSION>` and the tag message `Release v<VERSION>` are in `release.toml` lines 7–8 (D); no script reads either |
| 305–322 | Recovery 3: `jj new`, move `main`, `release-build.sh` and `release-smoke.sh`, book rebuild from nothing, own described commit, no moved public tag, nothing pushed before smoke passes | A | recovery table row 3; fragment with the four commands; "If the release source itself needs a correction" |
| 323–325 | Recovery 4 | A | recovery table row 4 and the paragraph after |
| 326–342 | Recovery 5: GitHub release, upload, draft, `--book-only`, `--tap-only`, byte comparison, `verify`, atomic push, same bytes add no commit | A | recovery table row 5 and the following paragraphs |
| 343–349 | Recovery 6: tap | A | recovery table row 6; tap paragraphs |
| 350–354 | Recovery 7: installation | A | recovery table row 7 |
| 355–360 | Recovery 8: Pages | A | recovery table row 8; last paragraphs |
| 362–367 | Lifecycle tests use isolated repositories; need `task setup`; "actual public recovery depends on the state that k11 observes" | A, X | ch12 "Prepare a version cut" (setup) and the last paragraph of the recovery section. The k11 sentence is X: a task identifier from an earlier workstream, no reader can follow it |

### `docs/authority.md` (23 lines)

| Lines | Group | Disp. | Home or reason |
|---|---|---|---|
| 1–7 | Title; what the note records; links to chapters 2 and 5 for authority boundaries | A, X | Chapters 2 and 5 hold the boundaries (already absorbed by `book-readme-and-authority-k69`). The sentence about the note itself and its links are X: they describe a removed file |
| 9–11 | Tests must never act as the enclosing development session; Cargo forced-empty controls for old and current names | A | ch11 "Keep tests separate from live sessions" (opening paragraph, table row "Cargo commands", the name list) |
| 11–13 | Empty values disable authority and do not redirect it | A | ch11 same section, paragraph after the name list |
| 13–15 | Runner helpers scrub the same names when invoked directly | A | ch11 table row "Runner subprocess helpers", and "also protect direct subprocess invocations" |
| 14–16 | Front and hostile fixtures use `env_clear`, private `HOME` and `TMPDIR`, fixture `PATH`; installed smoke uses `env -i` | A | ch11 table rows "CLI and hostile-runtime fixtures", "Installed smoke" |
| 16 | No test inherits a personal owner policy or a real completion path | A | ch11 "The CLI fixtures create temporary policies, so tests do not need the developer's personal owner policy"; "Never supply the enclosing session's real completion path" |
| 18–20 | Legacy `GROVE_*` and `HARNESS_DISPATCH_*` names are intentional extraction safety exceptions, not product dependencies | A | ch11 "The `GROVE_*` and `HARNESS_DISPATCH_*` names deliberately remain in these guards" |
| 20–23 | Caller-specific paths in provenance are historical attribution; Ravel does not load those projects; the inventory classifies active references, dotfiles included | A | ch11 same paragraph (it links `docs/caller-references.md`) |

### README groups removed or changed

| Group | Disp. | Home or reason |
|---|---|---|
| Book paragraph and the route to chapter 1 | kept | unchanged apart from one sentence that names the later parts |
| Reference paragraph naming the chapters (ch7, ch8, ch9, ch11, ch12, the unwritten ch10) | replaced | five links by number and title; the unwritten chapter 10 stays plain text |
| Notices line (checkout versus installation path) | kept, one sentence | ch12 "Check the archive contents" holds it. A release archive has the README but not the book |
| "Develop": tools (Rust 1.85, curl, tar, Task, ShellCheck, nextest) | A | ch11 "Prepare a checkout" (prerequisite table) |
| "Develop": the task list (`setup`, `build`, `typecheck`, `test`, `test:session`, `test:cli`, `probes`, `test:hostile`, `runtime:integrity`, `check`, `smoke`, `install PREFIX=`) | A | ch11 "Prepare a checkout", "Build the matched pair", "Choose a test seam", "Run the checks before handoff", "Install and test the installed pair", "Find a development task" |
| "Develop": what `typecheck` and `test:cli` check; focused checks while editing, full suite at handoff | A | ch11 "Build the matched pair" table; "Choose a test seam" (last sentence) |
| "Develop": `task check` stages, Clippy denial, nextest report, `target/check.*`, lockfile | A | ch11 "Run the checks before handoff" |
| "Develop": sandbox-exec and bubblewrap; Bun 1.4.2 from `task setup`; no host Bun or Node at run time; no model account or other checkout | A | ch11 "Prepare a checkout" table, "Choose a test seam" (last paragraph of the worker discussion), chapter opening |
| Private session channel paragraph | A | ch7 `#the-result-channel-carries-the-value`; ch11 "Find the source for a change" (channel paragraph, subprocess fixtures) |
| "Earlier project": preserved project, retrieval instructions | kept | under the heading "Retired project" |
| "Earlier project": the two release paragraphs | A | ch12 "Build archives before a release", "Check the archive contents", "Test the installed archive", "Prepare and test the Homebrew formula" |
| Licence line | kept | |

## Inbound references

Enumerated with `rg -n --hidden --no-ignore -e 'RELEASING|authority\.md'` over the tracked
tree (`.grove/`, `.jj/`, `.git/`, `target/` excluded), then again with the words
"release guide", "delivery guide", "delivery instructions", "authority note", "releasing guide"
(each hit read and classified). Positive control: before any edit the first expression found
the known hits of the brief (`scripts/release-prepare.sh`, `release-smoke.sh`, `Taskfile.yml`,
`README.md`, `docs/publication-evidence.md`, `docs/caller-references.md`, `docs/book/contract.md`).

| Reference | Class | Action |
|---|---|---|
| `scripts/release-prepare.sh` lines 52, 68 | script message that tells a person where recovery is | name chapter 12 "Recover a partial release without another bump", `docs/book/src/12-deliver-ravel.md`, anchor `#recover-a-partial-release-without-another-bump` |
| `scripts/release-smoke.sh` line 428 (Zig) | script message | → ch12 "Build archives before a release" (`#build-archives-before-a-release`), where the Zig prerequisite stands |
| `scripts/release-doctor.sh` line 240; `scripts/release-book.sh` lines 309, 325 | script messages | → ch12 "Set up Pages for the first release" (`#set-up-pages-for-the-first-release`), path and anchor. The doctor now prints the new text (seen in the `release:doctor` run) |
| `Taskfile.yml` lines 389, 395 | comments of the `release` task | name chapter 12 "Recover a partial release without another bump" and its path. The old comment cited a heading, "If the installed smoke test fails", that did not exist in the guide |
| `README.md` line 104 | live link | the release paragraph is removed (chapter 12 teaches it); the new chapter list links ch12 |
| `docs/specs/session-execution.md` line 50 | live link (normative: target only) | `../RELEASING.md` → `../book/src/12-deliver-ravel.md` |
| `docs/publication-evidence.md` line 100 | live link in an evidence note | → ch12 `#recover-a-partial-release-without-another-bump` |
| `docs/component-evidence.md` line 53 | live link in an evidence note | → ch12 (label "delivery chapter of the book") |
| `docs/release-lifecycle-evidence.md` line 59 | past-tense sentence, no link | "The release guide" → "Chapter 12 of the book" |
| `docs/caller-references.md` line 20 | inventory row | → ch11 "Keep tests separate from live sessions" and this inventory; instrument re-run (below) |
| `docs/book/contract.md` lines 409–410 | live prose | "Relation to other documents" edited in place (result, not plan) |
| `docs/book/figures.md` line 437 | history of a scratch figure | kept, with "now folded into chapter 12" |
| Receipts: 30 files (27 `subjects-*.json` / `*-source-*.json`, `source-paths.txt`, and the checks logs `docs/worker-reap-admission-checks.txt`, `docs/cancellation-failure-transfer-checks.txt`) | **frozen receipt** | not edited |

Why the receipts stay: they record digests and paths of the files as they were when a run
took them; a rewrite would destroy the digest-to-path record (the reason of
`book-architecture-fold-in-k164`). No script, test or task reads those names as input:
the only readers of `subjects-before/after.json` are `scripts/native-campaign.sh` and
`scripts/native-r01.sh`, and these write them into a fresh output directory. The subject list
of `native-campaign.sh` (`subject_paths`, lines 51–64) enumerates `Cargo.toml`, `Cargo.lock`,
`Taskfile.yml`, `.cargo`, `.config`, `scripts`, `crates` and three named files, none of them
a removed file. `source-paths.txt` has no reader in `scripts/`, `crates/`, `.cargo/` or `Taskfile.yml`.

Remaining match after the edits: only `docs/book/figures.md` line 438 (the history sentence),
the frozen receipts, and this log.

## Verification

Subjects were frozen before the gates: every tracked file outside `.grove/` (3,525 files after
the two removals), sha-256 list `7447dfbbac5aebb4…` before and after. The edits were finished before
the first digest. `.grove/` (this log) is the only tree written afterwards.

- **Link sweep** (enumerated, not pattern-listed; written for this run, code fences and spans
  stripped, file and heading anchor resolved): 47 Markdown files, 963 links: 226 to a file,
  569 to a file and anchor, 168 external, 0 broken (excluded: `docs/evidence/`, `retired/`, `.grove/`).
  **Control seen to fail:** a planted `docs/zz-control.md` with `RELEASING.md`, `authority.md#x` and a misspelt
  chapter 12 anchor was reported as 3 broken (exit 1), its three good chapter 12 anchors (the ones the script messages
  use) counted as resolved; the file was removed and the clean reading returned. A script message cites its anchor in
  text, so the anchor was checked by that control, not by the messages.
- **Name search controls:** `RELEASING|authority\.md` finds 31 files; the mutated `RELEASINX|authoritx\.md` finds 0;
  the same real pattern finds the names in the receipts (cross-tree); the wrong flag `rg -E` errors "unknown encoding".
- **Caller-reference instrument, re-run** (`rg --hidden --no-ignore -n -i -e 'grove|harness[-_]dispatch'`
  over the 258 tracked files outside `.grove/`, `retired/`, `docs/evidence/` and the checks logs; the parent
  revision gave 260): 90 lines in 34 files, against 96 lines at the parent. Positive controls: 6 lines of
  `.cargo/config.toml` and the 2 lines of `docs/component-evidence.md`. The mutated `grovv|harness[-_]dispatcz` found 0.
  A clean fixture gives exit 1 and `GROVE_X` exit 0. Cross-tree: the real expression finds Grove references in 99 files
  of the source project's crates, the mutated one 0. The only change in the match set is the loss of six lines:
  two in `docs/authority.md` and four in `docs/RELEASING.md` (the Grove-free `PATH`, "no Grove launch", "does not
  replace Grove", "no Grove dependency"). Each statement survives in chapters 11 and 12 without the word where the word was incidental
  (`PATH` without Grove: ch12 "Test the installed archive"; no Grove dependency: ch12 "Prepare and test the Homebrew
  formula"; Grove and harness-dispatch untouched: ch12 "Cut and deliver a release"; "no Grove launch": ch12 "Prepare a version cut", "runs no harness or model").
- **Gates** (one writer, in order): `task docs:book:build`, `task docs:figures` (14 figures match),
  `task docs:examples` (14 passed, 0 failed), `task docs:excerpts` (8 passed), `task release:test`,
  `task release:lifecycle:test`, `task release:doctor` and `task check` (339 passed, 46 skipped) all exit 0.
  Strict `task docs:ste` (28 files, 0 problems) and strict `task docs:breaks` (11 files, 0 runs
  longer than 450 words) pass; the README and `docs/component-evidence.md` are in those sets or were
  also checked alone. No task that publishes or tags ran.
- **`release:doctor`** reported every prerequisite met on this host (bash 5.3.20, rustc 1.93.1, all three target
  libraries, zig 0.17.0, cargo-zigbuild, authenticated `gh`, Bun 1.4.2, Docker linux/arm64 kernel 7.0.14-linuxkit,
  mdBook 0.5.4, jj, jq, brew, task, curl, tar, xz, shasum, shellcheck, cargo-release 1.1.2). It recorded one note:
  Pages is not enabled yet for `Linkuistics/Ravel`, which is the state before the first book release.
- STE counts of files outside the strict set, parent against now: `docs/book/contract.md` 179 and 179,
  `docs/book/figures.md` 423 and 423 (a first wording added one, fixed), `docs/publication-evidence.md` 32 and 32,
  `docs/release-lifecycle-evidence.md` 42 and 42, `docs/caller-references.md` 22 and 22, `docs/specs/session-execution.md`
  69 and 69. The edits add none.
- The leaf-wide review allowance was not spent. The classification above was made from the chapters as they
  stand, after the set proof (`book-second-half-k163`) and its guide-map audit.

## What survives, and what was deleted

- **Survives:** every teaching statement of `docs/RELEASING.md` is in chapter 12; every statement of `docs/authority.md`
  is in chapter 11 (and chapters 2 and 5 for the authority boundaries); every README "Develop" and release statement
  is in chapters 11 and 12. Script and `release.toml` details that no chapter lists (the Ubuntu helper image, the commit and
  tag message strings, the missing-changelog error, the replacement-control tests) stay in those files, which are their source.
- **Deleted with a reason:** the "0.0.0 now, 0.1.0 planned" and "first cut 0.0.0 → 0.1.0" sentences (stale: the tree holds
  `v0.1.0` and `v1.0.0`); the retired-daemon-bookmark mention (`docs/retired-project.md` keeps it); the "k11 observes" sentence
  (a task identifier no reader can follow); the authority note's own description and links; the README's "Develop" section,
  session-channel paragraph, release paragraph and guide-naming paragraph (all in chapters 7, 11 and 12).
- **Not changed, for the record:** `scripts/release-prepare.sh` line 50 still prints "planned minor is 0.1.0" for a
  first release with no `v0.0.0` baseline. It is the same stale idea as the deleted sentences, in a script path that a
  tagged tree never takes; this leaf's brief covers messages that name the removed files only.
