# reference-chapter-k143

## Goal

Copy-edit chapter 6 after draft `reference-chapter-k142`, under the STE
writing rules and the word decisions in `docs/book/ste.md`.

## Context

- The draft's new text for R16, R17 and R11 has had no copy-edit.
- Apply “unchanged” and “return no result to the caller” (R12 and R13 in
  `docs/book/ste.md`, “The result”) wherever chapter 6 states those concepts.
- Notes in chapter 6 give information, not instructions (Rule 5.5, as set
  proof `book-first-half-k132` found).
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:ste -- --strict` passes on chapter 6.
- The prose check finds no run that is longer than the limit in chapter 6.
- `task docs:book:build` passes.

## Notes

## Decisions (running log)

- Precedent: chapter 5's copy-edit `running-sessions-k141` and the race
  wording of chapter 2. Baseline at `c47582e`: strict STE and prose-break
  checks pass with zero problems on chapter 6.
- R12: the `start-session` entry, status 0 and the stdout row say that Ravel
  writes the sent result “unchanged” and “does not interpret” it, in place
  of “the exact sent result”, “delivered a valid result” and “the exact
  successful result”.
- R13: “return no result to the caller” replaces “prevent a successful
  session result”, “stops the delivery of the result”, “accepts no result”
  (statuses 129, 130 and 143, as in chapter 5), “returns no candidate
  result” (`session_cleanup_failed`) and “Discard the result”
  (`session_cancelled`). Source: `settle` in `crates/ravel/src/run.rs`
  returns the cancellation or cleanup error before the result write.
- R16: “which normally ends the harness” has no condition. As in chapter 5,
  the entry gives the mechanism: Ravel starts to stop the harness when the
  first result arrives, and a second call and that shutdown are in a race.
  The first use of “harness” on the page links to chapter 2.
- “X exits 2”, where X is an input, becomes “X gives status 2”. The input
  does not exit. A command, a verb or `end-session` still “exits”.
- “fresh” becomes “new” for the worker environment and `RAVEL_EXIT_FILE`,
  as chapter 5 has “New”.
- The two table summaries give information. “Do not mix …” and “Find the
  row for an import” were instructions or a signpost. The figure summaries
  belong to set art k144 and stay.
- Installed layout: the link text “Obtain Ravel” becomes “Install Ravel”,
  after the chapter 3 heading. The worker check happens “before it asks the
  worker to evaluate the policy” (source: `verify_hello` precedes the
  `evaluate` frame in `crates/ravel/src/worker.rs`), not “before it sends
  the policy”, because “send” is the result's word.
- The `--exit-dir` sentence that repeats the table's writable grant is
  removed.
- No art leaf is cut: set art k144 and set proof k147 serve this document.

## Validation

- Read the whole chapter against `docs/book/contract.md` section 2,
  `docs/book/ste.md` and the chapter 2 and 5 precedent. Two claims were
  checked in the source before the wording changed: `settle` in
  `crates/ravel/src/run.rs` and the hello-before-`evaluate` order in
  `crates/ravel/src/worker.rs`. No in-session reviewer was used.
- The search for the replaced wording (“exact sent result”, “accepts no
  result”, “candidate result”, “Discard the result”, “normally ends”,
  “fresh”, “Obtain Ravel”, “sends the policy” and the input “exits 2”
  forms) is clean in chapter 6. The same `grep -E` finds them in the parent
  revision `c47582e`.
- The strict STE check failed twice on this session's own edits: a sentence
  of 26 words, then a paragraph of seven sentences. Both were corrected.
  Final `task docs:ste -- --strict docs/book/src/06-reference.md`: zero
  problems. Final `task docs:breaks -- --strict` on the chapter: zero runs
  over 450 words.
- `task docs:book:build` passes without warnings. `task docs:examples`: 13
  passed, zero failed, zero skipped, six fragments not run, zero page
  errors.
- All 50 headings are unchanged (a diff of the heading lines against the
  parent revision is empty). The new harness link targets
  `## The managed process and its result` in chapter 2.
- These are mechanical and source checks. They establish no new-reader
  comprehension and no visual inspection. Set art k144 and set proof k147
  keep those charters.
