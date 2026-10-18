# deliver-ravel-k189


## Goal

Change the `release-delivery` activity figure of chapter 12
(`docs/book/src/figures/release-delivery.typ`) so that it shows the delivered
book steps. Keep the figure's conventions and regenerate its SVG and layout record.

## Context

- Read the draft's figure handoff in `deliver-ravel-k187` and the figure's entry in
  `docs/book/figures.md`. The figure has no Pages step today and its caption and
  entry say so.
- The order to show comes from `Taskfile.yml`: the book is built from the tag after
  the version guard and before the archive build; the book goes to Pages after the
  GitHub Release and before the tap; the Pages site is read last. A failed book
  build stops with the cut local. The new recovery exit is a Pages site that does
  not show the book. Check each position against the Taskfile and
  `scripts/release-publish.sh`.
- Keep the public-write boundary clear: the book build is before it, and the Pages
  push is after it. Keep the grey exits as manual recovery points, and change their
  count in the caption and in `figures.md`.
- Use `task docs:figures:update`, then `task docs:figures`, and view the result at
  the 750 px column. Record the file size and the set size. Whole-book art
  `ravel-book-k101` owns the set-size issue.

## Done when

- The figure, its caption in chapter 12, and its `figures.md` entry agree with the
  Taskfile and with each other. The figure check passes.
- The figure is legible at the column width.

## Notes
