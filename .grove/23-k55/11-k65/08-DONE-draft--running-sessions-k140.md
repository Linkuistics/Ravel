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

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. Its reader is a
  “16-year-old software engineer with no background in Ravel”. Its ordered
  section plan makes chapter 5 teach how to “run sessions under real
  conditions” and “interpret every outcome and exit code”. Its Emphasis
  requires success after cleanup and explicit limits. Its Exclusions keep
  agent installation, evidence campaigns and undelivered contracts outside
  this chapter. This correction retains that order and scope.
- R16 uses the chapter 2 handoff. The example's second call is an attempt
  that races shutdown, not a promised second action. The existing local
  schedule explains the early exit and group cleanup before reap.
- R11 uses two page tables: filesystem grants under confinement, and three
  failure codes compared by stage and meaning. These comparisons replace
  prose and answer reader questions without a new diagram.
- R17 links to chapter 3's Homebrew installation and version check.
  Copy-edit k141 retains R12 and R13. Set art k144 retains the three
  existing figure questions and applies the crisp style.

## Validation

- Read the whole chapter against the structure brief, both specifications
  and the three ADRs. The correction preserves the order, outcomes, limits,
  examples and figure questions. No code block or heading changed.
- Tier 2 graph lookup, outbound traces and exact source checked the changed
  technical claims. `completion.rs::publish_result` retains the first
  complete value. `run.rs::ESCALATION` in Ravel and the runner's `watch`
  establish the 2-second and 5-second waits, early exit, group stop and reap.
  Both confinement modules establish the grants in the new table. Ravel's
  `policy.rs`, `worker.rs` and `run.rs` establish the failure codes and stages.
  `run.rs::settle` retains cancellation and cleanup precedence.
- Graph generation `2026-10-18T06:41:05Z` reported no recorded coverage gap
  for the evidence paths. The changed chapter had stale metadata at the
  first coverage check, so its source and final diff were read directly.
  Some graph call edges resolved to unrelated names. The claims above use
  the exact source, not those edges. Coverage is a best-effort signal.
- Before the edit, the strict prose check found 524-word and 487-word runs.
  After the edit, `task docs:breaks -- --strict
  docs/book/src/05-running-sessions.md` reports no run longer than 450 words.
  The strict STE check also passes. An initial STE check found seven
  sentences in the expanded shutdown paragraph. The instruction now has
  its own paragraph, and the final check passes.
- `task docs:book:build` passes without a warning. `task docs:examples`
  passes all 13 examples, with no failure, skip or page error. Six labelled
  fragments are not run. The six chapter 5 examples each passed.
- All 42 chapter-local destinations resolve in the built book. An exact
  `rg -n -F` search finds the new installation link and no retired link in
  chapter 5. A scratch chapter with the old anchor restored makes the same
  search find it and makes the new-anchor search fail. The old link remains
  detectable in the ancestor's handoff. Chapter 3's rendered page has the
  new anchor and lacks the old one.
- SHA-256 digests before and after these checks match for all 198 subjects:
  files in `docs/book`, `scripts`, `crates` (without `node_modules` or
  `target`), `.cargo`, `.config`, and the repository-root files. The checks
  establish local source, build and fake-program behaviour. They establish
  no real-agent, interactive-terminal, remote-view or reader-comprehension
  result.
- The live later sibling `running-sessions-k141` already owns copy-edit.
  No duplicate leaf is needed. The document node stays open. Its brief
  carries the figure questions for set art k144. No architectural decision
  changed, so the ADR set stays as it is. No in-session reviewer was needed.
