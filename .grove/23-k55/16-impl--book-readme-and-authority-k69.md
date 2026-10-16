# book-readme-and-authority-k69

## Goal

Reduce the README's user-facing parts to a short entry into the book, and
reduce `docs/authority.md` to what Parts I and II do not absorb, leaving the
repository's front door honest while chapters 7–12 are still to come.

## Context

- `docs/book/contract.md`, "Relation to other documents": the README is reduced
  to a short entry into the book; the authority note is replaced by the book
  once the chapters that absorb it land. Chapter 2 and chapter 5 absorb its
  authority statements and limits. Its test-isolation guards (the forced-empty
  authority names in `.cargo/config.toml`, the `env_clear` fixtures) and its
  legacy `GROVE_*` and `HARNESS_DISPATCH_*` exceptions are contributor and
  provenance material that chapter 11 and `docs/caller-references.md` take
  later, so the file is reduced in place now and removed in the second half.
- The README ships in the release archives (`scripts/release-common.sh`
  lists it and `scripts/release-build.sh` copies it). It must stay a
  standalone file that makes sense inside an archive with no checkout: do not
  make its only content a link into `docs/` that an archive does not carry.
  Say what Ravel is in a few sentences and name the book's location and how to
  read it locally (`task docs:book`), and state that the repository checkout is
  where the book lives.
- Keep, unchanged, the README's "Develop" section, the "Earlier project" pointer
  and the licence line, which chapters 11 and 12 and the retired record absorb
  later. Move nothing out of them here.
- Everything the README says about the pair, the runner, the channel and
  delivery is checked against the book's chapters: if a statement is dropped, a
  chapter has it or the spec does; if it stays, it is still true.

## Done when

- The README's intro and "Use the pair" parts are replaced by a short entry
  (what Ravel is, where the book is and how to build it, the two verbs in one
  line), with no teaching and no duplicate of a chapter.
- `docs/authority.md` states only the test-isolation guards and the
  legacy-name exceptions, with its title and first sentence saying that. Its
  user-facing authority paragraphs are gone, each shown to be in chapter 2 or
  chapter 5 by anchor in this leaf's log. Its path and
  its role in `docs/caller-references.md` are unchanged, and that inventory's
  row for it is edited to match.
- Every inbound link to either file still resolves; the sweep is enumerated
  with a positive and a mutated control as in `book-guide-fold-in-k68`.
- `task check` and `task release:test` pass, and `task smoke` is run or its
  non-run explained, because the README is in the installed archives.
- The node brief's "Handed forward" is updated with what remains for chapters
  11 and 12.

## Notes

- Nothing the README says about unreleased archives, floors or the formula may
  become less accurate: those statements are chapter 12's, not this leaf's.
