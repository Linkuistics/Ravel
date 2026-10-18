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

## Notes
