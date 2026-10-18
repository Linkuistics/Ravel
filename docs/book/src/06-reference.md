# Reference

Use this chapter to find the command line, exit statuses, streams,
environment rules, configuration and installed paths. A [caller](02-actors-and-words.md#caller-and-owner) starts
a [session](02-actors-and-words.md#caller-and-owner). The
[owner policy](02-actors-and-words.md#caller-and-owner) selects its
[managed process](02-actors-and-words.md#the-managed-process-and-its-result).
Only the caller gives meaning to the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result).
Each entry links to the chapter that teaches its use.

The entries describe the version that this book names, installed with
[Homebrew](03-a-complete-session.md#install-ravel-and-check-its-version).

## Command line

### `ravel`, help and version

`ravel` takes one of two verbs: `start-session` or `end-session`. It has no
`help` subcommand. The table gives the help and version requests.

| Request | Output | Status |
|---|---|---|
| `ravel` without a verb | Help on stderr. | `2` |
| `ravel --help`, `ravel start-session --help`, `ravel end-session --help` | Help on stdout. `-h` is the short option on all three command lines. | `0` |
| `ravel --version`, `ravel -V` | The package version of the [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) on stdout. The version option belongs only to the top-level command. | `0` |

Help and version text is informational output, never an opaque result.
See [the first complete session](03-a-complete-session.md#run-the-first-session)
for the two verbs in use.

After `--`, Ravel reads no more options. For `end-session`, put `--` before a
result that looks like an option. Inside a live session,
`ravel end-session -- '--help'` sends the literal value `--help`. See
[result arguments](#end-session-result).

### `start-session`

`ravel start-session --kind TEXT` also needs exactly one of `--prompt TEXT`
and `--prompt-file PATH`. It reads the owner settings, checks and evaluates
the owner policy, then supervises the selected program. A
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision) launches nothing, and Ravel
tries no alternative route. A successful invocation returns the exact sent
result, but only after the reap, the confirmed absence of the process group
and the required cleanup. See [run a complete session](03-a-complete-session.md#run-the-first-session)
and [interpret the outcome](05-running-sessions.md#interpret-the-exit-status).

All fifteen options below belong after `start-session`. Only `--param`,
`--policy-env` and `--runtime-read` accept more than one occurrence. A
repeated single-value option or switch is a command-line error. `-h` and
`--help` are also available on this verb.

### `--kind TEXT`

`--kind` is required and supplies `request.kind` unchanged. Its value is
nonempty UTF-8. Ravel has no catalogue of
[kinds](02-actors-and-words.md#caller-and-owner) and gets no kind from a task
file. Ravel gives the token no meaning. An absent or empty kind exits 2. See
[supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--prompt TEXT` and `--prompt-file PATH`

Exactly one of these two prompt inputs is required.

| Option | Source of `request.prompt` |
|---|---|
| `--prompt TEXT` | The argument text, also when it starts with `-`. |
| `--prompt-file PATH` | One read of the file, relative to the caller's working directory. |

The prompt can be empty. It contains no NUL, is valid UTF-8 and has a maximum
of 1,048,576 bytes (1 MiB). Ravel puts its bytes unchanged in
`request.prompt`, with trailing newlines. Ravel adds no instructions to end
the session. The policy decides how to give that text to its selected program.

An unreadable prompt file reports `prompt_unreadable`. Invalid encoding, a
NUL or too many bytes reports `prompt_invalid`. Both exit 2. A command with
no prompt input, or with both prompt inputs, also exits 2. Ravel does not
implicitly read the prompt from stdin, and it refuses a prompt file that is a terminal. See [supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

> **Note:** Shell command substitution removes trailing newlines before Ravel
> receives an argument. `--prompt-file` preserves those bytes. An
> operating system can set a smaller argument limit. This limit also applies
> when the policy puts a prompt from a file into the arguments of the selected
> command.

### `--task-file PATH` and `--task-id ID`

These optional fields carry caller data. Invalid input exits 2.

| Option | Request field | Value and effect |
|---|---|---|
| `--task-file PATH` | `request.taskFile` | Ravel joins the caller's working directory and the given path to make an absolute UTF-8 path. It does not canonicalise or read the path. The file does not have to exist, and the path supplies no kind or task identity. |
| `--task-id ID` | `request.taskId` | Nonempty opaque UTF-8 with a maximum of 1,024 bytes. It can start with `-`. Ravel does not search for a task record. |

See
[caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) for
both fields.

### `--config PATH`

`--config` selects a policy entry, relative to the caller's working directory.
Without it, Ravel uses `$HOME/.config/ravel/policy.ts`, which needs an
absolute `HOME`. There is no repository search and no environment variable
that selects a different policy. The entry resolves to a readable regular
file. Its canonical path is UTF-8 and contains no `?`. A symlink selects its
real file, and the directory of that file is the base for relative imports.

An empty option value is malformed input, status 2. If Ravel cannot accept
the entry file, the status is 3. A different entry does not move the settings
file from `$HOME/.config/ravel/settings.json`. See
[define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request)
and [resolve its imports](04-writing-an-owner-policy.md#resolve-policy-imports).

### `--param NAME=VALUE`

`--param` adds a string to `request.params`, and it is repeatable with
different names. The table gives the parts of each argument and their limit.

| Part | Rule |
|---|---|
| Name | Nonempty UTF-8 without `=`. Each name occurs only once. |
| Separator | The first `=` separates the name from the value. |
| Value | UTF-8, possibly empty or with more `=` characters. |
| Whole argument | Can start with `-`. |
| Combined size | All names and values together have a limit of 65,536 bytes. The separators do not count. |

A repeated name, a missing separator, invalid
encoding or a total above the limit exits 2. Ravel gives names and values no
meaning. See
[caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--context PATH`

`--context` reads a regular file, relative to the caller's working directory,
as version-1 JSON [policy context](02-actors-and-words.md#three-different-kinds-of-information).
Ravel validates it and records its canonical name, byte count and SHA-256.
Without `loadContext`, the delivered document is this data with the measured
sources attached. With a loader, the data is input to the loader, and the
loader assembles the document that `select` receives. With no caller document
and no loader, `select` receives `undefined` as its policy-context argument.

Ravel refuses an unreadable, invalid or oversized document with status 3.
Malformed command-line syntax still gives status 2. This option supplies
policy context, never a
[session-result context](02-actors-and-words.md#three-different-kinds-of-information). See [supply policy context](04-writing-an-owner-policy.md#supply-policy-context).

### `--timeout-ms MS`

`--timeout-ms` replaces the owner's `timeoutMs` setting for this invocation.
The table gives its syntax and values. Invalid syntax or range exits 2.

| Property | Value |
|---|---|
| Input syntax | Decimal digits. |
| Inclusive range | 1,000–600,000 milliseconds. |
| Default, with no flag or setting | 30,000 milliseconds. |

Ravel validates the settings before the flag replaces a value. Thus, an
invalid owner setting fails even with a valid flag.

The deadline starts when the Ravel CLI spawns the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision).
It covers the worker startup, the policy import, the policy-context loader
and the selection. At expiry, Ravel stops the selection, attempts the
teardown of the worker and exits 124 with no managed launch. Cleanup time is
additional. The deadline does not limit the time that the managed process
works. See
[set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--context-bytes BYTES`

`--context-bytes` replaces the owner's `contextBytes` setting. It takes decimal
digits in the inclusive range 1–8,388,608 bytes. With no flag and no setting,
the budget is 262,144 bytes. Invalid syntax or range exits 2. Ravel validates
the settings first. Thus, an invalid setting fails even with a valid flag.

The budget limits the caller document and the encoded policy context that
selection receives, with the measured source records. It is also the maximum
for a host source read. Ravel refuses data above a bound, and it does not
truncate that data. `context_too_large` uses status 3. See [policy context](04-writing-an-owner-policy.md#supply-policy-context)
and [bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--policy-env NAME`

`--policy-env` grants the policy worker one caller environment variable in
addition to its base set. Repeat it to grant more names.

| Grant property | Behaviour |
|---|---|
| Owner settings | The flags add to the owner's `policyEnv` array. |
| Duplicate names | Ravel merges them into one grant, in the order of their first occurrence. |
| Value | A grant carries the caller's current value. An unset variable stays absent. |
| Recipient | The policy worker. This option does not grant a variable to the managed process. |

A name is nonempty UTF-8 and contains no `=`. It cannot match a
[reserved grant class](#reserved-worker-grants). A malformed name reports
`malformed_input`, and a reserved name reports `excluded_grant`. Both exit 2.
Grant diagnostics show each name and whether it is set, never its value.
See [grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--confine`

`--confine` restricts the filesystem access and the environment of the
managed process. It uses macOS `sandbox-exec` or Linux bubblewrap.
Confinement is off when the flag is absent. The owner policy and its worker
stay outside [confinement](02-actors-and-words.md#trust-and-confinement).
Ravel tests the backend before policy evaluation. It never uses an
unconfined launch in place of a confined launch.

The table gives the access of the confined managed process.

| Resource | Access |
|---|---|
| Canonical working directory and private run directory | Writable. |
| System runtime paths, selected executable and real executable file of the Ravel CLI | Readable. |
| Network | Permitted. |
| POSIX session and terminal | A new POSIX session, with EOF on stdin. Ravel does not lend the caller's controlling terminal. |
| Environment | Ravel restricts the [managed-process environment](#managed-process-environment) separately from the environment of the policy worker. |

Filesystem grants cannot overlap the policy directory or the owner settings,
and this rule also applies through symlinks. An unusable backend or input
reports `confinement_unusable`, and an overlap reports `confinement_overlap`.
Both exit 2. See [confine the managed process](05-running-sessions.md#confine-the-managed-process)
for the layout and the operational limits.

> **Note:** The project measured how Ravel handles inherited signal
> settings, cancellation, cleanup and terminal recovery on aarch64 macOS and
> aarch64 GNU/Linux. For x86_64 GNU/Linux, the project built Ravel and ran it
> only under emulation. Native x86-64 qualification is still open. The
> [support limits](05-running-sessions.md#understand-the-operational-limits)
> give the scope of these measurements.

### `--runtime-read FILE`

`--runtime-read` adds one read-only runtime file to a confined launch, and it
is repeatable. It needs `--confine`. Without that switch, the command exits 2.
The path resolves from the caller's working directory to a regular file that
exists, and its canonical name is UTF-8. The option grants no directory tree.

Read-only grants stay read-only inside writable directories. A grant that is
unusable, or that has an overlap, exits 2 before policy evaluation. See
[confinement grants](05-running-sessions.md#confine-the-managed-process).

### `--exit-dir DIR`

`--exit-dir` gives the parent directory of the private
[result channel](02-actors-and-words.md#the-managed-process-and-its-result).
The parent is a writable directory that exists. Ravel resolves it from the caller's
working directory. The table shows the parent for each form of launch.

| Launch | Without `--exit-dir` | With `--exit-dir DIR` |
|---|---|---|
| Unconfined | The system temporary directory. | `DIR`. |
| Confined | The private run directory. | `DIR`, also as a writable grant. |

With confinement, the parent directory is also a writable grant. The option
chooses a parent, not a result filename or a persistent store. An invalid
parent can report `exit_dir_unusable` with status 2. If Ravel later cannot
allocate the result channel, it reports `session_channel_unavailable` with
status 5. See
[choose a channel parent](05-running-sessions.md#confine-the-managed-process).

### `--json`

`--json` replaces Ravel's text routing notice and failure document with
version-1 JSON on stderr. It does not change the result on stdout or the
output of the managed process. It is off when it is absent, and it belongs to
`start-session`. See [streams and notices](#streams-and-notices) for the fields and the order of output,
and [read notices](05-running-sessions.md#read-notices-separately-from-results)
for their use.

> **Note:** After a successful parse of the command line, a value such as
> `--prompt '--json'` is caller data, not a switch. If the parse fails, Ravel
> searches the argument words for an exact `--json` to choose the error
> format. Help, version and the help for a missing verb keep their usual
> display format.

### `end-session RESULT`

`ravel end-session RESULT` sends one UTF-8 argument to the live enclosing
session. The inherited `RAVEL_EXIT_FILE` identifies that session. The verb
accepts no session ID, target or settings option, and it reads no policy or
settings. `-h` and `--help` show the help of this verb. `--` permits a result
that looks like an option.

The table gives the inputs of `end-session`. Quote `RESULT` so that it
stays one argument.

| Input | Value or effect |
|---|---|
| `RESULT` | One UTF-8 argument. It can be empty, multiline or Unicode, with a maximum of 1,048,576 bytes (1 MiB). |
| `RAVEL_EXIT_FILE` | The inherited path of the live session's private result channel. |
| `--` | Ends the options, so a value that looks like an option can follow as `RESULT`. |
| `-h`, `--help` | Shows help, without a send. |

Ravel keeps the first complete result that it receives. That first
`end-session` requests shutdown, which normally ends the harness. A second
`end-session` races that shutdown. For example, another process in the
group can send a result before Ravel stops it. Ravel keeps the first result,
even if the second command succeeds. Shutdown can stop the second command,
or removal of the result channel can make it fail.

Success exits 0 and prints nothing. It means only that the channel accepted
the result, or that a valid first result was already present. It does not
mean that the job ended or that the result reached the caller. Cancellation
and failure of required cleanup still prevent a successful session result.

`end-session` exits 1, with a text diagnostic on stderr, if the [session-result context](02-actors-and-words.md#three-different-kinds-of-information)
is missing, empty, stale or invalid. It also exits 1 for an oversized value
or an I/O failure of the send. If the Ravel CLI finds an argument-count or encoding
error, it exits 2. The operating system can set a smaller argument limit
before the executable starts.
See [accept a result only after success](05-running-sessions.md#accept-a-result-only-after-success).

> **Note:** After Ravel detects the sent result, it gives a job that still
> runs 2 seconds. Then it sends TERM, waits up to 5 more seconds and uses KILL
> if necessary. If the direct child exits, Ravel stops the other group members
> before the reap, with no new grace period. Ravel can terminate code that
> runs after `end-session`.

## Exit statuses

For `start-session`, accept the result on stdout only when the final status
is 0. Neither the exit status of the managed process nor an earlier
successful `end-session` decides the result of the enclosing session.

Cancellation has precedence over an original failure. An original failure
keeps its status if a later cleanup also fails. Otherwise, a failure of the
required cleanup stops the delivery of the result. All applicable cleanup
reports stay as diagnostics. See
[outcome precedence](05-running-sessions.md#read-notices-separately-from-results).

| Status | Meaning and next action | Taught in |
|---|---|---|
| `0` | `start-session` delivered a valid result after the reap, the confirmed absence of the group, the required cleanup and a successful write and flush. An empty result is valid. A later nonzero exit of the managed process does not make the sent result invalid. Interpret the bytes with the caller's convention. For `end-session`, status 0 confirms only that it sent the result, or that a valid first result was already present. An explicit help or version request also exits 0. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| `1` | `start-session` reports `session_result_missing`, even when the managed process exits 0. For `end-session`, status 1 means that it could not send the result. For example, the session-result context was missing, empty or stale, or the value was too large. Check that the call occurs inside a live session. A missing result is not an empty result. | [The sent result and its absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `2` | Malformed command-line input or settings, or confinement inputs that are unusable or overlap. Without a verb, `ravel` prints help with this status. Correct the named input or setting before you try again. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |
| `3` | The policy refused, or Ravel could not accept the entry, behaviour, shape, selection result or policy context. No fallback route runs. Read the code, the stage and the remedy. For `policy_refused`, also read `policyCode`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `5` | A worker or supervision failed, or the result was invalid. Required cleanup or terminal recovery failed, a group survived, or the result output failed. A missing, stale or mismatched worker also gives 5. Retain the diagnostics, repair the installation or the named resource, and discard any partial result bytes. | [Operational diagnostics](05-running-sessions.md#read-notices-separately-from-results) |
| `124` | Policy evaluation timed out, and no managed process started. Examine the work of the policy before you increase `timeoutMs` or `--timeout-ms`. The deadline does not limit the managed process. | [Bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `126` | The selected program is not executable, or the launch failed before the runner reported the start, for a reason other than ENOENT. Check the permissions, the format, the interpreter and the execution environment. Retain any recovery reports. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `127` | Ravel did not find the selected program, or the launch failed with ENOENT before the start. A script that exists can have a missing interpreter. Check `PATH`, the selected path and the interpreter. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `129` | Cancellation by SIGHUP (signal 1), during selection or supervision. Ravel accepts no result. Retain the cleanup reports, and try again only if the caller still wants the work. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `130` | Cancellation by SIGINT (signal 2), during selection or supervision. Ravel accepts no result. A keyboard Ctrl-C that goes only to the group of the managed process is not necessarily a cancellation of Ravel. | [Cancellation and terminal INT](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `143` | Cancellation by SIGTERM (signal 15), during selection or supervision. Ravel accepts no result. Retain the cleanup reports, and try again only if the caller did not intend the termination. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |

The three signal statuses in the table follow the shell convention: 128 plus
the signal number. Ravel attempts the cleanup and then raises the same signal
again. Thus, a process API reports termination by a signal. If the
propagation fails, Ravel reports the failure and uses the numeric status in
its place. A caller that uses a process API handles both cases. If a signal
is ignored or blocked when Ravel starts, that signal does not automatically
cause a cancellation.

> **Note:** SIGKILL or another abrupt death can stop Ravel before it decides
> the final outcome. A signal that arrives after the final decision about
> cancellation can interrupt the result write. Neither case confirms cleanup
> or successful delivery. A failed final write can leave a prefix on stdout.
> The [operational limits](05-running-sessions.md#understand-the-operational-limits)
> describe these cases.

## Streams and notices

### Standard streams

> **Table — Streams:** Do not mix the result on stdout with stderr. Stderr
> combines the managed output and the diagnostics. Stdin serves the managed
> process and does not implicitly supply a prompt.

| Stream | Session behaviour | Taught in |
|---|---|---|
| stdout | `start-session` writes only the exact successful result, with no added newline. An empty sent result is zero bytes. A failed final write can leave a prefix, so check the status. `end-session` writes nothing to stdout. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| stderr | The stdout and stderr of the managed process, Ravel notices and failures, captured policy diagnostics and operational reports. `--json` changes only Ravel's routing notice and failure document. | [Read notices](05-running-sessions.md#read-notices-separately-from-results) |
| stdin | Ravel does not implicitly read a prompt from stdin. The worker receives null stdin. An unconfined managed process can use the caller's stdin, and a confined one receives EOF. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |

A text routing notice starts with `ravel: starting provider` and names the
provider, model, effort and kind before the launch. Captured policy lines
have the prefix `policy stdout:` or `policy stderr:`. A failure starts with
`ravel: refused (CODE, stage STAGE):`. The word “refused” in this text also
covers operational failures, and the code and the status show which failure
it is. A failure of `end-session` to send the result starts with
`ravel: end-session:` instead. See [read notices](05-running-sessions.md#read-notices-separately-from-results).

### JSON routing notice

Ravel prepares the `--json` routing notice after three steps succeed: the
selection, the reservation of the result output and the allocation of the
result channel. It prints the notice after the supervision attempt, before it
decides the final outcome. The notice does not prove the launch, the cleanup
or success. Stderr stays a mixed stream, and the managed process can print
text that looks like a Ravel document. See
[read notices](05-running-sessions.md#read-notices-separately-from-results).

| Field | Value and meaning |
|---|---|
| `schemaVersion` | `1`, the version of the notice format. |
| `selection` | An object with `provider`, `model`, `effort` and `reason`. The owner supplies these strings. The labels do not connect Ravel to a model provider. |
| `policy` | `path`, the canonical path of the accepted entry, and `version`, the declared version of the policy. |
| `worker` | `path`, `version` (the package version), `buildId` and `bunVersion` of the matched compiled policy worker. The Ravel CLI accepts only a worker with its own build identity. This check is not code signing. |
| `selectionMs` | The elapsed time of policy evaluation, in milliseconds. |
| `context` | `null` when there is no delivered policy context. Otherwise, the report in the next table. It is never the session-result context. |
| `bounds` | Objects named `selection`, `context`, `source`, `sources`, `message` and `diagnostics`. Each has its field for the value with a unit (`ms`, `bytes` or `sources`) and `from`. `from` gives `default`, `fixed`, `settings.json` or the flag that set the value. |
| `policyEnvironment` | An array of `{name, set}` grants. `set` is a boolean that shows whether the caller has the variable, never its value. Ravel does not add the [base variables](#caller-inputs-and-worker-environment) of the worker to this list. |
| `resolution` | A string that gives the resolved executable path and the base of the resolution: an absolute path, the caller's working directory or a `PATH` entry. |
| `promptSource` | `argument` for `--prompt`. For `--prompt-file`, the file path joined to the caller's working directory. |
| `diagnostics` | An object with `stdout` and `stderr`, the captured policy streams as strings. Invalid UTF-8 bytes show as replacement characters. |
| `operationReports` | An array of the report strings that the [runner](02-actors-and-words.md#policy-evaluation-and-supervision) retained. It is present only when the runner returned its account of how the job ended. It is empty when there are no reports. A failed stop attempt in this list does not alone decide the final status. |

The non-null `context` report has six fields. They describe the data that
selection receives. [Chapter 4's policy-context section](04-writing-an-owner-policy.md#supply-policy-context)
teaches where they come from.

| Policy-context report field | Meaning |
|---|---|
| `loader` | Whether the `loadContext` of the policy made the delivered document. |
| `sources` | Measured source records. Each has the canonical `name`, `via` (`--context`, `readText` or `readJson`), `bytes` and `sha256`. |
| `sourceBytes` | The sum of the measured source byte counts. |
| `encodedBytes` | The byte length of the compact JSON encoding of the delivered document, with the measured records. |
| `sha256` | The SHA-256 of that encoding. |
| `value` | The delivered policy-context document. |

> **Note:** The notice can contain policy-context data and captured policy
> output. Grant diagnostics do not show environment values, but trusted
> policy code can still print a value itself.

### JSON failure document

A JSON failure document has `schemaVersion: 1`, an `error` object and an
optional top-level `diagnostics`. That object contains the policy `stdout` and
`stderr` strings. Read `code` and `stage` together, and retain the complete
stderr stream. Secondary operational reports can occur outside this document.
See [read failure notices](05-running-sessions.md#read-notices-separately-from-results).

| `error` field | Presence and meaning |
|---|---|
| `code` | Required. A stable Ravel diagnostic code, such as `malformed_input` or `session_result_missing`. |
| `stage` | Required. A stage string from the table below. |
| `message` | Required. An explanation. It can contain original and secondary operational details. |
| `remedy` | Required. A suggested correction or recovery action. |
| `exit` | Required. The numeric status. A cancellation also has its numeric status here. |
| `input` | Optional. The caller input involved, such as a flag. |
| `source` | Optional. The file that relates to the failure, such as the policy entry. |
| `location` | Optional. The location inside the source or data, such as `policy.select` or `result.args[2]`. |
| `policyCode` | Optional. The refusal code that the owner supplied, beside Ravel's `policy_refused`. |
| `bound` | Optional. The exhausted bound: `name`, the field for the value with its unit, and `from`. |
| `signal` | Optional. `SIGINT`, `SIGTERM` or `SIGHUP`, the signal of the cancellation. |

The stage tells where Ravel found a problem, not which process ultimately caused it. For
example, Ravel uses `evaluation` for the whole deadline, because the Ravel
CLI cannot see which policy operation ran at expiry.

| Stage | Scope | Taught in |
|---|---|---|
| `cli` | The command line and the owner settings. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `authority` | The choice of the policy entry and the attempt to open it, or the query for the caller's working directory. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `worker` | The search for the compiled worker, its start, or the check of the worker and its protocol. | [Check and evaluate](03-a-complete-session.md#check-the-request-and-evaluate-the-policy) |
| `evaluation` | The evaluation by the worker as a whole, with the deadline and the cancellation of selection. | [Bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names), [cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `load` | The import of the policy entry. | [Resolve imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `context` | The read, assembly, validation and measurement of policy context. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `validation` | The check of the exported policy object. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `selection` | The call to `select` and the check of the data that it returns. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `resolution` | The search for the selected program. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `exec` | The preparation and supervision of the managed launch, and the decision about its outcome. This stage also covers failures of the result output or the result channel before the spawn. | [Results](05-running-sessions.md#accept-a-result-only-after-success), [notices](05-running-sessions.md#read-notices-separately-from-results) |

## Environment variables

### Caller inputs and worker environment

The Ravel CLI starts the policy worker with a fresh environment. It contains
each of `HOME`, `PATH`, `TMPDIR`, `LANG` and the `LC_*` names that the caller
has set, with the caller's value. The explicit grants come next, and
`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` comes last. Other caller variables do
not reach the worker without a grant. See
[grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Variable or class | Meaning and effect | Taught in |
|---|---|---|
| `HOME` | Locates the default policy and the settings. The default policy needs an absolute value. With an explicit `--config`, an absent or relative `HOME` gives no owner settings. Ravel passes it to the worker when it is set. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `PATH` | The Ravel CLI resolves a bare selected program with the caller's value. If `PATH` is unset, there is no search. An empty value is an entry for the working directory. Relative entries resolve from the caller's working directory. The worker also receives this value, but `PATH` does not select the installed worker. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `TMPDIR` | The Ravel CLI uses it to choose the system temporary directory. That directory holds the private worker and run directories. It also holds the result channel of an unconfined launch without `--exit-dir`. The caller value reaches the worker when it is set. | [Channel placement](05-running-sessions.md#confine-the-managed-process), [worker environment](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `LANG`, `LC_*` | Caller locale values that Ravel passes to the worker when they are set. `LC_*` means every name with that exact case-sensitive prefix. | [Worker base environment](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| Explicitly granted names | `policyEnv` settings and `--policy-env` flags, with caller values. Ravel merges duplicate names. Unset names stay absent. | [Grant names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `BUN_RUNTIME_TRANSPILER_CACHE_PATH` | Ravel sets it to `0` for the worker, and this value disables the runtime transpiler cache. A caller value cannot replace it through a grant, because `BUN_*` is a reserved grant class. | [Resolve policy imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `XDG_CONFIG_HOME` | Ravel does not use it to find the policy or the settings. Its presence does not move `$HOME/.config/ravel`. It has no special grant exclusion. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |

### Reserved worker grants

`policyEnv` and `--policy-env` do not accept these nine exact names or prefix
classes. The match is case-sensitive. `*` means any suffix, also the empty
suffix. It is not shell expansion. A grant that names one of them reports
`excluded_grant` with status 2.

The reserved classes are rules for worker grants. They do not tell which
names Ravel removes from the environment of an unconfined managed process.
The next section gives that shorter list. See
[grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Name or class | Why the worker does not accept a caller grant |
|---|---|
| `BUN_*` | Bun startup and runtime controls, for example preload and executable mode. Ravel sets its own cache control. |
| `NODE_OPTIONS` | Options for the runtime and for the load of code. |
| `NODE_PATH` | Module search path. |
| `NODE_PRESERVE_SYMLINKS` | Resolver behaviour. |
| `NODE_CHANNEL_*` | Runtime controls for IPC descriptors. |
| `LD_*` | Dynamic-loader controls. |
| `DYLD_*` | macOS dynamic-loader controls. |
| `RAVEL_*` | Ravel's reserved controls. They include `RAVEL_EXIT_FILE`, the session-result context that the worker never receives. |
| `HARNESS_DISPATCH_*` | Names of the earlier `harness-dispatch` command. Ravel reserves them with its own names. |

### Managed-process environment

An unconfined managed process inherits the caller's environment, except for
six exact names that Ravel removes. The list of these names is below. Ravel
then supplies its fresh `RAVEL_EXIT_FILE`. Other names stay inherited, for
example any application credentials and the names in the classes that the
worker does not accept. A worker grant has no effect on that inheritance. See
[caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

| Name or class | Confined managed process | Taught in |
|---|---|---|
| `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG`, `LC_*` | Ravel retains the caller values when they are present. A retained `HOME` gives no filesystem access to that directory. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `TMPDIR`, `TMP`, `TEMP` | Ravel sets all three to the temporary directory of the private run. They replace the caller values. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `RAVEL_EXIT_FILE` | A fresh path for this session. Ravel supplies it after the environment grants. | [Nested sessions](05-running-sessions.md#keep-nested-sessions-separate) |
| Every other caller variable | Absent. `--policy-env` grants only the worker and cannot add the variable here. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |

The removed names are exactly `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID`, `RAVEL_STATE_DIR`,
`HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`. Ravel replaces the first with the new
result-channel path. The other five stay absent. These obsolete identity and
storage names create no store and select no session. See
[caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `RAVEL_EXIT_FILE`

`RAVEL_EXIT_FILE` is the inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information):
the path that `end-session` uses to send the result to its enclosing session.
The Ravel CLI sets it only for the managed launch. The policy worker never
receives it. A nested launch replaces it only for its own managed process.
The value of the outer caller does not change.

If the path is unset, empty or stale, `end-session` fails with status 1. Do
not use it as a persistent result location or as an address across sessions.
The result channel separates ordinary sessions. It is not a defence against
hostile code that runs as the same user. See
[nested sessions](05-running-sessions.md#keep-nested-sessions-separate) and
[operational limits](05-running-sessions.md#understand-the-operational-limits).

## Policy entry and settings

### `policy.ts`

`$HOME/.config/ravel/policy.ts` is the default owner entry. This default
needs an absolute `HOME`. `--config PATH` chooses a different entry, relative
to the caller's working directory. There is no repository search, environment
override or fallback policy.

The entry must resolve to a readable regular file, with a UTF-8 canonical
path that contains no `?`. A symlink uses its real file. The directory of the
real file is the base for relative imports. If Ravel cannot accept the entry
file, it exits 3 before owner code runs. See [define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request)
and [resolve imports](04-writing-an-owner-policy.md#resolve-policy-imports).

`policy` is the named export that Ravel evaluates. A default export alone is
not sufficient. The export is a plain object, not an array or a class
instance, with the [four policy fields](#policy-object) below. Trusted owner
code runs outside the confinement of the managed process, in a separate
worker that never receives a session-result context. After a successful
selection, the Ravel CLI reaps the worker and removes its private directory
before the launch. See
[policy evaluation](03-a-complete-session.md#check-the-request-and-evaluate-the-policy).

### `settings.json`

`$HOME/.config/ravel/settings.json` supplies owner defaults for `start-session`.
`--config` does not move it, and `XDG_CONFIG_HOME` does not select it. A
missing file, an absent or relative `HOME` or an empty JSON object supplies
no settings. A file that exists must be a readable regular file that contains
a JSON object. `end-session` reads neither settings nor policy. See
[set bounds and grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Key | Accepted value, default and consequence | Taught in |
|---|---|---|
| `timeoutMs` | Optional whole number of milliseconds, 1,000–600,000 inclusive. The default is 30,000. `--timeout-ms` replaces the value after settings validation. It limits the worker startup, the policy import, the policy-context loader and the selection, not the managed process. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `contextBytes` | Optional whole number of bytes, 1–8,388,608 inclusive. The default is 262,144. `--context-bytes` replaces it after validation. It limits the caller document and the delivered policy context. It is also the ceiling for a host read. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `policyEnv` | Optional array of environment-variable names. The default is no extra grants. Each name is nonempty UTF-8 without `=` and cannot match a reserved grant class. Flags add names. Ravel merges duplicates, and unset values stay absent. Grants affect the worker, not the inheritance of the managed process. | [Grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

These problems report `settings_invalid`, status 2:

- an unknown key, also the obsolete `stateDir` and a `config` key
- an unreadable or non-regular file
- invalid JSON, or a document that is not an object
- a `policyEnv` that is not an array of strings

Invalid numeric bounds and malformed grant names use `malformed_input`, and
excluded grants use `excluded_grant`. Both have status 2. Ravel validates the
whole file before flags replace values. Thus, an invalid setting fails even
with a valid flag. No setting opens persistent storage. See
[settings validation](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Bounds

`request.limits` gives six effective bounds. They are frozen values for the
policy to read, and the Ravel CLI and the worker enforce their own copies. A
policy cannot make them larger. Ravel refuses data that is too large, and it
does not truncate that data. The JSON notice names each bound separately and
records its origin.
See [set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Request field / notice name | Default or fixed value | Adjustment and consequence | Taught in |
|---|---:|---|---|
| `selectionMs` / `selection` | 30,000 ms | `timeoutMs`, then `--timeout-ms`, within 1,000–600,000. Ravel measures it from the worker spawn through the startup, the import, the loader and the selection. Expiry is `selection_timeout`, status 124. The teardown time is additional. | [Selection deadline](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `contextBytes` / `context` | 262,144 bytes | `contextBytes`, then `--context-bytes`, within 1–8,388,608. The caller file and the compact delivered JSON with `measured` must both fit. If one does not fit, the code is `context_too_large`, status 3. | [Policy-context budget](04-writing-an-owner-policy.md#supply-policy-context) |
| `sourceBytes` / `source` | The smaller of 65,536 bytes and `contextBytes` | A positive whole-number `maxBytes` of a host read can replace the default, up to `contextBytes`. Too many bytes, or a requested limit that is too large, cause `source_too_large`, status 3. The loader cannot prevent this refusal when it catches the exception. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `sources` / `sources` | Fixed 256 | It applies separately to the measured reads, with the caller document, and to the attributed `sources` array of the document. If either has more, the code is `too_many_sources`, status 3. A caught exception for a read bound still causes the refusal. | [Measured and attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `messageBytes` / `message` | Fixed 1,048,576 bytes | It limits a policy snapshot, a selection result or a failure report in the private protocol. A message that is too large gives `message_too_large`, status 3. A delivered document has the policy-context budget instead. | [Protocol bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `diagnosticsBytes` / `diagnostics` | Fixed 262,144 bytes | The combined worker stdout and stderr. Output above the limit gives `output_limit`, status 3. Ravel retains the first 262,144 bytes and discards the rest while it stops the worker. | [Output bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

`from` in a reported bound is `default`, `fixed`, `settings.json` or the flag
that set the bound. If the policy-context budget makes a source limit lower,
that limit gets the origin of the budget. If a read gave its own limit, a
source-bound refusal reports `maxBytes` as its `from`. The report uses `ms`,
`bytes` or `sources` for its numeric field. See [bounds and their inputs](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Policy and request

### Policy object

`policy` has the following fields. Ravel refuses unknown fields.
`definePolicy` helps TypeScript to check the object, but it does not replace
the runtime validation.
See [define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request).

| Field | Contract | Taught in |
|---|---|---|
| `schemaVersion` | Required literal number `2`. Other whole-number versions report `unsupported_version`. Missing or malformed values report `policy_invalid`, status 3. A version-1 policy declared `catalog` and `routes` in place of `select`. Ravel does not convert it automatically. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `version` | Required nonblank string that the owner maintains. Ravel reports it as the policy version. It is independent of Ravel's package version. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `select(request, context, host)` | Required function. It returns `Selected` or `Refused`, directly or through a promise. It receives the delivered policy context or `undefined`, and a host with `diagnostic` and `signal`. No fallback runs for a missing function, malformed output, an exception or a promise that can never resolve or reject. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `loadContext(request, host)` | Optional function. It returns version-1 `Context` or `Refused`, directly or through a promise, before `select`. If it fails, no selection occurs. Its host supplies measured reads. Without it, the caller's document, if there is one, is the delivered data. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |

### `SelectionRequest`

`SelectionRequest` is frozen, with its nested objects and arrays. An optional
field is absent when the caller did not supply it. Ravel does not give it the
default value of an empty string. The request carries data and limits, never
the session-result context. See [read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request).

| Field | Value and limits | Taught in |
|---|---|---|
| `schemaVersion` | Always the number `2`, independent of policy-context schema 1. | [Read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `kind` | Required nonempty caller token, unchanged from `--kind`. Ravel defines no catalogue. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt` | Required UTF-8 string, possibly empty, with a maximum of 1,048,576 bytes and no NUL. The caller bytes and newlines stay unchanged. The policy chooses how to give them to its program. | [Prompt delivery](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `cwd` | The caller's working directory as data. Relative input paths and selected program paths resolve from it. The worker starts owner code with `/` as its working directory. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `params` | String values by caller-supplied name, or `{}` if there are none. Names are nonempty without `=`. Ravel refuses a duplicate name. All names and values together have a maximum of 65,536 bytes. | [Caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskFile` | Optional absolute UTF-8 path, joined from `--task-file`. Ravel does not canonicalise or read it, and the file does not have to exist. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskId` | Optional nonempty opaque UTF-8 string, with a maximum of 1,024 bytes. It names no Ravel record. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `context` | Optional validated version-1 policy-context document of the caller, before Ravel attaches `measured`. A loader sees this input, also when it returns different data for `select`. | [Caller and delivered documents](04-writing-an-owner-policy.md#supply-policy-context) |
| `limits` | Required object with the six effective fields in [Bounds](#bounds). The policy cannot replace them. | [Effective bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

## Selection and refusal

### `Selected`

`Selected` is the command that `select` returns. All seven fields are
required, and Ravel refuses unknown fields. Ravel validates the shape, then
resolves the program. It does not check that the prompt is in the arguments
or that the labels describe the actual program. See [select a command](04-writing-an-owner-policy.md#select-a-command-or-refuse).

| Field | Accepted value and effect | Taught in |
|---|---|---|
| `status` | Literal `"selected"`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `program` | Nonblank string without NUL. It is an absolute path, a path relative to the caller that contains `/`, or a bare name that Ravel finds through the caller's `PATH`. It also supplies `argv[0]`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `args` | Array of strings after `program`. Ravel accepts an empty array and empty strings, but not NUL. Each string is one argument. Ravel does no word splitting, shell expansion or template interpretation. | [Construct arguments](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `provider` | Nonblank owner label for the model origin. Ravel reports it as given. Ravel does not infer it or use it to contact a provider. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `model` | Nonblank owner label. It does not have to occur in `args`, and it does not configure the selected program by itself. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `effort` | Nonblank owner label. Ravel defines no effort levels and does not change this label into a flag. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `reason` | Nonblank explanation by the owner. Ravel reports it verbatim in routing diagnostics. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |

### `Refused`

`Refused` declines the request from `select` or `loadContext`. All four
fields are required, and Ravel refuses unknown fields. Ravel launches nothing
and tries no other route. A valid refusal reports Ravel's `policy_refused`,
status 3, with the owner's code beside it as `policyCode`. The stage is
`selection` for `select` and `context` for `loadContext`. See [return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse)
and [refuse from a loader](04-writing-an-owner-policy.md#supply-policy-context).

| Field | Accepted value and effect | Taught in |
|---|---|---|
| `status` | Literal `"refused"`. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `code` | Nonblank owner-defined string. It is separate from the namespace of Ravel's diagnostic codes. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `message` | Nonblank explanation of what prevents selection. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `remedy` | Nonblank action that the caller or owner can take. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |

### Diagnostic codes

In a JSON failure document, `error.code`, `error.stage` and `error.exit`
identify a Ravel failure. These tables give the usual classification of each
code.

Cancellation has precedence over a failure that already occurred.
Otherwise, the original error decides the code and status, and Ravel reports
later cleanup errors beside it. Thus, a code in these tables can also occur
as retained detail beside a different outcome. Read the complete stderr
stream. See [interpret notices](05-running-sessions.md#read-notices-separately-from-results).

| Code | Stage | Status | Meaning and remedy | Taught in |
|---|---|---|---|---|
| `malformed_input` | `cli` | 2 | Invalid CLI input, numeric setting or grant name. Correct the named input and range. An invalid setting fails even with a valid flag. | [Inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `prompt_unreadable` | `cli` | 2 | Ravel cannot open or read the prompt file, or the file is a terminal. Supply a readable file that is not a terminal, or use `--prompt`. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt_invalid` | `cli` | 2 | The prompt is invalid UTF-8, contains NUL or is larger than 1 MiB. Correct the encoding or the content, or make the prompt shorter. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `settings_invalid` | `cli` | 2 | The settings file or its shape is invalid, for example with an unknown key. Correct it to the three optional keys, or remove it to use the defaults. | [Settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `excluded_grant` | `cli` | 2 | A grant in the settings or in a flag names a reserved variable or prefix. Remove the grant. A grant cannot replace the startup controls of the worker. | [Grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `confinement_unusable` | `cli` | 2 | Ravel cannot use the backend or the confinement inputs. Correct the named backend or path. No unconfined launch runs in its place. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `confinement_overlap` | `cli` | 2 | A grant overlaps the policy directory or the settings. Separate the paths, also their canonical symlink targets. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `exit_dir_unusable` | `cli` | 2 | The parent of the result channel is unusable. Name a writable directory that exists, or omit `--exit-dir`. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `cwd_unavailable` | `authority` | 3 | Ravel cannot get the caller's directory. Run from a readable directory that exists. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `home_unset` | `authority` | 3 | The default entry has no absolute `HOME`. Set an absolute `HOME` or use `--config`. | [Policy entry](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `policy_missing` | `authority` | 3 | The chosen entry does not exist. Create it or correct `--config`. Ravel does no search. | [Policy entry](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `policy_unreadable` | `authority` | 3 | The entry is unreadable or non-regular, or Ravel cannot import its canonical name exactly. Correct the file, the permissions or the UTF-8 path without `?`. | [Policy entry and imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `worker_missing` | `worker` | 5 | Ravel cannot find or start the worker at its location relative to the canonical path of the Ravel CLI. Reinstall or rebuild the matched pair, and check the execute permissions. | [Installed pair](03-a-complete-session.md#check-the-request-and-evaluate-the-policy) |
| `worker_identity_mismatch` | `worker` | 5 | The protocol, package version or source identity does not match the Ravel CLI. Install both binaries from one build. Ravel never accepts a test-only probe build of the worker. | [Installed pair](03-a-complete-session.md#check-the-request-and-evaluate-the-policy) |
| `worker_failed` | `worker` | 5 | The preparation, execution or required teardown of the worker failed. Examine the retained operation details, and repair the installation or the named resource. | [Operational failures](05-running-sessions.md#read-notices-separately-from-results) |
| `protocol_error` | `worker` | 5 | The worker conversation is malformed, closed unexpectedly or is not valid for a different reason. Rebuild or reinstall the matched pair. If the error occurs again, retain the diagnostics. | [Operational failures](05-running-sessions.md#read-notices-separately-from-results) |
| `selection_timeout` | `evaluation` | 124 | The whole worker deadline expired, with the startup, the policy import and the policy-context loader. Correct the stalled work, or increase the adjustable bound within its range. No managed launch follows. | [Deadline](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `selection_cancelled` | `evaluation` | 129/130/143 | HUP, INT or TERM ended the selection. Retain the teardown reports, and try again only if the caller still wants the work. Termination by the signal is the usual outcome at the process level. | [Selection cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `output_limit` | `evaluation` | 3 | The stdout and stderr of the worker together are larger than 262,144 bytes (256 KiB). Print less. The retained prefix is diagnostic evidence, not an accepted selection. | [Output bound](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `policy_import_failed` | `load` | 3 | The entry or an import throws or rejects. Or it has an await that can never finish, because no work is left that can resolve or reject it. Correct the import or the await, and install dependencies explicitly. | [Imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `policy_invalid` | `validation` | 3 | The named export is missing, is not plain, is unserialisable or has invalid fields. Return the version-2 policy object with a callable `select`. | [Policy shape](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `unsupported_version` | `validation` or `context` | 3 | The policy schema is not 2, or the policy-context schema is not 1. Use the supported shape or a compatible Ravel. A version-1 policy needs new code, not only a changed number. | [Policy shape](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request), [policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `message_too_large` | `load`, `context` or `selection` | 3 | A policy snapshot, a selection result or a failure report is larger than the fixed 1 MiB protocol bound. Make the related data or report shorter. This code also covers an oversized loader failure report at `context`. | [Protocol bound](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `context_unreadable` | `context` | 3 | Ravel cannot read the caller document as a regular file with a canonical UTF-8 path. Correct `--context` or omit it. | [Caller policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_invalid` | `context` | 3 | The caller or loader document is invalid JSON, or it has invalid fields or data. Or the loader returned nothing, unserialisable data or a malformed refusal. Correct the named location to version-1 data or a valid refusal. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_too_large` | `context` | 3 | The caller bytes, or the delivered JSON with the measurements, are larger than the budget. Deliver less, or increase `contextBytes` or `--context-bytes` within its range. | [Policy-context budget](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_loader_failed` | `context` | 3 | The loader throws or rejects. Correct its computation, or supply what it needs. `select` does not run without the data that the loader requires. | [Loader failures](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_source_unreadable` | `context` | 3 | A missing or unreadable file, bad UTF-8 or bad JSON in a host read caused the loader to fail. Supply or correct that source, or change what the loader requires. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_loader_unsettled` | `context` | 3 | The loader promise does not resolve or reject, and no work is left that can do so. Make every path resolve or reject. If active work uses all of the deadline, the code is `selection_timeout`. | [Loader failures](04-writing-an-owner-policy.md#supply-policy-context) |
| `source_too_large` | `context` | 3 | A host source is larger than its read limit, or the requested `maxBytes` is larger than the policy-context budget. Read less or choose an allowed limit. Selection does not occur, even if the loader catches the error. | [Host read bounds](04-writing-an-owner-policy.md#supply-policy-context) |
| `too_many_sources` | `context` | 3 | The count of measured or attributed sources is more than 256. Combine sources, or read or attribute fewer. | [Source limits](04-writing-an-owner-policy.md#supply-policy-context) |
| `policy_refused` | `selection` or `context` | 3 | `select` or the loader returned a valid refusal. Follow its `policyCode`, message and remedy, which the owner supplied. No alternative route runs. | [Refusals](04-writing-an-owner-policy.md#select-a-command-or-refuse), [loader refusals](04-writing-an-owner-policy.md#supply-policy-context) |
| `selection_abstained` | `selection` | 3 | `select` returned `undefined` or `null`. Return a selected command or an explicit refusal on every path. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_malformed` | `selection` | 3 | The result has the wrong shape, fields or types, or contains NUL, or Ravel cannot serialise it. Correct the named location to `Selected` or `Refused`. | [Selection shape](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_threw` | `selection` | 3 | `select` threw or rejected. Correct the failure. To decline intentionally, return a `Refused` value. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_unsettled` | `selection` | 3 | The promise does not resolve or reject, and no work is left that can do so. Make every path resolve or reject. If active work reaches its deadline, the code is `selection_timeout`. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `program_not_found` | `resolution` | 127 | The selected path does not exist, or no entry of the caller's `PATH` has the bare name. Correct the path or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `program_unexecutable` | `resolution` | 126 | The selected file is not an executable regular file, or Ravel cannot search the caller's `PATH` for a bare name. Correct the permissions, the file or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |

| Code | Stage | Status | Meaning and remedy | Taught in |
|---|---|---|---|---|
| `cancellation_unavailable` | `exec` | 5 | The Ravel CLI cannot prepare for cancellation. Examine the reported OS error and the execution environment. | [Operational failures](05-running-sessions.md#read-notices-separately-from-results) |
| `signal_state_unavailable` | `exec` | 5 | The capture of the signal state at entry, or a required transition of the signal state, failed. Retain all reports, and correct the execution environment. Do not treat any result as success. | [Signal and recovery limits](05-running-sessions.md#understand-the-operational-limits) |
| `output_unavailable` | `exec` | 5 | Ravel cannot reserve or restore the result stdout. Supply a writable output stream, and examine any error of the restoration. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_channel_unavailable` | `exec` | 5 | Ravel cannot allocate or prepare the private result channel. Correct the temporary storage or `--exit-dir`, and examine the reports of a partial cleanup. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `supervision_failed` | `exec` | 127, 126 or 5 | Before the runner reports the start, ENOENT gives 127, and other failures give 126. After the start, the status is 5. Check the executable and the interpreter, the execution environment and the retained reports of the runner and the terminal. | [Launch and supervision](05-running-sessions.md#interpret-the-exit-status) |
| `session_result_missing` | `exec` | 1 | The managed process ended and sent no result, even if its own status was 0. Make the managed process call `end-session` when its work succeeds. | [The sent result and its absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_result_invalid` | `exec` | 5 | Ravel cannot accept the sent entry in the result channel or the result bytes, for example because of an invalid encoding, type or size. Use `end-session` inside the live session, and send a result of 1 MiB or less. | [Result validity](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cleanup_failed` | `exec` | 5 | The required cleanup of the result channel or of a private directory failed. Examine the named path and the ownership report before you remove any residue. Ravel returns no candidate result. | [Cleanup limits](05-running-sessions.md#understand-the-operational-limits) |
| `process_group_survived` | `exec` | 5 | Ravel did not confirm the absence of the group when it decided the final outcome. Examine the reported group and the cleanup failures before you try again. The sent result alone is not sufficient. | [Cleanup and success](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cancelled` | `exec` | 129/130/143 | HUP, INT or TERM wins when Ravel decides the final outcome, even after Ravel received the result. Discard the result, and retain the original and cleanup errors. Try again only if the caller still wants the work. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `result_write_failed` | `exec` | 5 | The final write or flush of the result failed after Ravel decided the outcome. Correct the output destination, and discard any prefix that Ravel already wrote. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |

An `end-session` failure is a plain `ravel: end-session:` message with
status 1. It is not an entry in the JSON code table. A missing, empty or stale
`RAVEL_EXIT_FILE`, an oversized argument or a failed send does not mean an
empty result. See [send and accept results](05-running-sessions.md#accept-a-result-only-after-success).

## Policy context and host operations

### `Context` and `DeliveredContext`

`Context` is version-1 JSON data. `--context` supplies it, or `loadContext`
returns it. An absent optional field means unknown, which is different from a
known empty object, array or string. Ravel refuses unknown top-level fields,
for example `program`, `args`, `argv`, `command`, `select`, `loadContext`, `env`,
`environment`, `exec`, `shell` and `script`. These names inside free-form JSON
facts stay data. See [supply policy context](04-writing-an-owner-policy.md#supply-policy-context).

| Field | Accepted value | Taught in |
|---|---|---|
| `schemaVersion` | Required number `1`. It is independent of the version-2 request and policy contract. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `summary` | Optional string, which can be empty. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `acceptanceCriteria` | Optional array of strings. The array and the strings can be empty. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `facts` | Optional object that maps names to JSON values. A fact is data. Ravel never runs it as a command. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `assessments` | Optional object that maps names to attributed `{ by, value }` objects. | [Attributed judgements](04-writing-an-owner-policy.md#supply-policy-context) |
| `sources` | Optional array of 256 or fewer attributed source records. These records are separate from measured reads. A record does not cause the read of a file. | [Source attribution](04-writing-an-owner-policy.md#supply-policy-context) |

`DeliveredContext` adds the required `measured` to the accepted document and
is deeply frozen. Neither the caller nor the loader supplies that field.
Without a loader, selection receives the caller document with the
measurements, or `undefined` if there is no document.

A loader replaces the document that `select` receives, but `request.context`
still gives the caller's original input. Ravel does not merge the two
automatically. Ravel refuses a loader that returns nothing. See
[caller and delivered documents](04-writing-an-owner-policy.md#supply-policy-context).

`measured` holds the successful reads in order. The caller's `--context`
document comes first when it is present, then the host reads, also repeated
reads of one file. The separate limit of this list is 256 records. The
compact encoding of the whole delivered document, with this list, must fit
`contextBytes`. Source attribution and measurement do not prove who the
author was or that a claim is true. See
[measurement and its limits](04-writing-an-owner-policy.md#supply-policy-context).

### Nested data and measured records

`Json` allows null, booleans, finite numbers, strings, arrays and objects of
JSON values. Ravel cannot deliver cycles, executable values or other values
that are not JSON. Ravel reserves an object whose only key is `$ravel`, also
inside facts or assessment values, and refuses it as `context_invalid`. See
[JSON policy data](04-writing-an-owner-policy.md#supply-policy-context).

| Type and field | Accepted value or meaning | Taught in |
|---|---|---|
| `Assessment.by` | Required nonblank string that names who made the judgement. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `Assessment.value` | Required JSON value. Null is valid. Ravel refuses unknown assessment fields. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.name` | Required nonblank name, such as a path, URL or evidence label. It does not cause a read. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.sha256` | Optional digest of exactly 64 lowercase hexadecimal digits. At least this field or `version` is required. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.version` | Optional nonblank version string. At least this field or `sha256` is required. Ravel accepts both. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.bytes` | Optional nonnegative whole-number byte count. Ravel refuses unknown source-record fields. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.name` | Required canonical file path of the source that Ravel measured. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.via` | Required `"--context"`, `"readText"` or `"readJson"`. It identifies the read. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.bytes` | Required count of the bytes that Ravel actually read, before it parses any JSON. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.sha256` | Required lowercase SHA-256 digest of those bytes. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.name` | Required canonical file path that the host returns beside the content of a read. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.bytes` | Required actual byte count that the host returns. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.sha256` | Required lowercase SHA-256 of the bytes that the host read. The policy can put the three-field object in the attributed `sources`. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |

### Host operations

`SelectHost` supplies `diagnostic` and `signal`. `ContextHost` adds measured
reads, only during `loadContext`. If a loader retains its host, the host
still does not permit reads after the loader returns. Trusted policy can
still access the filesystem or the network directly, but these operations do
not measure that access. See [host operations](04-writing-an-owner-policy.md#supply-policy-context).

| Member | Behaviour and limits | Taught in |
|---|---|---|
| `diagnostic(text)` | Writes a string to the worker stderr and adds a final newline if there is none. It counts towards the combined `diagnosticsBytes`. It accepts no value that is not a string. | [Selection diagnostics](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `signal` | An `AbortSignal` that aborts when the Ravel CLI stops the worker with TERM, also at the selection deadline. Give it to cancellable work. After TERM, the worker still has one second to exit before the Ravel CLI kills it. | [Selection host](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `readText(path, maxBytes?)` | A synchronous read of a regular file, decoded as UTF-8. It returns `{ text, source }`. A nonempty path resolves from `request.cwd`, not from the policy file or the worker directory. | [Read policy sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `readJson(path, maxBytes?)` | The same read and measurement rules, then a JSON parse. It returns `{ value, source }`. The policy validates the returned data for its own needs. | [Read policy sources](04-writing-an-owner-policy.md#supply-policy-context) |

The default `maxBytes` is the smaller of 65,536 and `contextBytes`. An
explicit value is a positive safe whole number that is not larger than
`contextBytes`. These cases cause a refusal, even if the loader catches the
exception:

- a read above its limit
- a requested limit above the budget
- a read beyond the source count

For a missing or unreadable source, invalid UTF-8 or invalid JSON, the read
throws an exception. If that exception escapes the loader,
`context_source_unreadable` names the source. Only successful reads enter
the `measured` list. See [read bounds and failures](04-writing-an-owner-policy.md#supply-policy-context).

## SDK and module resolution

### `ravel/sdk`

`ravel/sdk` is the exact embedded module specifier. It exports the following
declarations. Type declarations help the typecheck, but the Ravel CLI
validates runtime values independently. Configure the editor and the
compiler with the installed `sdk/index.d.ts`. TypeScript path mappings do not
create runtime aliases. See
[typecheck public declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations).

| Export | Meaning | Taught in |
|---|---|---|
| `definePolicy(policy)` | Returns its `Policy` argument unchanged. It checks the fields of a literal and infers the parameter types of its methods. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `readChoice(directory, offered)` | Reads `.ravel-choice` in exactly that directory. It returns the offered names, `undefined` if there is no file, or a `Refused` value. Its rules come after this table. | [Offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice) |
| `CHOICE_FILE` | Literal `.ravel-choice`. It is not an input of the Ravel CLI or a rule for the discovery of configuration. | [Offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice) |
| `Policy` | The four-field version-2 interface of an owner policy. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `SelectionRequest` | The nine-field frozen caller request, with three optional fields. | [Read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `Limits` | The six effective numeric bounds that the policy can read. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `Selected` | The seven-field command and its selection labels. | [Select a command](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `Refused` | The four-field owner refusal from selection or from the loader. | [Refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `SelectionResult` | The union of `Selected` and `Refused`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `LoadContext` | The type of the loader function. It returns `Context` or `Refused`, synchronously or as a promise-like value. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `Context` | The six-field version-1 document. Only `schemaVersion` is required. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `DeliveredContext` | `Context` with the required `measured` list of read records. | [Delivered policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `Json` | The JSON data type for facts and assessment values. | [Policy-context data](04-writing-an-owner-policy.md#supply-policy-context) |
| `Assessment` | An attributed judgement with `by` and `value`. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord` | An attributed evidence name with a digest or version and an optional byte count. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource` | The four-field record of a measured read, with `via`. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource` | The three-field measurement that a host read returns beside its content. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `SelectHost` | The diagnostic operation and the abort signal for selection. | [Selection host](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `ContextHost` | The members of the selection host and the measured reads for the loader. | [Loader host](04-writing-an-owner-policy.md#supply-policy-context) |

`readChoice` accepts a regular UTF-8 file of 4,096 bytes or fewer. A missing
file returns `undefined`, and so does a dangling symlink. An empty file
returns an empty array. The helper splits the names at whitespace and retains
their order and duplicates.

A name outside `offered` returns `choice_unoffered`. An unreadable,
non-regular, oversized or invalid UTF-8 file returns `choice_unreadable`.
Return that refusal from the policy. Then Ravel shows it as `policy_refused`,
with the helper code in `policyCode`. See [choice-file validation](04-writing-an-owner-policy.md#offer-a-checkout-choice).

`directory` is the one directory that the helper reads. There is no search of
parent directories. Give `request.cwd` for a checkout choice. A relative
directory resolves from the working directory of the worker, which is `/` at
first. The policy decides the defaults, the number of names that it allows
and the meaning of each offered name.

The file adds no program, argument or label. This read is an ordinary SDK
filesystem read, not a measured host read. Thus, name the applied choice in
`reason`. See
[apply a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice).

### Import resolution

> **Table — Resolution bases:** The exact embedded specifiers select compiled
> modules. A file import resolves from the file that contains it, and other
> bare names use Bun's package lookup. Find the row for an import. It tells
> which copy the import loads and which directory is the base of the
> resolution.

| Name or input | Resolution and consequence | Taught in |
|---|---|---|
| `ravel/sdk` | The literal import selects the embedded SDK of the worker, even when a package of that name is also reachable. Edits to the installed source do not replace it. | [Embedded imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `ravel/examples/static` | The literal import selects the embedded static example. To use edits, import a copied file by its path. | [Embedded imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `./routes.ts` or another relative file import | Resolves from the file that contains the import. A policy symlink uses the location of its canonical real file. | [Relative imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Other bare specifiers, also other `ravel/*` names | Ordinary Bun package resolution from the file that contains the import, with `node_modules`. Ravel registers only the two exact names above. Ravel never installs missing dependencies automatically. | [Package imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `package.json` | The nearest file that reads as a package supplies `imports`, the name of the package and `exports`. The resolver skips files that it cannot parse and files that are not packages. Package `main` and `exports` resolution stays enabled. | [Package resolution](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Package `imports` alias with an embedded name as its target | It stays a package lookup and can load a package with the embedded name. The alias never reaches the embedded module. Use the literal specifier for the embedded identity. | [Alias limits](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Import without a file location | Resolves from the working directory of the worker, which is `/` before owner code runs. `/package.json` and `/node_modules` stay reachable inputs for the resolver. The parent directories of the temporary directory do not participate. | [Worker directory](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Dotenv, bunfig and tsconfig autoloading | The compiled worker disables them. Editor tsconfig aliases alone do not work at runtime. Package autoloading stays enabled. | [Startup controls](04-writing-an-owner-policy.md#resolve-policy-imports) |

Owner code can change the working directory of the worker or import any
code. A Bun `Worker` thread starts another runtime instance. The worker
disables dotenv autoloading, so that instance also does not load the dotenv
files that it finds. The disabled autoloading and the initial `/` working
directory control how the worker starts. They do not confine trusted policy.
See
[runtime boundaries](04-writing-an-owner-policy.md#resolve-policy-imports).

## Shipped sample

### `ravel/examples/static`

`ravel/examples/static` exports the following items. They are a base that
you can copy and edit. The kinds and effort names of the example are
conventions of this table, not Ravel's vocabulary. Replace its placeholder
program, provider and model before real use. See [use the starter table](04-writing-an-owner-policy.md#use-the-starter-route-table).

| Export | Behaviour | Taught in |
|---|---|---|
| `Command` | The type `Selected` without `status` and `reason`. The route supplies the program, the arguments and the labels. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `Route` | A function type from `SelectionRequest` to `Command`. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `agent(effort)` | Builds a route for `my-agent-wrapper`, with provider `your-provider` and model `your-model`. The route has model and effort arguments, and the unchanged prompt comes last. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `routes` | Exact table: `question` → `low`, `bugfix` → `medium`, `feature` → `high`, and `migration` and `architecture` → `xhigh`. There is no wildcard. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `selectRoute(routes, request)` | Finds an own property for the exact kind. A missing route returns the owner code `incomplete_mapping`. A match adds the selected status and a reason. The reason names the applied route, program, model and effort. | [Route selection](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `policy` | A version-2 starter that uses this table, with owner version `ravel/examples/static 2`. It has no `loadContext`. | [Starter policy](04-writing-an-owner-policy.md#use-the-starter-route-table) |

### `examples/sample.ts`

`libexec/ravel/examples/sample.ts` is the owner policy to copy, and its
source is `worker/sample/policy.ts`. It imports the embedded SDK and the
static table, and it uses the policy version `owner-choice-1`. It offers the
`.ravel-choice` names `default` and `careful` in `request.cwd`. With no file,
it chooses `default`, and an empty file or more than one name returns the
owner refusal `one_choice`. The sample returns the refusals
`choice_unreadable` and `choice_unoffered` of the helper unchanged. See [offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice).

> **Table — Sample choices:** `careful` replaces the static effort with exactly
> `high`, also when the static route chose `xhigh`. Both choices still refuse
> a missing route.

| Choice | Effect on the static selection |
|---|---|
| `default` | Returns the static selection unchanged. |
| `careful` | Does not change the program, model and provider. It sets the effort to exactly `high`, rebuilds the model, effort and prompt arguments, and uses the reason `careful choice`. |

Edit the copy for the arrangement that you want. Then validate each branch
with fake agents.
See [choice behaviour](04-writing-an-owner-policy.md#offer-a-checkout-choice).

The sample becomes owner configuration only when you copy it to the chosen
entry and edit it there. Ravel has no `init` command and installs no personal
policy. To edit the static table itself, copy `static.ts` too and import that
local file. An import of `ravel/examples/static` continues to use the
compiled copy. Validation with fake agents checks the delivery of arguments,
not the execution or permissions of a real agent. See
[copy a policy from another configuration](04-writing-an-owner-policy.md#copy-a-policy-from-another-configuration).

## Installed layout

`PREFIX` below is Ravel's installation root, independent of the owner's
`HOME`. For Homebrew, `brew --prefix ravel` gives this path.
Homebrew installs the Ravel CLI under `bin` and the worker and its
assets under `libexec/ravel`. These are the first nine paths in the table.

A release archive also carries `README.md` and `LICENSE` at its root. An
installed Ravel runs without Bun, Node.js or the source checkout on the host.
See [install Ravel and check its version](03-a-complete-session.md#install-ravel-and-check-its-version).

> **Note:** The contributor command `task install PREFIX=/path` builds the
> release pair from a checkout, installs the same nine paths and checks the
> installation. It is separate from the Homebrew installation in chapter 3.

| Path below `PREFIX` | Purpose and source relationship | Taught in |
|---|---|---|
| `bin/ravel` | The Ravel CLI, built in Rust from `crates/ravel`. It links the runner library in `crates/keyed-launch`. It checks the policy, supervises the session, decides the final outcome and writes the result. | [Installed pair](03-a-complete-session.md#install-ravel-and-check-its-version) |
| `libexec/ravel/ravel-policy` | The private compiled TypeScript worker, built from `crates/ravel/worker/src`. It embeds Bun, the SDK and the static example. It evaluates the policy and never receives a session-result context. | [Policy evaluation](03-a-complete-session.md#check-the-request-and-evaluate-the-policy) |
| `libexec/ravel/sdk/index.ts` | A copy of `worker/sdk/index.ts`, to read or copy. An edit to it does not change the embedded SDK. | [SDK and imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `libexec/ravel/sdk/index.d.ts` | Generated public SDK declarations for the typecheck of a policy. | [Installed declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations) |
| `libexec/ravel/examples/static.ts` | A copy of `worker/examples/static.ts`. To use edits, import a local copy by its path. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `libexec/ravel/examples/static.d.ts` | Generated declarations for the exports of the static example. | [Installed declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations) |
| `libexec/ravel/examples/sample.ts` | A copy of `worker/sample/policy.ts`. It is an owner entry that you can edit, and it uses checkout choices. Ravel does not copy it into `HOME` automatically. | [Copy a policy](04-writing-an-owner-policy.md#copy-a-policy-from-another-configuration) |
| `libexec/ravel/notices/NOTICES.md` | The inventory of runtime third-party notices, copied from `crates/ravel/notices`. Keep it with the worker. | [Installation assets](03-a-complete-session.md#install-ravel-and-check-its-version) |
| `libexec/ravel/notices/bun-LICENSE.md` | The bundled Bun licence and notice text, copied from `crates/ravel/notices`. | [Installation assets](03-a-complete-session.md#install-ravel-and-check-its-version) |
| `README.md` | An additional entry at the archive root that identifies the project. The release archive includes it beside the installed pair and assets. | [Obtain Ravel](03-a-complete-session.md#install-ravel-and-check-its-version) |
| `LICENSE` | The additional project licence at the archive root. | [Obtain Ravel](03-a-complete-session.md#install-ravel-and-check-its-version) |

Ravel finds `libexec/ravel/ravel-policy` relative to the canonical real path
of the Ravel CLI, also when the caller reaches `bin/ravel` through a symlink.
`PATH`, `argv[0]` and adjacent prefixes cannot choose a different worker. The
Ravel CLI checks protocol 1, the package version and the source build
identity before it sends the policy. Missing, stale and mismatched workers,
and test-only probe builds, fail with status 5. The build identity matches
the two binaries of one build, and it is not code signing. See
[check the installed pair](03-a-complete-session.md#check-the-request-and-evaluate-the-policy).

`crates/keyed-launch` is linked Rust library code, not an installed service
or a third executable. The Ravel CLI starts the worker for policy evaluation
and reaps it before the managed launch. Then it uses that library to
supervise the selected job. The worker embeds the executable SDK and example
modules. The installed source and declaration files support edits and the
typecheck separately.
See [the session's processes](03-a-complete-session.md#check-the-request-and-evaluate-the-policy).

> **Figure — Source responsibilities:** The runner and the code of the Ravel
> CLI that checks the request and decides the final outcome run in one
> process. The policy worker runs separately. In this C4 component view, the
> outer boundary groups the code inside the process of the Ravel CLI, and
> it is not a sandbox. Before the runner launches the managed process, the
> worker ends, and Ravel reaps it and removes its private directory.

![C4 component view: crates/ravel supplies the request checks and the decision about the final outcome inside the Ravel CLI. The Ravel CLI uses the linked runner from crates/keyed-launch. The Ravel CLI evaluates owner policy in a separate compiled worker from crates/ravel/worker/src, with Bun, the SDK and the static example embedded. Only after the reap of the worker and the removal of its private directory does the runner launch and supervise the selected managed process. The worker has no session-result context.](figures/source-components.svg)

> **Figure link:** [Open the source-responsibility view at full size](figures/source-components.svg).
> See [policy evaluation and supervision](02-actors-and-words.md#policy-evaluation-and-supervision)
> for the roles of the processes.

> **Figure — Installed files:** Keep the Ravel CLI and the worker from one
> build together. The editable sources and declarations beside them help
> policy authors, and they do not replace the embedded modules. In this UML
> deployment view, outlines group directories, not runtime boundaries, and a “file group” lists
> separate files, not an archive or a process. The dependency shows how the
> Ravel CLI finds and checks its worker, not concurrent process lifetimes.
> `PREFIX` is the installation directory, independent of the owner's `HOME`.

![UML deployment view: PREFIX contains bin/ravel, with the linked runner, and libexec/ravel/ravel-policy, with Bun and the embedded SDK and static example. The Ravel CLI finds the worker from its own canonical real path and checks the build identity. Beside the worker are the SDK source and declarations, the static-example source and declarations, the sample policy to copy and two notice files. An edit to these sources does not replace the embedded modules. An installed Ravel runs without Bun, Node.js or a source checkout on the host. A release archive also carries README.md and LICENSE at its root.](figures/installed-files.svg)

> **Figure link:** [Open the installed-file view at full size](figures/installed-files.svg).
> See [embedded imports and editable copies](04-writing-an-owner-policy.md#resolve-policy-imports)
> and [copy an owner policy](04-writing-an-owner-policy.md#copy-a-policy-from-another-configuration)
> for how to use the editable sources and declarations.

> **Note:** Chapter 10 stays unwritten until a supported extracted component
> exists. The installed layout is not a supported interface for another
> project. Native x86-64 Linux qualification is also still open. The book's
> [platform and operational limits](05-running-sessions.md#understand-the-operational-limits)
> distinguish native measurements from emulation.
