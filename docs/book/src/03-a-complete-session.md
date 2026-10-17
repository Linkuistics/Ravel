# A complete session, end to end

Run one [session](02-actors-and-words.md#caller-and-owner) with a small
[owner policy](02-actors-and-words.md#caller-and-owner) and a fake agent, then
follow the request through selection, launch, publication and cleanup. You
act as both [owner and caller](02-actors-and-words.md#caller-and-owner): you
write the routing rule, supply the prompt and choose the result you expect.
The fake is an ordinary shell program. It needs no model service or agent
account.

The example returns `finished`. That word is your convention, not a Ravel
command to continue or stop other work. Ravel returns it only after the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
has been reaped, its whole process group is gone and cleanup has succeeded.
The final write to standard output must also succeed for `start-session` to
exit with status 0.

## Obtain the build used by this chapter

This chapter describes the unreleased source checkout accompanying this book,
which includes changes after v1.0.0. Use a build from that checkout for the
example. A v1.0.0 release installation does not contain those later changes,
even if the checkout's version string still says `1.0.0`.

On a macOS or GNU/Linux host, have Rust 1.85 or later with Cargo,
Bash 4 or later, Task, curl, tar with gzip, bzip2 and xz support, and a SHA-256
utility (`shasum` or `sha256sum`) available. Run the
following commands from the root of this checkout. `task setup` fetches the
pinned build tools and worker dependencies. `task install` builds the
[Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) and the
worker, installs them as a matched pair in the named prefix, and runs a smoke
test from that installation alone. The prefix below stays inside the checkout,
and its `bin` directory goes first on your shell's `PATH`.

Fragment: build and installation setup; this block omits platform-specific
installation of the prerequisite tools and their build output.

```sh
task setup
task install PREFIX="$PWD/target/book-install"
export PATH="$PWD/target/book-install/bin:$PATH"
```

The installation contains both the Ravel CLI, `bin/ravel`, and its matching
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision),
`libexec/ravel/ravel-policy`. Keep the pair together: the Ravel CLI finds the
worker relative to its own real path and stops with an error if the worker is
missing or does not match. The worker embeds the Bun runtime, so using the
installation needs no separately installed Bun or Node. The Homebrew formula installs the same paired
layout from release archives; the checkout build here supplies the code this
chapter describes.

The [platform evidence in chapter 1](01-what-ravel-is-for.md#understand-the-evidence-limits)
distinguishes the measured aarch64 macOS and aarch64 GNU/Linux targets from
x86_64 GNU/Linux, which has only emulated execution evidence. Chapter 12,
“Delivering Ravel”, covers installation and target details.

## Run the first session

Create an empty directory for the example and enter it in the shell where you
set `PATH`. Create its `bin` subdirectory, then save the two files below at
the labelled paths. The command creates private `home` and `tmp` directories
and makes the fake executable. The command's cleared environment (`env -i`)
and explicit `--config` keep your normal owner settings and policy out of this
session.

Complete example: pass a prompt through a tiny policy to a fake agent, which
prints the prompt and publishes `finished`.

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

The standard output is exactly the eight bytes `finished`; Ravel adds no
newline. In an interactive shell, the next prompt therefore starts on the same
line unless the shell itself marks or supplies the missing newline. The
standard-error block shows the two stable lines, not a promise that the stream
contains only those lines. Ravel can also report cleanup operations with
details, such as process IDs, that differ between runs. On macOS, a stop
attempt against an already-exited group can report `Operation not permitted`;
Ravel keeps that report on standard error and permits success only after
confirming reap, terminal recovery and group absence. A report is not itself
proof of either success or failure: check the exit status of `start-session`.

The fake's `printf` writes to its own standard output, yet the line appears
on the caller's standard error. Ravel reserves the caller's standard output
for the [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
and, before launch, redirects the managed process's standard output to the
caller's standard error. The fake does
not interpret the prompt; its script is deliberately written to perform the
ending action the prompt requests. Ravel neither understands that instruction
nor inserts it.

The policy selects the fake for every request. `demo` is the caller's
[kind](02-actors-and-words.md#caller-and-owner), and this policy does not need
to branch on it. `provider`, `model` and `effort` are descriptive owner labels,
which you can see in the starting notice. They do not select a model service.
The actual command is `program` with the strings in `args`; here the unchanged
prompt becomes one argument. `definePolicy` returns the policy object
unchanged and gives it the SDK's TypeScript type, so a type checker can check
its fields; Ravel checks the values when it evaluates the policy.

## Check the request and evaluate the policy

The Ravel CLI first reads the settings and command-line inputs and checks the
policy file. `--config policy.ts` names a file relative to the caller's
working directory; Ravel resolves its real path and requires a readable
regular file. It does not search the working repository for another policy.
Settings still come from `$HOME/.config/ravel/settings.json`, even with
`--config`. The example's private `home` is empty, so the defaults apply.

The Ravel CLI finds its matched worker, starts it in the worker's empty private
directory and checks the worker's identity before sending the policy request.
The worker receives a restricted environment and no
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
It changes to `/` before importing owner policy. The request's `cwd` preserves
the caller's working directory; the worker's directory does not redefine it.

The worker imports `policy.ts` and calls `select`. The literal import
`ravel/sdk` resolves to the worker's embedded SDK, so this example needs no
local dependency installation. The policy has no policy-context loader and the
caller supplied no [policy context](02-actors-and-words.md#three-different-kinds-of-information).
Selection therefore depends only on the request in this example.

The worker sends its response through a private socket. The Ravel CLI checks
the policy metadata and the selected command; anything the worker prints is
diagnostic text, not a selection. After the response, the Ravel CLI confirms
that the worker has ended and been reaped, and removes the worker's private
directory. Only then does it resolve the selected executable and allow the
managed launch. If worker cleanup fails, Ravel does not launch, even after a
valid response.

Here, resolving `fake-agent` searches the caller's `PATH`, whose first
directory is the example's `bin`. The selected program starts in the caller's
working directory. Ravel executes the selected program and argument list
directly, without shell expansion or splitting the prompt into words. The fake's
shebang makes this particular program a shell script.

## Launch the managed job

After selection, Ravel allocates a fresh
[result channel](02-actors-and-words.md#the-managed-process-and-its-result):
a private temporary directory and a path where a result can be published.
It gives the managed process that path, its session-result context, in
`RAVEL_EXIT_FILE`. The example's `PATH` also lets the fake find the same
installed `ravel` for `end-session`.

Ravel's [runner](02-actors-and-words.md#policy-evaluation-and-supervision)
launches the managed process in a process group of its own. A **process
group** is an operating-system grouping to which a signal can be sent as a
unit. Children normally inherit their parent's group, so the group lets Ravel
stop the fake and any descendants that stay in it together. The managed
process and the members of its group form the **job** Ravel supervises, in the
job-control sense of a process group managed as one unit. Ravel remains the
managed process's parent while supervising it; `start-session` does not
replace itself with the selected program.

This is still one Ravel session. In this unconfined launch, making a process
group does not create a new POSIX session or log the process in again. A POSIX
session is a different operating-system grouping associated with job control;
the unconfined process remains in the caller's POSIX session. A
[confined](02-actors-and-words.md#trust-and-confinement) launch uses a new
POSIX session, a separate case that [chapter 5, “Running sessions”](05-running-sessions.md#confine-the-managed-process), teaches.

If you run from an interactive terminal, another group matters: the
terminal's **foreground process group**. The controlling terminal delivers
keyboard input and terminal-generated signals, such as INT from Ctrl-C, to
that foreground group. When Ravel's own process group is the foreground
group, Ravel saves the terminal modes and makes the managed process's group
the foreground group before that process executes the selected program. The
book calls this handoff lending the terminal. The selected program can then
read from the terminal as a foreground job. If another job holds the
foreground, Ravel launches without lending the terminal.

If it lent the terminal, Ravel makes its own group the foreground group again
on completion and restores the saved modes. The modes include settings such
as whether typed characters are echoed; recovery matters when a program
changes them. The fake does not read input or alter terminal modes. The
automated example check runs without a terminal, so its transcript
demonstrates routing and result delivery, not a foreground handoff. Chapter 9,
“Jobs, signals and the terminal”, develops the terminal rules, including which
process group Ravel takes the terminal back from in each case, together
with signal masks and failure guarantees.

## Publish the result and finish the job

The fake prints its argument and calls `ravel end-session -- "finished"`.
That invocation reads the inherited `RAVEL_EXIT_FILE` path. It reads no policy
or settings and needs no session ID. It writes the complete value into a
temporary file and publishes it without replacing any result already there.
The first complete publication wins.

Publication asks the Ravel CLI to finish the session. It does not mean that
the job has already ended, and `end-session` does not wait for the enclosing
session's cleanup. In this fake, the script has no further command and exits.
For a managed process that stays running, the runner gives it a grace period
after noticing publication, then sends TERM and finally KILL if needed. These
signals go to the managed process's group. Chapter 9 gives the detailed
timing.

The managed process's exit does not finish cleanup. Ravel stops remaining
members of the process group before reaping the direct child. **Reaping** is
the parent collecting a finished child's exit status and releasing the
operating system's remaining record of that child. The managed process leads
its group, so the group's ID is the managed process's PID, and the operating
system does not reuse that PID until the process is reaped. Sending the final
group signals before reap therefore avoids directing them at an ID that an
unrelated group could have taken.

The runner then recovers the terminal if it lent it, and confirms that the
process group is gone. The Ravel CLI validates and reads the published
result, removes the result channel's temporary directory, and checks
cancellation and cleanup outcomes before writing to the caller's standard
output. The appearance of a result file does not bypass any of those steps.

Success means that the result reaches the caller only after the managed
process has been reaped, its whole process group is gone and cleanup has
succeeded. A successful final write gives `start-session` status 0. If writing
fails, the caller may have received only a prefix and the invocation fails;
bytes on standard output alone do not prove success.

That guarantee covers the managed process's group. Three cases lie outside
it: a descendant that leaves the group, a detached child started by trusted
policy, and the Ravel CLI being killed by SIGKILL, which it cannot catch.
The
[trust limits in chapter 2](02-actors-and-words.md#trust-and-confinement)
also explain same-user tampering. A successful session shows that Ravel
delivered a published result after confirmed cleanup, not that the work
requested in the prompt is correct.

The sequence below shows the two boundaries a caller relies on: worker
cleanup precedes the managed launch, and job cleanup precedes result delivery.
It follows this chapter's unconfined successful path.

In this UML sequence, an X ends the pictured lifetime after confirmed reap or
removal, not at the instant a process exits. Blue participants are Ravel
processes, grey ones are external, and the gold result channel holds temporary
files. Green arrows carry the opaque result. The runner shares the Ravel
CLI's lifeline because it runs inside that process.

![The caller requests a session. The Ravel CLI checks the request, evaluates policy in its worker and confirms worker cleanup before launching the managed process. The managed process publishes to the temporary result channel. The Ravel CLI stops the group before reaping the direct child, recovers the terminal if lent, confirms group absence, reads the result, removes temporary files and decides the final outcome before delivering the result to the caller.](figures/successful-session.svg)

*Figure 3.1 — Publication requests completion; the final result write requires
successful cleanup and no cancellation.*
[Open the successful-session sequence at full size](figures/successful-session.svg).

In words, the Ravel CLI checks the request, checks the worker's identity,
checks its policy response, and confirms worker reap and private-directory
removal.
It then resolves the executable, allocates the result channel and launches
the managed process. Publication puts the value in that channel, without
returning it to the caller. The Ravel CLI stops the managed process's group
before reap, recovers a lent terminal and confirms group absence. It reads the
value, removes the channel's temporary directory and decides the outcome,
taking account of any cancellation or cleanup problem, before writing and
flushing the value. Each stage is explained above; the
figure omits timing and individual system calls.

## Recognise the other outcomes

Refusal, cancellation, natural exit without a result and failure are normal
outcomes, distinct from the successful example and from each other. A caller
needs to recognise each one.

- A [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) occurs
  before launch when owner policy declines the request or Ravel cannot accept
  the policy's file, behaviour or data. Ravel does not try another route.
- Cancellation by an INT, TERM or HUP signal can occur during selection or
  supervision. It suppresses the result even if a value was already published.
- Natural exit without publication supplies no result, even when the managed
  process exits with status 0. An explicitly empty result is different: a
  value was published, and successful cleanup can complete that session.
- A worker, supervision or cleanup failure prevents success. A failure of the
  final result write also fails the invocation after cleanup.

Conversely, a managed process can publish a result and later exit nonzero
without invalidating that result. Ravel checks publication, cancellation and
cleanup separately from the program's exit status. [Chapter 5](05-running-sessions.md#interpret-the-exit-status) teaches the
outcomes and their exit codes with examples.

The state machine below shows why a session with a published value can still
end without a successful result. It distinguishes the phase in which work
stops from the outcome the caller receives after cleanup attempts.

In this UML state machine, blue boxes are phases of Ravel's work, not
processes. Grey boxes are outcomes. The green route is the only path to
successful result delivery.

![Selection either allows a managed launch or stops before launch. Selection and supervision both lead to cleanup and the final outcome decision. Cancellation suppresses a result; otherwise a refusal, natural exit without publication or failure can remain the primary outcome. Only a valid result with confirmed reap, group absence and cleanup reaches the final write. Successful writing and flushing complete the session; a failed write fails it.](figures/session-outcomes.svg)

*Figure 3.2 — Publication, cleanup and the final write are separate conditions
of success; cancellation can suppress a published value.*
[Open the outcome state machine at full size](figures/session-outcomes.svg).

In words, a refusal stops selection before launch, while cancellation and
failure can stop either selection or supervision. Natural exit without
publication belongs to supervision, even if the managed process exits 0.
Every path attempts its required cleanup. When Ravel then decides the final
outcome, cancellation comes first. Otherwise Ravel keeps an original refusal
or failure, including the missing-result outcome, and reports any further
cleanup problems beside it. A cleanup failure prevents an otherwise successful result. Only a valid
published value with no cancellation and confirmed reap, group absence and
cleanup reaches the final write and flush. An empty publication is a value,
and the managed process's later nonzero exit does not by itself invalidate it.

The machine groups the cleanup operations described above; it does not imply
that failed cleanup was completed. Its cancellation conditions apply up to
and including the moment Ravel decides the final outcome. A signal that arrives after that decision
can still terminate the Ravel CLI during writing, and a failed write may leave
only a prefix on standard output. Chapter 8 develops the full outcome decision.

To write routing rules beyond this single fake, continue with
[chapter 4, “Writing an owner policy”](04-writing-an-owner-policy.md).
To invoke Ravel from your own tool, continue with
[chapter 5, “Running sessions”](05-running-sessions.md).
Chapter 8, “Following a session through the code”, follows this
same sequence into the implementation.
