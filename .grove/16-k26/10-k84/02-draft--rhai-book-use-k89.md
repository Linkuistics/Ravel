# rhai-book-use-k89

## Goal

Apply all lessons and the term map to chapter 4, “Write a configuration script”.

## Context

- The node brief `rhai-book-use-k84` gives the required corrections and the term map.
- Draft `rhai-book-use-k88` settled the terms and wrote the chapter 1 explanations. Restate them in the same words.
- Chapter 4 explains evaluation, so the Rhai worker explanation (F12) applies in “Follow the evaluation of a policy”.
  Check it against `crates/ravel/src/rhai_worker/front.rs`, `crates/ravel/src/worker.rs` and ADR `policy-evaluation-precedes-the-launch`.
- “Policy” occurs about 300 times in this chapter. Literal names such as `policy.rhai` and the `policy_*` codes stay.
- Check every heading against L61. Several headings name “policy”.

## Done when

- Chapter 4 follows the term map, the contract and all lessons, with the earlier corrections preserved.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapter 4.
- Each link into a changed heading of chapter 4 resolves.
- New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change other chapters, except for a link that this draft breaks.
