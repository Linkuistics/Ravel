# ravel-book-k200

## Goal

Read the whole eleven-chapter book and copy-edit it against its prose
contract. Check sentences, terminology and consistency after the structure
change. Keep the draft's structure and technical meaning.

## Context

- The node brief `ravel-book-without-chapter-ten-k196` names the owner's
  decision and the work's limits. `docs/book/contract.md` is the structure
  brief and prose contract. `docs/book/ste.md` adds the book's word rules.
- The draft `ravel-book-k197` removed the planned component chapter.
  “Develop Ravel” is chapter 10, “Deliver Ravel” is chapter 11, and Part IV
  is “Develop and deliver Ravel”. The chapter files have the new numbers.
- The new passages explain why Ravel supports no separate component.
  They are in the README, the contract, chapters 1, 6, 7 and 9, and the two
  renamed chapters. The runner is private and serves the one-session CLI.
  Its return does not restore process-wide signal state. Another program
  invokes the CLI as a child process.
- The reuse route in chapter 1 answers the reader at that entry. Keep the
  persistent-runner limit and the native x86-64 Linux evidence limit.
- The draft converted affected prose in preparatory commits. The contract's
  untouched sections retain `tc: pending` markers. Several chapter markers
  explain technical word distinctions that the prose checker cannot make.
  Read those reasons before changing a marker or its text.
- `release-script-chapter-names-k199` precedes this leaf. It owns script
  messages and comments, including the checker scripts' old chapter ranges.
- Evidence records keep their historical labels. Their links now reach the
  renamed chapters. Do not convert or modernise the records' prose.
- The draft's required book checks passed. Its link checker rejected a
  planted old path and a missing anchor. Its reference check detected a
  planted stale name. The draft log records the scope and input digests.

## Done when

- The whole book has received the copy-edit. Every change stays within this
  stage's charter. Later-stage findings are in the node's Handed forward list.
- The affected prose passes `ste-lint.py`, with justified departures where
  needed. The repository's strict book STE and prose-break checks pass.
- `task docs:book:build`, `docs:examples`, `docs:excerpts` and `docs:figures`
  pass after any changes.
- The last act adds an art leaf under this node, with the bare stem
  `ravel-book`, unless a later live art leaf already exists.

## Notes

- This is a copy-edit stage, not another structure draft. The remaining
  art and whole-book proof stages are mandatory.
- The k191 log found no chapter 9 defect to hand to proof.
- Local build and link checks establish no visual-rendering result or new
  platform qualification. Retain the book's existing evidence limits.
