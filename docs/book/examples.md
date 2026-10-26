# Examples: convention, seam and checks

This note defines the Markdown form of a complete example and how the checker runs it.
The contract requires every complete example to run.
This note supplies the mechanics for authors and the checker.
It sits beside the contract, outside the published book.

## What a reader sees

A complete example has a label paragraph and labelled code blocks.
An optional HTML comment carries a platform requirement that rendered views hide.

````markdown
Complete example: route every request to a fake `claude`, which sends
`continue`.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "convention-example";
fn select(request, context) {
    #{ status: "selected", program: "claude", args: [],
       provider: "local", model: "fake", effort: "none", reason: "example" }
}
```

**`bin/claude`**

```sh
#!/bin/sh
ravel end-session -- "continue"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/claude || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.rhai --prompt "Review the change."
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

GitHub, an editor and mdBook show the same Markdown that the checker reads.
A runnable Markdown page needs no build step.

## The grammar

An example starts at a paragraph whose text starts with `Complete example`.
It ends at the next heading or the next `Complete example` or `Fragment` paragraph.
Each fenced block inside it directly follows a label paragraph: one line of bold text.
HTML comments between the label and block have no effect.
Any other intervening paragraph removes the label, so the checker refuses that block.

The language tag serves the reader and highlighter. The example runner does not read it.

| Label | Meaning | Requirement |
|---|---|---|
| ``**`path`**`` | Write a file at that relative path before any command runs. Make paths under `bin/` executable. | Optional. |
| `**Command**` | Run a shell script. The first line starts with `$ `. Each later `$ ` starts another command. Other lines continue the previous command. | At least one. |
| `**Standard output**` | Compare stdout, with a final newline. | One stdout label per command. |
| `**Standard output, without a final newline**` | Compare stdout without a final newline. | Alternative stdout label. |
| `**Standard error**` | Compare the whole stderr stream, with a final newline. | Optional. |
| `**Standard error, without a final newline**` | Compare the whole stderr stream without a final newline. | Optional. |
| `**Standard error includes**` | Require nonempty text that occurs contiguously in stderr. | Optional. |
| `**Exit status**` | Compare one integer. A signal gives 128 plus its number, as the shell reports it. INT gives 130, and TERM gives 143. | One per command. |

A Command block runs with `sh -c`, without `-e`.
Its output includes every command, and its status is the last command's status.
Use separate Command blocks when each status matters.
Result blocks describe the whole preceding Command block.
A second Command block has its own result blocks.

The checker enforces these rules:

- Each result label occurs at most once per command. Stderr has at most one claim, exact or partial.
- A missing stdout block fails. An empty block claims an empty stream.
- A missing exit-status block fails.
- A missing stderr block makes no claim about stderr. Only this omission leaves a stream without a comparison.
- Files precede every command. No file follows a Command block.
- File paths use letters, digits, `.`, `_`, `-` and `/`. They are relative, with no `..` component or empty component.
- Each file path occurs once. A label cannot contain a shell command.
- Pages use LF line endings. The checker refuses CRLF because it compares bytes.

A fragment starts with `Fragment` and states what it omits.
Its paragraph directly precedes one block and covers only that block.
A second block, or a paragraph between the label and block, leaves the next block unlabelled.

An excerpt starts with `Excerpt from` and a source path.
It directly precedes one block of whole source lines and covers only that block.
The example extractor recognises it but neither runs nor compares it.
`task docs:excerpts` compares the source lines. `excerpts.md` defines that convention.

Outside examples, fragments and excerpts, the checker refuses an untagged fence.
It also refuses a language outside its list of languages that never identify examples.
That list is in `scripts/examples-extract.awk`:
`text`, `yaml`, `yml`, `markdown`, `md`, `diff`, `ini`, `xml`, `html`, `ruby` and `python`.
The comparison ignores case. A chapter that needs another language adds it there.

Thus, unlabelled `sh`, `console`, `shell-session` and `ts` blocks fail, as do unknown language tags.
`rust` and `toml` also need a label.
They can quote Ravel source, so the check must not silently ignore them.
The excerpt note explains that rule.

The checker also refuses fences on list-item lines and fences indented at least four spaces.
An indented code block that starts with `$ ` fails too.
Move such blocks outside the list or remove the indentation.
Fences can use backticks or tildes, with at least three characters.
An example can appear in a blockquote.

## What runs

For each example, the checker creates a fresh scratch directory with `home`, `work` and `tmp`.
It also creates a directory with a `ravel` symlink to the built Ravel CLI.
Each command runs in `work` through `/bin/sh -c`, with null stdin and `env -i`.

| Input | Value | Corresponding test input |
|---|---|---|
| `HOME` | `<scratch>/home`, empty | `Fixture::command` in `crates/ravel/tests/cli.rs`. |
| `TMPDIR` | `<scratch>/tmp` | The same fixture. |
| `PATH` | `<scratch>/ravel:/usr/bin:/bin` | The fixture, plus the directory with `ravel`. |
| Working directory | `<scratch>/work` | The same fixture. |
| Other variables | Absent | `env_clear()`. |

Examples supply their own `PATH="$PWD/bin:$PATH" ravel ...` commands.
The reader sees the commands that run.
A route to `claude` or `codex` reaches only the fake that the example supplies.
No personal policy or settings enter the run, and no real model receives a call.
Real Rhai policy evaluation runs in the built Ravel binary, re-executed as its internal worker.

The checkout still builds the separate worker until the format-removal increment.
That build dependency does not make the Rhai examples use the old worker.

Each command runs in its own process group for at most 60 seconds.
After that time, the checker kills the group and records a failure.
An expected status cannot excuse this timeout.
Commands run in order and share the example's scratch directory.

## The one normalisation

The checker replaces every scratch-directory path in stdout and stderr with `<scratch>` before comparison.
A claimed path therefore reads `<scratch>/work/policy.rhai` instead of the temporary path of that run.

This substitution is not one-to-one.
A command that prints literal `<scratch>` satisfies a claim that contains those characters.
An example therefore cannot use this substitution alone to prove a path.
The checker changes nothing else in either stream.

Process IDs, timestamps and elapsed times remain unchanged.
An example that prints them claims only stable stderr text with `Standard error includes`, or leaves stderr without a claim.
Every other comparison is byte for byte.

## Skipped, never passed

The comment `<!-- example: requires NAME -->` restricts an example to platforms with that feature.
It can appear anywhere inside the example.
The summary reports passed, failed and skipped counts separately.
A skip prints its reason.

| Name | Required platform feature |
|---|---|
| `confinement` | `/usr/bin/sandbox-exec` on macOS, or `bwrap` on Linux's `PATH`. |

An unknown requirement fails. A misspelt requirement cannot silently skip a test.

## Cancellation by a signal

An example can cause cancellation without a terminal.
During supervision, the fake agent sends INT to its parent, the Ravel CLI, with `kill -INT "$PPID"`.
Ravel reports `session_cancelled` and raises the signal again.
The shell therefore reports 128 plus the signal number.

The refusal's first two lines are stable.
Earlier stop-attempt lines contain a process-group ID and can vary.
One sandboxed tool session also reported `Operation not permitted`, whose cause was not established.
The example therefore uses `Standard error includes` for the refusal, not an exact stderr claim.

During selection, the policy calls a configured fake through `ravel::run_harness`.
The fake sends `kill -INT "$PPID"` to its parent, the Ravel CLI.
The policy itself cannot send a signal.
Ravel stops the harness run before the worker and reports `selection_cancelled`, status 130.
The example claims only the stable diagnostic line, because teardown details vary.

Terminal-dependent cancellation does not run through this seam.
Examples include keyboard Ctrl-C, job-control stop and continue, and a closed terminal.
A chapter uses a Fragment label that identifies the omitted terminal interaction.

## Real agent routes

A policy fragment selects Claude Code or Codex.
A complete example checks that route through a fake `bin/claude` or `bin/codex` that records its arguments.
The chapter records the real agent's version and the date of the check against its own help.
Those facts appear beside the route, so readers can assess their age.
The example checker does not establish them.

## Run the checker and its controls

| Command | Purpose |
|---|---|
| `task docs:examples` | Build the CLI and worker, then run every complete example in `docs/book/src/`. |
| `task docs:examples:test` | Pass correct fixtures and show each safeguard refuse a wrong fixture. |
| `task docs:examples:mutants` | Remove each safeguard in turn from a checker copy and show which control finds the omission. First pass the controls against the unchanged checker. |

The checker consists of `scripts/examples.sh` and `scripts/examples-extract.awk`.
Its fixtures live in `scripts/examples-fixtures/pages/`, outside the book.
`task check` does not run these examples, though its shellcheck stage checks their scripts.
A draft runs `task docs:examples` as its own verification.

### Why a shell script

A nextest case could reuse `Fixture` from the CLI tests.
The checker instead uses shell for three reasons:

- Nextest cases enter `task test` and `task check`. A chapter edit would affect those tasks unless the cases were ignored by default.
- Examples come from Markdown at runtime. Nextest knows compiled test functions, so it needs generated cases or one loop with less precise failure names.
- Book and figure checks already use scripts with Taskfile wrappers. The example checker follows that arrangement.

The cost is a duplicate environment definition in `cli.rs` and `examples.sh`.
A control reads `cli.rs` and fails when the fixture environment differs from the script's environment.

### What the controls show

The controls exercise these conditions and report the first failure line:

- Changed expected stdout or exit status, missing stdout or exit status.
- A fake program absent from the example. A caller-PATH program of the same name records any accidental execution.
- A CLI or worker built from different worker source. Another control uses a matching digest with a build older than Rust or worker sources.
- A digest covers the worker's TypeScript. A modification-time check finds a CLI built before a Rust edit. A source timestamp change causes refusal.
- An unlabelled executable, unknown-language, Rust, TOML or untagged block.
- An indented transcript, a fence on a list-item line, or a CRLF page.
- An unlabelled block inside an example, intervening prose, or a fragment label followed by two blocks.
- An unknown label, a repeated result block, an empty partial claim, or a file label with a shell command.
- Correct blockquotes, including a claimed line that starts with `>`, tilde fences, and four-backtick fences around three-backtick lines.
- Eleven commands that run in order.
- Changed stderr, an omitted trailing line in an exact claim, or absent text in a partial claim.
- Incorrect final-newline claims or an unnormalised path in an expected stream.
- A page with an example, fragment and exact TypeScript excerpt. Both checkers pass it, and the example checker still refuses an unlabelled block after the excerpt.
- A skipped example, an unknown requirement, a file after a command, an escaped or repeated path, and a command timeout.
- The fixture's `HOME`, `TMPDIR` and `PATH`, with the caller's variables and stdin absent.
- A caller with invalid personal settings and recording fakes for `claude`, `codex` and `ravel`. The examples pass, and no caller fake runs.
- The same isolation with an empty caller HOME and a PATH with only those fakes and system commands.
- Signal cancellation that passes five consecutive runs, and a wrong expected signal number that fails.
