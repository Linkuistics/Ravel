# Reference

Use this chapter to look up the command line, exit statuses, streams,
environment rules, configuration and installed paths. A [caller](02-actors-and-words.md#caller-and-owner) starts
a [session](02-actors-and-words.md#caller-and-owner); the
[owner policy](02-actors-and-words.md#caller-and-owner) selects its
[managed process](02-actors-and-words.md#the-managed-process-and-its-result).
Only the caller gives meaning to the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result).
Each entry links to the chapter that teaches its use.

The entries describe the [checkout build used by this book](03-a-complete-session.md#obtain-the-build-used-by-this-chapter).

## Command line

### `ravel`, help and version

`ravel` takes one of two verbs: `start-session` or `end-session`. It has no
`help` subcommand. Invoking it without a verb prints help to stderr and exits
2. Explicit help requests exit 0 and print to stdout:
`ravel --help`, `ravel start-session --help` and `ravel end-session --help`.
`-h` is the short help option on all three command lines.

`ravel --version`, or `ravel -V`, prints the front's package version to stdout
and exits 0. The version option belongs to the top-level command, not to
either verb. Help and version output are informational output, not session
results. See [the first complete session](03-a-complete-session.md#run-the-first-session)
for the two verbs in use.

`--` ends option parsing. For `end-session`, place it before a result that
looks like an option: `ravel end-session -- '--help'` publishes the literal
value `--help` inside a live session. See [result arguments](#end-session-result).

### `start-session`

`ravel start-session --kind TEXT` also needs exactly one of `--prompt TEXT`
and `--prompt-file PATH`. It reads owner settings, admits and evaluates owner
policy, then supervises the selected program. A refusal launches nothing and
tries no alternative route. A successful invocation returns the exact
published result only after reap, confirmed process-group absence and required
cleanup. See [run a complete session](03-a-complete-session.md#run-the-first-session)
and [interpret the outcome](05-running-sessions.md#interpret-the-exit-status).

All fifteen options below belong after `start-session`. Only `--param`,
`--policy-env` and `--runtime-read` accept repeated occurrences; repeated
single-value options or switches are command-line errors. `-h` and `--help`
are also available on this verb.

### `--kind TEXT`

`--kind` is required and supplies `request.kind` unchanged. Its value is
nonempty UTF-8. Ravel has no catalogue of
[kinds](02-actors-and-words.md#caller-and-owner), derives none from a task
file, and gives the token no meaning. An absent or empty kind exits 2. See
[supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--prompt TEXT` and `--prompt-file PATH`

`--prompt` supplies the text directly; `--prompt-file` reads it once from a
path relative to the caller's working directory. Exactly one is required.
The prompt can be empty, contains no NUL, is valid UTF-8 and is at most
1,048,576 bytes (1 MiB). Ravel preserves its bytes, including trailing
newlines, in `request.prompt`. It adds no ending instruction. Policy decides
how to pass that text to its selected program.

An unreadable prompt file reports `prompt_unreadable`; invalid encoding, NUL
or excessive size reports `prompt_invalid`. Both exit 2. Missing both inputs
or supplying both is also status 2. Ravel does not implicitly read stdin and
refuses a prompt file that is a terminal. `--prompt` accepts a value beginning
with `-`. See [supply the prompt and caller data](05-running-sessions.md#supply-the-prompt-and-caller-data).

> **Note:** Shell command substitution strips trailing newlines before Ravel
> receives an argument. Use `--prompt-file` when those bytes matter. Operating
> systems can impose smaller argument limits, including when policy puts a
> file-supplied prompt into the selected command's arguments.

### `--task-file PATH` and `--task-id ID`

`--task-file` supplies optional `request.taskFile`, an absolute UTF-8 path
formed by joining the caller's working directory to the supplied path. It is
not canonicalised or read, and need not exist. It supplies no kind or task
identity.

`--task-id` supplies optional `request.taskId`, nonempty opaque UTF-8 of at
most 1,024 bytes. It accepts a value beginning with `-`. Ravel does not look
up a task record. Invalid input exits 2. See
[caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) for
both fields.

### `--config PATH`

`--config` selects a policy entry relative to the caller's working directory.
Without it, Ravel uses `$HOME/.config/ravel/policy.ts`, requiring an absolute
`HOME`. There is no repository search or environment-selected policy override.
The entry resolves to a readable regular file; its canonical path is UTF-8
and contains no `?`. A symlink selects its real file, whose directory supplies
the base for relative imports.

An empty option value is malformed input, status 2. Failure to admit the
entry is status 3. Selecting another entry does not move the settings file
from `$HOME/.config/ravel/settings.json`. See
[define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request)
and [resolve its imports](04-writing-an-owner-policy.md#resolve-policy-imports).

### `--param NAME=VALUE`

`--param` adds a string to `request.params` and is repeatable with different
names. The first `=` separates the name from the value. A name is nonempty
and contains no `=`; the value can be empty or contain further `=` characters.
The whole argument is UTF-8 and can begin with `-`. All names and values
together are limited to 65,536 bytes; separator characters do not count.
A repeated name, missing separator, invalid encoding or overflow exits 2.
Ravel gives names and values no meaning. See
[caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `--context PATH`

`--context` reads a regular file relative to the caller's working directory
as version-1 JSON [policy context](02-actors-and-words.md#three-different-kinds-of-information).
Ravel validates it and records its canonical name, byte count and SHA-256.
Without `loadContext`, the delivered document is this data with measured
sources attached. With a loader, it is input to the loader, which assembles
the document delivered to `select`. With neither a caller document nor a
loader, `select` receives `undefined` as its policy-context argument.

An unreadable, invalid or oversized document refuses with status 3; malformed
command-line syntax still uses 2. This option supplies data, not permission to
publish a session result. See [supply policy context](04-writing-an-owner-policy.md#supply-policy-context).

### `--timeout-ms MS`

`--timeout-ms` replaces the owner's `timeoutMs` setting for this invocation.
It takes decimal digits in the inclusive range 1,000–600,000 milliseconds.
With neither flag nor setting, the value is 30,000 milliseconds. Invalid
syntax or range exits 2.

Settings are validated before the flag replaces a value. A valid flag does
not rescue an invalid owner setting.

The deadline begins at worker spawn and covers worker startup, policy import,
policy-context loading and selection. Expiry stops selection, attempts worker
teardown and exits 124 with no managed launch. Cleanup time is additional;
the deadline is not a limit on the managed process's working time. See
[set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--context-bytes BYTES`

`--context-bytes` replaces the owner's `contextBytes` setting. It takes decimal
digits in the inclusive range 1–8,388,608 bytes. With neither flag nor setting,
the budget is 262,144 bytes. Invalid syntax or range exits 2.
Settings are validated first, so a valid flag cannot rescue an invalid setting.

The budget bounds the caller document and the encoded policy context delivered
to selection, including measured source records. It also caps a host source
read. Exceeding a bound refuses instead of truncating; `context_too_large`
uses status 3. See [policy context](04-writing-an-owner-policy.md#supply-policy-context)
and [bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--policy-env NAME`

`--policy-env` grants the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
one caller environment variable beyond its base set. Repeat it to grant more
names. The flags add to the owner's `policyEnv` array; duplicate names become
one grant in first-given order. A grant carries the caller's current value,
and an unset variable stays absent. This option does not grant a variable to
the managed process.

A name is nonempty UTF-8, contains no `=`, and cannot match a
[reserved grant class](#reserved-worker-grants). A malformed name reports
`malformed_input`; a reserved name reports `excluded_grant`; both exit 2.
Grant diagnostics show each name and whether it is set, never its value.
See [grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

### `--confine`

`--confine` restricts the managed process's filesystem access and environment,
using macOS `sandbox-exec` or Linux bubblewrap. It is off unless supplied.
Owner policy and its worker remain outside
[confinement](02-actors-and-words.md#trust-and-confinement). Ravel tests the
backend before policy evaluation and never falls back to an unconfined launch.

The canonical working directory and a private run directory are writable.
System runtime paths, the selected executable and the real Ravel executable
are admitted for reads. Network access is permitted. The confined process
starts a new POSIX session, receives EOF stdin and is not lent the caller's
controlling terminal. Its [environment](#managed-process-environment) is
restricted separately from the policy worker's.

Filesystem grants cannot overlap the policy directory or owner settings,
including through symlinks. Unusable inputs/backend report
`confinement_unusable`; overlaps report `confinement_overlap`; both exit 2.
See [confine the managed process](05-running-sessions.md#confine-the-managed-process)
for the layout and operational limits.

> **Note:** Native-entry behaviour has been measured on aarch64 macOS and
> aarch64 GNU/Linux. x86_64 GNU/Linux has been built and run under emulation
> only; native x86-64 qualification remains open. See the
> [support limits](05-running-sessions.md#understand-the-operational-limits).

### `--runtime-read FILE`

`--runtime-read` adds one read-only runtime file to a confined launch and is
repeatable. It requires `--confine`; omission of that switch exits 2. The path
resolves from the caller's working directory to an existing regular file
whose canonical name is UTF-8. It grants no directory tree. Read-only grants
remain read-only inside writable directories. Unusable or overlapping grants
exit 2 before policy evaluation. See
[confinement grants](05-running-sessions.md#confine-the-managed-process).

### `--exit-dir DIR`

`--exit-dir` allocates the private
[result channel](02-actors-and-words.md#the-managed-process-and-its-result)
beneath an existing writable directory, resolved from the caller's working
directory. It works with or without confinement. Without the flag, an
unconfined launch uses the system temporary directory; a confined launch uses
its private run directory.

With confinement, the supplied parent directory is also a writable grant.
The option chooses a parent, not a result filename or a persistent store.
An invalid parent can report `exit_dir_unusable` with status 2; a later failure
to allocate the channel reports `session_channel_unavailable` with status 5.
See [choose a channel parent](05-running-sessions.md#confine-the-managed-process).

### `--json`

`--json` replaces Ravel's text routing notice and failure document with
version-1 JSON on stderr. It leaves result stdout and managed-program output
unchanged. It is off unless supplied and belongs to `start-session`.
See [streams and notices](#streams-and-notices) for fields and emission order,
and [read notices](05-running-sessions.md#read-notices-without-confusing-them-with-results)
for their use.

> **Note:** Once parsing succeeds, a value such as `--prompt '--json'` is
> caller data, not a switch. If parsing fails, Ravel scans the argument words
> for an exact `--json` to choose the error format. Help, version and the help
> printed for a missing verb keep their ordinary display format.

### `end-session RESULT`

`ravel end-session RESULT` publishes one UTF-8 argument to the live enclosing
session identified by inherited `RAVEL_EXIT_FILE`. It accepts no session ID,
target or settings option and reads no policy or settings. `-h` and `--help`
display this verb's help; `--` permits an option-looking result.

`RESULT` can be empty, multiline or Unicode and is at most 1,048,576 bytes
(1 MiB). Quote it to keep it one argument. The first complete publication
wins; a repeated valid publication succeeds without replacing that value.
Success exits 0 and prints nothing. It means publication was accepted, not
that the enclosing job has finished or its result reached the caller.

Missing, empty, stale or invalid session-result context, an oversized value,
or publication I/O failure exits 1 with a text diagnostic on stderr.
Argument-count or encoding errors detected by the CLI exit 2. The operating
system can impose a smaller argument limit before the executable starts.
See [accept a result only after success](05-running-sessions.md#accept-a-result-only-after-success).

> **Note:** After detecting publication, Ravel gives a still-running job
> 2 seconds, sends TERM, waits up to 5 more seconds, then uses KILL if needed.
> If the direct child exits, remaining group members are stopped before reap
> without a fresh grace period. Code following `end-session` can be terminated.

## Exit statuses

For `start-session`, accept result stdout only when the final status is 0.
The child's exit status and an earlier successful `end-session` do not decide
the enclosing session's result. Cancellation takes precedence over an original
failure; an original failure keeps its status if later cleanup also fails.
Otherwise required cleanup failure prevents result delivery. All applicable
cleanup reports remain diagnostics. See
[outcome precedence](05-running-sessions.md#read-notices-without-confusing-them-with-results).

| Status | Meaning and next action | Teaching |
|---|---|---|
| `0` | `start-session` delivered a valid result after reap, confirmed group absence, required cleanup and successful write/flush. An empty result is valid; a child's later nonzero exit does not invalidate publication. Interpret the bytes using the caller's convention. `end-session` status 0 confirms only publication or an already valid first result. Explicit help/version also exit 0. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| `1` | `start-session` reports `session_result_missing`, even after child exit 0. `end-session` reports failed publication, including missing/empty/stale session-result context or an oversized value. Check that publication occurs inside a live session; absence is not an empty result. | [Publication and absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `2` | Malformed CLI input/settings or unusable/overlapping confinement inputs. Missing a verb prints help with this status. Correct the named input or setting before retrying. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |
| `3` | Policy refusal or unacceptable entry, behaviour, shape, result or policy context. No fallback route runs. Read code, stage and remedy; for `policy_refused`, also read `policyCode`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `5` | Worker/supervision failure, invalid result, required cleanup or terminal-recovery failure, surviving group, or result-output failure. Missing/stale/mismatched workers also use 5. Retain diagnostics, repair the installation or named resource, and discard any partial result bytes. | [Operational diagnostics](05-running-sessions.md#read-notices-without-confusing-them-with-results) |
| `124` | Policy evaluation timed out; no managed process launched. Inspect the policy's work before increasing `timeoutMs` or `--timeout-ms`. This is not a managed-process deadline. | [Bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `126` | The selected program is unexecutable, or launch failed before start was reported for a reason other than ENOENT. Check permissions, format, interpreter and execution environment; retain any recovery reports. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `127` | The selected program was not found, or pre-start launch failed with ENOENT. An existing script can have a missing interpreter. Check `PATH`, the selected path and interpreter. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `129` | SIGHUP cancellation, signal 1, during selection or supervision. No result is accepted. Retain cleanup reports and retry only if still intended. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `130` | SIGINT cancellation, signal 2, during selection or supervision. No result is accepted. A keyboard Ctrl-C sent only to the managed group is not necessarily cancellation of Ravel. | [Cancellation and terminal INT](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `143` | SIGTERM cancellation, signal 15, during selection or supervision. No result is accepted. Retain cleanup reports and retry only if termination was unintended. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |

The three signal numbers in the table use the shell convention, 128 plus the
signal. Ravel attempts cleanup and re-raises the selected signal; a process
API normally reports signal termination. If propagation fails, it reports the
failure and uses the numeric status as fallback. A process API caller handles
both. Ignored or blocked entry signals are not automatically cancellation.

> **Note:** SIGKILL or another abrupt death can bypass settlement. A newly
> arriving signal after the final cancellation decision can interrupt the
> result write. Neither case certifies cleanup or successful delivery; a final
> write failure can leave a prefix on stdout. See [operational limits](05-running-sessions.md#understand-the-operational-limits).

## Streams and notices

### Standard streams

| Stream | Session behaviour | Teaching |
|---|---|---|
| stdout | `start-session` writes only the exact successful result, without an added newline. Empty publication is zero bytes. A failed final write can leave a prefix, so check status. `end-session` writes no stdout. | [Accept a result](05-running-sessions.md#accept-a-result-only-after-success) |
| stderr | Managed stdout and stderr, Ravel notices and failures, captured policy diagnostics and operational reports. `--json` changes only Ravel's routing/failure documents. | [Read notices](05-running-sessions.md#read-notices-without-confusing-them-with-results) |
| stdin | Ravel does not implicitly consume a prompt from stdin. The worker receives null stdin. The unconfined managed program can use caller stdin; a confined program receives EOF. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [confinement](05-running-sessions.md#confine-the-managed-process) |

Text routing notices begin `ravel: starting provider` and name provider,
model, effort and kind before launch. Captured policy lines have `policy
stdout:` or `policy stderr:` prefixes. A failure begins `ravel: refused
(CODE, stage STAGE):`; the word “refused” in this text also covers operational
failures, distinguished by code and status. `end-session` publication failures
instead begin `ravel: end-session:`. See [read notices](05-running-sessions.md#read-notices-without-confusing-them-with-results).

### JSON routing notice

The `--json` routing notice is emitted after the supervision attempt, before
final settlement, when selection reached the point of preparing that notice.
It is not proof of launch, cleanup or success. Stderr remains a mixed stream,
and the managed program can print text that resembles a Ravel document. See
[read notices](05-running-sessions.md#read-notices-without-confusing-them-with-results).

| Field | Value and meaning |
|---|---|
| `schemaVersion` | `1`, the notice format version. |
| `selection` | Object with `provider`, `model`, `effort` and `reason`, all owner-supplied strings. These labels imply no model-provider integration. |
| `policy` | `path`, the admitted canonical entry, and `version`, the policy's declared version. |
| `worker` | `path`, `version` (package version), `buildId` and `bunVersion` of the matched compiled policy worker. Build identity pairs the front and worker; it is not code signing. |
| `selectionMs` | Elapsed policy evaluation time in milliseconds. |
| `context` | `null` when there is no delivered policy context; otherwise the report described below. It is never the session-result channel. |
| `bounds` | Objects named `selection`, `context`, `source`, `sources`, `message` and `diagnostics`. Each has its unit-valued field (`ms`, `bytes` or `sources`) and `from`, identifying `default`, `fixed`, `settings.json` or the overriding flag. |
| `policyEnvironment` | Array of `{name, set}` grants. `set` is a boolean reporting caller presence, never the value. Base environment variables are not implicitly added to this grant list. |
| `resolution` | String describing the resolved executable path and whether resolution used an absolute path, caller working directory or `PATH` entry. |
| `promptSource` | `argument` for `--prompt`, or the file path joined to the caller's working directory for `--prompt-file`. |
| `diagnostics` | Object with `stdout` and `stderr`, the captured policy streams as strings. Invalid UTF-8 bytes are shown with replacement characters. |
| `operationReports` | Present only when the runner returned its account of the job's ending; an array of retained report strings, empty when there are none. Absent if no such account was returned. A retained failed stop attempt does not alone determine the final status. |

The non-null `context` report has six fields. They describe the data delivered
to selection; [chapter 4's policy-context section](04-writing-an-owner-policy.md#supply-policy-context)
teaches their provenance.

| Policy-context report field | Meaning |
|---|---|
| `loader` | Whether the policy's `loadContext` produced the delivered document. |
| `sources` | Measured source records, each with canonical `name`, `via` (`--context`, `readText` or `readJson`), `bytes` and `sha256`. |
| `sourceBytes` | Sum of measured source byte counts. |
| `encodedBytes` | Byte length of the compact delivered JSON encoding, including measured records. |
| `sha256` | SHA-256 of that encoding. |
| `value` | The delivered policy-context document itself. |

> **Note:** The notice can contain policy-context data and captured policy
> output. The promise that grant diagnostics omit environment values does
> not prevent trusted policy code from printing a value itself.

### JSON failure document

A JSON failure document has `schemaVersion: 1`, an `error` object, and optional
top-level `diagnostics` containing policy `stdout` and `stderr` strings. Read
`code` and `stage` together and retain the complete stderr stream: secondary
operational reports can appear outside this document. See
[read failure notices](05-running-sessions.md#read-notices-without-confusing-them-with-results).

| `error` field | Presence and meaning |
|---|---|
| `code` | Required stable Ravel diagnostic code, such as `malformed_input` or `session_result_missing`. |
| `stage` | Required stage string from the table below. |
| `message` | Required explanation; can retain original and secondary operational details. |
| `remedy` | Required suggested correction or recovery action. |
| `exit` | Required numeric status, including the numeric representation of cancellation. |
| `input` | Optional caller input involved, such as a flag. |
| `source` | Optional file associated with the failure, commonly the policy entry. |
| `location` | Optional location inside the source/data, such as `policy.select` or `result.args[2]`. |
| `policyCode` | Optional owner-supplied refusal code alongside Ravel's `policy_refused`. |
| `bound` | Optional exhausted bound: `name`, the unit-valued field and `from`. |
| `signal` | Optional `SIGINT`, `SIGTERM` or `SIGHUP`, naming cancellation. |

The stage says where Ravel detected a problem, not which process ultimately
caused it. For example, `evaluation` is used for the whole deadline because
the front cannot see which policy operation was running at expiry.

| Stage | Scope | Teaching |
|---|---|---|
| `cli` | Command line and owner settings. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `authority` | Choosing and opening the policy entry or obtaining the caller's working directory. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `worker` | Finding, starting or verifying the compiled worker and its protocol. | [Admit and evaluate](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy) |
| `evaluation` | The worker's evaluation as a whole, including deadline and selection cancellation. | [Bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names), [cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `load` | Importing the policy entry. | [Resolve imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `context` | Reading, assembling, validating and measuring policy context. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `validation` | Checking the exported policy object. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `selection` | Calling `select` and judging its returned data. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `resolution` | Finding the selected program. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `exec` | Preparing, supervising and settling the managed launch, including output/channel failures before spawn. | [Results](05-running-sessions.md#accept-a-result-only-after-success), [notices](05-running-sessions.md#read-notices-without-confusing-them-with-results) |

## Environment variables

### Caller inputs and worker environment

The front starts the policy worker with a fresh environment: caller values
for `HOME`, `PATH`, `TMPDIR`, `LANG` and every `LC_*` name that is set, plus
explicit grants, then Ravel's own cache setting. Other caller variables do not
reach it without a grant. See [grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Variable or class | Meaning and handling | Teaching |
|---|---|---|
| `HOME` | Locates the default policy and settings. The default policy needs an absolute value; with explicit `--config`, absent or relative `HOME` yields no owner settings. Passed to the worker when set. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `PATH` | The front resolves a bare selected program using the caller's value. Unset means no search; an empty value is a working-directory entry. Relative entries resolve from the caller's working directory. The worker receives this value too, but it does not select the installed worker. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `TMPDIR` | Feeds the system temporary-directory choice used by the front for private worker/run directories and an unconfined channel without `--exit-dir`. Its caller value reaches the worker when set. | [Channel placement](05-running-sessions.md#confine-the-managed-process), [worker environment](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `LANG`, `LC_*` | Caller locale values passed to the worker when set. `LC_*` means every name with that exact case-sensitive prefix. | [Worker base environment](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| Explicitly granted names | `policyEnv` settings plus `--policy-env` flags, with caller values and duplicate names merged. Unset names remain absent. | [Grant names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `BUN_RUNTIME_TRANSPILER_CACHE_PATH` | Set by Ravel to `0` for the worker, disabling its runtime transpiler cache. Caller values cannot override it through a grant. | [Resolve policy imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `XDG_CONFIG_HOME` | Not consulted for policy or settings discovery. Its presence does not move `$HOME/.config/ravel`. It has no special grant exclusion. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |

### Reserved worker grants

These nine exact names or prefix classes are refused in both `policyEnv` and
`--policy-env`. Matching is case-sensitive. `*` denotes any suffix, including
an empty suffix; it is not shell expansion. The refusal is `excluded_grant`
with status 2. These are worker-grant rules; they do not describe blanket
scrubbing of the unconfined managed process. See
[grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Name or class | Why the worker does not accept a caller grant |
|---|---|
| `BUN_*` | Bun startup/runtime controls, including preloading and executable mode; Ravel sets its own cache control. |
| `NODE_OPTIONS` | Runtime/code-loading options. |
| `NODE_PATH` | Module search path. |
| `NODE_PRESERVE_SYMLINKS` | Resolver behaviour. |
| `NODE_CHANNEL_*` | Runtime IPC-descriptor controls. |
| `LD_*` | Dynamic-loader controls. |
| `DYLD_*` | macOS dynamic-loader controls. |
| `RAVEL_*` | Ravel's reserved controls, including the result-channel name. |
| `HARNESS_DISPATCH_*` | Legacy dispatch controls. |

### Managed-process environment

An unconfined managed process inherits the caller's environment except for
the six exact scrubbed names below. Ravel then supplies its fresh
`RAVEL_EXIT_FILE`. Other names, including arbitrary application credentials
and names in worker-excluded classes, remain inherited. A worker grant is
irrelevant to that inheritance. See
[caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

| Name or class | Confined managed process | Teaching |
|---|---|---|
| `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG`, `LC_*` | Caller values retained when present. Retaining `HOME` does not grant filesystem access to it. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `TMPDIR`, `TMP`, `TEMP` | All set to the private run's temporary directory, replacing caller values. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |
| `RAVEL_EXIT_FILE` | Fresh path for this session, supplied after the environment grants. | [Nested sessions](05-running-sessions.md#keep-nested-sessions-separate) |
| Every other caller variable | Absent; `--policy-env` grants only the worker and cannot add it here. | [Confined environment](05-running-sessions.md#confine-the-managed-process) |

The scrub set is exactly `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID`, `RAVEL_STATE_DIR`,
`HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`. The first is replaced with the new channel path;
the other five stay absent. These obsolete identity/storage names create no
store and select no session. See [caller environment](05-running-sessions.md#supply-the-prompt-and-caller-data).

### `RAVEL_EXIT_FILE`

`RAVEL_EXIT_FILE` is the inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information):
the path `end-session` uses to publish to its enclosing session. The front
sets it only for the managed launch. The policy worker never receives it.
Nested launches replace it only for their own managed process, leaving the
outer caller's value unchanged.

An unset, empty or stale path makes `end-session` fail with status 1. Do not
use it as a persistent result location or a cross-session address. The channel
isolates ordinary sessions but does not authenticate against hostile code
running as the same user. See [nested sessions](05-running-sessions.md#keep-nested-sessions-separate)
and [operational limits](05-running-sessions.md#understand-the-operational-limits).

## Policy entry and settings

### `policy.ts`

`$HOME/.config/ravel/policy.ts` is the default owner entry. `HOME` must be
absolute for this default; `--config PATH` chooses another entry relative to
the caller's working directory. There is no repository search, environment
override or fallback policy. The entry must resolve to a readable regular
file with a UTF-8 canonical path containing no `?`. A symlink uses its real
file, including that file's directory for relative imports. Admission failure
exits 3 before owner code runs. See [define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request)
and [resolve imports](04-writing-an-owner-policy.md#resolve-policy-imports).

`policy` is the named export Ravel evaluates. A default export alone is
insufficient. The export is a plain object, not an array or class instance,
with the [four policy fields](#policy-object) below. Trusted owner code runs
outside managed-process confinement, in a separate worker without
session-result authority. Successful selection requires confirmed worker reap
and private-directory cleanup before launch. See [policy admission](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy).

### `settings.json`

`$HOME/.config/ravel/settings.json` supplies owner defaults for `start-session`.
`--config` does not relocate it and `XDG_CONFIG_HOME` does not select it.
A missing file, or absent/relative `HOME`, supplies no settings; an empty
JSON object also supplies none. An existing file must be a readable regular
file containing a JSON object. `end-session` reads neither settings nor
policy. See [set bounds and grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Key | Accepted value, default and consequence | Teaching |
|---|---|---|
| `timeoutMs` | Optional whole-number milliseconds, 1,000–600,000 inclusive; default 30,000. `--timeout-ms` replaces the value after settings validation. Bounds worker startup, import, loading and selection, not the managed process. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `contextBytes` | Optional whole-number bytes, 1–8,388,608 inclusive; default 262,144. `--context-bytes` replaces it after validation. Caps the caller document and delivered policy context, and sets the ceiling for a host read. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `policyEnv` | Optional array of environment-variable names; default no extra grants. Names are nonempty UTF-8 without `=` and cannot match a reserved grant class. Flags add names; duplicates collapse and unset values stay absent. Grants affect the worker, not managed-process inheritance. | [Grant environment names](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

Unknown keys, including obsolete `stateDir` and a putative `config` key,
report `settings_invalid`, status 2. Unreadable/non-regular files, invalid
JSON, non-object documents and a `policyEnv` other than an array of strings
use the same code. Invalid numeric bounds or malformed grant names use
`malformed_input`; excluded grants use `excluded_grant`, both status 2.
The whole file is validated before flags replace values, so a valid flag
cannot rescue a bad setting. No setting opens persistent storage. See
[settings validation](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Bounds

`request.limits` reports six effective bounds. These are frozen values for
policy to read; the front and worker enforce their own copies. A policy
cannot raise them. Overflow refuses selection rather than admitting truncated
data. The JSON notice names each bound separately and records its origin.
See [set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

| Request field / notice name | Default or fixed value | Adjustment and consequence | Teaching |
|---|---:|---|---|
| `selectionMs` / `selection` | 30,000 ms | `timeoutMs`, then `--timeout-ms`, within 1,000–600,000. Measured from worker spawn through startup, import, loader and selection. Expiry is `selection_timeout`, status 124; teardown time is additional. | [Selection deadline](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `contextBytes` / `context` | 262,144 bytes | `contextBytes`, then `--context-bytes`, within 1–8,388,608. Both the caller file and compact delivered JSON including `measured` must fit; `context_too_large`, status 3. | [Policy-context budget](04-writing-an-owner-policy.md#supply-policy-context) |
| `sourceBytes` / `source` | Smaller of 65,536 bytes and `contextBytes` | A host read's positive whole-number `maxBytes` can replace the default up to `contextBytes`. Excess bytes or an excessive requested limit cause `source_too_large`, status 3, even if the loader catches the exception. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `sources` / `sources` | Fixed 256 | Applies separately to measured reads (including the caller document) and to the document's attributed `sources` array. Exceeding either is `too_many_sources`, status 3; a caught read-bound exception still refuses. | [Measured and attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `messageBytes` / `message` | Fixed 1,048,576 bytes | Bounds a policy snapshot, selection result or failure report in the private protocol. Oversized messages use `message_too_large`, status 3. A delivered document instead has the policy-context budget. | [Protocol bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `diagnosticsBytes` / `diagnostics` | Fixed 262,144 bytes | Combined worker stdout/stderr. Excess produces `output_limit`, status 3; Ravel retains the first 262,144 bytes and discards the rest while stopping the worker. | [Output bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

`from` in a reported bound is `default`, `fixed`, `settings.json`, or the
flag that set it. A source limit lowered by the policy-context budget inherits
that budget's origin; a read's own limit is reported as `maxBytes` in a
source-bound refusal. The report uses `ms`, `bytes` or `sources` for its
numeric field. See [bounds and their inputs](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Policy and request

### Policy object

`policy` has the following fields. Unknown fields refuse; `definePolicy`
helps TypeScript check the object but does not replace runtime validation.
See [define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request).

| Field | Contract | Teaching |
|---|---|---|
| `schemaVersion` | Required literal number `2`. Other whole-number versions report `unsupported_version`; missing or malformed values report `policy_invalid`, status 3. Version 1's catalogue is not converted automatically. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `version` | Required nonblank owner-maintained string. Reported as the policy version; independent of Ravel's package version. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `select(request, context, host)` | Required function. Returns `Selected` or `Refused`, directly or through a promise. Receives delivered policy context or `undefined`, plus diagnostic/signal operations. No fallback runs for absence, malformed output, exceptions or an unsettled promise. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `loadContext(request, host)` | Optional function. Returns version-1 `Context` or `Refused`, directly or through a promise, before `select`. Its failure prevents selection. Its host supplies measured reads. Omission leaves the caller's document, if any, as the delivered data. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |

### `SelectionRequest`

`SelectionRequest` is frozen, including its nested objects and arrays.
Optional fields are absent when the caller did not supply them; they are not
defaulted to empty strings. It carries data and limits, never a result-channel
path. See [read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request).

| Field | Value and limits | Teaching |
|---|---|---|
| `schemaVersion` | Always the number `2`, independently of policy-context schema 1. | [Read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `kind` | Required nonempty caller token, unchanged from `--kind`; no Ravel-defined catalogue. | [Caller inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt` | Required UTF-8 string, possibly empty, at most 1,048,576 bytes, without NUL. Caller bytes and newlines remain unchanged; policy chooses how to pass them on. | [Prompt delivery](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `cwd` | Caller working directory as data. Relative input paths and selected program paths resolve from it; the worker runs owner code initially from `/`. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `params` | String values by caller-supplied name; `{}` if none. Names are nonempty without `=`, duplicate names refuse, and combined names/values are at most 65,536 bytes. | [Caller parameters](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskFile` | Optional absolute UTF-8 path joined from `--task-file`; not canonicalised, read or required to exist. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `taskId` | Optional nonempty opaque UTF-8 string, at most 1,024 bytes. Names no Ravel record. | [Caller data](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `context` | Optional validated caller version-1 policy-context document, before Ravel attaches `measured`. A loader sees this input even if it returns different data for `select`. | [Caller and delivered documents](04-writing-an-owner-policy.md#supply-policy-context) |
| `limits` | Required object with the six effective fields in [Bounds](#bounds); no policy-side override. | [Effective bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |

## Selection and refusal

### `Selected`

`Selected` is the command returned by `select`. All seven fields are required;
unknown fields refuse. Ravel validates shape, then resolves the program. It
does not check that the prompt appears in the arguments or that the labels
describe the actual program. See [select a command](04-writing-an-owner-policy.md#select-a-command-or-refuse).

| Field | Accepted value and effect | Teaching |
|---|---|---|
| `status` | Literal `"selected"`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `program` | Nonblank string without NUL. Absolute path, caller-relative path containing `/`, or bare name searched on caller `PATH`. Also supplies `argv[0]`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `args` | Array of strings after `program`; empty array and empty strings are accepted, NUL is not. Each string is one argument: no splitting, shell expansion or template interpretation. | [Construct arguments](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `provider` | Nonblank owner label for model origin; reported as given, not inferred or used to contact a provider. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `model` | Nonblank owner label; need not occur in `args` and does not configure the harness by itself. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `effort` | Nonblank owner label; Ravel defines no effort levels and does not turn this label into a flag. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `reason` | Nonblank owner explanation reported verbatim in routing diagnostics. | [Selection labels](04-writing-an-owner-policy.md#select-a-command-or-refuse) |

### `Refused`

`Refused` declines the request from either `select` or `loadContext`. All four
fields are required and unknown fields refuse. It launches nothing and tries
no other route. A valid refusal reports Ravel's `policy_refused`, status 3,
with the owner's code beside it as `policyCode`; the stage is `selection` or
`context` respectively. See [return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse)
and [refuse from a loader](04-writing-an-owner-policy.md#supply-policy-context).

| Field | Accepted value and effect | Teaching |
|---|---|---|
| `status` | Literal `"refused"`. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `code` | Nonblank owner-defined string. Separate from Ravel's diagnostic-code namespace. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `message` | Nonblank explanation of what prevents selection. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `remedy` | Nonblank action the caller or owner can take. | [Return a refusal](04-writing-an-owner-policy.md#select-a-command-or-refuse) |

### Diagnostic codes

`error.code`, `error.stage` and `error.exit` identify a Ravel failure in a
JSON failure document. These tables give the ordinary classification of each
code. Cancellation takes precedence over an existing failure; otherwise the
original error stays primary and later cleanup errors remain reports. A code
listed here may therefore appear as retained detail beside another outcome.
Read the complete stderr stream. See [interpret notices](05-running-sessions.md#read-notices-without-confusing-them-with-results).

| Code | Stage; status | Meaning and remedy | Teaching |
|---|---|---|---|
| `malformed_input` | `cli`; 2 | Invalid CLI input, numeric setting or grant name. Correct the named input and range; a flag cannot rescue invalid settings. | [Inputs](05-running-sessions.md#supply-the-prompt-and-caller-data), [settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `prompt_unreadable` | `cli`; 2 | Prompt file cannot be opened/read or is a terminal. Supply a readable non-terminal file or `--prompt`. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `prompt_invalid` | `cli`; 2 | Prompt is invalid UTF-8, contains NUL or exceeds 1 MiB. Correct the encoding/content or shorten it. | [Prompt inputs](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `settings_invalid` | `cli`; 2 | Settings file or shape is invalid, including unknown keys. Correct it to the three optional keys or remove it for defaults. | [Settings](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `excluded_grant` | `cli`; 2 | A settings/flag grant names a reserved variable or prefix. Remove the grant; it cannot override worker startup controls. | [Grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `confinement_unusable` | `cli`; 2 | Backend or confinement inputs cannot be used. Correct the named backend/path; no unconfined fallback runs. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `confinement_overlap` | `cli`; 2 | A grant overlaps the policy directory or settings. Separate the paths, including their canonical symlink targets. | [Confinement](05-running-sessions.md#confine-the-managed-process) |
| `exit_dir_unusable` | `cli`; 2 | Channel parent is unusable. Name an existing writable directory or omit `--exit-dir`. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `cwd_unavailable` | `authority`; 3 | Caller directory cannot be obtained. Run from an existing readable directory. | [Caller directory](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `home_unset` | `authority`; 3 | Default entry has no absolute `HOME`. Set an absolute `HOME` or choose `--config`. | [Policy entry](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `policy_missing` | `authority`; 3 | Chosen entry does not exist. Create it or correct `--config`; no search follows. | [Policy entry](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `policy_unreadable` | `authority`; 3 | Entry is unreadable/non-regular or its canonical name cannot be imported exactly. Correct the file, permissions or UTF-8 path without `?`. | [Policy entry and imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `worker_missing` | `worker`; 5 | The canonical front's worker cannot be located or started. Reinstall/rebuild the matched pair and check execution permissions. | [Installed pair](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy) |
| `worker_identity_mismatch` | `worker`; 5 | Protocol, package version or source identity does not match the front. Install both binaries from one build; probe workers are never admitted. | [Installed pair](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy) |
| `worker_failed` | `worker`; 5 | Worker preparation, execution or required teardown failed. Inspect retained operation details and repair the installation or named resource. | [Operational failures](05-running-sessions.md#read-notices-without-confusing-them-with-results) |
| `protocol_error` | `worker`; 5 | Worker conversation is malformed, closed unexpectedly or otherwise invalid. Rebuild/reinstall the matching pair; retain diagnostics if it recurs. | [Operational failures](05-running-sessions.md#read-notices-without-confusing-them-with-results) |
| `selection_timeout` | `evaluation`; 124 | Whole worker deadline expired, including startup/import/loading. Fix stalled work or raise the adjustable bound within its range. No managed launch follows. | [Deadline](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `selection_cancelled` | `evaluation`; 129/130/143 | HUP/INT/TERM ended selection. Retain teardown reports; retry only if still intended. Signal termination is the normal process-level result. | [Selection cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `output_limit` | `evaluation`; 3 | Worker stdout/stderr together exceed 256 KiB. Print less; the retained prefix is diagnostic evidence, not an accepted selection. | [Output bound](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `policy_import_failed` | `load`; 3 | Entry/import throws, rejects or has an unsettled await with no work left to settle it. Fix the import or await and install dependencies explicitly. | [Imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `policy_invalid` | `validation`; 3 | Named export is missing, not plain, unserialisable or has invalid fields. Return the version-2 policy object with a callable `select`. | [Policy shape](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `unsupported_version` | `validation` or `context`; 3 | Policy schema is not 2 or policy-context schema is not 1. Use the supported shape or a compatible Ravel; a version-1 policy needs rewriting, not just a changed number. | [Policy shape](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request), [policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `message_too_large` | `load`, `context` or `selection`; 3 | A policy snapshot, result or failure report exceeds the fixed 1 MiB protocol bound. Shorten the corresponding data/report; this includes an oversized loader failure report at `context`. | [Protocol bound](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `context_unreadable` | `context`; 3 | Caller document cannot be read as a regular file with a canonical UTF-8 path. Correct `--context` or omit it. | [Caller policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_invalid` | `context`; 3 | Caller/loader document is invalid JSON or has invalid fields/data; loader returned nothing, unserialisable data or a malformed refusal. Correct the named location to version-1 data or a valid refusal. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_too_large` | `context`; 3 | Caller bytes or delivered JSON including measurements exceed the budget. Deliver less or increase `contextBytes`/`--context-bytes` within its range. | [Policy-context budget](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_loader_failed` | `context`; 3 | Loader throws or rejects. Fix its computation or supply what it requires; `select` will not run without the required data. | [Loader failures](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_source_unreadable` | `context`; 3 | A host read's missing/unreadable file, bad UTF-8 or bad JSON caused loader failure. Supply/correct that source or change the loader's requirement. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `context_loader_unsettled` | `context`; 3 | Loader promise remains pending with nothing left to settle it. Make every path resolve or reject. Active work that instead exhausts the deadline uses `selection_timeout`. | [Loader failures](04-writing-an-owner-policy.md#supply-policy-context) |
| `source_too_large` | `context`; 3 | Host source exceeds its read limit, or requested `maxBytes` exceeds the policy-context budget. Read less or choose an allowed limit; catching the breach does not permit selection. | [Host read bounds](04-writing-an-owner-policy.md#supply-policy-context) |
| `too_many_sources` | `context`; 3 | Measured or attributed source count exceeds 256. Combine sources or read/attribute fewer. | [Source ledgers](04-writing-an-owner-policy.md#supply-policy-context) |
| `policy_refused` | `selection` or `context`; 3 | `select` or the loader returned a valid refusal. Follow its owner-supplied `policyCode`, message and remedy; no alternative route runs. | [Refusals](04-writing-an-owner-policy.md#select-a-command-or-refuse), [loader refusals](04-writing-an-owner-policy.md#supply-policy-context) |
| `selection_abstained` | `selection`; 3 | `select` returned `undefined` or `null`. Return a selected command or explicit refusal on every path. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_malformed` | `selection`; 3 | Result has the wrong shape, fields, types, NUL or cannot be serialised. Correct the named location to `Selected` or `Refused`. | [Selection shape](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_threw` | `selection`; 3 | `select` threw or rejected. Fix the failure; decline with a `Refused` value when intentional. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `selection_unsettled` | `selection`; 3 | Promise remains pending with no work left to settle it. Make every path resolve or reject; active work hitting its deadline instead uses `selection_timeout`. | [Selection failures](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `program_not_found` | `resolution`; 127 | Selected file/name cannot be found in the caller's execution context. Correct the path or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |
| `program_unexecutable` | `resolution`; 126 | Selected file is not executable/regular or caller `PATH` cannot be searched as required. Correct permissions, file or `PATH`. | [Program resolution](05-running-sessions.md#supply-the-prompt-and-caller-data) |

| Code | Stage; status | Meaning and remedy | Teaching |
|---|---|---|---|
| `cancellation_unavailable` | `exec`; 5 | Front cannot establish cancellation handling. Inspect the reported OS error and execution environment. | [Operational failures](05-running-sessions.md#read-notices-without-confusing-them-with-results) |
| `signal_state_unavailable` | `exec`; 5 | Entry capture or a required signal-state transition failed. Retain all reports and correct the execution environment; no result certifies success. | [Signal and recovery limits](05-running-sessions.md#understand-the-operational-limits) |
| `output_unavailable` | `exec`; 5 | Result stdout cannot be reserved or restored. Supply a writable output stream and inspect any restoration error. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_channel_unavailable` | `exec`; 5 | Private result channel cannot be allocated/prepared. Correct temporary storage or `--exit-dir`; inspect partial-cleanup reports. | [Channel placement](05-running-sessions.md#confine-the-managed-process) |
| `supervision_failed` | `exec`; 127, 126 or 5 | Before start is reported: ENOENT gives 127, other failures 126. After start: 5. Check executable/interpreter, execution environment and retained runner/terminal reports. | [Launch and supervision](05-running-sessions.md#interpret-the-exit-status) |
| `session_result_missing` | `exec`; 1 | Managed process ended without publishing, even if its own status was 0. Arrange an `end-session` call for successful work. | [Publication and absence](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_result_invalid` | `exec`; 5 | Published channel entry or result bytes cannot be accepted, including invalid encoding, type or size. Use `end-session` inside the live session and keep the result within 1 MiB. | [Result validity](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cleanup_failed` | `exec`; 5 | Required channel/private-directory cleanup failed. Inspect the named path and ownership report before removing any residue; no candidate result is returned. | [Cleanup limits](05-running-sessions.md#understand-the-operational-limits) |
| `process_group_survived` | `exec`; 5 | Group absence was not established at settlement. Inspect the reported group and cleanup failures before retrying; publication alone is insufficient. | [Cleanup and success](05-running-sessions.md#accept-a-result-only-after-success) |
| `session_cancelled` | `exec`; 129/130/143 | HUP/INT/TERM wins at final settlement, even after publication. Discard the result and retain original/cleanup errors; retry only if intended. | [Cancellation](05-running-sessions.md#cancel-during-selection-or-supervision) |
| `result_write_failed` | `exec`; 5 | Final result write/flush failed after settlement. Fix the output destination and discard any prefix already written. | [Result output](05-running-sessions.md#accept-a-result-only-after-success) |

`end-session` publication failures are plain `ravel: end-session:` messages,
status 1, rather than entries in the JSON code table. Missing, empty or stale
`RAVEL_EXIT_FILE`, an oversized argument or a failed publication do not mean
an empty result. See [publish and accept results](05-running-sessions.md#accept-a-result-only-after-success).

## Policy context and host operations

### `Context` and `DeliveredContext`

`Context` is version-1 JSON data, supplied by `--context` or returned by
`loadContext`. Absent optional fields mean unknown, distinct from a known
empty object/array/string. Unknown top-level fields refuse, including
`program`, `args`, `argv`, `command`, `select`, `loadContext`, `env`,
`environment`, `exec`, `shell` and `script`. Those names inside free-form JSON
facts remain data. See [supply policy context](04-writing-an-owner-policy.md#supply-policy-context).

| Field | Accepted value | Teaching |
|---|---|---|
| `schemaVersion` | Required number `1`; independent of the version-2 request/policy contract. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `summary` | Optional string, including empty. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `acceptanceCriteria` | Optional array of strings; both array and strings may be empty. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `facts` | Optional object mapping names to JSON values. No executable authority follows from a fact. | [Policy-context shape](04-writing-an-owner-policy.md#supply-policy-context) |
| `assessments` | Optional object mapping names to attributed `{ by, value }` objects. | [Attributed judgements](04-writing-an-owner-policy.md#supply-policy-context) |
| `sources` | Optional array of at most 256 attributed source records. Separate from measured reads; supplying a record does not read a file. | [Source attribution](04-writing-an-owner-policy.md#supply-policy-context) |

`DeliveredContext` adds required `measured` to the accepted document and is
deeply frozen. Neither caller nor loader supplies that field. Without a
loader, selection receives the caller document plus measurements, or
`undefined` if there is no document. A loader replaces the document delivered
to `select`; `request.context` still names the caller's original input and
there is no automatic merge. A loader returning nothing refuses. See
[caller and delivered documents](04-writing-an-owner-policy.md#supply-policy-context).

`measured` holds successful reads in order: the caller's `--context` document
first when present, then host reads, including repeat reads of one file.
Its separate limit is 256 records. The compact encoding of the entire
delivered document, including this ledger, must fit `contextBytes`. Source
attribution and measurement authenticate neither an author nor a claim. See
[measurement and its limits](04-writing-an-owner-policy.md#supply-policy-context).

### Nested data and measured records

`Json` admits null, booleans, finite numbers, strings, arrays and objects of
JSON values. Cycles and executable/non-JSON values cannot be delivered. An
object whose only key is `$ravel` is reserved even inside facts or assessment
values and refuses as `context_invalid`. See [JSON policy data](04-writing-an-owner-policy.md#supply-policy-context).

| Type and field | Accepted value or meaning | Teaching |
|---|---|---|
| `Assessment.by` | Required nonblank string naming who made the judgement. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `Assessment.value` | Required JSON value; null is valid. Unknown assessment fields refuse. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.name` | Required nonblank name, such as a path, URL or evidence label. Does not cause a read. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.sha256` | Optional digest, exactly 64 lowercase hexadecimal digits. At least this field or `version` is required. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.version` | Optional nonblank version string. At least this field or `sha256` is required; both are accepted. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord.bytes` | Optional nonnegative whole-number byte count. Unknown source-record fields refuse. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.name` | Required canonical file path, measured by Ravel. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.via` | Required `"--context"`, `"readText"` or `"readJson"`, identifying the read. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.bytes` | Required actual bytes read, before any JSON parsing. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource.sha256` | Required lowercase SHA-256 digest of those bytes. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.name` | Required canonical file path returned beside a host read's content. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.bytes` | Required actual byte count returned by the host. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource.sha256` | Required lowercase SHA-256 of the bytes read. The three-field object can be placed in attributed `sources`. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |

### Host operations

`SelectHost` supplies `diagnostic` and `signal`; `ContextHost` adds measured
reads during `loadContext` only. Retaining a loader host does not permit reads
after the loader returns. Direct trusted-policy filesystem/network access
remains possible but is not measured by these operations. See [host operations](04-writing-an-owner-policy.md#supply-policy-context).

| Member | Behaviour and limits | Teaching |
|---|---|---|
| `diagnostic(text)` | Writes a string to worker stderr, adding a final newline if absent. Counts towards combined `diagnosticsBytes`. Accepts no non-string value. | [Selection diagnostics](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `signal` | `AbortSignal` aborted when the front stops the worker with TERM, including at the selection deadline. Pass it to cancellable work; it does not extend teardown grace. | [Selection host](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `readText(path, maxBytes?)` | Synchronous regular-file read, UTF-8 decoded, returning `{ text, source }`. Nonempty path resolves from `request.cwd`, not the policy file or worker directory. | [Read policy sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `readJson(path, maxBytes?)` | Same read/measurement rules, then JSON parsing, returning `{ value, source }`. Policy validates the returned data for its own needs. | [Read policy sources](04-writing-an-owner-policy.md#supply-policy-context) |

`maxBytes` defaults to the smaller of 65,536 and `contextBytes`. An explicit
value is a positive safe whole number no larger than `contextBytes`.
Exceeding the read limit, asking above the budget or attempting a read beyond
the source count records a breach that still refuses if caught. Missing or
unreadable sources, invalid UTF-8 and invalid JSON throw; when that failure
escapes the loader, `context_source_unreadable` names the source. Only
successful reads enter the measured ledger. See [read bounds and failures](04-writing-an-owner-policy.md#supply-policy-context).

## SDK and module resolution

### `ravel/sdk`

`ravel/sdk` is the exact embedded module specifier. The following declarations
are exported; type declarations guide typechecking, while the front validates
runtime values independently. Configure the editor/compiler against installed
`sdk/index.d.ts`; TypeScript path mappings do not create runtime aliases.
See [typecheck public declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations).

| Export | Meaning | Teaching |
|---|---|---|
| `definePolicy(policy)` | Returns its `Policy` argument unchanged; checks a literal's fields and infers method parameter types. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `readChoice(directory, offered)` | Reads `.ravel-choice` in exactly that directory; returns offered names, `undefined` for no file, or a `Refused` value. Detailed rules follow. | [Offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice) |
| `CHOICE_FILE` | Literal `.ravel-choice`; not a Ravel CLI input or configuration-discovery rule. | [Offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice) |
| `Policy` | Four-field version-2 owner policy interface. | [Define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `SelectionRequest` | Nine-field frozen caller request, with three optional fields. | [Read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `Limits` | Six effective numeric bounds exposed to policy. | [Set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names) |
| `Selected` | Seven-field command and routing labels. | [Select a command](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `Refused` | Four-field owner refusal from selection or loading. | [Refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `SelectionResult` | Union of `Selected` and `Refused`. | [Select or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse) |
| `LoadContext` | Loader function type, returning `Context` or `Refused` synchronously or as a promise-like value. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `Context` | Six-field version-1 document; only `schemaVersion` required. | [Supply policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `DeliveredContext` | `Context` plus required measured-source ledger. | [Delivered policy context](04-writing-an-owner-policy.md#supply-policy-context) |
| `Json` | JSON data type for facts and assessment values. | [Policy-context data](04-writing-an-owner-policy.md#supply-policy-context) |
| `Assessment` | Attributed judgement with `by` and `value`. | [Assessments](04-writing-an-owner-policy.md#supply-policy-context) |
| `SourceRecord` | Attributed evidence name with digest/version and optional byte count. | [Attributed sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `MeasuredSource` | Four-field measured read record, including `via`. | [Measured sources](04-writing-an-owner-policy.md#supply-policy-context) |
| `ReadSource` | Three-field measurement returned beside host-read content. | [Host reads](04-writing-an-owner-policy.md#supply-policy-context) |
| `SelectHost` | Diagnostic operation and abort signal for selection. | [Selection host](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request) |
| `ContextHost` | Selection-host members plus measured reads for the loader. | [Loader host](04-writing-an-owner-policy.md#supply-policy-context) |

`readChoice` accepts a regular UTF-8 file of at most 4,096 bytes. Missing
files, including dangling symlinks, return `undefined`; an empty file returns
an empty array. Names split on whitespace and retain their order and
duplicates. A name outside `offered` returns `choice_unoffered`; unreadable,
non-regular, oversized or invalid UTF-8 files return `choice_unreadable`.
Return that refusal from policy to expose it as `policy_refused` with the
helper code in `policyCode`. See [choice-file validation](04-writing-an-owner-policy.md#offer-a-checkout-choice).

`directory` is the one directory read; there is no parent search. Pass
`request.cwd` for a checkout choice. A relative directory resolves from the
worker's directory, initially `/`. Policy decides defaults, cardinality and
the meaning of each offered name. The file introduces no program, argument
or label; this ordinary SDK filesystem read is not a measured host read, so
name the applied choice in `reason`. See [apply a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice).

### Import resolution

| Name or input | Resolution and consequence | Teaching |
|---|---|---|
| `ravel/sdk` | Literal import selects the worker's embedded SDK even beside a package shadow. Installed source edits do not replace it. | [Embedded imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `ravel/examples/static` | Literal import selects the embedded static example. Import a copied file by its path to use edits. | [Embedded imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `./routes.ts` or another relative file import | Resolves from the importing file; a policy symlink uses its canonical real file's location. | [Relative imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Other bare specifiers, including other `ravel/*` names | Ordinary Bun package resolution from the importing file, including `node_modules`; only the two exact names above are registered. Missing dependencies are never installed automatically. | [Package imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `package.json` | Nearest file that reads as a package supplies `imports`, the package's own name and `exports`; unparseable/non-package files are passed over. Package `main`/`exports` resolution remains enabled. | [Package resolution](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Package `imports` alias targeting an embedded name | Remains a package lookup and can load a shadow; it does not return to embedded registration. Use the literal specifier for embedded identity. | [Alias limits](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Import without a file location | Resolves from process cwd, initially `/` before owner code runs. `/package.json` and `/node_modules` remain reachable resolver inputs; temporary-directory ancestors do not participate. | [Worker directory](04-writing-an-owner-policy.md#resolve-policy-imports) |
| Dotenv, bunfig and tsconfig autoloading | Disabled in the compiled worker; editor tsconfig aliases alone do not work at runtime. Package autoloading stays enabled. | [Startup controls](04-writing-an-owner-policy.md#resolve-policy-imports) |

Owner code can change cwd or import arbitrary code. A Bun `Worker` thread
starts another runtime instance; disabled dotenv autoloading also prevents
that instance from loading ambient dotenv files. These are startup controls,
not confinement of trusted policy. See [runtime boundaries](04-writing-an-owner-policy.md#resolve-policy-imports).

## Shipped sample

### `ravel/examples/static`

`ravel/examples/static` exports the following editable starting points.
The example's kinds and effort names are conventions of this table, not
Ravel's vocabulary. Its placeholder program/provider/model need replacing
before real use. See [use the starter table](04-writing-an-owner-policy.md#use-the-starter-route-table).

| Export | Behaviour | Teaching |
|---|---|---|
| `Command` | Type `Selected` without `status` and `reason`; the route supplies program, args and labels. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `Route` | Function type from `SelectionRequest` to `Command`. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `agent(effort)` | Builds a route for `my-agent-wrapper`, provider `your-provider`, model `your-model`, with model/effort arguments and the unchanged prompt last. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `routes` | Exact table: `question` → `low`; `bugfix` → `medium`; `feature` → `high`; `migration` and `architecture` → `xhigh`. No wildcard. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `selectRoute(routes, request)` | Looks up an own property for the exact kind. Missing route returns owner code `incomplete_mapping`; a match adds selected status and a reason naming the applied route, program, model and effort. | [Route selection](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `policy` | Version-2 starter using this table, owner version `ravel/examples/static 2`; no context loader. | [Starter policy](04-writing-an-owner-policy.md#use-the-starter-route-table) |

### `examples/sample.ts`

`libexec/ravel/examples/sample.ts` is the copyable owner policy, sourced from
`worker/sample/policy.ts`. It imports the embedded SDK and static table, uses
policy version `owner-choice-1`, and offers `.ravel-choice` names `default`
and `careful` in `request.cwd`. With no file it chooses `default`; an empty
file or more than one name returns owner refusal `one_choice`. The helper's
unreadable/unoffered refusals pass through unchanged. See [offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice).

`default` returns the static selection. `careful` keeps its program/model/provider
and sets effort to exactly `high`, rebuilds the model/effort/prompt arguments
and uses reason `careful choice`; a missing route still refuses. This sets
`high` even for a static `xhigh` route. Edit the copy to express the arrangement
you intend and validate every branch with fake programs. See [choice behaviour](04-writing-an-owner-policy.md#offer-a-checkout-choice).

`policy.ts` becomes owner configuration only when copied to the chosen entry
and edited there. Ravel has no `init` command and installs no personal policy.
To edit the static table itself, copy `static.ts` too and import that local
file; importing `ravel/examples/static` continues to use the compiled copy.
Fake routes check argument delivery, not real-agent execution or permissions.
See [copy an existing policy](04-writing-an-owner-policy.md#copy-an-existing-policy).

## Installed layout

`PREFIX` below is the installation root, independent of the owner's `HOME`.
`task install PREFIX=/path` builds the release pair, installs its assets and
checks the installation. Installed execution needs neither host Bun/Node nor
the source checkout. See [obtain the build](03-a-complete-session.md#obtain-the-build-used-by-this-chapter).

| Path below `PREFIX` | Purpose and source relationship | Teaching |
|---|---|---|
| `bin/ravel` | Native Rust front built from `crates/ravel`, linking the runner in `crates/keyed-launch`. Owns policy admission, supervision and final result settlement. | [Installed pair](03-a-complete-session.md#obtain-the-build-used-by-this-chapter) |
| `libexec/ravel/ravel-policy` | Private compiled TypeScript worker built from `crates/ravel/worker/src`, embedding Bun, SDK and static example. Evaluates policy; never holds session-result authority. | [Policy evaluation](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy) |
| `libexec/ravel/sdk/index.ts` | Copy of `worker/sdk/index.ts`, for reading or copying. Editing it does not change the embedded SDK. | [SDK and imports](04-writing-an-owner-policy.md#resolve-policy-imports) |
| `libexec/ravel/sdk/index.d.ts` | Generated public SDK declarations for policy typechecking. | [Installed declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations) |
| `libexec/ravel/examples/static.ts` | Copy of `worker/examples/static.ts`. Import a local copy by path to use edits. | [Starter table](04-writing-an-owner-policy.md#use-the-starter-route-table) |
| `libexec/ravel/examples/static.d.ts` | Generated declarations for the static example exports. | [Installed declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations) |
| `libexec/ravel/examples/sample.ts` | Copy of `worker/sample/policy.ts`, an owner-editable entry using checkout choices. Not automatically copied into `HOME`. | [Copy a policy](04-writing-an-owner-policy.md#copy-an-existing-policy) |
| `libexec/ravel/notices/NOTICES.md` | Runtime third-party notice inventory copied from `crates/ravel/notices`. Keep it with the worker. | [Installation assets](03-a-complete-session.md#obtain-the-build-used-by-this-chapter) |
| `libexec/ravel/notices/bun-LICENSE.md` | Bundled Bun licence/notice text copied from `crates/ravel/notices`. | [Installation assets](03-a-complete-session.md#obtain-the-build-used-by-this-chapter) |
| `README.md` | Additional archive-root entry identifying the project; archive packaging includes it alongside the installed pair/assets. | [Obtain Ravel](03-a-complete-session.md#obtain-the-build-used-by-this-chapter) |
| `LICENSE` | Additional archive-root project licence. | [Obtain Ravel](03-a-complete-session.md#obtain-the-build-used-by-this-chapter) |

`libexec/ravel/ravel-policy` is found relative to the canonical real front,
including when `bin/ravel` is reached through a symlink. `PATH`, `argv[0]`
and neighbouring prefixes cannot choose another worker. The front checks
protocol 1, package version and source build identity before sending policy.
Missing, stale, mismatched and probe workers fail with status 5. Build identity
pairs binaries; it is not code signing. See [admit the installed pair](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy).

`crates/keyed-launch` is linked Rust library code, not an installed service
or third executable. The front starts the worker for policy evaluation,
reaps it before managed launch, then supervises the selected job using that
library. The worker embeds executable SDK/example modules; the installed
source and declaration files support editing and typechecking separately.
See [the session's processes](03-a-complete-session.md#admit-the-request-and-evaluate-the-policy).

> **Note:** Chapter 10 remains unwritten until a supported extracted
> component exists. The installed layout does not establish such a consumer
> contract. Native x86-64 Linux qualification also remains open; the book's
> [platform and operational limits](05-running-sessions.md#understand-the-operational-limits)
> distinguish native measurements from emulation.
