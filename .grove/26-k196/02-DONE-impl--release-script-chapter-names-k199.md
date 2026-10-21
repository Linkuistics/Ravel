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

## Decisions (running log)

- Changed `chapter 12` to `chapter 11` and `12-deliver-ravel.md` to
  `11-deliver-ravel.md` in the six release scripts and in three comments of
  `Taskfile.yml`. The three section headings that the messages quote exist in
  `docs/book/src/11-deliver-ravel.md`.
- Changed “chapters 7-12” to “chapters 7-11” in the opening comments of
  `scripts/ste.sh` and `scripts/breaks.sh`. Neither script has a fixed chapter
  count, so no behaviour changed.
- A search for the old names outside `docs/` found nothing. The same search
  found a planted old name, so the instrument works. The pattern was: `Chapter 12`,
  `12-deliver`, `11-develop`, `10-component`, a range ending in 12, and
  “chapter twelve”. The first pattern also matched correct new names, so it
  was replaced with this stale-only pattern.
- `docs/release-lifecycle-evidence.md` line 59 still says “Chapter 12 of the
  book”. It is an evidence record under `docs/`. The node brief allows only
  link-target changes in these records, so this leaf leaves it. The copy-edit
  and proof stages should decide whether it needs a change.
- `task release:lifecycle:test` passed. The first `task check` run failed on
  `partial_acquisition_cleanup_worker_timeout_cancel_term_wait`, a test with a
  1000 ms selection bound. The test passed three times alone and the second
  full `task check` run passed (339 tests). This change touches no Rust code, so
  the failure was load-dependent. It is a possible flaky test that no leaf owns.
