# A complete session, end to end

Run one [session](02-actors-and-words.md#caller-and-owner) with a small
[owner policy](02-actors-and-words.md#caller-and-owner) and a fake agent.
Then follow the request through selection, launch, the sent result and
cleanup. You act as both [owner and caller](02-actors-and-words.md#caller-and-owner).
You write the rule in the owner policy, supply the prompt and choose the
result that you expect. The fake is an ordinary shell program. It needs no
model service or agent account.

The example returns `finished`. That word is your convention, not a Ravel
command to continue or stop other work. Ravel returns this word only after it
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
Markdown or local build. [Chapter 12](12-deliver-ravel.md#keep-the-book-with-its-release)
explains this limit and the address of each release's book.

The directory that `brew --prefix ravel` prints contains the
[Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision), `bin/ravel`, and the
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
that matches it, `libexec/ravel/ravel-policy`. Keep the two files together.
The Ravel CLI finds the worker relative to its own real path. If the worker
is missing or does not match, the Ravel CLI stops with an error.

The worker contains the Bun runtime. Thus, you do not need to install Bun or
Node.

The [platform evidence in chapter 1](01-what-ravel-is-for.md#understand-the-evidence-limits)
separates the measured aarch64 macOS and aarch64 GNU/Linux targets from
x86_64 GNU/Linux. For x86_64 GNU/Linux, the only execution evidence comes from
emulation. [Chapter 12, “Deliver Ravel”](12-deliver-ravel.md), gives the installation and target
details.

## Run the first session

Use the shell where `ravel --version` succeeded. Create an empty directory
for the example, then make it the current directory. Create a `bin`
subdirectory in it. Then save the two files below at the labelled paths.

The command below creates private `home` and `tmp` directories and makes
the fake executable. The command clears the environment (`env -i`) and gives
an explicit `--config`. Thus, this session does not use your usual owner
settings or policy.

Complete example: pass a prompt through a small policy to a fake agent. The
fake prints the prompt and sends `finished`.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "first-session",
  select(request) {
    return {
      status: "selected",
      program: "fake-agent",
      args: [request.prompt],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Learn the session protocol without a model service.",
    };
  },
});
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
$ mkdir -p home tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Print this prompt, then call ravel end-session -- "finished".'
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

The policy selects the fake for every request. `demo` is the caller's
[kind](02-actors-and-words.md#caller-and-owner), and this policy does not need
to examine it. The routing notice is the `ravel: starting` line on standard
error. The policy returns these fields:

| Field | Value in this example | Effect |
|---|---|---|
| `program` | `fake-agent` | Names the program to execute |
| `args` | `[request.prompt]` | Passes the unchanged prompt as one argument |
| `provider` | `local` | Describes the provider in the routing notice |
| `model` | `fake` | Describes the model in the routing notice |
| `effort` | `none` | Describes the effort in the routing notice |

The `provider`, `model` and `effort` labels do not select a model service.
`definePolicy` returns the
policy object unchanged and gives it the TypeScript type of the SDK. Thus, a
type checker can check its fields. Ravel checks the values when it evaluates
the policy.

## Check the request and evaluate the policy

The Ravel CLI first reads the settings and the command-line inputs and checks
the policy file. `--config policy.ts` names a file relative to the caller's
working directory. Ravel resolves its real path and requires a readable
regular file. It does not search the caller's repository for a different
policy. Settings still come from `$HOME/.config/ravel/settings.json`, even
when `--config` is present. The private `home` of the example is empty, so the
defaults apply.

The Ravel CLI finds the worker that matches it and starts it in the empty private
directory of the worker. It checks the identity of the worker before it sends
the policy request. The worker receives a restricted environment and no
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
It changes to `/` before it imports the owner policy. The `cwd` of the request
keeps the caller's working directory. The directory of the worker does not
change that value.

The worker imports `policy.ts` and calls `select`. The literal import
`ravel/sdk` resolves to the embedded SDK of the worker. Thus, this example
needs no local installation of dependencies. The policy has no policy-context
loader, and the caller supplied no
[policy context](02-actors-and-words.md#three-different-kinds-of-information).
In this example, the selection therefore depends only on the request.

The worker sends its response through a private socket. The Ravel CLI checks
the policy metadata and the selected command. Anything that the worker prints
is diagnostic text, not a selection.

The worker and the managed process have different roles in this example:

| Property | Policy worker | Managed process |
|---|---|---|
| Purpose | Evaluate the owner policy | Execute the selected command |
| Working directory | Starts in a private directory, then changes to `/` before the policy import | Starts in the caller's working directory |
| Session-result context | None | Receives its own `RAVEL_EXIT_FILE` path |
| Response to Ravel | Policy response through a private socket | Opaque result through the [result channel](02-actors-and-words.md#the-managed-process-and-its-result) |

After the response, the Ravel CLI confirms that the worker ended and that the
Ravel CLI reaped it. It also removes the private directory of the worker. Only
then does it resolve the selected executable and allow the managed launch. If
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
send a result. Ravel gives that path to the managed process in
`RAVEL_EXIT_FILE`. This path is the session-result context of the managed
process. The `PATH` of the example also lets the fake find the same installed
`ravel` for `end-session`.

The [runner](02-actors-and-words.md#policy-evaluation-and-supervision) in
Ravel launches the managed process in a process group of its own. A
**process group** is a set of processes. A program can send one signal to all
processes in the group. A child usually inherits the group of its parent.
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
That command reads the inherited `RAVEL_EXIT_FILE` path. It reads no policy
or settings and needs no session ID. It writes the complete value into a
temporary file. Then it sends the value into the result channel, but it does
not replace a result that is already there. Ravel keeps the first complete
result that it receives.

When `end-session` sends the result, it asks the Ravel CLI to end the
session. This request does not mean that the job already ended. Also,
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
- a detached child that trusted policy starts
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

![The caller requests a session. The Ravel CLI checks the request, evaluates the policy in its worker and confirms the cleanup of the worker. Then it launches the managed process, which sends the result to the temporary result channel. The Ravel CLI stops the group before it reaps the direct child, and recovers the terminal if it lent it. It confirms that the group is gone, reads the result, removes the temporary files and decides the final outcome. Then it returns the result unchanged to the caller.](figures/successful-session.svg)

*Figure 3.1 — The sent result requests completion. The final write of the
result requires successful cleanup and no cancellation.*
[Open the successful-session sequence at full size](figures/successful-session.svg).

In words, the Ravel CLI checks the request, the identity of the worker and
the policy response. Then it confirms the reap of the worker and the removal
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

- A [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) occurs
  before the launch. It occurs when the owner policy declines the request, or
  when Ravel cannot accept the file, behaviour or data of the policy. Ravel
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

To write rules in the owner policy beyond this single fake, continue with
[chapter 4, “Write an owner policy”](04-writing-an-owner-policy.md).
To invoke Ravel from your own tool, continue with
[chapter 5, “Run sessions”](05-running-sessions.md).
[Chapter 8, “Follow a session through the code”](08-follow-a-session-through-the-code.md), follows this
same sequence into the implementation.
