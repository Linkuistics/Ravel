# Jobs, signals and the terminal

A result requests the end of a job. It does not prove that the job has ended.
The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) stays alive to supervise that end
before it returns the result to the caller.
The [runner](07-the-parts-of-ravel.md#the-runner-owns-the-local-job-mechanics)
inside the CLI supplies the process and terminal operations.

[Chapter 3](03-a-complete-session.md#send-the-result-and-finish-the-job)
introduced a job and its reap through one session. This chapter explains
why the order matters, how Ravel ends a stubborn process, and what confinement
changes. [Chapter 8](08-follow-a-session-through-the-code.md#finish-the-managed-job)
shows where these operations occur in the code.

## The managed process leads a job

The [managed process](02-actors-and-words.md#the-managed-process-and-its-result)
is the direct child that Ravel starts from the selected command.
For an ordinary unconfined launch, Ravel creates a new **process group** for
that child. The group's ID is the child's PID. Descendants inherit this group
unless they explicitly change it. The managed process and the members of its
group form the **job**.

A signal to a process group can reach its members together. A signal to the
child's PID reaches only that child. If Ravel stopped only the child, a helper
could retain a file lock or continue to change the caller's files.
The group gives Ravel a scope for shutdown that includes ordinary descendants.

A **POSIX session** can contain several process groups and can have a
controlling terminal. An ordinary unconfined launch keeps the caller's POSIX
session. Thus, Ravel can give the terminal's foreground to the new group.
The terminal rules select which group can read terminal input and receive
signals from the keyboard.

| Choice for the managed launch | Consequence |
|---|---|
| A new process group in the caller's POSIX session | Ravel can signal the job together and can lend the caller's terminal |
| A new POSIX session through `setsid` | The child starts without the caller's controlling terminal. This suits the detached modes below |
| Only the child's PID as the shutdown target | Descendants can survive their parent. This does not meet Ravel's cleanup contract |

A descendant can leave the job's group, for example through its own
`setsid` call. Group cleanup does not cover that descendant. Ravel does not
promise to track every process descended from the managed process.
A new process group also creates no new Ravel session: that term means one
`start-session` invocation.

## Follow the escalation after a result

An interactive harness can finish its work and then wait for another prompt.
The caller therefore tells it to send an
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
with `end-session`. The command creates the completion file in the
[result channel](07-the-parts-of-ravel.md#the-result-channel-carries-the-value).
The runner observes that file's presence. Ravel checks its contents separately.

The runner normally checks the child and channel at 500-millisecond intervals.
Its first observation of the file starts the grace period. Thus, the schedule
starts at observation, not at the instant the sender calls `end-session`.

| Stage | Condition | Action and wait |
|---|---|---|
| Grace | The runner observes the completion file while the child still runs | Allow 2 seconds for the child to exit |
| TERM | The child still runs after the grace | Send SIGTERM to the group and the child. Allow up to 5 seconds for the child to exit |
| KILL | The child still runs after the kill grace | Send SIGKILL to the group and the child. Check for exit at 10-millisecond intervals |
| Final cleanup | The runner observes the child's exit | Stop the other group members before the reap, then recover the terminal and confirm group absence |

SIGTERM gives the program a chance to handle shutdown. A program can ignore
SIGTERM. It cannot catch or ignore SIGKILL. If the child exits during either
grace period, Ravel proceeds to final cleanup without the later escalation stage.
Descendants receive no separate grace period after the child exits.

The 2-second and 5-second values belong to the Ravel CLI. The runner accepts
its caller's durations. These values are not a seven-second deadline for
`start-session`: selection, observation delays and final cleanup also take time.
Without a result or cancellation, this escalation places no time limit on the
managed process.

After the escalation's first KILL attempt, the runner uses a two-second bound
for its exit checks and reap attempts. It does not restart that bound after
an error. Expiry reports unresolved ownership and prevents result success.
This bound limits retries. It cannot guarantee a wall-clock deadline when the
scheduler or a kernel operation stalls.

## Run a fake that ignores TERM

Use the Ravel installation from
[chapter 3](03-a-complete-session.md#install-ravel-and-check-its-version).
Create an empty directory, make it the current directory, and create `bin`
inside it. Save these files at the labelled paths.

The fake installs its TERM ignore before it sends the result. It then stays
alive. Ravel has to reach KILL to end it. The command uses an empty private
HOME, an explicit policy and a cleared environment.

Complete example: a fake sends a result and then ignores TERM.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "job-escalation",
  select() {
    return {
      status: "selected",
      program: "fake-agent",
      args: [],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Observe the end of a job without a model service.",
    };
  },
});
```

**`bin/fake-agent`**

```sh
#!/bin/sh
set -eu
trap '' TERM
ravel end-session -- "finished"
while :; do
  sleep 0.05
done
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Send finished, then wait.'
```

**Standard output, without a final newline**

```text
finished
```

**Exit status**

```text
0
```

The example returns `finished` only after the shutdown. The child's death
from KILL does not invalidate its result. This transcript claims the result
bytes and exit status, not an exact duration or a fixed standard-error stream.
The automatic example check has no terminal and uses no real harness.

## Stop the group before the reap

A process can exit before its parent collects its status. Until that
collection, it remains a **zombie**: the system retains its identity and exit
status, although it executes no more code. To **reap** the child, the parent
collects that status and releases the retained process identity.

Ravel first observes exit without a reap. Its `waitid` call uses `WNOWAIT`
for that purpose. The unreaped child reserves its PID, which is also the
job's group ID. Ravel uses that reservation for the last group signals.

If Ravel reaped first, the system could reuse the ID for an unrelated group.
A later KILL to that number could then hit the wrong job.
The normal cleanup path therefore has this order:

| Step | Operation | What it establishes |
|---|---|---|
| 1 | Observe the child's exit with `waitid` and `WNOWAIT` | The child has exited, but still reserves its identity. A stop is not an exit |
| 2 | Send KILL to its group twice, with a 20-millisecond pause | Attempt to stop the other members while the identity remains reserved |
| 3 | Reap the direct child with `waitpid` | Confirm the child's status and release its identity |
| 4 | Recover the terminal if Ravel lent it | Check foreground recovery and restore the saved modes |
| 5 | Query the group without a signal, for up to one second | Confirm absence only when the system returns `ESRCH`, meaning no such process group |

The second KILL addresses a macOS case where a fork overlaps the first group
signal and leaves a new member. Both attempts precede the reap.
They do not change the direct child's status: it has already exited.
Ravel reaps its direct child. Other descendants' parents, or the system,
reap those descendants.

A successful signal operation does not prove that the group is gone.
The final query uses `kill` with signal 0, which sends no signal.
Success or `EPERM` means that absence remains unconfirmed. An unrelated group
that reuses the ID also prevents confirmation. Ravel does not signal that
number after the reap.

On macOS, a group with only zombies can reject KILL with `EPERM`, or
`Operation not permitted`. Ravel can retain this stop report without a session failure, but only after confirmed reap, successful terminal recovery and
an `ESRCH` absence check. The report alone never confirms cleanup.
Ravel writes retained reports as `ravel:` lines on standard error, or in
`operationReports` with `--json`.

If the group still answers after the confirmation wait, Ravel reports it and
returns no result to the caller. A query error also prevents success.
On an observation or reap error, Ravel attempts bounded cleanup and reports
what it cannot confirm. Once it loses or releases the child identity, it
stops all numeric signal attempts. It never invents a reap on timeout.

## Handle cancellation

Cancellation differs from a sent result. INT, TERM or HUP received by the
Ravel CLI suppresses the result, even if the managed process already sent it.
The runner checks for cancellation before it accepts an observed child exit.
The CLI also checks signals around supervision and final cleanup.

| Where cancellation occurs | Process action | Result |
|---|---|---|
| During policy selection | Stop and wait for the policy worker. Do not start the managed process | Return no result to the caller |
| During unconfined supervision | Forward the signal to the group and the direct child. If necessary, use KILL after the kill grace | Complete cleanup attempts, report cancellation, and return no result |
| During confined supervision | Attempt KILL immediately on the group and the direct child | Complete cleanup attempts, report cancellation, and return no result |

For unconfined supervision, cancellation skips the result's initial grace.
If the kill grace has already begun, cancellation does not restart it.
A noninteractive runner child can itself supervise another process. The
forwarded signal and kill grace let that supervisor attempt its own cleanup.
A confined job receives KILL immediately so that its shutdown can fit within
an outer supervisor's grace.

The two [cancellation sequences in chapter 5](05-running-sessions.md#cancel-during-selection-or-supervision)
show the selection and supervision cases, with complete fake-program examples.
[Chapter 8](08-follow-a-session-through-the-code.md#choose-the-final-outcome)
explains which signal and diagnostic Ravel reports when failures overlap.
A cleanup failure stays visible beside cancellation. It does not permit a
previously sent result to reach standard output.

## Separate noninteractive execution from confinement

The runner has a separate `run_noninteractive` interface. It starts a new
POSIX session, supplies end-of-file on standard input, and closes inherited
descriptors above standard error at exec. Both output streams go to a regular
file that its caller supplies. It has no completion channel, so only child
exit or cancellation ends the launch.

This interface isolates process control. It does not restrict filesystem
access. It is also different from an ordinary `start-session` invocation
that happens to have no terminal.

| Mode | POSIX session and input | Output and completion |
|---|---|---|
| Ordinary unconfined `start-session` | A new process group in the caller's POSIX session. Inherited input, with terminal access when available | Managed output goes to the caller's standard error. A result can request shutdown |
| Runner `run_noninteractive` | A new POSIX session, no inherited controlling terminal, EOF input, no inherited descriptors above 2 after exec | Both streams go to the supplied regular file. No completion channel |
| Confined `start-session` | A new POSIX session with the same input and descriptor isolation | Managed output goes to standard error. The private result channel remains available |

A new POSIX session removes the inherited controlling terminal relationship.
It does not itself restrict files or prevent a descendant from a move to another
group. The confinement backend supplies filesystem restrictions separately.
These distinctions explain why the runner hides process mechanics behind
separate launch interfaces, while the CLI owns policy and result decisions.

## Apply filesystem confinement

[Confinement](02-actors-and-words.md#trust-and-confinement) restricts the managed
process and its descendants. It does not restrict the trusted
[owner policy](02-actors-and-words.md#caller-and-owner) or its
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision).
The CLI checks the grants and probes the backend before policy selection.
If the backend is absent or cannot establish its restrictions, Ravel refuses.
There is no unconfined fallback.

| Resource | Access for a confined managed process |
|---|---|
| Canonical working directory and private run directory | Read and write |
| An explicit `--exit-dir` | Read and write to that channel parent directory |
| Selected executable and the real Ravel executable | Read and execute |
| Each regular file named with `--runtime-read` | Read only, even under a writable directory |
| System runtime resources | Reads from the backend's platform list, such as system programs, libraries and certificate files |
| Other file contents | No grant. File metadata can remain visible on macOS |

Ravel rejects canonical grants that overlap the policy directory or owner
settings. A runtime grant names one regular file, not a directory.
The environment starts empty. Ravel adds HOME, USER, LOGNAME, PATH, LANG and
LC_* values, private temporary-directory values, and this launch's result path.
HOME's presence does not grant access to all files under it.

On macOS, `/usr/bin/sandbox-exec` applies a Seatbelt profile. Descendants
inherit the restrictions. The profile allows the declared writable roots,
read-only runtime files and system reads. It also allows network access.
The system reads include `/opt/homebrew/Cellar`, so confinement is not a claim
that only explicitly named runtime files are readable.

On Linux, bubblewrap starts from an empty mount namespace: a filesystem view
with only the mounts the backend supplies. Ravel adds system read-only mounts,
writable roots and explicit read-only file mounts. It supplies private `/proc`
and `/dev` views and makes their mounts read-only.
Bubblewrap also uses user, PID, IPC and UTS namespaces. These separate user
identity, process IDs, interprocess communication and hostname state.
Ravel does not request a network namespace, so confinement does not disable
network access.

The runner calls `setsid` before the backend executes. The confined launch
needs no terminal handoff, and its new POSIX session avoids the caller's
controlling terminal. On Linux, the payload stays in the backend monitor's
group so that ordinary descendants remain within its cleanup scope.

[Chapter 5's confinement example](05-running-sessions.md#confine-the-managed-process)
shows the writable roots, a read-only grant and refusal cases through the CLI.
The runner tests add direct process checks:

| Test source under `crates/keyed-launch/tests/` | What it checks |
|---|---|
| `launch.rs` | Grace and KILL branches, ordinary descendants gone on return, and an unrelated process left untouched |
| `internal/wait_events.rs` | Cleanup order, a reported survivor, and cancellation across wait failures through a private process substitute |
| `noninteractive.rs` | EOF input, descriptor isolation, a new POSIX session, forwarded cancellation, and immediate confined cancellation |
| `confinement.rs` | Denied file reads and writes through a child and descendant, denied symlink access, and read-only runtime files |
| `job.rs` | Terminal foreground and mode behaviour through pseudo-terminals |

The selected process, isolation and Seatbelt tests passed on aarch64 macOS
on 18 October 2026. A macOS run does not execute the Linux bubblewrap backend.
The tests check named operations and process shapes. They do not establish
containment against every same-user attack or cleanup of a descendant that
leaves the job's group. They also do not qualify a native x86-64 Linux host.
