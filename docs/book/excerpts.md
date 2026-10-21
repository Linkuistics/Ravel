# Excerpts: convention and check

This note settles how a chapter quotes source code and how `task docs:excerpts`
compares the quotation with the file. The contract (`contract.md`, "Examples
policy") owns *that* each excerpt is exact. This note owns the mechanics. It
sits beside the contract and is not part of the book that readers get.

A walkthrough excerpt is a quotation of source. If the source changes, the
check fails. Then a person decides if the chapter or the quotation moves. The
check never rewrites a page.

## What a reader sees

An excerpt is a label paragraph and one fenced block. The label names the file.
The block holds lines of that file.

~~~~markdown
Excerpt from `crates/ravel/src/run.rs`: the first two imports of the file.
The comment on the module comes before them.

```rust
use std::ffi::{OsStr, OsString};
use std::io::{self, Write as _};
```
~~~~

The same Markdown is what GitHub, an editor and mdBook show. The label is
ordinary text for a reader. Nothing in an excerpt is hidden.

## The grammar

- **The label** is a paragraph. Its first line starts with `Excerpt from` and
  then a path in code marks. No emphasis marks surround the paragraph. The
  rest of the paragraph is free text. Use it to say in words what comes before
  or after the lines.
- **The block** is the one fenced block directly after the label. An HTML
  comment between them is ignored. A paragraph or a heading between them leaves
  the label with no block, and that is an error. The end of the page does the
  same.
- **One label governs one block.** A second fenced block with no paragraph
  between the two is an error. Give the second block its own label.
- **The language tag** is for the reader and the highlighter. The check does not
  read it. A block can have any tag, or none. Fences can be backticks or
  tildes, of any length of three or more, and an excerpt can sit in a
  blockquote. The same shapes that `examples.md` cannot read are errors here,
  for example a fence on a list-item line.
- **An empty block** is an error, and so is a block of blank lines only.
- **An excerpt does not contain an elision marker.** A reader who needs two
  parts of one function sees two excerpts. Each has its own label.
- **A malformed label** is an error. A paragraph whose first word is `Excerpt`
  or `Excerpts` must have the exact form above. Thus a typo cannot turn an
  excerpt into a block that nothing checks.

The label gives no line numbers, because a line number goes stale when an
earlier line changes. The check looks for the lines in the whole file. If the
same lines occur more than one time, the excerpt passes.

## The path

The path is relative to the root of the checkout. It is plain: letters, digits,
`.`, `_`, `-` and `/` only. It does not start with `/`. It has no empty part, no `.` part
and no `..` part. A path that is not plain is an error before any file is
examined. The check then does this with the file:

| The path is | The check |
|---|---|
| absent (moved, removed or misspelt) | fails and says that the file does not exist |
| a directory | fails. Name a file. |
| a symbolic link, or inside a linked directory | fails. Name the file that the link points to. |
| not tracked by git | fails. A reader finds only tracked files in a clone. |
| inside a `target` or `node_modules` directory | fails, even if someone adds the file to git by force |
| inside `.grove` | fails. The finish cycle removes that directory. |
| a regular tracked file | is compared |

A generated file is untracked. The `.gitignore` of the repository ignores
`target/`, so a build output fails the rule for tracked files. The rule by name
for `target` and `node_modules` stays as a second guard.

The check asks git (`git ls-files`). A new file counts after git lists it. In a
Jujutsu checkout, a `jj` command (for example `jj st`) makes git list a new
file. The check stops with an error if the root is not a git checkout.

## What "byte for byte" means

The check compares lines. A line is the bytes up to the next LF, or up to the
end of the file. A final LF does not start another line. The block holds one or
more whole lines. Its last line ends at the end fence.

- **One contiguous run.** The lines of the block equal consecutive lines of the
  file. The first line of the block is a whole line of the file, and the same is
  true of the last line. A part of a line fails.
- **LF line endings.** A page with CRLF line endings is an error. A source line
  that holds a CR byte does not equal a page line, so it cannot be quoted.
- **Every byte counts.** Spaces and tabs at the end of a line are part of the
  line. An editor that removes them from a page makes the check fail. This is
  correct, because the quotation is no longer exact.
- **The final newline.** The end fence ends the last line of the block. That
  newline is not a claim about the file. A file with no final newline can be
  quoted to its last line. A file with a trailing newline has no extra empty
  line after it. A blank line at the end of a block is a real blank line, and
  the file must hold it.
- **Nothing else changes.** The check removes blockquote markers from a quoted
  page, as the example check does. It does not change tabs, indentation or
  encoding.

When an excerpt fails, the message names the page and the line of the label,
the label itself, and the first line that differs. It shows that line from the
block and from the file, with control characters written as `\xNN`. When the
first line of the block is not in the file, the message says so. If a longer
line of the file holds that text, the message says that too.

## One extractor, two checks

`scripts/examples-extract.awk` reads the pages for both checks. It is the only
code that decides which block is which, so the two checks cannot disagree about
one page. A block under an excerpt label is not an example. Thus a TypeScript
excerpt needs no language on the never-an-example list of `examples.md`.

| Block | `task docs:examples` | `task docs:excerpts` |
|---|---|---|
| under a `Complete example` label | runs it | does not read it |
| under a `Fragment` label | does not run it | does not read it |
| under an `Excerpt from` label | does not run it | compares it with the file |
| `text`, `yaml`, `markdown`, `diff` and other languages that are never examples, with no label | accepts it | does not read it |
| any other block with no label, such as `ts`, `sh` and `console`, and a block with no tag | fails the page | does not read it |

A `rust` or `toml` block is in the last row. The earlier list of
`examples.md` held these two languages, so a bare block was valid. The list does
not hold them now. A block of either language is a fragment, a complete example
or an excerpt. The book describes Ravel's own source, and that source is Rust
and TOML. A bare block can be a quotation that nothing checks, or code that the
author invented and a reader takes for a quotation.

This rule holds for every page. It is not limited to chapters 7–11, because a
page number goes stale, and chapters 1–6 hold no such block. The other
languages on the list stay as they are. A chapter that quotes one of them, for
example the Homebrew formula, uses an excerpt label to get the check.

A fragment shows code that the author wrote for the page. An excerpt quotes the
source. The check cannot find a fragment that is in truth a quotation, so use an
excerpt label for every quotation, even a short one.

## Run the check, and check the checker

| Command | What it does |
|---|---|
| `task docs:excerpts` | checks every excerpt under `docs/book/src/`. `-- PAGE-OR-DIR...` checks those instead. It needs no build. |
| `task docs:excerpts:test` | runs the controls, and then the mutants |

The check is `scripts/excerpts.sh`. The controls are `scripts/excerpts.test.sh`.
**`task check` does not run any of it**, so an edit to a chapter or to a quoted
file cannot turn `task check` red. A unit that drafts a chapter runs
`task docs:excerpts` itself, as part of its own proof. A change to a quoted file runs it too.

Each control is made to fire against a wrong subject. A control that changes a
source file does it in a scratch repository, never in the checkout:

- a changed byte in the excerpt, and a changed byte in the source
- a moved source file, and a removed source file
- a CRLF page, and spaces at the end of a line
- a label with no block (a paragraph, a heading or the end of the page comes
  between them)
- a block with no label, and two blocks under one label
- a path with `..`, an absolute path, a missing path, and the other paths that
  are not plain
- a directory, a symbolic link and a linked directory, an untracked file, and a
  file in `target`, `node_modules` or `.grove`
- a root that is not a git checkout
- an excerpt that is a part of a line, at the start and at the end
- a final newline that is absent, a blank line at the end, and a source with
  CRLF
- an empty block, a block of blank lines, and a malformed label
- one good excerpt for each of Rust, TypeScript, TOML and shell
- the fence shapes: tildes, a blockquote, an info string, and a four-backtick
  fence around a file that holds a fence
- one stale excerpt among good ones, which gives status 1 and a count

The mutants remove one safeguard at a time from a copy of the checker. The
controls run first against the unmutated copy, so no control can "kill" a
mutant because it fails already. Each mutant must make its control fail.
`task docs:examples:test` holds the controls of the example check for this
convention. A scratch page with an example, a fragment and a TypeScript
excerpt passes both checks. A bare `ts`, `sh`, `rust` or `toml` block still
fails.

### Why a shell script

The reasons are those of `examples.md`. The pages are Markdown that the compiler
does not know. The figure, example and book checks beside this one are scripts
under `scripts/` with a `task` wrapper, so a contributor meets one shape.
