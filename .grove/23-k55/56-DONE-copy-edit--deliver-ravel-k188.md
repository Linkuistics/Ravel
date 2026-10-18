# deliver-ravel-k188


## Goal

Copy-edit the changes that draft `deliver-ravel-k187` made to chapter 12 and, if
the draft changed it, to chapter 3. Make the sentences, terms and links agree with
the prose contract and the strict STE check. Do not restructure.

## Context

- Read the draft's log in `deliver-ravel-k187` for the exact sections it changed.
  The release-order, Pages and recovery text is new, so check each word that names
  an action once: build, stage, publish, verify, live, resume. One word names one
  action.
- Keep each file name, command, URL and identifier exact. The Pages address must be
  the same in every place.
- Keep the recovery counts and the table of the four book steps consistent after
  your edits.
- Run strict STE and the prose-break check on the touched chapters.

## Done when

- The new text passes the contract's prose rules, strict STE and the prose-break
  check, with the meaning unchanged.
- Any defect that an earlier stage owns is in a re-run leaf or the brief's
  `## Handed forward` list.

## Decisions (running log)

- Baseline on 2026-10-18: strict STE and strict prose breaks pass on chapters
  3 and 12 before any edit. This copy-edit reads for the rules a script cannot
  find, as `docs/book/ste.md` directs.
- Term: `CONTEXT.md` owns **Owner** (the person who controls Ravel's policy and
  settings). The draft used "the owner" for the person who enables Pages. That
  is a different meaning beside a glossary term. Use "a repository
  administrator" for Pages setup, in prose, the dated checks and the limits.
- One word per action, as the brief lists them:
  - the `publish` action is "put the book on the Pages branch" (the
    `ste.md` decision for the book). "Add" and "send" for this action change.
  - `verify` is the mode name only. The site check is "the `live` check".
  - "resume" stays for `--book-only` and `--tap-only`. "Repeat only `live`"
    replaces "needs to run again".
  - "the stage" is named in the `build` row before other rows use it.
  - "Pages setting" replaces "Pages source" for the one setting.
- Spelling: "GitHub release" in lower case, as the earlier chapter text has
  it. The draft's "GitHub Release" occurrences change.
- Checkable claims: "the Pages remote answers" becomes what `check_remote`
  does (Git reads `origin`, which holds the Pages branch). "The local
  rehearsal added about 7 MB" becomes a local `target/book` build of about
  7 MB (measured: `du -sk target/book` gives 7512 KB). "an earlier release
  directory" in the `publish` row becomes the release's own directory, which is
  what the script compares. "half a book" becomes the unchanged remote branch
  that the script reports.
- Reader-facing text names no Grove stage. "The Pages correction" becomes a
  third dated set of checks. "The queued art stage" becomes the current state
  of the release activity. Art k189 updates that cell when it redraws.
- Chapter 3: "select your installed version from the book on GitHub Pages"
  becomes "open the book for your installed version from the release list on
  GitHub Pages", since the root page is the release list. "The release
  address" becomes "the address of each release's book". The version line and
  the install anchor are unchanged. The Pages address is the same in chapters
  3 and 12: root `https://linkuistics.github.io/Ravel/`, release
  `https://linkuistics.github.io/Ravel/vX.Y.Z/`.
- Setup steps: one action per step, so the setup has five steps. The first
  step is a command ("Run the first version cut"). "Step 2" in the following
  paragraph still names the step that needs the branch.
- Paragraph topics: the `check-remote` paragraph no longer carries the doctor's
  limits. Those limits start the preparation paragraph. No section, heading or
  table moved.
- Final checks on 2026-10-18, after all edits: strict STE found 0 problems and
  strict prose breaks found no long run in chapters 3 and 12. The book build
  logged no `ERROR` or `WARN`. Excerpts: 8 passed. Examples: 14 passed, 0
  failed, 0 skipped. The SHA-256 lists of the tracked `docs/book` and `scripts`
  files were identical before and after the runs.
- Art k189 and proof k190 already stand live after this leaf, so the last act
  cuts nothing. The art entry in the node brief's `## Handed forward` list
  now also names the `docs:figures` row of the third dated check set. No
  earlier stage owns a defect found here, so no re-run leaf. This leaf used no
  in-session reviewer: each change is a word or sentence change checked against
  the script source.

## Notes
