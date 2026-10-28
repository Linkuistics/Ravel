# Reference

Do you have a question about one option, one exit status, one line on
standard error or one file? This chapter answers it in one entry. You can come
here from any page of the book, or directly from an error message. You do not
have to read the earlier chapters first.

Each entry explains its terms and links to the chapter that teaches the subject.
The entries repeat short explanations so that each entry makes sense alone.

Many entries use the code review of
[chapter 1](01-what-ravel-is-for.md#follow-one-code-review-from-request-to-result)
as their example. An orchestrator, a program that runs LLM work as steps of
its own work, wants an LLM to review the last commit. It writes a prompt file,
`review-prompt.txt`, and runs this command:

Fragment: the request of chapter 1's review. This block omits the code of the
orchestrator that runs the command.

```sh
ravel start-session --kind review --prompt-file review-prompt.txt
```

The prompt tells the LLM to run `ravel end-session approved` if it finds no
problem. If it finds a problem, the LLM writes `review-notes.md` and runs
`ravel end-session changes-needed`.

Chapter 1 shows a real run of this request. Its short configuration script selects a fake agent in place of
Claude Code. The fake agent wrote `review-notes.md` and sent
`changes-needed`, as the LLM does when it finds a problem. Ravel wrote
`changes-needed` on standard output, and `ravel start-session` exited with
status 0. The orchestrator gives the two
values their meaning. Ravel does not.

First, the words that this chapter uses on every page. The owner's
configuration has two parts, the configuration script and the settings. The
last column connects each word to the review:

| Word | What it names | In the review |
|---|---|---|
| [Session](02-actors-and-words.md#caller-and-owner) | One run of `ravel start-session`, from the request to the result | The review |
| [Caller](02-actors-and-words.md#caller-and-owner) | The person or program that runs that command. The caller gives a prompt and a kind. | The orchestrator |
| [Kind](02-actors-and-words.md#caller-and-owner) | A short token that names the sort of work | `review` |
| [Owner](02-actors-and-words.md#caller-and-owner) | The person who controls the configuration of Ravel for the account. The owner can be you. | The person who chose a harness for `review` |
| [Configuration script](02-actors-and-words.md#caller-and-owner) | A file of Rhai code that the owner writes. Ravel gives the request to the script. The script selects a program and its arguments, or it refuses the request. | The script that selected the harness for `review` |
| Settings | The values in the JSON file `settings.json`, such as limits and the harnesses that the script can run | The default values |
| [Rhai worker](02-actors-and-words.md#evaluation-and-supervision) | The second copy of the `ravel` executable that evaluates the configuration script. Each session has a new worker, and it ends before Ravel launches the selected program. | The process that ran the script for `review` |
| [Managed process](02-actors-and-words.md#the-managed-process-and-its-result) | The program that the configuration script selected. It is usually a [harness](02-actors-and-words.md#the-managed-process-and-its-result), such as Claude Code or Codex, the program in which an LLM works on the prompt. | Claude Code, or a fake agent in the book's run |
| [Result](02-actors-and-words.md#the-managed-process-and-its-result) | The text that a program in the session sends with `ravel end-session`. Ravel does not interpret it. Thus, the book calls it an opaque result. | `changes-needed` |
| [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) | The `ravel` command-line program. It evaluates the configuration script in the Rhai worker, launches the selected program and returns the result. | The program that ran the review and returned `changes-needed` |

Ravel uses `config` in file names, options and diagnostic codes, such as
`config.rhai`, `--config-env` and `config_refused`. Its diagnostic prose uses
“configuration script”.

Why does Ravel evaluate the script in a separate process? The Ravel CLI
launches a second copy of its own executable as a child process, the Rhai
worker. The parent stays in control while the script runs:

- **The deadline.** Each evaluation has a deadline. When it expires, the
  parent ends evaluation and can signal the worker process. Rhai cannot
  interrupt some of its own functions, such as `sleep`. A kernel stall can
  delay the worker's exit.
- **Requests from the script.** The worker only computes. When the script
  reads a file or runs a harness, the worker asks the parent. The parent does
  the work and sends the data back.
- **Isolation of a failure.** If the script exhausts the memory of the worker
  or overflows its stack, only the worker stops. The parent continues,
  reports the failure to the caller and launches no program.

The worker does not run at the same time as the selected program. Ravel waits
until the worker exits and collects its exit status. Only then does Ravel
launch the program that the script selected.

What is Rhai? Rhai is a small computer language for programs that are written
in Rust. Its engine is part of the `ravel` executable, so you install no
separate language runtime. Rhai looks like a mix of JavaScript and Rust, and
its values have dynamic types.

A Rhai script changes things outside itself only through functions that the host program gives it. In Ravel, these are the
`ravel::` functions. Some are [host capabilities](#host-capabilities), which
ask the Ravel CLI to do work. Other helpers run in the worker.

The [Rhai Book](https://rhai.rs/book/) documents the language, and its
[language reference](https://rhai.rs/book/ref/) lists each statement and
operator. The [online playground](https://rhai.rs/playground/stable/) runs
Rhai code in your browser. It does not have the `ravel::` functions or Ravel's
limits. [Chapter 4](04-write-a-configuration-script.md#know-the-rhai-language)
teaches the Rhai that a configuration script needs.

Which entry answers your question? Use the table, or the headings of this
chapter:

| Your question | Entry |
|---|---|
| Which options does `ravel start-session` accept? | [`start-session`](#start-session) and the entries after it |
| How does a program send its result? | [`end-session RESULT`](#end-session-result) |
| What does an exit status mean? | [Exit statuses](#exit-statuses) |
| What does a line on standard error mean? | [Streams and notices](#streams-and-notices) |
| What does a code such as `config_refused` mean? | [Diagnostic codes](#diagnostic-codes) |
| Which environment variables does Ravel read, set or remove? | [Environment variables](#environment-variables) |
| Where is the configuration script, and which settings exist? | [Configuration script and settings](#configuration-script-and-settings) |
| Which limits apply to evaluation? | [Bounds](#bounds) and [Engine and conversion bounds](#engine-and-conversion-bounds) |
| What does the configuration script receive and return? | [Configuration script and request](#configuration-script-and-request) and [Selection and refusal](#selection-and-refusal) |
| Which `ravel::` functions can the configuration script call? | [Selection context and host capabilities](#selection-context-and-host-capabilities) |
| Which example configuration scripts does Ravel install? | [Example configuration scripts](#example-configuration-scripts) |
| Where are the installed files? | [Installed layout](#installed-layout) |

The entries describe the version that this book names, installed with
[Homebrew](03-a-complete-session.md#install-ravel-and-check-its-version).

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

## Command line

### `ravel`, help and version

Do you want to know which verbs Ravel has, or which version you installed?
Ask the Ravel CLI. `ravel` takes one of two verbs: `start-session` or
`end-session`. It has no `help` subcommand. The table gives the help and
version requests.

| Request | Output | Status |
|---|---|---|
| `ravel` without a verb | Help on standard error. | `2` |
| `ravel --help`, `ravel start-session --help`, `ravel end-session --help` | Help on standard output. `-h` is the short option on all three command lines. | `0` |
| `ravel --version`, `ravel -V` | The package version of the [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) on standard output. The version option belongs only to the top-level command. | `0` |

Fragment: the output of `ravel --help` from the checkout build on 2026-10-29.
This block omits the command and its status, 0.

```text
Route and supervise a session, returning its opaque result

Usage: ravel <COMMAND>

Commands:
  start-session  Route the prompt unchanged through an owner Rhai configuration script and supervise its selected process
  end-session    Send exactly one opaque result to Ravel and ask Ravel to end the enclosing session

Options:
  -h, --help     Print help
  -V, --version  Print version

Examples:
  ravel start-session --kind feature --prompt 'Implement the parser; finish with ravel end-session done'
  ravel end-session done
```

Help and version text is informational output, never an opaque result.
See [the first complete session](03-a-complete-session.md#run-the-first-session)
for the two verbs in use.

After `--`, Ravel reads no more options. For `end-session`, put `--` before a
result that looks like an option. Inside a live session,
`ravel end-session -- '--help'` sends the literal value `--help`. See
[result arguments](#end-session-result).

### `start-session`

<!-- tc: depart synonym-rotation because launch refers to process creation, while start also denotes the beginning of a deadline -->
You use `ravel start-session` when you want a harness, or another program, to
do one piece of work and give your program one result.
`ravel start-session --kind TEXT` also needs exactly one of `--prompt TEXT`
and `--prompt-file PATH`.

What happens after you run the command? The table follows one successful
session from your command to your result:

| Order | Who acts | What happens |
|---|---|---|
| 1 | The caller | Runs `ravel start-session` with a kind, a prompt and optional caller data |
| 2 | The Ravel CLI | Reads the settings, checks the file of the configuration script and launches a second copy of its own executable as the Rhai worker |
| 3 | The configuration script | Runs in the Rhai worker and returns a selection: a program, its arguments and labels |
| 4 | The Ravel CLI | Reaps the worker, writes the text routing notice on the caller's standard error and launches the selected program. With `--json`, the notice follows the supervision attempt. |
| 5 | The managed process | Does the work, then runs `ravel end-session RESULT` |
| 6 | The Ravel CLI | Stops the program and the rest of its process group, completes cleanup and writes the result on standard output |

To **reap** a process is to collect its exit status from the operating system.
After the reap, the operating system can use the process ID again. Ravel
launches the managed process in a **process group** of its own. Each child of
the managed process is in that group too, unless the child leaves it. The
[routing notice](#standard-streams) is the line that tells the caller which
harness, model and effort the configuration script chose.

At row 3, the configuration script can also refuse. After a
[refusal](02-actors-and-words.md#evaluation-and-supervision), Ravel
launches no final managed process and tries no alternative route. Before it
returns, the script can run harnesses of its own. These
[harness runs](#harness-run-request) return their output to the script, not to
the caller.

<!-- tc: depart synonym-rotation because confirmed absence is a successful cleanup observation, not just an input check -->
At row 6, Ravel writes the sent result unchanged to standard output. This write occurs only after the reap, the confirmed
absence of the process group and the required cleanup. Ravel does not
interpret the result. Then `ravel start-session` exits with status 0.

Fragment: the first command of the example `notices` in chapter 5, “Read
notices separately from results”, and its standard error. That example runs
the review of chapter 1 in the directory `~/ravel-chapter-5/notices`. This
block omits the configuration script, the prompt file `review-prompt.txt`,
the fake agent `review-agent` and the environment of the command. Chapter 5
runs this session as a complete example, and the book's example check
compares these two lines.

```console
$ ravel start-session --kind review --config config.rhai --prompt-file review-prompt.txt
ravel: starting provider local, model fake, effort low for kind "review"
review-agent: the review is complete
```

The first line is the routing notice of the Ravel CLI. The second line comes
from the fake agent. The fake agent then ran `ravel end-session -- "changes-needed"`.
Thus, standard output held `changes-needed`, with no added newline, and the status
was 0. See [run a complete session](03-a-complete-session.md#run-the-first-session)
and [interpret the outcome](05-running-sessions.md#interpret-the-exit-status).

All fifteen options below belong after `start-session`. Only `--param`,
`--config-env` and `--runtime-read` accept more than one occurrence. A
repeated single-value option or switch is a command-line error. `-h` and
`--help` are also available on this verb.

### `--kind TEXT`

<!-- tc: depart synonym-rotation because the owner's words "extend or modify the prompt" are kept -->
Why does the caller give a kind? The
[kind](02-actors-and-words.md#caller-and-owner) is a short token that tells
the configuration script what sort of work the caller wants, such as `review` or
`bugfix`. The script can use the kind to choose a harness, a model and an
effort. It can also use the kind to extend or modify the prompt.

`--kind` is required and supplies `request.kind` unchanged. Its value is
nonempty UTF-8. An absent or empty kind gives status 2.

Ravel has no catalogue of kinds and gets no kind from a task file. Ravel gives
the token no meaning. Only the configuration script and the caller do. See
[supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--prompt TEXT` and `--prompt-file PATH`

The prompt is the text of the task for the LLM. It also tells the LLM how to
end the session, for example “Then run: ravel end-session changes-needed” in
chapter 1's review. Ravel adds
no instructions to end the session. Thus, your prompt gives them.

Exactly one of these two prompt inputs is required.

| Option | Source of `request.prompt` |
|---|---|
| `--prompt TEXT` | The argument text, also when its first character is `-`. |
| `--prompt-file PATH` | One read of the file, relative to the caller's working directory. |

The prompt can be empty. It contains no NUL, is valid UTF-8 and has a maximum
of 1,048,576 bytes (1 MiB). Ravel puts its bytes unchanged in
`request.prompt`, with trailing newlines. The configuration script decides how to give that
text to its selected program. Usually the script puts it in an argument of the
harness, and the script can add text before or after it.

An unreadable prompt file reports `prompt_unreadable`. Invalid encoding, a
NUL or too many bytes reports `prompt_invalid`. Both give status 2. A command with
no prompt input, or with both prompt inputs, also exits 2. Ravel does not
implicitly read the prompt from standard input, and it refuses a prompt file that is a terminal. See [supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

> **Note:** Shell command substitution removes trailing newlines before Ravel
> receives an argument. `--prompt-file` preserves those bytes. An
> operating system can set a smaller argument limit. This limit also applies
> when the configuration script puts a prompt from a file into the arguments of the selected
> command.

### `--task-file PATH` and `--task-id ID`

<!-- tc: depart synonym-rotation because the owner's words "extend or modify the prompt" are kept -->
You may want to tell the configuration script which task the session serves. These
two options carry that caller data to the script. The script can use it to
choose a route, or to extend or modify the prompt. Ravel itself does nothing
with the values. Invalid input gives status 2.

| Option | Request field | Value and effect |
|---|---|---|
| `--task-file PATH` | `request.taskFile` | Ravel joins the caller's working directory and the given path to make an absolute UTF-8 path. It does not canonicalise or read the path. The file does not have to exist, and the path supplies no kind or task identity. |
| `--task-id ID` | `request.taskId` | Nonempty opaque UTF-8 with a maximum of 1,024 bytes. Its first character can be `-`. Ravel does not search for a task record. |

For example, `--task-id T-17` gives the script `request.taskId` with the value
`"T-17"`. Without the option, `request.taskId` is `()`, Rhai's empty value.
The [request entry](#selectionrequest) shows a real request with this value.
See
[caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) for
both fields.

### `--config PATH`

You may want to try a configuration script before you install it, or use one
script for one project. `--config` selects that script file, which the book
calls the **entry**. The path is relative to the caller's working directory.
Without the option, Ravel uses `$HOME/.config/ravel/config.rhai`, which needs
an absolute `HOME`. There is no repository search and no environment variable
that selects a different configuration script.

The entry resolves to a readable regular UTF-8 file of at most 1 MiB. Its canonical path is UTF-8. A symlink selects its
real file, and the directory of that file is the base for relative imports.

An empty option value is malformed input, status 2. If Ravel cannot accept
the entry file, the status is 3. A different entry does not move the settings
file from `$HOME/.config/ravel/settings.json`. See
[define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request)
and [import other Rhai files](04-write-a-configuration-script.md#import-other-rhai-files).

### `--param NAME=VALUE`

<!-- tc: depart synonym-rotation because the owner's words "extend or modify the prompt" are kept -->
You may want to give the configuration script a value that only your caller
knows, such as the name of a repository. The script can use the value to choose a
route, or to extend or modify the prompt. `--param` adds a string to
`request.params`, and it is repeatable with different names.

For example, `--param repo=ravel` gives the script `request.params` with the
value `#{repo: "ravel"}`. The [request entry](#selectionrequest) shows a real
request with this value. The table gives the parts of each argument and their
limit.

| Part | Rule |
|---|---|
| Name | Nonempty UTF-8 without `=`. Each name occurs only once. |
| Separator | The first `=` separates the name from the value. |
| Value | UTF-8, possibly empty or with more `=` characters. |
| Whole argument | Its first character can be `-`. |
| Combined size | All names and values together have a limit of 65,536 bytes. The separators do not count. |

A repeated name, a missing separator, invalid
encoding or a total above the limit gives status 2. Ravel gives names and values no
meaning. See
[caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--context PATH`

You may want facts about the task to change the choice of the configuration
script. For example, a large change can go to a higher effort.
**Selection context** is a JSON document of such facts, such as a summary and
acceptance criteria. The script receives it as the second argument of
`select`.

The book uses “context” for three different things. This option supplies only
the first:

| Name | What it is |
|---|---|
| Selection context | JSON data about the task, for the choice of the configuration script |
| Session-result context | The path in `RAVEL_EXIT_FILE`, through which `end-session` sends the result |
| The caller's working directory | The directory of the request, in `request.cwd` |

<!-- tc: depart synonym-rotation because validation enforces a data shape, while confirmation establishes cleanup -->
`--context` reads a regular file, relative to the caller's working directory,
as [selection context](02-actors-and-words.md#three-different-kinds-of-information).
Ravel validates it and records its canonical name, byte count and SHA-256.
The [selection-context entry](#context-and-deliveredcontext) shows a real
document as `select` receives it.

Without `load_context`, the delivered document is this data with the measured
sources attached. With a loader, the data is input to the loader, and the
loader assembles the document that `select` receives. With no caller document
and no loader, `select` receives `()` as its selection-context argument.

Ravel refuses an unreadable, invalid or oversized document with status 3.
Malformed command-line syntax still gives status 2. This option supplies
selection context, never a
[session-result context](02-actors-and-words.md#three-different-kinds-of-information). See [supply selection context](04-write-a-configuration-script.md#supply-selection-context).

### `--timeout-ms MS`

<!-- tc: depart synonym-rotation because launch names process creation, while begin names the first moment of a deadline -->
You may want more time for evaluation, for example when the configuration
script asks a model through a harness run. **Evaluation** is the work of the
Rhai worker, up to the selection or refusal. `--timeout-ms` replaces the
`timeoutMs` setting for this invocation. The table gives its syntax
and values. Invalid syntax or range gives status 2.

| Property | Value |
|---|---|
| Input syntax | Decimal digits. |
| Inclusive range | 1,000–600,000 milliseconds. |
| Default, with no flag or setting | 30,000 milliseconds. |

Ravel validates the settings before the flag replaces a value. Thus, an
invalid setting fails even with a valid flag.

<!-- tc: depart synonym-rotation because launch names process creation, while begin names the first moment of a deadline -->
The deadline begins when the Ravel CLI spawns the [Rhai worker](02-actors-and-words.md#evaluation-and-supervision).
It covers the worker startup, the import of the configuration script, the
loader of the selection context and the selection, with every host call. At expiry, Ravel stops any evaluation
harness run, then the worker. It reports `selection_timeout`, status 124,
unless cancellation or failed harness cleanup takes precedence. Cleanup time is additional.

The deadline does not limit the time that the managed process
works. See
[set bounds and environment grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

### `--context-bytes BYTES`

You may want a larger budget when the selection context holds a long
task description. `--context-bytes` replaces the `contextBytes` setting.
It takes decimal digits in the inclusive range 1–8,388,608 bytes. With no flag
and no setting, the budget is 262,144 bytes.

Invalid syntax or range gives status 2. Ravel validates the settings first.
Thus, an invalid setting fails even with a valid flag.

The budget limits the caller document and the encoded selection context that
selection receives, with the measured source records. It is also the maximum
for a host source read. Ravel refuses data above a bound, and it does not
truncate that data. `context_too_large` uses status 3. See [selection context](04-write-a-configuration-script.md#supply-selection-context)
and [bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

### `--config-env NAME`

You may want the configuration script to read a variable of the caller, for
example a mode that selects a different effort. `--config-env` grants the
script one such variable. Repeat the option to grant more names.

Why does the script need a grant? The Rhai worker has an empty environment.
When the script calls `ravel::env(name)`, the worker sends the request to the
Ravel CLI over a private socket. The Ravel CLI checks the name against the base
names and the grants. Then it answers with the value from the caller's
environment, or with `()` when the variable is unset.

For example, chapter 4 grants `BOOK_EFFORT`, and the caller sets
`BOOK_EFFORT=high`. Then `ravel::env("BOOK_EFFORT")` returns `"high"`. A name
without a grant throws `env_not_granted`, also when the caller sets it. See
[the variables that the script can read](#variables-that-ravel-and-the-configuration-script-read)
for the base names.

| Grant property | Behaviour |
|---|---|
| Settings | The flags add to the `configEnv` array of the settings. |
| Duplicate names | Ravel merges them into one grant, in the order of their first occurrence. |
| Value | A grant carries the caller's current value. An unset variable returns `()` through the helper. |
| Recipient | The configuration script reads the value through `ravel::env`. This option changes no managed-process or harness-run environment. |

A name is nonempty UTF-8 and contains no `=`. It cannot match a
[reserved grant class](#names-that-an-environment-grant-cannot-use). A malformed name reports
`malformed_input`, and a reserved name reports `excluded_grant`. Both give
status 2.
Grant diagnostics show each name and whether it is set, never its value.
See [set bounds and environment grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

### `--confine`

<!-- tc: depart synonym-rotation because launch identifies process creation and start also describes the beginning of evaluation -->

You may want to limit what the managed process can change on your computer.
For example, an LLM that edits files in one project needs no write access to
your other files. `--confine` restricts the filesystem access and the
environment of the managed process and each evaluation harness run.
Confinement is off when the flag is absent.

Ravel uses one sandbox tool on each operating system:

- **macOS: `/usr/bin/sandbox-exec`.** This system command runs a program
  inside a sandbox that a profile describes. Ravel gives it a profile that
  refuses access by default and then permits the grants below. Its manual
  page, `man sandbox-exec`, marks the command as deprecated.
- **Linux: bubblewrap, at `/usr/bin/bwrap` or `/bin/bwrap`.** This tool runs a
  program in new Linux namespaces. Ravel runs it with an empty filesystem
  view and adds only the grants below.
  [The bubblewrap project page](https://github.com/containers/bubblewrap)
  describes the tool.

<!-- tc: depart synonym-rotation because launch identifies process creation and start also describes the beginning of evaluation -->
The configuration script and the Rhai worker
stay outside [confinement](02-actors-and-words.md#trust-and-confinement).
Ravel tests the backend before it evaluates the configuration script. It never uses an
unconfined launch in place of a confined launch.

The table gives the access of the confined managed process.

| Resource | Access |
|---|---|
| Canonical working directory and private run directory | Writable. |
| System runtime paths, selected executable and real executable file of the Ravel CLI | Readable. |
| Network | Permitted. |
| POSIX session and terminal | A new POSIX session, with EOF on standard input. Ravel does not lend the caller's controlling terminal. |
| Environment | Ravel restricts the [managed-process environment](#managed-process-environment) separately from the environment of the Rhai worker. |

Filesystem grants cannot overlap the directory of the configuration script or
the settings,
and this rule also applies through symlinks. An unusable backend or input
reports `confinement_unusable`, and an overlap reports `confinement_overlap`.
Both give status 2. See [confine the managed process](05-running-sessions.md#confine-the-managed-process)
for the layout of a confined launch.

> **Note:** The project measured inherited signal settings, cancellation,
> cleanup and terminal recovery on aarch64 macOS and aarch64 GNU/Linux. For
> x86_64 GNU/Linux, the project built Ravel and ran it only under emulation. Native x86-64 qualification is still open. The
> row for a different host in
> [Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup)
> gives the scope of these measurements.

### `--runtime-read FILE`

A confined harness can need one file outside its working directory, for
example a configuration file of the harness. A confined process cannot read
such a file without a grant. `--runtime-read` adds one read-only runtime file
to a confined launch, and it is repeatable.

The option needs `--confine`. Without that switch, the command exits 2.
The path resolves from the caller's working directory to a regular file that
exists, and its canonical name is UTF-8. The option grants no directory tree.

Read-only grants stay read-only inside writable directories. If a grant is
unusable or has an overlap, Ravel stops with status 2 before it
evaluates the configuration script. See
[confinement grants](05-running-sessions.md#confine-the-managed-process).

### `--exit-dir DIR`

The [result channel](02-actors-and-words.md#the-managed-process-and-its-result)
is a private temporary directory, with a path for one result file in it.
`end-session` writes the result to that file, and Ravel reads it from there.
By default, Ravel makes the channel in the system temporary directory.
`--exit-dir` chooses a different parent directory for the channel.

The parent is a writable directory that exists. Ravel resolves it from the caller's
working directory. The table shows the parent for each form of launch.

| Launch | Without `--exit-dir` | With `--exit-dir DIR` |
|---|---|---|
| Unconfined | The system temporary directory. | `DIR`. |
| Confined | The private run directory. | `DIR`, also as a writable grant. |

The option chooses a parent, not a result filename or a persistent store. An invalid
parent can report `exit_dir_unusable` with status 2. If Ravel later cannot
allocate the result channel, it reports `session_channel_unavailable` with
status 5. See
[choose a channel parent](05-running-sessions.md#confine-the-managed-process).

### `--json`

Your program may want to read the routing notice and the failures as data, not
as text. The routing notice tells the caller which harness, model and effort
the configuration script chose. A failure document tells the caller why a session failed.

`--json` replaces Ravel's text routing notice and failure document with
JSON documents on standard error. It does not change the result on standard output or the
output of the managed process. It is off when it is absent, and it belongs to
`start-session`.

The time of the notice also changes. Ravel writes the text notice before the
launch. Ravel writes the JSON notice after the supervision attempt, so the JSON
document also records what happened during the session.

Fragment: the `selection` member of the JSON routing notice of chapter 1's
review request, run again with `--json`. The script of that run selects a fake
agent, so the labels are `local`, `fake` and `low`. This block omits the other
members of the document.

```json
{"selection":{"effort":"low","model":"fake","provider":"local","reason":"Reviews go to the fake agent."}}
```

See [streams and notices](#streams-and-notices) for the fields and the order of output,
and [read notices](05-running-sessions.md#read-notices-separately-from-results)
for their use.

<!-- tc: depart synonym-rotation because display format names presentation, not an action to show data -->
> **Note:** After a successful parse of the command line, a value such as
> `--prompt '--json'` is caller data, not a switch. If the parse fails, Ravel
> searches the argument words for an exact `--json` to choose the error
> format. Help, version and the help for a missing verb keep their usual
> display format.

### `end-session RESULT`

A harness usually cannot exit by itself when its LLM finishes the task. It
waits for more input. Thus, the caller's prompt tells the LLM to run
`ravel end-session RESULT` when the work is complete. This command sends one
UTF-8 argument, the result, to the live session that encloses it. It also
tells Ravel to stop that session.

How does `end-session` find its session? It takes no session ID and no target
argument. Before the launch, Ravel makes a private
[result channel](#--exit-dir-dir) for the session. Ravel puts the path of its
result file in the `RAVEL_EXIT_FILE` environment variable of the program that
it launches. Each child process gets a copy of its parent's environment, unless
the parent changes it. The table shows the path when each parent leaves it unchanged:

<!-- tc: depart synonym-rotation because the table repeats chapter 1's words, and start names any parent's process creation while launch names Ravel's supervised launch -->
| Process | Which process starts it | Its `RAVEL_EXIT_FILE` |
|---|---|---|
| `ravel start-session` | The caller | None, unless the caller itself runs in a session |
| The harness, which is the managed process | Ravel | A new path that Ravel makes for this session |
| A shell for a command of the LLM | The harness | The same path, inherited from the harness |
| `ravel end-session changes-needed` | That shell | The same path, inherited from the shell. The command writes the result there. |

Each child inherits its parent's path unless its parent changes it.
Ravel sets a new path for the inner harness.

![A caller starts outer start-session without a result path, and its Rhai worker has no path. Ravel gives the outer harness an outer path, which its shell and end-session inherit. The shell starts inner start-session with the outer path. The inner harness receives a new inner path, which its end-session inherits. Each end-session writes to its own path. An evaluation harness run gets a separate path and returns its result to the configuration script.](figures/inherited-result-path.svg)

*Figure: The nested run gives each session's children its own result path.*

[Open the inherited result paths at full size](figures/inherited-result-path.svg).

The book calls this path the
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
The [`RAVEL_EXIT_FILE` entry](#ravel_exit_file) shows real paths, also for a
nested session. The verb accepts no settings option, and it reads no configuration
script or settings. `-h` and `--help` show the help of this verb. `--` permits a result
that looks like an option.

What happens after the send? Ravel finds the result in the channel. Ravel
then stops the harness and the rest of its process group, and completes
cleanup. Then `ravel start-session` writes the result on its standard output
and exits 0. Thus, the value that the LLM gave to `end-session` becomes the
result of `ravel start-session`.

The table gives the inputs of `end-session`. Quote `RESULT` so that it
stays one argument.

| Input | Value or effect |
|---|---|
| `RESULT` | One UTF-8 argument. It can be empty, multiline or Unicode, with a maximum of 1,048,576 bytes (1 MiB). |
| `RAVEL_EXIT_FILE` | The inherited path of the live session's private result channel. |
| `--` | Ends the options, so a value that looks like an option can follow as `RESULT`. |
| `-h`, `--help` | Shows help, without a send. |

Ravel keeps the first complete result that it receives. After that result
arrives, Ravel stops the
[harness](02-actors-and-words.md#the-managed-process-and-its-result). Thus,
a second `end-session` and that shutdown are in a race: either can occur
first. For example, another process in the group can send a result before
Ravel stops that process. Ravel keeps the first result, even if the second
command succeeds. The shutdown can stop the second command, or the removal
of the result channel can make it fail.

A successful `end-session` exits 0 and prints nothing. It means only that the channel accepted
the result, or that a valid first result was already present. It does not
mean that the job ended or that the result reached the caller. A
cancellation or a failure of required cleanup still returns no result to
the caller.

`end-session` exits 1, with a text diagnostic on standard error, if the [session-result context](02-actors-and-words.md#three-different-kinds-of-information)
is missing, empty, stale or invalid. It also exits 1 for an oversized value
or an I/O failure of the send. If the Ravel CLI finds an argument-count or encoding
error, it exits 2. The operating system can set a smaller argument limit
before the executable runs.

Fragment: standard error of `ravel end-session changes-needed` in a shell outside
any session, from the checkout build on 2026-10-29. This block omits the
command and its status, 1.

```text
ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or empty)
```

See [accept a result only after success](05-running-sessions.md#accept-a-result-only-after-success).

> **Note:** After Ravel detects the sent result, it gives a job that still
> runs 2 seconds. Then it sends TERM, waits up to 5 more seconds and uses KILL
> if necessary. If the direct child exits, Ravel stops the other group members
> before the reap, with no new grace period. Ravel can stop code that
> runs after `end-session`.

## Exit statuses

Your program ran `ravel start-session` or `ravel end-session` and received an
exit status. What does it mean, and what do you do next? Find the status in
the table below. The status is the exit status of the Ravel CLI, not the exit
status of the managed process.

For `start-session`, accept the result on standard output only when the final status
is 0. Neither the exit status of the managed process nor an earlier
successful `end-session` decides the result of the enclosing session.

Status 0 needs complete **cleanup**. In cleanup, Ravel stops the processes
that remain in the process group of the managed process. Ravel reaps the
managed process, which collects its exit status from the operating system.
Ravel then confirms that the group is gone and removes the private result
channel.

Cancellation has precedence over an original failure. Failed harness cleanup
can replace an evaluation deadline with `harness_cleanup_failed`, status 5.
After evaluation, an original failure keeps its status if later cleanup also fails.
Required cleanup failures prevent success. All applicable cleanup
reports stay as diagnostics. See
[outcome precedence](05-running-sessions.md#read-notices-separately-from-results).

| Status | Meaning and next action | Taught in |
|---|---|---|
| `0` | `start-session` wrote the sent result unchanged to standard output and flushed it. This occurs after the reap, the confirmed absence of the group and the required cleanup. An empty result is valid. A later nonzero exit of the managed process does not make the sent result invalid. Ravel does not interpret the result. Interpret it with the caller's convention. For `end-session`, status 0 confirms only that it sent the result, or that a valid first result was already present. An explicit help or version request also gives status 0. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| `1` | `start-session` reports `session_result_missing`, even when the managed process exits 0. For `end-session`, status 1 means that it could not send the result. For example, the session-result context was missing, empty or stale, or the value was too large. Check that the call occurs inside a live session. A missing result is not an empty result. | [The sent result and its absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `2` | Malformed command-line input or settings, or confinement inputs that are unusable or overlap. Without a verb, `ravel` prints help with this status. Correct the named input or setting before you try again. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |
| `3` | The configuration script refused, or Ravel could not accept its entry, behaviour, shape or selection result, or the selection context. Evaluation bounds and a `start-session` inside an evaluation harness run also use 3. No fallback route runs. Read the code, the stage and the remedy. For `config_refused`, also read `configCode`. | [Select or refuse](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `5` | A worker or supervision failed, or the result was invalid. Required cleanup or terminal recovery failed, a group survived, or the result output failed. A failed worker re-execution, identity mismatch or evaluation harness cleanup also gives 5. Retain the diagnostics, repair the installation or the named resource, and discard any partial result bytes. | [Operational diagnostics](05-running-sessions.md#read-notices-separately-from-results) <!-- tc: depart synonym-rotation because repair restores a resource, while correct also describes a valid value --> |
| `124` | The evaluation of the configuration script timed out, and Ravel launched no managed process. Examine the work of the script before you increase `timeoutMs` or `--timeout-ms`. Failed harness cleanup or cancellation can replace this outcome. The deadline does not limit the managed process. | [Bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `126` | The selected program is not executable, or the launch failed before the runner reported the start, for a reason other than ENOENT. Check the permissions, the format, the interpreter and the execution environment. Retain any recovery reports. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) <!-- tc: depart synonym-rotation because the start is the runner's named report, while launch names process creation --> |
| `127` | Ravel did not find the selected program, or the launch failed with ENOENT before the start. A script that exists can have a missing interpreter. Check `PATH`, the selected path and the interpreter. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) <!-- tc: depart synonym-rotation because the start is the runner's named report, while launch names process creation --> |
| `129` | Cancellation by HUP (signal 1), during selection or supervision. Ravel returns no result to the caller, even if it already received one. Retain the cleanup reports, and try again only if the caller still wants the work. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `130` | Cancellation by INT (signal 2), during selection or supervision. Ravel returns no result to the caller, even if it already received one. A keyboard Ctrl-C that goes only to the group of the managed process is not necessarily a cancellation of Ravel. | [Cancellation and terminal INT](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `143` | Cancellation by TERM (signal 15), during selection or supervision. Ravel returns no result to the caller, even if it already received one. Retain the cleanup reports, and try again only if the caller did not intend the termination. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |

The three signal statuses in the table follow the shell convention: 128 plus
the signal number. Ravel attempts the cleanup and then raises the same signal
again. Thus, a process API reports termination by a signal. If the
propagation fails, Ravel reports the failure and uses the numeric status in
its place. A caller that uses a process API handles both cases. If a signal
is ignored or blocked when Ravel starts, that signal does not automatically
cause a cancellation.

> **Note:** KILL or another abrupt stop can end Ravel before it decides
> the final outcome. A signal that arrives after the final decision about
> cancellation can interrupt the result write. Neither case confirms the
> cleanup or that the caller received the complete result. A failed final write can leave a prefix on standard output.
> The row for an abrupt stop in
> [Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup)
> describes the first case.
> [Interpret the exit status](05-running-sessions.md#interpret-the-exit-status)
> describes the second.

## Streams and notices

### Standard streams

Your program reads the result from one stream and the diagnostics from
another. Which stream carries what? The table gives the answer for each
standard stream of `ravel start-session`.

> **Table — Streams:** Standard output carries only the result. Standard error combines the
> managed output and the diagnostics. Standard input serves the managed
> process and does not implicitly supply a prompt.

| Stream | Session behaviour | Taught in |
|---|---|---|
| Standard output | `start-session` writes only the result of a successful session, unchanged and with no added newline. Ravel does not interpret it. An empty sent result is zero bytes. A failed final write can leave a prefix, so check the status. `end-session` writes nothing to standard output. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| Standard error | The standard output and standard error of the managed process, Ravel notices and failures, captured diagnostics of the configuration script and operational reports. Without `--json`, each runner operation report has the form `ravel: <report>`. `--json` changes only Ravel's routing notice and failure document. | [Read notices](05-running-sessions.md#read-notices-separately-from-results) and [runner operation reports](08-follow-a-session-through-the-code.md#complete-cleanup-and-write-the-result) |
| Evaluation harness output | Captured standard output, standard error and optional result return to the configuration script as data. They reach a caller stream only if the script copies them into visible output. | [Harness runs](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |
| Standard input | Ravel does not implicitly read a prompt from standard input. The worker receives a null standard input. A harness run receives its requested text or EOF. An unconfined managed process can use the caller's standard input, and a confined one receives EOF. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |

What does the configuration script give back to Ravel? It gives back a
**selection** or a refusal. A selection is the map that `select` returns when it chooses a
program. It holds `program` and `args`, the command that Ravel runs. It also
holds the labels `provider`, `model`, `effort` and `reason`. A label
describes the choice of the script, but it does not configure the harness.

The caller does not read the owner's configuration script. Thus, Ravel tells
the caller what the script chose in the **routing notice**:

- **Who writes it:** the Ravel CLI.
- **When:** in text form, after the configuration script selects a program
  and before Ravel launches it. In JSON form, after the supervision attempt.
- **Where:** the caller's standard error. Standard output carries only the
  result.
- **Who reads it:** the caller, or the person who watches the terminal.
- **Why:** this notice is the way in which Ravel tells the caller which
  harness, model and effort the configuration script chose.

Fragment: the text routing notice of chapter 1's real run of the review, as
Ravel writes it on standard error. The script of that run selects a fake
agent, not Claude Code. This block omits the other lines on standard error.

```text
ravel: starting provider local, model fake, effort low for kind "review"
```

The selection separates the command that Ravel launches from the labels that the caller reads.

![The Rhai worker returns the fake-agent command and the local, fake and low labels. The Ravel CLI reaps the worker, writes the routing notice on the caller's standard error and launches fake-agent with args. Labels do not go to the program. With JSON, the notice follows the supervision attempt.](figures/routing-notice.svg)

*Figure: The review selection reaches the program and the caller by separate paths.*

[Open the selection and routing notice at full size](figures/routing-notice.svg).

The line names the provider, model, effort and kind. These are the labels of
the selection and the caller's kind. The line does not show `program` or
`args`.

Ravel writes other lines on standard error too. Each one has a known first
part:

| First part of the line | Who writes it, and what it means |
|---|---|
| `ravel: starting provider` | The Ravel CLI. The text routing notice. |
| `config stdout:` or `config stderr:` | The Ravel CLI, with one line of output of the configuration script. `print` writes to the standard output of the script, and `debug` writes to its standard error. |
| `ravel: refused (CODE, stage STAGE):` | The Ravel CLI. A failure. The word “refused” also covers operational failures, and the code and the status show which failure it is. The [diagnostic codes](#diagnostic-codes) entry shows a real example. |
| `ravel: end-session:` | `ravel end-session`. It could not send the result. |
| `ravel: ` and a runner report | The Ravel CLI. A report of the runner about the stop of a process or the cleanup. |

Any other line comes from the managed process, or from a program that it
started. The managed process can also write a line that looks like a Ravel
line. Thus, the form of a line does not prove that Ravel wrote it. See [read notices](05-running-sessions.md#read-notices-separately-from-results).

### JSON routing notice

You may want your program to record what the configuration script chose and
what evaluation used, for each session. With `--json`, the Ravel CLI writes the routing notice
as one JSON document on the caller's standard error. The text routing notice
names only the provider, model, effort and kind. The JSON document also gives
the `reason`, the program file that Ravel found, the files of the
configuration script and the bounds.

Ravel prepares the `--json` routing notice after three steps succeed: the
selection, the reservation of the result output and the allocation of the
result channel. It prints the notice after the supervision attempt, before it
decides the final outcome. The notice does not prove the launch, the cleanup
or success. Standard error stays a mixed stream, and the managed process can print
text that looks like a Ravel document. See
[read notices](05-running-sessions.md#read-notices-separately-from-results).

Fragment: standard error of the `--json` review command of the example
`notices` in chapter 5, “Read notices separately from results”. It comes from
a real run on macOS. The run used the directory `/tmp/ravel-chapter-5/notices` in
place of `~/ravel-chapter-5/notices`, and macOS shows `/tmp` as
`/private/tmp`. Ravel writes the JSON document as one line. This block shows
it on several lines and omits the members `bounds`, `context`,
`diagnostics`, `operationReports`, `configEnvironment`, `reads` and `worker`.
The value of `selectionMs` changes in each run.

```text
review-agent: the review is complete
{
  "harnessRuns": [],
  "config": {
    "modules": [
      {
        "bytes": 619,
        "name": "/private/tmp/ravel-chapter-5/notices/config.rhai",
        "sha256": "813135dac982142b7d78e7ee5c1ae800e970ecc3e730de300f73ec5cc0b98246"
      }
    ],
    "path": "/private/tmp/ravel-chapter-5/notices/config.rhai",
    "version": "notice-demo"
  },
  "promptSource": "/private/tmp/ravel-chapter-5/notices/review-prompt.txt",
  "resolution": "/tmp/ravel-chapter-5/notices/bin/review-agent (found on PATH in /tmp/ravel-chapter-5/notices/bin)",
  "schemaVersion": 2,
  "selection": {
    "effort": "low",
    "model": "fake",
    "provider": "local",
    "reason": "Reviews go to the fake agent."
  },
  "selectionMs": 3
}
```

The line of the fake agent comes first, because Ravel writes the JSON notice
after the supervision attempt. The tables below give each member.

| Field | Value and meaning |
|---|---|
| `schemaVersion` | `2`, the version of the shape of the routing notice. The failure document has its own value, `2`. |
| `selection` | An object with `provider`, `model`, `effort` and `reason`. The owner supplies these strings. The labels do not connect Ravel to a model provider. |
| `config` | The configuration script: `path`, the canonical path of its entry, `version`, the owner's `VERSION` string, and `modules`, the file measurements described below. |
| `worker` | `path`, the re-executed binary, `version`, its package version, `buildId`, its build identity, and `rhaiVersion`, its Rhai version. Ravel checks the worker identity before evaluation. This is not code signing. |
| `selectionMs` | The elapsed time of the evaluation of the configuration script, in milliseconds. |
| `context` | `null` when there is no delivered selection context. Otherwise, the report in the next table. It is never the session-result context. |
| `bounds` | Objects named `selection`, `context`, `source`, `sources`, `message`, `diagnostics` and `harnessRuns`. Each has its field for the value with a unit (`ms`, `bytes`, `sources` or `runs`) and `from`. `from` gives `default`, `fixed`, `settings.json` or the flag that set the value. |
| `configEnvironment` | An array of `{name, set}` grants. `set` is a boolean that shows whether the caller has the variable, never its value. Ravel does not add the [base variables](#variables-that-ravel-and-the-configuration-script-read) that `ravel::env` permits to this list. |
| `resolution` | A string that gives the resolved executable path and the base of the resolution: an absolute path, the caller's working directory or a `PATH` entry. |
| `promptSource` | `argument` for `--prompt`. For `--prompt-file`, the file path joined to the caller's working directory. |
| `diagnostics` | An object with `stdout` and `stderr`, the captured streams of the configuration script as strings. Invalid UTF-8 bytes show as replacement characters. |
| `operationReports` | An array of the report strings that the [runner](02-actors-and-words.md#evaluation-and-supervision) retained. It is present only when the runner returned its account of how the job ended. It is empty when there are no reports. A failed stop attempt in this list does not alone decide the final status. |

The following arrays describe evaluation activity. They can be empty.
Each record has all the fields in its row.

| Array | Record fields and order |
|---|---|
| `config.modules` | `name`: canonical file path. `bytes`: source byte count. `sha256`: lowercase SHA-256 of those bytes. The entry comes first, then modules in first-import order. |
| `reads` | `via`: `read_text` or `read_json`. `phase`: `load`, `context` or `selection`. `name`: canonical path. `bytes`: count before the JSON parse. `sha256`: lowercase digest. Successful host reads appear in call order, also for repeated reads. |
| `harnessRuns` | `harness`: settings entry name. `status`: the run status. `exitCode`: integer or `null`. `signal`: signal name or `null`. `elapsedMs`: integer duration. `sentResult`: whether the run sent a result. Calls appear in order, also for `unavailable` calls. |

`load` means module top-level execution, `context` means `load_context`, and
`selection` means `select`. The caller's context file appears in `context.sources`,
not in `reads`. Failed reads do not appear in `reads`, even when their bytes
consumed a source from the budget.

A harness-run record contains no prompt, output, result text or token usage.
Its `status` is `exited`, `signalled`, `timed_out`, `output_limit` or `unavailable`.
An unavailable call has null exit code and signal, zero elapsed time and `sentResult: false`.
See [harness-run outcomes](#harness-run-result) for the meaning of each status.

The non-null `context` report has six fields. They describe the data that
selection receives. [Chapter 4's selection-context section](04-write-a-configuration-script.md#supply-selection-context)
teaches where they come from.

| Selection-context report field | Meaning |
|---|---|
| `loader` | Whether the `load_context` of the configuration script made the delivered document. |
| `sources` | Measured source records. Each has the canonical `name`, `via` (`--context`, `read_text` or `read_json`), `bytes` and `sha256`. |
| `sourceBytes` | The sum of the measured source byte counts. |
| `encodedBytes` | The byte length of the compact JSON encoding of the delivered document, with the measured records. |
| `sha256` | The SHA-256 of that encoding. |
| `value` | The delivered selection-context document. |

> **Note:** The notice can contain selection-context data and the captured
> output of the configuration script. Grant diagnostics do not show
> environment values, but the trusted script can still print a value itself.

### JSON failure document

When a session fails with `--json`, the Ravel CLI writes a failure document
on the caller's standard error. Your program can read from it why the session
failed and what to do next. A failed final write can leave a prefix of the
result on standard output. Discard those bytes and use the failure status.

Fragment: the failure document of the third command of the example `notices`
in chapter 5, “Read notices separately from results”. The configuration
script has no rule for the kind `deploy`. This block omits the command and
its status, 3. Here `<scratch>/work` stands for the example's directory. In
your shell, the document shows the absolute path of that directory, for
example `/Users/you/ravel-chapter-5/notices/config.rhai`.

```text
{"diagnostics":{"stderr":"","stdout":""},"error":{"code":"config_refused","configCode":"unknown_kind","exit":3,"input":"--kind deploy","message":"the configuration script <scratch>/work/config.rhai refused the selection: This configuration script has no rule for the kind deploy.","remedy":"Use the kind review.","source":"<scratch>/work/config.rhai","stage":"selection"},"schemaVersion":2}
```

A JSON failure document has `schemaVersion: 2`, an `error` object and an
optional top-level `diagnostics`. That object contains the `stdout` and
`stderr` strings of the configuration script. Ravel supplies the first part of
the `message`. The configuration script supplies the `configCode`, the words
of the `message` after its colon and the `remedy`.

Read `code` and `stage`
together, and retain the complete standard error stream. Secondary operational reports can occur outside this document.
See [read failure notices](05-running-sessions.md#read-notices-separately-from-results).

| `error` field | Presence and meaning |
|---|---|
| `code` | Required. A stable Ravel diagnostic code, such as `malformed_input` or `session_result_missing`. |
| `stage` | Required. A stage string from the table below. |
| `message` | Required. An explanation. It can contain original and secondary operational details. |
| `remedy` | Required. A suggested correction or recovery action. |
| `exit` | Required. The numeric status. A cancellation also has its numeric status here. |
| `input` | Optional. The caller input involved, such as a flag. |
| `source` | Optional. The file that relates to the failure, such as the entry of the configuration script. |
| `location` | Optional. The location inside the source or data, such as `result.args[2]` or a script position `LINE:COLUMN`. A host-request failure also names the function and field. |
| `configCode` | Optional. The refusal code that the owner supplied, beside Ravel's `config_refused`. |
| `bound` | Optional. The exhausted bound: `name`, the field for the value with its unit, and `from`. |
| `signal` | Optional. `SIGINT`, `SIGTERM` or `SIGHUP`, the signal of the cancellation. |

The stage tells where Ravel found a problem, not which process ultimately caused it. For
example, Ravel uses `evaluation` for the whole deadline, because the Ravel
CLI cannot see which operation of the script ran at expiry.

| Stage | Scope | Taught in |
|---|---|---|
| `cli` | The command line and the settings. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `authority` | The choice of the entry of the configuration script and the attempt to open it, or the query for the caller's working directory. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `worker` | The re-execution of Ravel as its Rhai worker, or the check of the worker and its protocol. | [Check and evaluate](03-a-complete-session.md#check-the-request-and-evaluate-the-configuration-script) |
| `evaluation` | The evaluation by the worker as a whole, with the deadline and the cancellation of selection. | [Bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings), [cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `load` | The import of the entry of the configuration script and of each module that it imports. | [Import other Rhai files](04-write-a-configuration-script.md#import-other-rhai-files) |
| `context` | The read, assembly, validation and measurement of the selection context. | [Supply selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `validation` | The check of the exports and the function parameter counts of the configuration script. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `selection` | The call to `select` and the check of the data that it returns. | [Select or refuse](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `resolution` | The search for the selected program. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `exec` | The preparation and supervision of the managed launch, and the decision about its outcome. This stage also covers failures of the result output or the result channel before the spawn. | [Results](05-running-sessions.md#accept-a-result-only-after-success), [notices](05-running-sessions.md#read-notices-separately-from-results) |

## Environment variables

### Variables that Ravel and the configuration script read

Which caller variables can the configuration script read, and how? The Rhai
worker, the separate process that evaluates the script, receives an empty
environment. The script calls `ravel::env(name)` to ask for a caller value. The worker sends
that request to the Ravel CLI over a private socket. The Ravel CLI checks the
name and answers from the caller's environment.

The helper permits the base names below and explicit grants.
A permitted but unset name returns `()`, Rhai's empty value.

An ungranted name throws `env_not_granted`, even when unset.
A value outside UTF-8 throws `env_not_utf8`.
See [environment grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Variable or class | Meaning and effect |
|---|---|
| `HOME` | Locates the default configuration script and settings. The default entry needs an absolute value. With `--config`, an absent or relative value supplies no settings. The helper permits it. |
| `PATH` | Resolves a bare program for a final selection or harness run. If unset, Ravel cannot search. An empty or relative entry resolves from the caller's working directory. The helper permits it. |
| `TMPDIR` | Chooses the system temporary directory for private worker and run directories, and unconfined result channels without `--exit-dir`. The helper permits it. |
| `LANG`, `LC_*` | Caller locale values that the helper permits. `LC_*` means every name with that case-sensitive prefix. |
| Explicit grants | `configEnv` and `--config-env` add names that the helper permits. Duplicates count once. The caller supplies each value. |
| `XDG_CONFIG_HOME` | Does not move the configuration script or the settings from `$HOME/.config/ravel`. It needs an explicit grant for `ravel::env`. |
| `RAVEL_CONFIG_RUN` | Marks an evaluation harness run. Its presence, even with an empty value, makes `start-session` refuse before it reads the settings or evaluates the configuration script. |

A grant controls the helper, not file access or the environment of an unconfined
harness run. The trusted configuration script can read a credential file through a measured read,
or receive credentials that a harness prints. Ravel does not redact that data.

### Names that an environment grant cannot use

Some names control the dynamic loader or Ravel itself. `configEnv` and
`--config-env` do not accept a name in these four prefix classes.

The match is case-sensitive. `*` means any suffix, also the empty
suffix. It is not shell expansion. A grant that names one of them reports
`excluded_grant` with status 2. Other names, such as `NODE_OPTIONS`, are
names that you can grant.

The reserved classes are rules for `ravel::env` grants. They do not tell which
names Ravel removes from the environment of an unconfined managed process.
The next section gives those six exact names. See
[grant environment names](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Class | Why Ravel reserves it |
|---|---|
| `LD_*` | Dynamic-loader controls. They can make a program load other libraries. |
| `DYLD_*` | macOS dynamic-loader controls, with the same risk. |
| `RAVEL_*` | Ravel's own controls. They include `RAVEL_EXIT_FILE`, the session-result context that the Rhai worker never receives. |
| `HARNESS_DISPATCH_*` | Names of the earlier `harness-dispatch` command. Ravel reserves them with its own names. |

### Managed-process environment

Which variables does the managed process receive? When a process starts a
child, the child gets a copy of the environment of its parent, unless the
parent changes the copy. The Ravel CLI changes the copy for the managed process
in two ways. It removes six exact names, and it adds a new `RAVEL_EXIT_FILE`.

An unconfined managed process inherits the caller's environment, except for
six exact names that Ravel removes. The list of these names is below. Ravel
then supplies its new `RAVEL_EXIT_FILE`. Other names stay inherited, for
example any application credentials and the names in the reserved classes
above. An environment grant has no effect on that inheritance. See
[caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

| Name or class | Confined managed process | Taught in |
|---|---|---|
| `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG`, `LC_*` | Ravel retains the caller values when they are present. A retained `HOME` gives no filesystem access to that directory. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `TMPDIR`, `TMP`, `TEMP` | Ravel sets all three to the temporary directory of the private run. They replace the caller values. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `RAVEL_EXIT_FILE` | A new path for this session. Ravel supplies it after the environment grants. | [Nested sessions](05-running-sessions.md#keep-nested-sessions-separate) |
| Every other caller variable | Absent. `--config-env` grants only `ravel::env` access and adds no variable here. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |

The removed names are exactly `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID`, `RAVEL_STATE_DIR`,
`HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`. Ravel replaces the first with the new
result-channel path. The other five stay absent. Each evaluation harness run uses the same rules,
with its own result path and the additional `RAVEL_CONFIG_RUN` marker.

A final managed process receives no marker, because an invocation that inherits
it refuses before selection. These obsolete identity and
storage names create no store and select no session. See
[caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `RAVEL_EXIT_FILE`

`RAVEL_EXIT_FILE` is the inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information):
the path that `end-session` uses to send the result to its enclosing session.
The path names one result file in the private result channel of the session.
The Ravel CLI sets it separately for each managed launch and evaluation harness run. The Rhai worker never
receives it.

How does the path reach `end-session`? Ravel puts it in the environment of the
program that it launches. Each child process gets a copy of the environment of
its parent, unless the parent changes it. If each parent leaves the path
unchanged, a shell that the harness starts and the `ravel end-session` in that
shell have the same path. `end-session` reads the path from its own environment
and writes the result to that file.

What happens when sessions nest? A nested launch replaces the path only for
its own managed process. The value of the outer caller does not change. In the
real run below, a fake agent printed its own `RAVEL_EXIT_FILE`. Then it was the
caller of a second, inner session. The inner fake agent printed its value, and
the outer fake agent printed its value again.

Fragment: standard error of the real nested session of chapter 1, on macOS
with `TMPDIR=/tmp`. This block omits the cleanup reports. The random parts of
each path change in each run. macOS gives `/tmp` the real path `/private/tmp`.

```text
ravel: starting provider local, model fake, effort none for kind "outer"
outer-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-mlcN86/signal-c7d5f2eed4198eb768996f7b4868bbcc
ravel: starting provider local, model fake, effort none for kind "inner"
inner-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-QpztGn/signal-b643cbee529e5887c7d2a3d0496210c9
outer-agent: the inner session returned inner-done
outer-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-mlcN86/signal-c7d5f2eed4198eb768996f7b4868bbcc
```

Each child inherits its parent's path unless its parent changes it.
Ravel sets a new path for the inner harness.

![A caller starts outer start-session without a result path, and its Rhai worker has no path. Ravel gives the outer harness an outer path, which its shell and end-session inherit. The shell starts inner start-session with the outer path. The inner harness receives a new inner path, which its end-session inherits. Each end-session writes to its own path. An evaluation harness run gets a separate path and returns its result to the configuration script.](figures/inherited-result-path.svg)

*Figure: The nested run gives each session's children its own result path.*

[Open the inherited result paths at full size](figures/inherited-result-path.svg).

The inner session got a new path, but only the inner fake agent received it.
The outer fake agent kept its own path. Thus, each `end-session` goes to the
session that encloses it.

If the path is unset, empty or stale, `end-session` fails with status 1. Do
not use it as a persistent result location or as an address across sessions.
The result channel separates ordinary sessions. It is not a defence against
hostile code that runs as the same user. See
[nested sessions](05-running-sessions.md#keep-nested-sessions-separate) and
the row for code of the same user in
[Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup).

## Configuration script and settings

### `config.rhai`

Where does Ravel find the configuration script? The configuration script is a
file of Rhai code that chooses the program for each session. The book calls
the file that Ravel loads first the **entry**. The entry can import other
Rhai files.

`$HOME/.config/ravel/config.rhai` is the default entry.
This default needs an absolute `HOME`.
`--config PATH` chooses a different entry, relative to the caller's working directory.
There is no repository search, environment override or fallback script.
An explicit entry can have any file name that ends in `.rhai`, such as
`review.rhai`.

The entry must canonicalise to a readable regular UTF-8 file of at most 1 MiB.
Its canonical path must also be UTF-8. A symlink selects its real file.
Imports resolve from the directory of that real file.
An unacceptable entry gives status 3 before the script runs.
See [define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request).

| Entry choice | Ravel 2.0.0 behaviour |
|---|---|
| Default Rhai entry exists | Evaluate it. Other formats of configuration script do not participate. |
| Default Rhai entry absent, but `policy.ts` exists | Refuse with `config_format_retired`, status 3. The remedy names `config.rhai` and chapter 4, “Write a configuration script”. |
| Neither default file exists | Refuse with `config_missing`, status 3. |
| Explicit `.rhai` entry | Evaluate it after file checks. |
| Explicit `.ts`, `.mts`, `.cts`, `.js`, `.mjs` or `.cjs` entry | Refuse with `config_format_retired`, status 3. |
| Other explicit extension | Refuse with `config_unreadable`, status 3. Supply a `.rhai` file. |

Ravel evaluates the entry in the Rhai worker, a second copy of its own executable.
At the deadline, the Ravel CLI ends evaluation and can signal the worker.
A kernel stall can delay its exit. A memory or stack failure of the script stops only the worker.
The configuration script has no direct filesystem, network, process or environment access.
It uses the [host capabilities](#host-capabilities) for permitted effects.

The worker receives no session-result context.
Before the final managed launch, every host call ends and Ravel reaps the worker.
Ravel also removes the worker's private directory.

### `settings.json`

You may want the same limits, grants and harnesses for every session of your
account. The owner writes them once in a settings file. A command-line option
can then replace some values for one session.

`$HOME/.config/ravel/settings.json` supplies the owner's defaults for `start-session`.
`--config` does not move it, and `XDG_CONFIG_HOME` does not select it.

Fragment: a settings file with four of the five keys, which the checkout build
accepted on 2026-10-29. This block omits the configuration script and the
program `fake-classifier`.

```json
{"timeoutMs":60000,"configEnv":["BOOK_EFFORT"],"harnesses":{"classifier":{"program":"fake-classifier","capabilities":["classify"]}},"budgets":{"harnessRuns":1}}
```

These settings give evaluation 60 seconds and let the configuration script
read `BOOK_EFFORT`. They name one harness, `classifier`, and permit one harness run
for each session. `contextBytes` is absent, so its default applies.

A missing file, an absent or relative `HOME` or an empty JSON object supplies
no settings. A file that exists must be a readable regular file that contains
a JSON object. `end-session` reads neither the settings nor the configuration script. See
[set bounds and grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Key | Accepted value, default and consequence | Taught in |
|---|---|---|
| `timeoutMs` | Optional whole number of milliseconds, 1,000–600,000 inclusive. The default is 30,000. `--timeout-ms` replaces the value after settings validation. It limits worker startup, module loads, the loader, selection and every host call. It does not limit the final managed process. | [Set bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `contextBytes` | Optional whole number of bytes, 1–8,388,608 inclusive. The default is 262,144. `--context-bytes` replaces it after validation. It limits the caller document and the delivered selection context. It is also the ceiling for a host read. | [Set bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `configEnv` | Optional array of environment-variable names. The default is no extra grants. Each name is nonempty UTF-8 without `=` and cannot match a reserved grant class. Flags add names. Ravel merges duplicates, and unset values stay absent. Grants permit `ravel::env` reads. They change no managed-process or harness-run environment. | [Set bounds and environment grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `harnesses` | Optional object of named programs for harness runs. Default: no entries. Each entry has the fields below. Final selections need no entry. | [Harness entries](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |
| `budgets` | Optional object with only `harnessRuns`. An empty object uses the default budget. | [Evaluation budgets](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |

A harness name contains 1–64 lowercase ASCII letters, digits or dashes.
The first character is a letter or digit. Ravel refuses unknown nested fields.

| Nested setting | Accepted value and default |
|---|---|
| `harnesses.NAME.program` | Required nonempty string without NUL. An absolute path, a relative path with `/`, or a bare `PATH` name. Resolution uses the caller's directory and environment. |
| `harnesses.NAME.capabilities` | Optional array of nonempty strings. Default: empty. Discovery repeats these owner labels without interpretation. |
| `budgets.harnessRuns` | Optional integer from 0 to 16 inclusive. Default: 0. Only settings can raise it. Each launched harness run consumes one. |

These problems report `settings_invalid`, status 2:

- an unknown key, also the obsolete `stateDir` and a `config` key
- an unreadable or non-regular file
- invalid JSON, or a document that is not an object
- a `configEnv` that is not an array of strings
- a malformed `harnesses` entry or `budgets` object, also an invalid `harnessRuns` value

Invalid `timeoutMs` or `contextBytes` values and malformed grant names use `malformed_input`, and
excluded grants use `excluded_grant`. Both have status 2. Ravel validates the
whole file before flags replace values. Thus, an invalid setting fails even
with a valid flag. No setting opens persistent storage. See
[settings validation](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

## Bounds

A **bound** is a limit on evaluation: on its time, on the data that it reads
or on the harness runs that it makes. When a bound stops evaluation, Ravel
launches no final managed process. Harness runs can occur before that stop.
You can change some bounds in the settings or on the
command line. Ravel sets the others, and you cannot change them.

The configuration script can read the effective bounds in `request.limits`. For example,
with no settings and no bound options, a real request had this member:

Fragment: the `limits` member of the request in the
[request entry](#selectionrequest). This block omits the other members.

```json
{"limits":{"contextBytes":262144,"diagnosticsBytes":262144,"harnessRuns":0,"messageBytes":1048576,"selectionMs":30000,"sourceBytes":65536,"sources":256}}
```

`request.limits` gives seven effective bounds. The Ravel CLI and worker enforce
their own copies. A change to the request map cannot raise a bound.
Evaluation bounds end selection, even inside `try`. A harness run
can instead return its own timeout or output-limit status as data. The JSON notice names each bound separately and
records its origin.

See [set bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Request field / notice name | Default or fixed value | Adjustment and consequence | Taught in <!-- tc: depart synonym-rotation because fixed describes an immutable bound, not an action to correct an error --> |
|---|---:|---|---|
| `selectionMs` / `selection` | 30,000 ms | `timeoutMs`, then `--timeout-ms`, within 1,000–600,000. Ravel measures it from the worker spawn through the startup, the import, the loader and the selection. Expiry is `selection_timeout`, status 124, unless cancellation or failed harness cleanup takes precedence. The teardown time is additional. | [Selection deadline](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `contextBytes` / `context` | 262,144 bytes | `contextBytes`, then `--context-bytes`, within 1–8,388,608. The caller file and the compact delivered JSON with `measured` must both fit. If one does not fit, the code is `context_too_large`, status 3. | [Selection-context budget](04-write-a-configuration-script.md#supply-selection-context) |
| `sourceBytes` / `source` | The smaller of 65,536 bytes and `contextBytes` | A positive whole-number `max_bytes` of a host read can replace the default, up to `contextBytes`. Too many bytes, or a requested limit that is too large, cause `source_too_large`, status 3. No script `try` can prevent this refusal. | [Host reads](04-write-a-configuration-script.md#supply-selection-context) |
| `sources` / `sources` | Fixed 256 | It applies separately to the measured reads, with the caller document and reads in every phase, and to the attributed `sources` array of the document. If either has more, the code is `too_many_sources`, status 3. A caught exception for a read bound still causes the refusal. | [Measured and attributed sources](04-write-a-configuration-script.md#supply-selection-context) |
| `messageBytes` / `message` | Fixed 1,048,576 bytes | It limits a host request, selection result or failure report from the worker. A message that is too large gives `message_too_large`, status 3. A delivered document has the selection-context budget instead. | [Protocol bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `diagnosticsBytes` / `diagnostics` | Fixed 262,144 bytes | The combined standard output and standard error of the worker, with `print` and `debug` output. Output above the limit gives `output_limit`, status 3. Ravel retains the first 262,144 bytes and discards the rest while it stops the worker. | [Output bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `harnessRuns` / `harnessRuns` | 0 runs | Only `budgets.harnessRuns` changes it, within 0–16. A launched run consumes one. Exhaustion gives `harness_budget_exhausted`, status 3. | [Harness runs](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |

`from` in a reported bound is `default`, `fixed`, `settings.json` or the flag
that set the bound. If the selection-context budget makes a source limit lower,
that limit gets the origin of the budget. If a read gave its own limit, a
source-bound refusal reports `max_bytes` as its `from`. The report uses `ms`,
`bytes`, `sources` or `runs` for its numeric field. See [bounds and their inputs](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

## Configuration script and request

### Contents of a configuration script

What must a configuration script contain? Its entry is a Rhai module. It
exports two constants and defines a `select` function. Here is a short
configuration script. It sends each `review` request to a fake agent and refuses every other kind.

Fragment: `config.rhai` of the example `notices` in chapter 5, “Read notices
separately from results”. This block omits the fake agent `review-agent` and
the command that runs the script. Chapter 5 runs this script as a complete
example.

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "notice-demo";

fn select(request, context) {
    if request.kind == "review" {
        return #{
            status: "selected",
            program: "review-agent",
            args: [request.prompt],
            provider: "local",
            model: "fake",
            effort: "low",
            reason: "Reviews go to the fake agent.",
        };
    }
    #{
        status: "refused",
        code: "unknown_kind",
        message: "This configuration script has no rule for the kind " + request.kind + ".",
        remedy: "Use the kind review.",
    }
}
```

`#{ … }` is a Rhai map, a set of named values. The last expression of a Rhai
function is its return value. Thus, `select` returns the refusal map when the
kind is not `review`.

The entry exports constants and defines public functions with the parameter counts below.
Other constants and functions can serve as helpers.
Top-level statements run once before the loader and selection.
See [define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request).

| Declaration | Contract |
|---|---|
| `export const SCHEMA_VERSION = 3;` | Required integer 3. Another integer gives `unsupported_version`. A missing or mistyped export gives `config_invalid`, status 3. |
| `export const VERSION = "...";` | Required nonblank owner-maintained string. The notice reports it independently of Ravel's package version. |
| `fn select(request, context)` | Required public function with two parameters. Returns a selected or refused map. The second argument is the delivered selection context or `()`. |
| `fn load_context(request)` | Optional public function with one parameter. Returns selection context or a refusal before `select`. A return of `()` gives `context_invalid`. |

Functions read top-level constants through `global::NAME`.
They do not capture variables around them.
Ravel checks undefined variables at compilation and refuses absent map properties at execution.
Ravel disables `eval`.
`print` writes to the script's captured standard output, and `debug` writes to its captured standard error with source information.
Both streams count towards the diagnostics bound and reach the caller's standard error.

### `SelectionRequest`

What does the configuration script receive? The Ravel CLI builds the
**selection request** from the caller's command and the effective bounds. The
script receives it as the first argument of `select`, and of `load_context` if
the script has one. The script can use each field to choose a route or to build the arguments.

The selection request is a Rhai map. Every documented field is present.
An absent optional input has the value `()`, Rhai's empty value, not an empty
string. The request carries data and limits, never the session-result context.

Fragment: a real request, from the checkout build on 2026-10-29. The
configuration script printed `ravel::json_encode(request)`, and the Ravel CLI wrote that line on
standard error. This block omits the other lines and the command.

```text
config stdout: {"context":null,"cwd":"/private/tmp/ch6","kind":"review","limits":{"contextBytes":262144,"diagnosticsBytes":262144,"harnessRuns":0,"messageBytes":1048576,"selectionMs":30000,"sourceBytes":65536,"sources":256},"params":{"repo":"ravel"},"prompt":"Review the changes to src/parse.rs and tests/parse.rs in the last commit.\nCheck that the code is correct and that the tests cover each change.\n\nWhen the review is complete, end this session with one command:\n- If you found no problem, run: ravel end-session approved\n- If you found a problem, write your findings to review-notes.md.\n  Then run: ravel end-session changes-needed\n","schemaVersion":3,"taskFile":null,"taskId":"T-17"}
```

The caller gave `--kind review`, `--param repo=ravel`, `--task-id T-17` and
`--prompt-file review-prompt.txt`, the prompt file of chapter 1's review. The
prompt keeps each newline of the file, also the last one, which JSON shows as
`\n`. JSON shows each `()` as `null`, so `context` and `taskFile` were `()` in
the script. See [read the request](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request).

| Field | Value and limits | Taught in |
|---|---|---|
| `schemaVersion` | Always the integer `3`, the version of the interface between Ravel and the configuration script. The selection context has its own `schemaVersion`, `1`. | [Read the request](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `kind` | Required nonempty caller token, unchanged from `--kind`. Ravel defines no catalogue. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt` | Required UTF-8 string, possibly empty, with a maximum of 1,048,576 bytes and no NUL. The caller bytes and newlines stay unchanged. The configuration script chooses how to give them to its program. | [Prompt delivery](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `cwd` | The caller's working directory as data. Relative input paths and selected program paths resolve from it. The worker starts in a private directory. The script cannot change that directory or read files directly. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `params` | String values by caller-supplied name, or `#{}` if there are none. Names are nonempty without `=`. Ravel refuses a duplicate name. All names and values together have a maximum of 65,536 bytes. | [Caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskFile` | Absolute UTF-8 path from `--task-file`, or `()`. Ravel does not canonicalise or read it, and the file does not have to exist. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskId` | Nonempty opaque UTF-8 string of at most 1,024 bytes, or `()`. It names no Ravel record. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `context` | The caller's checked selection context, or `()`, before Ravel attaches `measured`. A loader sees this input, also when it returns different data for `select`. | [Caller and delivered documents](04-write-a-configuration-script.md#supply-selection-context) |
| `limits` | Required map with the seven effective fields in [Bounds](#bounds). A change to this map cannot change enforcement. | [Effective bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |

## Selection and refusal

### `Selected`

What does the configuration script return when it chooses a program? It returns a
**selection**: a map with `status: "selected"`. The selected map is the
command that `select` returns. `program` and `args` are the command that Ravel
runs. The other fields are labels.

A label describes the choice of the script, but it does not configure the
harness. For example, a script that selects Claude Code puts the model option
of Claude Code in `args`. The `model` label only tells the caller which model
the script chose. Ravel writes the labels in the
[routing notice](#standard-streams) on the caller's standard error. The labels
do not reach the program.

All seven fields are required, and Ravel refuses unknown fields. Ravel
validates the shape, then resolves the program. It does not check that the
prompt is in the arguments or that the labels describe the actual program.
The [contents entry](#contents-of-a-configuration-script) shows a real selection, and the table
gives its values. See [select a command](04-write-a-configuration-script.md#select-a-command-or-refuse).

| Field | Accepted value and effect | Taught in |
|---|---|---|
| `status` | Literal `"selected"`. | [Select or refuse](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `program` | Nonblank string without NUL. It is an absolute path, a path relative to the caller that contains `/`, or a bare name that Ravel finds through the caller's `PATH`. It also supplies `argv[0]`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `args` | Array of strings after `program`. Ravel accepts an empty array and empty strings, but not NUL. Each string is one argument. Ravel does no word splitting, shell expansion or template interpretation. | [Construct arguments](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `provider` | Nonblank owner label for the model origin. Ravel reports it as given. Ravel does not infer it or use it to contact a provider. | [Selection labels](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `model` | Nonblank owner label. It does not have to occur in `args`, and it does not configure the selected program by itself. | [Selection labels](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `effort` | Nonblank owner label. Ravel defines no effort levels and does not change this label into a flag. | [Selection labels](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `reason` | Nonblank explanation by the owner. Ravel reports it verbatim in routing diagnostics. | [Selection labels](04-write-a-configuration-script.md#select-a-command-or-refuse) |

### `Refused`

You may want the configuration script to decline a request, for example a
kind that has no route. The script then returns a **refusal**: a map with `status: "refused"`.
The refused map declines the request from `select` or `load_context`. All four
fields are required, and Ravel refuses unknown fields. Ravel launches no final
managed process and tries no other route.

A valid refusal reports Ravel's `config_refused`,
status 3, with the owner's code beside it as `configCode`. The stage is
`selection` for `select` and `context` for `load_context`. See [return a refusal](04-write-a-configuration-script.md#select-a-command-or-refuse)
and [refuse from a loader](04-write-a-configuration-script.md#supply-selection-context).

| Field | Accepted value and effect | Taught in |
|---|---|---|
| `status` | Literal `"refused"`. | [Return a refusal](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `code` | Nonblank owner-defined string. It is separate from the namespace of Ravel's diagnostic codes. | [Return a refusal](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `message` | Nonblank explanation of what prevents selection. | [Return a refusal](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `remedy` | Nonblank action that the caller or owner can take. | [Return a refusal](04-write-a-configuration-script.md#select-a-command-or-refuse) |

What does the caller receive after a refusal? The configuration script of the
[contents entry](#contents-of-a-configuration-script) refuses the kind `deploy` with the code
`unknown_kind`. The Ravel CLI writes the refusal on the caller's standard
error and exits with status 3. Standard output stays empty. The
[diagnostic codes](#diagnostic-codes) entry shows the real text, and the
[JSON failure document](#json-failure-document) entry shows the JSON form.

### Diagnostic codes

Did a session fail with a code such as `config_refused`? Find the code in the
tables below. Each row gives the stage, the status and what to do next. The
Ravel CLI writes each failure on the caller's standard error. Without
`--json`, the first line has the form `ravel: refused (CODE, stage STAGE):`,
and each detail follows on its own line.

Fragment: standard error of a refused request, from the checkout build on
2026-10-29. The configuration script of the
[contents entry](#contents-of-a-configuration-script), saved as
`refuse.rhai` in `/tmp/ch6`, refused the kind `deploy`. This block omits the
command and its status, 3.

```text
ravel: refused (config_refused, stage selection): the configuration script /private/tmp/ch6/refuse.rhai refused the selection: This configuration script has no rule for the kind deploy.
  input: --kind deploy
  source: /private/tmp/ch6/refuse.rhai
  config code: unknown_kind
  remedy: Use the kind review.
```

`config_refused` is Ravel's code. `unknown_kind` is the code of the owner,
from the refusal map. Ravel supplies the first line up to “refused the
selection:”, and the input and source lines. The configuration script
supplies the rest of the first line, the config code and the remedy.

With `--json`, the same failure is a
[JSON failure document](#json-failure-document). In a JSON failure document,
`error.code`, `error.stage` and `error.exit` identify a Ravel failure. These
tables give the usual classification of each code.

Cancellation has precedence over a failure that already occurred.
Failed evaluation harness cleanup can replace a deadline with status 5.
After evaluation, the original error decides the code and status.
Ravel reports later cleanup errors beside it. Thus, a code in these tables can also occur
as retained detail beside a different outcome.

Read the complete standard error
stream. See [interpret notices](05-running-sessions.md#read-notices-separately-from-results).

| Code | Stage | Status | Meaning and remedy | Taught in |
|---|---|---|---|---|
| `malformed_input` | `cli` | 2 | Invalid CLI input, numeric setting or grant name. Correct the named input and range. An invalid setting fails even with a valid flag. | [Inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `prompt_unreadable` | `cli` | 2 | Ravel cannot open or read the prompt file, or the file is a terminal. Supply a readable file that is not a terminal, or use `--prompt`. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt_invalid` | `cli` | 2 | The prompt is invalid UTF-8, contains NUL or is larger than 1 MiB. Correct the encoding or the content, or make the prompt shorter. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `settings_invalid` | `cli` | 2 | The settings file, a key or a nested entry is invalid. Correct the named location to the five optional settings and their documented shapes. | [Settings and bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `excluded_grant` | `cli` | 2 | A grant in the settings or in a flag names a reserved variable or prefix: a dynamic-loader control or one of Ravel's own names. Remove the grant. | [Grants](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `confinement_unusable` | `cli` | 2 | Ravel cannot use the backend or the confinement inputs. Correct the named backend or path. No unconfined launch runs in its place. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `confinement_overlap` | `cli` | 2 | A grant overlaps the directory of the configuration script or the settings. Separate the paths, also their canonical symlink targets. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `exit_dir_unusable` | `cli` | 2 | The parent of the result channel is unusable. Name a writable directory that exists, or omit `--exit-dir`. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `cwd_unavailable` | `authority` | 3 | Ravel cannot get the caller's directory. Run from a readable directory that exists. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `home_unset` | `authority` | 3 | The default entry has no absolute `HOME`. Set an absolute `HOME` or use `--config`. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `config_missing` | `authority` | 3 | The chosen entry does not exist. Create it or correct `--config`. Ravel does no search. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `config_unreadable` | `authority` | 3 | The entry is not a readable regular UTF-8 file within 1 MiB, or its path or extension is unusable. Supply a valid `.rhai` entry. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `worker_missing` | `worker` | 5 | Ravel cannot locate or re-execute its own binary as the Rhai worker. Repair the installation or execute permissions. | [Validate with fake agents](04-write-a-configuration-script.md#validate-a-configuration-script-with-fake-agents) |
| `worker_identity_mismatch` | `worker` | 5 | The re-executed worker reports a different protocol, package, build or Rhai identity. Reinstall Ravel and start a new invocation. | [Validate with fake agents](04-write-a-configuration-script.md#validate-a-configuration-script-with-fake-agents) |
| `worker_failed` | `worker` | 5 | The preparation, execution or required teardown of the worker failed. Examine the retained operation details, and repair the installation or the named resource. | [Operational failures](05-running-sessions.md#read-notices-separately-from-results) |
| `protocol_error` | `worker` | 5 | The private worker conversation is malformed or closed unexpectedly. Retain the diagnostics and reinstall or rebuild Ravel. | [Validate with fake agents](04-write-a-configuration-script.md#validate-a-configuration-script-with-fake-agents) |
| `selection_timeout` | `evaluation` | 124 | The whole deadline expired during worker startup, imports, loader, selection or host calls. Reduce the work or increase the permitted deadline. Failed harness cleanup can replace this outcome. | [Settings and bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `selection_cancelled` | `evaluation` | 129/130/143 | HUP, INT or TERM ended the selection. Retain the teardown reports, and try again only if the caller still wants the work. Termination by the signal is the usual outcome at the process level. | [Selection cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `output_limit` | `evaluation` | 3 | The standard output and standard error of the worker together are larger than 262,144 bytes (256 KiB). Print less. The retained prefix is diagnostic evidence, not an accepted selection. | [Output bound](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `config_import_failed` | `load` | 3 | A module read, import cycle, module-count breach or top-level execution failed. Correct the file, import chain or top-level code. | [Import other Rhai files](04-write-a-configuration-script.md#import-other-rhai-files) |
| `config_invalid` | `validation` | 3 | A required export is absent or mistyped, or a function is private or has the wrong parameter count. Supply the exports and functions of [the contents entry](#contents-of-a-configuration-script). | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `unsupported_version` | `validation or context` | 3 | The `SCHEMA_VERSION` of the configuration script is not 3, or the `schemaVersion` of the selection context is not 1. Supply the supported shape, not only its version number. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `message_too_large` | `selection` | 3 | A host request, selection result or failure frame exceeds the 1 MiB worker-message bound. Reduce the data. The delivered selection context has its separate budget. | [Settings and bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `context_unreadable` | `context` | 3 | Ravel cannot read the caller document as a regular file with a canonical UTF-8 path. Correct `--context` or omit it. | [Selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `context_invalid` | `context` | 3 | The caller or loader document is invalid JSON, or it has invalid fields or data. Or the loader returned nothing, unserialisable data or a malformed refusal. Correct the named location to valid selection context or a valid refusal. | [Selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `context_too_large` | `context` | 3 | The caller bytes, or the delivered JSON with the measurements, are larger than the budget. Deliver less, or increase `contextBytes` or `--context-bytes` within its range. | [Selection-context budget](04-write-a-configuration-script.md#supply-selection-context) |
| `context_loader_failed` | `context` | 3 | An ordinary error escaped `load_context`. Correct the source or handle the data error. No selection follows without the required selection context. | [Selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `source_too_large` | `context` | 3 | A source exceeds its read limit, or `max_bytes` exceeds the context budget. Read less or use an allowed limit. No `try` can prevent refusal. | [Selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `too_many_sources` | `context` | 3 | Measured reads, with the caller document and failed UTF-8 or JSON conversion, exhausted 256 sources. Or attributed sources exceed 256. Read or attribute fewer sources. | [Selection context](04-write-a-configuration-script.md#supply-selection-context) |
| `config_refused` | `selection` or `context` | 3 | `select` or the loader returned a valid refusal. Follow its `configCode`, message and remedy, which the owner supplied. No alternative route runs. | [Refusals](04-write-a-configuration-script.md#select-a-command-or-refuse), [loader refusals](04-write-a-configuration-script.md#supply-selection-context) |
| `selection_abstained` | `selection` | 3 | `select` returned `()`. Return a selected or refused map on every path. | [Select or refuse](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `selection_malformed` | `selection` | 3 | The result has the wrong shape, fields or types, or contains NUL, or Ravel cannot serialise it. Correct the named location to `Selected` or `Refused`. | [Selection shape](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `selection_threw` | `selection` | 3 | An ordinary error escaped `select`. Correct the reported source location or handle the data error. To decline intentionally, return a refusal map. | [Select or refuse](04-write-a-configuration-script.md#select-a-command-or-refuse) |
| `config_parse_failed` | `load` | 3 | The entry or an imported module does not compile. Correct the syntax or undefined variable at the source location. | [Validate with fake agents](04-write-a-configuration-script.md#validate-a-configuration-script-with-fake-agents) |
| `config_format_retired` | `authority` | 3 | The selected entry uses a retired format. Supply a Rhai configuration script, as chapter 4 teaches. No compatibility path runs. | [Define a configuration script](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request) |
| `config_reentry_refused` | `authority` | 3 | `start-session` inherited `RAVEL_CONFIG_RUN` from an evaluation harness run. Call the function of the route in the configuration script directly, not another session. | [Harness runs](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |
| `host_request_invalid` | `load, context or selection` | 3 | A host call has an invalid argument type, field or value. Correct the function and field at the reported location. This failure is not catchable. | [Data helpers](04-write-a-configuration-script.md#check-json-with-the-data-helpers) |
| `script_limit` | `evaluation` | 3 | The configuration script exceeded an engine bound. Reduce its operations, recursion or data size. A `try` block cannot suppress this failure. | [Settings and bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `memory_limit` | `evaluation` | 3 | The worker exceeded its 1 GiB heap bound. Reduce retained data. The worker exits, and no managed launch follows. | [Settings and bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings) |
| `harness_budget_exhausted` | `evaluation` | 3 | The configuration script requested a harness run with no runs left. Reduce calls or raise `budgets.harnessRuns` in the settings, at most to 16. No caller flag raises it. | [Harness runs](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |
| `harness_cleanup_failed` | `evaluation` | 5 | A harness run left a group, failed reap or failed required cleanup. Retain the reports and inspect the named resources. This failure replaces an evaluation deadline. | [Harness runs](04-write-a-configuration-script.md#run-a-harness-during-evaluation) |
| `program_not_found` | `resolution` | 127 | The selected path does not exist, or no entry of the caller's `PATH` has the bare name. Correct the path or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `program_unexecutable` | `resolution` | 126 | The selected file is not an executable regular file, or Ravel cannot search the caller's `PATH` for a bare name. Correct the permissions, the file or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |

| Code | Stage | Status | Meaning and remedy | Taught in |
|---|---|---|---|---|
| `cancellation_unavailable` | `exec` | 5 | The Ravel CLI cannot prepare for cancellation. Examine the reported OS error and the execution environment. | [Operational failures](05-running-sessions.md#read-notices-separately-from-results) |
| `signal_state_unavailable` | `exec` | 5 | The capture of the signal state at entry, or a required transition of the signal state, failed. Retain all reports, and correct the execution environment. Do not treat any result as success. | [Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup) |
| `output_unavailable` | `exec` | 5 | Ravel cannot reserve or restore standard output for the result. Supply a writable output stream, and examine any error of the restoration. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_channel_unavailable` | `exec` | 5 | Ravel cannot allocate or prepare the private result channel. Correct the temporary storage or `--exit-dir`, and examine the reports of a partial cleanup. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `supervision_failed` | `exec` | 127, 126 or 5 | Before the runner reports the start, ENOENT gives 127, and other failures give 126. After the start, the status is 5. Check the executable and the interpreter, the execution environment and the retained reports of the runner and the terminal. | [Launch and supervision](05-running-sessions.md#interpret-the-exit-status) |
| `session_result_missing` | `exec` | 1 | The managed process ended and sent no result, even if its own status was 0. Make the managed process call `end-session` when its work succeeds. | [The sent result and its absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_result_invalid` | `exec` | 5 | Ravel cannot accept the sent entry in the result channel or the result bytes, for example because of an invalid encoding, type or size. Use `end-session` inside the live session, and send a result of 1 MiB or less. | [Result validity](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cleanup_failed` | `exec` | 5 | The required cleanup of the result channel or of a private directory failed. Examine the named path and the ownership report before you remove any residue. Ravel returns no result to the caller. | [Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup) |
| `process_group_survived` | `exec` | 5 | Ravel did not confirm the absence of the group when it decided the final outcome. Examine the reported group and the cleanup failures before you try again. The sent result alone is not sufficient. | [Cleanup and success](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cancelled` | `exec` | 129/130/143 | HUP, INT or TERM has priority when Ravel decides the final outcome, even after Ravel received the result. Ravel returns no result to the caller. Retain the original and cleanup errors. Try again only if the caller still wants the work. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `result_write_failed` | `exec` | 5 | The final write or flush of the result failed after Ravel decided the outcome. Correct the output destination, and discard any prefix that Ravel already wrote. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |

An `end-session` failure is a plain `ravel: end-session:` message with
status 1. It is not an entry in the JSON code table. A missing, empty or stale
`RAVEL_EXIT_FILE`, an oversized argument or a failed send does not mean an
empty result. See [send and accept results](05-running-sessions.md#accept-a-result-only-after-success).

## Selection context and host capabilities

### `Context` and `DeliveredContext`

You may want facts about the task to change the choice of the configuration
script. For example, a large change can go to a higher effort.
**Selection context** carries such facts to `select`, as its second argument. It is data for the decision,
not authority to execute a command. Do not confuse it with the
session-result context, the path in `RAVEL_EXIT_FILE`.

Fragment: a real caller document and the delivered selection context that
`select` received, from the checkout build on 2026-10-29. The caller gave
`--context context.json`, and the configuration script printed
`ravel::json_encode(context)`. This block omits the command and the other
lines on standard error.

```text
$ cat context.json
{"schemaVersion":1,"summary":"Review the change","facts":{"effort":"high"}}
config stdout: {"facts":{"effort":"high"},"measured":[{"bytes":76,"name":"/private/tmp/ch6/context.json","sha256":"695ce021a52f9af91d5b1b71e5571bbe1096b0c9543f108c8fc7864453512d88","via":"--context"}],"schemaVersion":1,"summary":"Review the change"}
```

The script had no `load_context`, so `select` received the caller's document.
Ravel added `measured`, with the size and the SHA-256 digest of the 76 bytes
that it read. The script can now read `context.facts.effort` and choose the
effort `high`.

Selection context is a JSON document. `--context` supplies it, or
`load_context` returns it. An absent optional field means unknown, which is different from a
known empty object, array or string. Ravel refuses unknown top-level fields,
for example `program`, `args`, `argv`, `command`, `select`, `load_context`, `env`,
`environment`, `exec`, `shell` and `script`. These names inside free-form JSON
facts stay data. See [supply selection context](04-write-a-configuration-script.md#supply-selection-context).

| Field | Accepted value | Taught in |
|---|---|---|
| `schemaVersion` | Required number `1`. The selection request has its own `schemaVersion`, `3`. | [Selection-context shape](04-write-a-configuration-script.md#supply-selection-context) |
| `summary` | Optional string, which can be empty. | [Selection-context shape](04-write-a-configuration-script.md#supply-selection-context) |
| `acceptanceCriteria` | Optional array of strings. The array and the strings can be empty. | [Selection-context shape](04-write-a-configuration-script.md#supply-selection-context) |
| `facts` | Optional object that maps names to JSON values. A fact is data. Ravel never runs it as a command. | [Selection-context shape](04-write-a-configuration-script.md#supply-selection-context) |
| `assessments` | Optional object that maps names to attributed `{ by, value }` objects. | [Attributed judgements](04-write-a-configuration-script.md#supply-selection-context) |
| `sources` | Optional array of 256 or fewer attributed source records. These records are separate from measured reads. A record does not cause the read of a file. | [Source attribution](04-write-a-configuration-script.md#supply-selection-context) |

The delivered selection-context map adds `measured` to the accepted document. Neither the caller nor the loader supplies that field.
Without a loader, selection receives the caller document with the
measurements, or `()` if there is no document.

A loader replaces the document that `select` receives, but `request.context`
still gives the caller's original input. Ravel does not merge the two
automatically. Ravel refuses a loader that returns nothing. See
[caller and delivered documents](04-write-a-configuration-script.md#supply-selection-context).

`measured` holds the successful reads in order. The caller's `--context`
document comes first when present, then successful `load_context` reads,
also repeated reads of one file. Top-level and selection reads appear
only in the notice's `reads` list.

The separate limit of this list is 256 records. The
compact encoding of the whole delivered document, with this list, must fit
`contextBytes`. Source attribution and measurement do not prove who the
author was or that a claim is true. See
[measurement and its limits](04-write-a-configuration-script.md#supply-selection-context).

### Nested data and measured records

This entry gives the shapes inside a selection-context document: assessments,
source records and the records of measured reads. Use it when Ravel refuses a
document as `context_invalid` and names a field.

JSON data allows null, booleans, finite numbers, strings, arrays and objects of
JSON values. Ravel cannot deliver cycles, executable values or other values
that are not JSON. An object whose only key is `$ravel` is ordinary JSON data. See
[JSON data in selection context](04-write-a-configuration-script.md#supply-selection-context).

| Type and field | Accepted value or meaning | Taught in |
|---|---|---|
| `Assessment.by` | Required nonblank string that names who made the judgement. | [Assessments](04-write-a-configuration-script.md#supply-selection-context) |
| `Assessment.value` | Required JSON value. Null is valid. Ravel refuses unknown assessment fields. | [Assessments](04-write-a-configuration-script.md#supply-selection-context) |
| `SourceRecord.name` | Required nonblank name, such as a path, URL or evidence label. It does not cause a read. | [Attributed sources](04-write-a-configuration-script.md#supply-selection-context) |
| `SourceRecord.sha256` | Optional digest of exactly 64 lowercase hexadecimal digits. At least this field or `version` is required. | [Attributed sources](04-write-a-configuration-script.md#supply-selection-context) |
| `SourceRecord.version` | Optional nonblank version string. At least this field or `sha256` is required. Ravel accepts both. | [Attributed sources](04-write-a-configuration-script.md#supply-selection-context) |
| `SourceRecord.bytes` | Optional nonnegative whole-number byte count. Ravel refuses unknown source-record fields. | [Attributed sources](04-write-a-configuration-script.md#supply-selection-context) |
| `MeasuredSource.name` | Required canonical file path of the source that Ravel measured. | [Measured sources](04-write-a-configuration-script.md#supply-selection-context) |
| `MeasuredSource.via` | Required `"--context"`, `"read_text"` or `"read_json"`. It identifies the read. | [Measured sources](04-write-a-configuration-script.md#supply-selection-context) |
| `MeasuredSource.bytes` | Required count of the bytes that Ravel actually read, before it parses any JSON. | [Measured sources](04-write-a-configuration-script.md#supply-selection-context) |
| `MeasuredSource.sha256` | Required lowercase SHA-256 digest of those bytes. | [Measured sources](04-write-a-configuration-script.md#supply-selection-context) |
| `ReadSource.name` | Required canonical file path that the host returns beside the content of a read. | [Host reads](04-write-a-configuration-script.md#supply-selection-context) |
| `ReadSource.bytes` | Required actual byte count that the host returns. | [Host reads](04-write-a-configuration-script.md#supply-selection-context) |
| `ReadSource.sha256` | Required lowercase SHA-256 of the bytes that the host read. The configuration script can put the three-field object in the attributed `sources`. | [Host reads](04-write-a-configuration-script.md#supply-selection-context) |

### Host capabilities

A configuration script cannot read a file, read the environment or run a
program by itself. When it needs one of these effects, it calls a
**host capability**, a `ravel::` function that Ravel gives to the script. For example, `ravel::read_json`
reads a JSON file, and `ravel::run_harness` runs a harness.

How does a host call work? The Rhai worker sends the request to the Ravel
CLI over a private socket and waits. The Ravel CLI checks the request, does
the work and sends the answer back. Then the script continues with the
returned value. The two reads, `env`, `harnesses`, `remaining` and
`run_harness` work in this way. The data helpers, `json_parse`, `json_encode`,
`json_check`, `uuid_v4`, `path_resolve` and `path_parent`, compute inside the
worker and send no request.

The `ravel` namespace supplies the functions below at the module's top level,
in `load_context` and in `select`.
Calls are synchronous, so only one host call runs at a time.
See [host capabilities](04-write-a-configuration-script.md#check-json-with-the-data-helpers).

| Function | Arguments, return value and failure |
|---|---|
| `ravel::read_text(path)` | Read a nonempty string path to a regular UTF-8 file. A relative path uses `request.cwd`. Return `#{ text, source }`, with the source record described above. |
| `ravel::read_text(path, max_bytes)` | The same read with an explicit positive integer byte limit, at most `contextBytes`. The default is the smaller of 65,536 and `contextBytes`. |
| `ravel::read_json(path)` | The same read and measurement, then a strict JSON parse. Return `#{ value, source }`. JSON depth is at most 100 levels. |
| `ravel::read_json(path, max_bytes)` | The same JSON read with the explicit limit described for `read_text`. |
| `ravel::json_parse(text)` | Parse a string as strict JSON. Return its Rhai value. Invalid JSON or depth beyond 100 levels throws `json_invalid`. |
| `ravel::json_encode(value)` | Return canonical JSON text, with sorted object keys and no whitespace. A value without a JSON form throws `json_unencodable`. |
| `ravel::json_check(value, schema)` | Check the JSON Schema subset below and return the value unchanged. Throw `schema_invalid`, `json_unencodable` or `json_mismatch` on failure. |
| `ravel::uuid_v4()` | Return a random version-4 UUID string in lowercase hexadecimal. If the operating system supplies no random bytes, throw `random_unavailable`. |
| `ravel::path_resolve(base, path)` | Both arguments are strings. Join a relative path to an absolute base, then normalise it. An absolute path replaces the base. A relative base throws `path_not_absolute`. |
| `ravel::path_parent(path)` | Return the parent of an absolute string path. Remove final slashes and the last segment. The parent of `/` is `/`. A relative path throws `path_not_absolute`. |
| `ravel::env(name)` | Read a string name from the caller's permitted environment. Return its string value or `()` when unset. Throw `env_not_granted` or `env_not_utf8` on failure. |
| `ravel::harnesses()` | Return the settings entries in name order, with the discovery fields below. Resolve programs without execution or network access. |
| `ravel::remaining()` | Return the milliseconds left, run budget and source count in the fields below. |
| `ravel::run_harness(request)` | Run the named settings entry with the request below. Return the run result after cleanup. A budget breach or cleanup failure ends evaluation. |

The read functions throw catchable maps for absent, non-regular, unreadable,
non-UTF-8 or invalid-JSON sources. The [data-error table](#host-data-errors)
gives each code. A map has `code`, `message` and a `name` when applicable.
A wrong argument type or malformed host request ends evaluation as `host_request_invalid`.
Rhai resolves function arity before the host function runs. A wrong argument count
therefore raises an ordinary function-call error instead.

A source consumes one count when its bytes arrive, even if UTF-8 or JSON conversion fails.
A read with no file consumes none. Repeated reads each consume one.
Only successful reads enter the measurement records.
Entry and module loads have their own module bound and do not consume this source count.

A read above its byte bound gives `source_too_large`.
A read after the source count reaches 256 gives `too_many_sources`.
Neither failure is catchable. A requested `max_bytes` above `contextBytes` also ends evaluation.
The source-count check occurs before the read, also a request for an absent file.

The path helpers read no file and resolve no symlink.
`path_resolve` removes empty and `.` segments and applies `..` without a step above `/`.
`path_parent` keeps the spelling that remains: `/a//b` gives `/a/`, and `/a/./b` gives `/a/.`.
See [data helpers](04-write-a-configuration-script.md#check-json-with-the-data-helpers) for their use.

### JSON values and schema checks

A configuration script often handles JSON: a selection-context document, a
file that it reads or the answer of a model. You may want to check that an
answer has the shape that your script needs before you use it. `ravel::json_check` does this.
This entry gives the conversion between Rhai values and JSON, and the schema
keywords that the check accepts.

Values that cross between the configuration script and Ravel use this conversion.
Arrays and maps nest at most 100 levels.
A conversion failure identifies the value's location, such as `result.args[2]`.
See [data conversion](04-write-a-configuration-script.md#check-json-with-the-data-helpers).

| Rhai value | JSON value or outcome |
|---|---|
| `()` | `null`. |
| Boolean | Boolean. |
| Signed 64-bit integer | Integer. A JSON integer outside this range becomes a float on input. |
| Finite float | Number. Ravel refuses NaN and infinities. |
| String | String. |
| Array | Array of converted values. |
| Map | Object of converted values. |
| Character, BLOB, function pointer, timestamp, range or custom type | Refused. These values have no JSON form at this boundary. |

`json_parse` rejects comments, final commas, single-quoted strings, NaN and text after the value.
`json_encode` sorts object keys by code point and writes no whitespace.
It escapes quotes, backslashes and control characters with JSON escapes.
Other characters remain UTF-8.
Finite floats use the shortest form that reads back exactly and retains their float type.
These helpers have different rules from Rhai's built-in `parse_json` and `to_json`.

`json_check` accepts only the keywords below. The root schema has level 1.
Each nested property schema or `items` schema adds one level, up to 32.
An unknown keyword or unsupported schema shape throws `schema_invalid`.
A mismatch throws `json_mismatch`. Its location can contain property names,
but its message omits property values.

| Keyword | Accepted value and effect |
|---|---|
| `type` | One of `object`, `array`, `string`, `number`, `integer`, `boolean`, `null`, or an array of those names. An integral float such as `3.0` matches `integer`. |
| `properties` | Object from property names to schemas. Applies to objects. |
| `required` | Array of property names that an object must contain. |
| `additionalProperties` | Boolean that permits or refuses object properties outside `properties`. |
| `items` | Schema for each array item. |
| `enum` | Array of scalar JSON values: strings, numbers, booleans or nulls. |
| `title`, `description` | Strings that the check ignores. |

Object keywords impose no requirement on an array, and `items` imposes none on an object.
Use `type` when the value must have a particular kind.
A schema checks data shape and permitted values. The configuration script still decides their meaning.

### Discovery and available budgets

You may want the configuration script to choose only a harness that is
installed on this computer. `ravel::harnesses()` reports the harness entries
of the settings without execution. The Ravel CLI finds each program as it finds a
selected program, and it tells the script whether it found an executable file.
It reports availability, not a recommendation or live model list.

Fragment: the value that `ravel::harnesses()` returned with the settings of
the [settings entry](#settingsjson), from the checkout build on 2026-10-29.
The configuration script printed it with `ravel::json_encode`. This block omits the command
and the other lines.

```text
config stdout: [{"available":true,"capabilities":["classify"],"name":"classifier","program":"fake-classifier","reason":null,"resolved":"/tmp/ch6/bin/fake-classifier"}]
```

Each map contains these fields.
See [discover harnesses](04-write-a-configuration-script.md#run-a-harness-during-evaluation).

| Field | Value |
|---|---|
| `name` | Settings entry name. Entries appear in name order. |
| `program` | The owner's program string. |
| `available` | Boolean: whether program resolution found an executable regular file. |
| `resolved` | Resolved path when available, otherwise `()`. |
| `reason` | `()` when available. Otherwise `not_found` or `not_executable`. |
| `capabilities` | The owner's string labels, unchanged. An omitted setting gives an empty array. |

You may want the configuration script to decide whether it has time and budget for one more
harness run. `ravel::remaining()` returns the following map for this evaluation.
It changes no budget. Each nested session in the final managed process has its own budgets.
See [evaluation bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Field | Value |
|---|---|
| `ms` | Milliseconds before the whole evaluation deadline. |
| `harnessRuns` | The run budget of the settings minus the runs that started. |
| `sources` | 256 minus the caller's context document and host reads whose bytes arrived. Failed UTF-8 or JSON conversion still consumes a source. |

### Harness-run request

You may want an LLM to choose the best harness, model, effort and prompt
additions for a kind and a prompt. You can do this with a harness run in your
configuration script. A **harness run** is one run of a program from your `harnesses`
settings, during evaluation. Its answer goes back to the script, not to the
caller. The script then makes its own choice.

How does the answer get back to the script? Follow one harness run:

| Order | Who acts | What happens |
|---|---|---|
| 1 | The configuration script | Calls `ravel::run_harness` with the name of a settings entry, the arguments and the text for standard input |
| 2 | The Rhai worker | Sends the request to the Ravel CLI over its private socket and waits |
| 3 | The Ravel CLI | Finds the entry's program, checks the run budget and runs the program in a new POSIX session with its own result channel |
| 4 | The harness run | Reads its standard input, writes its answer on standard output and exits |
| 5 | The Ravel CLI | Stops the processes that remain in the group of the run, completes cleanup and sends a map back to the worker |
| 6 | The configuration script | Reads `status`, `exitCode` and `stdout` from the map, then continues |

The harness run returns its answer to the configuration script before Ravel launches the final managed process.

![The Rhai worker requests a claude harness run from the Ravel CLI and waits. The CLI checks the budget and starts the run with its own result channel. The run reads standard input, writes its answer and exits. After cleanup the CLI returns a map to the worker. The configuration script selects, the CLI reaps the worker, writes the routing notice and launches the managed process.](figures/evaluation-harness.svg)

*Figure: The evaluation harness run and the managed process are separate launches.*

[Open the evaluation harness sequence at full size](figures/evaluation-harness.svg).

A harness run differs from the managed process. The managed process is the
final program, which Ravel launches after evaluation. Its result goes to the
caller. A harness run happens during evaluation, and its output goes only to
the configuration script.

`ravel::run_harness(request)` executes a program that the `harnesses` settings name.
It evaluates no configuration script and selects no final command.
The request is a map with these fields and no others.
See [run a harness](04-write-a-configuration-script.md#run-a-harness-during-evaluation).

| Field | Contract |
|---|---|
| `harness` | Required string name of a settings entry. An unknown name returns `unavailable`. |
| `args` | Required array of strings after the entry's program. Empty arrays and strings are valid. NUL is not. Each string is one argument. |
| `stdin` | Optional string for standard input. Without it, input is at end of file. |
| `timeoutMs` | Optional positive integer in milliseconds. Ravel lowers it to the time left for evaluation. Without it, all the time left applies. |

The configuration script supplies every argument and decides whether the prompt belongs in an argument or standard input.
Ravel builds no harness-specific argument and parses no harness output.
An [LLM call](02-actors-and-words.md#evaluation-and-supervision) uses this same operation.
The harness uses its own sign-in. Ravel holds no model credential or token budget.

Each run uses the caller's working directory, with a new POSIX session and no terminal.
It uses the managed-process environment rules and the caller's confinement options.
It receives `RAVEL_CONFIG_RUN` and a fresh `RAVEL_EXIT_FILE` for its own result channel.
A result sent there reaches only the configuration script, not the outer session's caller.

Each launched run consumes one from `budgets.harnessRuns`.
An unavailable call consumes none, even with no budget left.
After successful program resolution, a call with no budget left ends evaluation before launch.
A malformed request also ends evaluation, as `host_request_invalid`.
The script cannot catch these evaluation failures.

### Harness-run result

The call returns after the run's required cleanup.
A natural exit does not need an `end-session` value.
The configuration script interprets the status, output and optional result.

Fragment: the map that `ravel::run_harness` returned for a fake classifier,
from the checkout build on 2026-10-29. The classifier wrote `review` and
exited. The configuration script printed the map with `ravel::json_encode`. This block omits
the command and the other lines.

```text
config stdout: {"elapsedMs":389,"error":null,"exitCode":0,"result":null,"signal":null,"status":"exited","stderr":"","stdout":"review\n"}
```

The run exited with status 0 after 389 milliseconds. It did not use
`end-session`, so `result` was `()`, which JSON shows as `null`. Its answer
was on standard output. The value of `elapsedMs` changes in each run.

Every returned map has the following fields.
See [interpret harness results](04-write-a-configuration-script.md#run-a-harness-during-evaluation).

| Field | Value and meaning |
|---|---|
| `status` | One of the five statuses in the next table. |
| `exitCode` | Integer process exit code when available, otherwise `()`. A nonzero code is data for the script. |
| `signal` | Signal name such as `SIGTERM`, or `()`. |
| `stdout` | Captured text, at most 1 MiB of source bytes. Invalid UTF-8 becomes U+FFFD. |
| `stderr` | Captured text, with the same independent 1 MiB bound and UTF-8 replacement rule. |
| `result` | Text sent with `end-session`, or `()` when no result exists. An empty string is a sent result. |
| `elapsedMs` | Integer elapsed milliseconds. An unavailable call reports zero. |
| `error` | `()` except for `unavailable`, which supplies `#{ code, message }`. The codes appear below. |

| Status | Meaning |
|---|---|
| `exited` | The program exited. Inspect `exitCode`, output and any result. |
| `signalled` | A signal ended the program. Inspect `signal`. |
| `timed_out` | This run's deadline expired. Ravel stopped the run and completed cleanup. Evaluation can continue if its whole deadline remains. |
| `output_limit` | One output stream exceeded 1 MiB. Ravel stopped the run and kept the bounded output. Evaluation can continue. |
| `unavailable` | No program started. Output strings are empty, result and termination fields are `()`, and `error` explains why. |

A result sent with `end-session` starts the same 2-second grace and 5-second TERM grace as a final managed process.
A per-run timeout requests TERM immediately, with KILL after the kill grace if necessary.
Cleanup covers descendants that stay in the run's process group.
A group that survives, a failed reap or failed required temporary-directory cleanup ends evaluation as `harness_cleanup_failed`, status 5.
The script cannot catch that failure.

### Host data errors

Your configuration script can handle some failures of a host call itself. For example, a
missing file can mean “use the default route”.

These codes reach the script as data. They have no independent CLI exit status.
A thrown helper error can reach `catch`.
If it escapes, Ravel reports `config_import_failed`, `context_loader_failed` or `selection_threw`, as the evaluation phase requires.
Unavailable-run errors instead appear in `run.error`.
See [catchable errors](04-write-a-configuration-script.md#let-a-repository-choose-an-arrangement) and [harness outcomes](04-write-a-configuration-script.md#run-a-harness-during-evaluation).

| Code | Origin and remedy |
|---|---|
| `source_missing` | A measured read found no file. Supply the file or handle its absence. |
| `source_not_regular` | The source is not a regular file. Supply a regular file, not a directory or FIFO. |
| `source_unreadable` | The read, canonical path or encoded reply is unusable. Check permissions, path encoding and source content. |
| `source_not_utf8` | Source bytes do not decode as UTF-8. Correct the file encoding. The read still consumes one source. |
| `source_invalid_json` | `read_json` found invalid JSON or depth beyond 100 levels. Correct the data. The read still consumes one source. |
| `json_invalid` | `json_parse` found invalid strict JSON or excessive depth. Correct the text. |
| `json_unencodable` | A value given to `json_encode` or `json_check` cannot cross the JSON boundary. Correct the named value or its depth. |
| `schema_invalid` | `json_check` received an unsupported schema. Use the listed keywords and depth bound. |
| `json_mismatch` | A value fails `json_check`. Inspect its named location and handle the unusable value. |
| `path_not_absolute` | A path helper needs an absolute base or argument. Supply the caller's absolute directory or another absolute path. |
| `env_not_granted` | `ravel::env` cannot read that name. Grant it explicitly if permitted, or remove the read. |
| `env_not_utf8` | The permitted caller variable contains non-UTF-8 bytes. Correct the caller value. |
| `random_unavailable` | `uuid_v4` received no random bytes from the operating system. Handle the failure or correct the OS resource problem. |
| `unknown_harness` | `run_harness` names no settings entry. Correct the entry name or add the intended entry. |
| `harness_not_found` | The configured program does not resolve. Correct its path or the caller's `PATH`. |
| `harness_not_executable` | The configured program is not executable. Correct its file type or execute permissions. |
| `harness_launch_failed` | The runner could not start the configured program. Inspect the message, interpreter and execution environment. |

## Module resolution

### Import resolution

You may want to keep a route table or a helper function in one file and use it
from several configuration scripts. A Rhai `import` statement does this.

`import "PATH" as NAME;` loads a file module.
A relative import resolves from the directory of the file that imports it.
An absolute import uses its own path.
An entry that is a symlink uses the directory of its real file.
See [import other Rhai files](04-write-a-configuration-script.md#import-other-rhai-files).

| Import property | Rule |
|---|---|
| Extension | Ravel appends `.rhai` when the path has no extension. |
| File | Readable regular UTF-8 file, at most 1 MiB, with a canonical UTF-8 name. |
| Repeated import | Each distinct canonical file loads once per evaluation. Repetition does not consume another module. |
| Count | At most 64 distinct imported modules, besides the entry. |
| Cycle | Ravel refuses an import cycle as `config_import_failed`. |
| Measurement | The notice lists the entry first, then modules in first-import order, with canonical name, bytes and SHA-256. |
| Embedded modules | None. Every import names a file. The `ravel` host namespace needs no import. |

A missing, unreadable, oversized or cyclic module gives `config_import_failed`.
A syntax error gives `config_parse_failed` with its source location.
The importer cannot catch a failed host import and continue selection.

### Engine and conversion bounds

What stops a configuration script that loops for ever or makes too much data? The Rhai
engine counts the operations, the call depth and the size of the data of a
script. Ravel sets a limit for each of these counts. When the script reaches a
limit, evaluation ends and Ravel launches no final managed process.

Ravel uses the same bounds in both development and release builds.
An engine limit ends evaluation as `script_limit`, status 3.
The worker heap bound gives `memory_limit`, status 3.
A `try` block cannot catch either, also inside an array or map callback.
See [evaluation bounds](04-write-a-configuration-script.md#set-bounds-and-environment-grants-in-the-settings).

| Bound | Fixed value and consequence |
|---|---|
| Operations | 100,000,000 across evaluation. |
| Function call depth | 64 levels. |
| Expression depth | 64 at top level, 32 inside a function. |
| String size | 16,777,216 bytes (16 MiB). |
| Array size | 1,048,576 elements, with nested elements. |
| Map size | 1,048,576 properties, with nested properties. |
| Variables | 16,384 at one time. |
| Script functions | 4,096. |
| Worker heap | 1,073,741,824 live allocated bytes (1 GiB). |
| Worker evaluation stack | 64 MiB. A worker abort is an operational failure, not a successful selection. |
| Imported modules | 64 files besides the entry. A 65th distinct module gives `config_import_failed`. |
| Entry or module source | 1,048,576 bytes per file. Entry failure gives `config_unreadable`, and module failure gives `config_import_failed`. |
| Encoded host-read reply | 19,922,944 bytes (19 MiB), with JSON escapes and record fields. Excess throws catchable `source_unreadable` and consumes one source. |
| JSON conversion depth | 100 levels of arrays and maps. The boundary reports a data-conversion error at the value's location. |
| JSON Schema depth | 32 levels, with the root at level 1. Excess gives catchable `schema_invalid`. |
| Host calls at one time | One, because each host call is synchronous. |
| Harness output | 1,048,576 bytes per stream. Excess returns run status `output_limit` after cleanup. |
| One harness deadline | The smaller of its positive `timeoutMs` and the whole time left. Expiry can return `timed_out`. |

Rhai checks container size when a call returns a value.
An in-place change can exceed that container limit before such a check.
The worker heap bound still applies.
At the whole evaluation deadline, Ravel ends evaluation and stops the worker, even if a native call does not yield.

At cancellation or the whole deadline, Ravel abandons a read and does not wait for its reply.
It stops an active harness run before it stops and reaps the worker.
Ravel waits at most 8 seconds for harness cleanup: the 5-second kill grace, 2 seconds for a failed reap and 1 second for group confirmation.
If the runner has not completed cleanup by that limit, Ravel reports `harness_cleanup_failed`.
Worker teardown follows and has its own 1-second grace before KILL.

Kernel stalls can still delay process exit. These bounds are not a real-time guarantee.

## Example configuration scripts

You may want to begin from a configuration script that already runs, instead
of an empty file. Ravel installs three example configuration scripts for this
purpose.

The examples are editable Rhai files under `share/ravel/examples/`.
Their checkout source is `crates/ravel/examples/`.
That directory also holds `harness-table.rhai`, an example for Ravel's own tests, which Ravel does not install.
Copy the files that your chosen example imports, then replace the placeholder commands and labels.
Ravel installs no personal configuration script and has no `init` command.
See [copy a configuration script and its files](04-write-a-configuration-script.md#copy-a-configuration-script-and-its-files).

| File | Behaviour and dependencies |
|---|---|
| `static.rhai` | Exports `SCHEMA_VERSION` 3, `VERSION` `ravel/examples/static 3` and `ROUTES`. `agent(effort)` constructs a route, and `select_route(routes, request)` applies it. `select` uses the exported table. |
| `sample.rhai` | Imports adjacent `static.rhai`. Version `owner-choice-1`. Reads `.ravel-choice` in `request.cwd` and applies the choices below. |
| `llm.rhai` | Imports adjacent `static.rhai`. Version `llm-triage-1`. Calls the `claude` harness entry with a 20,000 ms timeout and a schema restricted to table keys. |

`static.rhai` selects `my-agent-wrapper`, provider `your-provider` and model `your-model`.
Its arguments are `--model`, the model, `--effort`, the effort, and the unchanged prompt.
The table maps `question` to `low`, `bugfix` to `medium`, and `feature` to `high`.
It maps `migration` and `architecture` to `xhigh`.
A missing kind returns the owner refusal `incomplete_mapping`.
These names belong to the sample, not Ravel's vocabulary.

The choice sample reads at most 4,096 bytes and splits names at whitespace.
It preserves their order and duplicates, and searches no parent directory.
Successful reads appear in the notice.
See [let a repository choose an arrangement](04-write-a-configuration-script.md#let-a-repository-choose-an-arrangement).

| Choice input | Outcome |
|---|---|
| Missing file, also a dangling symlink | Use `default`. |
| Exactly `default` | Return the starter selection unchanged. |
| Exactly `careful` | Preserve program, model and provider. Set effort to `high`, rebuild the arguments, and use reason `careful choice`. A starter refusal stays a refusal. |
| Empty file or multiple offered names | Return owner code `one_choice`. |
| An unoffered name | Return owner code `choice_unoffered`. |
| Unreadable, non-regular or invalid-UTF-8 file | Return owner code `choice_unreadable`. |
| More than 4,096 bytes, or a read limit above `contextBytes` | End evaluation as `source_too_large`. The sample cannot catch a bound breach. |

`llm.rhai` needs a `claude` settings entry and at least one harness run in the budget.
It accepts only an allowed table key from a successful structured answer.
After an unavailable, unsuccessful or invalid answer, it uses the caller's kind instead.
An exhausted budget or whole deadline still ends evaluation.
The [Claude Code recipe](04-write-a-configuration-script.md#ask-a-model-through-claude-code)
teaches that call, and the [Codex recipe](04-write-a-configuration-script.md#ask-a-model-through-codex)
teaches its file-based schema alternative.

## Installed layout

Where are the files of an installed Ravel? You may want the executable for a
caller's configuration, or the example configuration scripts to copy.

`PREFIX` is the installation root, independent of the owner's `HOME`.
For Homebrew, `brew --prefix ravel` gives this path.
Ravel 2.0.0 installs one executable with the Rhai engine inside it.
It needs no separate language runtime or source checkout.
See [install Ravel](03-a-complete-session.md#install-ravel-and-check-its-version).

| Path below `PREFIX` | Purpose |
|---|---|
| `bin/ravel` | The Rust CLI, linked runner and embedded Rhai engine. Ravel re-executes this binary for its separate Rhai worker. |
| `share/ravel/examples/static.rhai` | Editable starter table. |
| `share/ravel/examples/sample.rhai` | Editable choice script, with an import of `static.rhai`. |
| `share/ravel/examples/llm.rhai` | Editable model-assisted script, with an import of `static.rhai`. |
| `share/ravel/notices/THIRD-PARTY-NOTICES.md` | The licence notices of the Rust dependencies inside `bin/ravel`. |

The installation, the release archive and the Homebrew formula carry the same notice file.
The archive carries the project `README.md` and `LICENSE` at its root.
The [delivery chapter](11-deliver-ravel.md) gives release archive details.

The following figures show source responsibilities and installed files.
Ravel's runner is library code in the CLI, not a service or another executable.
The Rhai worker is a separate process of the same binary.
Ravel completes host calls, reaps the worker and removes its directory before the final managed launch.

> **Figure — Source responsibilities:** This C4 view groups internal code by process responsibility.
> The same linked runner supervises evaluation harness runs and the final managed process at different times.
> It does not define a sandbox or a supported library interface.

![Source responsibilities: crates/ravel checks requests, controls host calls and decides the final outcome. crates/keyed-launch supplies the runner inside that CLI process. A separate Rhai worker uses the same binary to evaluate the configuration script and request host capabilities. The runner supervises each evaluation harness run, whose data returns to the configuration script. Every host call and worker cleanup ends before the runner launches the final managed process.](figures/source-components.svg)

> **Figure link:** [Open the source-responsibility view at full size](figures/source-components.svg).

> **Figure — Installed files:** This deployment view groups installed files by directory.
> One executable supplies both the CLI and the separate Rhai worker. The three Rhai examples remain editable files.
> Directory outlines do not show concurrent process lifetimes.

![Installed files: bin/ravel contains the CLI, linked runner and Rhai engine. The CLI starts the same binary as a Rhai worker. Under share/ravel, examples holds three editable files: static.rhai, sample.rhai and llm.rhai. The notices directory holds THIRD-PARTY-NOTICES.md for Rust dependencies. The archive adds README.md and LICENSE at its root.](figures/installed-files.svg)

> **Figure link:** [Open the installed-file view at full size](figures/installed-files.svg).

> **Note:** Ravel supports no separate component. Another program invokes the CLI as a child process.
> The private runner can change process-wide signal state and does not restore that state on return.
> [Chapter 1](01-what-ravel-is-for.md#choose-a-route-through-the-book) explains the supported integration route.

> **Note:** Native x86-64 Linux qualification remains open.
> The row for a different host in
> [Know when Ravel cannot guarantee cleanup](05-running-sessions.md#know-when-ravel-cannot-guarantee-cleanup)
> distinguishes native measurements from emulation.
