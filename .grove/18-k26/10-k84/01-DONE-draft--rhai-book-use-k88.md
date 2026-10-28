# rhai-book-use-k88

## Goal

Settle the shared terms, then apply all lessons to Part I, chapters 1–3.
Write the new chapter 1 explanations: the concrete review task, the caller's complete task contract and the separate worker.

## Context

- The node brief `rhai-book-use-k84` gives the required corrections, the term map and the decomposition.
- Findings F9–F16 are in `rhai-book-use-k74`. The owner reviewed all of Part I before them.
- `docs/book/contract.md` §2 and `docs/documentation-lessons.md` L01–L62 hold the rules.

## Done when

- `docs/book/ste.md`, `CONTEXT.md`, `SUMMARY.md` and the contract use the term map and the new titles.
- Chapter 1 opens with a concrete review task: files, prompt, permitted results, the `end-session` instruction and the caller's action for each result.
  The general caller model follows that example.
- Chapter 1 explains the Rhai worker: a second copy of `ravel`, the parent's deadline and host service, and failure isolation.
  It also says that the worker ends before the final launch.
- Grove links to its repository where each page introduces it. Chapter 3 says “`cd` into it”.
- JSON prose names the document's purpose without a version label.
- Chapters 1–3 follow the term map and all lessons. Every link into the renamed headings resolves.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict STE and break checks pass for the changed files.
- The document node brief `rhai-book-use-k26` names each figure change for art under “Handed forward”.
- New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change chapters 4–11, except for a link or title that this draft breaks.

## Decisions (running log)

- **Term map, 2026-10-28.** The brief of `rhai-book-use-k84` records it and its reasons.
  `docs/book/ste.md`, “Configuration words” and “JSON documents”, `CONTEXT.md` and contract §1–§2 now carry it.
  `SUMMARY.md` and the chapter 2 and chapter 4 titles use the new titles.
- **Worker facts, 2026-10-28.** Direct source reads, not graph evidence: the graph server did not connect.
  `rhai_worker/front.rs`, `executable`, uses `current_exe`. `worker.rs`, `supervise`, builds a `Command`, sets `pre_exec` and calls `spawn`.
  ADR `policy-evaluation-precedes-the-launch` gives the deadline, the unstoppable `sleep` and `pad` calls, and the isolation of memory, stack and abort failures.
- **Real run of the review example, 2026-10-28.** The checkout build (`ravel 1.1.0`) ran in a private home on macOS, with the chapter 1 script and a fake agent.
  The fake wrote `review-notes.md` and ran `ravel end-session changes-needed`. Status 0, standard output `changes-needed` without a newline.
  Standard error had the routing notice and two `Operation not permitted` stop reports. `ravel end-session approved` outside a session gave the chapter's line and status 1.
- **Anchors, 2026-10-28.** Four headings changed: “Evaluation and supervision” in chapter 2, two chapter 1 headings and one chapter 3 heading.
  One substitution changed every inbound link in chapters 3–11. The spec's remedy and `typescript-worker-cutover-k31` now cite the chapter 4 title.
  The chapter 4 heading “Define a policy and read the request” stays for `rhai-book-use-k89`, with its inbound link from chapter 3.
- **Checks, 2026-10-28.** `task docs:examples`: 21 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:book:build` passed.
  Strict `docs:ste` and `docs:breaks` passed for chapters 1–3. Strict `docs:ste` passed for `SUMMARY.md` and `CONTEXT.md`.
  The general checker found no hard problem in the changed records. In the lessons file, the book checker found the same problems as at the parent revision.
  A local link check passed 1,690 targets. A planted old anchor failed it. A search of the built pages found no raw Markdown, and a planted leak failed that search.
