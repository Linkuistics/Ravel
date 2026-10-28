# Run sessions

Your owner policy is a file of Rhai code that selects a program for a request.
Rhai is a small computer language that Ravel contains. Now you want to run a
session, one run of `ravel start-session`, from a script or your terminal.
You may also use an orchestrator, a program that runs sessions as steps of its
own work.

You want to know what each outcome means for your program. This chapter shows
how to give a request and how to read the result and the routing notice. The
routing notice tells the caller, the person or program that requested the
session, which provider, model and effort the policy chose. This chapter also
shows how to run a session inside another session and identify each outcome.
It also explains confinement, which restricts a program's filesystem access
and environment.

First, the words that this chapter uses on every page:

- A [session](02-actors-and-words.md#caller-and-owner) is one run of
  `ravel start-session`, from the request to the result.
- The [caller](02-actors-and-words.md#caller-and-owner) is the person or
  program that runs that command, for example an orchestrator. The caller gives
  a prompt and a [kind](02-actors-and-words.md#caller-and-owner). A kind is a
  short token that names the sort of work, such as `review`.
- The [owner](02-actors-and-words.md#caller-and-owner) is the person who
  controls the configuration of Ravel for the account. The owner can be you.
- The [owner policy](02-actors-and-words.md#caller-and-owner) is a file of
  Rhai code that the owner writes. Rhai is a small computer language, and its
  engine is part of the `ravel` executable. Ravel gives the request to the
  policy. The policy then selects a program and its arguments, or it refuses
  the request.
- The
  [managed process](02-actors-and-words.md#the-managed-process-and-its-result)
  is the program that the policy selected. It is usually a
  [harness](02-actors-and-words.md#the-managed-process-and-its-result), such as
  Claude Code or Codex, the program in which an LLM works on the prompt. Ravel
  launches the managed process and supervises it.
- The [result](02-actors-and-words.md#the-managed-process-and-its-result) is
  the text that a program in the session sends with `ravel end-session`. Ravel
  does not interpret it. Thus, the book calls it an opaque result.
- The [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) is
  the `ravel` command-line program. It evaluates the policy in a separate
  process, launches the selected program and returns the result.

<!-- tc: depart synonym-rotation because launch names process creation, while start also names the beginning of output -->
What happens between your command and your result? The table restates the
order from
[chapter 1, “From a request to a result”](01-what-ravel-is-for.md#from-a-request-to-a-result).
Each section of this chapter adds detail to one or more rows. Read the
sections in order, or go to the row that you need:

| Order | Who acts | What happens | Section that teaches it |
|---|---|---|---|
| 1 | The caller | Runs `ravel start-session` with a kind, a prompt and optional caller data | [Supply the prompt and caller data](#supply-the-prompt-and-caller-data) |
| 2 | The owner policy | Selects a program and its arguments, or refuses the request | [Chapter 4](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| 3 | The Ravel CLI | Writes the text routing notice on the caller's standard error. With `--json`, the notice follows the supervision attempt. | [Read notices separately from results](#read-notices-separately-from-results) |
| 4 | The Ravel CLI | Launches the selected program, with or without confinement, and supervises it | [Confine the managed process](#confine-the-managed-process) |
| 5 | The selected program | Does its work, which can include an inner session, and sends a result with `ravel end-session` | [Keep nested sessions separate](#keep-nested-sessions-separate) |
| 6 | The Ravel CLI | Stops the program and its process group, completes cleanup, writes the result on standard output and exits with a status | [Accept a result only after success](#accept-a-result-only-after-success) and [Interpret the exit status](#interpret-the-exit-status) |

A cancellation can stop the order at rows 2 to 6.
[Cancel during selection or supervision](#cancel-during-selection-or-supervision)
explains what you receive then. Did you come here for an exit code? Go to the
[status table](#interpret-the-exit-status). It gives each normal status of the
Ravel CLI and the three cancellation signals that Ravel handles, with an action
for each.

## Run the examples of this chapter

Every complete example in this chapter uses fake programs. A **fake agent** is
a small shell script that stands where a harness stands. Most fake agents run
`ravel end-session` themselves, as an LLM in a harness does when the prompt
tells it to. The examples use real policy evaluation. No example needs an agent
account or a model service.

<!-- tc: depart synonym-rotation because a version check compares output, while confirmation establishes completed cleanup -->
First, follow chapter 3 to
[install Ravel and check its version](03-a-complete-session.md#install-ravel-and-check-its-version).
Then use a fresh directory for each complete example, and create the labelled
files there. Run the commands of the example in that directory. Each command
line has these parts:

| Part of the command | What it does |
|---|---|
| `mkdir -p home tmp` | Makes a private home directory and a private temporary directory |
| `chmod +x bin/*` | Makes the fake programs executable |
| `env -i` | Runs the command with an empty environment. Your own variables, such as a `RAVEL_EXIT_FILE` of an enclosing session, do not enter the example. |
| `HOME="$PWD/home"` and `TMPDIR="$PWD/tmp"` | Give Ravel the private directories. Thus, your personal policy and settings do not enter the example. |
| `PATH="$PWD/bin:$PATH"` | Puts the fake programs first on the search path |
| `ravel start-session --config policy.rhai` | Runs a session with the policy file of the example |

The book checker runs the same examples with the built Ravel CLI. It compares
each output with the output that the book shows.

## Supply the prompt and caller data

What does the selected program receive, and how can you give the policy more
facts about the task? You may want the LLM to receive your instructions
exactly as you wrote them. You may also want the policy to choose a route from
facts such as a profile name or a task file. This section shows the inputs of
`start-session` that do these things.

Pass exactly one of `--prompt TEXT` and `--prompt-file PATH`. Ravel gives the
text to the policy unchanged, also when it is an empty string or ends with
newlines. It never reads a prompt from the standard input of a terminal. A
prompt is UTF-8, contains no NUL and has a maximum size of 1 MiB. Ravel does
not truncate input that is not valid, but stops the invocation with status 2.

| Input | When to use it | What Ravel reads |
|---|---|---|
| `--prompt TEXT` | A short prompt that you write in the command | The argument, as the shell gives it to Ravel |
| `--prompt-file PATH` | A prompt whose exact text is important, for example with trailing newlines | The file, relative to the caller's working directory |

Ravel adds no instructions to end the session. An interactive harness does
not exit when its LLM completes the task. It waits for more input. Thus, your
prompt tells the LLM to run `ravel end-session RESULT` when the task is
complete. `RESULT` is a placeholder for the text to send. Your prompt gives
the permitted values and tells the LLM when to use each value.

The argument limits of the operating system can be smaller than 1 MiB. These
limits also apply when the policy puts a prompt from a file into the
arguments of the selected program.

Why is a file safer for exact text? Shell command substitution removes
trailing newlines. Thus, `--prompt "$(cat prompt.txt)"` can change the text
before Ravel receives it. `--prompt-file prompt.txt` reads the file directly.
Ravel changes no text before the policy receives it. Then the policy makes the arguments of the
selected command and decides how to give the prompt to that program.

The [kind](02-actors-and-words.md#caller-and-owner) is a required caller
token that is not empty. Ravel has no catalogue of kinds and does not get a
kind from a file name. A policy can route `review`, refuse it or treat it as
any other kind. The caller and the policy author agree on these tokens.

<!-- tc: depart synonym-rotation because the sentence keeps the owner's word "modify" from finding F3 -->
The caller can also use the kind to extend or modify the prompt. For example,
if the harness supports skills, the caller can add a line to the prompt for
the kind `review`. That line tells the LLM which skill to load. The policy can
also put text before the prompt for a kind. Ravel gives neither the kind nor
the prompt a meaning.

These inputs give data to the policy. They do not tell Ravel to read a task
or to give the task a meaning:

| Input | What the policy receives |
|---|---|
| `--param profile=careful` | `request.params.profile` with the string `careful`. Each name is not empty, contains no `=` and occurs one time. The value can be empty or contain more `=` characters. The limit for all names and values together is 65,536 UTF-8 bytes. |
| `--task-file tasks/check.md` | An absolute UTF-8 `request.taskFile`. Ravel makes it from the given path and the caller's working directory. Ravel does not read that file, and the file does not have to exist. Ravel gets no kind or identity from it. |
| `--task-id example-17` | An opaque `request.taskId`. It is UTF-8, is not empty and has a maximum of 1,024 bytes. Ravel does not search for a stored task. |
| `--context caller-context.json` | A version-1 JSON document that Ravel measures and validates. If the policy has no loader, the document becomes the second argument to `select`. A loader can make a different document for the policy. <!-- tc: depart synonym-rotation because validation enforces a data shape, while confirmation establishes cleanup --> |

That last document is
[policy context](02-actors-and-words.md#three-different-kinds-of-information),
which is data that the policy uses to choose the route. The book uses the word
“context” for three different things, and it always names which one:

| Name | What it is | Who uses it |
|---|---|---|
| Policy context | The version-1 JSON document from `--context`, or the document that the policy's `load_context` function makes | The policy, to make its choice |
| [Session-result context](02-actors-and-words.md#three-different-kinds-of-information) | The path of the result file, which Ravel puts in the `RAVEL_EXIT_FILE` environment variable of the program that it launches | `ravel end-session`, to send the result |
| Working directory, `cwd` | The current directory of the caller | The selected program, which runs there |

Chapter 4 teaches
[policy context and loaders](04-writing-an-owner-policy.md#supply-policy-context)
and [bounds and environment grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

Ravel runs your policy in the
[policy worker](02-actors-and-words.md#evaluation-and-supervision), a
separate process. This work is policy evaluation. The policy can ask the Ravel
CLI to do work for it, for example to read a file or to run a harness. It does this through a
[host capability](02-actors-and-words.md#evaluation-and-supervision), a
`ravel::` function such as `ravel::read_text` or `ravel::run_harness`. Each such
request is a **host call**.

How long can evaluation take? The evaluation deadline defaults to 30 seconds
and covers every host call. An
[LLM call](02-actors-and-words.md#evaluation-and-supervision) is a
harness run that asks a model a question. It uses that same deadline.

The owner can set `timeoutMs` in settings. For one invocation, `--timeout-ms`
replaces that value. The permitted range is 1,000–600,000 ms. This bound
applies to policy evaluation, not to the time that the managed process works.

The owner also sets `budgets.harnessRuns`, from 0 to 16, in settings. This
[evaluation budget](02-actors-and-words.md#evaluation-and-supervision)
limits the number of harness runs in one evaluation. Its default is 0, and
callers have no flag that raises it.

The selected program runs in the caller's working directory. The form of its
program path sets the base that Ravel uses to find the program:

| Selected program | Resolution base |
|---|---|
| Absolute path | The path itself |
| Relative path that contains `/` | The caller's working directory |
| Bare program name | The caller's `PATH` |

Ravel gives the argument strings directly to the program, without shell word
splitting or expansion.

How does the managed process get its environment? A child process gets a copy
of the environment of its parent. Thus, an unconfined managed process gets the
environment of the Ravel CLI, which is the caller's environment. Ravel makes
two changes to that copy before the launch:

- It removes `RAVEL_RUN_ID`, `RAVEL_STATE_DIR`, `HARNESS_DISPATCH_EXIT_FILE`,
  `HARNESS_DISPATCH_RUN_ID` and `HARNESS_DISPATCH_STATE_DIR`. These variables
  are controls of an enclosing session or of another launcher.
- It replaces `RAVEL_EXIT_FILE` with the path of the result file of the new
  session.

Thus, `end-session` in the program and its children finds only the path of
the session that Ravel made for them. Put the installed `ravel` on the `PATH` of the managed
process. As an alternative, write the absolute path of `ravel` in the prompt's
instructions to end the session. A confined managed process gets a smaller
environment, which [Confine the managed process](#confine-the-managed-process)
describes.

Complete example: pass a prompt from a file, with its final newline, and
caller data. The policy uses `profile` as an argument. It does not use the
task fields or the policy context. The fake agent sends the prompt itself as
the result, so the result shows what the agent received. Create the
labelled files in a new directory. Make `bin/echo-prompt` executable before
you run the command.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "prompt-demo";

fn select(request, context) {
    let profile = if request.params.contains("profile") {
        request.params.profile
    } else { "default" };
    #{
        status: "selected",
        program: "echo-prompt",
        args: [request.prompt, profile],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Show the caller's prompt unchanged",
    }
}
```

**`bin/echo-prompt`**

```sh
#!/bin/sh
set -eu
printf 'profile=%s\n' "$2"
ravel end-session -- "$1"
```

**`prompt.txt`**

```text
Review the public behaviour.
When you finish, send the agreed result.
```

**`caller-context.json`**

```json
{"schemaVersion":1,"summary":"Check the public interface."}
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind review --config policy.rhai --prompt-file prompt.txt --param profile=careful --task-file tasks/check.md --task-id example-17 --context caller-context.json
```

**Standard output**

```text
Review the public behaviour.
When you finish, send the agreed result.
```

**Standard error includes**

```text
ravel: starting provider local, model fake, effort none for kind "review"
profile=careful
```

**Exit status**

```text
0
```

The table follows this example from the command to the result that the caller
receives:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | The caller | Runs `start-session` with the prompt file, one parameter, the task fields and the policy context |
| 2 | The owner policy | Selects `echo-prompt`, with the prompt and `careful` as its two arguments |
| 3 | The Ravel CLI | Writes the routing notice `ravel: starting provider local, model fake, effort none for kind "review"` on standard error |
| 4 | `echo-prompt` | Writes `profile=careful` on its standard output. Ravel sends that output to the caller's standard error. |
| 5 | `echo-prompt` | Runs `ravel end-session` with the prompt as the result |
| 6 | The Ravel CLI | Stops the program's process group, completes cleanup and writes the prompt unchanged on standard output. It exits with status 0. |

The final newline came from `prompt.txt`. Ravel added no newline. The
`printf` output of the fake agent goes to the caller's standard error, because
Ravel uses standard output only for the result. This example does not need a
`tasks/check.md` file.

## Accept a result only after success

When can your program trust the result? You may want to act on the result as
soon as a program in the session sends it. But Ravel can still fail after that
moment, for example during cleanup. Thus, keep the result bytes and the status
of `start-session` together, and accept the result only with status 0.

Two operating-system words explain the conditions of status 0:

| Word | What it means | Why status 0 depends on it |
|---|---|---|
| [Process group](03-a-complete-session.md#launch-the-managed-job) | A set of processes that the operating system can signal together. Ravel puts the managed process in a new group. Its descendants stay in that group unless they leave it. | When the group is gone, no process of the session that stayed in the group still runs. |
| [Reap](03-a-complete-session.md#send-the-result-and-finish-the-job) | A parent collects the exit status of its child process. | After the reap, Ravel knows that the managed process ended. |

Status 0 means that all of these conditions are true:

- A program in the session sent a valid result.
- Ravel reaped the direct child, which is the managed process.
- Ravel confirmed that the process group of the managed process is gone. <!-- tc: depart synonym-rotation because confirmation establishes completed cleanup, while a check compares a value -->
- The required cleanup succeeded.
- The final write and flush of the result succeeded.

An empty sent result is a successful result with zero bytes. If a managed
process exits with status 0 and sends no result, the status is 1. Ravel then
returns no result to the caller. But if a program sends a result and the child
then exits with a nonzero status, `start-session` can still give status 0.
Ravel's status shows whether Ravel delivered a sent result after cleanup. It
does not show whether the work in the prompt was correct.

`end-session` takes one UTF-8 argument with a maximum size of 1 MiB. Quote
it, and put `--` before a value that looks like an option, such as `--help`.
Ravel writes an empty, multiline or Unicode value unchanged to standard
output and does not interpret it. The operating
system can apply a smaller limit for command arguments before `end-session`
runs.

How does Ravel receive the result? `end-session` writes it through the
[result channel](02-actors-and-words.md#the-managed-process-and-its-result),
the private directory and result file that Ravel made for the session.

Ravel keeps the first complete result that it receives. When that first result
arrives, Ravel starts to stop the <!-- tc: depart synonym-rotation because start names the beginning of the shutdown here, while launch names process creation -->
[harness](02-actors-and-words.md#the-managed-process-and-its-result). Thus, a
second `end-session` call and the shutdown are in a race: either can occur
first. For example, another process in the group can send a result before
Ravel stops that process. Ravel keeps the first result, even if the second
command succeeds.

Complete example: compare three outcomes. In the first, the program sends a
result and then tries a second call before it exits with status 7. The
shutdown can stop the program before that call or that exit. In the second,
the program sends an empty result. In the third, it sends no result and exits
naturally with status 0. Create these files in a new directory and make
`bin/outcome` executable.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "outcome-demo";

fn select(request, context) {
    #{
        status: "selected",
        program: "outcome",
        args: [request.kind, request.prompt],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Compare a sent result with natural exit",
    }
}
```

**`bin/outcome`**

```sh
#!/bin/sh
set -eu
if [ "$1" = natural ]; then
  exit 0
fi
ravel end-session -- "$2"
ravel end-session -- "ignored second value"
exit 7
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind send --config policy.rhai --prompt '--help'
```

**Standard output, without a final newline**

```text
--help
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind send --config policy.rhai --prompt ''
```

**Standard output**

```text
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind natural --config policy.rhai --prompt ''
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (session_result_missing, stage exec): the managed process ended without a result (exit status: 0)
```

**Exit status**

```text
1
```

The table follows each command to what the caller receives:

| Command | What the program does | What the caller receives |
|---|---|---|
| `--kind send --prompt '--help'` | Sends `--help`, then tries to send a second value and to exit with status 7 | The result `--help` and status 0. Ravel kept the first result. |
| `--kind send --prompt ''` | Sends an empty result | An empty result and status 0 |
| `--kind natural --prompt ''` | Exits with status 0 and sends no result | No result, the diagnostic `session_result_missing` and status 1 |

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

`end-session` returns before the enclosing session finishes its cleanup.
When Ravel detects the sent result, a job that still runs gets 2 seconds to
finish. Then Ravel sends TERM, waits for up to 5 more seconds and sends KILL
if necessary. These waits end early if the direct child exits. Ravel then
stops the other group members before it reaps the child. Those members do not
get a new 2-second wait.

Send the result after the work that has to finish, because Ravel can stop
code that comes after `end-session`.

For an automated caller that needs trailing newlines intact, redirect standard output
to a file. Read the file only after you check the status. Shell command
substitution removes trailing newlines. Use it only when the caller's result
convention accepts this change. A failed final write can leave only the first
part of the result in the file, so the bytes alone do not prove success.
Ravel has no persistent store for results, so it cannot recover a result
after a failed delivery.

Fragment: capture the unchanged result and the status. This fragment omits the
policy, the prompt file and how the caller handles each successful value or
failure.

```sh
if ravel start-session --kind review --config policy.rhai --prompt-file prompt.txt >result.txt; then
  status=0
else
  status=$?
fi
```

## Keep nested sessions separate

You may want the program of one session to start another session. For
example, an orchestrator can run as a harness and ask Ravel for a review
session as one step of its work. Then it uses the review's result in its own
work. The program that asks is the caller of the inner session. How does each
`end-session` reach the right session?

The answer is the path in `RAVEL_EXIT_FILE`.
[Chapter 1, “How `end-session` finds its session”](01-what-ravel-is-for.md#how-end-session-finds-its-session),
explains it, and this section restates it for nested sessions. Before each
launch, Ravel makes a
[result channel](02-actors-and-words.md#the-managed-process-and-its-result),
which is a private temporary directory with a path for one result file. Ravel
puts that path in the `RAVEL_EXIT_FILE` environment variable of the program
that it launches. The book calls this path the
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).

A child process gets a copy of the environment of its parent. Thus, each
descendant of the managed process inherits the same path, unless its parent
changes the environment. `ravel end-session` finds the path in its own
environment. `end-session` takes
no session ID and no target argument. An inner `start-session` gives a new path
only to its own managed process:

| Process | Which process starts it | Its `RAVEL_EXIT_FILE` | What its `end-session` call sends |
|---|---|---|---|
| Outer `ravel start-session` | The caller | None, unless the caller itself runs in a session | It does not call `end-session` |
| Outer managed process | The outer Ravel CLI | The outer path | The result of the outer session |
| Inner `ravel start-session` | The outer managed process | The outer path, inherited | It does not call `end-session` |
| Inner managed process | The inner Ravel CLI | A new inner path | The result of the inner session. Ravel returns it to the outer process after the inner cleanup. |
| Outer managed process, after the inner session | The outer Ravel CLI | The outer path, unchanged | The result of the outer session |

The figure uses the paths of the separate run in chapter 1.

Each child inherits its parent's path unless its parent changes it.
Ravel sets a new path for the inner harness.

![A caller starts outer start-session without a result path, and its policy worker has no path. Ravel gives the outer harness an outer path, which its shell and end-session inherit. The shell starts inner start-session with the outer path. The inner harness receives a new inner path, which its end-session inherits. Each end-session writes to its own path. An evaluation harness run gets a separate path and returns its result to the policy.](figures/inherited-result-path.svg)

*Figure: The nested run gives each session's children its own result path.*

[Open the inherited result paths at full size](figures/inherited-result-path.svg).

Here is a real run of the nested example below. Each fake agent also printed
its own `RAVEL_EXIT_FILE` on standard error. The outer agent printed its value
before and after the inner session.

Fragment: standard error of a real nested session on macOS with
`TMPDIR=/tmp`. This block omits the cleanup reports. The random parts of each
path change in each run. macOS gives `/tmp` the real path `/private/tmp`.

```text
ravel: starting provider local, model fake, effort none for kind "outer"
outer: RAVEL_EXIT_FILE=/private/tmp/ravel-session-DW2gTg/signal-c8008b48ae03d4d4a5e671806d997d6a
ravel: starting provider local, model fake, effort none for kind "inner"
inner: RAVEL_EXIT_FILE=/private/tmp/ravel-session-47e8Ar/signal-803d525c20ad7bff18912ce6edb1d662
outer: RAVEL_EXIT_FILE=/private/tmp/ravel-session-DW2gTg/signal-c8008b48ae03d4d4a5e671806d997d6a
```

Ravel made a directory under `TMPDIR` for each session. The inner agent
received the new path. The outer agent kept its own path, also after the inner
session. Thus, each `end-session` goes to the session that encloses it.

When the inner invocation returns, the outer process decides what to send for
the outer session. Ravel does not change the inner result into an outer
result. The outer process does this itself. When the inner process sends its
result, the outer session does not finish.

An evaluation
[harness run](02-actors-and-words.md#evaluation-and-supervision) is
different. A harness run is a program that the policy starts with
`ravel::run_harness` during evaluation, to ask a question before it chooses.
Ravel gives that run a result path of its own, so its result goes back to the
policy, not to a caller. The owner's harness entry names its program, so the
policy makes no selection for it.

Ravel also gives a harness run the
`RAVEL_POLICY_RUN` marker, an environment variable. If a program in that run
calls `start-session`, Ravel refuses with `policy_reentry_refused`, status 3.
The refusal occurs before Ravel reads settings or policy.

The final managed process receives no such marker and can start nested sessions.
Each nested session has its own
[evaluation budgets](02-actors-and-words.md#evaluation-and-supervision).
The marker does not protect against code that removes it: such code escapes the refusal.
Such an escape can leave a separate process group outside the outer cleanup.

Complete example: the inner program sends `inner`, and the outer program
uses that result in `inner|outer`. Create these files in a new directory and
make both files under `bin/` executable. These values have no trailing
newlines, so command substitution keeps them unchanged.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "nested-demo";

fn select(request, context) {
    #{
        status: "selected",
        program: if request.kind == "outer" { "outer" } else { "inner" },
        args: [ravel::path_resolve(request.cwd, "policy.rhai")],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Give each nested session its own result",
    }
}
```

**`bin/inner`**

```sh
#!/bin/sh
set -eu
ravel end-session -- "inner"
```

**`bin/outer`**

```sh
#!/bin/sh
set -eu
enclosing=$RAVEL_EXIT_FILE
result=$(ravel start-session --kind inner --config "$1" --prompt "Send inner.")
[ "$RAVEL_EXIT_FILE" = "$enclosing" ]
ravel end-session -- "$result|outer"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind outer --config policy.rhai --prompt 'Run the inner session. Then add |outer to its result and send it.'
```

**Standard output, without a final newline**

```text
inner|outer
```

**Exit status**

```text
0
```

The table follows this example from the caller's command to the caller's
result:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | The caller | Runs `start-session` with the kind `outer` |
| 2 | The outer Ravel CLI | Launches `outer` with the outer path in `RAVEL_EXIT_FILE` |
| 3 | `outer` | Keeps its path in `enclosing`, then runs `start-session` with the kind `inner` |
| 4 | The inner Ravel CLI | Launches `inner` with a new inner path |
| 5 | `inner` | Sends `inner` to the inner path |
| 6 | The inner Ravel CLI | Completes the inner cleanup and writes `inner` on its standard output, which `outer` captures |
| 7 | `outer` | Checks that its path did not change, then sends `inner|outer` to the outer path |
| 8 | The outer Ravel CLI | Completes the outer cleanup and writes `inner|outer` on the caller's standard output. It exits with status 0. |

Each layer's Ravel CLI supervises only the group of its own managed process,
and it does the cleanup only for that group. Thus, if an inner Ravel CLI stops
abruptly before it completes its cleanup, the group of its managed process can
remain. The outer Ravel CLI does not stop that group. The
[operational limits](#understand-the-operational-limits) apply at each layer.
The example checks that the results stay separate. It does not check
interactive job control for nested sessions.

## Confine the managed process

You may want to limit what the managed process can change on your computer.
For example, an LLM that edits files in one project needs no write access to
your other files.
[Confinement](02-actors-and-words.md#trust-and-confinement) does this. Ravel
launches the managed process inside a sandbox of the operating system, which
restricts its filesystem access. Ravel also gives it a smaller environment.
Add `--confine` when you want these restrictions.

Ravel uses one sandbox tool on each operating system:

- **macOS: `/usr/bin/sandbox-exec`.** This system command runs a program
  inside a sandbox that a profile describes. Ravel gives it a profile that
  refuses access by default and then permits the grants below. Its
  manual page, `man sandbox-exec`, marks the command as deprecated.
- **Linux: bubblewrap, at `/usr/bin/bwrap` or `/bin/bwrap`.** This tool runs a
  program in new Linux namespaces. Ravel starts it with an empty filesystem
  view and adds only the grants below.
  [The bubblewrap project page](https://github.com/containers/bubblewrap)
  describes the tool.

<!-- tc: depart synonym-rotation because launch names the managed launch, while start names other process creation -->
Before Ravel evaluates the policy, it tests whether the backend can make a
sandbox. If the backend is not available or not usable, the invocation stops
with `confinement_unusable` and status 2. Ravel does not then launch the
process without confinement.

The owner policy and its
[policy worker](02-actors-and-words.md#evaluation-and-supervision)
stay outside the confinement. A **measured read** is a read of a file through
`ravel::read_text` or `ravel::read_json`. The policy's measured reads can
still reach any regular file that the user can read. Evaluation harness runs
use the same confinement rules as the managed process. Thus, `--confine` also
restricts those runs.

These paths have different grants for the confined managed process:

| Path | Access | How the process gets access |
|---|---|---|
| Canonical working directory and private run directory | Read and write | Ravel grants them for each confined launch. |
| System runtime paths, selected executable and real executable file of the Ravel CLI | Read-only | Ravel grants them for each confined launch. |
| One additional regular file that exists | Read-only | `--runtime-read FILE`. Repeat it for each file. It requires `--confine` and grants no directory tree. |
| Parent of the result directory | Read and write | `--exit-dir DIR`. The directory must exist and be writable. |

If the managed process needs a full directory, copy the task data into the
working directory. A read-only runtime grant stays read-only, even inside a
writable directory. Confinement permits network access. It is not an offline
mode.

`--exit-dir DIR` puts the private result directory of the session in a
writable directory that exists. It works with or without confinement. With
confinement, it also gives write access to that parent directory. Thus, do
not choose a parent directory that contains files that the managed process
must not change. Without the option, an unconfined session uses the system
temporary directory. A confined session puts the result channel in its
private run directory.

Keep the policy directory and the owner settings outside every filesystem
grant. Ravel makes each path canonical. If paths overlap, Ravel stops with
an error before it evaluates the policy. This check also finds overlaps
through symlinks or through the system runtime grants. For example, if you
run `--confine --config ./policy.rhai` from the directory that contains that
policy, the policy overlaps the writable working directory. Put the policy in
a separate owner directory, and invoke Ravel from the project directory.

`confinement_overlap` reports the paths that overlap. `confinement_unusable`
reports a resource or a backend that Ravel cannot use. Both use status 2.
Before selection, an `--exit-dir` that is not valid can also give
`exit_dir_unusable`. Later failures of allocation or cleanup have their own
diagnostics.

Complete example: select `ravel end-session` itself as a minimal confined
program. This example checks confinement and result delivery without another
agent. Create the labelled policy, then run the command from the directory
that contains `owner`. The command makes a separate project directory and
makes it the current directory. Thus, the writable directory does not
contain the policy.

<!-- example: requires confinement -->

**`owner/policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "confined-demo";

fn select(request, context) {
    #{
        status: "selected",
        program: "ravel",
        args: ["end-session", "--", "confined"],
        provider: "local",
        model: "sender",
        effort: "none",
        reason: "Check a confined sent result",
    }
}
```

**Command**

```console
$ mkdir -p home tmp project || exit "$?"
$ example_home="$PWD/home" && example_tmp="$PWD/tmp" && cd project || exit "$?"
$ env -i HOME="$example_home" TMPDIR="$example_tmp" PATH="$PATH" ravel start-session --kind check --config ../owner/policy.rhai --confine --prompt 'Send confined.'
```

**Standard output, without a final newline**

```text
confined
```

**Exit status**

```text
0
```

Here, the selected program is `ravel end-session` itself. It runs inside the
sandbox, in the `project` directory, and finds its result path in its own
`RAVEL_EXIT_FILE`. It sends `confined`. After cleanup, the caller receives
`confined` on standard output and status 0.

The confined environment keeps `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and
`LC_*`. Ravel sets `TMPDIR`, `TMP` and `TEMP` to the private run directory and
gives the managed process a new `RAVEL_EXIT_FILE`. The `HOME` variable does
not give access to the directory that it names. The other caller variables are
not in the environment.

`--policy-env` grants a
variable to `ravel::env`, the policy's environment helper.
It adds no variable to a confined managed process or harness run.

Confinement also changes how the managed process uses your terminal. A
**POSIX session** is an operating-system set of process groups that can share
one controlling terminal. It is not a Ravel session. To
[lend](03-a-complete-session.md#launch-the-managed-job) the terminal, Ravel
makes the group of the managed process the foreground group of the terminal.
Then that group receives the keyboard input. After the session, Ravel gives
the terminal back to its own group.

A confined launch starts a new POSIX session and is not interactive. Ravel
does not lend it the caller's controlling terminal. An unconfined launch uses
a new process group in the caller's POSIX session. Ravel lends it the terminal
if Ravel's own process group is the foreground group of the terminal. This
difference is important for an interactive program. [Chapter 9, “Jobs, signals and the
terminal”](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal), teaches the mechanism.

Confinement is separate from the permission flags of an agent program.
Neither the unconfined mode nor the confined mode confines the owner policy.
Use this comparison to choose
confinement together with its effects on the environment and the terminal:

| Property | Unconfined managed process | Confined managed process |
|---|---|---|
| Filesystem access | No Ravel confinement | Write access to the working directory and the private run directory. Read access to the runtime paths and the explicit grants. |
| Caller environment | Inherited, without the five variables that Ravel removes | Only `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and `LC_*` |
| Temporary-path variables | Inherited | `TMPDIR`, `TMP` and `TEMP` set to the private run directory |
| Session-result context | New `RAVEL_EXIT_FILE` | New `RAVEL_EXIT_FILE` |
| Process group | New process group in the caller's POSIX session | New POSIX session |
| Caller terminal | Lent when Ravel's group is the foreground group of the terminal | Not lent. Not interactive. |
| Network | No Ravel confinement | Confinement permits it |

## Read notices separately from results

Your program needs the result. It can also want to know what the policy chose.
For example, an orchestrator can record which harness, model and effort ran
each task. Ravel gives these two kinds of information on two different
streams. This section shows what each stream carries and when Ravel writes to
it.

What does the policy give back to Ravel? It gives back a selection or a
refusal. A **selection** is the map that `select` returns when it chooses a
program. It holds `program` and `args`, the command that Ravel runs. It also
holds the labels `provider`, `model`, `effort` and `reason`.

A label describes
the choice of the policy, but it does not configure the harness. For example,
a policy that selects Claude Code puts the model option of Claude Code in
`args`. The `model` label only tells the caller which model the policy chose.
These labels are owner text, not integrations with model providers.

The caller does not read the owner's policy. Thus, Ravel tells the caller what
the policy chose in the **routing notice**:

- **Who writes it:** the Ravel CLI.
- **When:** in text form, after the policy selects a program and before Ravel
  launches it. In JSON form, after the supervision attempt.
- **Where:** the caller's standard error. Standard output carries only the
  result.
- **Who reads it:** the caller, or the person who watches the terminal.
- **Why:** this notice is the way in which Ravel tells the caller which
  harness, model and effort the policy chose.

The text form is one line that starts with `ravel: starting provider`. It
gives the selected provider, model, effort and kind. `--json` replaces that
line with a version-2 JSON document. The document gives more detail, such as
the `reason` and the program file that Ravel found.

Complete example: run one review session in text form, the same session with
`--json`, and a request that the policy refuses. The policy uses the values of
the review example in chapter 1. The fake agent writes one line, then sends
`continue`. Create these files in a new directory and make
`bin/review-agent` executable.

**`policy.rhai`**

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
        message: "This policy has no rule for the kind " + request.kind + ".",
        remedy: "Use the kind review.",
    }
}
```

**`bin/review-agent`**

```sh
#!/bin/sh
set -eu
echo "review-agent: the review is complete"
ravel end-session -- "continue"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind review --config policy.rhai --prompt 'Review the last commit. Then run ravel end-session continue.'
```

**Standard output, without a final newline**

```text
continue
```

**Standard error includes**

```text
ravel: starting provider local, model fake, effort low for kind "review"
review-agent: the review is complete
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --json --kind review --config policy.rhai --prompt 'Review the last commit. Then run ravel end-session continue.'
```

**Standard output, without a final newline**

```text
continue
```

**Standard error includes**

```text
"schemaVersion":2,"selection":{"effort":"low","model":"fake","provider":"local","reason":"Reviews go to the fake agent."}
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --json --kind deploy --config policy.rhai --prompt 'Deploy the last commit.'
```

**Standard output**

```text
```

**Standard error**

```text
{"diagnostics":{"stderr":"","stdout":""},"error":{"code":"policy_refused","exit":3,"input":"--kind deploy","message":"the policy <scratch>/work/policy.rhai refused the selection: This policy has no rule for the kind deploy.","policyCode":"unknown_kind","remedy":"Use the kind review.","source":"<scratch>/work/policy.rhai","stage":"selection"},"schemaVersion":1}
```

**Exit status**

```text
3
```

Here `<scratch>` is the temporary directory that the example checker of the
book uses. A manual run shows the actual absolute path of your policy.

The table follows the three commands to what the caller receives:

| Command | What Ravel writes on standard error, in order | What the caller receives |
|---|---|---|
| Text form | The routing notice, then the line of the fake agent | `continue` and status 0 |
| `--json` | The line of the fake agent, then the JSON routing notice | `continue` and status 0 |
| `--json --kind deploy` | Only the JSON failure document. The policy refused, so Ravel launched no program. | No result and status 3 |

In text form, the notice comes first, because Ravel writes it before the
launch. With `--json`, the notice comes last, because Ravel writes it after
the supervision attempt. The text form tells you the choice before the
program starts. The JSON form also records what happened during the session.

Fragment: standard error of the `--json` review command from a real run on
macOS. Ravel writes the JSON document as one line. This block shows it on
several lines and omits the members `bounds`, `context`, `diagnostics`,
`operationReports`, `policyEnvironment`, `reads` and `worker`.

```text
review-agent: the review is complete
{
  "harnessRuns": [],
  "policy": {
    "modules": [
      {
        "bytes": 605,
        "name": "/private/tmp/ch5r/policy.rhai",
        "sha256": "2f04fb9b1b9192ea4408b6dc3950180e04a6d1c1b1df68adb13064fe9ac50d53"
      }
    ],
    "path": "/private/tmp/ch5r/policy.rhai",
    "version": "notice-demo"
  },
  "promptSource": "argument",
  "resolution": "/tmp/ch5r/bin/review-agent (found on PATH in /tmp/ch5r/bin)",
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

`--json` changes the routing notice and the failure document of Ravel into
JSON on standard error. It does not change the result into JSON, and it does
not change other output on standard error. The managed process can still
write any text there. Keep standard error as a diagnostic stream, and do not
parse the full stream as one JSON document. The managed process can write a
line that looks like a Ravel notice. Thus, the form of a line does not prove
that Ravel wrote it.

Read standard output only as the result of a successful session. Keep standard error as a
mixed diagnostic stream, even with `--json`:

| Output | Destination | Effect of `--json` |
|---|---|---|
| Unchanged opaque result after a successful session | The caller's standard output, with no added newline | None |
| Standard output and standard error of the managed process | The caller's standard error | None. Any program text stays. |
| Standard output, standard error and optional result of an evaluation harness run | Return data for the policy | None. They reach a caller stream only if the policy copies them into visible output. |
| Routing notice and failure document of Ravel | The caller's standard error | JSON documents replace their text form |

The version-2 routing notice gives `selection`, `policy`, `worker`,
`selectionMs`, the delivered `context`, the effective `bounds`,
`policyEnvironment`, `resolution`, `promptSource` and the captured policy
`diagnostics`. The field `context` is policy context, not the result
channel. Grant diagnostics show the name of each environment variable and
whether it is set. They never show its value. Policy code can still write
values itself. Thus, keep its diagnostics with the care that those values
need.

The notice also records what evaluation used:

| Field | Recorded data |
|---|---|
| `worker.rhaiVersion` | Version of the embedded Rhai engine |
| `policy.modules` | Entry first, then imported modules, each with its canonical name, byte count and SHA-256 digest |
| `reads` | Successful measured reads, with their operation, phase, canonical name, byte count and digest |
| `harnessRuns` | Each harness run, with its harness name, status, exit code or signal, elapsed time and whether it sent a result |
| `bounds.harnessRuns` | Effective run budget and its origin |

An unavailable call appears in `harnessRuns` even though no program started.
The run record contains no prompt, answer or token usage.
The policy can still expose that data through its own diagnostics or context.

Ravel writes this JSON notice after the supervision attempt and before it
decides the final outcome. The
[runner](02-actors-and-words.md#evaluation-and-supervision) is the part
of the Ravel CLI that launches and supervises the managed process. If the
runner completed supervision and returned its record of how the job ended,
`operationReports` holds the operational reports that the runner kept. The
array is empty if there are no reports. If supervision failed before the
runner returned that record, the notice has no such field.

Some reports, such as a failed stop attempt, stay visible even when later
checks confirm successful cleanup. A report alone does not decide success,
and the routing notice also does not. Check the final status.

A JSON failure has `schemaVersion: 1` and an `error` object with `code`,
`stage`, `message`, `remedy` and `exit`. It adds the fields that apply, such
as `input`, `source`, `location`, `policyCode`, `bound` and `signal`, and the
captured policy `diagnostics` when they are available. The refused command of
the example above shows `policyCode`, the owner's own code. Read the code and
the stage together. These examples distinguish a policy decision from failures
before and after launch:

| Code | Stage | Meaning |
|---|---|---|
| `policy_refused` | `selection` | The owner policy declined the request. |
| `worker_identity_mismatch` | `worker` | The re-executed Ravel binary reports another build. Ravel stops before policy evaluation. |
| `harness_budget_exhausted` | `evaluation` | The policy requested a run with no run budget left. Evaluation ends. |
| `harness_cleanup_failed` | `evaluation` | Ravel could not complete the cleanup of an evaluation harness run. No managed process starts. |
| `session_result_missing` | `exec` | The managed process ended without a sent result. |

The text form of each failure starts with `ravel: refused`, also when no
policy refused the request. The code and the status show the difference
between a policy
[refusal](02-actors-and-words.md#evaluation-and-supervision) and an
operational failure.

For worker teardown and final-session cleanup, an original failure keeps its
code and status if cleanup also fails. Ravel reports those cleanup problems
together with the original failure.

An evaluation harness run has an exception. If its cleanup fails after the
whole deadline expires, `harness_cleanup_failed` replaces `selection_timeout`.
The status is then 5, not 124. Cancellation has precedence over both.

Ravel then sets the cancellation
status, keeps the original and cleanup diagnostics and returns no result to
the caller. If required cleanup fails, Ravel also returns no result to the
caller, even when everything else succeeds. Thus, keep the complete standard error
stream, even after the caller classifies the exit status.

## Interpret the exit status

Your program has the status of `start-session`. What does it mean, and what
does your program do next? This table gives the normal outcomes of the Ravel
CLI. It comes from the status constants of the Ravel CLI, its launch and
final-outcome code and its code for signals.

A caller checks the status of `start-session`. It does not use the status of
the managed process or the earlier status of `end-session`. For a session
result, accept standard output only with status 0 from `start-session`. A help
or version request is a separate invocation for information, and it sends no
result.

These sections teach the subject of each row:

- statuses 0 and 1: [Accept a result only after success](#accept-a-result-only-after-success)
- status 2: [Supply the prompt and caller data](#supply-the-prompt-and-caller-data)
  and [Confine the managed process](#confine-the-managed-process)
- status 3: chapter 4's [Select a command or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse)
- status 5: [Read notices separately from results](#read-notices-separately-from-results)
- status 124: chapter 4's [bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names)
- statuses 126 and 127: program resolution, in
  [Supply the prompt and caller data](#supply-the-prompt-and-caller-data)
- statuses 129, 130 and 143: [Cancel during selection or supervision](#cancel-during-selection-or-supervision)

| Status | Meaning | What the caller does |
|---|---|---|
| `0` | `start-session` wrote the sent result unchanged to standard output after Ravel confirmed the reap, the absence of the group and the required cleanup. An explicitly empty result is valid. A later nonzero exit of the child does not make it invalid. For `end-session`, status 0 means that it sent the result, or that a valid first result was already present. A help or version request also exits with 0. | Interpret the result of a successful session with the caller's own convention. Ravel does not interpret it. Wait for `start-session`. The success of `end-session` alone does not confirm cleanup. |
| `1` | `start-session`: `session_result_missing`, even if the child exited with status 0. `end-session`: it could not send the result. For example, `RAVEL_EXIT_FILE` was missing, empty or stale, the result channel was not valid, or the value was larger than its limit. | Check that the managed process calls the installed `ravel end-session` with one result before it exits. Check also that it makes the call inside its live enclosing session. Keep any cleanup diagnostics too. Do not treat a missing result as an empty result. |
| `2` | Malformed command-line input or settings, invalid prompt or data flags, or confinement inputs that are unusable or overlap. A missing subcommand shows the help with status 2. An argument-count error of `end-session` also uses 2. | Correct the named input, setting or confinement layout. Run the command again only after you apply the remedy of the diagnostic. |
| `3` | A refusal by the policy, or Ravel's refusal of the policy. The owner policy declined, or Ravel could not accept its entry, behaviour, shape, returned data or policy context. Script limits, the worker heap bound, exhausted evaluation budgets and policy re-entry also use 3. Ravel tries no other route. | Read `code`, `stage`, the location and the remedy. For `policy_refused`, examine the owner's `policyCode` and explanation. Change the request or the policy deliberately. Do not silently substitute a program. |
| `5` | A worker failed, or supervision failed after the runner reported the launch. The result data was not valid, or required cleanup or terminal recovery failed. A group survived, or the result output was unavailable or failed. A failed re-execution or worker identity mismatch also uses 5, as does `harness_cleanup_failed` during evaluation. | Keep the full diagnostics. Repair the installation, or the process or filesystem problem that the diagnostic names. Examine the unresolved processes or temporary directories that Ravel reports before you try again. Discard any partial result bytes. <!-- tc: depart synonym-rotation because repair restores a resource, while correct also describes a valid value --> |
| `124` | The whole policy evaluation deadline expired. It covers worker startup, module loads, the context loader, selection and every host call. Ravel launched no managed process. A failed harness cleanup replaces this outcome with status 5. Cancellation also has priority. | Examine the work of the policy. Change `--timeout-ms` or the owner's `timeoutMs` only if you want that longer evaluation. A single harness run's `timed_out` status is data for the policy. This is not a time limit for the managed process. |
| `126` | Ravel could not execute the selected program, or the launch failed, before the runner reported that the program started. The cause is not ENOENT (no such file). | Check the execute permissions, the format, the interpreter, the platform and the execution environment. A launch failure before the start can also need terminal recovery. Keep those diagnostics. Ravel runs no substitute. |
| `127` | Program resolution found no executable, or a launch failed with ENOENT before the start. An executable script can exist while its interpreter is missing. | Check the caller's `PATH`, the selected path and its interpreter. Install or correct the program that you intended. |
| `129` | Cancellation by HUP (signal 1), during selection or supervision. Ravel returns no result to the caller, even if it already received one. | Treat the invocation as cancelled. Keep the cleanup reports. Try again only if the caller still wants the work. |
| `130` | Cancellation by INT (signal 2), during selection or supervision. Ravel returns no result to the caller, even if it already received one. | Treat the invocation as cancelled. Distinguish a signal to Ravel from a terminal signal to only the group of the managed process. [Cancel during selection or supervision](#cancel-during-selection-or-supervision) explains the difference. |
| `143` | Cancellation by TERM (signal 15), during selection or supervision. Ravel returns no result to the caller, even if it already received one. | Treat the invocation as cancelled. Keep the cleanup reports. Try again only if the termination was not intended. |

The last three numbers follow the shell convention: 128 plus the signal
number. Ravel reports the cancellation, attempts the cleanup and then raises
the signal again. Thus, a process API reports termination by that signal,
not a numeric exit. If the propagation of the signal fails, Ravel reports
that failure and exits with the same number as a fallback. A caller that
uses a process API handles both forms.

A signal such as KILL can stop the Ravel CLI at any point. The normal
procedure of Ravel does not apply to such a signal. Then the shell status
does not show that cleanup or result delivery occurred. Also, a new signal
that arrives after the final cancellation decision can interrupt the result
write. No atomic guarantee covers both the arrival of a signal and an
arbitrary write to standard output.

The following view expands one step of
[chapter 3's outcome state machine](03-a-complete-session.md#recognise-the-other-outcomes):
the step where Ravel decides the final outcome. Cancellation has priority over an
original failure. After evaluation decides its failure, the final cleanup
does not replace that failure's status. During evaluation, failed harness
cleanup can first replace a deadline with `harness_cleanup_failed`.
Only a valid result with confirmed cleanup reaches the final write.

The view shows the precedence of outcomes, not the order of function calls.
Ravel attempts the applicable cleanup before it reports an outcome.

The view covers `start-session`. The table also covers `end-session` and the
help and version invocations.

![Ravel checks cancellation first and keeps any original failure with its status. Then it requires confirmed cleanup before it writes the result unchanged. Cancellation gives 129, 130 or 143 and returns no result to the caller. Original failures keep 1, 2, 3, 5, 124, 126 or 127. A cleanup or final-write failure gives 5. Only a successful write and flush gives 0.](figures/settled-exit-status.svg)

[Open the final-outcome view at full size](figures/settled-exit-status.svg).

## Cancel during selection or supervision

You may need to stop a session before it ends. For example, the person at the
terminal presses Ctrl-C, or your orchestrator stops its wait. A
**cancellation** is an INT, TERM or HUP signal that reaches the Ravel CLI.
Ravel then stops the work, attempts cleanup and returns no result to the
caller. What Ravel stops depends on the phase of the session:

| Phase | What runs | What Ravel does at a cancellation |
|---|---|---|
| Selection | The policy worker, and any harness run that the policy started | Attempts to stop and reap the harness run, then the worker. Launches no managed process. |
| Supervision | The managed process and its process group | Attempts to stop the group, to reap the managed process and to recover the terminal. Returns no result, even after a sent result. |

A cancellation during selection stops evaluation and prevents the managed launch.
Ravel abandons a read in progress and does not wait for it.
If a harness run is in progress, Ravel first stops its group and attempts its reap.
Then it stops and reaps the worker and removes the private directories.
Ravel reports the cancellation with any cleanup problems.

The worker has no session-result
context. Thus, it cannot send a result for an enclosing session through the
usual `end-session` interface.

An evaluation harness run can therefore need cleanup before any managed launch.
The example below uses a fake harness that sends INT to its parent, the Ravel CLI.
The policy itself cannot send a signal.

In these UML sequences, no lifetime ends with an X. Ravel attempts
cleanup, and a failed attempt does not confirm a reap or a removal.

The selection sequence shows that Ravel attempts harness cleanup before worker teardown.
The fake evaluation harness sends the signal directly to the Ravel CLI.
Its own result channel is separate from the channel of a final managed process.

![During selection, the policy worker requests a harness run and waits for data. The Ravel CLI starts the fake evaluation harness with its own result channel. The harness sends INT to the Ravel CLI. Ravel attempts harness group shutdown, reap and directory cleanup within the stop bound, then worker teardown and private-directory cleanup. It retains cleanup problems and reports cancellation. It launches no final managed process and returns no result to the caller.](figures/selection-cancellation.svg)

[Open the selection cancellation sequence at full size](figures/selection-cancellation.svg).

Complete example: a fake evaluation harness sends INT to the Ravel CLI.
The policy waits for the harness run. No final managed process starts.

**`settings.json`**

```json
{"harnesses":{"cancel":{"program":"cancel-selection"}},"budgets":{"harnessRuns":1}}
```

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "cancel-selection";

fn select(request, context) {
    ravel::run_harness(#{ harness: "cancel", args: [] });
    #{ status: "refused", code: "unexpected_return",
       message: "The call returned without cancellation.",
       remedy: "Check that the fake harness sent INT." }
}
```

**`bin/cancel-selection`**

```sh
#!/bin/sh
set -eu
kill -INT "$PPID"
while :; do sleep 1; done
```

**Command**

```console
$ mkdir -p home/.config/ravel tmp && chmod +x bin/* || exit "$?"
$ cp settings.json home/.config/ravel/settings.json || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.rhai --prompt 'No managed process should launch.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (selection_cancelled, stage evaluation): selection with the policy <scratch>/work/policy.rhai was cancelled by SIGINT: worker teardown was attempted and nothing was launched
```

**Exit status**

```text
130
```

Here `<scratch>` is the temporary directory that the example checker of the
book uses. A manual run shows the actual absolute path of your policy. The
details of worker teardown can change from run to run. Thus, the transcript
claims only the stable diagnostic line.
Here, the diagnostic's “nothing was launched” refers to the final managed process.
The evaluation harness did run before cancellation.

The owner enables this call with a named harness entry and a run budget of one.
The run has no terminal, so its signal does not depend on keyboard input.
[Chapter 4](04-writing-an-owner-policy.md#run-a-harness-during-evaluation)
explains the call's inputs and output.

The table follows this example to what the caller receives:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | The caller | Runs `start-session` with the kind `check` |
| 2 | The owner policy | Calls `ravel::run_harness` for the harness entry `cancel` and waits |
| 3 | The Ravel CLI | Starts `cancel-selection` as a harness run |
| 4 | `cancel-selection` | Sends INT to its parent, the Ravel CLI |
| 5 | The Ravel CLI | Attempts to stop and reap the harness run, then the worker. It launches no managed process. |
| 6 | The Ravel CLI | Writes `selection_cancelled` on standard error. The caller receives no result and status 130. |

The whole deadline uses the same stop order as cancellation.
Cleanup takes additional time. A signal consumed before the final outcome decision
has priority over a deadline, and the report keeps the earlier failure.
A stop of an LLM call ends local work only. A model service can still complete
a request that it already accepted.

During supervision, a cancellation stops the group of the managed process.
Ravel attempts the reap, the terminal recovery and the temporary cleanup
before it reports the signal. Ravel returns no result to the caller, even when
it already received a result. A cleanup failure does not change the
cancellation into success. Ravel reports the failure together with the
cancellation.

The supervision sequence shows why a cancellation returns no result to the
caller, even after the managed process sent a result. The result reaches the
temporary result channel, but Ravel does not write it to standard output. The runner
runs inside the Ravel CLI. The group of the managed process includes the
descendants that stay in that group. The cleanup steps in the figure are
attempts, and Ravel keeps their failures as diagnostics.

![The managed process sends candidate through end-session. Then it sends INT to the Ravel CLI. The Ravel CLI attempts group shutdown, reap, terminal recovery and temporary cleanup. It returns no result to the caller and reports cancellation, even if cleanup fails.](figures/supervision-cancellation.svg)

[Open the supervision cancellation sequence at full size](figures/supervision-cancellation.svg).

Complete example: the program sends a candidate result and sends INT
directly to the Ravel CLI. Then it stays alive until the supervisor stops the
job. Create the files in a new directory and make `bin/cancel-after-result`
executable.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "cancel-session";

fn select(request, context) {
    #{
        status: "selected",
        program: "cancel-after-result",
        args: [],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Cancel after a sent result",
    }
}
```

**`bin/cancel-after-result`**

```sh
#!/bin/sh
set -eu
ravel end-session -- "candidate"
kill -INT "$PPID"
while :; do sleep 1; done
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.rhai --prompt 'Show cancellation after a sent result.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (session_cancelled, stage exec): session selected by <scratch>/work/policy.rhai was cancelled by SIGINT
```

**Exit status**

```text
130
```

The program sent `candidate`, and Ravel received it. Then INT reached the
Ravel CLI. Ravel attempted to stop the group and to complete cleanup. Then it
wrote `session_cancelled` on standard error. The caller receives no result and
status 130, although a program sent a result.

A signal that you send to the Ravel CLI is different from Ctrl-C at the
terminal. After Ravel lends the terminal, the group of the managed process is
the foreground group of the terminal. Ctrl-C then makes the terminal send INT
to that group, not to the Ravel CLI. If only that group receives INT and
exits with no sent result, the Ravel CLI reports `session_result_missing`
with status 1. It does not report cancellation with status 130.

An interactive program can also handle or ignore the keyboard signal itself.
Use the status and the diagnostic of the Ravel CLI. Do not infer the outcome
from the key that you pressed. [Chapter 9](09-jobs-signals-and-the-terminal.md#preserve-the-entry-signal-state) explains signal inheritance and
[terminal recovery](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal).

Fragment: run an interactive route and press Ctrl-C at the terminal. This
fragment omits the policy, the interactive program and the physical
interaction with the terminal. The automated examples do not test keyboard
delivery or foreground recovery.

```sh
ravel start-session --kind interactive --config policy.rhai --prompt-file prompt.txt
```

## Understand the operational limits

What can an unsuccessful invocation leave behind? Ravel's cleanup guarantee
does not apply under the conditions in this table. Use the table to decide
what the caller examines after such an invocation:

| Condition | Consequence |
|---|---|
| Code that runs as the same user makes unauthorised changes to the result channel or the installation. | Private paths and the inherited session-result context keep ordinary sessions apart. They are not a defence against hostile code of the same user. The worker identity check confirms that the re-executed binary reports the expected build. It does not check a trusted signature. |
| The Ravel CLI stops abruptly, for example because of KILL. | It cannot complete supervision or cleanup. The group of the managed process and the temporary directories can remain. There is no successful completion and no persistent recovery record. |
| A managed descendant deliberately leaves its process group. | It is outside the cleanup guarantee of that group. The absence of the group does not prove that the detached process is gone. |
| An evaluation harness run starts a descendant that leaves its process group. | Cleanup of the run does not reach that descendant. The policy worker itself starts no program. Confinement does not restrict measured reads. |
| Another process changes the foreground group of the terminal during recovery, or the system gives a recorded group ID to an unrelated group. | Recovery assumes two conditions between the runner's check and its handback. No other process or thread changes the foreground group. No unrelated group gets the recorded group ID. Ravel reports a failed handback. Do not treat a launch failure as proof that Ravel restored the terminal. [Chapter 9](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal) explains the conditions. |
| The host is different from the measured platforms. | The project measured how Ravel handles inherited signal settings, cancellation, cleanup and terminal recovery. It measured them on aarch64 macOS and aarch64 GNU/Linux. The project built Ravel for x86_64 GNU/Linux and ran it only under emulation. Native x86-64 qualification is not complete. These checks do not cover all combinations of kernel, libc and compiler. |

Chapter 2 explains these [trust limits](02-actors-and-words.md#trust-and-confinement).
[Ravel issue 1](https://github.com/Linkuistics/Ravel/issues/1) tracks the
native x86-64 qualification. The examples in this chapter check the choice
of command, the delivery of the result and the status. They do not show the
behaviour of real agents, interactive terminal operation or whether a new
reader can use the book.

## Check your caller before real use

In short, the caller runs `ravel start-session` with a kind, a prompt and
optional caller data. The owner policy selects a program, and the Ravel CLI
tells the caller about that choice in the routing notice on standard error.
Ravel launches the program with a new path in `RAVEL_EXIT_FILE`. A program in
the session sends one result to that path with `ravel end-session`. Ravel stops
the program and its process group, completes cleanup and writes the result
unchanged on standard output. The status of `start-session` tells the caller
whether it can use that result.

Before your program runs sessions with real harnesses, check each of these
points with fake agents:

| Point to check | What a good caller does |
|---|---|
| The prompt | Tells the LLM how and when to run `ravel end-session`, with the permitted result values |
| The status | Uses the result only after status 0 from `start-session` |
| The result bytes | Redirects standard output to a file when trailing newlines are important |
| A missing result | Treats status 1 as no result, not as an empty result |
| Standard error | Keeps it as a mixed diagnostic stream, and takes no line as proof of what Ravel wrote |
| A cancellation | Treats 129, 130, 143 and termination by a signal as no result |
| A nested session | Sends its own result after the inner session returns |
| Confinement | Keeps the policy and the owner settings outside every filesystem grant |

[Chapter 6, “Reference”](06-reference.md), gives the reference entries for the
inputs and outputs that this chapter teaches.
