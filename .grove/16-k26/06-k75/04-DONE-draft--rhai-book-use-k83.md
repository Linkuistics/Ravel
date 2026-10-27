# rhai-book-use-k83

## Goal

Apply the owner's Read 1 patterns to chapter 6, “Reference”. Then run the mechanical gates for all of chapters 1–6.

## Context

- The node brief `rhai-book-use-k75` gives the findings and the facts that need a check against the source.
- Section 2 of `docs/book/contract.md` gives the ten patterns.
- Chapter 6 is the lookup chapter. A reader can arrive at any entry without the earlier chapters.
  Thus, “Explain a term before you use it” and “Repeat and restate” apply to each entry.
  The `> **Note:**` form of “Statements and asides” stays.

## Done when

- Chapter 6 follows each pattern in section 2 of the contract.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–6.
- Each link from chapters 7–11 and the README into chapters 1–6 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- This is the last draft under the node. Its retirement closes `rhai-book-use-k75` by the four steps of a close.
- Do not change chapters 7–11.

## Decisions (running log)

- **Structure brief, 2026-10-28.** The node brief names `docs/book/contract.md`. It states the reader (§1, “The reader”), the ordered section plan with chapter 6's row, and the emphasis and exclusions.
- **Real values, 2026-10-28.** The checkout build (`ravel 1.1.0`, Rhai path) ran four cases in `/tmp/ch6` with an empty environment and `TMPDIR=/tmp/ch6/tmp`.
  A policy printed `ravel::json_encode(request)` for `--param repo=ravel --task-id T-17`. Its routing notice used chapter 1's review values.
  A refusing policy gave the text failure `ravel: refused (policy_refused, stage selection): …` with status 3. A harness run of a fake classifier returned `{"elapsedMs":389,"error":null,"exitCode":0,"result":null,"signal":null,"status":"exited","stderr":"","stdout":"review\n"}`.
  `ravel end-session continue` outside a session gave chapter 1's line and status 1. `ravel --help` and `ravel end-session --help` gave the help text.
  The runs also printed stop attempts with `Operation not permitted`. This session's sandbox denies those signals, so the chapter does not quote them.
- **Facts checked against the source, 2026-10-28.** The worker asks the Ravel CLI over a Unix socket pair (`worker.rs`). The Ravel CLI answers `ravel::env` from the caller's environment and its grants (`rhai_worker/host.rs`, `bridge.rs`).
  The two reads, `env`, `harnesses`, `remaining` and `run_harness` send a request. The JSON, UUID and path helpers compute inside the worker (`bridge.rs`).
  `print` writes to the worker's standard output and `debug` to its standard error (`engine.rs`). The Ravel CLI adds `policy stdout:` or `policy stderr:` (`refusal.rs`).
  The source describes the reserved grant classes as runtime, loader and Ravel controls (`environment.rs`). The draft removed its own motive for them.
- **Shape of chapter 6, 2026-10-28.** The chapter opens with the reader's question and the words of every page. A short Rhai introduction with the three links and a table from question to entry follow.
  Each entry keeps its heading and anchor, and each now opens with the reader's goal or question. `start-session` and “Harness-run request” have order tables. `end-session` has chapter 1's table of processes.
  Fifteen new fragments give real values. They show the help text, the review session, both notices, a failure document and the text refusal.
  They also show the nested paths, settings, a request, a policy context, the discovery list and a harness-run map.
  Fragments from chapters 1, 4 and 5 keep their values, and their labels name the chapter.
- **A defect of my own draft, 2026-10-28.** An edit ended where an old sentence continued on the same line, so a closing fence in “`--json`” stayed open. The built page showed the rest of the entry as code, and the `end-session` heading had no id.
  The STE, break and example checks passed. The link check found the missing anchor. A search of the built HTML for raw Markdown found the leak, and it was empty for chapters 1–6 after the fix. L56 records the lesson.
- **No in-session reviewer, 2026-10-28.** Each new fact came from the source or a real run. Copy-edit, art and proof follow.
- **Handed forward, 2026-10-28.** The F2, F5 and F4 entries of the document node brief now give the chapter 6 place of each figure.
- **Checks, 2026-10-28.** Evidence class: mechanical.
  `task docs:examples`: 21 passed, 0 failed, 0 skipped, 53 fragments not run. `task docs:excerpts`: 9 passed. `task docs:figures`: 14 match. `task docs:book:build` had no error or warning.
  `task docs:ste -- --strict` and `task docs:breaks -- --strict` passed for chapters 1–6. The skill's prose checker reports no hard finding for chapter 6 and the lessons record.
  A script compared 534 links with the ids of the built HTML. They are the links into chapters 1–6 from all chapters and the README, and the links out of chapter 6. None failed. Its first run found three bad anchors of this draft. Each of the 40 H3 headings of chapter 6 has an id.
- **Lessons, 2026-10-28.** `docs/documentation-lessons.md` has L56. The applications of L04–L09, L39 and L54 name chapter 6.
- **Node close, 2026-10-28.** Each point of the node's “Done when” holds after drafts `rhai-book-use-k80` to `rhai-book-use-k83`. Copy-edit `rhai-book-use-k76` already follows the node, so this draft cuts nothing.
