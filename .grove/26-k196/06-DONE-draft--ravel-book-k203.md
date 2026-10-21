# ravel-book-k203


## Goal

Correct two whole-book proof findings. Move chapter 11's component-policy
paragraph to the introduction and reconcile the recorded Codex version.



## Context

- `ravel-book-k202` read the whole eleven-chapter book. Its log records the
  findings and validation. This is the first correction run under this node.
- Chapter 11's Pages setup section ends with a paragraph about component
  support. It begins “The book that a release carries teaches one delivered
  tool”.
  Move that explanation beside the chapter's opening description of the
  delivered pair. Adjust its transition if needed. Keep its technical meaning.
- `docs/book/contract.md`, “Omissions and unknowns”, records codex-cli
  0.162.1 for the 2026-10-16 agent check. Chapter 4 records 0.163.0.
  The log of `writing-an-owner-policy-k85` explicitly replaces the earlier
  0.162.1 check with 0.163.0. Make the contract describe that final recorded
  check. Do not claim a new check of current agents.
- Preserve the no-component decision, private runner, process-wide signal
  limit and CLI child-process interface. Keep all platform and reader limits.
- This ordered correction run continues through `ravel-book-k204`,
  `ravel-book-k205` and `ravel-book-k206`. The run keeps every stage. The changed
  placement affects prose and layout, so both copy-edit and art read it.

## Done when

- The draft corrects both findings and reads the whole book and contract
  against its charter. No unrelated structure change follows.
- The required book build, example, excerpt, figure, strict STE and strict
  prose-break checks pass. Apply the prose-conversion rule before an edit
  where it requires a preparatory change.
- Keep the already queued correction stages. Add no duplicate successor.

## Notes

The k202 check passed with `NEXTEST_TEST_THREADS=4`: 339 tests passed,
46 skipped. Two default-schedule runs timed out in different one-second
worker fixtures. Both cases passed alone. The cause remains unproven.
Product source and test deadlines stayed unchanged.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. It states the new
  developer reader, the eleven-chapter order, reader outcomes, emphasis and
  exclusions. This correction keeps that plan.
- Move the component-support paragraph unchanged after chapter 11's opening
  paragraph. It explains the delivered tool before archive and website tasks.
- Use the final agent check in the k85 log: Claude Code 2.1.301 and
  codex-cli 0.163.0 on 2026-10-16. This corrects recorded provenance and
  claims no new check of current agents.
- The technical-communication checker reports no hard finding in either
  edited document or this leaf. The contract's edited section is already
  converted. No preparatory conversion commit is needed.
- Copy-edit k204, art k205 and proof k206 already form the ordered correction
  run. Keep them and add no successor.
- Read the whole contract, SUMMARY and all eleven chapters against the draft
  charter. The two requested corrections are the only draft changes. The
  book keeps the CLI child-process interface and the private runner's limits.
- Checked the command, settings, SDK, environment and bounds inventories
  against their source. Read the session and result-job specifications and
  the three ADRs. The front's run and settlement paths and the runner entry
  agree with the retained lifecycle teaching. The public task list agrees
  with the contributor and delivery chapters.
- Graph verification used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T14:48:50Z`. Coverage reports no recorded gap for the checked
  paths. Source metadata matches for the Rust and SDK evidence. The book,
  task notes and some supporting documents have changed or lack tracking
  metadata in that generation. Direct source reads supply their evidence.
  This is task-directed verification, not a completeness claim for the graph.
- The source comparison shows that chapter 11 changes only the paragraph's
  position. The built HTML puts it before the first chapter section. The first
  HTML check wrongly used the page's keyboard-help heading. Limiting the check
  to `main` corrects that instrument, and a displaced scratch control fails it.
- The book build passes. All 14 complete examples and 8 excerpts pass.
  All 14 figures match their sources. Strict STE reports no problem in
  28 files, and strict prose breaks reports no long run in the 11 chapters.
- `task release:lifecycle:test` passes. Its complete log is
  `target/release-lifecycle.k5SSuo`. These are isolated fixtures, not a
  release or a check of the live website.
- `NEXTEST_TEST_THREADS=4 task check` passes: formatting, scripts, worker
  build and probes, runtime integrity, worker types, lint, terminal recovery
  and all 339 Rust tests. Nextest skips 46 tests. The check log is
  `target/check.v2CuDS`. This run does not diagnose the earlier scheduling
  failures or add platform qualification.
- Validation used 4,641 frozen inputs: every tracked file outside `.grove`,
  every file under `target/tools` and the worker's `node_modules`, and
  `ste-lint.py`. Before and after manifests in `/tmp/ravel-k203-inputs-*.json`
  match, with SHA-256
  `8d4ed37197c44dd1896de71767e8d271c7f9d8edbf4d7570d7de3bd5583bfbff`.
  No measured input changed during validation.
- No new agent, live website, reader-comprehension or visual check took
  place. Copy-edit k204 and art k205 retain their whole-book charters.
  No new finding needs a handoff. The parent stays open for the queued run.
