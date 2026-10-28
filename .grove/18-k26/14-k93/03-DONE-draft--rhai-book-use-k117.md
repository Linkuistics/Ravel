# rhai-book-use-k117

## Goal

Read chapters 1 and 2 against the full contract and lessons, especially the setup pattern and the renamed configuration interface.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Drafts `rhai-book-use-k115` and `rhai-book-use-k116` applied the setup pattern to chapters 3–5. Use their words.
- Chapters 1 and 2 have no complete example. They show real-run fragments and a configuration script that the reader reads but does not run.

## Done when

- Each fragment is visibly a fragment and says what it omits. Nothing in these chapters reads as a procedure that lacks its setup.
- Each recorded fragment agrees with a fresh run of its example, where its inputs changed (L64).
- Files, arguments, settings, fields, diagnostics and help agree with the implementation. No old literal-name explanation remains.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict prose and break checks pass for chapters 1 and 2.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change other chapters, except for a link or title that this draft breaks.

## Decisions (running log)

- **Structure brief, 2026-10-29.** `docs/book/contract.md` §1 names the reader, the ordered section plan, the emphasis and the exclusions. Chapters 1 and 2 are rows 1 and 2 of that plan.
- **Baseline, 2026-10-29.** Before any edit, strict `docs:ste` and `docs:breaks` passed for chapters 1 and 2. Neither chapter has a complete example.
- **Fresh runs, 2026-10-29.** The checkout build 1.1.0 ran chapter 1's review request on macOS 27.0.1, with `TMPDIR=/tmp` and a private home. The chapter's script was at `home/.config/ravel/config.rhai`, and a fake agent wrote `review-notes.md` and sent `changes-needed`.
  Standard output was `changes-needed` with no newline, the status was 0, and standard error had the routing notice and two cleanup reports. The `--json` run gave the same `selection` member, byte for byte. Its `resolution` member names the program file that Ravel found.
  A nested run gave the same lines as chapter 1's fragment. Only the random parts of the two paths differed. Outside a session, `ravel end-session changes-needed` gave the fragment's line and status 1. No recorded value changes.
- **Interface, 2026-10-29.** The options in both chapters occur in the help of the checkout build. The settings reader accepts `timeoutMs`, `contextBytes`, `configEnv`, `harnesses` and `budgets`, which agree with the chapters' list.
  `ravel::read_text`, `ravel::read_json`, `ravel::run_harness` and `load_context` occur in the worker source, and a misspelt control name occurs nowhere. The production worker launch passes an empty environment.
  “Policy” occurs in neither chapter, and the same search finds it in `contract.md`. No old literal-name explanation remains. The five figures of the two chapters have no “policy” label.
- **Nothing to run in Part I's first two chapters, 2026-10-29.** Each chapter now says near its opening that its code blocks are fragments and that chapter 3, “Run the first session”, gives the first session that the reader runs.
  The request fragment says what it needs before a reader can type it. The script fragment says that the reader saves nothing.
- **Prompt-file label, 2026-10-29.** The label said that Ravel does not read the words in the file. Ravel reads the file and gives its text unchanged to the configuration script, so the label now says that Ravel does not interpret the text (L69).
- **The real run names its script, 2026-10-29.** The run's routing notice names `local` and `fake`, but the table above it says that the configuration chooses Claude Code. The text now says that the run used the chapter's own short script at its default path, which selects a fake agent.
- **The JSON fragment, 2026-10-29.** It said “of the same session”. A `--json` run is a separate session, so the label now says “of the same request, run again with `--json`”.
- **Request fragment, 2026-10-29.** A first wording said that chapter 3 gives the reader “each of these”. Chapter 3 gives no `review-prompt.txt`, so the text now says that chapter 3 installs Ravel and then gives the files and command of a simpler first session.
- **A departure in a table row, 2026-10-29.** The general prose checker reported “correct” and `fix` as one action, in the parent revision too. “Correct” describes the work, and `fix` is the literal kind of a later session. The checker scopes a marker to one line, so the departure marker is in the table row.
- **Checks, 2026-10-29.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:book:build` passed with no warning or error.
  Strict `docs:ste` and `docs:breaks` passed for chapters 1 and 2. A planted semicolon in a scratch copy of chapter 2 failed the strict prose check.
  A local check of the built book resolved all 1,719 local links and anchors, and found no raw Markdown in chapters 1 and 2. A planted broken anchor and a planted fence failed it. Each chapter heading has an id. The only headings without one are mdBook's menu and help titles.
  The final chapter 1 has SHA-256 `758d09f3c8cc…`, and chapter 2 has `b2d89d506840…`.
- **Lessons, 2026-10-29.** L69 records the prompt-file label. L70 records the rule for commands on a page before the installation. L71 records the rule for a real run whose values differ from the story.
  The general prose checker and the book checker report the same findings for the lessons file as for the parent revision.
- **Nothing handed forward, 2026-10-29.** This draft changed no figure, and no figure label of chapters 1 and 2 has an old configuration name. Copy-edit `rhai-book-use-k94` reads the new sentences with the rest of the chapters.
