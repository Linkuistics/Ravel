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
ends the invocation with status 2 rather than being truncated. Operating-system argument limits can be
smaller, including when the policy puts a file-supplied prompt into the
selected program's arguments.

Use a file when keeping the exact text matters. Shell command substitution
removes trailing newlines, so `--prompt "$(cat prompt.txt)"` can change the
text before Ravel receives it. `--prompt-file prompt.txt` reads the file
directly, relative to the caller's working directory. Ravel keeps the text
unchanged only as far as the policy: the policy constructs the selected
command's arguments and decides how to pass the prompt onwards.

The [kind](02-actors-and-words.md#caller-and-owner) is a required, nonempty
caller token. Ravel has no catalogue of kinds and derives none from a file
name. A policy may route `review`, refuse it, or treat it like any other kind.
The caller and policy author agree on that vocabulary.

These inputs supply policy data; they do not ask Ravel to read a task or assign
it meaning:

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

The selected program runs in the caller's working directory. The spelling of
its program path determines where Ravel looks for it:

| Selected program | Resolution base |
|---|---|
| Absolute path | The named path itself |
| Relative path containing `/` | The caller's working directory |
| Bare program name | The caller's `PATH` |

Ravel passes the
argument strings directly, without shell splitting or expansion. An unconfined
managed process otherwise inherits the caller's environment, with
`RAVEL_RUN_ID`, `RAVEL_STATE_DIR`, `HARNESS_DISPATCH_EXIT_FILE`,
`HARNESS_DISPATCH_RUN_ID` and `HARNESS_DISPATCH_STATE_DIR` removed and
`RAVEL_EXIT_FILE` replaced for the new session. Put the installed `ravel` on
the managed process's `PATH`, or give the absolute path of `ravel` in the
prompt's ending instructions. A confined managed process gets a smaller
environment, described under
[Confine the managed process](#confine-the-managed-process).

Complete example: send a file prompt with its final newline, plus caller data.
The policy uses `profile` as an argument; it does not use the task fields or
policy context. The fake agent publishes the prompt itself, so the returned
bytes show what reached it. Create the labelled files in a fresh directory, and make
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

The final newline came from `prompt.txt`; Ravel added none. The fake agent's
`printf` output appears on the caller's stderr because Ravel reserves stdout for the
result. No `tasks/check.md` file is needed in this example.

## Accept a result only after success

Keep the result bytes and the status of `start-session` together. Status 0
means a valid result was published, the direct child was
[reaped](03-a-complete-session.md#publish-the-result-and-finish-the-job), the
managed process's [process group](03-a-complete-session.md#launch-the-managed-job)
was confirmed gone, required cleanup succeeded, and the final result write and
flush succeeded.

An empty published result is a successful result with zero bytes. A managed
process exiting 0 without publishing is status 1, with no result. Conversely,
publication followed by a nonzero child exit can still give `start-session`
status 0. Ravel's status reports whether a published result was delivered after
cleanup, not whether the work in the prompt was correct.

`end-session` takes one UTF-8 argument of at most 1 MiB. Quote it, and use
`--` before an option-looking value such as `--help`. Empty, multiline and
Unicode values keep their exact bytes. The operating system can impose a
smaller command-argument limit before `end-session` starts. A first complete
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

`end-session` returns before the enclosing session finishes cleanup. Once
Ravel notices publication, a still-running job gets 2 seconds to finish, then
TERM, up to 5 more seconds, and KILL if needed. If the direct child exits,
Ravel stops the remaining group members before reaping it, and those members
do not get a fresh 2-second wait. Put publication after work that has to finish;
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
session allocates its own
[result channel](02-actors-and-words.md#the-managed-process-and-its-result) and replaces `RAVEL_EXIT_FILE` only
for its own managed process. The outer process keeps its enclosing value.
When the inner invocation returns, the outer process decides what to publish
for the outer session. Publishing the inner result does not finish the outer
session, and `end-session` accepts no session ID or target argument.

Each process uses the session-result context it inherited to publish;
the outer process must explicitly turn the inner result into an outer result:

| Process | Session-result context | What its `end-session` call publishes |
|---|---|---|
| Outer managed process, also the inner caller | Keeps the outer session's `RAVEL_EXIT_FILE` | The outer session's result |
| Inner managed process | Receives a fresh inner `RAVEL_EXIT_FILE` | The inner session's result, returned to the outer process after inner cleanup |

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

Each layer's native front supervises and cleans up only its own managed
process's group. An inner front that dies without completing its cleanup can
therefore leave its managed process's group behind, and the outer front does
not stop that group.
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
unusable backend ends the invocation with `confinement_unusable` and status 2;
Ravel does not fall back to an unconfined launch.

The canonical working directory and a private run directory are writable.
System runtime paths, the selected executable and the real
[native front](02-actors-and-words.md#policy-evaluation-and-supervision)
executable are admitted for reads. `--runtime-read FILE`, repeated for each additional file,
grants existing regular files read-only and requires `--confine`. It grants
no directory tree; copy task data into the working directory when the
managed process needs a whole directory. Read-only runtime grants remain read-only even inside a
writable directory. Confinement permits network access; it is not an offline
mode.

`--exit-dir DIR` places the session's private result directory beneath an
existing writable directory. It works with or without confinement. With
confinement it also grants that parent directory writable access, so choose a
parent holding nothing the managed process should be able to change. Without the option, an unconfined session uses the
system temporary directory; a confined session puts the result channel
beneath its private run directory.

Keep the policy directory and owner settings outside every filesystem grant.
Ravel canonicalises paths and stops with an error on overlaps, including
overlaps through symlinks or system runtime grants, before evaluating policy. For example,
running with `--confine --config ./policy.ts` from the directory containing
that policy overlaps the writable working directory. Put policy in a
separate owner directory, and invoke Ravel from the project directory.
`confinement_overlap` reports the conflicting paths; `confinement_unusable`
reports an unusable resource or backend. Both use status 2. An invalid
`--exit-dir` can similarly report `exit_dir_unusable` before selection;
later allocation or cleanup failures have their own diagnostics.

Complete example: select `ravel end-session` itself as a minimal confined
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

A confined launch starts a new POSIX session and is noninteractive: Ravel does
not [lend](03-a-complete-session.md#launch-the-managed-job) it the caller's
controlling terminal. An unconfined launch instead
uses a new process group in the caller's POSIX session, and Ravel lends it the
terminal when Ravel's own process group is the terminal's foreground group. This difference matters for an
interactive program; chapter 9, “Jobs, signals and the terminal”, develops
the mechanism. Confinement is a separate choice from an agent program's own
permission flags.

Use this comparison to choose confinement with its environment and terminal
consequences together; neither mode confines the owner policy:

| Property | Unconfined managed process | Confined managed process |
|---|---|---|
| Filesystem access | No Ravel confinement | Writable working and private run directories; admitted runtime reads and explicit grants |
| Caller environment | Inherited, with the obsolete variables listed above removed | Only `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and `LC_*` retained |
| Temporary-path variables | Inherited | `TMPDIR`, `TMP` and `TEMP` point to the private run directory |
| Session-result context | Fresh `RAVEL_EXIT_FILE` | Fresh `RAVEL_EXIT_FILE` |
| Process grouping | New process group in the caller's POSIX session | New POSIX session |
| Caller terminal | Lent when Ravel's group owns the foreground | Not lent; noninteractive |
| Network | No Ravel confinement | Permitted by confinement |

## Read notices without confusing them with results

For `start-session`, stdout contains only the exact result bytes, without an
added newline. The managed process's stdout goes to the caller's stderr,
alongside its stderr, Ravel's diagnostics and the routing notice. In text
form, the routing notice is one line printed before launch, beginning
`ravel: starting provider`, that names the selected provider, model, effort
and kind. Those labels are owner text, not integrations with model providers.

`--json` changes Ravel's routing notice and failure document into JSON on
stderr. It does not turn the result into JSON or convert all other stderr
output. The managed program can still print arbitrary text there. Preserve
stderr as a diagnostic stream; do not parse the entire stream as one JSON
document. The managed program can print a line that looks like a Ravel notice,
so a line's shape does not prove that Ravel wrote it.

Reserve stdout for the successful session result, and keep stderr as a mixed
diagnostic stream even with `--json`:

| Output | Destination | Effect of `--json` |
|---|---|---|
| Exact opaque result after successful settlement | Caller stdout, without an added newline | None |
| Managed process stdout and stderr | Caller stderr | None; arbitrary program text remains |
| Ravel routing notice and failure document | Caller stderr | JSON documents replace their text form |

The version-1 routing notice describes `selection`, `policy`, `worker`,
`selectionMs`, delivered `context`, effective `bounds`, `policyEnvironment`,
`resolution`, `promptSource` and captured policy `diagnostics`. The field
named `context` is policy context; it is not the result channel. Grant
diagnostics show environment names and whether they were set, never their
values. Policy code can still print values itself, so retain its diagnostics
with the care those values require.

Ravel emits this JSON notice after the supervision attempt, before final
settlement. When supervision completed and the
[runner](02-actors-and-words.md#policy-evaluation-and-supervision) returned
its account of how the job ended, `operationReports` contains the runner's
retained operational reports, as an empty array when there are none. If
supervision failed before the runner returned that account, the field is
absent. Reports such as
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
a policy [refusal](02-actors-and-words.md#policy-evaluation-and-supervision)
from an operational failure.

An original failure keeps its code and status when later cleanup also fails;
Ravel reports the cleanup problems alongside it. Cancellation takes precedence
over both: it sets the status, suppresses the result and keeps the original
and cleanup diagnostics. If
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

Each row's subject is taught elsewhere: statuses 0 and 1 under
[Accept a result only after success](#accept-a-result-only-after-success);
status 2 under [Supply the prompt and caller data](#supply-the-prompt-and-caller-data)
and [Confine the managed process](#confine-the-managed-process); status 3 in
chapter 4's [Select a command or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse);
status 5 under [Read notices without confusing them with results](#read-notices-without-confusing-them-with-results);
status 124 in chapter 4's [bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names);
statuses 126 and 127 with program resolution under
[Supply the prompt and caller data](#supply-the-prompt-and-caller-data); and
statuses 129, 130 and 143 under
[Cancel during selection or supervision](#cancel-during-selection-or-supervision).

| Status | Meaning | What the caller does |
|---|---|---|
| **0** | `start-session` delivered the exact published result after confirmed reap, group absence and required cleanup. An explicitly empty result is valid; the child's later nonzero exit does not invalidate it. `end-session` status 0 means publication succeeded or a first valid result was already present. A help or version request also exits 0. | Interpret a successful session's bytes using the caller's own convention. Wait for `start-session`; `end-session` success alone does not confirm cleanup. |
| **1** | `start-session`: `session_result_missing`, even if the child exited 0. `end-session`: publication failed, for example because `RAVEL_EXIT_FILE` was missing, empty or stale, the result channel was invalid, or the value exceeded its bound. | Check that the managed program calls the installed `ravel end-session` with one result before exiting, inside its live enclosing session. Retain any cleanup diagnostics too. Do not treat absence as an empty publication. |
| **2** | Malformed command-line input or settings, invalid prompt or data flags, or unusable or overlapping confinement inputs. Missing a subcommand displays help with status 2. An `end-session` argument-count error also uses 2. | Correct the named input, setting or confinement layout. Rerun only after the diagnostic's remedy applies. |
| **3** | Refusal by the policy or of its admission: owner policy declined, or Ravel could not accept its entry, behaviour, shape, returned data or policy context. No alternate route is tried. | Read `code`, `stage`, location and remedy. For `policy_refused`, inspect the owner's `policyCode` and explanation. Change the request or policy deliberately; do not silently substitute a program. |
| **5** | Worker failure, a supervision failure after launch was reported, invalid result data, failure of required cleanup or terminal recovery, a surviving group, or result output that was unavailable or failed. A missing or stale policy worker, or one that does not match the native front, also uses 5. | Retain the full diagnostics. Repair the matched installation, or the process or filesystem problem the diagnostic names. Check reported unresolved processes or temporary directories before retrying; discard any partial result bytes. |
| **124** | The policy evaluation deadline expired, covering worker startup, policy import, loading policy context and selection. No managed process was launched. | Inspect the policy's work. Adjust `--timeout-ms` or the owner's `timeoutMs` only if that longer selection is intended. This is not a managed-process time limit. |
| **126** | The selected program could not be executed, or launch failed, for a reason other than ENOENT (no such file), before the runner reported that the program started. | Check execute permissions, format, interpreter, platform and execution environment. A pre-start launch failure can also need terminal recovery; retain those diagnostics. Ravel runs no substitute. |
| **127** | Program resolution found no executable, or a pre-start launch failed with ENOENT. An executable script can exist while its interpreter is missing. | Check the caller's `PATH`, the selected path and its interpreter. Install or correct the intended program. |
| **129** | Cancellation by SIGHUP (signal 1), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled, retain cleanup reports and retry only if the caller still wants the work. |
| **130** | Cancellation by SIGINT (signal 2), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled; distinguish a signal sent to Ravel from a terminal signal sent only to the managed process's group, as explained under [Cancel during selection or supervision](#cancel-during-selection-or-supervision). |
| **143** | Cancellation by SIGTERM (signal 15), during selection or supervision. No result is accepted, even if one was published. | Treat the invocation as cancelled, retain cleanup reports and retry only if termination was unintended. |

The last three numbers are the shell convention, 128 plus the signal number.
Ravel reports cancellation, attempts cleanup and then re-raises the signal,
so a process API reports termination by that signal rather than a numeric
exit. If signal propagation itself fails, Ravel reports that
failure and exits with the corresponding number as a fallback. A caller using
a process API handles both forms.

A signal such as SIGKILL can terminate the front outside this normal
settlement. Its shell status does not certify cleanup or result delivery.
Likewise, a newly arriving signal after the final cancellation decision can
interrupt the result write. There is no atomic guarantee covering both signal
arrival and an arbitrary stdout write.

The following view expands [chapter 3's outcome state machine](03-a-complete-session.md#recognise-the-other-outcomes)
at settlement: cancellation wins over an original failure, which keeps its
status if later cleanup also fails. Only a valid result with confirmed cleanup
reaches the final write. This is the precedence of outcomes, not the order of
function calls; applicable cleanup is attempted before reporting any outcome.

This UML state machine starts at the solid dot, entering the settlement detail.
Blue rounded boxes are conceptual phases of the native front's work, grey
boxes are outcomes, and bracketed arrow labels give transition conditions.
The green route is successful result delivery. The view covers `start-session`;
the table also covers `end-session`, help and version invocations.

![Settlement checks cancellation first, then retains an original failure and its status, then requires confirmed cleanup before writing the result. Cancellation gives 129, 130 or 143; original failures retain 1, 2, 3, 5, 124, 126 or 127. Cleanup or final-write failure gives 5; only a successful write and flush gives 0.](figures/settled-exit-status.svg)

[Open the settlement view at full size](figures/settled-exit-status.svg).

## Cancel during selection or supervision

Cancellation of selection stops the policy worker and prevents the managed
launch. Ravel attempts worker teardown and reports the cancellation with any
teardown problems. The worker has no session-result context, so it cannot
publish an enclosing result through the ordinary `end-session` interface.

The selection sequence shows why cancellation can require worker cleanup even
though no managed process was launched. It follows the example below, where
the worker sends INT directly to the native front.

In these UML sequences, order runs down the dashed participant lifelines;
it is not an elapsed-time scale. Solid arrows name actions, bent arrows name
work within a participant, and the dashed return arrow reports the outcome to
the caller. Blue boxes are Ravel processes, grey boxes are external participants,
and labelled bars group steps. No lifetime ends with an X here: teardown is
attempted, and a failure cannot certify reap or removal.

![The caller starts selection, the policy worker sends SIGINT to the native front, and the front attempts worker teardown and private-directory cleanup. It retains teardown problems, launches no managed process, and reports cancellation with no result.](figures/selection-cancellation.svg)

[Open the selection cancellation sequence at full size](figures/selection-cancellation.svg).

Complete example: the policy sends SIGINT directly to its parent, the native
front, then waits indefinitely. This causes selection cancellation without a terminal or
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

During supervision, cancellation stops the managed process's group and attempts reap,
terminal recovery and temporary cleanup before reporting the signal. The
result stays suppressed even when it was already published. Cleanup failure
does not change cancellation into success; the failure is reported beside it.

The supervision sequence shows that publication does not protect a value from
cancellation: the green arrow reaches the gold temporary result channel, but
no result reaches the caller. The runner executes inside the native front;
the managed process's group includes the descendants that stay in that group.
The pictured cleanup steps are attempts, with failures retained as diagnostics.

![The managed process publishes candidate via end-session, then sends SIGINT to the native front. The front attempts group shutdown, reap, terminal recovery and temporary cleanup, suppresses the published result, and reports cancellation even if cleanup fails.](figures/supervision-cancellation.svg)

[Open the supervision cancellation sequence at full size](figures/supervision-cancellation.svg).

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
lends the terminal, the managed process's group is the terminal's foreground
group, and Ctrl-C makes the terminal send INT to that group, not to the front.
If only that group receives INT and exits without publishing, the front
reports `session_result_missing` with status 1 rather than cancellation 130.
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

Ravel's cleanup guarantee does not hold under the conditions in this table.
Use them when deciding what an unsuccessful invocation leaves for the caller
to investigate:

| Condition | Consequence |
|---|---|
| Code runs as the same user and tampers with the result channel or installation. | Private paths and inherited session-result context isolate ordinary sessions; they do not authenticate against hostile same-user code. The worker identity check confirms that the policy worker matches the native front's build; it does not verify a trusted signer. |
| The native front dies abruptly, including SIGKILL. | It cannot finish supervision or cleanup. The managed process's group and temporary directories can remain; there is no successful completion or persistent recovery record. |
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
