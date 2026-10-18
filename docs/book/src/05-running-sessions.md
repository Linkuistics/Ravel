# Run sessions

A [caller](02-actors-and-words.md#caller-and-owner) starts a
[session](02-actors-and-words.md#caller-and-owner), gives its instructions
and interprets its result. The [owner policy](02-actors-and-words.md#caller-and-owner)
chooses the program. Ravel supervises that
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
and returns an [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
only after it confirms the cleanup. This chapter shows how to give a request
and how to run nested or confined sessions. It also shows how to identify
each outcome that a caller can see.

Follow chapter 3 to [install Ravel and check its version](03-a-complete-session.md#install-ravel-and-check-its-version).
These examples use real policy evaluation and small local programs. They do
not need an agent account or a model service. If you came here for an exit
code, go to the [status table](#interpret-the-exit-status). It gives each
normal status of the CLI and the three cancellation signals that Ravel
handles, with an action for each.

## Supply the prompt and caller data

Pass exactly one of `--prompt TEXT` and `--prompt-file PATH`. Ravel gives the
text to the policy unchanged, also when it is an empty string or ends with
newlines. Ravel adds no instructions to end the session. It never
reads a prompt from terminal stdin. A prompt is UTF-8, contains no NUL and
has a maximum size of 1 MiB. Ravel does not truncate input that is not
valid, but stops the invocation with status 2.

The argument limits of the operating system can be smaller than 1 MiB. These
limits also apply when the policy puts a prompt from a file into the
arguments of the selected program.

Use a file when the exact text is important. Shell command substitution
removes trailing newlines. Thus, `--prompt "$(cat prompt.txt)"` can change
the text before Ravel receives it. `--prompt-file prompt.txt` reads the file
directly, relative to the caller's working directory. Ravel changes no text
before the policy receives it. Then the policy makes the arguments of the
selected command and decides how to give the prompt to that program.

The [kind](02-actors-and-words.md#caller-and-owner) is a required caller
token that is not empty. Ravel has no catalogue of kinds and does not get a
kind from a file name. A policy can route `review`, refuse it or treat it as
any other kind. The caller and the policy author agree on these tokens.

These inputs give data to the policy. They do not tell Ravel to read a task
or to give the task a meaning:

| Input | What the policy receives |
|---|---|
| `--param profile=careful` | `request.params.profile` with the string `careful`. Each name is not empty, contains no `=` and occurs one time. The value can be empty or contain more `=` characters. The limit for all names and values together is 65,536 UTF-8 bytes. |
| `--task-file tasks/check.md` | An absolute UTF-8 `request.taskFile`. Ravel makes it from the given path and the caller's working directory. Ravel does not read that file, and the file does not have to exist. Ravel gets no kind or identity from it. |
| `--task-id example-17` | An opaque `request.taskId`. It is UTF-8, is not empty and has a maximum of 1,024 bytes. Ravel does not search for a stored task. |
| `--context caller-context.json` | A version-1 JSON document that Ravel measures and validates. If the policy has no loader, the document becomes the second argument to `select`. A loader can make a different document for the policy. |

That last document is [policy context](02-actors-and-words.md#three-different-kinds-of-information),
which is data that the policy uses to choose the route. It is different from
the working directory of the request, `cwd`. It is also different from the
[session-result context](02-actors-and-words.md#three-different-kinds-of-information),
which later lets the managed process send a result. Chapter 4 teaches
[policy context and loaders](04-writing-an-owner-policy.md#supply-policy-context)
and [bounds and environment grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).
The `--timeout-ms` bound applies to policy evaluation, not to the time that
the managed process works.

The selected program runs in the caller's working directory. The form of its
program path sets the base that Ravel uses to find the program:

| Selected program | Resolution base |
|---|---|
| Absolute path | The path itself |
| Relative path that contains `/` | The caller's working directory |
| Bare program name | The caller's `PATH` |

Ravel gives the argument strings directly to the program, without shell word
splitting or expansion. An unconfined managed process inherits the caller's
environment, with two changes. Ravel removes `RAVEL_RUN_ID`,
`RAVEL_STATE_DIR`, `HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID`
and `HARNESS_DISPATCH_STATE_DIR`, and it replaces `RAVEL_EXIT_FILE` for the
new session. Put the installed `ravel` on the `PATH` of the managed process.
As an alternative, write the absolute path of `ravel` in the prompt's
instructions to end the session. A confined managed process gets a smaller
environment, which [Confine the managed process](#confine-the-managed-process)
describes.

Complete example: pass a prompt from a file, with its final newline, and
caller data. The policy uses `profile` as an argument. It does not use the
task fields or the policy context. The fake agent sends the prompt itself as
the result, so the result shows what the agent received. Create the
labelled files in a new directory. Make `bin/echo-prompt` executable before
you run the command.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "prompt-demo",
  select(request) {
    return {
      status: "selected",
      program: "echo-prompt",
      args: [request.prompt, request.params.profile ?? "default"],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Show the caller's prompt unchanged",
    };
  },
});
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind review --config policy.ts --prompt-file prompt.txt --param profile=careful --task-file tasks/check.md --task-id example-17 --context caller-context.json
```

**Standard output**

```text
Review the public behaviour.
When you finish, send the agreed result.
```

**Standard error includes**

```text
profile=careful
```

**Exit status**

```text
0
```

The final newline came from `prompt.txt`. Ravel added no newline. The
`printf` output of the fake agent goes to the caller's stderr, because Ravel
uses stdout only for the result. This example does not need a `tasks/check.md`
file.

## Accept a result only after success

Keep the result bytes and the status of `start-session` together. Status 0
means that all of these conditions are true:

- A program in the session sent a valid result.
- Ravel [reaped](03-a-complete-session.md#send-the-result-and-finish-the-job)
  the direct child.
- Ravel confirmed that the
  [process group](03-a-complete-session.md#launch-the-managed-job) of the
  managed process is gone.
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
starts.

Ravel keeps the first complete result that it receives through the
[result channel](02-actors-and-words.md#the-managed-process-and-its-result).
When that first result arrives, Ravel starts to stop the
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

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "outcome-demo",
  select(request) {
    return {
      status: "selected",
      program: "outcome",
      args: [request.kind, request.prompt],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Compare a sent result with natural exit",
    };
  },
});
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind send --config policy.ts --prompt '--help'
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind send --config policy.ts --prompt ''
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind natural --config policy.ts --prompt ''
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

For an automated caller that needs trailing newlines intact, redirect stdout
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
if ravel start-session --kind review --config policy.ts --prompt-file prompt.txt >result.txt; then
  status=0
else
  status=$?
fi
```

## Keep nested sessions separate

A managed process can be the caller of another `start-session`. The inner
session allocates its own
[result channel](02-actors-and-words.md#the-managed-process-and-its-result).
It replaces `RAVEL_EXIT_FILE` only for its own managed process. The outer
process keeps the value of its enclosing session. When the inner invocation returns, the
outer process decides what to send for the outer session. When the inner
process sends its result, the outer session does not finish, and
`end-session` accepts no session ID or target argument.

Each process uses the session-result context that it inherited to send a
result. Ravel does not change the inner result into an outer result. The
outer process does this itself:

| Process | Session-result context | What its `end-session` call sends |
|---|---|---|
| Outer managed process, also the inner caller | Keeps the `RAVEL_EXIT_FILE` of the outer session | The result of the outer session |
| Inner managed process | Gets a new inner `RAVEL_EXIT_FILE` | The result of the inner session. Ravel returns it to the outer process after the inner cleanup. |

Complete example: the inner program sends `inner`, and the outer program
uses that result in `inner|outer`. Create these files in a new directory and
make both files under `bin/` executable. These values have no trailing
newlines, so command substitution keeps them unchanged.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "nested-demo",
  select(request) {
    return {
      status: "selected",
      program: request.kind === "outer" ? "outer" : "inner",
      args: [`${request.cwd}/policy.ts`],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Give each nested session its own result",
    };
  },
});
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind outer --config policy.ts --prompt 'Run the inner session. Then add |outer to its result and send it.'
```

**Standard output, without a final newline**

```text
inner|outer
```

**Exit status**

```text
0
```

Each layer's [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision)
supervises only the group of its own managed process, and it does the
cleanup only for that group. Thus, if an inner Ravel CLI dies before it
completes its cleanup, the group of its managed process can remain. The
outer Ravel CLI does not stop that group. The
[operational limits](#understand-the-operational-limits) apply at each layer.
The example checks that the results stay separate. It does not check
interactive job control for nested sessions.

## Confine the managed process

[Confinement](02-actors-and-words.md#trust-and-confinement) restricts the
filesystem access and the environment of the managed process. Add
`--confine` when you want that restriction. The owner policy and its
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
stay outside the confinement. Policy is trusted executable configuration.

On macOS, Ravel uses `/usr/bin/sandbox-exec`. On Linux, it uses the system
bubblewrap executable, `/usr/bin/bwrap` or `/bin/bwrap`. Before Ravel
evaluates the policy, it tests whether the backend can make a sandbox. If
the backend is not available or not usable, the invocation stops with
`confinement_unusable` and status 2. Ravel does not then launch the process
without confinement.

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
run `--confine --config ./policy.ts` from the directory that contains that
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

**`owner/policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "confined-demo",
  select() {
    return {
      status: "selected",
      program: "ravel",
      args: ["end-session", "--", "confined"],
      provider: "local",
      model: "sender",
      effort: "none",
      reason: "Check a confined sent result",
    };
  },
});
```

**Command**

```console
$ mkdir -p project && cd project && ravel start-session --kind check --config ../owner/policy.ts --confine --prompt 'Send confined.'
```

**Standard output, without a final newline**

```text
confined
```

**Exit status**

```text
0
```

The confined environment keeps `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and
`LC_*`. Ravel sets `TMPDIR`, `TMP` and `TEMP` to the private run directory and
gives the managed process a new `RAVEL_EXIT_FILE`. The `HOME` variable does
not give access to the directory that it names. The other caller variables are
not in the environment. This also applies to `--policy-env`. It grants a
variable to the policy worker, not to this managed process.

A confined launch starts a new POSIX session and is not interactive. Ravel
does not [lend](03-a-complete-session.md#launch-the-managed-job) it the
caller's controlling terminal. An unconfined launch uses a new process group
in the caller's POSIX session. Ravel lends it the terminal if Ravel's own
process group is the foreground group of the terminal. This difference is
important for an interactive program. [Chapter 9, “Jobs, signals and the
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

For `start-session`, stdout contains only the result. Ravel writes the result
unchanged, with no added newline, and does not interpret it. The stdout of the
managed process goes to the caller's stderr, together with its stderr, Ravel's
diagnostics and the routing notice. In text form, the routing notice is one
line that Ravel writes before the launch. The line starts with
`ravel: starting provider` and gives the selected provider, model, effort and
kind.
These labels are owner text, not integrations with model providers.

`--json` changes the routing notice and the failure document of Ravel into
JSON on stderr. It does not change the result into JSON, and it does not
change other stderr output. The managed process can still write any text
there. Keep stderr as a diagnostic stream, and do not parse the full stream
as one JSON document. The managed process can write a line that looks like a
Ravel notice. Thus, the form of a line does not prove that Ravel wrote it.

Read stdout only as the result of a successful session. Keep stderr as a
mixed diagnostic stream, even with `--json`:

| Output | Destination | Effect of `--json` |
|---|---|---|
| Unchanged opaque result after a successful session | Caller stdout, with no added newline | None |
| Stdout and stderr of the managed process | Caller stderr | None. Any program text stays. |
| Routing notice and failure document of Ravel | Caller stderr | JSON documents replace their text form |

The version-1 routing notice gives `selection`, `policy`, `worker`,
`selectionMs`, the delivered `context`, the effective `bounds`,
`policyEnvironment`, `resolution`, `promptSource` and the captured policy
`diagnostics`. The field `context` is policy context, not the result
channel. Grant diagnostics show the name of each environment variable and
whether it is set. They never show its value. Policy code can still write
values itself. Thus, keep its diagnostics with the care that those values
need.

Ravel writes this JSON notice after the supervision attempt and before it
decides the final outcome. If the
[runner](02-actors-and-words.md#policy-evaluation-and-supervision) completed
supervision and returned its record of how the job ended,
`operationReports` holds the operational reports that the runner kept. The
array is empty if there are no reports. If supervision failed before the
runner returned that record, the notice has no such field.

Some reports, such as a failed stop attempt, stay visible even when later
checks confirm successful cleanup. A report alone does not decide success,
and the routing notice also does not. Check the final status.

A JSON failure has `schemaVersion: 1` and an `error` object with `code`,
`stage`, `message`, `remedy` and `exit`. It adds the fields that apply, such
as `input`, `source`, `location`, `policyCode`, `bound` and `signal`, and the
captured policy `diagnostics` when they are available. Read the code and the
stage together. These examples distinguish a policy decision from failures
before and after launch:

| Code | Stage | Meaning |
|---|---|---|
| `policy_refused` | `selection` | The owner policy declined the request. |
| `worker_identity_mismatch` | `worker` | The policy worker does not match the Ravel CLI. Ravel stops before policy evaluation. |
| `session_result_missing` | `exec` | The managed process ended without a sent result. |

The text form of each failure starts with `ravel: refused`, also when no
policy refused the request. The code and the status show the difference
between a policy
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision) and an
operational failure.

If later cleanup also fails, an original failure keeps its code and status.
Ravel reports the cleanup problems together with the original failure.
Cancellation has precedence over both. Ravel then sets the cancellation
status, keeps the original and cleanup diagnostics and returns no result to
the caller. If required cleanup fails, Ravel also returns no result to the
caller, even when everything else succeeds. Thus, keep the complete stderr
stream, even after the caller classifies the exit status.

## Interpret the exit status

This table gives the normal outcomes of the Ravel CLI. It comes from the
status constants of the Ravel CLI, its launch and final-outcome code and its
code for signals. A caller checks the status of `start-session`. It does not
use the status of the managed process or the earlier status of
`end-session`. For a session result, accept stdout only with status 0 from
`start-session`. A help or version request is a separate invocation for
information, and it sends no result.

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
| `0` | `start-session` wrote the sent result unchanged to stdout after Ravel confirmed the reap, the absence of the group and the required cleanup. An explicitly empty result is valid. A later nonzero exit of the child does not make it invalid. For `end-session`, status 0 means that it sent the result, or that a valid first result was already present. A help or version request also exits with 0. | Interpret the result of a successful session with the caller's own convention. Ravel does not interpret it. Wait for `start-session`. The success of `end-session` alone does not confirm cleanup. |
| `1` | `start-session`: `session_result_missing`, even if the child exited with status 0. `end-session`: it could not send the result. For example, `RAVEL_EXIT_FILE` was missing, empty or stale, the result channel was not valid, or the value was larger than its limit. | Check that the managed process calls the installed `ravel end-session` with one result before it exits. Check also that it makes the call inside its live enclosing session. Keep any cleanup diagnostics too. Do not treat a missing result as an empty result. |
| `2` | Malformed command-line input or settings, invalid prompt or data flags, or confinement inputs that are unusable or overlap. A missing subcommand shows the help with status 2. An argument-count error of `end-session` also uses 2. | Correct the named input, setting or confinement layout. Run the command again only after you apply the remedy of the diagnostic. |
| `3` | A refusal by the policy, or Ravel's refusal of the policy. The owner policy declined, or Ravel could not accept its entry, behaviour, shape, returned data or policy context. Ravel tries no other route. | Read `code`, `stage`, the location and the remedy. For `policy_refused`, examine the owner's `policyCode` and explanation. Change the request or the policy deliberately. Do not silently substitute a program. |
| `5` | A worker failed, or supervision failed after the runner reported the launch. The result data was not valid, or required cleanup or terminal recovery failed. A group survived, or the result output was unavailable or failed. A missing or stale policy worker, or one that does not match the Ravel CLI, also uses 5. | Keep the full diagnostics. Repair the matched installation, or the process or filesystem problem that the diagnostic names. Examine the unresolved processes or temporary directories that Ravel reports before you try again. Discard any partial result bytes. |
| `124` | The deadline for policy evaluation expired. The deadline covers the worker startup, the policy import, the load of policy context and the selection. Ravel launched no managed process. | Examine the work of the policy. Change `--timeout-ms` or the owner's `timeoutMs` only if you want that longer selection. This is not a time limit for the managed process. |
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
arbitrary write to stdout.

The following view expands one step of
[chapter 3's outcome state machine](03-a-complete-session.md#recognise-the-other-outcomes):
the step where Ravel decides the final outcome. Cancellation wins over an
original failure. An original failure keeps its status if later cleanup also
fails. Only a valid result with confirmed cleanup gets to the final write.
The view shows the precedence of outcomes, not the order of function calls.
Ravel attempts the applicable cleanup before it reports an outcome.

The view covers `start-session`. The table also covers `end-session` and the
help and version invocations.

![Ravel checks cancellation first and keeps any original failure with its status. Then it requires confirmed cleanup before it writes the result unchanged. Cancellation gives 129, 130 or 143 and returns no result to the caller. Original failures keep 1, 2, 3, 5, 124, 126 or 127. A cleanup or final-write failure gives 5. Only a successful write and flush gives 0.](figures/settled-exit-status.svg)

[Open the final-outcome view at full size](figures/settled-exit-status.svg).

## Cancel during selection or supervision

A cancellation during selection stops the policy worker and prevents the
managed launch. Ravel attempts the worker teardown and reports the
cancellation with any teardown problems. The worker has no session-result
context. Thus, it cannot send a result for an enclosing session through the
usual `end-session` interface.

The selection sequence shows why a cancellation can need worker cleanup,
although Ravel launched no managed process. It follows the example below,
where the worker sends INT directly to the Ravel CLI.

In these UML sequences, no lifetime ends with an X. Ravel attempts
cleanup, and a failed attempt does not confirm a reap or a removal.

![The caller starts selection. The policy worker sends INT to the Ravel CLI. The Ravel CLI attempts worker teardown and private-directory cleanup. It keeps the teardown problems, launches no managed process and reports cancellation. It returns no result to the caller.](figures/selection-cancellation.svg)

[Open the selection cancellation sequence at full size](figures/selection-cancellation.svg).

Complete example: the policy sends INT directly to its parent, the Ravel
CLI. Then it waits indefinitely. This example causes a cancellation during
selection without a terminal or a managed process. Create the policy in a
new directory.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "cancel-selection",
  async select() {
    process.kill(process.ppid, "SIGINT");
    return await new Promise<never>(() => {});
  },
});
```

**Command**

```console
$ ravel start-session --kind check --config policy.ts --prompt 'No program should launch.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (selection_cancelled, stage evaluation): selection with the policy <scratch>/work/policy.ts was cancelled by SIGINT: worker teardown was attempted and nothing was launched
```

**Exit status**

```text
130
```

Here `<scratch>` is the temporary directory that the example checker of the
book uses. A manual run shows the actual absolute path of your policy. The
details of worker teardown can change from run to run. Thus, the transcript
claims only the stable diagnostic line.

During supervision, a cancellation stops the group of the managed process.
Ravel attempts the reap, the terminal recovery and the temporary cleanup
before it reports the signal. Ravel returns no result to the caller, even when
it already received a result. A cleanup failure does not change the
cancellation into success. Ravel reports the failure together with the
cancellation.

The supervision sequence shows why a cancellation returns no result to the
caller, even after the managed process sent a result. The result reaches the
temporary result channel, but Ravel does not write it to stdout. The runner
runs inside the Ravel CLI. The group of the managed process includes the
descendants that stay in that group. The cleanup steps in the figure are
attempts, and Ravel keeps their failures as diagnostics.

![The managed process sends candidate through end-session. Then it sends INT to the Ravel CLI. The Ravel CLI attempts group shutdown, reap, terminal recovery and temporary cleanup. It returns no result to the caller and reports cancellation, even if cleanup fails.](figures/supervision-cancellation.svg)

[Open the supervision cancellation sequence at full size](figures/supervision-cancellation.svg).

Complete example: the program sends a candidate result and sends INT
directly to the Ravel CLI. Then it stays alive until the supervisor stops the
job. Create the files in a new directory and make `bin/cancel-after-result`
executable.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "cancel-session",
  select() {
    return {
      status: "selected",
      program: "cancel-after-result",
      args: [],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Cancel after a sent result",
    };
  },
});
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.ts --prompt 'Show cancellation after a sent result.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (session_cancelled, stage exec): session selected by <scratch>/work/policy.ts was cancelled by SIGINT
```

**Exit status**

```text
130
```

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
ravel start-session --kind interactive --config policy.ts --prompt-file prompt.txt
```

## Understand the operational limits

Ravel's cleanup guarantee does not apply under the conditions in this table.
Use the table to decide what an unsuccessful invocation can leave for the
caller to examine:

| Condition | Consequence |
|---|---|
| Code that runs as the same user makes unauthorised changes to the result channel or the installation. | Private paths and the inherited session-result context keep ordinary sessions apart. They are not a defence against hostile code of the same user. The worker identity check confirms that the policy worker matches the build of the Ravel CLI. It does not check a trusted signature. |
| The Ravel CLI dies abruptly, for example from KILL. | It cannot complete supervision or cleanup. The group of the managed process and the temporary directories can remain. There is no successful completion and no persistent recovery record. |
| A managed descendant deliberately leaves its process group. | It is outside the cleanup guarantee of that group. The absence of the group does not prove that the detached process is gone. |
| Trusted owner policy starts a detached child. | Worker teardown does not guarantee the cleanup of that child. The confinement of the managed process does not contain policy code. |
| Another process changes the foreground group of the terminal during recovery, or the system gives a recorded group ID to an unrelated group. | Recovery assumes two conditions between the runner's check and its handback. No other process or thread changes the foreground group. No unrelated group gets the recorded group ID. Ravel reports a failed handback. Do not treat a launch failure as proof that Ravel restored the terminal. [Chapter 9](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal) explains the conditions. |
| The host is different from the measured platforms. | The project measured how Ravel handles inherited signal settings, cancellation, cleanup and terminal recovery. It measured them on aarch64 macOS and aarch64 GNU/Linux. The project built Ravel for x86_64 GNU/Linux and ran it only under emulation. Native x86-64 qualification is not complete. These checks do not cover all combinations of kernel, libc and compiler. |

Chapter 2 explains these [trust limits](02-actors-and-words.md#trust-and-confinement).
[Ravel issue 1](https://github.com/Linkuistics/Ravel/issues/1) tracks the
native x86-64 qualification. The examples in this chapter check the choice
of command, the delivery of the result and the status. They do not show the
behaviour of real agents, interactive terminal operation or whether a new
reader can use the book. [Chapter 6, “Reference”](06-reference.md), gives the
reference entries for the inputs and outputs that this chapter teaches.
