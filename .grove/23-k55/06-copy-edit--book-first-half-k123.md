# book-first-half-k123

## Goal

Make the book's shared STE word decisions before the chapter sessions apply
ASD-STE100. Write `docs/book/ste.md`, convert `CONTEXT.md` to STE, and change
the part and chapter titles in `docs/book/src/SUMMARY.md` to the contract's
new titles.

## Context

- Cause: read 1 (`book-read-one-k70`, its log of 2026-10-17). The owner made
  ASD-STE100 the base of the prose contract and chose “send” to replace
  “publish”, “publication” and “submit” for the result. Read section 2 of
  `docs/book/contract.md` first: base, STE specification, word decisions,
  spelling, technical names and plain vocabulary.
- The STE specification is `~/Downloads/ASD-STE100_ISSUE9.pdf` (SHA-256 in
  the contract). `pdftotext -layout` (Homebrew) extracts its text. Keep the
  PDF and every extract outside the repository, for example in your scratch
  directory. Its copyright notice prohibits unauthorised distribution.
- Your file is the input for these sessions: copy-edit k125 (chapters 1 and
  2, after draft k124), k126 (chapter 3), k127 (chapter 4), k128 (chapter 5)
  and k129 (chapter 6), then set art k130, the README and help leaf k131 and
  set proof k132. If two chapters put one concept into different words, the
  set proof must repair it. Thus, decide each word once, here.
- Words that read 1 already looked up (Issue 9 dictionary):
  - Not approved: submit (use SEND), ask and request (use TELL), end as a
    verb (use STOP), exact (use ACCURATE), check (use MAKE SURE, or the noun
    CHECK), allow (use LET), choose (use SELECT), evaluate (use EXAMINE),
    route as a verb (use PUT), run as a verb (use OPERATE), deliver (use
    SUPPLY), return as a verb (use GO), opaque (NOT TRANSPARENT).
  - Approved: SEND, RECEIVE, KEEP, TELL, STOP, START, ACCEPT, SELECT, CANCEL,
    WAIT, GIVE.
  - Rule 1.12 category 2 (computer processes and applications) gives
    technical verbs such as “validate”, “load”, “install” and “process”. A
    technical verb is correct only when no approved verb gives the same
    information. Decide each candidate with its reason, for example “run” (a
    program), “launch”, “evaluate” (code), “reap”, “exit” and “route”.
  - “Opaque result” is a technical noun (Rule 1.6).
- The book's words are in chapters 1–6, `SUMMARY.md`, `CONTEXT.md`, the
  README and the figure text (`docs/book/src/figures/*.typ` and the
  `.layout.json` records).

## Done when

- A script enumerates every distinct word in those sources, and each word is
  then classified: approved, technical noun, technical verb (with category and
  reason), or not approved (with its replacement). Use the
  enumerate-then-classify method of Grove's `references/execute.md`, not a
  list of known words. The classifier fails on a planted unapproved word.
- `docs/book/ste.md` records the decisions that the chapters need: the
  technical nouns (with `CONTEXT.md` as the term list), the technical verbs
  with category and reason, and the replacement for each unapproved word that
  the book uses. Each decision has a few words. The file does not copy
  dictionary text.
- The result vocabulary is in the file: `end-session` sends the result, and
  Ravel receives and keeps the first result. The book does not use
  “publish”, “publication”, “submit” or “submission” for this concept. The
  “published site” of the book on GitHub Pages is a different concept. Record
  its STE wording too.
- `CONTEXT.md` follows STE and `ste.md`, and each term keeps its meaning.
- `SUMMARY.md` shows the contract's titles: “Use Ravel”, “Write an owner
  policy”, “Run sessions”, “Follow a session through the code”, “Reuse and
  develop Ravel”, “Use Ravel's components in your project”, “Develop Ravel”
  and “Deliver Ravel”. The file names stay. The H1 of chapters 4 and 5
  changes in their own copy-edits.
- `task docs:book:build` passes.

## Notes

- Cut nothing. The chapter copy-edits, set art and set proof already stand.
- If the word work does not fit one session, decompose this leaf, with the
  classification and `ste.md` as its first child.
- The specifications, decision records and other guides are outside this
  leaf. Later work that changes them applies STE.
