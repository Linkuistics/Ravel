# release-script-chapter-names-k199

## Goal

Make the release scripts and the Taskfile name the renumbered chapters. After
`ravel-book-k197`, “Deliver Ravel” is chapter 11, and its file is
`docs/book/src/11-deliver-ravel.md`.

## Context

- The node brief `ravel-book-without-chapter-ten-k196`.
- At commit `dbec471f`, these places name chapter 12 or its file:
  - `scripts/release-smoke.sh`, line 428
  - `scripts/release-doctor.sh`, line 240
  - `scripts/release-prepare.sh`, lines 52 and 68
  - `scripts/release-book.sh`, lines 309 and 325
  - `Taskfile.yml`, lines 389 and 396
- The draft's wider search also found old chapter ranges in the opening
  comments of `scripts/ste.sh` and `scripts/breaks.sh`. Update these comments.
  Both scripts discover chapters by file name, without a fixed count.
- At that commit, no test asserts these messages. A search of the test
  scripts for “chapter 1[0-2]” found nothing.

## Done when

- No script, Taskfile comment or other code file names a chapter by its old
  number or its old file name. A search with a control that finds a planted
  old name shows this.
- `task release:lifecycle:test` and `task check` pass.

## Notes

- Change code only. The editorial stages own the book.
