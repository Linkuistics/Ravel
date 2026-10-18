# develop-ravel-k194

## Goal

Proof chapter 11 after the correction run. Check the added statement about
which tests need the built worker against the source and the Taskfile, and check
that inventory 5 still holds.

## Context

- Added by `develop-ravel-k192`. Origin: `book-architecture-fold-in-k164`, which
  stays live until this run ends.
- Enumerate `task --list` again. Do not remove the `docs:components` entry from
  the chapter: the fold-in removes that task and edits chapter 11 itself when it
  runs again.

## Done when

- The statement is true on a fresh checkout, or the log says what you could not
  check. The usual gates pass: build, figures, examples, excerpts, strict STE,
  strict prose breaks and `task check`, on subjects with equal digests before
  and after.
- The chapter's handoff list is clear.

## Notes

The fold-in `book-architecture-fold-in-k164` runs again after this leaf. It
needs chapter 11 stable, because it edits the task list and the evidence table.

## Decisions (running log)
