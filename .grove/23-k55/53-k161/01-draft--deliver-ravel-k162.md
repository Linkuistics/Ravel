# deliver-ravel-k162

## Goal

Draft **part A** of chapter 12, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/12-deliver-ravel.md`, linked from
`docs/book/src/SUMMARY.md`: versions and identity, the archive layout and
manifest, the checks before a cut, the platform floors and the installed-layout
smoke. The headings for part B are not written here.

## Context

- Chapters 1–3, 5, 7 and 11 are proved before this runs. Chapter 11 teaches
  `task install` and `task smoke`; this chapter continues from them to archives
  and floors. Link back instead of re-teaching.
- Read `docs/RELEASING.md` with the node brief's warning: its first paragraph
  is stale about versions and tags. Check every statement about versions,
  baselines and floors against `Cargo.toml`, `git tag`, `CHANGELOG.md`, the
  scripts and `docs/specs/session-execution.md` (“Installed boundary”,
  “Archives and platform floors”).
- Run what the host allows and report the rest. `task release:test` needs no
  Docker. `task release:archives` with `TARGETS=aarch64-apple-darwin` builds one
  archive. State the host, the tools found and the tools missing.
  `task release:smoke` needs Docker with linux/arm64, Zig and cargo-zigbuild; if
  they are missing, say that the chapter describes those floors from the script
  and the evidence documents and that no run was made for it.
- Settle the heading structure for the whole of chapter 12 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and, if the
  chapter holds an excerpt, `task docs:excerpts`.
- The log lists which tasks part A teaches and which remain for part B, and the
  destination of each `docs/RELEASING.md` section that part A covers.
- Last act: `leaf-add` a `draft` leaf (bare stem `deliver-ravel`) whose body
  states part B's scope from the node brief, the headings reserved for it and
  anything part A learned that changes it; then `leaf-add` the `copy-edit` leaf
  if no live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  statements of the platform floors, because a wrong floor misleads a reader who
  installs on an old system.
