# A complete session, end to end

In this chapter, you run one
[session](02-actors-and-words.md#caller-and-owner), one run of
`ravel start-session`, on your own computer. The session uses a small
[configuration script](02-actors-and-words.md#caller-and-owner), the Rhai file
that chooses the program, and a fake agent. Then you follow the request
through selection, launch, the sent result and cleanup.

You act as both [owner and caller](02-actors-and-words.md#caller-and-owner).
The owner controls the configuration of Ravel, and the caller requests the
session and interprets its result. You write the rule in the configuration
script, supply the prompt and choose the result that you expect. The fake is
an ordinary shell program. It needs no model service or agent account.

The fake is the **managed process**, the program that the configuration script
selects.
Its **process group** holds it and the descendants that stay in that group.
Ravel can signal all members together. To **reap** its child, Ravel collects
the child's exit status from the operating system.

The example returns `finished`. In chapter 1, the review prompt permitted
`approved` and `changes-needed`, and the orchestrator had an action for each
value. Here, the prompt permits one value, and your action as the caller is to
check the exit status and read `finished`. That word is your convention, not a
Ravel command to continue or stop other work. Ravel returns this word only after it
reaps the [managed process](02-actors-and-words.md#the-managed-process-and-its-result),
the whole process group is gone and cleanup succeeds. `start-session` exits
with status 0 only if its final write to standard output also succeeds.

## Install Ravel and check its version

Use a macOS or GNU/Linux host with Homebrew. The Homebrew formula
`linkuistics/taps/ravel` installs release archives for aarch64 macOS,
aarch64 GNU/Linux and x86_64 GNU/Linux.

Fragment: installation and version check. This block omits Homebrew setup
and the installation output.

```sh
brew install linkuistics/taps/ravel
ravel --version
```

Before you use the examples of this book, compare the version output with
this version line:

```text
ravel @RAVEL_BOOK_VERSION@
```

The release procedure writes the version of the release into that line. If
the line has a placeholder, this copy of the book is not from a release. If
the two versions are different, open the book for your installed version from
[the release list on GitHub Pages](https://linkuistics.github.io/Ravel/).
The release procedure keeps each release's book in its own directory there.

No release carries the book yet. Until the first one does, use the checkout's
Markdown or local build. [Chapter 11](11-deliver-ravel.md#keep-the-book-with-its-release)
explains this limit and the address of each release's book.

The directory that `brew --prefix ravel` prints contains the
[Ravel CLI](02-actors-and-words.md#evaluation-and-supervision), `bin/ravel`.
That executable also contains the Rhai engine, which runs the configuration
script. Ravel launches a second copy of its own binary as the separate
[Rhai worker](02-actors-and-words.md#evaluation-and-supervision), the process
that evaluates the configuration script. You install no separate worker or
language runtime.

The [platform evidence in chapter 1](01-what-ravel-is-for.md#see-what-we-tested-and-what-remains-unchecked)
separates the measured aarch64 macOS and aarch64 GNU/Linux targets from
x86_64 GNU/Linux. For x86_64 GNU/Linux, the only execution evidence comes from
emulation. [Chapter 11, “Deliver Ravel”](11-deliver-ravel.md), gives the installation and target
details.

## Run the first session

The example of this chapter is the first session. Its directory is
`~/ravel-first-session`, in your home directory. It holds a configuration script that selects one fake
agent for every request. The fake prints the prompt that it receives and
sends the result `finished`.

When you finish the steps, the directory holds these paths:

```text
~/ravel-first-session/
├── config.rhai        you save it
├── bin/               you make it
│   └── fake-agent     you save it
├── home/              the command makes it
└── tmp/               the command makes it
```

Each path has one purpose in the session:

| Path | Who makes it | Purpose |
|---|---|---|
| `config.rhai` | You | The configuration script. The option `--config config.rhai` selects it. |
| `bin/fake-agent` | You | The fake agent, a shell script that the configuration script selects. It stands where a harness stands. |
| `home/` | The command | An empty home directory for Ravel. Ravel finds no settings file there, so the default settings apply. |
| `tmp/` | The command | A private temporary directory. Ravel makes the result channel of the session here. |

Do these steps in the shell where `ravel --version` succeeded:

1. Make the example directory and its `bin` subdirectory, then `cd` into the
   example directory. The fragment after these steps gives the two commands.
2. Save the two files of the complete example at their labelled paths. Each
   label is a path in `ravel-first-session`.
3. Type the command of the complete example without its first `$ `, which
   stands for the prompt of your shell.
4. Run `echo "$?"` immediately after the command to see its exit status.
   Compare the output and the status with the example.

Fragment: make the example directory and `cd` into it. This block omits the
two files, which you save next. The book checker makes its own directory for
each example and does not run this block.

```sh
mkdir -p ~/ravel-first-session/bin
cd ~/ravel-first-session
```

The `~` stands for your home directory, so these commands work from any
current directory.

Complete example: pass a prompt through a small configuration script to a
fake agent. The fake prints the prompt and sends `finished`.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "first-session";

fn select(request, context) {
    #{
        status: "selected",
        program: "fake-agent",
        args: [request.prompt],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Learn the session protocol without a model service.",
    }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
set -eu
printf 'fake-agent received: %s\n' "$1"
ravel end-session -- "finished"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config config.rhai --prompt 'Print this prompt, then call ravel end-session -- "finished".'
```

**Standard output, without a final newline**

```text
finished
```

**Standard error includes**

```text
ravel: starting provider local, model fake, effort none for kind "demo"
fake-agent received: Print this prompt, then call ravel end-session -- "finished".
```

**Exit status**

```text
0
```

In your terminal, the lines of standard output and standard error appear
together. The book shows each stream in its own block. Ravel writes the
lines of the second block before the result.

What did the command do? It is one line with three commands, joined by `&&`.
Each command runs only if the command before it succeeded. These are its
parts:

| Part of the command | What it does in `ravel-first-session` |
|---|---|
| `mkdir -p home tmp` | Makes `home` and `tmp`. With `-p`, `mkdir` gives no error when they exist already. |
| `chmod +x bin/fake-agent` | Makes the fake executable, so that Ravel can run it as a program |
| `env -i` | Runs the rest of the line with an empty environment. Your own variables, such as a `RAVEL_EXIT_FILE` of an enclosing session, do not enter the session. |
| `HOME="$PWD/home"` and `TMPDIR="$PWD/tmp"` | Give Ravel the private directories. Your shell expands `$PWD` before `env` runs, so each value is an absolute path. |
| `PATH="$PWD/bin:$PATH"` | Puts `bin` first on the search path. Your shell expands `$PATH` before `env -i` clears the environment, so your installed `ravel` stays on the path. |
| `ravel start-session` | Asks Ravel for one session |
| `--kind demo` | Gives the caller's kind. This configuration script does not examine it. |
| `--config config.rhai` | Selects the configuration script of the example, relative to the current directory |
| `--prompt '…'` | Gives the caller's prompt. The script passes it unchanged to the fake. |

The private home and the explicit `--config` keep your usual settings and
configuration script out of this session. Without `--config`, Ravel looks
for the default script `home/.config/ravel/config.rhai`. That file does not
exist, so Ravel refuses with `config_missing` and exit status 3.

<!-- tc: depart synonym-rotation because the start of a shell prompt is a text position, not a process launch -->
Ravel writes the result unchanged to standard output. Here, the output is
the eight bytes `finished`. Ravel does not interpret the result and adds no
newline. Thus, in an interactive shell, the next prompt starts on the same
line, unless the shell itself marks or supplies the missing newline.

The standard-error block shows the two stable lines. It does not promise
that the stream contains only those lines. Ravel can also report cleanup operations
with details, such as process IDs, that are different in each run.

<!-- tc: depart synonym-rotation because confirmation establishes completed cleanup, while the caller checks an exit status -->
On macOS, an attempt to stop a group whose members already exited can report
`Operation not permitted`. Ravel writes that report to standard error. It
allows success only after it confirms the reap, the terminal recovery and the
absence of the group. A report alone does not prove success or failure. Check
the exit status of `start-session`.

The fake's `printf` writes to its own standard output, but the line appears
on the caller's standard error. Ravel uses the caller's standard output only
for the [opaque result](02-actors-and-words.md#the-managed-process-and-its-result).
Before the launch, Ravel redirects the standard output of the managed process
to the caller's standard error. The fake does not interpret the prompt. Its
author wrote the script to run `end-session`, which is the action that the
prompt requests. Ravel does not understand that instruction and does not add it.

The configuration script selects the fake for every request. `demo` is the
caller's [kind](02-actors-and-words.md#caller-and-owner), the token that names
the sort of work. This script does not need to examine it.

What did the configuration script choose? Look at the first line on standard
error. This line is the **routing notice**. The Ravel CLI writes it on your
standard error after the script selects a program and before the launch. It
tells you, the caller, which provider, model and effort the script chose, and
for which kind. The values come from the labels of the selection.

The configuration script returns a **selection**, the map with
`status: "selected"`. Ravel reads these fields from it:

| Field | Value in this example | Effect |
|---|---|---|
| `status` | `"selected"` | Tells Ravel that the map is a selection, not a refusal |
| `program` | `fake-agent` | Names the program to execute |
| `args` | `[request.prompt]` | Passes the unchanged prompt as one argument |
| `provider` | `local` | Describes the provider in the routing notice |
| `model` | `fake` | Describes the model in the routing notice |
| `effort` | `none` | Describes the effort in the routing notice |
| `reason` | `"Learn the session protocol without a model service."` | Explains the choice in the JSON form of the routing notice |

The `provider`, `model` and `effort` labels do not select a model service.
A real configuration script puts the options of its harness in `args`. The labels only
report the choice.

This file is Rhai, the small language that Ravel contains.
`SCHEMA_VERSION` identifies the interface between Ravel and the script, and
`VERSION` names this revision of the owner's script. `fn` defines a function, and `#{ ... }` constructs a map.
The final expression returns that map. Ravel checks its fields during evaluation.

The function accepts a request and selection context. This example needs only
`request.prompt`, so it leaves the second argument unused.
[Chapter 4](04-write-a-configuration-script.md#define-a-configuration-script-and-read-the-request)
teaches the syntax and the other fields.

## Check the request and evaluate the configuration script

The Ravel CLI first reads the settings and the command-line inputs and checks
the configuration script. `--config config.rhai` names a file relative to the
caller's working directory. Ravel resolves its real path and requires a
readable regular file. It does not search the caller's repository for a
different configuration script. Settings still come from `$HOME/.config/ravel/settings.json`, even
when `--config` is present. The private `home` of the example is empty, so the
defaults apply.

The Ravel CLI launches a second copy of its own binary, the Rhai worker, in
the worker's empty private directory. At the deadline, the Ravel CLI ends
evaluation and can signal the separate worker. A kernel stall can delay the
worker's exit. A memory or stack failure in the script ends only the worker.
The Ravel CLI checks the identity of the worker before it sends the evaluation
request.
The worker receives an empty environment and no
[session-result context](02-actors-and-words.md#three-different-kinds-of-information),
the `RAVEL_EXIT_FILE` path with which a program sends the session result.

The `cwd` of the request keeps the caller's working directory.
The configuration script has no direct filesystem access. Ravel reads `config.rhai` and
sends its source to the worker, which evaluates it and calls `select`.

The configuration script has no `load_context` function, and the caller
supplied no
[selection context](02-actors-and-words.md#three-different-kinds-of-information),
the optional JSON data that informs the choice.
In this example, the selection therefore depends only on the request.
The second argument to `select` is `()`, Rhai's value for no value.

A configuration script that needs files or model advice asks Ravel through
[host capabilities](02-actors-and-words.md#evaluation-and-supervision),
the `ravel::` functions that the Ravel CLI performs for the script.
Ravel measures each successful file read and supervises each evaluation
[harness run](02-actors-and-words.md#evaluation-and-supervision). This script needs neither operation.

The worker sends its response through a private socket. The Ravel CLI checks
the metadata of the script and the selected command. Anything that the worker prints
is diagnostic text, not a selection.

The worker and the managed process have different roles in this example:

| Property | Rhai worker | Managed process |
|---|---|---|
| Purpose | Evaluate the configuration script | Execute the selected command |
| Working directory | Starts in a private directory. The script cannot read files directly or change that directory. | Starts in the caller's working directory |
| Session-result context | None | Receives its own `RAVEL_EXIT_FILE` path |
| Response to Ravel | Selection or refusal through a private socket | Opaque result through the [result channel](02-actors-and-words.md#the-managed-process-and-its-result) |

After the response, the Ravel CLI confirms that the worker ended and that the
Ravel CLI reaped it. It also removes the private directory of the worker. Only
then does it resolve the selected executable and allow the managed launch.
Any host call also ends before that launch. If
the worker cleanup fails, Ravel does not launch, even after a valid response.

Here, Ravel searches the caller's `PATH` for `fake-agent`. The first directory
in that `PATH` is the `bin` directory of the example. The selected program
starts in the caller's working directory. Ravel executes the selected program
and argument list directly. It does not use shell expansion, and it does not
split the prompt into words. The fake still runs in a shell, because its
first line, `#!/bin/sh`, tells the operating system to run the file with
`/bin/sh`.

## Launch the managed job

After selection, Ravel makes a new
[result channel](02-actors-and-words.md#the-managed-process-and-its-result).
This channel is a private temporary directory and a path where a program can
send a result. Ravel puts that path in the `RAVEL_EXIT_FILE` environment
variable of the managed process. This path is the session-result context of
the managed process. Here, the managed process is the fake agent.

How does `end-session` find that path later? Each child process gets a copy
of the environment of its parent, unless the parent changes it. The fake
starts `ravel end-session` as its child, so that command inherits the same
`RAVEL_EXIT_FILE`. In a real run on macOS with `TMPDIR=/tmp`, the path was
`/private/tmp/ravel-session-mlcN86/signal-c7d5f2eed4198eb768996f7b4868bbcc`.
The random parts change in each run. The `PATH` of the example also lets the
fake find the same installed `ravel` for `end-session`.

The [runner](02-actors-and-words.md#evaluation-and-supervision) in
Ravel launches the managed process in a process group of its own. A
**process group** is a set of processes. A program can send one signal to all
processes in the group. A child inherits the group of its parent unless it moves to another group.
Thus, with one signal to the group, Ravel can stop the fake and each
descendant that stays in that group.

The managed process and the members of its group form the **job** that Ravel
supervises. Here, “job” has its job-control meaning: a process group that a
program controls as one unit. Ravel stays the parent of the managed process
while it supervises the process. `start-session` does not replace itself with
the selected program.

The new process group does not make a new Ravel session or a new login.
These three terms name different units:

| Unit | What it means | In this unconfined example |
|---|---|---|
| Ravel session | One `start-session` invocation | Covers selection, the managed job and cleanup |
| Process group | Processes that can receive a signal as one group | The managed process leads a new group |
| POSIX session | An operating-system unit that job control uses | The managed process stays in the caller's POSIX session |

A [confined](02-actors-and-words.md#trust-and-confinement) launch uses a new
POSIX session. [Chapter 5, “Run sessions”](05-running-sessions.md#confine-the-managed-process),
teaches that separate case.

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

If you start Ravel from an interactive terminal, one more group is important: the
**foreground process group** of the terminal. The controlling terminal sends
keyboard input to that group. It also sends the signals that the terminal
makes, such as INT from Ctrl-C, to that group.

If Ravel's own process group is the foreground group, Ravel saves the
terminal modes. Then it makes the group of the managed process the foreground
group. Ravel does these two steps before the managed process executes the
selected program. This book says that Ravel lends the terminal. The selected
program can then read from the terminal as a foreground job. If a different
job has the foreground, Ravel launches the program and does not lend the
terminal.

If Ravel lent the terminal, it makes its own group the foreground group again
at completion and restores the saved modes. The modes include settings such
as whether the terminal shows typed characters. This recovery is important
when a program changes the modes. The fake does not read input or change
terminal modes. The automatic example check runs without a terminal. Thus,
its transcript shows the choice of command and the delivery of the result,
not a change of the foreground group.

[Chapter 9, “Jobs, signals and the terminal”](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal), gives the terminal rules. These
rules include, for each case, the process group from which Ravel recovers the
terminal. [Chapter 9](09-jobs-signals-and-the-terminal.md#preserve-the-entry-signal-state) also gives signal masks and [failure guarantees](09-jobs-signals-and-the-terminal.md#know-the-guarantees-and-their-limits).

## Send the result and finish the job

The fake prints its argument and calls `ravel end-session -- "finished"`.
That command reads the inherited `RAVEL_EXIT_FILE` path. It reads no
configuration and needs no session ID. It writes the complete value into a
temporary file. Then it sends the value into the result channel, but it does
not replace a result that is already there. Ravel keeps the first complete
result that it receives.

Follow that command to its result. When `end-session` sends the result, it
asks the Ravel CLI to end the session. Ravel then stops the fake and the rest
of its process group. After cleanup, `finished` becomes the result of
`ravel start-session`.

The request does not mean that the job already ended. Also,
`end-session` does not wait for the cleanup of the enclosing session. In this
fake, the script has no more commands, so it exits.

A managed process can continue to run after it sends the result. The runner
then uses these stages. [Chapter 9](09-jobs-signals-and-the-terminal.md#follow-the-escalation-after-a-result) gives the exact times.

| Stage | When it starts | Runner action |
|---|---|---|
| Grace period | The runner detects the sent result | Allows time for the managed process to exit |
| TERM | The managed process still runs after the grace period | Sends TERM to the process group, then allows a second grace period |
| KILL | The managed process still runs after the second grace period | Sends KILL to the process group |

The exit of the managed process does not complete the cleanup. Before it
reaps the direct child, Ravel stops the members of the process group that
still run. To **reap** a finished child, the parent collects its exit status. This
action also releases the last record of that child in the operating system.

The managed process leads its group. Thus, the ID of the group is the PID of
the managed process. The operating system does not use that PID again until
Ravel reaps the process. Thus, Ravel sends the last group signals before the
reap, so that they cannot go to an unrelated group that took the same ID.

Then, if the runner lent the terminal, it recovers the terminal. It also
confirms that the process group is gone. The Ravel CLI checks and reads the
sent result. It removes the temporary directory of the result channel. Then,
before it writes to the caller's standard output, it checks the outcomes of
cancellation and cleanup. The appearance of the result file does not remove
any of these steps.

Success means that the result reaches the caller only after Ravel reaps the
managed process, the whole process group is gone and cleanup succeeds.
`start-session` exits with status 0 only if the final write also succeeds. If
the write fails, the caller can receive only the start of the result, and
the invocation fails. Thus, bytes on standard output alone do not prove success.

That guarantee is for the process group of the managed process. It does not
include these three cases:

- a descendant that leaves the group
- a descendant of an evaluation harness run that leaves its group
- a KILL that stops the Ravel CLI (the Ravel CLI cannot catch KILL)

The [trust limits in chapter 2](02-actors-and-words.md#trust-and-confinement)
also explain why code that runs as the same user can change files or
launch unrelated processes. A
successful session shows that Ravel delivered a sent result after it confirmed
the cleanup. It does not show that the work that the prompt requests is
correct.

The sequence below shows two orders that Ravel guarantees. Worker
cleanup comes before the managed launch, and job cleanup comes before the
delivery of the result. The figure follows the successful path of this
chapter, without confinement.

In this UML sequence, an X ends the pictured lifetime after confirmed reap or
removal, not at the instant a process exits. The runner shares the Ravel
CLI's lifeline because it runs inside that process.

![The caller requests a session. The Ravel CLI checks the request, evaluates the configuration script in its Rhai worker and confirms the cleanup of the worker. Then it launches the managed process, which sends the result to the temporary result channel. The Ravel CLI stops the group before it reaps the direct child, and recovers the terminal if it lent it. It confirms that the group is gone, reads the result, removes the temporary files and decides the final outcome. Then it returns the result unchanged to the caller.](figures/successful-session.svg)

*Figure 3.1 — The sent result requests completion. The final write of the
result requires successful cleanup and no cancellation.*
[Open the successful-session sequence at full size](figures/successful-session.svg).

In words, the Ravel CLI checks the request, the identity of the worker and
the response of the configuration script. Then it confirms the reap of the worker and the removal
of the private directory of the worker. It resolves the executable, makes
the result channel and launches the managed process. `end-session` sends the
value into that channel. It does not return the value to the caller.

The Ravel CLI stops the group of the managed process before the reap. It
recovers a lent terminal and confirms that the group is gone. It reads the
value and removes the temporary directory of the channel. Then it decides the
outcome. That decision uses any cancellation or cleanup problem. Then it
writes the value unchanged to standard output and flushes it.

The text above explains each stage. The figure does not show times or
individual system calls.

## Recognise the other outcomes

Refusal, cancellation, natural exit without a result and failure are normal
outcomes. Each one is different from the successful example and from the
others. A caller needs to recognise each one.

- A [refusal](02-actors-and-words.md#evaluation-and-supervision) occurs
  before the final managed launch. Evaluation harness runs can occur before
  the refusal. It occurs when the configuration script declines the
  request, or when Ravel cannot accept the file, behaviour or data of the
  script. Ravel
  does not try a different route and returns no result to the caller.
- Cancellation by an INT, TERM or HUP signal can occur during selection or
  supervision. The session then returns no result to the caller, even if a
  program already sent a value.
- A natural exit without a sent result also returns no result to the caller,
  even when the managed process exits with status 0. An empty sent result is
  different. A program sent an empty value, and the session can succeed after
  cleanup.
- A failure of the worker, of supervision or of cleanup prevents success. A
  failure of the final write of the result also makes the invocation fail
  after cleanup.

Also, a managed process can send a result and later exit with a nonzero
status. That exit does not make the result invalid. Ravel checks the sent
result, cancellation and cleanup separately from the exit status of the
program. [Chapter 5](05-running-sessions.md#interpret-the-exit-status) teaches
the outcomes and their exit codes with examples.

The state machine below shows why a session with a sent value can end
without a successful result. It separates the phase in which work stops from
the outcome that the caller receives after the cleanup attempts.

![Selection allows a managed launch or stops before the launch. After selection or supervision, Ravel does the cleanup and decides the final outcome. Cancellation returns no result to the caller. Otherwise, a refusal, a natural exit without a sent result or a failure can stay the primary outcome. Only a valid result with a confirmed reap, the absence of the group and cleanup reaches the final write. A successful write and flush complete the session, and a failed write makes the session fail.](figures/session-outcomes.svg)

*Figure 3.2 — The sent result, cleanup and the final write are separate
conditions of success. Cancellation returns no result to the caller, even after a program sends a value.*
[Open the outcome state machine at full size](figures/session-outcomes.svg).

In words, a refusal stops selection before the launch. Cancellation and
failure can stop selection or supervision. A natural exit without a sent
result is part of supervision, even if the managed process exits with status
0. On each path, Ravel attempts the necessary cleanup.

Then Ravel decides the final outcome, and cancellation has priority. If no
cancellation occurred, the outcome is the original refusal or failure. A
missing result is one of these failures. Ravel reports any further cleanup problems
next to that outcome. A cleanup failure
prevents success, even when everything else succeeds.

Only a valid sent value with no cancellation, a confirmed reap, confirmed
absence of the group and confirmed cleanup reaches the final write and flush. An
empty sent result is a value. A later nonzero exit of the managed process
does not, by itself, make the value invalid.

The machine shows the cleanup operations above as a group. It does not mean
that a failed cleanup completed. Its cancellation conditions apply until
Ravel decides the final outcome, and also at that decision. A signal that
arrives after that decision can still stop the Ravel CLI while it writes the
result. A failed write can leave only the start of the result on standard
output. [Chapter 8](08-follow-a-session-through-the-code.md#choose-the-final-outcome)
gives the full outcome decision.

To write rules in the configuration script beyond this single fake, continue with
[chapter 4, “Write a configuration script”](04-write-a-configuration-script.md).
To invoke Ravel from your own tool, continue with
[chapter 5, “Run sessions”](05-running-sessions.md).
[Chapter 8, “Follow a session through the code”](08-follow-a-session-through-the-code.md), follows this
same sequence into the implementation.
