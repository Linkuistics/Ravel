# rhai-book-use-k90

## Goal

Apply all lessons and the term map to chapter 5, “Run sessions”.

## Context

- The node brief `rhai-book-use-k84` gives the required corrections and the term map.
- Draft `rhai-book-use-k88` wrote the concrete review task and the caller's complete task contract in chapter 1 (F9).
  Chapter 5 teaches the caller's side in full. Restate that example where the caller writes its prompt and acts on each result.
- F11 applies in “Read notices separately from results”: name the routing notice and the failure document, not their versions.
- Check every heading against L61, including “Understand the operational limits”.
- Draft `rhai-book-use-k89` renamed twelve chapter 4 headings and changed their anchors in this chapter. Link texts that quote an old title or use the old terms remain, for example “supply policy context” and “set bounds”. Make each link text agree with the new chapter 4 heading or with the term map.

## Done when

- Chapter 5 follows the term map, the contract and all lessons, with the earlier corrections preserved.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapter 5.
- Each link into a changed heading of chapter 5 resolves.
- New lessons are in `docs/documentation-lessons.md`.

## Notes

- Keep the recorded dependency on the final notice path.

## Decisions (running log)

- **Structure brief, 2026-10-28.** `docs/book/contract.md` §1 states the reader, the ordered section plan, the emphasis and the exclusions. The node brief of `rhai-book-use-k26` names it by path.
- **Concrete case first, 2026-10-28.** The chapter opening restates chapter 1's review: its command, its result `changes-needed`, status 0 and the orchestrator's next action. Then the general words and the order table follow.
- **Review prompt and the caller's actions, 2026-10-28.** “Supply the prompt and caller data” shows chapter 1's prompt file with the same words and its four parts.
  “Accept a result only after success” replaces the capture fragment with a complete example. In it, an orchestrator script runs the review, keeps the status and the result together and acts on each outcome.
  The notice example of “Read notices separately from results” sends `changes-needed`, as in chapter 1, in place of `continue`.
- **Heading, 2026-10-28.** L61: “Understand the operational limits” becomes “Know when Ravel cannot guarantee cleanup”. The section's table opens with the conditions where the cleanup guarantee does not apply.
  The other headings name their subjects in familiar words and stay. Chapter 6 gets the new anchor only. Its link texts belong to `rhai-book-use-k91`.
- **New complete example, 2026-10-28.** The orchestrator example passes `task docs:examples`. A copy with one changed output line failed the check.
- **Real run of the `--json` review, 2026-10-28.** The notice example now uses chapter 1's script message, prompt file and result. Thus, the real-run fragment changed.
  The checkout build (`ravel 1.1.0`) ran in `/tmp/ch5r` on macOS, with a private home and the example's files. Status 0, standard output `changes-needed`.
  The fragment shows 619 bytes and SHA-256 `813135da…` for `policy.rhai`, `promptSource` as the canonical prompt path and `selectionMs` 9. `wc -c` and `shasum` of the file agree.
- **Checks, 2026-10-28.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. The new orchestrator example is the 22nd. `task docs:excerpts`: 9 passed. `task docs:book:build` passed.
  Strict `docs:ste` and `docs:breaks` passed for chapter 5. The general checker found no hard problem in chapter 5, the lessons record or the changed task files.
  A local link check passed 1,017 targets in the built book. A copy with the old chapter 5 anchor failed it six times. A repository search found no old anchor.
  The built chapter has 11 headings with 11 ids and no raw Markdown. A planted fence failed that search.
  The SHA-256 digests of chapters 5 and 6, the lessons record, `examples.md` and `scripts/examples.sh` were the same before and after the final run.
- **Term sweep, 2026-10-28.** Outside code, chapter 5 uses “policy” only in link targets, the sentence about literal names and the note on Ravel's diagnostic words. The same search on the parent revision found 30 lines with old terms or JSON version labels.
- **No in-session reviewer, 2026-10-28.** The new claims are example outputs, which the example check compares, and restatements of chapter 1 that agree with the source. The allowance stays unspent.
