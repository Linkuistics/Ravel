# Running sessions

A [caller](02-actors-and-words.md#caller-and-owner) starts a
[session](02-actors-and-words.md#caller-and-owner), supplies its instructions
and interprets its result. The [owner policy](02-actors-and-words.md#caller-and-owner)
chooses the program. Ravel supervises that
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
and returns an [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
only after confirmed cleanup. This chapter teaches how to supply a request,
run nested or confined sessions, and distinguish the outcomes a caller can see.

Use the [checkout build described in chapter 3](03-a-complete-session.md#obtain-the-build-used-by-this-chapter).
These examples use real policy evaluation and small local programs, without
an agent account or model service. If you arrived here for an exit code, the
[status table](#interpret-the-exit-status) covers every normal CLI status and
the three handled cancellation signals, with an action for each.

## Supply the prompt and caller data

Pass exactly one of `--prompt TEXT` and `--prompt-file PATH`. Ravel gives the
text to the policy byte for byte, including an empty string or trailing
newlines. It adds no ending instruction and never reads a prompt from terminal
stdin. A prompt is UTF-8, contains no NUL and is at most 1 MiB; invalid input
is refused rather than truncated. Operating-system argument limits can be
smaller, including when the policy puts a file-supplied prompt into the
selected program's arguments.

Use a file when keeping the exact text matters. Shell command substitution
removes trailing newlines, so `--prompt "$(cat prompt.txt)"` can change the
text before Ravel receives it. `--prompt-file prompt.txt` reads the file
directly, relative to the caller's working directory. The unchanged-text
promise ends at the policy: the policy constructs the selected command's
arguments and decides how to pass the prompt onwards.

The [kind](02-actors-and-words.md#caller-and-owner) is a required, nonempty
caller token. Ravel has no catalogue of kinds and derives none from a file
name. A policy may route `review`, refuse it, or treat it like any other kind.
The caller and policy author agree on that vocabulary.

Other inputs carry data the policy can use:

| Input | What the policy receives |
|---|---|
| `--param profile=careful` | `request.params.profile` with the string `careful`. Each name is nonempty, contains no `=`, and occurs once. The value can be empty or contain further `=` characters. All names and values together are limited to 65,536 UTF-8 bytes. |
| `--task-file tasks/check.md` | An absolute UTF-8 `request.taskFile`, formed relative to the caller's working directory. Ravel does not read that file or require it to exist, and derives neither kind nor identity from it. |
| `--task-id example-17` | An opaque `request.taskId`: nonempty UTF-8, at most 1,024 bytes. Ravel does not look up a stored task. |
| `--context caller-context.json` | A measured, validated version-1 JSON document. With no policy loader it becomes the second argument to `select`; a loader can assemble a different delivered document. |

That last document is [policy context](02-actors-and-words.md#three-different-kinds-of-information),
data used in choosing the route. It is different from both the request's
working directory, `cwd`, and the
[session-result context](02-actors-and-words.md#three-different-kinds-of-information)
that later lets the managed process publish. Chapter 4 teaches
[policy context and loaders](04-writing-an-owner-policy.md#supply-policy-context)
and [bounds and environment grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).
The `--timeout-ms` bound applies to policy evaluation, not the managed
process's working time.

The selected program runs in the caller's working directory. Ravel resolves
an absolute program path directly, a relative path containing `/` against that
directory, and a bare program name through the caller's `PATH`. It passes the
argument strings directly, without shell splitting or expansion. An unconfined
managed process otherwise inherits the caller's environment, with old launch
controls removed and `RAVEL_EXIT_FILE` replaced for the new session. Put the
installed `ravel` on its `PATH`, or give it the executable's absolute path in
the ending instructions. The confined environment is described below.

Complete example: send a file prompt with its final newline, plus caller data.
The policy uses `profile` as an argument; it does not use the task fields or
policy context. The fake publishes the prompt itself so the returned bytes
show what reached it. Create the labelled files in a fresh directory, and make
`bin/echo-prompt` executable before running the command.

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
Finish by publishing the agreed result.
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
Finish by publishing the agreed result.
```

**Standard error includes**

```text
profile=careful
```

**Exit status**

```text
0
```

The final newline came from `prompt.txt`; Ravel added none. The fake's
`printf` appears on the caller's stderr because Ravel reserves stdout for the
result. No `tasks/check.md` file is needed in this example.

## Accept a result only after success

Keep the result bytes and the status of `start-session` together. Status 0
means a valid result was published, the direct child was reaped, its process
group was confirmed gone, required cleanup succeeded, and the final result
write and flush succeeded. **Reaping** means collecting the finished child's
status and releasing its operating-system record. The
[process group](03-a-complete-session.md#launch-the-managed-job) groups the
managed process and descendants that remain with it for cleanup.

An empty published value is a successful result with zero bytes. A managed
process exiting 0 without publishing is status 1, with no result. Conversely,
publication followed by a nonzero child exit can still give `start-session`
status 0. Ravel's status describes the session protocol, not whether the work
in the prompt was correct.

`end-session` takes one UTF-8 argument of at most 1 MiB. Quote it, and use
`--` before an option-looking value such as `--help`. Empty, multiline and
Unicode values keep their exact bytes. The operating system can impose a
smaller command-argument limit before the publisher starts. A first complete
publication wins; a repeated valid call succeeds without replacing it.

Complete example: compare publication followed by child exit 7, an empty
publication, and natural exit 0 without publication. Create these files in
a fresh directory and make `bin/outcome` executable.

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
      reason: "Compare publication with natural exit",
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind publish --config policy.ts --prompt '--help'
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind publish --config policy.ts --prompt ''
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

The publisher returns before the enclosing session finishes cleanup. Once
Ravel notices publication, a still-running job gets 2 seconds to finish, then
TERM, up to 5 more seconds, and KILL if needed. If the direct child exits,
Ravel stops remaining group members before reaping it, without granting those
survivors another full grace. Put publication after work that has to finish;
code after `end-session` may be terminated.

For an automated caller that needs trailing newlines intact, redirect stdout
to a file and read it only after checking status. Shell command substitution
is suitable only when the caller's result convention tolerates its removal
of trailing newlines. A failed final write can leave a prefix in the file;
bytes alone do not establish success. Ravel has no persistent result store
from which to recover an unsuccessful delivery.

Fragment: capture the exact result and status; this omits the policy, prompt
file and the caller's handling of each successful value or failure.

```sh
if ravel start-session --kind review --config policy.ts --prompt-file prompt.txt >result.txt; then
  status=0
else
  status=$?
fi
```

## Keep nested sessions separate

A managed process can become the caller of another `start-session`. The inner
session allocates its own result channel and replaces `RAVEL_EXIT_FILE` only
for its own managed process. The outer process keeps its enclosing value.
When the inner invocation returns, the outer process decides what to publish
for the outer session. Publishing the inner result does not finish the outer
session, and `end-session` accepts no session ID or target argument.

Complete example: the inner program publishes `inner`, and the outer program
uses that result in `inner|outer`. Create these files in a fresh directory
and make both files under `bin/` executable. These values have no trailing
newlines, so command substitution preserves them.

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
result=$(ravel start-session --kind inner --config "$1" --prompt "Publish inner.")
[ "$RAVEL_EXIT_FILE" = "$enclosing" ]
ravel end-session -- "$result|outer"
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind outer --config policy.ts --prompt 'Run the inner session, then publish its result with |outer appended.'
```

**Standard output, without a final newline**

```text
inner|outer
```

**Exit status**

```text
0
```

Each layer supervises its own process group. Nesting does not turn several
groups into one cleanup boundary: in particular, an inner front that dies
without completing its cleanup can leave its separate managed group behind.
The [operational limits](#understand-the-operational-limits) apply at every
layer. The example checks result isolation, not interactive nested job control.

## Confine the managed process

[Confinement](02-actors-and-words.md#trust-and-confinement) restricts the
managed process's filesystem access and environment. Add `--confine` when
you want that restriction. Owner policy and its
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
remain outside it; policy is trusted executable configuration.

Ravel uses `/usr/bin/sandbox-exec` on macOS and the system bubblewrap
executable, `/usr/bin/bwrap` or `/bin/bwrap`, on Linux. It tests whether the
backend can establish a sandbox before evaluating policy. An unavailable or
unusable backend refuses the invocation; Ravel does not fall back to an
unconfined launch.

The canonical working directory and a private run directory are writable.
System runtime paths, the selected executable and the real Ravel front are
admitted for reads. `--runtime-read FILE`, repeated for each additional file,
grants existing regular files read-only and requires `--confine`. It grants
no directory tree; stage task data in the working directory when it needs
that broader access. Read-only runtime grants remain read-only even inside a
writable directory. Confinement permits network access; it is not an offline
mode.

`--exit-dir DIR` places the session's private result directory beneath an
existing writable directory. It works with or without confinement. With
confinement it also grants that parent directory writable access, so choose
its contents accordingly. Without the option, an unconfined session uses the
system temporary directory; a confined session puts the channel beneath its
private run directory.

Keep the policy directory and owner settings outside every filesystem grant.
Ravel canonicalises paths and refuses overlaps, including overlaps through
symlinks or system runtime grants, before evaluating policy. For example,
running with `--confine --config ./policy.ts` from the directory containing
that policy overlaps the writable working directory. Put policy in a
separate owner directory, and invoke Ravel from the project directory.
`confinement_overlap` reports the conflicting paths; `confinement_unusable`
reports an unusable resource or backend. Both use status 2. An invalid
`--exit-dir` can similarly report `exit_dir_unusable` before selection;
later allocation or cleanup failures have their own diagnostics.

Complete example: select the `ravel` publisher itself as a minimal confined
program. This checks confinement and result delivery without another agent.
Create the labelled policy, then run the command from its parent directory.
The command creates and enters a separate project directory so that the
writable directory does not contain policy.

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
      model: "publisher",
      effort: "none",
      reason: "Check confined publication",
    };
  },
});
```

**Command**

```console
$ mkdir -p project && cd project && ravel start-session --kind check --config ../owner/policy.ts --confine --prompt 'Publish confined.'
```

**Standard output, without a final newline**

```text
confined
```

**Exit status**

```text
0
```

The confined environment retains `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG`
and `LC_*`. Ravel points `TMPDIR`, `TMP` and `TEMP` into the private run
directory and supplies the new `RAVEL_EXIT_FILE`. Retaining `HOME` does not
grant access to its directory. Other caller variables are absent; in
particular, `--policy-env` grants a variable to the policy worker, not to
this managed process.

A confined launch starts a new POSIX session and is noninteractive: it does
not borrow the caller's controlling terminal. An unconfined launch instead
uses a new process group in the caller's POSIX session and can borrow the
terminal when Ravel owns its foreground. This difference matters for an
interactive program; chapter 9, “Jobs, signals and the terminal”, develops
the mechanism. Confinement is a separate choice from an agent program's own
permission flags.

## Read notices without confusing them with results

For `start-session`, stdout contains only the exact result bytes, without an
added newline. The managed process's stdout goes to the caller's stderr,
alongside its stderr, Ravel's diagnostics and routing notices. A human-readable
starting notice identifies the selected provider, model, effort and kind;
those labels are owner text, not integrations with model providers.

`--json` changes Ravel's routing notice and failure document into JSON on
stderr. It does not turn the result into JSON or convert all other stderr
output. The managed program can still print arbitrary text there. Preserve
stderr as a diagnostic stream; do not parse the entire stream as one JSON
document or treat arbitrary program output as an authenticated Ravel notice.

The version-1 routing notice describes `selection`, `policy`, `worker`,
`selectionMs`, delivered `context`, effective `bounds`, `policyEnvironment`,
`resolution`, `promptSource` and captured policy `diagnostics`. The field
named `context` is policy context; it is not the result channel. Grant
diagnostics show environment names and whether they were set, never their
values. Policy code can still print values itself, so retain its diagnostics
with the care those values require.

Ravel emits this JSON notice after the supervision attempt, before final
settlement. When the runner returned an ending, `operationReports` contains
its retained operational reports, including an empty array when there are
none. If no ending was returned, that field can be absent. Reports such as
a failed stop attempt remain visible even when later checks establish
successful cleanup. Their presence alone does not decide success, and neither
does the routing notice itself. Check the final status.

A JSON failure has `schemaVersion: 1` and an `error` object with `code`,
`stage`, `message`, `remedy` and `exit`. It adds relevant fields such as
`input`, `source`, `location`, `policyCode`, `bound` and `signal`, and captured
policy `diagnostics` when available. Read the code and stage together:
`policy_refused` at selection is different from `worker_identity_mismatch`
before evaluation or `session_result_missing` after launch. The text format
uses “refused” in its heading for failures too; the code and status distinguish
a policy refusal from an operational failure.

An original failure remains primary when later cleanup also fails; Ravel
reports the cleanup problems alongside it. Cancellation takes precedence and
suppresses the result while retaining original and cleanup diagnostics. If
the operation would otherwise succeed, failed required cleanup prevents
result delivery. Keeping the complete stderr stream therefore matters even
when a caller has already classified the exit status.

## Interpret the exit status

This table covers Ravel's normal CLI outcomes, enumerated from the executable's
status constants, launch and settlement branches, and signal handling. A caller
checks the status of `start-session`, not the managed process's status or the
earlier status of `end-session`. For a session result, accept stdout only with
`start-session` status 0. Help and version requests are separate informational
invocations and do not publish a result.

| Status | Meaning | What the caller does |
|---|---|---|
| **0** | `start-session` delivered the exact published result after confirmed reap, group absence and required cleanup. An explicitly empty result is valid; the child's later nonzero exit does not invalidate it. `end-session` status 0 means publication succeeded or a first valid result was already present. Help/version also use 0. | Interpret a successful session's bytes using the caller's own convention. Wait for `start-session`; publisher success alone does not confirm cleanup. |
| **1** | `start-session`: `session_result_missing`, even if the child exited 0. `end-session`: publication failed, for example because `RAVEL_EXIT_FILE` was missing, empty or stale, the channel was invalid, or the value exceeded its bound. | Check that the managed program calls the installed publisher with one result before exiting, inside its live enclosing session. Retain any cleanup diagnostics too. Do not treat absence as an empty publication. |
| **2** | Malformed command-line input or settings, invalid prompt or data flags, or unusable/overlapping confinement inputs. Missing a subcommand displays help with status 2. An `end-session` argument-count error also uses 2. | Correct the named input, setting or confinement layout. Rerun only after the diagnostic's remedy applies. |
| **3** | Policy/admission refusal: owner policy declined, or Ravel could not accept its entry, behaviour, shape, returned data or policy context. No alternate route is tried. | Read `code`, `stage`, location and remedy. For `policy_refused`, inspect the owner's `policyCode` and explanation. Change the request or policy deliberately; do not silently substitute a program. |
| **5** | Worker failure, a supervision failure after launch was reported, invalid result data, required cleanup/recovery failure, a surviving group, or unavailable/failed result output. A missing, stale or mismatched front/worker pair also uses 5. | Retain the full diagnostics. Repair the matched installation or named process/filesystem problem. Check reported unresolved processes or temporary directories before retrying; discard any partial result bytes. |
| **124** | The policy evaluation deadline expired, covering worker startup, policy import, loading policy context and selection. No managed process was launched. | Inspect the policy's work. Adjust `--timeout-ms` or the owner's `timeoutMs` only if that longer selection is intended. This is not a managed-process time limit. |
| **126** | The selected program could not be executed, or launch failed before the runner reported it started for a reason other than ENOENT (missing file). | Check execute permissions, format, interpreter, platform and execution environment. A pre-start launch failure can also need terminal recovery; retain those diagnostics. Ravel runs no substitute. |
| **127** | Program resolution found no executable, or a pre-start launch failed with ENOENT. An executable script can exist while its interpreter is missing. | Check the caller's `PATH`, the selected path and its interpreter. Install or correct the intended program. |
| **129** | Cancellation by SIGHUP (signal 1), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled, retain cleanup reports and retry only if the caller still wants the work. |
| **130** | Cancellation by SIGINT (signal 2), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled; distinguish a signal sent to Ravel from a terminal signal sent only to the managed foreground group, as explained below. |
| **143** | Cancellation by SIGTERM (signal 15), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled, retain cleanup reports and retry only if termination was unintended. |

The last three numbers are the shell convention, 128 plus the signal number.
Ravel reports cancellation, attempts cleanup and then re-raises the signal,
so a process API normally reports termination by that signal rather than a
normal numeric exit. If signal propagation itself fails, Ravel reports that
failure and exits with the corresponding number as a fallback. A caller using
a process API handles both forms.

A signal such as SIGKILL can terminate the front outside this normal
settlement. Its shell status does not certify cleanup or result delivery.
Likewise, a newly arriving signal after the final cancellation decision can
interrupt the result write. There is no atomic guarantee covering both signal
arrival and an arbitrary stdout write.

## Cancel during selection or supervision

Cancellation of selection stops the policy worker and prevents the managed
launch. Ravel attempts worker teardown and reports the cancellation with any
teardown problems. The worker has no session-result context, so it cannot
publish an enclosing result through the ordinary `end-session` interface.

Complete example: policy sends SIGINT directly to its parent, the
[native front](02-actors-and-words.md#policy-evaluation-and-supervision), then
waits indefinitely. This causes selection cancellation without a terminal or
managed process. Create the policy in a fresh directory.

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

Here `<scratch>` denotes the temporary directory used by the book's example
checker. A manual run shows your policy's actual absolute path. Worker teardown
details can vary, so the transcript claims only the stable diagnostic line.

During supervision, cancellation stops the managed group and attempts reap,
terminal recovery and temporary cleanup before reporting the signal. The
result stays suppressed even when it was already published. Cleanup failure
does not change cancellation into success; the failure is reported beside it.

Complete example: publish a candidate result, send SIGINT directly to the
front, then stay alive for the supervisor to stop the job. Create the files
in a fresh directory and make `bin/cancel-after-result` executable.

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
      reason: "Cancel after publication",
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
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.ts --prompt 'Demonstrate cancellation after publication.'
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

Sending a signal to the front is different from typing Ctrl-C. Once Ravel
lends the terminal, the terminal sends INT to the managed foreground group.
If only that group receives INT and exits without publishing, the front can
report `session_result_missing` with status 1 rather than cancellation 130.
An interactive program can also handle or ignore the keyboard signal itself.
Use the front's status and diagnostic instead of inferring an outcome from
the key pressed. Chapter 9 explains signal inheritance and terminal recovery.

Fragment: run an interactive route and press Ctrl-C at the terminal. This
omits the policy, interactive program and physical terminal interaction; the
automated examples do not test keyboard delivery or foreground recovery.

```sh
ravel start-session --kind interactive --config policy.ts --prompt-file prompt.txt
```

## Understand the operational limits

The cleanup promise has a defined boundary. Use these limits when deciding
what an unsuccessful invocation leaves for the caller to investigate:

| Condition | Consequence |
|---|---|
| Code runs as the same user and tampers with the result channel or installation. | Private paths and inherited session-result context isolate ordinary sessions; they do not authenticate against hostile same-user code. The matching worker identity pairs builds, not trusted signers. |
| The native front dies abruptly, including SIGKILL. | It cannot finish supervision or cleanup. A managed job and temporary debris can remain; there is no successful completion or persistent recovery record. |
| A managed descendant deliberately leaves its process group. | It is outside that group's cleanup guarantee. Group absence does not prove that such a detached process is gone. |
| Trusted owner policy starts a detached child. | Worker teardown does not guarantee cleanup of that child. Managed-process confinement does not contain policy code. |
| The terminal foreground is changed by another actor during recovery, or a recorded group identity is reused. | Recovery depends on the runner's foreground-authority and identity conditions. A failed handback is reported; do not treat a launch failure as proof that the terminal was restored. Chapter 9 explains the conditions. |
| The host differs from the measured platforms. | The native-entry checks were measured on aarch64 macOS and aarch64 GNU/Linux. x86_64 GNU/Linux was built and run under emulation only; native x86-64 qualification remains open. These checks do not establish every kernel, libc or compiler combination. |

Chapter 2 explains the [authority boundary](02-actors-and-words.md#trust-and-confinement).
The native x86-64 qualification is tracked in
[Ravel issue 1](https://github.com/Linkuistics/Ravel/issues/1).
The examples here establish their checked routing, byte delivery and status
behaviour; they do not establish real-agent behaviour, interactive terminal
operation or whether a new reader can use the book. Chapter 6, “Reference”,
provides the lookup entries for the inputs and outputs taught here.
