# Examples: convention, seam and checking

This note settles how a complete example is written in a chapter and how
`task docs:examples` runs and compares it, so a drafting session and the checker
read the same blocks. The contract (`contract.md`, "Examples policy") owns *that*
every complete example runs; this note owns the mechanics. It sits beside the
contract and is not part of the published book.

## What a reader sees

A complete example is ordinary Markdown: a label paragraph, then labelled code
blocks. Nothing in it is hidden from a reader except an optional one-line HTML
comment, which neither mdBook nor GitHub renders.

````markdown
Complete example: route every request to a fake `claude`, which publishes
`continue`.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";
export const policy = definePolicy({ /* … */ });
```

**`bin/claude`**

```sh
#!/bin/sh
ravel end-session -- "continue"
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.ts --prompt "Review the change."
```

**Standard output, without a final newline**

```text
continue
```

**Exit status**

```text
0
```
````

The same Markdown is what GitHub, an editor and mdBook show, and it is what the
checker reads. A runnable `.md` page needs no build step.

## The grammar

An example starts at a paragraph whose text begins `Complete example` and ends
at the next heading or the next `Complete example` or `Fragment` paragraph.
Inside it, **every fenced block is directly preceded by a label paragraph**, a
paragraph that is one line of bold text. HTML comments between the label and the
block are ignored. Any other paragraph between them removes the label, and the
block is then an error. Inside an example the language tag is for the reader
and the highlighter and is not read.

| Label | The block is | Required |
|---|---|---|
| ``**`path`**`` | a file written to that relative path in the example's working directory before any command runs. Paths under `bin/` are made executable. | no |
| `**Command**` | a shell script: the first line begins `$ ` and each later line beginning `$ ` starts another command; a line without the prefix continues the previous one. The blocks that follow describe the script as a whole: its standard output is every command's, and its exit status is that of the last command (`sh -c` without `-e`). Steps whose own status matters are separate Command blocks. | one or more |
| `**Standard output**` | the command's standard output, including a final newline | each command |
| `**Standard output, without a final newline**` | the same, for a stream that does not end in a newline (an opaque result is such a stream) | (one of the two) |
| `**Standard error**` | the whole of standard error, including a final newline | no |
| `**Standard error, without a final newline**` | the same, without the final newline | no |
| `**Standard error includes**` | text that appears contiguously somewhere in standard error | no |
| `**Exit status**` | one integer. A command that ends from a signal has status 128 plus the signal, as a shell reports it (SIGINT is 130, SIGTERM 143) | each command |

- Each result block appears at most once per command, and standard error has one
  claim (exact, exact without a final newline, or includes). A repeat fails the
  page, because the later block would silently replace the earlier. An
  `includes` block must not be empty.
- A command with no standard output block fails the page: an empty block
  (nothing between the fences) claims that the stream is empty.
- A command with no standard error block claims nothing about standard error.
  This is the one place the checker does not compare a stream, and it is stated
  here and by the label the author did not write.
- Blocks after a `**Command**` belong to that command. A second command in the
  same example repeats its own blocks. Files may not follow a command.
- A file path is plain and relative: letters, digits, `.`, `_`, `-` and `/`
  only, no leading `/`, no `..` part, no empty part, and no path given twice.
- A page uses LF line endings. A CRLF page is refused rather than compared, because
  a stream is compared byte for byte.

A **fragment** is a paragraph beginning `Fragment` that says what the block
omits, directly followed by one block. It is not run, and it covers that block
only: a second block, or any paragraph between the label and its block, leaves
the next block unlabelled.

The check **fails closed** for blocks outside an example. A fenced block with no
language tag is an error, and so is one in any language not on the checker's list
of languages that are never examples (`text`, `rust`, `toml`, `yaml`, `yml`,
`markdown`, `md`, `diff`, `ini`, `xml`, `html`, `ruby`, `python`, `plantuml`,
`d2`; the list is in `scripts/examples-extract.awk`, and a chapter that needs
another adds it there). The match ignores case. So an unlabelled `sh`, `console`,
`shell-session` or `ts` block is caught, and so is one nobody thought to list.
Two shapes that Markdown would show but the checker cannot read are errors for
the same reason: a fence opened on a list-item line (`- ```sh`), and an indented
code block that begins `$ `. A fence indented four or more spaces is an error too;
unindent it or move it out of the list. Fences may be backticks or tildes, of any
length of three or more, and an example may sit in a blockquote.

## What runs

For each example the checker makes a fresh scratch directory holding `home`,
`work`, `tmp` and a directory with one symlink `ravel` to the built front, then
runs each command with `/bin/sh -c` in `work`, with standard input from
`/dev/null`, under `env -i`:

| | Value | The same as |
|---|---|---|
| `HOME` | `<scratch>/home`, empty | `Fixture::command` in `crates/ravel/tests/cli.rs` |
| `TMPDIR` | `<scratch>/tmp` | the same |
| `PATH` | `<scratch>/ravel:/usr/bin:/bin` | the same, plus the directory holding `ravel` |
| current directory | `<scratch>/work` | the same |
| everything else | absent | `env_clear()` |

So the examples write `PATH="$PWD/bin:$PATH" ravel …` themselves, and what a
reader sees is what ran. No real agent program is reachable (a route to `claude`
or `codex` finds only the fake the example supplies), no personal policy or
settings are read, and no real model is contacted. Real TypeScript policy
evaluation runs because the built front pairs with its compiled worker exactly
as an installation does.

A command runs in a process group of its own for at most 60 seconds. One that
runs longer has the group killed and fails whatever the page claims: the checker
records the timeout itself, so a claimed status cannot excuse it. The commands
of an example run in order, one scratch directory shared.

## The one normalisation

The scratch directory path is different every run, so **every occurrence of it
in standard output and standard error is replaced with `<scratch>` before the
comparison**. A claimed block therefore reads `<scratch>/work/policy.ts` where
the real path is `/private/var/folders/…/work/policy.ts`. The rule is not
one-to-one: a command that prints the literal `<scratch>` satisfies a claim
written with it, so an example that means to show a path does not rely on the
substitution to prove one. Nothing else is normalised: a process id, a timestamp or an elapsed time in a stream is not
replaced, so an example that would print one does not claim that stream
(`Standard error includes` the stable part, or no standard error block).

Everything else is a byte-for-byte comparison.

## Skipped, never passed

`<!-- example: requires NAME -->` anywhere within the example makes it run only
where the platform has the feature. The summary line reports
`passed`, `failed` and `skipped` separately, and a skip prints its reason.

| Name | Present when |
|---|---|
| `confinement` | macOS has `/usr/bin/sandbox-exec`; Linux has `bwrap` on `PATH` |

An unknown name fails the page, so a misspelt requirement cannot skip silently.

## Cancellation by a signal

A cancellation that the example itself causes runs under the seam without a
terminal, and gives the same result every time:

- **During supervision**, the fake agent sends its parent, the front, a signal:
  `kill -INT "$PPID"` in the fake. The front reports `session_cancelled` and
  ends by re-raising the signal, so the command's status is 128 plus its
  number. The refusal's first two lines are stable. The front's `stop attempt`
  lines before them are not: they carry a process-group id, and under one
  sandboxed tool session they also ended `Operation not permitted` (the cause
  was not established). The example therefore claims the refusal with
  `Standard error includes`, never the whole stream.
- **During selection**, the policy sends `process.kill(process.ppid, "SIGINT")`.
  The status is 130 and the first line (`selection_cancelled`) is stable. The
  worker's reap detail below it differs between runs (`signal: 15` or `exit
  status: 0`), so only that line is claimed.

A cancellation that needs a terminal (Ctrl-C typed at the keyboard, job-control
stop and continue, a closed terminal) is **not** run by the seam, and a chapter
shows it as a `Fragment` that says the terminal is what it omits.

## Real agent routes

A route to Claude Code or Codex is shown as the policy fragment that selects it
and is validated by an example whose `bin/claude` or `bin/codex` is a fake
executable of the same name that records its arguments. The chapter that shows
the route records the agent's version and the date its flags were checked
against that agent's own `--help`; the checker does not check that claim and it
is written next to the route, where a reader sees its age.

## Running and checking the checker

| Command | What it does |
|---|---|
| `task docs:examples` | builds the front and worker, then runs every complete example under `docs/book/src/` |
| `task docs:examples:test` | the controls: every refusal below is made to fire against a wrong fixture page, and the good pages pass |
| `task docs:examples:mutants` | removes one safeguard at a time from a copy of the checker and shows the control that notices; it first runs those controls against the unmutated checker, so none can "kill" a mutant by already failing |

The checker is `scripts/examples.sh` with its extractor
`scripts/examples-extract.awk`; the fixture pages are in
`scripts/examples-fixtures/pages/`, outside the book. **`task check` does not run
any of it**, so an edit to a chapter cannot turn `task check` red; the new
scripts pass its shellcheck stage. A drafting unit runs `task docs:examples`
itself, as part of its own proof.

### Why a shell script and not nextest

The seam's neighbours are Rust test cases, and a nextest case would reuse
`Fixture` directly. They were not chosen for three reasons. A nextest run is
`task test` and `task check`, so a chapter edit would turn them red unless the
cases were ignored by default and run by name, which hides them. The examples are
discovered from Markdown at run time, and a nextest case is a function the
compiler knows, so each would need one generated function or one case that
loops, whose failures nextest cannot name. And the figure and book checks beside
this one are scripts under `scripts/` with a `task` wrapper, so a contributor
meets the same shape in all three. The cost is that the seam is written twice,
in `cli.rs` and in `examples.sh`; a control reads `cli.rs` and fails when the
`Fixture`'s environment no longer matches the one in the script.

### What the controls show

Each is seen to fire, and the first line of the failure it names is the reason:

- a changed expected standard output, a changed exit status, a command with no
  standard output block, a command with no exit status block;
- a fake program the page does not supply, with a program of the same name on
  the caller's `PATH` that records being run and must not be;
- a front, and separately a worker, built from different worker source than the
  checkout's, and a pair with today's digest but older than the Rust and worker
  sources (all refused before any example runs, naming which). The digest covers
  the worker's TypeScript; the modification-time check covers a front built before
  a Rust edit. Touching a source file makes the check refuse, which is the safe
  direction;
- a runnable, unknown-language or untagged block outside any example, an
  indented transcript, a fence on a list-item line, a CRLF page, a block with no
  label inside an example, prose between a label and its block, a fragment label
  followed by two blocks, an unknown label, a repeated result block, an empty
  `includes`, and a file label holding a shell command (refused, and the command
  does not run);
- the shapes an example may take (a blockquote with a claimed line that itself
  starts with `>`, tilde fences, a four-backtick fence around a three-backtick
  line) read correctly, and eleven commands run in order;
- a changed standard error, a standard error claim that omits a trailing line
  (exact means the whole stream), an `includes` claim the stream does not
  contain, a newline claim that is wrong for the stream, a raw path where the
  page should have written `<scratch>`;
- a skipped example counted as skipped and not passed, an unknown requirement, a
  file after a command, a path that escapes the directory, a path given twice, a
  command that runs past the timeout;
- the seam's own `HOME`, `TMPDIR` and `PATH` printed from inside an example, the
  caller's variable and standard input absent;
- a caller whose `HOME` holds an invalid `settings.json` and whose `PATH` leads
  with a `claude`, a `codex` and a `ravel` that record being run: the examples
  still pass and none of the three runs; and the same with an empty `HOME` and a
  `PATH` of the fakes plus `/usr/bin:/bin` only;
- a cancellation by signal that passes five runs in a row, and its signal number
  claimed wrongly.
