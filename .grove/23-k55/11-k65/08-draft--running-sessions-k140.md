# running-sessions-k140

## Goal

Draft chapter 5 again for the owner's read-1 findings R16 (the race), R17
(the link to the installation in chapter 3) and R11 (runs of prose that are
longer than the limit).

## Context

- The owner's words are in `book-read-one-k70`'s log, “Owner findings
  R10–R17”.
- **R16.** Line 192 at `1ef02398`: “A repeated valid call succeeds, but it
  does not replace that result.” Say that a second call is a race. The first
  `end-session` normally ends the harness. Ravel waits 2 seconds, sends TERM,
  waits up to 5 seconds, then sends KILL and reaps the group. A second call
  can arrive only in that time. Use the wording that draft
  `what-ravel-is-for-and-its-actors-k134` wrote in the brief of node
  `what-ravel-is-for-and-its-actors-k62`.
- **R17.** Line 13 links to `#obtain-the-build-used-by-this-chapter`. Use the
  new anchor from the brief of node `a-complete-session-k63`.
- **R11.** With code blocks as breaks, chapter 5 has runs of 529 words (lines
  404–472, “Confine the managed process”) and 499 words (lines 568–628,
  “Interpret the exit status”). Measure with the check from
  `book-prose-breaks-check-k133`. Add tables where items compare on the same
  attributes. Write each figure need under `## Handed forward` in this node's
  brief, with its reader question, for set art `book-first-half-k144`.
- R12 occurs at line 189 (“keep their exact bytes”) and line 632 (“delivered
  the exact sent result”). The copy-edit applies it.
- This is the first draft re-run on this document. Set art and set proof
  serve all six chapters, so this node gets no art or proof leaf.

## Done when

- Chapter 5 states the race in agreement with chapter 2.
- Every link to the installation in chapter 3 works. A search with a control
  shows it.
- The prose check finds no run that is longer than the limit in chapter 5.
- This node's brief lists each figure need.
- `task docs:book:build` and `task docs:examples` pass.

## Notes
