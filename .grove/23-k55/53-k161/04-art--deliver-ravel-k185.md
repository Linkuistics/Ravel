# deliver-ravel-k185

## Goal

Give chapter 12, `docs/book/src/12-deliver-ravel.md`, its figures under
`docs/book/contract.md` (“Figure needs”, “Format and media”) and
`docs/book/figures.md`. Own the figures, their alt text, captions and
full-size links. Do not change the chapter's prose teaching, which copy-edit
`deliver-ravel-k183` completed.

## Context

- Read the node brief's “Handed forward” list first. Both art entries are
  yours. Clear each entry that this leaf closes. Leave the proof entry.
- **Reuse, do not redraw, the installed view.** “Check the archive contents”
  links to chapter 6's installed-layout view (`figures/installed-files.svg`,
  in chapter 6's “Installed layout” entry). The
  contract row “How do the packages, the worker and the SDK fit together …
  in an installation?” names chapter 12. Decide where the reuse sits in that
  section: beside the manifest table, or as the linked view only. The table
  stays, because it states the exact manifest paths.
- **Draw one UML activity figure** for the contract row “How is a release
  built and checked?”. Use the activity module `_activity.typ`
  (`book-figure-activity-k153`) and the shared style. Its question, from the
  node brief: “Which checks precede the first public change, and where does a
  failed release resume?”
- Source of the activity order: the step table in “Cut and deliver a release”
  and `Taskfile.yml`'s internal `release` task. The order is: lock; doctor and
  tap preflight; preparation; tap preflight again and `task check`;
  cargo-release preview, then the cut; the `released` book guard on the tag;
  move `main`; build into `target/dist`; formula test and every-target smoke;
  then the public writes: push `main`, push the tag, GitHub release, tap; then
  `brew update`, install or upgrade, `brew test` and the version check.
- Show the boundary between local work and the first public change (the
  push of `main`) plainly. A failure before it leaves the cut local.
- Show where a failed release resumes. Use the recovery table's seven rows in
  “Recover a partial release without another bump”. A failure after the cut
  keeps that version and tag. No path leads back to a new version cut.
- **Do not depict GitHub Pages deployment as a step.** The release task does
  not build or deploy the book. `book-release-pages-k184` owns that missing
  step and a later editorial correction run. A figure that showed it would
  state an unimplemented behaviour.
- The chapter has two other tables that already break the prose: the
  platform-route table and the limits table in “Read the platform checks
  correctly”. They are the page's own tables and need no figure source.
- All figure text is STE (contract section 2). Use “the Ravel CLI”, not
  “front”. Use “version cut”, and “bump” only where the heading has it. Use
  arm64 and x86-64 in prose, and the target names only as target names.

## Done when

- The activity figure exists as `.typ` source, SVG and `.layout.json`, with
  alt text, a caption and a full-size link in “Cut and deliver a release”.
  Its step order matches the Taskfile, and it shows no Pages step.
- The installed view is reused as decided, with no second source.
- The figure regeneration check, the book build, strict STE (the layout
  records included), strict prose breaks and examples pass. The figures load
  in the locally built book.
- The closed Handed forward entries are cleared. Last act: add the bare-stem
  `deliver-ravel` proof leaf under this node, unless a live later sibling
  already holds it.

## Notes

- The node's two draft logs (k162, k182) and copy-edit k183 hold the source
  checks and the dated check results. Copy-edit made no structural change.
