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

## Decisions (running log)

- Keep chapter 6's installed view as the linked view only. The manifest table
  already carries the exact paths. A second embedded copy adds no relation.
  This is an editorial choice, not a contract restriction or a medium failure.
- Draw `release-delivery` with the shared activity module and crisp style.
  Use sequential actions, a local/public boundary and seven failure exits that
  name the recovery point. Grey exits stop the automatic task and name manual
  recovery. They do not loop to another version cut. The chapter's step and
  recovery tables remain the full text alternative.
- Keep the other page tables. They carry the version checks, target routes,
  platform limits, task comparison, book guards and observed checks. No extra
  figure for those relations is needed. This is an editorial choice.
- Put the decision explanation before the activity. The first check found
  that the explanation after the image extended the next prose run to 470
  words. Moving this figure text preserves all teaching and lets the image
  break that run. No chapter prose or figure source changes for this fix.
- Name Ravel's version at the Homebrew prefix in the final action. The first
  label, “Check the Homebrew version”, could mean Homebrew's own version.

## Source and figure checks

The whole chapter was read under the art charter. The internal `release` task
in `Taskfile.yml` supplies the action order. `scripts/release-build.sh` calls
the formula check at the end of the tagged build, before Task calls the smoke
script. The figure uses no fork or join, because the commands are sequential.

| Recovery-table row | Figure exit |
|---|---|
| Before any version write | `before-write` |
| During the local cut | `partial-cut` |
| Tagged cut, but build, formula or smoke failed | `local-failure`, also reached for the tagged-book guard |
| Main or tag push failed | `push-failure` |
| GitHub release or asset upload failed | `github-failure` |
| Tap copy, commit or push failed | `tap-failure` |
| Local installation or final check failed | `install-failure` |

The guards summarise each group, rather than defer its failure handling.
The key and caption state that failure stops at its own step. The local
region ends before the first push of `main`. Every post-cut exit retains the
version. The key preserves the separate source/tag decision and the ban on
moving a public tag. No Pages action appears.

Graph coverage was checked at Tier 2, generation 2026-10-18T11:58:46Z for
the final source paths. The Taskfile, chapter, activity module, figure sources,
layout record and release build script had no recorded gap. SVGs are excluded
by suffix. Their direct render, browser view and regeneration comparison
supply the evidence instead. This is a source and visual check, not a new
runtime result.

## Verification

Checks on 2026-10-18:

| Check | Result |
|---|---|
| `task docs:figures` | All 14 SVGs and layout records match their sources |
| `task docs:book:build` | Passed with no error or warning |
| `task docs:ste -- --strict` | Zero problems in 28 files, including every layout record |
| `task docs:breaks -- --strict` | Zero long runs across 11 chapters after the caption move |
| `task docs:examples` | 14 passed, zero failed or skipped, 26 labelled fragments not run, zero page errors |
| `task docs:excerpts` | Eight passed, zero failed or page errors |

The initial prose-break failure and the caption correction are recorded above.
The complete examples passed both before and after that correction. The final
version-label edit changed only the figure source, SVG and record, followed by
fresh figure, build and strict STE passes. No source excerpt or example changed.

Each finished check set had the same 219 source-file SHA-256 digests before
and after it. The subject list covers tracked book files, scripts, crate
sources and fixtures, Cargo manifests/lockfile, Taskfile, release configuration,
glossary and README. Scratch lists and per-task logs are under `target/k185-*`.
No check ran across an edit to those subjects.

Chrome supplied the local light-page view at a 750 px column in a 1227 × 919
viewport. The full activity, key and corrected version label were inspected
at chapter width. Both linked SVGs rendered, with the release activity's upper
region inspected at full size. The final caption placement was rechecked.
`docs/book/figures.md` records the view and size limits. The new figure is
263,202 bytes. The full set is 3,175,724 bytes and its existing size issue
remains with whole-book art k101. No new medium or renderer pin is needed.

Both art handoffs are closed and cleared. The proof handoff about Pages stays.
No technical or prose-teaching correction is handed back. The figure work
changes no ADR. Proof remains due, so this leaf closes no parent node.
