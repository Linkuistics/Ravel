# book-first-half-k132

## Goal

Prove chapters 1–6, the README, `CONTEXT.md`, `SUMMARY.md` and the
`end-session` help text as a set after read 1's corrections, before the
owner's resumed read.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17): findings R3–R9,
  the change to ASD-STE100, “send” for the result and the command-form
  titles. The run: word decisions k123, then draft k124 and copy-edit k125
  (chapters 1–2), copy-edits k126–k129 (chapters 3–6), set art k130 and the
  README and help leaf k131.
- Read all of `docs/book/contract.md`, `docs/book/ste.md`, `CONTEXT.md` and
  the record of proof k100. The STE specification is at the contract's path.
  Keep it and every extract out of the repository.
- Five sessions converted the chapters to STE one at a time. Look for one
  concept in different words in different chapters, and for a shared word
  decision that `ste.md` does not hold.

## Done when

- Every chapter, the README, `CONTEXT.md` and the figure text follow the STE
  writing rules and `ste.md`. `task docs:ste -- --strict` passes on all of
  them, and `task docs:ste:test` passes (its controls fail on a planted
  problem for each rule).
- R3–R9 are corrected as the read 1 log records. R6 and R9 agree with the
  code and the specifications.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in the book, the README, `CONTEXT.md` or the help text. The search has a
  control seen to fail.
- The titles agree in the contract, `SUMMARY.md` and each chapter's H1. All
  local links and anchors resolve, and a broken-anchor control fails.
- The source inventories still agree with the CLI, the SDK and the
  specifications, and the reading routes still work.
- A fresh `task docs:book:build`, `task docs:figures`, `task docs:examples`
  and `task check` pass on unchanged inputs. The served book shows every
  figure.

## Notes

- Cut nothing. The owner's resumed read, `book-read-one-k70`, follows.
- Record what you could not view, as k100 did.
