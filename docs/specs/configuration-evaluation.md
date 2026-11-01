<a id="configuration-evaluation"></a>

# Configuration evaluation

This specification describes the current Rhai configuration evaluation path for Ravel 2.0.0.
It owns entry discovery, requests, selection context, selections, settings, host capabilities, evaluation bounds and delivery.
The [session execution specification](session-execution.md) owns the managed launch and result contract.
The [completion specification](result-job-completion.md) owns detailed settlement and native-entry requirements.
The migration section records the retired TypeScript correspondence.

The [skill discovery specification](skill-discovery.md) owns skill directory delivery.
It defines the request fields `originalPrompt`, `prompt` and `skills`, and the settings key `skills`.

Three decisions bind this area:

- [the configuration script uses Rhai](../adr/configuration-script-uses-rhai.md).
- [configuration evaluation precedes the launch](../adr/configuration-evaluation-precedes-the-launch.md).
- [a configuration script asks a model through a harness run](../adr/a-configuration-script-asks-a-model-through-a-harness-run.md).

The decision that [the launched child is a job](../adr/the-launched-child-is-a-job.md)
governs each harness run.

The [glossary](../../CONTEXT.md) defines the terms. GitHub issue 2 states the requirements and the acceptance criteria. The
[design views](../design/rhai-configuration/README.md) draw the processes, the
evaluation sequence and the paths that stop an evaluation.

## Problem

Ravel 1.x evaluated a TypeScript configuration script. Its Bun worker required a JavaScript runtime, runtime notices, startup controls and target checks. Most configuration scripts are small tables that choose a harness, a
model and an effort. They need none of that runtime.

Owners also want a configuration script to ask a model a question, for example to classify a
task before it chooses a route. A 1.x script needed its own process or network code for that work. Ravel controlled neither its deadline nor its cleanup. The owner signs in to Claude Code and Codex with a
subscription. A subscription sign-in is not an API key that another program can
send.

## Solution

The configuration script is a Rhai module. Ravel evaluates it in a Rhai worker,
which is Ravel's own binary in an internal role. The worker only computes.

The worker has an empty environment. It reads no file and launches no process. Ravel
trusts the configuration script, so a configuration script can still receive a credential through a
host capability, as [credentials](#credentials) states.

The configuration script asks the Ravel CLI for each host capability: measured reads,
discovery and harness runs. The Ravel CLI performs each one and returns data.
It runs a harness that the owner names in settings through the runner, as a job
with a result channel of its own. The data helpers run in the worker.

An LLM call is a harness run. The configuration script runs Claude Code or Codex
noninteractively, with the arguments of that harness. The harness uses the
owner's own sign-in. The configuration script reads the answer from the output of the run or
from its result. Ravel holds no model credential, opens no network connection
and builds no argument that is specific to a harness.

The outcome of a host call is data that the script interprets. A harness run
that Ravel stops at its own deadline or at its output bound is such an outcome.
A breach of any other bound ends the evaluation. The script cannot catch that
breach with `try`. That holds when the breach happens inside a function pointer,
such as the closure that `map` runs.

The Ravel CLI kills
the worker at the deadline or on cancellation. It stops the host work in
progress, and nothing launches. On the path to a launch, each host call ends
before the worker sends its result. The Ravel CLI reaps the worker before the
managed process launches.

The caller supplies the request and interprets the result. The script selects the command, and Ravel supervises it.

## Decisions

### Versions and toolchain

| Component | Pin | Role |
|---|---|---|
| Rust | 1.99.1, edition 2024 | The workspace floor and the toolchain file. It is the newest stable release at design time |
| Rhai | `=1.27.0`, with the `compact_str` feature | The configuration script language in the worker |
| Tokio | 1, a current-thread runtime | The asynchronous runtime of the evaluation phase |

The design relies only on Rhai interfaces that it checked against the source of
the pinned release. The Rhai build never enables `unchecked`, because that
feature removes every limit. The owner chose the newest stable Rust as the
floor, and a later release may raise it through the upgrade checks.

Ravel has no LLM client library. Rig, reqwest and rustls are not dependencies,
because a [harness run](#llm-calls-through-a-harness) makes each LLM call.

The design considered unreleased Rhai 1.27.1 changes for two issues.
One removes the endless loop of `pad` with an empty padding string (rhaiscript/rhai#1195).
The other restores imports for a closure that a host calls after the script ends (rhaiscript/rhai#1193). Ravel
calls named functions and holds no closure, so the second issue does not reach
it. A test guards the case.

Rhai's default limits differ between debug and release builds. In a debug
build, the call depth and the two expression depths are 8, 32 and 16. A release
build has 64, 64 and 32. Ravel sets every limit itself, so tests and releases
agree.

### Configuration entry and discovery

The default entry is `$HOME/.config/ravel/config.rhai`, and `HOME` must be
absolute. `--config PATH` names another entry, relative to the caller's working
directory. There is no search, no environment override and no fallback.

`policy.rhai` is not a default-path alias. An explicit `.rhai` entry may have
any basename, including `policy.rhai`. The historical `policy.ts` probe below
only produces a retired-format refusal. It never selects an entry.

| Entry state | Result |
|---|---|
| `config.rhai` exists | Ravel evaluates it, and it does not open `policy.ts` |
| Only `policy.ts` exists | `config_format_retired`, exit 3. The remedy names `config.rhai` and the book's chapter 4, "Write a configuration script" |
| Neither file exists | `config_missing`, exit 3. The message names `config.rhai` |
| `--config` names a `.rhai` file | Ravel evaluates it |
| `--config` names a `.ts`, `.mts`, `.cts`, `.js`, `.mjs` or `.cjs` file | `config_format_retired`, exit 3 |
| `--config` names any other file | `config_unreadable`, exit 3. The remedy asks for a `.rhai` file |

The entry must canonicalise to a readable regular UTF-8 file of at most 1 MiB.
Ravel admits a symlink by its real file. Imports then resolve from the directory
of that real file. Settings stay at `$HOME/.config/ravel/settings.json`, also
with `--config`.

A module import has the form `import "PATH" as NAME;`. A relative path resolves
from the directory of the importing file. Ravel appends `.rhai` when the path
has no extension, and it accepts an absolute path. The Ravel CLI reads each
module with the file rules of the entry, and each module loads once per
evaluation.

A missing or unreadable module, a cycle or a 65th module refuses as
`config_import_failed`. The bound counts distinct imported files, excluding the entry. An import of a loaded
module is no new module, however many import statements run.

Ravel measures the entry and every module. The routing notice lists each with
its canonical name, its byte count and its SHA-256. Ravel 2.0.0 embeds no module
of its own, so every import names a file.

### The configuration module

| Export | Required | Meaning |
|---|---|---|
| `SCHEMA_VERSION` | Yes | The constant 4. Another value, including 3, refuses as `unsupported_version` |
| `VERSION` | Yes | A nonblank string that the owner maintains. The notice reports it |
| `fn select(request, context)` | Yes | It returns a selection or a refusal |
| `fn load_context(request)` | No | It returns a selection context or a refusal |

Ravel evaluates the entry as a module. Its top-level statements run once, before
any of its functions. A missing export, a wrong arity or a blank version refuses
as `config_invalid`.

Other functions and constants are helpers of the configuration script. A function reads a top-level constant through the `global::` namespace, such as
`global::TABLE`. Rhai functions do not see the variables around them. A wrapper
configuration script imports another module and calls its `select`. It can change the
selection that comes back.

The engine runs in a strict mode:

- An undefined variable fails when the script compiles.
- An absent map property fails with a source location, so a mistyped field
  cannot read as `()`.
- Ravel disables `eval`. A configuration script cannot run text as code, such as the answer
  of a model.
- `print` writes to the captured configuration script stdout. `debug` writes to the captured
  configuration script stderr.

### Request, context and selection

`kind` is an open nonempty caller token. Ravel derives no kind or identity from a task file or filename.
`taskId` is optional nonempty UTF-8 of at most 1,024 bytes.
`taskFile` is optional and becomes an absolute UTF-8 path relative to the caller's working directory.
It need not exist, and Ravel does not read it automatically.

The prompt requires exactly one of `--prompt` and `--prompt-file`.
Ravel preserves its UTF-8 bytes, including empty text and newlines, in `originalPrompt`.
The prompt bound is 1 MiB, although platform argument limits can refuse less.
A prompt file resolves against the caller's working directory. Ravel never reads a prompt from terminal stdin.

With skill delivery disabled, `prompt` holds the same bytes as `originalPrompt`, and Ravel adds no instructions.
With delivery enabled, `prompt` is the [prepared prompt](skill-discovery.md#the-prepared-prompt).

Each repeatable `--param NAME=VALUE` supplies caller data.
A name must be nonempty, contain no `=` and occur once. The value is the exact text after the first `=`.
All names and values together have a bound of 65,536 UTF-8 bytes.

The request is a map. Every documented field is present. An optional field that
the caller did not supply holds `()`.

| Field | Type | Meaning |
|---|---|---|
| `schemaVersion` | The integer 4 | The version of this request shape |
| `kind` | String | The caller's kind |
| `originalPrompt` | String | The caller's prompt, unchanged |
| `prompt` | String | The prompt to deliver. It equals `originalPrompt` unless skill delivery prepared it |
| `cwd` | String | The caller's working directory, as data |
| `params` | Map of strings | Each `--param NAME=VALUE` |
| `taskFile` | String or `()` | The caller's `--task-file`, absolute. Ravel does not read it |
| `taskId` | String or `()` | The caller's `--task-id` |
| `context` | Map or `()` | The caller's `--context` document, checked |
| `skills` | Map or `()` | The resolved skill directories, or `()` with delivery disabled. The skill discovery specification defines the map |
| `limits` | Map | The effective bounds, for reading |

`limits` holds `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes`, `diagnosticsBytes` and `harnessRuns`. A configuration script cannot raise any
of them. Map keys keep their documented JSON names, so a value converts to JSON unchanged. Host functions use the snake-case names of Rhai.

The selection context uses schema version 1 and the checks below. Without
`load_context`, `select` receives the caller's context with its measurement, or
`()`. With `load_context`, `select` receives what it returns, with the measured
sources attached. A `load_context` that returns `()` refuses as
`context_invalid`. One that returns a refusal refuses as `config_refused`, and
`select` does not run.

A selection context is a JSON object with these fields:

| Field | Shape |
|---|---|
| `schemaVersion` | Required integer 1 |
| `summary` | Optional string |
| `acceptanceCriteria` | Optional array of strings |
| `facts` | Optional object |
| `assessments` | Optional object whose values contain only required `by` and `value` fields. `by` is nonblank text, and `value` is JSON data |
| `sources` | Optional array of at most 256 source records |

A source record has a nonblank `name` and at least one of `sha256` or `version`. `sha256` has 64 lowercase hexadecimal digits. `version` is nonblank text, and optional `bytes` is a nonnegative integer.

A source record has no other field. Unknown or executable context fields refuse. The caller and loader cannot supply `measured`.

Ravel attaches that list from its own reads, with the caller's document first. Attribution is data and supplies no execution authority or authentication. The delivered JSON encoding must fit the context budget.

Ravel never truncates the delivered context.

`select` returns one of two maps:

| Map | Fields |
|---|---|
| Selected | `status: "selected"`, `program`, `args`, `provider`, `model`, `effort` and `reason` |
| Refused | `status: "refused"`, `code`, `message` and `remedy` |

All listed fields are required, and no unknown field is present. The strings are nonblank, except that an argument can be empty. `program` and every argument must contain no NUL.

`args` is an array of UTF-8 strings. Ravel performs no shell expansion or word splitting. A shell runs only when the script selects one.

Absolute paths, relative paths with separators and `PATH` names resolve in the caller's execution context. The labels are owner text, and Ravel assigns them no provider integration. A refusal launches nothing and carries the owner's code as `configCode`.

A return of `()`
refuses as `selection_abstained`. An uncaught error refuses as
`selection_threw`, with the file, the line and the column. Ravel never
substitutes another route.

### Conversion at the boundary

Every value that crosses between Rhai and the Ravel CLI is JSON data. One
converter applies the same rules in both directions. It names the location of
each value that it refuses, such as `result.args[2]`.

| Rhai value | JSON value |
|---|---|
| `()` | `null` |
| Boolean | Boolean |
| 64-bit integer | Integer |
| Float | Number. NaN and the infinities refuse |
| String | String |
| Array | Array |
| Object map | Object |
| Character, BLOB, function pointer, timestamp, range or custom type | Refused |

From JSON, an integer outside the signed 64-bit range becomes a float. A shared
value that a closure captured converts as its current content. Arrays and
maps nest at most 100 levels. A deeper value refuses at its location.

### The Rhai worker

The Ravel CLI launches the worker when it re-executes its own binary in an
internal role. On Linux it executes the running image itself. On macOS it
executes the canonical path that it resolved when it launched. In both cases the
worker reports the build identity of its binary. The Ravel CLI
refuses another identity as `worker_identity_mismatch`, and a worker that cannot
launch as `worker_missing`.

The worker gets a private mode-0700 empty
directory, null stdin, an empty environment and the private socket at fd 3. It
inherits no other descriptor. The internal role accepts no command-line input,
and without the expected socket and handshake it exits with status 2. It cannot
launch a process or read a file, so it offers no bypass of selection.

The private socket carries length-prefixed JSON frames, separately from stdout and stderr. The handshake checks protocol 1, package version, build identity and Rhai version before evaluation. The build identity hashes the Rust sources of Ravel and its runner, the crate manifest, build script and Cargo lockfile.

It pairs the supervisor and worker builds. It does not authenticate the installation against its owner. The Ravel CLI drains and bounds worker stdout and stderr concurrently.

Neither stream can supply a selection or an opaque result.

Admission follows the [worker teardown contract](result-job-completion.md#worker-teardown-before-admission), including failure precedence and diagnostic retention.

The worker evaluates the script on one thread with a stack of 64 MiB. Its engine
limits are fixed:

| Limit | Value | Breach |
|---|---|---|
| Operations | 100,000,000 | `script_limit` |
| Call depth | 64 | `script_limit` |
| Expression depth | 64 at the top level, 32 in a function | `script_limit` |
| String | 16 MiB | `script_limit` |
| Array, with nested elements counted | 1,048,576 | `script_limit` |
| Object map, with nested properties counted | 1,048,576 | `script_limit` |
| Variables at one time | 16,384 | `script_limit` |
| Script functions | 4,096 | `script_limit` |
| Modules | 64 | `config_import_failed` |
| Worker heap | 1 GiB | `memory_limit` |

A counting allocator in the worker enforces the heap bound. It counts every live
byte of the worker process. When an allocation would pass the bound, the worker
reports `memory_limit` and exits.

The script cannot catch an engine limit with `try`. Rhai passes these errors
unwrapped through the boundary of a script function, and `try` does not catch
them. Rhai 1.27.0 does wrap every error that crosses a call of a function
pointer in the array and map functions. `try` catches that wrapper whatever it
holds.

The worker replaces each of those functions. A replacement does what the
original does. At the end of a pointer call it returns an error that `try`
cannot catch as it is.

It wraps every other error as before. So an ordinary
`throw` in a closure is still caught. A call-depth, data-size or variable breach
ends the evaluation.

The replaced functions are the 16 call sites of `call_raw_with_extra_args` in
Rhai 1.27.0. Those are `for_each`, `map`, `filter`, `find`, `find_map`,
`index_of`, `some`, `all`, `reduce`, `reduce_rev`, `zip`, `drain` and `retain`
of an array, and `map`, `filter`, `drain` and `retain` of an object map. They
include the deprecated forms that name a function with a string, such as
`map("f")` and `drain("f")`.

Rhai 1.27.0 also drops the error of a comparer
closure in `dedup`, in `sort` and in `order`, with no `try` involved. `order`
is also exported as `order_by`, and it calls the original `sort_by` in Rust, so
a replacement of `sort_by` does not reach it. The worker replaces those
functions too, and the string forms of `dedup` and `sort`. Each replacement
returns the first error of the comparer, as `map` does.

The worker counts operations itself, in the progress hook, for the top level
and `select` together. It latches a breach of the operations bound when it
happens, and it refuses that breach whatever the script returns afterwards. This
is a second line for the one limit that Rhai reports through a hook.

Rhai checks a data size when a call returns a value. It does not check the size
when `push` or an index assignment grows a container in place. A script can grow
a container past its size limit with those calls, and the heap bound ends that
growth.

The worker exits after it sends its result. The Ravel CLI reaps it and removes
its private directory before the managed process launches.

### The bridge between the script and the CLI

A host function is a blocking call in the worker. It sends one request to the
Ravel CLI over the private socket and waits for the reply. The script therefore
sees only synchronous calls. It conducts several interactions in sequence, and it
keeps any message history itself.

The Ravel CLI serves the socket from a Tokio current-thread runtime that exists
only for the evaluation phase. While a host call runs, the runtime keeps
watching the deadline, the cancellation latch, the health of the worker and the
output bounds. A long read or harness run therefore never blocks the work that
stops it.

The runtime thread makes no filesystem call. A reader thread of the Ravel CLI
does the filesystem work of the host functions. That work is the reads of the
entry, of each module and of each measured source. It also includes the program
lookups of discovery and of a harness run. The Ravel CLI reads the entry after it
launches the worker, so the whole deadline covers every read.

The runtime waits for the reply of the reader as it waits for any host call. The
reader is not a blocking task of the runtime, because Tokio cannot stop a running
blocking task, and its shutdown waits for one.

The runtime spawns no blocking task. Its shutdown therefore never waits for
work that Tokio cannot stop.

Only one host call runs at a time, because the script is synchronous. The
worker sends its result only after the reply to its last host call. A message
from the worker while a host call is in progress breaks the protocol, and the
evaluation refuses as `worker_failed`. On the path to a launch, no host call is
therefore in progress. The runtime shuts down with all of its tasks finished
before the managed process launches.

Each message from the worker is at most `messageBytes`, 1 MiB, except a
delivered context, which has the context budget. A reply from the Ravel CLI has
the bound of its capability.

#### Frame bounds

A frame is a four-byte length and that many bytes of JSON. One constant bounds
every frame that the Ravel CLI sends and the worker reads. The largest such
frames are the evaluate frame and the reply to a harness run.

The evaluate frame carries the entry source and the request, with both prompts
and `skills`. It also carries the caller's context twice: in `request.context`,
and as the delivered context in `selectContext`. Its bound comes from the
public bounds at the largest JSON escape. A control character escapes to six
bytes. A parameter adds at most six bytes of quotes and separators, beside the
escape of its name and value.

```text
6 × (entry 1 MiB + original prompt 1 MiB + prepared prompt 1 MiB + task ID 1,024 bytes)
+ 12 × parameters 65,536 bytes
+ 2 × context budget maximum 8 MiB
+ descriptor 1 MiB
+ envelope 1 MiB
= 38,541,312 bytes
```

The reply to a harness run holds three strings of at most 1 MiB each, escaped
six-fold. One more message bound covers the rest of the map. That is 19 MiB. The
frame bound is the larger value, 38,541,312 bytes.

Every request within the public bounds therefore fits. Only an exceptionally
long kind, working directory or task-file path can pass the bound, because
Ravel does not bound those three values itself. Before it sends the evaluate
frame, the Ravel CLI compares the encoded frame with the frame bound. A larger
frame refuses as `malformed_input`, stage `cli`, exit 2. The refusal names the
encoded size and the largest members of the request.

Ravel then stops and reaps the worker, and it evaluates nothing. A frame over
the bound is never a `protocol_error` or a `worker_failed`.

The worker checks each of its own messages against `messageBytes` before it
sends it. A script result over that bound refuses as `message_too_large`,
exit 3. A selection that carries the prepared prompt in `args` uses part of
that bound. The argument limit of the platform can refuse a smaller selection
when the program launches.

The frame bound is a conservative sum, not the encoded size of one request.
Its envelope allowance of 1 MiB is much larger than the other members of an
ordinary request. With delivery enabled, the prepared prompt contains the
original prompt. With delivery disabled, no descriptor exists. The two prompts
and the descriptor therefore cannot all fill their allowances.

A request at every public bound thus encodes to less than the frame bound. A
bound that is one byte lower still admits that request. Such a control cannot
show whether the check runs.

Three tests cover the bound and its check:

1. At the command-line seam, a request at every public bound, with the largest
   escape, fits and evaluates.
2. The check compares the encoded frame with a limit that it receives as an
   argument. A unit test encodes a known evaluate frame. The frame passes at a
   limit equal to its encoded size, and it refuses at a limit one byte lower.
   The test also shows that the production limit is 38,541,312 bytes.
3. At the command-line seam, a request fills the public bounds. A long kind
   takes its encoded frame over the frame bound. The request refuses as
   `malformed_input` before evaluation. A removed check gives another failure
   or an evaluation, so this test also detects that removal.

The argument limit of the platform bounds the kind and the task-file path. The
platform also bounds the working directory. On a platform with small limits, no
request can then pass the frame bound. The local stage establishes on which
release platforms test 3 can run. It records each platform where test 3 cannot
run. There, test 2 alone covers the comparison.

No test needs a public flag that changes the frame bound. A selection over the
message bound refuses as `message_too_large`.

A stop does not wait for a read in progress. The Ravel CLI abandons the read and
its reply, and nothing launches after the stop. The reader thread keeps the
descriptor of that read, and it closes it when the call returns or the Ravel CLI
exits.

The Ravel CLI creates the thread with INT, TERM and HUP blocked, as it creates
the readers of the worker output. The thread can outlive the stop through the
finalisation of the Ravel CLI, and it must not take a signal there. Ravel opens
every descriptor close-on-exec, so no harness run inherits one. These rules
bound the waiting of Ravel. As in the [native-entry contract](result-job-completion.md#checked-native-entry-transitions),
they are no real-time guarantee against a stall in the kernel.

The kernel can hold a filesystem call without interruption, as with a read from
a stalled network filesystem. Such a call can then delay the exit of the Ravel
CLI. The refusal and its report are already written at that time.

### Host capabilities

All host functions are in the `ravel` namespace. Each one is available in
`load_context`, in `select` and at the top level of a module.

| Function | Returns | Data failure | Ends the evaluation |
|---|---|---|---|
| `ravel::read_text(path)`, `ravel::read_text(path, max_bytes)` | `#{ text, source }` | A thrown map: `source_missing`, `source_not_regular`, `source_unreadable` or `source_not_utf8` | `source_too_large`, `too_many_sources` |
| `ravel::read_json(path)`, `ravel::read_json(path, max_bytes)` | `#{ value, source }` | As `read_text`, and `source_invalid_json` | As `read_text` |
| `ravel::json_parse(text)` | A value | A thrown `json_invalid` | Nothing |
| `ravel::json_encode(value)` | A string | A thrown `json_unencodable` | Nothing |
| `ravel::json_check(value, schema)` | The value, unchanged | A thrown `json_mismatch`, `schema_invalid` or `json_unencodable` | Nothing |
| `ravel::uuid_v4()` | A string | Nothing | Nothing |
| `ravel::path_resolve(base, path)` | A string | A thrown `path_not_absolute` | Nothing |
| `ravel::path_parent(path)` | A string | A thrown `path_not_absolute` | Nothing |
| `ravel::env(name)` | A string or `()` | A thrown `env_not_granted` or `env_not_utf8` | Nothing |
| `ravel::harnesses()` | An array of maps | Nothing | Nothing |
| `ravel::remaining()` | A map | Nothing | Nothing |
| `ravel::run_harness(request)` | A result map | A `status` other than `exited` | A budget, the whole deadline, cancellation, `host_request_invalid`, `harness_cleanup_failed` |

A thrown map has `code`, `message` and, where it applies, `name`. The script can
catch it. A malformed request to any host function ends the evaluation as
`host_request_invalid`. Its location names the function, the field and the line
in the script.

#### Measured reads

A read resolves a relative path against `request.cwd`. It reads a regular UTF-8
file once. It opens the file without blocking, so a FIFO cannot hold
the read. It checks that the open file is a regular file, and it reads at most
one byte beyond its bound.

A read returns the content with a source record: the canonical `name`, the
`bytes` that it read and their `sha256`. `max_bytes` defaults to `sourceBytes`
and may rise to `contextBytes`. The bound of 256 sources counts the caller's
context document and every read in the evaluation.

A read counts once the file
gave its bytes, whether or not they decode. A file that is not UTF-8, is not JSON or
nests too deep is therefore a source, and a script cannot retry it without
limit. A read that finds no file to read is no source.

`read_json` returns a value that nests at most 100 levels, the bound of the
conversion rule. A deeper file is a data failure, `source_invalid_json`. Its
message names the first level past the bound.

Reads that `load_context` makes enter the `measured` list of the delivered
context. The caller's document comes first, when there is one. Ravel
also measures the reads that `select` and the top level make, and the notice
lists every read with its phase. A breach of a read
bound refuses even if the script catches the error. The shipped sample reads a choice file with `read_text` and `split()`.

#### Data helpers

`json_parse` parses strict JSON. `json_encode` writes the canonical form of
Ravel:

- It writes no whitespace.
- It writes object members in ascending order of their keys, by code point.
- It writes `\"`, `\\`, `\b`, `\f`, `\n`, `\r` and `\t` in strings. It
  writes each other character below U+0020 as `\u00xx`, with lowercase
  hexadecimal digits. It writes every remaining character as UTF-8.
- It writes integers in decimal, and finite floats in the shortest form that
  reads back exactly, such as `1.5`, `1.0`, `1e-7` and `1e+21`. A float keeps
  its type when `json_parse` reads it back.

Equal values therefore give equal bytes. For a string that Rhai can hold,
`json_encode` writes the bytes that `JSON.stringify` writes. A caller that needs
a fixed member order, such as the provider receipt of Grove, therefore joins
members that `json_encode` wrote.

The `to_json` of Rhai writes strings with Rust
debug escapes, which are not JSON for control characters. The `parse_json` of
Rhai reads Rhai syntax, so it accepts a trailing comma and a single-quoted
string. Neither is part of the contract of Ravel.

A value that has no JSON form throws `json_unencodable`, and its message names
the location, such as `value.args[2]`. A text that is not strict JSON throws
`json_invalid`. Strict JSON has no trailing comma, comment, single-quoted
string, `NaN` or text after the value. `json_parse` reads an integer outside
the signed 64-bit range as a float. It throws `json_invalid` for a value that
nests deeper than 100 levels, the bound of the conversion rule.

`json_check` checks a value against a JSON Schema in the subset below. It
returns the value unchanged when the value matches. A script checks the answer
of an [LLM call](#llm-calls-through-a-harness) with it, after `json_parse`. A
schema checks shape only, and the script decides what the value means.

| Keyword | Accepted value |
|---|---|
| `type` | One of `object`, `array`, `string`, `number`, `integer`, `boolean` and `null`, or an array of them |
| `properties` | A map from a name to a schema |
| `required` | An array of property names |
| `additionalProperties` | A boolean |
| `items` | A schema |
| `enum` | An array of strings, numbers, booleans or nulls |
| `title`, `description` | Strings. The check ignores them |

Any other keyword, or a nesting deeper than 32 levels, throws `schema_invalid`.
The root schema is level 1, and each member of `properties` and each `items`
schema is one level more. The message names the keyword or the schema that
breaks the rule, such as `schema.properties.a.format`.

A keyword applies to the kinds of value that it describes. `required`,
`properties` and `additionalProperties` say nothing about an array, and `items`
says nothing about an object. A number with no fraction is an `integer`, so
`3.0` matches.

A value that does not match throws `json_mismatch`. Its message
gives the location in the style of the conversion table, such as
`value.items[1].name`. The location can include property names from the value,
but the message omits property values.

The check converts the value by the conversion rule. A value with no JSON form
throws `json_unencodable`, as in `json_encode`.

The data helpers run in the worker and send no message. `json_encode` may write
any string that Rhai can hold, which is more than one message of the worker
holds.

`uuid_v4` returns a random version 4 UUID in lowercase hexadecimal. It takes its
randomness from the operating system.

`path_resolve` and `path_parent` are lexical, and they read no file.
`path_resolve` joins a relative path to an absolute base. It then removes `.` and
empty segments, and it applies each `..` to the segment before it, never above
`/`. An absolute path replaces the base. `path_parent` removes the trailing
slashes and the last segment, and the parent of `/` is `/`.

For absolute inputs,
both follow the rules of the `path.resolve` and `path.dirname` of Node, which
the migration of the owner's configuration script needs. That includes the cases where
`dirname` does not normalise: the parent of `/a//b` is `/a/`, the parent of
`/a/./b` is `/a/.`, and the parent of `//a` is `//`. A relative `base` or a
relative `path_parent` argument throws `path_not_absolute`, whatever the other
argument is.

`env` returns the caller's value of a readable name, or `()` when that name is
unset. The readable names are `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*` and the
names that `configEnv` or `--config-env` grant. Any other name throws
`env_not_granted`, whether or not the caller has set it, and the message never
holds a value. A value that is not UTF-8 throws `env_not_utf8`, because a Rhai
string holds UTF-8 only. The worker itself has no environment.

#### Discovery

`harnesses()` returns one map for each harness entry, in name order. Each map
has `name`, `program`, `available`, `resolved`, `reason` and `capabilities`.
Ravel resolves the program as it resolves a selected program, and it runs
nothing.

`reason` is `not_found` or `not_executable` when `available` is false.
`resolved` is the path that the lookup found, and `()` when `available` is
false. `reason` is `()` when `available` is true. `capabilities` repeats the
labels of the owner, and Ravel gives them no meaning. An owner can label the
models that a harness offers there, such as `model:sonnet`.

`remaining()` returns `ms`, `harnessRuns` and `sources`: what the evaluation
still has. `ms` is the time to the whole deadline, and `harnessRuns` is the
budget less the runs that the runner launched. `sources` is the bound of
256 less the caller's document and every read that arrived.

Discovery uses no
network and runs no harness. It reports availability, never a recommendation. A
harness offers no model list that Ravel can read without a run, so Ravel lists
only the labels that the owner writes.

#### Harness runs

`ravel::run_harness(request)` runs a harness that settings name, through the
runner, and returns what happened. It never evaluates a configuration script.

| Field | Type | Required | Meaning |
|---|---|---|---|
| `harness` | String | Yes | The name of a harness entry in settings |
| `args` | Array of strings | Yes | The arguments after the program of the entry. No string holds a NUL |
| `stdin` | String | No | Text for standard input. Without it, standard input is at end of file |
| `timeoutMs` | Integer | No | The deadline of this run. Ravel lowers it to the time that remains |

The program of the run is the program of the entry, and the script supplies the
arguments. The owner fixes the program, and the script supplies its argument vector. The script decides where a prompt goes: into an argument or into
`stdin`. Ravel builds no argument that is specific to a harness.

A harness run has these properties:

- It runs noninteractively, in a new POSIX session, with no controlling
  terminal.
- Its working directory is `request.cwd`.
- It receives the environment of a managed process. That is the caller's
  environment with the same scrub, or the confined environment.
- It receives `RAVEL_EXIT_FILE` for a fresh result channel of its own, and the
  marker `RAVEL_CONFIG_RUN`.
- It receives `RAVEL_SKILL_DIRECTORIES` and, under `--confine`, the read grants
  of the skill resources, as the
  [skill discovery specification](skill-discovery.md#ravel_skill_directories)
  states.
- Ravel confines it, with a private run directory of its own, when the caller
  passed `--confine`.

The runner supervises the run as a job, and its process group is the scope of
cleanup. A value sent on its channel triggers the escalation of a managed
process. At the deadline of the run or on cancellation, the Ravel CLI asks the
runner to stop it. The runner escalates from TERM to KILL. Each output stream
has a bound of 1 MiB, and Ravel stops the run at the first byte beyond it.

| Field | Meaning |
|---|---|
| `status` | `exited`, `signalled`, `timed_out`, `output_limit` or `unavailable` |
| `exitCode` | An integer, or `()` |
| `signal` | A signal name such as `SIGTERM`, or `()` |
| `stdout`, `stderr` | The captured text. Invalid UTF-8 shows as U+FFFD |
| `result` | The value that the run sent with `end-session`, or `()` |
| `elapsedMs` | An integer |
| `error` | `()`, or `#{ code, message }` when `status` is `unavailable` |

`unavailable` has the codes `unknown_harness`, `harness_not_found`,
`harness_not_executable` and `harness_launch_failed`. Those runs use no budget.
A run whose process group survives cleanup, or whose reap fails, ends the
evaluation as `harness_cleanup_failed`, exit 5. The script interprets the
output, the exit status and `result`, and Ravel judges none of them. A session
identifier that a harness prints reaches the script only in that output.

#### LLM calls through a harness

An LLM call is a harness run whose program asks a model one question and prints
the answer. The owner names Claude Code or Codex in a harness entry, and the
configuration script supplies the noninteractive arguments of that harness. Ravel builds no
argument that is specific to a harness, and it parses no output of a harness.
The rules of [harness runs](#harness-runs) apply unchanged. They cover the
budget, the deadlines, the output bounds, the environment and the stop.

The harness signs in with the owner's own sign-in, from its own store. Ravel
reads no sign-in and passes none on. Anthropic permits the sign-in of a Claude
subscription only in the unmodified Claude Code program. Its terms forbid
software that collects, stores or passes on a sign-in or a session token
([legal and compliance](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use)).
A harness run keeps that rule, because the program of the run is the harness
itself.

The script owns the question and the answer:

- It builds the prompt, and it puts the prompt into an argument or into `stdin`.
- It chooses the model, the effort and the tools with the flags of the harness.
- It reads the answer from `stdout`, or from `result` when the harness sends the
  answer with `end-session`.
- It parses a JSON answer with `ravel::json_parse`, and it checks the shape with
  `ravel::json_check`.
- It reads a failure, such as a refusal or a rate limit, from the exit status
  and the output. It decides whether to try again or to try another harness.

The installed releases at design time were Claude Code 2.1.310 and Codex CLI
0.164.0. The facts below come from their documentation and their `--help`
output. The book gives the recipes, and the smoke tasks prove them.

| Fact | Claude Code | Codex |
|---|---|---|
| A noninteractive run | `claude -p` prints the answer and exits | `codex exec` prints the final message on `stdout` |
| JSON output | `--output-format json` writes one document. The answer is in `result`, beside `usage` and `total_cost_usd` | `--json` writes JSON Lines events. `turn.completed` carries `usage` |
| Structured output | `--json-schema` takes the schema as text. The value is in `structured_output`, which is absent when the run ends without valid structured output | `--output-schema` takes the path of a schema file |
| The sign-in | `--bare` never reads the sign-in, so a call through a subscription omits it | `codex exec` uses the saved sign-in by default |

The sources are [Run Claude Code programmatically](https://code.claude.com/docs/en/headless)
and [Codex non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode).

A script cannot write a file. A Codex call with structured output therefore
names a schema file that the owner keeps. The script reads the same file with
`ravel::read_json`.

It then checks the answer against the schema that Codex
received. An unconfined run can read a schema file beside the configuration script. Confinement refuses a grant that overlaps the directory of the configuration script, so a
confined run cannot read the file there.

Ravel reports no usage and counts no token for an LLM call. The usage that a
harness prints reaches only the script, and a configuration script that wants it in the
diagnostics prints it. The harness-run budget, the deadline of the run and the
whole deadline bound the work instead. A subscription also has rate limits of
its own, which Ravel cannot see.

The prompt and the answer stay out of the notices and logs of Ravel, as all
output of a harness run does. The notice lists the run with its harness, its
status and its elapsed time. A script that copies an answer into the context
that `load_context` returns, or into its own output, publishes the answer
there. The notice reports the delivered context.

### Credentials

Ravel holds no model credential. It reads no API key and no sign-in, and it
stores and sends none. Each harness finds its own sign-in or key.

Ravel trusts the configuration script, because the owner writes it. The credential rules
therefore do not defend against the configuration script. They keep Ravel itself from holding
or passing on a credential, as the terms of the harnesses require. The data
that a configuration script reads stays untrusted, such as a context document, a file or the
answer of a model. That is why Ravel disables `eval`.

A harness run receives the environment of a managed process. That is the
caller's environment with the same scrub, or the confined environment. An
unconfined run therefore has the owner's `HOME`, and with it the sign-in store
of the harness. An unconfined run also receives an API key that is in the
caller's environment, such as `ANTHROPIC_API_KEY`, as any other caller variable.
Ravel does not scrub it, because a harness may authenticate with it.

The confined environment keeps `HOME`. Confinement makes only the working
directory and the private run directory writable. A harness that writes to its
store can therefore fail, for example when it refreshes a token.

Ravel does not
widen the confinement for a sign-in. Whether a harness can use its sign-in
inside confinement depends on the harness and the platform. The smoke tasks run
unconfined.

#### Sandbox home for real harness runs

Ravel reads its settings from `$HOME/.config/ravel` and has no override. A
harness run receives the caller's `HOME`. A smoke task or a skill trial
therefore cannot run in the owner's `HOME` without writing the owner's
settings.

Each such run uses a sandbox home with its own settings and configuration
script. The sandbox home reuses only the sign-in store of each harness from the
real home. Sign-in and harness configuration have different owners, so no link
joins them.

| Harness | The one linked path in the sandbox home | State that the sandbox home owns |
|---|---|---|
| Claude Code on macOS | `Library/Keychains`, the login keychain that Claude Code finds through `HOME` | `.claude` and `.claude.json` |
| Claude Code on Linux | `.claude/.credentials.json`, the sign-in file | The rest of `.claude`, and `.claude.json` |
| Codex | `.codex/auth.json`. Each run sets `CODEX_HOME` to the sandbox `.codex` | The configuration, hooks, agents, plugins and skills of the sandbox `.codex` |

A link to the whole `.codex` or `.claude` tree is not permitted. It exposes the
real configuration, hooks, plugins and skills. A configuration write in the
sandbox would also change the real files.

The Claude Code rows hold for the default locations only. Claude Code reads
`CLAUDE_CONFIG_DIR` in place of `$HOME/.claude`. That variable also moves the
sign-in. The sign-in file and the macOS Keychain entry then belong to that
directory.

Three more variables add Claude Code plugin locations:
`CLAUDE_CODE_PLUGIN_CACHE_DIR`, `CLAUDE_CODE_PLUGIN_DIRS` and
`CLAUDE_CODE_PLUGIN_SEED_DIR`. Ravel scrubs none of these four names from the
environment of a managed process or a harness run.

A Claude Code run refuses before the harness launches when its inherited
environment holds one of the four names. An inherited value can expose a real configuration
or plugin tree that no link check sees. The remedy is to launch the run without
that variable.

The run does not remove such a variable. After a removal, the linked default
sign-in can belong to another account. The run also does not set
`CLAUDE_CONFIG_DIR` to the sandbox, because that value moves the sign-in away
from the linked path.

Before the harness launches, each run lists every link in the sandbox home. The
list holds only the path of the table for that harness. The run records the
effective configuration directory of the harness. For Claude Code, that is the
sandbox `.claude`. For Codex, it is the effective `CODEX_HOME`, and the run
refuses an inherited value that points into the real home. After the run, it
checks that each linked path is still a link.

A Claude Code sign-in with a claude.ai account also downloads the skills of
that account into the sandbox `.claude/skills/synced`. These skills are
platform-managed residue, not a native copy from the real home. A trial records
them with the other skills that the session lists.

Each wrapper that builds a sandbox home has isolation controls with fake
harnesses. An inherited `CLAUDE_CONFIG_DIR` that names a decoy real directory
must refuse before the fake harness launches. Each plugin-location variable
must refuse in the same way. So must an inherited `CODEX_HOME` in the real
home. `task harness:smoke:test` runs these controls for the smoke tasks. Actual
trials rely on the sandbox only after these controls pass.

A harness can replace the sign-in link with a file when it refreshes a token.
The run then fails. Its remedy is to sign in again in the real home, because
the real store can hold a revoked token.

The smoke tasks `task harness:smoke:claude` and `task harness:smoke:codex` use
this technique. The task reads no sign-in, because the harness opens its own
store. The task also removes the API keys from the environment, so the call
proves the sign-in.

The credential boundary covers the caller variables that `ravel::env` serves. It
is not a filesystem sandbox. A measured read is not confined by path, so a
configuration script that reads a file that holds a credential receives the credential.

The
sign-in file of a harness, such as `~/.codex/auth.json`, can be such a file. On
Linux, the
environment file of a process under `/proc` can hold a credential. That includes
the file of the Ravel CLI itself, which performs the read.

Ravel accepts this exposure, because it trusts the configuration script. A list of refused
paths would not remove the exposure in any case, because the other files and
processes of the same user stay readable. An unconfined harness run that prints
its environment also returns a credential of the caller's environment to the
script in its output.

### Settings and environment grants

Settings remain at `$HOME/.config/ravel/settings.json`, including with an explicit `--config` entry. A missing settings file selects defaults. Without an absolute `HOME`, an explicit entry uses defaults.

A settings file must be a readable regular JSON object. Its only keys are `timeoutMs`, `contextBytes`, `configEnv`, `harnesses`, `budgets` and `skills`. Unknown keys refuse as `settings_invalid`, exit 2.

A member name that repeats in any object of the file refuses the same way. The [skill discovery specification](skill-discovery.md#settings) owns the `skills` object.

No key opens a persistent store. Flags override the timeout and context bounds. `--config-env` adds names to `configEnv`, with duplicates counted once.

Ravel 2.0.0 refuses `--policy-env` as an unknown option and `policyEnv` as an unknown settings key. Neither is an alias.

`configEnv` is an array of environment names. A grant must be nonempty UTF-8 and contain no `=`. Names in `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*` are reserved and refuse as `excluded_grant`, exit 2.

Other malformed grants refuse as `malformed_input`, exit 2. Bun and Node variables are ordinary grantable names after runtime removal. Grants let `ravel::env` read caller values through the Ravel CLI.

They do not populate the Rhai worker's empty environment. Diagnostics report grant names and presence, never values. The six exact controls scrubbed for a managed launch are a separate rule in the session spec.

### Harness entries

The `settings.json` key `harnesses` is a map from a name to an entry. A name has
1 to 64 lowercase letters, digits and dashes. Its first character is a letter or
a digit. An unknown field refuses as `settings_invalid`.

| Field | Type | Meaning |
|---|---|---|
| `program` | String | Required and nonempty. An absolute path, a `PATH` name or a relative path with a separator. Ravel resolves it as it resolves a selected program |
| `capabilities` | Array of nonempty strings | Optional labels that discovery repeats |

A settings problem names its place in the file, such as
`harnesses.claude.program` or `budgets.harnessRuns`, as the `location` of the
refusal.

### Evaluation budgets and their composition

| Bound | Default | Source and range | Breach |
|---|---|---|---|
| Whole evaluation | 30,000 ms | `timeoutMs` or `--timeout-ms`, from 1,000 to 600,000 | `selection_timeout`, exit 124 |
| One harness run | The time that remains | The `timeoutMs` of the run, lowered to the time that remains | The run ends as `timed_out` |
| Delivered context | 262,144 bytes | `contextBytes` or `--context-bytes`, from 1 to 8,388,608 | `context_too_large`, exit 3 |
| One measured read | The smaller of 65,536 bytes and the context budget | `max_bytes`, up to the context budget | `source_too_large`, exit 3 |
| Measured sources | 256 | Fixed | `too_many_sources`, exit 3 |
| Worker messages | 1,048,576 bytes | Fixed | `message_too_large`, exit 3 |
| Configuration diagnostics | 262,144 bytes | Fixed | `output_limit`, exit 3 |
| Harness runs | 0 | `budgets.harnessRuns`, from 0 to 16 | `harness_budget_exhausted`, exit 3 |
| Harness output | 1,048,576 bytes for each stream | Fixed | The run ends as `output_limit` |
| Host calls at one time | 1 | Fixed by the synchronous script | None |
| Engine limits and worker heap | As in the Rhai worker section | Fixed | `script_limit` or `memory_limit`, exit 3 |

The bounds compose by these rules:

- Every budget belongs to one evaluation, which is one `start-session`. A nested
  session in a managed process has budgets of its own.
- The whole deadline runs from the launch of the worker to the judged result. It
  covers the import, `load_context`, `select` and every host call.
- The deadline of a harness run is the smaller of its `timeoutMs` and the time
  that remains. When the whole deadline passes during a run, the evaluation
  ends, not only the run.
- The deadline and the output bound of a harness run belong to that run. When
  Ravel stops a run at one of them, `run_harness` returns `timed_out` or
  `output_limit`, and the script goes on. A breach of any other bound in the
  table ends the evaluation, and the script cannot catch it.
- The run status `output_limit` is data. The refusal `output_limit`, exit 3, is
  the breach of the diagnostics bound.
- A harness run uses one run when the runner launches it. An LLM call is a
  harness run, so it uses one run too.
- A run with no run left ends the evaluation before the runner launches it. The
  script cannot catch it, because the worker hears nothing more.
- Teardown after the deadline or a cancellation adds at most the kill-grace of
  the runner and one second. It does not wait for a read in progress.
- Only the owner sets the harness-run budget, in settings. Callers get no new
  flag. A script can read its budgets but cannot raise one.

### Cancellation and deadlines

An INT, TERM or HUP during the evaluation, or the end of the whole deadline,
stops the evaluation in this order:

1. The Ravel CLI records the first cause. A later cause does not repeat the
   stop.
2.

It abandons a read in progress, and it does not wait for it.
3. It asks the runner to stop a harness run in progress. The runner escalates
   from TERM to KILL, clears the group and reaps the child.
4.

It stops the worker with TERM, kills it if it is still there after one
   second, and reaps it. The worker holds nothing that needs a graceful stop.
5. It removes the private directories, and it reports every cleanup failure.
6.

It reads the cancellation latch again, and then it refuses.

Only one of steps 2 and 3 applies, because one host call runs at a time.

A stop that the runner reports ends within the kill-grace of the runner and one
second. Ravel does not wait for a read in progress. If the runner does not
report, Ravel waits at most the kill-grace and three more seconds. Those seconds
are the runner's bounds for a reap that fails and for the group to disappear.
Ravel then ends the evaluation as `harness_cleanup_failed`.

The final read of the latch applies the precedence of the
[completion contract](result-job-completion.md#outcome-and-diagnostic-order), for every evaluation. A signal that Ravel consumed before that read wins over the deadline.
The refusal is then `selection_cancelled` with 128 plus its number, and Ravel
raises the same signal again. Among signals, the first consumed signal stays the
cause. The report keeps an earlier deadline as the original failure.

Without a signal, the deadline refuses as `selection_timeout`, exit 124. A run
whose group survives, or whose reap fails, replaces the deadline: the refusal is
`harness_cleanup_failed`, exit 5. The same final read follows every other
refusal of the evaluation.

A consumed signal therefore also wins over a breach
of a bound and over a cleanup failure. The report keeps the replaced refusal as
the original failure. No managed process launches after a stop.

The worker also checks a stop flag at each Rhai operation, so a cooperative
script stops at once. The flag is set when the Ravel CLI closes its end of the
private socket. A native call that does not yield stops only when the
worker dies. Examples are `sleep` and the `pad` loop of Rhai 1.27.0. That is why
the worker is a process.

A stop of an LLM call cancels local work only. The model service behind the
harness may finish a request that it already accepted, and it may count that
request against the owner's plan. Ravel cannot withdraw such a request.

### Diagnostics, notices and results

Stdout carries only the final result. The output of a harness run
returns to the script, and it reaches no stream unless the script prints it.
The configuration script's `print` and `debug` output, the notices of Ravel and the runner
reports go to stderr.

The JSON routing notice uses schema version 2:

- `worker` reports `path`, `version`, `buildId` and `rhaiVersion`.
- `config.modules` lists the canonical name, bytes and SHA-256 of the entry and
  of each module. The entry comes first, and each module follows in the order
  of its first import.
- `reads` lists every measured read with its `via`, its phase, its name, its
  bytes and its SHA-256. The `via` is `read_text` or `read_json`, and the phase
  is `load`, `context` or `selection`. A read that failed is no measured read,
  also when its bytes arrived and did not decode.
- `harnessRuns` lists the harness, status, exit code or signal, elapsed time of
  each run, and whether it sent a result. An LLM call is one of these runs. The
  list also holds a call that launched nothing, with the status `unavailable`.
- `bounds` includes `harnessRuns`.
- `skills` reports skill delivery, or `null` when delivery is disabled. The
  [skill discovery specification](skill-discovery.md#diagnostics) defines it.

The JSON failure document uses schema version 2, with the owner refusal code
in `configCode`. Routing notices retain version 2 and report `config` and
`configEnvironment`. The script interface has version 4.

Text diagnostics
use `config stdout:` and `config stderr:`. The former `policy_` prefix on
Ravel-owned codes becomes `config_`, and the former `policy.` prefix on
named script locations becomes `config.`. Owner strings and opaque results
remain unchanged. For an error in a script, its
`location` has the form `LINE:COLUMN`, and its `source` names the file.

### Errors and refusals

The tables below name configuration refusals and worker failures.
A Rhai function returns synchronously, so it cannot leave a promise unsettled.

| Code | Stage | Exit | Meaning |
|---|---|---|---|
| `home_unset` | `authority` | 3 | Default discovery lacks an absolute `HOME` |
| `config_missing` | `authority` | 3 | The selected Rhai entry does not exist |
| `config_unreadable` | `authority` | 3 | Entry admission, reading or extension checks failed |
| `context_unreadable` | `context` | 3 | Ravel cannot read the caller's context document |
| `context_invalid` | `context` | 3 | Caller or loader context failed shape checks |
| `context_too_large` | `context` | 3 | Caller or delivered context exceeds its byte budget |
| `source_too_large` | `context` | 3 | A measured source exceeds its read bound |
| `too_many_sources` | `context` | 3 | The source count exceeds its bound |
| `selection_abstained` | `selection` | 3 | `select` returned `()` |
| `selection_malformed` | `selection` | 3 | The selected or refused map failed shape checks |
| `config_refused` | `context` or `selection` | 3 | The script refused, with its code in `configCode` |
| `message_too_large` | `selection` | 3 | A script result exceeds the message bound |
| `output_limit` | `evaluation` | 3 | Worker diagnostics exceed their combined bound |
| `selection_timeout` | `evaluation` | 124 | The whole evaluation deadline passed |
| `selection_cancelled` | `evaluation` | 128 plus signal | INT, TERM or HUP cancelled evaluation |
| `protocol_error` | `worker` | 5 | The private conversation violated its frame contract |
| `config_format_retired` | `authority` | 3 | The entry is TypeScript or JavaScript. Write a Rhai configuration script with chapter 4 of the book |
| `config_reentry_refused` | `authority` | 3 | `start-session` runs inside a harness run, where it cannot select |
| `config_parse_failed` | `load` | 3 | A syntax error in the entry or a module, with its file, line and column |
| `config_import_failed` | `load` | 3 | A module is missing, unreadable, too large, in a cycle or the 65th, or its top level failed |
| `config_invalid` | `validation` | 3 | An export is missing or has the wrong arity, or the version is blank |
| `unsupported_version` | `validation` or `context` | 3 | Script `SCHEMA_VERSION` is not 4, or the selection context has an unsupported unsigned schema version instead of 1 |
| `selection_threw` | `selection` | 3 | `select` raised an uncaught error. The report has its file, line and column |
| `context_loader_failed` | `context` | 3 | `load_context` raised an uncaught error. The report has its file, line and column |
| `host_request_invalid` | `load`, `context` or `selection` | 3 | A host function received a malformed request |
| `script_limit` | `evaluation` | 3 | An engine limit, which `bound` names |
| `memory_limit` | `evaluation` | 3 | The worker heap reached 1 GiB |
| `harness_budget_exhausted` | `evaluation` | 3 | No harness run remained |
| `harness_cleanup_failed` | `evaluation` | 5 | The group of a harness run survived, or its reap failed |
| `worker_missing` | `worker` | 5 | Ravel cannot re-execute its own binary |
| `worker_identity_mismatch` | `worker` | 5 | The re-executed binary reports another build |
| `worker_failed` | `worker` | 5 | The worker died, broke the protocol or failed its teardown |

Settings problems in the new keys use `settings_invalid`, exit 2. A configuration script
refusal stays `config_refused`, with the
owner's `configCode`. Ravel never selects a fallback after a failure. The
defaults for the harness, the model, the effort and every recovery belong to the
configuration script. The host defaults are the ones in this specification.

### Re-entry and completion authority

A harness run executes its harness directly, through the runner. It never
invokes `start-session`, so it evaluates no configuration script. An LLM call is a harness
run, and the same rule holds for it. A configuration script that wants the route of another
kind for a question calls its own route function. It needs no second selection,
as [a configuration script asks a model through a harness run](../adr/a-configuration-script-asks-a-model-through-a-harness-run.md)
records.

`start-session` refuses as `config_reentry_refused` when `RAVEL_CONFIG_RUN` is in
its environment. It refuses before it reads settings or a configuration script. The final managed process cannot inherit that marker, because this refusal prevents its launch.

Ravel scrubs only the six exact launch controls in the
[managed-process contract](session-execution.md#managed-process-and-result). It does not scrub every `RAVEL_*` name. Each harness run receives its own fresh marker and result path.

Sessions that the final harness nests therefore stay allowed.

The refusal is cooperative. It is isolation by inherited context, as in the
[session execution specification](session-execution.md#managed-process-and-result),
not authentication against code of the same user. A harness run that removes the
marker from its environment escapes the refusal. A Ravel 1.x on `PATH` also
escapes it, because 1.x ignores the marker.

After an escape, the budget and the deadline of the outer evaluation bound only
the harness run itself. The stop of the run reaches the process group of the
run. The managed process of an escaped `start-session` is a job in a group of
its own, so it can outlive that stop.
[The launched child is a job](../adr/the-launched-child-is-a-job.md) puts every
process that leaves its group outside group cleanup.

Completion authority stays separate:

- The result channel of the outer session does not exist during evaluation.
  Ravel allocates it after the selection.
- Each harness run has a fresh channel, and its `RAVEL_EXIT_FILE` names only
  that channel. The value that it sends reaches only the script, as `result`.
- When Ravel itself runs inside an enclosing session, the channel of the run
  replaces the enclosing `RAVEL_EXIT_FILE`. The run cannot complete the
  enclosing session.
- The worker receives no `RAVEL_EXIT_FILE` and runs no program.

### Installed layout and delivery

The installation holds these files:

- `bin/ravel`, the only executable. Its Rhai worker is the same binary in an internal role.
- `share/ravel/examples/`, with `static.rhai`, `sample.rhai` and `llm.rhai`.
- `share/ravel/notices/THIRD-PARTY-NOTICES.md`, the notices for the Rust dependency tree.

`task notices` uses pinned cargo-about 0.8.4 and locked Cargo metadata.
It writes `target/notices/THIRD-PARTY-NOTICES.md` and refuses a licence outside the allowlist.
Installation, archives and the Homebrew formula copy those bytes unchanged.
Rhai lists `smartstring`, which is MPL-2.0, although `compact_str` keeps its code out of the build.
The allowlist records that distinction.

`task install PREFIX=/path` builds and verifies the installation from that prefix alone.
`task smoke` exercises a disposable prefix through the executable directly and through a relative symlink.
Neither path requires a host JavaScript runtime or caller binary.
A Git cache miss of skill delivery needs Git, as the [skill discovery specification](skill-discovery.md#exact-git-acquisition-and-offline-reuse) states.
The book checks use a checkout-local Bun as a development tool, outside the installed product.
The release carries no JavaScript runtime, SDK, declarations or separate worker executable.

`task release:archives` builds unreleased archives for aarch64-apple-darwin,
aarch64-unknown-linux-gnu and x86_64-unknown-linux-gnu.
Each archive contains the installation above, README and LICENSE.
An exact manifest refuses omissions, extras, probes and caller assets.
The Linux build uses Zig's glibc 2.17 target.

`task release:smoke` rebuilds and checks the archives on their target userlands. The macOS target runs natively. Linux targets run in pinned glibc 2.17 userlands.

Floor and above-floor symbol probes check the libc boundary. QEMU models Nehalem and Cortex-A53 for CPU checks. Above-floor instructions must fail with SIGILL at the floor and run under `-cpu max`.

The same direct/symlink installation check runs on every target. Containers and QEMU do not measure a kernel floor.

The Homebrew formula installs the same files and requires no Grove dependency.
Its temporary-prefix test imports the installed Rhai static example and checks exact result bytes through a symlink.
These preparation tasks neither cut nor publish a release.
The [delivery instructions](../book/src/11-deliver-ravel.md) own prerequisites, publication and recovery procedures.

### Migration

Ravel 2.0.0 is a clean cutover with no compatibility period. The precedence of
the entry lets a Grove session that Ravel 1.1.0 launched keep reading
`policy.ts`, while 2.0.0 reads `config.rhai`.

| TypeScript 1.x | Rhai 2.0.0 |
|---|---|
| `~/.config/ravel/policy.ts` | `~/.config/ravel/config.rhai` |
| `export const policy = definePolicy({ … })` | A module with `SCHEMA_VERSION`, `VERSION`, `select` and optionally `load_context` |
| `schemaVersion: 2` | `export const SCHEMA_VERSION = 4;` |
| `version` | `export const VERSION = "…";` |
| `select(request, context, host)` | `fn select(request, context)` |
| `loadContext(request, host)` | `fn load_context(request)` |
| `request.prompt`, the caller's prompt | `request.originalPrompt`. `request.prompt` is the prompt to deliver |
| `host.readText`, `host.readJson` | `ravel::read_text`, `ravel::read_json`, now in every phase |
| `host.diagnostic(text)` | `debug(text)` |
| `host.signal` | Nothing. The worker stops the script itself |
| `readChoice` and `CHOICE_FILE` | A function in the sample, with `ravel::read_text` and `split()` |
| `import … from "ravel/sdk"` | Nothing. The host functions are built in |
| `import … from "ravel/examples/static"` | A copy of `static.rhai`, imported by its path |
| `./module.ts` imports | `import "module" as module;` |
| `node:fs` reads | `ravel::read_text`, which Ravel measures |
| `resolve` and `dirname` from `node:path` | `ravel::path_resolve`, `ravel::path_parent` |
| `randomUUID` from `node:crypto` | `ravel::uuid_v4` |
| `JSON.parse`, `JSON.stringify` | `ravel::json_parse`, `ravel::json_encode` |
| `process.env.NAME` | `ravel::env("NAME")` |
| `fetch` to a model API | `ravel::run_harness` with a harness entry for Claude Code or Codex, which uses its own sign-in |
| Code that launches a process | `ravel::run_harness`, with a harness entry in settings |
| `async` and `await` | Ordinary calls |
| `settings.json` keys | Unchanged, plus `harnesses`, `budgets` and `skills` |

The owner chose a one-off manual conversion of their configuration on
2026-10-26. The book has no migration chapter and assumes no TypeScript
knowledge. Its [chapter 4](../book/src/04-write-a-configuration-script.md) teaches
Rhai configuration script authoring directly. The translated examples and their equivalence
checks remain inputs to the manual conversion. The table above records the
implementation correspondence, outside the book.

Before removal, paired command-line runs compared TypeScript configurations with their Rhai translations over one request table.
The observations covered routing notices, fake-harness argument vectors, delivered contexts and refusals.
Those results are frozen fixtures. Current tests compare Rhai execution with those fixtures and never execute the retired worker.

#### Frozen comparison rules

The comparison checks routing choices, fake-harness argument vectors, delivered context values and refusal classifications.
It normalises sandbox paths. It excludes notice versions, configuration and worker identities, timing, signal reports and fields that only Rhai reports.
Those Rhai fields are measured reads, harness runs and their budget.

Context digests differ across sandbox paths. Each live encoded-byte count is checked against its actual value before path normalisation.
Runtime-specific decoder or system-error messages can differ while their refusal code, stage, exit status and remedy remain equal.

The retained cases require these intentional differences:

- A choice file beyond its read bound ends Rhai evaluation as `source_too_large`, rather than the former `choice_unreadable` refusal.
- U+FEFF and U+0085 have different whitespace classifications in JavaScript and Rhai splitting or trimming.
- An invalid UTF-8 jj store path refuses in Rhai, while TypeScript decoded it with replacement characters.

The tests prove those differences and keep their reasons beside each case.
Frozen TypeScript observations remain unchanged. Passing this comparison does not claim equality of every runtime field or every possible input.


Grove's helpers move in Linkuistics/grove#24. They need these capabilities, and
Ravel's own tests prove each one without Grove code:

- `read_text` for the task file.
- `json_parse` and `json_encode`.
- The string functions of Rhai for the handle and the marker lines.
- `uuid_v4`.
- A module import by path.
- A wrapper that calls the `select` of another module.

### Upgrades

A Rhai upgrade changes one pin. A Taskfile task proves it at the command-line
seam with the engine tests and the hostile classes, and each hostile class keeps
its firing control.

A new release of Claude Code or Codex changes no code of Ravel. Ravel builds no
argument that is specific to a harness, and it parses no output of a harness.
The owner's configuration script holds the recipe of each call. The opt-in smoke tasks make one
real LLM call through each of the two harnesses, with the owner's sign-in. They
are `task harness:smoke:claude` and `task harness:smoke:codex`.

An owner runs them after a harness upgrade to check the recipes that the book
gives. Each call spends subscription usage.

## Requirements

### Requirement: Deterministic selection

Ravel SHALL select from a Rhai configuration script that makes no host call, with no network
access and no harness entry configured.

#### Scenario: Route table

- **WHEN** a configuration script maps `kind` to a harness, a model and an effort in a table
- **THEN** the fake harness receives the argument vector from the table, and
  the notice reports no harness run

### Requirement: Generic LLM interaction

Ravel SHALL let a configuration script build a question, ask a model through a harness run and
interpret the answer. Ravel SHALL build no argument that is specific to a
harness.

#### Scenario: LLM-assisted selection

- **WHEN** `select` runs a fake harness that classifies the prompt, and maps
  its answer to a route
- **THEN** Ravel launches that route, the fake harness ran once as a harness
  run, and it received exactly the arguments and the `stdin` of the script

#### Scenario: Another purpose

- **WHEN** `load_context` runs a fake harness for a summary and returns the
  summary in `facts`
- **THEN** `select` receives the summary in the delivered context

#### Scenario: A structured answer that does not match

- **WHEN** a fake harness prints a JSON answer that lacks a required property,
  and the configuration script checks it with `ravel::json_check` under `try`
- **THEN** the catch block receives `json_mismatch` with the location of the
  property and no part of the answer, and the configuration script goes on

#### Scenario: A failed call and a fallback

- **WHEN** the first fake harness exits 1 with a rate-limit message, and the
  configuration script then runs a second harness
- **THEN** the script sees `exited` with `exitCode` 1, and the session uses the
  answer of the second harness

### Requirement: Measured context and neutral discovery

Ravel SHALL measure and report every read. Discovery SHALL report configuration
and availability only.

#### Scenario: A read in select

- **WHEN** `select` reads a file with `ravel::read_text`
- **THEN** the notice lists the read with its phase, its bytes and its SHA-256

#### Scenario: A FIFO as a source

- **WHEN** `select` calls `ravel::read_text` on a FIFO that no process writes
- **THEN** `read_text` throws `source_not_regular` at once, and the evaluation
  goes on

### Requirement: Direct invocation without re-entry

A harness run SHALL execute its harness without a selection. `start-session`
SHALL refuse inside a harness run.

#### Scenario: Re-entry attempt

- **WHEN** a fake harness in a harness run invokes `ravel start-session`
- **THEN** that invocation exits 3 with `config_reentry_refused`, and the script
  receives the exit status of the run

#### Scenario: Completion attempt

- **WHEN** a fake harness in a harness run sends `end-session other`
- **THEN** the script receives `other` as `result`, and the session returns only
  the value of the managed process

### Requirement: Typed boundaries

Ravel SHALL refuse a malformed selection, context or host request with its
location. It SHALL NOT launch from one.

#### Scenario: Unknown field

- **WHEN** `select` returns a selection with an extra field
- **THEN** Ravel exits 3 with `selection_malformed` and the location of the field

#### Scenario: Wrong argument type

- **WHEN** a configuration script calls `ravel::run_harness` with an integer in `args`
- **THEN** Ravel exits 3 with `host_request_invalid`, and the report names the
  field and the line of the script

### Requirement: Script failures with locations

Ravel SHALL report a parse error and an uncaught runtime error with the file,
the line and the column.

#### Scenario: Syntax error in a module

- **WHEN** an imported module has a syntax error
- **THEN** Ravel exits 3 with `config_parse_failed`, and `source` names that
  module

### Requirement: Harness failures are data

The status of each harness run SHALL reach the script, unless the evaluation
stops during the run. A stop at the deadline or at the output bound of the run
SHALL end only the run. A cleanup failure SHALL end the evaluation.

#### Scenario: Missing program

- **WHEN** a harness entry names a program that is not on `PATH`
- **THEN** `run_harness` returns `unavailable` with `harness_not_found`, and the
  run uses no budget

#### Scenario: Output bound

- **WHEN** a fake harness writes 2 MiB to `stdout`, and the configuration script then selects
  a route
- **THEN** `run_harness` returns `output_limit`, the process group of the run is
  gone, and Ravel launches the route

#### Scenario: Deadline of one run

- **WHEN** a fake harness never exits in a run with a `timeoutMs` of 1,000, and
  the configuration script then selects a route
- **THEN** `run_harness` returns `timed_out`, the process group of the run is
  gone, and Ravel launches the route

### Requirement: Deadlines and cancellation stop the evaluation

At the whole deadline or on cancellation, Ravel SHALL stop the script and the
harness run. It SHALL clear the process group of the run, reap the child of the
run and the worker, and launch nothing. A process that left the group of the run
is outside this stop. Ravel SHALL NOT wait for a read in progress. A consumed
signal SHALL win over the deadline.

#### Scenario: A native call that does not yield

- **WHEN** `select` calls `sleep(600)`
- **THEN** Ravel exits 124 with `selection_timeout` within the deadline and its
  teardown bound

#### Scenario: A stalled harness

- **WHEN** a fake harness in a harness run never exits
- **THEN** Ravel exits 124 with `selection_timeout`, and neither the harness nor
  its child exists afterwards

#### Scenario: Descendants of a harness run

- **WHEN** Ravel receives TERM while a fake harness and its child sleep
- **THEN** Ravel exits 143, and neither process exists afterwards

#### Scenario: A signal after the deadline

- **WHEN** the deadline passes during a harness run whose fake harness ignores
  TERM, and Ravel receives TERM before the kill-grace ends
- **THEN** Ravel exits 143 with `selection_cancelled` and raises TERM, and the
  report keeps `selection_timeout` as the original failure

### Requirement: Resource exhaustion

Ravel SHALL end the evaluation at a breach of each bound in the
[budget table](#evaluation-budgets-and-their-composition). The deadline and the
output bound of one harness run are the exceptions. The script SHALL NOT catch a
breach with `try`.
That holds for a breach in the script itself and for a call-depth, data-size or
variable breach inside the call of a function pointer. A breach of the
operations bound or of the heap bound SHALL end the evaluation, however the
script handles errors.

#### Scenario: Caught budget breach

- **WHEN** a configuration script with `budgets.harnessRuns` of 1 wraps a second harness run
  in `try`
- **THEN** Ravel exits 3 with `harness_budget_exhausted`

#### Scenario: Memory

- **WHEN** a configuration script grows an array of strings without end
- **THEN** Ravel exits 3 with `memory_limit` or `script_limit`, and the Ravel CLI
  survives to report it

#### Scenario: A breach inside a function pointer under `try`

- **WHEN** a configuration script wraps `[1].map(|x| ..)` in `try`, and the closure breaches
  the call depth, a data size or the variable bound
- **THEN** Ravel exits 3 with `script_limit`, and nothing launches

#### Scenario: An ordinary error inside a function pointer under `try`

- **WHEN** a configuration script wraps `[1].map(|x| throw "boom")` in `try`
- **THEN** the catch block runs with the thrown value, and the configuration script goes on

#### Scenario: A breach inside a comparer

- **WHEN** a configuration script recurses past the call depth inside the comparer of `sort`,
  `order`, `order_by` or `dedup`
- **THEN** Ravel exits 3 with `script_limit`, and nothing launches

### Requirement: Credential boundary

Ravel SHALL hold no model credential. A harness run SHALL receive the environment
of a managed process. `ravel::env` SHALL return no caller variable outside the
readable names and the grants. Ravel SHALL remove no credential from a measured
read or from the output of a harness run, because it trusts the configuration script.

#### Scenario: Leak probes

- **WHEN** the caller's environment holds a sentinel `ANTHROPIC_API_KEY` that
  no grant names
- **AND** a script that makes no measured read and runs no harness calls
  `ravel::env` with that name and prints every other value that it can reach
- **THEN** `env` throws `env_not_granted`, and the sentinel appears in no
  diagnostic or notice

#### Scenario: A harness finds its own sign-in

- **WHEN** a fake harness in an unconfined harness run prints its `HOME` and the
  value of the same ungranted sentinel
- **THEN** the script receives the caller's `HOME` and the sentinel, both
  unchanged

### Requirement: Migration equivalence

Each translated configuration SHALL satisfy the [frozen comparison rules](#frozen-comparison-rules) for choices, arguments, contexts and refusals.
The comparison SHALL preserve its exclusions and prove each intentional difference.

#### Scenario: The owner-shaped configuration script

- **WHEN** the translation of the owner-shaped configuration script runs in a secondary jj
  workspace for every kind that it lists
- **THEN** each argument vector equals its frozen TypeScript observation, including
  the `--add-dir` grant of the main repository

### Requirement: Entry precedence

Ravel SHALL evaluate `config.rhai` when it exists. It SHALL refuse a lone
`policy.ts` with the migration remedy.

#### Scenario: Both files

- **WHEN** both `config.rhai` and `policy.ts` exist in the configuration
  directory
- **THEN** Ravel evaluates `config.rhai` and does not open `policy.ts`

### Requirement: Request schema 4

Ravel SHALL evaluate only a configuration script that exports `SCHEMA_VERSION`
4. With skill delivery disabled, `request.prompt` SHALL equal
`request.originalPrompt`.

#### Scenario: A schema-3 script

- **WHEN** a configuration script exports `SCHEMA_VERSION` 3, and no settings
  file exists
- **THEN** Ravel exits 3 with `unsupported_version`, and the remedy names
  `originalPrompt`

#### Scenario: Disabled delivery

- **WHEN** a schema-4 script prints the request, and the settings hold no
  `skills` key
- **THEN** `prompt` and `originalPrompt` hold the caller's bytes, and `skills`
  is `()`

## Test seams

The owner agreed one command-line seam in the requirements session. Acceptance tests run the real `ravel start-session` against a sandbox home that
holds `config.rhai` and `settings.json`. Fake harness scripts on a private `PATH`
stand behind it, as the existing tests use. They serve the managed process and
the harness runs, and with them the LLM calls. A fake harness for an LLM call
prints a scripted answer, and it records its arguments and its `stdin` for the
test.

Acceptance tests use the real Rhai worker and runner. Opt-in Taskfile smoke
tasks make one real LLM call through Claude Code and one through Codex, with the
owner's sign-in. The default test run never includes them.

Hostile checks retain their applicable firing controls. Configuration scripts at the same seam
test the pure functions, such as `json_encode`, `json_check` and `path_resolve`. Engine unit tests supplement this seam.

No fake model runs inside the binary.

The seam cannot make the kernel hold a read. The nonblocking open removes the
stalls that a test can make, such as a FIFO. The FIFO case proves the open, and
its firing control is a blocking open of the same FIFO, which stalls. No test at
the seam proves that a stop never waits for a read. That property rests on the
reader thread and the stop order in this specification.

A signal after the deadline needs a teardown that lasts. On Linux, a test stops
the worker. A stopped worker takes the TERM of the teardown only when it
continues, so the Ravel CLI waits out its grace. macOS ends a stopped process on
TERM. The case therefore runs on Linux only, and on macOS the stop order rests
on the unit tests of the supervision.

## Out of scope

- A verb that evaluates a configuration script without a launch. The owner kept it out of this
  workstream.
- A call from Ravel itself to a model service, through Rig or another HTTP
  client. The owner removed it, because a subscription sign-in works only in
  its own harness.
- A nested `start-session` inside a harness run. A configuration script calls its own route
  function instead.
- Arguments or output parsing that are specific to a harness. The owner's
  configuration script holds each recipe.
- Usage accounting and a token budget for an LLM call. Ravel does not read the
  output of a harness.
- Streaming answers to a script.
- More than one host call at a time.
- Live model lists.
- Modules embedded in Ravel, such as a successor to `ravel/sdk`.
- A regular-expression host function. The string functions of Rhai parse a
  handle and a marker line, and the tests prove it.
- A host function that takes a URL or a credential. A script reaches a model
  only through a harness run.
- A filesystem sandbox. A measured read can read each file that the user can
  read.
- A compatibility period for TypeScript.
- The port of Grove's helpers, which Linkuistics/grove#24 carries.
- A port of the book checks that run on Bun.
