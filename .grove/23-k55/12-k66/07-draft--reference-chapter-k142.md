# reference-chapter-k142

## Goal

Draft chapter 6 again for the owner's read-1 findings R16 (the race), R17
(the installation in chapter 3) and R11 (runs of prose that are longer than
the limit).

## Context

- The owner's words are in `book-read-one-k70`'s log, “Owner findings
  R10–R17”.
- **R16.** Line 266 at `1ef02398`: “A repeated valid call succeeds, but it
  does not replace that result.” Use the race wording that draft
  `what-ravel-is-for-and-its-actors-k134` wrote in the brief of node
  `what-ravel-is-for-and-its-actors-k62`, in the form of a reference entry.
- **R17.** Line 12 (“The entries describe the checkout build that this book
  uses”), line 960 and the installed-layout table link to
  `#obtain-the-build-used-by-this-chapter`. The entries now describe the
  version that the book names, installed with Homebrew. Use the new anchor
  from the brief of node `a-complete-session-k63`.
- The installed layout keeps its paths below `PREFIX` in a Homebrew
  installation. Check this against the formula (`bin.install "bin/ravel"` and
  `libexec.install "libexec/ravel"`). `task install PREFIX=…` is a
  contributor task. Keep it only where an entry needs it.
- **R11.** The command-line entries are a run of 2,200 words (lines 1–295).
  Measure with the check from `book-prose-breaks-check-k133`. A table of the
  options of each verb can carry the entries. Each entry stays a direct
  entry: it has the local context that it needs, and it links back to where
  the book teaches its subject.
- This is the first draft re-run on this document. Draft k94 was the second
  part of the first draft. Set art and set proof serve all six chapters, so
  this node gets no art or proof leaf.

## Done when

- Chapter 6 states the race in agreement with chapter 2.
- Every link to the installation in chapter 3 works. A search with a control
  shows it.
- The prose check finds no run that is longer than the limit in chapter 6.
- Every CLI input and installed path of inventories 1 and 2 still has its
  entry.
- `task docs:book:build` and `task docs:examples` pass.

## Notes
