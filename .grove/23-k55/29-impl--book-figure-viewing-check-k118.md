# book-figure-viewing-check-k118

## Goal

Look at every migrated figure at chapter width in the real reading environment
and fix what a reader would trip over, before the owner's resumed read.

## Context

- Environments in order: the locally served book (`task docs:book`) in Chrome at
  the light page's ~750px chapter column and at the full-size SVG link; the Navy
  theme (the former art passes used Auto/light and Navy); VS Code's Markdown
  preview if it opens (earlier passes found it unresponsive; record that again
  rather than skipping silently). GitHub and the published site remain
  unavailable for this unpublished branch and are named as unchecked.
- Compare each figure against its former PlantUML version (git history) for lost
  facts; the "What each view means" table in `docs/book/figures.md` is the list
  of what must survive.
- This check supplies visual evidence, not reader comprehension. The owner's
  stand-in read is `book-read-one-k70`, which follows.

## Done when

- All nine figures were opened at chapter width and full size; each page's alt
  text, caption and key still describe the figure. Defects found are fixed in the
  `.typ` source, or recorded for the whole-book art pass `ravel-book-k101` with
  the reason.
- Total committed SVG size is recorded against the plan's budget (below).
- A dated viewing record is added to `docs/book/figures.md` with the same
  limits-stated shape as earlier ones, noting the environments not reached.
- `task docs:figures`, `task docs:book:build`, `task docs:examples` and `task
  check` pass.
