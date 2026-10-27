# rhai-book-use-k82

## Goal

Apply the owner's Read 1 patterns to chapter 5, “Run sessions”.

## Context

- The node brief `rhai-book-use-k75` gives the findings and the facts that need a check against the source.
- Section 2 of `docs/book/contract.md` gives the ten patterns.
- Chapter 1 now explains the routing notice and how `end-session` finds its path (`rhai-book-use-k80`).
  Chapter 5 teaches both in full: notices in “Read notices separately from results”, and nesting in “Keep nested sessions separate”.
  Restate the short explanations there, and keep the terms that chapter 1 uses.

## Done when

- Chapter 5 follows each pattern in section 2 of the contract.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapter 5.
- Each link into chapter 5 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Keep the recorded dependency on the final notice path.
- Do not change chapters 1–4 or 6–11, except for a link that this draft breaks.

## Decisions (running log)
