# rhai-book-use-k91

## Goal

Apply all lessons and the term map to chapter 6, “Reference”. Then run the mechanical gates for all of chapters 1–6.

## Context

- The node brief `rhai-book-use-k84` gives the required corrections and the term map.
- Chapter 6 is the lookup chapter. A reader can arrive at any entry, so each entry restates its terms.
- Chapter 6 holds most of the JSON version labels (F11). Keep exact `schemaVersion` values in the entries that teach them.
- Several headings use “Policy”, such as “Policy entry and settings”. Keep identifier headings, such as `SelectionRequest`, exact.
- Draft `rhai-book-use-k89` renamed twelve chapter 4 headings and changed their 97 anchors in this chapter. Link texts that quote an old title or use the old terms remain, for example “Policy behaviour”, “Policy-context shape” and “copy a policy”. Make each link text agree with the new chapter 4 heading or with the term map. The new titles are in the running log of `rhai-book-use-k89`.
- Draft `rhai-book-use-k90` renamed chapter 5's “Understand the operational limits” to “Know when Ravel cannot guarantee cleanup” and changed its six anchors in this chapter.
  Their link texts still say “operational limits”, “support limits”, “Signal and recovery limits”, “Cleanup limits” and “platform limits”. Make each one agree with the new heading or name the row of its table.
- Chapter 5 now links to the entries “`--context PATH`” and “JSON failure document” for the fields and values of those documents. Keep those entries and their anchors.

## Done when

- Chapter 6 follows the term map, the contract and all lessons, with the earlier corrections preserved.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–6.
- Each link from chapters 7–11 and the README into chapters 1–6 resolves.
- New lessons are in `docs/documentation-lessons.md`.

## Notes

- This is the last draft under the node. Its retirement closes `rhai-book-use-k84`.

## Decisions (running log)

- **Structure brief, 2026-10-28.** `docs/book/contract.md` §1 states the reader, the ordered section plan, the emphasis and the exclusions. The node brief of `rhai-book-use-k26` names it by path.
- **Inbound anchors, 2026-10-28.** A repository search found links into chapter 6 only at `#installed-layout`, `#command-line`, `#diagnostic-codes`, `#exit-statuses`, `#import-resolution`, `#json-failure-document` and `#--context-path`. The renamed headings below change none of them. Thus, their anchor changes stay inside chapter 6.
- **The review values, 2026-10-28.** Chapter 6 used the result `continue` and the prompt “Review the last commit. Then run ravel end-session continue.” from before draft `rhai-book-use-k88`.
  Chapters 1 and 5 now use `review-prompt.txt` and `changes-needed`. Chapter 6 takes the same values in its fragments, its inheritance table and its `end-session` examples (L57).
- **Fragments that quote chapter 5, 2026-10-28.** The `start-session` fragment, the configuration-script fragment, the JSON routing notice and the JSON failure document copy chapter 5's current example and its recorded real run (L64). The real-run values are those that draft `rhai-book-use-k90` recorded: 619 bytes, SHA-256 `813135da…`, the canonical prompt path and `selectionMs` 9.
- **Worker facts, 2026-10-28.** `run.rs` writes `promptSource` as the path of `PromptSource::File`, which `inputs.rs` builds from the caller's working directory. ADR `policy-evaluation-precedes-the-launch` gives the deadline, the unstoppable `sleep` and `pad`, and the isolation of memory, stack and abort failures. `front.rs` re-executes the running image. Chapter 6 restates chapter 1's three reasons in the words of chapter 1.
- **Real runs, 2026-10-28.** The checkout build (`ravel 1.1.0`) ran in `/tmp/ch6` on macOS with a private home, `env -i` and the script and prompt file of chapter 5.
  The refusal gave status 3 and the five lines in “Diagnostic codes”, with the message “This configuration script has no rule for the kind deploy.”
  The request of `--kind review --param repo=ravel --task-id T-17 --prompt-file review-prompt.txt` gave the line in “`SelectionRequest`”. Its prompt keeps the final newline.
  `ravel end-session changes-needed` outside a session gave status 1 and the same line as `continue`.
- **Headings, 2026-10-28.** L61 and the term map change eight headings. Identifier headings stay exact.
  - “Policy entry and settings” to “Configuration script and settings”.
  - “Policy and request” to “Configuration script and request”.
  - “Policy module” to “Contents of a configuration script”.
  - “Policy context and host operations” to “Selection context and host capabilities”.
  - “Host operations” to “Host capabilities”, the term of `CONTEXT.md` and of the chapter's own definition.
  - “Caller inputs and worker environment” to “Variables that Ravel and the configuration script read”.
  - “Reserved worker grants” to “Names that an environment grant cannot use”.
  - “Shipped sample” to “Example configuration scripts”. The entry has three files, and “shipped” is shorthand.
- **Link texts, 2026-10-28.** Each table link with the text “Policy behaviour” now names its destination's subject, for example “Define a configuration script” and “Harness runs”.
  The six links to chapter 5's renamed section use its heading or name its row.
  The note of “Exit statuses” claimed that one section describes two cases. Chapter 5's table describes only the abrupt stop. The interrupted result write is in “Interpret the exit status”. The note now links each case to its own section.
- **JSON version labels, 2026-10-28.** Prose names each document by its purpose. Exact values stay in the field tables of the routing notice, the failure document, the request and the selection context. The script fragment keeps them too.
  The `policy_invalid` remedy now names the exports and functions of the contents entry, not a “schema-3 module”.
- **Checks, 2026-10-28.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:figures`: 17 figures match. `task docs:book:build` passed.
  Strict `docs:ste` and `docs:breaks` passed for chapters 1–6. A copy of chapter 6 with a planted semicolon failed `ste`, and a copy with a 460-word run failed `breaks`.
  A local link check of the built book and the README passed 843 links. A copy of chapter 7 with the old anchor `#shipped-sample` failed it twice, and a README copy with a missing anchor failed it once.
  The built chapter 6 has 53 headings with 53 ids, as in the source, and no raw Markdown. A planted fence failed that search. It has none of the eight old heading ids.
  The general checker found no hard problem in chapter 6. The SHA-256 digests of 267 tracked inputs were the same before and after the runs.
- **Term sweep, 2026-10-28.** Outside link targets, code and fences, chapter 6 uses “policy” only in the sentences about literal names, the CLI help and Ravel's diagnostic words.
- **No in-session reviewer, 2026-10-28.** The changed claims are restatements of chapters 1 and 5 that the source supports. The others are fragments of recorded runs and link texts that the link check covers. The allowance stays unspent.
- **No later stage cut, 2026-10-28.** Copy-edit `rhai-book-use-k85` stands live under the document node. This retirement closes `rhai-book-use-k84`. Its “Done when” holds, and the document node's brief has its closure note.
