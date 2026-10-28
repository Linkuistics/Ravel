# rhai-book-use-k116

## Goal

Before a procedure in chapter 5 asks the reader to act, make it supply its example and files.
It also supplies its context and complete command first.
Reconcile chapter 5 with the renamed configuration interface.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Draft `rhai-book-use-k115` applied the setup pattern to chapters 3 and 4. Use its words and order. Its running log records its choices.
- Chapter 5's “Run the examples of this chapter” section has the shape that F17 found in chapter 4.

## Done when

- Each chapter 5 procedure names its example and shows its workspace, files, purposes and contents before any action.
  It distinguishes the reader's files from what the command creates, and shows a complete command before its parts.
- Files, arguments, settings, fields, diagnostics and help in chapter 5 agree with the implementation.
  Re-run each changed example whose real-run fragment records its output (L64).
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict prose and break checks pass for chapter 5.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change other chapters, except for a link or title that this draft breaks.

## Decisions (running log)

- **Structure brief, 2026-10-29.** `docs/book/contract.md` §1 names the reader, the ordered section plan, the emphasis and the exclusions. Chapter 5's purpose is row 5 of that plan.
- **Baseline, 2026-10-29.** Before any edit, `task docs:examples -- docs/book/src/05-running-sessions.md` passed 8 examples, with 5 fragments not run.
- **Workspace, 2026-10-29.** Chapter 5 uses `~/ravel-chapter-5`, with one directory for each of its eight examples, as chapter 4 does.
  The first example stays in “Supply the prompt and caller data”, because its files need that section's explanation of the inputs.
  The setup section lists all eight examples and the four steps. The first example gets the tree, the path table and the command-part table after its command. Each later example gets a paragraph with its directory and its differences.
- **Prompt input, 2026-10-29.** `crates/ravel/src/inputs.rs` never reads a prompt from standard input, terminal or not. The chapter said “the standard input of a terminal”, which implied a read from other standard input. It now says “standard input”.
- **`--json` failures, 2026-10-29.** A run with `--json` and a natural exit printed a JSON failure document with `session_result_missing`. The chapter's claim that `--json` covers failures after the launch holds, although the help names only refusals and routing notices.
- **`--context` size, 2026-10-29.** The caller-data table now gives the default limit of 262,144 bytes for the compact JSON of the selection context, and its two controls. `limits.rs` and `context.rs` state the value and the measured encoding.
- **`|| exit "$?"`, 2026-10-29.** Ten reader command lines lost it (L68). `orchestrator.sh` keeps one, because it is a script file, not a line that the reader types.
- **`<scratch>/work`, 2026-10-29.** The `notices`, `cancel-selection` and `cancel-session` outputs now say that it stands for the reader's example directory, in chapter 4's words.
- **JSON fragment run, 2026-10-29.** The `notices` files ran again with the checkout build 1.1.0 in `/tmp/ravel-chapter-5/notices`, `TMPDIR=/tmp`. The script still has 619 bytes and digest `813135da…8246`. Only the paths and `selectionMs` (3) changed in the fragment.
  Chapter 6 quotes the earlier `/tmp/ch5r` run. The body of `rhai-book-use-k118` now names that difference, because chapter 6 belongs to that draft.
- **Interface, 2026-10-29.** Every option in chapter 5 occurs in the help of the checkout build, and a control `--policy-env` occurs zero times. Each diagnostic code, JSON field and settings key of chapter 5 occurs in `crates/ravel/src`.
  “Policy” occurs nowhere in chapter 5. The same search finds it in `contract.md`.
  The help of `--json` names only refusals and routing notices, but a `--json` run with a natural exit wrote a JSON failure document. The help belongs to the product, not to this draft.
- **Reader walk, 2026-10-29.** One `env -i` zsh process for each of the eight examples followed only the visible steps. It ran the page's `mkdir` and `cd`, saved the files without the executable bit, then ran each command block.
  All eight matched stdout, stderr claims and status, with `<scratch>/work` replaced by the example directory. A control without `chmod` failed seven. It cannot reach `confined`, which has no `bin`.
  The final walk ran on chapter 5 with SHA-256 `32a0c283…4b11`. This is execution evidence, not reader comprehension evidence (L41, L67).
- **Checks, 2026-10-29.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:book:build` passed with no warning or error.
  Strict `docs:ste` and `docs:breaks` passed for chapter 5. A local link, raw-Markdown and heading-id check of the built chapters 1–11 found no problem in 794 links. A planted broken anchor and a planted fence failed it.
- **Lessons, 2026-10-29.** L67 records this application and the control's reach. L69 records the qualifier that implied a false complement. The book checker and the general prose checker report the same findings for the lessons file as for the parent revision.
- **Nothing handed forward, 2026-10-29.** This draft changed no figure, and no figure shows the setup of an example. Copy-edit `rhai-book-use-k94` already reads chapter 5's new setup as connected instructions.
