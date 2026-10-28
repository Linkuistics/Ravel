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

## Decisions (running log)

- **Structure brief, 2026-10-28.** `docs/book/contract.md` §1 states the reader, the ordered section plan, the emphasis and the exclusions. The node brief of `rhai-book-use-k26` names it by path.
- **Worker facts, 2026-10-28.** Direct source reads, because the graph server did not connect.
  `rhai_worker/front.rs`, `executable`, launches `/proc/self/exe` on Linux and the canonical `current_exe` on macOS. `worker.rs`, `supervise`, clears the environment, spawns the worker and stops and reaps it before it returns.
  ADR `policy-evaluation-precedes-the-launch` gives the deadline, the unstoppable `sleep` and `pad`, and the isolation of memory, stack and abort failures.
  Chapter 4 restates the words of chapters 1 and 2: the deadline, requests from the script and the isolation of a failure.
  It also says that the worker ends before the launch.
- **Concrete case first, 2026-10-28.** “Follow the evaluation of a configuration script” follows chapter 1's review script, with its real kind, program and routing notice, before the general order. The chapter opening names the same review.
- **Headings, 2026-10-28.** L61 and the term map change these headings. Each new heading names its subject:
  - “Follow the evaluation of a policy” to “Follow the evaluation of a configuration script”.
  - “Define a policy and read the request” to “Define a configuration script and read the request”.
  - “Validate a policy with fake agents” to “Validate a configuration script with fake agents”.
  - “Use the starter route table” to “Put each kind and its route in one table”.
  - “Resolve policy imports” to “Import other Rhai files”.
  - “Offer a checkout choice” to “Let a repository choose an arrangement”.
  - “Supply policy context” to “Supply selection context”.
  - “Use the data helpers” to “Check JSON with the data helpers”.
  - “Set bounds and grant environment names” to “Set bounds and environment grants in the settings”.
  - “Route to Claude Code and Codex” to “Select Claude Code or Codex for the caller's work”.
  - “Copy a policy from another configuration” to “Copy a configuration script and its files”.
  - “Review your policy before real use” to “Review your configuration script before real use”.
  The other headings already name their subjects. Chapters 3, 5, 6 and 10 get the new anchors only. Their link texts belong to drafts `rhai-book-use-k90` and `rhai-book-use-k91`.
- **Messages in example scripts, 2026-10-28.** A refusal `message` is prose that the reader sees on standard error. It follows the term map, as chapter 1's script does. Expected outputs change with it. `VERSION` strings, `reason` labels and Ravel's own diagnostic words stay.
- **Process verbs, 2026-10-28.** Chapter 4 says “start” for a process (L13). The copied worker explanation says “starts” where chapter 1 says “launches”.
- **Checks, 2026-10-28.** `task docs:examples`: 21 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:book:build` passed.
  Strict `docs:ste` and `docs:breaks` passed for chapter 4. The general checker found no hard problem in chapter 4, the lessons record or the changed task files.
  A local link check passed 2,054 targets in the built book. A planted old anchor failed it. A search of old anchors in the repository found none, and the same search found 97 lines in the parent chapter 6.
  The built chapter has 19 headings with 19 ids and no raw Markdown. A planted fence failed that search. The chapter digests were the same before and after the final run.
- **Term sweep, 2026-10-28.** Outside code, chapter 4 uses “policy” only in the sentence about literal names. “Owner” remains for the role, its labels and its sign-in.
