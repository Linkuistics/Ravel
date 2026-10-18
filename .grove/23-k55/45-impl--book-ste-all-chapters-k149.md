# book-ste-all-chapters-k149

## Goal

Make `task docs:ste`, run with no file arguments, examine every chapter of the
book, so the second half's chapters 7–12 are covered when they exist. Today the
default file list in `scripts/ste.sh` is `docs/book/src/0[1-6]-*.md`, so a
chapter 07 to 12 is outside the plain report. A stage that names its own files
with `-- --strict FILE…` is not affected, but the set proof, the whole-book art
pass and the whole-book proof read the default.

## Context

- The default list is the `files=(…)` line in `scripts/ste.sh`. Its siblings
  already use an open pattern: `scripts/breaks.sh` takes
  `docs/book/src/[0-9][0-9]-*.md`, and the example check finds every `.md` page
  under the book. Confirm both reads, with a positive control on a scratch page
  that is not a chapter 1–6, before this leaf says the other checks need no
  change.
- `docs/book/ste.md` ("The check") says which files the check examines; update
  it with the change. `scripts/ste.test.sh` holds the controls and its mutants
  (`task docs:ste:test`).
- Chapters 7–12 do not exist yet. The pattern must work when the glob matches
  only chapters 1–6, and must still examine `SUMMARY.md`, `CONTEXT.md`,
  `README.md` and the figure layout records as now.
- Chapter 10 has no text until the component work delivers it. A missing
  chapter 10 is not an error.
- The STE rules and the word decisions are not changed here.

## Done when

- `task docs:ste` with no arguments examines every `[0-9][0-9]-*.md` chapter
  under `docs/book/src/`, in addition to its current files. A control shows it:
  a scratch chapter file named `07-….md` (in a copy of the tree or through the
  test script's existing fixture mechanism, never committed under
  `docs/book/src/`) with a planted long sentence makes the default run report
  it, and the same file with the pattern left at `0[1-6]` does not. Watch the
  second half of the control fail on the old pattern before you credit the new
  one.
- `task docs:ste:test` passes, including its mutants, with that control added.
- `docs/book/ste.md` states the new file list.
- The report on the current book is unchanged (chapters 1–6 give the same
  result as before the change, and the strict form still passes on each).
- The leaf records whether `task docs:breaks` and `task docs:examples` need a
  similar change, with the positive control that shows it.

## Notes

- Keep the change to the file list and its tests. The book's tasks stay out of
  `task check`.
- Use the STE writing rules for every sentence you write in `ste.md`.
