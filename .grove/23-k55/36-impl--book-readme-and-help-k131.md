# book-readme-and-help-k131

## Goal

Change the `end-session` help text to “send”, in STE. Bring the README into
line with the corrected chapter 1, in STE.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). For R7 the owner
  chose “Replace it, CLI help too”, then the verb “send”. The owner also made
  ASD-STE100 the rule for all documentation.
- The help strings are in `crates/ravel/src/cli.rs`: the `end-session` doc
  comment (“Publish exactly one opaque result to the enclosing session and
  request its end”) and its `after_help` (“… The first publication wins. …”).
  At `5bc411d1`, no test and no book page quoted them. Make sure that this is
  still true before you change them.
- STE: the writing rules and the word decisions in `docs/book/ste.md`. The
  book does not use the STE dictionary (owner, 2026-10-18).
  `task docs:ste -- --strict README.md` examines the README.
- README: its first paragraph follows chapter 1, as `book-readme-k106` did
  after the last correction. The README is documentation, so all of it
  follows STE. Keep its sections and links. The Develop and Earlier project
  sections stay until chapters 11 and 12 take them (node brief, “Handed
  forward to the second half”).

## Done when

- `ravel end-session --help` and `ravel --help` show the new text. It uses
  “send”, follows STE and keeps every fact: one result, the inherited
  `RAVEL_EXIT_FILE`, no session ID or target argument, Ravel keeps the first
  result, missing or stale context exits 1, and no settings or policy are
  read.
- The README follows STE and chapter 1. It has no “publish” for the result.
  A search with a control seen to fail shows this.
- `task check` and `task docs:book:build` pass.

## Notes

- Cut nothing. Set proof k132 already stands.
- The other help text stays as it is. Later work that changes it applies
  STE.
