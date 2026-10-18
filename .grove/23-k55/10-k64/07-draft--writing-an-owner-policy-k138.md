# writing-an-owner-policy-k138

## Goal

Draft chapter 4 again for the owner's read-1 findings R17 (the link to the
installation in chapter 3) and R11 (runs of prose that are longer than the
limit).

## Context

- The owner's words are in `book-read-one-k70`'s log, “Owner findings
  R10–R17”.
- **R17.** Line 11 at `1ef02398` links to
  `#obtain-the-build-used-by-this-chapter` in chapter 3. Draft
  `a-complete-session-k136` replaces that section. Use the new anchor that the
  brief of node `a-complete-session-k63` gives. Where the text says “the
  checkout build”, say that the reader uses the Ravel that they installed.
- **R11.** With code blocks as breaks, chapter 4 has runs of 549 words (lines
  1477–1538, “Route to Claude Code and Codex”), 487 words (lines 1714–1764,
  “Copy a policy from another configuration”) and 466 words (lines 892–942,
  in “Supply policy context”). Measure with the check from
  `book-prose-breaks-check-k133`. Add tables where items compare on the same
  attributes. Write each figure need under `## Handed forward` in this node's
  brief, with its reader question, for set art `book-first-half-k144`.
- Chapter 1's new text gives the reason for a harness (R14) and the loose
  contract (R15). The routes to Claude Code and Codex must agree with it.
  Name no price or plan.
- This is the first draft re-run on this document. Set art and set proof
  serve all six chapters, so this node gets no art or proof leaf.

## Done when

- Every link to the installation in chapter 3 works. A search with a control
  shows it.
- The prose check finds no run that is longer than the limit in chapter 4.
- This node's brief lists each figure need.
- `task docs:book:build` and `task docs:examples` pass.

## Notes
