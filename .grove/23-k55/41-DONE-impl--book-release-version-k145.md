# book-release-version-k145

## Goal

Make the release write its own version into the version placeholder of the
book in chapter 3, and check it. This is the owner's decision “Set at release
time” for R17 (2026-10-18).

## Context

- The contract's examples policy, “The reader's Ravel”, and R17 in
  `book-read-one-k70`'s log. Draft `a-complete-session-k136` names the
  version in one place as a placeholder. The brief of node
  `a-complete-session-k63` gives its exact form.
- `release.toml` uses the `pre-release-replacements` of cargo-release 1.1.2
  for `CHANGELOG.md`. The same mechanism can write `{{version}}` into
  chapter 3. After a release, the line holds that version. Make the next
  release find the line again, for example with a pattern that matches the
  placeholder or any version.
- `task release:doctor` checks the release prerequisites. Make a release stop
  when the version in the book is not the release version after the
  replacement.
- GitHub and raw-file views show the Markdown as it is. Thus, do not use an
  include that only mdBook understands.
- Chapter 12, in the second half, teaches the release tasks. Write what it
  must say about this mechanism in the book node's brief, under “Handed
  forward to the second half”.

## Done when

- A dry run in an isolated cargo-release fixture shows the placeholder
  replaced by the release version.
- The check refuses a version that does not agree. A control shows it fail.
- `task check`, `task docs:book:build` and `task docs:examples` pass.

## Notes

## Decisions (running log)

- 2026-10-18: The replacement is a second `pre-release-replacements` entry in
  `release.toml`. Its search matches the whole line with the placeholder or any
  `X.Y.Z`, with `exactly = 1`, so the next cut finds the line again. The file
  path is relative to the package directory, as the changelog path is.
  Verified with cargo-release 1.1.2 in isolated fixtures (the tool's own dry
  run shows `+ravel 1.1.0`), not from memory.
- 2026-10-18: The check is `scripts/release-book-version.sh`. `ready` runs in
  `task release:doctor`. `released vX.Y.Z` runs in the release task after the
  cut and before the build, and reads the page at the tag with `git show`. The
  tag is what ships, and the jj working copy may lag the cargo-release commit.
- 2026-10-18: The production `release.toml` now needs the book page, so the
  prepare fixture (`release-prepare.test.sh`) gets a copy of the real page. The
  first full lifecycle run failed there, which showed the dependency.
- 2026-10-18: Chapter 12's handoff is written in `_ravel-book.md` under
  "Handed forward to the second half". Chapter 3 prose is unchanged.
- 2026-10-18: Not changed: the Taskfile comment that cites a RELEASING.md
  heading "If the installed smoke test fails", which does not exist. It
  pre-dates this leaf.
