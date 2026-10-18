# running-sessions-k141

## Goal

Copy-edit chapter 5 after draft `running-sessions-k140`, under the STE writing
rules and the word decisions in `docs/book/ste.md`.

## Context

- **R12.** Line 189 at `1ef02398` says that values “keep their exact bytes”,
  and line 632 says “delivered the exact sent result”. Use “unchanged”, and
  say that Ravel does not interpret the result (`docs/book/ste.md`, “The
  result”). Apply R13 (“return no result to the caller”) wherever chapter 5
  states that concept.
- The draft's new text for R16, R17 and R11 has had no copy-edit.
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- R12 and R13 apply wherever chapter 5 states those concepts.
- `task docs:ste -- --strict` passes on chapter 5.
- The prose check finds no run that is longer than the limit in chapter 5.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

## Decisions (running log)

- R12: the result sentences say “unchanged” and “does not interpret”
  (lines on `end-session` values, stdout, the output table and status 0).
  The prompt sentence also says “unchanged”, as chapters 1 and 4 do.
  “exact text” for the prompt file stays, because R12 is about the result.
- R13: “return no result to the caller” replaces “with no result”,
  “suppresses the result”, “prevents the delivery”, “stays suppressed”,
  “no result reaches the caller” and “accepts no result” (three status
  rows).
- R16: “races that shutdown” becomes the mechanism. The first result starts
  the shutdown (source: `Escalation::grace` runs from the channel's
  appearance), and the second call and the shutdown are in a race. “normally”
  is removed, because the sentence gives no condition. The first use of
  “harness” links to chapter 2.
- Consistency: “Read” becomes “Read-only” in the grants table, “Fresh”
  becomes “New” for `RAVEL_EXIT_FILE`, and “the obsolete variables listed
  above” becomes “the five variables that Ravel removes” (checked against
  `crates/ravel/src/run.rs`). The text-form sentence now quotes
  `ravel: refused`.
- The two figure alt texts and the layout records keep the old words. They
  belong to set art k144, as chapter 3's precedent shows. The node brief
  names each one.
- No art leaf is cut: set art k144 and set proof k147 serve this document.
