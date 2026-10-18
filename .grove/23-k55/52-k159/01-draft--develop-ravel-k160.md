# develop-ravel-k160

## Goal

Draft **part A** of chapter 11, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/11-develop-ravel.md`, linked from
`docs/book/src/SUMMARY.md`: setting up a checkout, building, the test seams,
`task check`, and adding a test. The headings for part B are not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 7 gives the
  reasons for the parts; this chapter is the contributor's map of the same
  source. Link back instead of re-teaching.
- Start from a clean checkout (a scratch `jj` workspace or a fresh clone of the
  working copy in the scratchpad, never the live tree) and follow your own
  chapter as the reader would: `task setup`, `task build`, `task typecheck`,
  the focused tests, `task check`. Record which of them you ran, on which host,
  with the elapsed time and what they need (disk, network for `task setup`).
  The first run of `task setup` fetches the pinned tools, so say what is
  fetched and what is verified.
- Add a test in each seam on a scratch copy to learn what a contributor sees:
  one runner test with a real process (and the PTY helper where it applies), one
  command-boundary test in `crates/ravel/tests/cli.rs`, and one unit test in
  the Ravel CLI. The fragment in the chapter is what you actually added, with
  the file it came from. Read `task test:list` for the filters.
- Settle the heading structure for the whole of chapter 11 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- The README's “Develop” section is a source, not a place to copy from.
  Its sentences are listed in the node brief. Rewrite them in the book's
  words, and verify each against the Taskfile and the scripts.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and, if the
  chapter holds an excerpt, `task docs:excerpts`.
- The log lists which tasks of `task --list` part A teaches and which remain
  for part B, and the destination of each README “Develop” sentence that part A
  covers.
- Last act: `leaf-add` a `draft` leaf (bare stem `develop-ravel`) whose body
  states part B's scope from the node brief, the headings reserved for it and
  anything part A learned that changes it; then `leaf-add` the `copy-edit` leaf
  if no live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claims of what `task check` runs and writes, because the chapter's reader
  trusts them without running them.
