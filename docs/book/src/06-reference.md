# Reference

Use this chapter to look up the command line, exit statuses, streams and
environment rules. A [caller](02-actors-and-words.md#caller-and-owner) starts
a [session](02-actors-and-words.md#caller-and-owner); the
[owner policy](02-actors-and-words.md#caller-and-owner) selects its
[managed process](02-actors-and-words.md#the-managed-process-and-its-result).
Only the caller gives meaning to the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result).
Each entry links to the chapter that teaches its use.

The entries describe the [checkout build used by this book](03-a-complete-session.md#obtain-the-build-used-by-this-chapter).
The configuration and installed-layout sections at the end are reserved for
the next drafting part; their entries are not complete yet.

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

Reserved for the configuration draft. Teaching: [define a policy](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request)
and [set bounds and grants](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Bounds

Reserved for the configuration draft. Teaching: [set bounds](04-writing-an-owner-policy.md#set-bounds-and-grant-environment-names).

## Policy and request

Reserved for the configuration draft. Teaching: [define a policy and read the request](04-writing-an-owner-policy.md#define-a-policy-and-read-the-request).

## Selection and refusal

Reserved for the configuration draft, including diagnostic-code entries.
Teaching: [select a command or refuse](04-writing-an-owner-policy.md#select-a-command-or-refuse).

## Policy context and host operations

Reserved for the configuration draft. Teaching: [supply policy context](04-writing-an-owner-policy.md#supply-policy-context).

## SDK and module resolution

Reserved for the configuration draft. Teaching: [typecheck declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations)
and [resolve imports](04-writing-an-owner-policy.md#resolve-policy-imports).

## Shipped sample

Reserved for the configuration draft. Teaching: [use the starter route table](04-writing-an-owner-policy.md#use-the-starter-route-table)
and [offer a checkout choice](04-writing-an-owner-policy.md#offer-a-checkout-choice).

## Installed layout

Reserved for the configuration draft and the subsequent art stage's reusable
package/deployment figures. Teaching: [obtain the build](03-a-complete-session.md#obtain-the-build-used-by-this-chapter)
and [typecheck against installed declarations](04-writing-an-owner-policy.md#typecheck-against-the-public-declarations).
