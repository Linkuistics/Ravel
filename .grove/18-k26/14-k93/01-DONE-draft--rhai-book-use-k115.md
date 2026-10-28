# rhai-book-use-k115

## Goal

Before a procedure in chapters 3 and 4 asks the reader to act, make it supply its example and files.
It also supplies its context and complete command first.
Resolve F17 in chapter 4, and reconcile both chapters with the renamed configuration interface.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- F17 and F18 are in `rhai-book-use-k74`. Contract §2, “Make example setup concrete”, and lesson L67 state the pattern.
- `configuration-names-k114` closed the product rename. Derive names from the source and the built CLI, not from memory.
- `docs/book/examples.md` and `scripts/examples.sh` prepare example files automatically. That preparation does not show that the prose told the reader to do it.

## Done when

- Chapter 4's setup names the first example and shows its workspace, directories, file paths, purposes and contents before any action.
  It distinguishes the files that the reader saves from the directories that the command creates.
  It shows a complete command before a table explains its parts, and each later example states its differences where it occurs.
- Chapter 3's first session follows the same pattern.
- Fragments in both chapters stay visibly distinct from complete procedures.
- Files, arguments, settings, fields, diagnostics and help in both chapters agree with the implementation.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass. Strict prose and break checks pass for chapters 3 and 4.
- Inbound links to any changed heading resolve. Figure work is under “Handed forward” in `rhai-book-use-k26`, with exact locations.
- New transferable lessons are in `docs/documentation-lessons.md`.

## Notes

- Do not change chapters 1, 2, 5 and 6, except for a link or title that this draft breaks. Later drafts own them.

## Decisions (running log)

- **Four drafts, 2026-10-29.** Chapters 1–6 have about 7,640 lines and 417 KB. The node brief gives the decomposition and its reasons.
- **Baseline, 2026-10-29.** Before any edit, `task docs:examples` for chapters 3 and 4 passed 13 examples, with 5 fragments not run.
- **Headings stay, 2026-10-29.** The F17 correction changes the content and order inside sections, not their headings.
  Chapters 1, 6 and 8 link to `#run-the-first-session` and `#define-a-configuration-script-and-read-the-request`. Those anchors stay valid.
- **Named workspaces, 2026-10-29.** Chapter 3 uses `ravel-first-session`. Chapter 4 uses one workspace, `ravel-chapter-4`, with one named directory for each example.
  A separate directory for each example matters: later commands write `.ravel-choice`, `call.args`, `invalid-answer` and `failed-call`, which change the next command's result.
- **The first chapter 4 example moves, 2026-10-29.** It moves from “Define a configuration script and read the request” into “Run the examples of this chapter”.
  The reader then sees the files and the complete command before the table that explains the command's parts.
  The definition section explains that same file after the run, which also follows L57: a concrete case before the general rules.
- **No `|| exit "$?"` in a reader's command, 2026-10-29.** The checker runs each block with `sh -c`, but the reader types the lines into an interactive shell.
  There, a failed `chmod` followed by `exit` closes the reader's shell. Without it, a failure still changes the next output, so the check still fails.
  In the loop of the interactive-routes example, `|| break` replaces it for the same reason.
- **Chapter 3, 2026-10-29.** “Run the first session” now names `~/ravel-first-session` and shows its paths in a tree.
  A table says who makes each path and why. Four numbered steps and a labelled `mkdir`/`cd` fragment precede the complete example.
  A table of the command's parts follows the command. Ravel makes the result channel under `tmp/`: a real run showed the path.
  Without `--config`, the same private home gives `config_missing` and status 3 for `home/.config/ravel/config.rhai`, as the page now says.
- **Chapter 4, 2026-10-29.** “Run the examples of this chapter” now holds the first example, `fixed-route`, with its tree, path table, steps, files and complete command.
  The command-part table follows the command. Each of the eleven later examples has a paragraph that names its directory and its differences.
  `VERSION` values `my-policy-1` and `first-policy-1` became `fixed-route-1` (F18). No other page quotes either value.
  The refusal table and the Codex example now say that `<scratch>/work` stands for the reader's own example directory.
- **Workspace under `~`, 2026-10-29.** A read of the new section in order found that a relative `mkdir` for the second example ran inside `fixed-route`, after step 1's `cd`.
  The workspaces are now `~/ravel-first-session` and `~/ravel-chapter-4/NAME`, so each step works from any current directory.
- **Smoke-task prerequisites, 2026-10-29.** `scripts/harness-smoke.sh` builds Ravel with `cargo build`. The table of the two smoke tasks now names Task and the Rust toolchain, with a link to chapter 10's checkout preparation.
- **Reader walk, 2026-10-29.** One `env -i` zsh process followed the visible steps for all 13 examples of chapters 3 and 4.
  It had a private reader home and the checkout build on `PATH`.
  It used the page's `mkdir -p ~/…/bin` and `cd ~/…` commands and saved each file without the executable bit. Then it ran each command block.
  Twelve examples matched stdout and status exactly. The Codex example differed only by the `<scratch>/work` substitution of the schema path.
  A control without `chmod` failed with status 126. This is mechanical execution evidence, not reader comprehension evidence (L41, L67).
- **Interface, 2026-10-29.** Every Ravel option in chapters 3 and 4 is in `ravel --help` or `ravel start-session --help` of the checkout build.
  Fifty diagnostic codes, fields and settings keys from chapter 4 each occur in `crates/ravel/src`. The old names `policy_refused`, `policyCode` and `policy-env` occur zero times there.
- **Checks, 2026-10-29.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:book:build` passed with no warning.
  Strict `docs:ste` and `docs:breaks` passed for chapters 3 and 4. A local link check of the built chapters passed 835 targets, and a planted old anchor failed it.
  A search of the built chapters 3 and 4 found no raw Markdown, and a planted fence failed that search. Each heading has an id.
- **Lessons, 2026-10-29.** L67 records this draft's application and reader walk. L68 records the interactive-shell rule for example commands.
  The general prose checker reports in the lessons file only the six hard findings that the parent revision already had. The book checker reports the same set as the parent revision.
- **Nothing handed forward, 2026-10-29.** This draft changed no figure. The copy-edit leaf `rhai-book-use-k94` already reads the new setup as connected instructions.
