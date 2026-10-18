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

## Decisions (running log)

- 2026-10-18: At the start, no test and no book page quoted the old help
  strings (a search of the checkout outside `.grove/` found them only in
  `crates/ravel/src/cli.rs`). The changed text needs no test change.
- 2026-10-18: The `end-session` doc comment is now “Send exactly one opaque
  result to Ravel and ask Ravel to end the enclosing session”. The `after_help`
  is rewritten as five short sentences that keep every fact. The top-level
  `about` and the `start-session` text stay as they are (the leaf notes).
- 2026-10-18: The README is rewritten in STE with the same sections and links.
  Where a sentence had a semicolon or an -ing word, it is split or reworded.
  “Publication” for the release is now “delivery”.
- 2026-10-18: Control for the “publish/publication” search: the same pattern
  finds 4 lines in the README and 2 in `cli.rs` at the parent commit, and none
  after the change.
