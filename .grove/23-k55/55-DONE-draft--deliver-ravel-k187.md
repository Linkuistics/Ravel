# deliver-ravel-k187


## Goal

Correct chapter 12, `docs/book/src/12-deliver-ravel.md`, so that it teaches the
release as `book-release-pages-k184` now implements it: the book is built from
the tag and delivered to GitHub Pages between the GitHub Release and the tap. Own
structure and technical truth. This is the one correction run that k184 owes
before set proof `book-second-half-k163`. Set proof must not absorb missing
drafting.

## Context

- Read first: the implementation record in `book-release-pages-k184` (its
  `## Decisions (running log)`), `docs/RELEASING.md` (sections "Version cuts and
  publication", "The book on GitHub Pages" and "Recover a partial release without
  another bump"), `scripts/release-book.sh`, `scripts/release-publish.sh`,
  `scripts/release-doctor.sh`, `scripts/release-book.test.sh` and the release
  section of `Taskfile.yml`. Check each fact against these files. Do not copy the
  guide.
- What changed, to be checked and not trusted from this list:
  - Release order in `Taskfile.yml`: a check that the Pages remote answers before
    the cut; `release-book.sh build vX.Y.Z` after the version guard and before the
    archive build; the GitHub Release, then the book, then the tap (all inside
    `scripts/release-publish.sh`); `release-book.sh live vX.Y.Z` as the last step.
  - The layout: branch `gh-pages`, one directory per release at
    `https://linkuistics.github.io/Ravel/vX.Y.Z/`, a root index, `.nojekyll`.
    A published release directory is final. Say why a branch and not a workflow.
  - What `build`, `publish`, `verify` and `live` each refuse (a table is in the
    guide). `publish` refuses a tag that is not public and a Release that is not
    complete. `--book-only` is a new resume mode and `--tap-only` verifies the book.
  - `release:doctor` now checks the pinned mdBook and the Pages setting. Not enabled
    is a note. `release:lifecycle:test` now needs `task setup` and covers the book.
    No public task was added: re-enumerate from `task --list`, and keep the ten.
  - Recovery gains the book rows: the build retry in the local-cut row, `--book-only`
    in the GitHub row, and a new last row for a Pages site that does not show the
    book (`live` only). Chapter 12 names the number of recovery branches in prose
    and in the figure caption: update both counts.
- Replace the paragraphs that state the implementation gap ("This checkout does not
  yet implement that step", "No release carries the book yet" as a Pages
  statement). The true part stays: tag `v1.0.0` has no `docs/book`. No release
  carries the book yet, and no site exists.
- Teach "Set up Pages for the first release": the branch comes first, then the
  one-time Pages setting, then `live`. Say that the owner does it. Say what the
  fixtures could not establish (the served address, the create-endpoint call,
  Pages delay, GitHub's rendering). Do not claim a deployed site. Keep source
  facts and measured limits distinct.
- Chapter 3 says "use the book for your installed release on GitHub Pages. Each
  release puts its own book there." If the address now belongs in that sentence,
  change it through chapter 3's owner: edit `03-a-complete-session.md` in this
  session, keep its one version line and its anchor `#install-ravel-and-check-its-version`,
  and say in the log what changed. Chapter 1 line about the unchecked Pages site
  stays true.
- Cover the guide statements new to `docs/RELEASING.md` in the log's destination
  map for fold-in `book-contributor-delivery-fold-in-k165`.
- Update the chapter's "Checks made for this chapter" with dated results. Run the
  book build, strict STE, strict prose breaks, examples and excerpt checks. Do not
  run a version cut, push, tag or Pages deployment.
- Art must change the `release-delivery` figure: leave a handoff in the log naming
  the new steps and the new recovery exit. This is the first correction run of
  chapter 12 against this stage. A second re-run of one stage against this
  document is an escalation, not a third leaf.

## Done when

- The chapter states the delivered Pages mechanism and its first-release setup, and
  agrees with the scripts, the Taskfile and `docs/RELEASING.md`.
- No sentence says the Pages step is absent. No sentence says a site is deployed.
- The recovery counts, task inventory and chapter 3's statement agree.
- The chapter passes the book checks, and the log carries the figure handoff.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`, named by the book
  node. It identifies “a 16-year-old software engineer with no background in
  Ravel”, gives the ordered chapter purposes, and states “Build, test and
  deliver the project” as an outcome. Its Emphasis and Exclusions sections
  require stated limits and keep evidence campaigns outside the book. This
  correction retains chapter 12's settled order and adds first-release setup
  within its book-delivery teaching, before recovery.
- The source read distinguishes three checks: the doctor reads the Pages
  setting; the release task separately checks remote reachability; `live`
  reads the served pages last. The guide says the doctor runs both preflights,
  but `release-doctor.sh` calls only `check-pages`. Teach the implemented
  split. Likewise, the export check compares file counts, not every file's
  bytes. Teach the tag export and its file-count check precisely.
- Retain all ten public release tasks from a fresh `task --list` inventory.
  No new public task belongs to this correction. The release uses script
  modes for book delivery and recovery. Chapter 3 gains the release-list
  address and the pre-first-release limit, while its version line and install
  anchor remain unchanged.
- The four correction stages already stand at this level, in order, with
  copy-edit k188, art k189 and proof k190 after this draft. Cut no duplicate
  successor. The release activity remains art's work; its precise handoff
  goes in this log and the book node's `## Handed forward` list.

- Fresh official GitHub source-settings documentation on 2026-10-18 says
  branch delivery still uses a GitHub-managed workflow. It explicitly names
  `.nojekyll` for prebuilt content. Teach “no custom workflow file”, not “no
  workflow”. The earlier k184 uncertainty about that file is resolved by
  this source read, without a deployment observation. The source-settings,
  Pages limits and create-endpoint pages are linked beside their claims in
  chapter 12. No source choice, pin, architecture or ADR changes.

### Source checks and task inventory

The Tier 2 graph project was ready at generation `2026-10-18T12:23:46Z`.
Coverage reported no recorded gap and matching metadata for the 23 evidence
paths checked. This is a best-effort signal. Direct reads of the scripts,
Taskfile, guide and book contract supplied the actual evidence. Shell and
Markdown are read directly under the non-code/source-verification fallback.

The fresh `task --list` inventory contains these ten public release tasks:

| Task | Destination in chapter 12 |
|---|---|
| `release:archives` | Build archives before a release |
| `release:smoke` | Test the installed archive |
| `release:test` | Check the archive contents |
| `release:formula` | Prepare and test the Homebrew formula |
| `release:doctor` | Prepare a version cut |
| `release:notes` | Prepare a version cut |
| `release:lifecycle:test` | Prepare a version cut; recovery and dated checks |
| `release:patch` | Prepare a version cut; Cut and deliver a release |
| `release:minor` | Prepare a version cut; Cut and deliver a release |
| `release:major` | Prepare a version cut; Cut and deliver a release |

`install` and `smoke` remain chapter 11's tasks, linked in the opening.
The internal `release` task is not a public eleventh task. No inventory item
is missing. The eight recovery rows are before writes, partial local cut,
tagged build/test, source push, GitHub/book, tap, installation, and served Pages.
The prose and activity alternative now name eight. The SVG still has seven
exits and is explicitly handed to art below.

### Added guide-statement destinations for fold-in k165

This extends k182's complete guide/README map. It does not remove a guide.
Each row covers the added or changed statement group in `docs/RELEASING.md`.

| Guide statement group | Destination or disposition |
|---|---|
| Version cuts: Pages remote check; tagged book build; GitHub Release → book → tap; final served-site check | Prepare a version cut; ordered table in Cut and deliver a release |
| Address by installed version; root release list newest first | Build and deliver the tagged book; chapter 3 links to the root list |
| `gh-pages` from `/`; per-version directory; root index and `.nojekyll`; final release directory | Build and deliver the tagged book, opening layout paragraphs |
| Branch rather than a replacement artifact with only the newest book | Build and deliver the tagged book, branch rationale. Qualify the guide's “no workflow”: no custom file, but GitHub still deploys through its own workflow |
| Four mode table and refusals; public tag and complete Release gates | Four-mode table; adjacent complete-asset explanation |
| Tag export and version; stage path and identity; scratch clone; unforced whole push | Build and deliver the tagged book, paragraphs after the mode table. Export checks file count; do not claim a byte comparison that the code does not perform |
| Read-only remote and Pages checks; disabled source note; other source refusal | Prepare a version cut, doctor and separate remote-check paragraphs. The guide says the doctor runs both; source shows the release task owns `check-remote` |
| First branch, owner enables source, then `live`; settings and create endpoint | Set up Pages for the first release, ordered steps and labelled API fragment |
| Site and soft limits; about 7 MB per release; size refusal | Build and deliver the tagged book, limits paragraph. State the script's actual `du` threshold separately from GitHub's 1 GB maximum |
| Public address, create call, Pages delay unestablished | Set up Pages for the first release; dated checks. Add GitHub's view, as the brief requires |
| Local book-build retry and unchanged release identity | Tagged-build recovery row and rebuild fragment; source-correction paragraphs |
| Completed GitHub Release: `--book-only`, then `--tap-only`; download comparison and book verification | GitHub/book recovery row; two-mode resume fragment and mode explanations |
| Stage missing, retry same bytes, refuse different directory, no manual replacement | Recovery paragraphs after the resume fragment |
| Final Pages failure: `verify`, source check/setup, only `live` again | Eighth recovery row and final recovery paragraph |
| Lifecycle needs `task setup`; real pinned build, local remote and loopback server | Prepare a version cut; recovery suite paragraph; dated check results |

### Figure handoff to art k189

Update `docs/book/src/figures/release-delivery.typ`, its generated SVG/layout
record and its `docs/book/figures.md` entry. The ordered table in chapter 12
is the textual source for these changes, checked against Taskfile and scripts:

- Add Pages remote reachability to the pre-cut prerequisites.
- After the tagged-book version guard and `main` move, build the tag's book
  before archives. A failed book build joins the local tagged-build recovery.
- After the GitHub Release, add the Pages branch push and `verify`, before
  the tap. Route book failure to the GitHub/book recovery branch, whose resume
  uses `--book-only`, then `--tap-only`.
- After installation and its checks, add `live` as the final action. Add the
  eighth grey recovery exit for a site that does not show the book. Its path
  checks branch/source and repeats only `live`, with owner setup if disabled.
- Keep the public-write boundary after every archive smoke route. Retain the
  seven earlier exits and reconcile the caption, alternative, key and figure
  entry with eight recovery branches. The existing SVG and metadata still
  describe the earlier seven-exit path; this draft does not redraw them.

Chapter 3's installation section now points to the versioned-book list,
states the pre-first-release limit, and links to chapter 12's explanation.
It retains one version line and `#install-ravel-and-check-its-version`.
Chapter 1 is untouched. Copy-edit k188 owns the new prose in both chapters.

### Checks and limits

On 2026-10-18 `release:doctor` passed on Darwin arm64 with Rust 1.93.1,
Bun 1.4.2 and mdBook 0.5.4. Pages was disabled and produced the expected note.
`release:lifecycle:test` passed with full log
`target/release-lifecycle.qjn35U`; its book fixtures use no public write.
Fourteen examples passed, none failed or skipped, with 27 labelled fragments
not run. Eight excerpts passed and fourteen figures matched their sources.
The 220 tracked input files enumerated from `docs/book`, `scripts`, `crates`,
Cargo manifests/lockfile, Taskfile, release configuration and `.cargo` had
identical SHA-256 lists before and after these finished runs. The local
subject list and digests are under `target/k187-checks/`.

Initial STE and prose-break checks found only new prose issues: long
paragraphs, a 26-word sentence, the word “warning”, and a 494-word prose run.
The changes split paragraphs, used the exact `WARN` output name, shortened
the sentence and placed the delivery explanation beside the mode table.
Both strict checks then passed. After the dated check-text addition, final
validation passed: the book build logged no error or warning; strict STE
found zero problems in chapters 3 and 12; strict prose breaks found no long
run in either chapter; fourteen examples passed with none failed or skipped;
and eight excerpts passed. The same 220 tracked subjects had identical
SHA-256 lists before and after all final runs completed. The figure inputs
are unchanged from the successful fourteen-figure comparison.

The tagged-tree check confirms that `v1.0.0` contains `Cargo.toml` and no
`docs/book` tree. `release-book-version.sh ready` confirms chapter 3 still
has its one replaceable version line. Its install heading is unchanged.
The document and source reads discharge structure and technical truth for
this correction. The existing copy-edit, art and proof leaves retain their
charters. No second draft correction is cut.

The final act of drafting is discharged by live copy-edit sibling k188.
Retire k187 and seal this focused change. The book node still has live
work, so no parent closes and no cascade or ADR promotion is due.

No project cut, push, tag, tap change, permanent installation or public Pages
deployment ran. No archive/formula test was repeated. The public address,
owner's create-endpoint call, Pages delay and GitHub rendering remain
unobserved. This draft claims no public book, new runtime/platform result or
reader-comprehension result. The source corrections require no new
architectural decision. No in-session reviewer was needed or dispatched.

## Notes
