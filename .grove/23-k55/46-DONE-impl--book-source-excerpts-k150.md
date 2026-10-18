# book-source-excerpts-k150

## Goal

Build the source-excerpt check that the contract's examples policy requires:
"Each walkthrough excerpt names the file it comes from, and a Taskfile task
checks that the excerpt still appears there byte for byte." Chapter 8 follows
one session through the code and quotes the source, and its draft must not
start before this check exists and has been seen to fail.

## Context

- Contract: `docs/book/contract.md`, "Examples policy" (the last bullet) and the
  section plan's note that chapter 8 "quotes source excerpts but does not cover
  the complete source". The model is the example check: `docs/book/examples.md`
  (the convention and what the check does), `scripts/examples.sh`,
  `scripts/examples-extract.awk`, `scripts/examples.test.sh` (controls and
  mutants) and the `docs:examples*` tasks in `Taskfile.yml`.
- **The two checks must agree on one page.** The example check fails closed for
  a fenced block outside an example: a block with no language tag fails, and so
  does a block in a language that is not on its never-an-example list (`rust`,
  `toml`, `text` and others are on it; `ts`, `sh` and `console` are not). A
  chapter 8 excerpt from the worker (TypeScript) is such a block today. Settle
  how a labelled excerpt is recognised by both checks so that no excerpt needs
  a language added to the never-an-example list, and so that an unlabelled
  `ts`, `sh` or untagged block still fails closed.
- Decide the convention, and write it in a note beside `examples.md`
  (`docs/book/excerpts.md`, unless the grammar shares so much with
  `examples.md` that one note reads better). It must state: the label
  paragraph (it begins with a fixed word and names the source file by a plain
  repository-relative path, with no line numbers because a line number goes
  stale), the one fenced block that the label governs, and what "byte for
  byte" means here (one contiguous run of whole lines of that file, LF line
  endings, with how a final newline counts). An excerpt does not contain an
  elision marker. A reader who needs two parts of one function sees two
  excerpts, each with its own label. A label can say in words what comes before
  or after.
- The check reads every chapter under `docs/book/src/`, because chapters 9, 11
  and 12 can also quote the source. It must pass on chapters 1–6 as they are.
- Source paths are checkout paths. Say what the check does with a path that is
  absent, a directory, a symbolic link, a path with `..`, a file that is not
  tracked in the checkout, and a generated file (the book should not quote
  `target/`).
- The chapters will also hold `rust`, `ts` and `toml` blocks that are fragments
  or complete examples under the existing rules. Keep those rules. Decide
  whether a `rust` or `ts` block that is neither a fragment, an example nor an
  excerpt is an error in chapters 7–12, with the reason, and record the
  decision in the note.

## Done when

- A Taskfile task `docs:excerpts` checks every excerpt and exits 1 with a
  message that names the page, the label and the first differing line, on any
  excerpt that no longer occurs in its file. A `docs:excerpts:test` task (and
  mutants if the example check's pattern earns them) shows the controls below.
  Both tasks are listed in `task --list`, and both stay out of `task check`.
- Controls, each seen to fail before it is credited: a changed byte in the
  excerpt; a changed byte in the source (on a scratch copy, never in the real
  source); a moved or deleted source file; CRLF in the page; trailing
  whitespace; a label with no block, a block with no label, and two blocks
  under one label; a path with `..`, an absolute path and a missing path; an
  excerpt that is a substring in the middle of a line rather than whole lines;
  and one passing excerpt for each language chapter 8 is likely to use (Rust,
  TypeScript, TOML, shell). Removing one safeguard at a time from a copy of the
  checker makes one control fail for each.
- `task docs:examples` and `task docs:excerpts` both pass on a scratch page that
  holds an example, a fragment and a TypeScript excerpt of a real file
  (`crates/ravel/worker/sdk/index.ts` or `crates/ravel/src/run.rs`), and both
  pass on chapters 1–6 unchanged. The unlabelled-block control of the example
  check still fails.
- The note and `docs/book/contract.md` ("Examples policy") agree. Edit the
  contract's bullet only to point at the note, as the examples bullet does.
- `task docs:ste -- --strict` passes on the new note.

## Notes

- Do not draft any part of chapter 8 here. The scratch page is a fixture.
- A walkthrough excerpt is a quotation of source. If the source changes, the
  check fails and a person decides whether the chapter or the quotation moves.
  Say so in the note.
- Scripts follow the existing style (`set -euo pipefail`, ShellCheck clean).
  `task check` runs the script lint, so run it before the leaf ends.

## Decisions (running log)

- **One extractor reads both conventions.** `scripts/examples-extract.awk` now
  also recognises an excerpt label (a paragraph whose first word is `Excerpt` or
  `Excerpts`, emphasis marks ignored). The block directly after it is covered
  like a fragment's block, so no language joins the never-an-example list, and
  it is written to `xc-NNNN/` for `scripts/excerpts.sh`. A bare `ts`, `sh` or
  untagged block still fails closed. This is how both checks agree on one page.
- **Label grammar.** First line is exactly ``Excerpt from `PATH` `` (path in code
  marks, no emphasis around the paragraph); the rest of the paragraph is free
  text for what comes before or after. A malformed label, a label with no block
  (paragraph, heading or page end between), an empty or blank block and a second
  block under one label are errors reported by `docs:excerpts`.
- **Byte for byte = whole lines.** The block's lines must equal consecutive lines
  of the file (a line ends at LF or end of file; a final LF is not a line). The
  fence end is not a claim about the file's final newline. CRLF page refused; a
  source line that holds CR cannot match. A run that occurs twice passes.
- **Path rules.** Plain relative path (the file-label alphabet, no `.`/`..`/empty
  part, not absolute). Refused: absent, directory, non-regular file, symbolic
  link (path or any parent), untracked by `git ls-files`, a `target` or
  `node_modules` component (even if force-added), and `.grove/` (deleted by the
  finish cycle). Not a git checkout is an error.
- **Bare `rust` and `toml` blocks are errors on every page**, not only chapters
  7–12: they left the never-an-example list. Reason: the book describes Ravel's
  own source (Rust, TOML), so a bare block is either an unchecked quotation or
  invented code that reads as one; a page-number scope would also go stale.
  Chapters 1–6 hold none. `shapes.md` fixture used a bare `rust` block as its
  "never an example" case and now uses `text`. Other listed languages (ruby,
  yaml, …) stay; a chapter that quotes one uses an excerpt label.
- **Mutant fix.** The example check's existing `label-path-unchecked` mutant
  matched the first occurrence of the path rule, which moved into the new
  excerpt code; its `FROM` now includes the file-label context. 29 of 29 killed.
- **`docs:examples` takes `-- PAGE-OR-DIR...`** (CLI_ARGS) so the Done-when scratch
  page can be run through the task. `docs:excerpts:test` runs controls and then
  mutants (the `docs:ste:test` pattern), which keeps two new tasks in the list.
- **Evidence.** `task docs:excerpts:test`: all controls ok, 24 of 24 mutants
  killed. `task docs:examples:test` ok and `docs:examples:mutants` 29 of 29.
  Scratch page (example, fragment, TS excerpt, Rust excerpt of real files):
  `docs:examples` 3 passed, `docs:excerpts` 2 passed. Chapters 1–6: both pass.
  `task docs:ste -- --strict docs/book/excerpts.md` 0 problems. `task check`
  green (one earlier red run was `worker_reap_admission_*` timing flakiness under
  load; three isolated reruns passed and no Rust file changed).
