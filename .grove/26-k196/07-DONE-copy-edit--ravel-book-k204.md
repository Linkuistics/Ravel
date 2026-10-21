# ravel-book-k204


## Goal

Copy-edit the whole book after the corrections in `ravel-book-k203`.



## Context

Read `ravel-book-k202` for the findings and `ravel-book-k203` for their
correction. Chapter 11's component-policy paragraph moves from Pages setup
to the introduction. The contract's recorded Codex version also changes.
The contract, `docs/book/ste.md` and the technical-communication skill govern
the prose. Keep the technical meaning, structure and evidence limits.

## Done when

- The whole book passes the copy-edit charter, including the moved paragraph
  and its transitions. Each later-stage finding has an owner.
- The book build, examples, excerpts, figures, strict STE and strict prose
  breaks pass. The affected prose passes the skill's checker.
- Keep the queued art and proof leaves. Add no duplicate successor.

## Notes

This is a correction run. A second re-run of one stage escalates under the
editorial family rules. The k202 log records the workspace-check scheduling limit.

## Decisions (running log)

- Read the contract's prose rules, `ste.md`, `CONTEXT.md`, the contents,
  the README and all eleven chapters. The moved chapter 11 paragraph and its
  neighbours were read in place.
- The moved paragraph's first sentence now ties to the pair in the opening
  paragraph: “teaches this pair as one delivered tool”. At the old Pages
  position, “the book that a release carries” tied to the website topic.
  At the new position, the paragraph before it names the pair.
- The moved paragraph is now the first use of “runner” in chapter 11. The
  contract links a term at its first use on each page. Thus, “private
  runner” links to chapter 7's runner section.
- One phrase for the CLI's lifetime: “the Ravel CLI, which supervises one
  session and then exits”, as `ravel-book-k200` decided. Chapters 6 and 10
  and the contract omitted “and then exits”. Chapters 7 and 9 used “exits
  after the session” and “exits after one session”. All five now use the
  one phrase.
- Chapter 7 used “consumer” in a table introduction before the paragraph
  that defines it. The introduction now names the Ravel CLI, which that
  paragraph says is the only consumer in Ravel.
- No other copy-edit defect remains. The glossary terms, the signal-name
  rule, the result words and the chapter names agree across the book.
  No figure text changed. No finding belongs to a later stage, so the
  node's Handed forward list stays as it is.
- The technical-communication checker reports no hard finding in the five
  edited chapters, the contract or this leaf. The contract's count stays
  at four pending sections and no hard finding.
- All six required book tasks exit with status 0. They are `docs:book:build`,
  `docs:examples` (14 passed, 0 skipped), `docs:excerpts` (8 passed),
  `docs:figures` (14 match), `docs:ste -- --strict` (0 problems in 28
  files) and `docs:breaks -- --strict` (0 long runs in 11 files).
  SHA-256 digests of the 3,537 tracked and untracked files outside
  `.grove/` agree before and after the checks.
- The built chapter 11 page links to the runner anchor, and chapter 7's page
  has that anchor. A misspelt copy of the anchor finds nothing, so the
  check can fail. This is a local build only. GitHub and GitHub Pages stay
  unchecked.
- Art `ravel-book-k205` and proof `ravel-book-k206` are already queued in
  this run. This stage adds no successor.
