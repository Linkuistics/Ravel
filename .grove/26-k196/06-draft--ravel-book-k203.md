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
