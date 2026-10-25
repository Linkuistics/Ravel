# Policy evaluation

This specification describes how Ravel 2.0.0 evaluates an owner policy written
in Rhai. It covers the policy entry, the policy worker, the host capabilities,
the evaluation budgets, cancellation, the typed boundaries, the errors and the
migration from TypeScript. The [session execution specification](session-execution.md)
describes the released 1.x path. Its policy sections stay current until the
2.0.0 cutover, and the work that removes the TypeScript path moves them here.

Three decisions bind this area:
[the owner policy is a Rhai script](../adr/owner-policy-is-a-rhai-script.md),
[policy evaluation precedes the launch](../adr/policy-evaluation-precedes-the-launch.md)
and [owner settings name the LLM endpoints](../adr/owner-settings-name-the-llm-endpoints.md).
The decision that [the launched child is a job](../adr/the-launched-child-is-a-job.md)
governs each harness run. The [glossary](../../CONTEXT.md) defines the terms.
GitHub issue 2 states the requirements and the acceptance criteria. The
[design views](../design/rhai-policy/README.md) draw the processes, the
evaluation sequence and the paths that stop an evaluation.

## Problem

A Ravel 1.x owner policy is TypeScript. A compiled Bun worker evaluates it, so
each release carries a JavaScript runtime, its notices, its startup controls
and its target checks. Most policies are small tables that choose a harness, a
model and an effort. They need none of that runtime.

Owners also want a policy to ask a model a question, for example to classify a
task before it chooses a route. A 1.x policy can do that only with its own
network code and its own credential. Ravel then controls neither the endpoint,
the credential, the deadline nor the cleanup.

## Solution

An owner policy becomes a Rhai module. Ravel evaluates it in a policy worker,
which is Ravel's own binary in an internal role. The worker only computes. It
holds no credential, reads no file and launches no process.

The policy asks the Ravel CLI for each host capability: measured reads, data
helpers, discovery, LLM calls and harness runs. The Ravel CLI performs each one
and returns data. It makes LLM calls through Rig, to LLM endpoints that the
owner names in settings. It runs a harness through the runner, as a job with a
result channel of its own.

Every outcome of a host call is data that the script interprets. Every breach of
a bound ends the evaluation. The script cannot catch a breach with `try`. That
holds when the breach happens inside a function pointer, such as the closure
that `map` runs.

The Ravel CLI kills
the worker at the deadline or on cancellation. It stops the host work in
progress, and nothing launches. On the path to a launch, each host call ends
before the worker sends its result. The Ravel CLI reaps the worker before the
managed process launches.

Callers do not change. The `start-session` flags, the policy context schema, the
result channel and the selection fields keep their meaning.

## Decisions

### Versions and toolchain

| Component | Pin | Role |
|---|---|---|
| Rust | 1.99.1, edition 2024 | The workspace floor and the toolchain file. It is the newest stable release at design time |
| Rhai | `=1.27.0`, with the `compact_str` feature | The policy language in the worker |
| `rig-core`, `rig-http`, `rig-reqwest` | `=0.45.0` each | LLM calls in the Ravel CLI |
| `reqwest` | 0.13, through `rig-reqwest` | The HTTP client that Ravel builds itself |
| `rig-cassette` | `=0.45.0`, for development only | Recorded provider exchanges for tests |
| Tokio | 1, a current-thread runtime | The asynchronous runtime of the evaluation phase |

The design relies only on Rhai and Rig interfaces that it checked against the
source of each pinned release. The Rhai build never enables `unchecked`, because
that feature removes every limit. Rig 0.45.0 needs Rust 1.97.1. The owner chose
the newest stable Rust instead of that floor, and a later release may raise it
again through the same upgrade checks.

Rhai 1.27.1 is not released. It removes the endless loop of `pad` with an empty
padding string (rhaiscript/rhai#1195). It also restores the imports of a closure
that a host calls after the script ends (rhaiscript/rhai#1193). Ravel
calls named functions and holds no closure, so the second issue does not reach
it. A test guards the case.

Rhai's default limits differ between debug and release builds. In a debug
build, the call depth and the two expression depths are 8, 32 and 16. A release
build has 64, 64 and 32. Ravel sets every limit itself, so tests and releases
agree.

### Policy entry and discovery

The default entry is `$HOME/.config/ravel/policy.rhai`, and `HOME` must be
absolute. `--config PATH` names another entry, relative to the caller's working
directory. There is no search, no environment override and no fallback.

| Entry state | Result |
|---|---|
| `policy.rhai` exists | Ravel evaluates it, and it does not open `policy.ts` |
| Only `policy.ts` exists | `policy_format_retired`, exit 3. The remedy names `policy.rhai` and the migration guide |
| Neither file exists | `policy_missing`, exit 3. The message names `policy.rhai` |
| `--config` names a `.rhai` file | Ravel evaluates it |
| `--config` names a `.ts`, `.mts`, `.cts`, `.js`, `.mjs` or `.cjs` file | `policy_format_retired`, exit 3 |
| `--config` names any other file | `policy_unreadable`, exit 3. The remedy asks for a `.rhai` file |

The entry must canonicalise to a readable regular UTF-8 file of at most 1 MiB.
Ravel admits a symlink by its real file. Imports then resolve from the directory
of that real file. Settings stay at `$HOME/.config/ravel/settings.json`, also
with `--config`.

Until the increment that removes the TypeScript path, main keeps both paths.
There the extension of the entry selects the path, and the default entry prefers
`policy.rhai`. The released 2.0.0 has only the Rhai path.

A module import has the form `import "PATH" as NAME;`. A relative path resolves
from the directory of the importing file. Ravel appends `.rhai` when the path
has no extension, and it accepts an absolute path. The Ravel CLI reads each
module with the file rules of the entry, and each module loads once per
evaluation. A missing or unreadable module, a cycle or a 65th module refuses as
`policy_import_failed`. The bound counts distinct files. An import of a loaded
module is no new module, however many import statements run.

Ravel measures the entry and every module. The routing notice lists each with
its canonical name, its byte count and its SHA-256. Ravel 2.0.0 embeds no module
of its own, so every import names a file.

### The policy module

| Export | Required | Meaning |
|---|---|---|
| `SCHEMA_VERSION` | Yes | The constant 3. Another value refuses as `unsupported_version` |
| `VERSION` | Yes | A nonblank string that the owner maintains. The notice reports it |
| `fn select(request, context)` | Yes | It returns a selection or a refusal |
| `fn load_context(request)` | No | It returns a policy context or a refusal |

Ravel evaluates the entry as a module. Its top-level statements run once, before
any of its functions. A missing export, a wrong arity or a blank version refuses
as `policy_invalid`. Other functions and constants are helpers of the policy. A
function reads a top-level constant through the `global::` namespace, such as
`global::TABLE`. Rhai functions do not see the variables around them. A wrapper
policy imports another module and calls its `select`. It can change the
selection that comes back.

The engine runs in a strict mode:

- An undefined variable fails when the script compiles.
- An absent map property fails with a source location, so a mistyped field
  cannot read as `()`.
- Ravel disables `eval`. A policy cannot run text as code, such as the answer
  of a model.
- `print` writes to the captured policy stdout. `debug` writes to the captured
  policy stderr.

### Request, context and selection

The request is a map. Every documented field is present. An optional field that
the caller did not supply holds `()`.

| Field | Type | Meaning |
|---|---|---|
| `schemaVersion` | The integer 3 | The version of this request shape |
| `kind` | String | The caller's kind |
| `prompt` | String | The caller's prompt, unchanged |
| `cwd` | String | The caller's working directory, as data |
| `params` | Map of strings | Each `--param NAME=VALUE` |
| `taskFile` | String or `()` | The caller's `--task-file`, absolute. Ravel does not read it |
| `taskId` | String or `()` | The caller's `--task-id` |
| `context` | Map or `()` | The caller's `--context` document, checked |
| `limits` | Map | The effective bounds, for reading |

`limits` holds `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes`, `diagnosticsBytes`, `llmCalls`, `llmOutputTokens` and
`harnessRuns`. A policy cannot raise any of them. Map keys keep the JSON names of
the 1.x contract, so a value converts to JSON unchanged. Host functions use the
snake-case names of Rhai.

The policy context keeps schema version 1, its fields and its checks. Without
`load_context`, `select` receives the caller's context with its measurement, or
`()`. With `load_context`, `select` receives what it returns, with the measured
sources attached. A `load_context` that returns `()` refuses as
`context_invalid`. One that returns a refusal refuses as `policy_refused`, and
`select` does not run.

`select` returns one of two maps:

| Map | Fields |
|---|---|
| Selected | `status: "selected"`, `program`, `args`, `provider`, `model`, `effort` and `reason` |
| Refused | `status: "refused"`, `code`, `message` and `remedy` |

The 1.x selection checks apply unchanged. The strings are nonblank, `args` is an
array of strings with no NUL, and no unknown field is present. A return of `()`
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

### The policy worker

The Ravel CLI launches the worker when it re-executes its own binary in an
internal role. On Linux it executes the running image itself. On macOS it
executes the canonical path that it resolved when it launched. In both cases the
worker reports the build identity of its binary. The Ravel CLI
refuses another identity as `worker_identity_mismatch`, and a worker that cannot
launch as `worker_missing`.

The 1.x worker hygiene stays. The worker gets a private mode-0700 empty
directory, null stdin, an empty environment and the private socket at fd 3. It
inherits no other descriptor. The internal role accepts no command-line input,
and without the expected socket and handshake it exits with status 2. It cannot
launch a process or read a file, so it offers no bypass of selection.

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
| Modules | 64 | `policy_import_failed` |
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
cannot catch as it is. It wraps every other error as before. So an ordinary
`throw` in a closure is still caught. A call-depth, data-size or variable breach
ends the evaluation.

The replaced functions are the 16 call sites of `call_raw_with_extra_args` in
Rhai 1.27.0. Those are `for_each`, `map`, `filter`, `find`, `find_map`,
`index_of`, `some`, `all`, `reduce`, `reduce_rev`, `zip`, `drain` and `retain`
of an array, and `map`, `filter`, `drain` and `retain` of an object map. They
include the deprecated forms that name a function with a string, such as
`map("f")` and `drain("f")`. Rhai 1.27.0 also drops the error of a comparer
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
its private directory before the managed process launches, as in 1.x.

### The bridge between the script and the CLI

A host function is a blocking call in the worker. It sends one request to the
Ravel CLI over the private socket and waits for the reply. The script therefore
sees only synchronous calls. It conducts several interactions in sequence, and it
keeps any message history itself.

The Ravel CLI serves the socket from a Tokio current-thread runtime that exists
only for the evaluation phase. While a host call runs, the runtime keeps
watching the deadline, the cancellation latch, the health of the worker and the
output bounds. A long LLM call, read or harness run therefore never blocks the
work that stops it.

The runtime thread makes no filesystem call. A reader thread of the Ravel CLI
does the filesystem work of the host functions. That work is the reads of the
entry, of each module and of each measured source. It also includes the program
lookups of discovery and of a harness run. The Ravel CLI reads the entry after it
launches the worker, so the whole deadline covers every read.

The runtime waits for the reply of the reader as it waits for any host call. The
reader is not a blocking task of the runtime, because Tokio cannot stop a running
blocking task, and its shutdown waits for one.

Ravel retires the runtime without waiting for a blocking task. The HTTP client
looks up a host name with `getaddrinfo` in such a task, and Tokio cannot stop it.
A lookup that stalls would otherwise hold the Ravel CLI after a stop. The thread
of the lookup then ends with the process. No blocking task exists on the path to
a launch, because no host call is in progress there.

Only one host call runs at a time, because the script is synchronous. The
worker sends its result only after the reply to its last host call. A message
from the worker while a host call is in progress breaks the protocol, and the
evaluation refuses as `worker_failed`. On the path to a launch, no host call is
therefore in progress. The runtime shuts down with all of its tasks finished
before the managed process launches.

Each message from the worker is at most `messageBytes`, 1 MiB, except a
delivered context, which has the context budget. A reply from the Ravel CLI has
the bound of its capability.

A stop does not wait for a read in progress. The Ravel CLI abandons the read and
its reply, and nothing launches after the stop. The reader thread keeps the
descriptor of that read, and it closes it when the call returns or the Ravel CLI
exits. The thread starts with INT, TERM and HUP blocked, as the readers of the
worker output do. It can outlive the stop through the finalisation of the Ravel
CLI, and it must not take a signal there. Ravel opens every descriptor
close-on-exec, so no harness run inherits one. These rules bound the waiting of
Ravel. As in the [native-entry contract](result-job-completion.md#checked-native-entry-transitions),
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
| `ravel::uuid_v4()` | A string | Nothing | Nothing |
| `ravel::path_resolve(base, path)` | A string | A thrown `path_not_absolute` | Nothing |
| `ravel::path_parent(path)` | A string | A thrown `path_not_absolute` | Nothing |
| `ravel::env(name)` | A string or `()` | A thrown `env_not_granted` or `env_not_utf8` | Nothing |
| `ravel::endpoints()` | An array of maps | Nothing | Nothing |
| `ravel::harnesses()` | An array of maps | Nothing | Nothing |
| `ravel::remaining()` | A map | Nothing | Nothing |
| `ravel::llm(request)` | A response map | `status: "failed"` | A budget, the deadline, cancellation, `host_request_invalid` |
| `ravel::run_harness(request)` | A result map | A `status` other than `exited` | A budget, the deadline, cancellation, `host_request_invalid`, `harness_cleanup_failed` |

A thrown map has `code`, `message` and, where it applies, `name`. The script can
catch it. A malformed request to any host function ends the evaluation as
`host_request_invalid`. Its location names the function, the field and the line
in the script.

#### Measured reads

A read resolves a relative path against `request.cwd`. It reads a regular UTF-8
file once. It opens the file without blocking, as in 1.x, so a FIFO cannot hold
the read. It checks that the open file is a regular file, and it reads at most
one byte beyond its bound.

A read returns the content with a source record: the canonical `name`, the
`bytes` that it read and their `sha256`. `max_bytes` defaults to `sourceBytes`
and may rise to `contextBytes`. The bound of 256 sources counts the caller's
context document and every read in the evaluation. A read counts once the file
gave its bytes, whether or not they decode. A file that is not UTF-8, is not JSON or
nests too deep is therefore a source, and a script cannot retry it without
limit. A read that finds no file to read is no source.

`read_json` returns a value that nests at most 100 levels, the bound of the
conversion rule. A deeper file is a data failure, `source_invalid_json`. Its
message names the first level past the bound.

Reads that `load_context` makes enter the `measured` list of the delivered
context, as in 1.x. The caller's document comes first, when there is one. Ravel
also measures the reads that `select` and the top level make, and the notice
lists every read with its phase. A breach of a read
bound refuses even if the script catches the error. The 1.x choice-file helper
becomes a few lines of the shipped sample, with `read_text` and `split()`.

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
members that `json_encode` wrote. The `to_json` of Rhai writes strings with Rust
debug escapes, which are not JSON for control characters. The `parse_json` of
Rhai reads Rhai syntax, so it accepts a trailing comma and a single-quoted
string. Neither is part of the contract of Ravel.

A value that has no JSON form throws `json_unencodable`, and its message names
the location, such as `value.args[2]`. A text that is not strict JSON throws
`json_invalid`. Strict JSON has no trailing comma, comment, single-quoted
string, `NaN` or text after the value. `json_parse` reads an integer outside
the signed 64-bit range as a float, and it throws `json_invalid` for a value
that nests deeper than 100 levels, the bound of the conversion rule.

The data helpers run in the worker and send no message. `json_encode` may write
any string that Rhai can hold, which is more than one message of the worker
holds.

`uuid_v4` returns a random version 4 UUID in lowercase hexadecimal. It takes its
randomness from the operating system.

`path_resolve` and `path_parent` are lexical, and they read no file.
`path_resolve` joins a relative path to an absolute base. It then removes `.` and
empty segments, and it applies each `..` to the segment before it, never above
`/`. An absolute path replaces the base. `path_parent` removes the trailing
slashes and the last segment, and the parent of `/` is `/`. For absolute inputs,
both follow the rules of the `path.resolve` and `path.dirname` of Node, which
the migration of the owner's policy needs. That includes the cases where
`dirname` does not normalise: the parent of `/a//b` is `/a/`, the parent of
`/a/./b` is `/a/.`, and the parent of `//a` is `//`. A relative `base` or a
relative `path_parent` argument throws `path_not_absolute`, whatever the other
argument is.

`env` returns the caller's value of a readable name, or `()` when that name is
unset. The readable names are `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*` and the
names that `policyEnv` or `--policy-env` grant. Any other name throws
`env_not_granted`, whether or not the caller has set it, and the message never
holds a value. A value that is not UTF-8 throws `env_not_utf8`, because a Rhai
string holds UTF-8 only. No readable name is the credential variable of an LLM
endpoint, by the [credential rules](#llm-endpoints-and-credentials). The worker
itself has no environment.

#### Discovery

`endpoints()` returns one map for each configured LLM endpoint, in name order.
Each map has `name`, `type`, `available`, `reason`, `models` and
`structuredOutput`. `available` is false when the credential variable is unset
or empty, and `reason` is then `credential_unset`. An `openai-compatible`
endpoint with no credential variable is available. `structuredOutput` is true
for `anthropic` and `openai`, and it is the owner's setting for
`openai-compatible`. `models` is the owner's list, or an empty array, which
means that Ravel passes any model name.

`harnesses()` returns one map for each harness entry, in name order. Each map
has `name`, `program`, `available`, `resolved`, `reason` and `capabilities`.
Ravel resolves the program as it resolves a selected program, and it runs
nothing. `reason` is `not_found` or `not_executable` when `available` is false.
`resolved` is the path that the lookup found, and `()` when `available` is
false. `reason` is `()` when `available` is true. `capabilities` repeats the
labels of the owner, and Ravel gives them no meaning.

`remaining()` returns `ms`, `llmCalls`, `llmOutputTokens`, `harnessRuns` and
`sources`: what the evaluation still has. `ms` is the time to the whole
deadline, and `sources` is the bound of 256 less the caller's document and every
read that arrived. Discovery uses no network, and it reports availability, never
a recommendation. Rig does not offer a model list
that every endpoint supports, so Ravel lists only the models that the owner
names.

#### LLM calls

`ravel::llm(request)` sends one completion request through an LLM endpoint and
returns its response.

| Field | Type | Required | Meaning |
|---|---|---|---|
| `endpoint` | String | Yes | The name of an LLM endpoint in settings |
| `model` | String | Yes | Ravel passes it unchanged. When the endpoint lists models, it must be one of them |
| `messages` | Array | Yes | One or more `#{ role, content }` maps. `role` is `system`, `user` or `assistant`, and the last role is `user`. `content` is a string |
| `maxTokens` | Integer | No | 1 or more. The default is 1,024. Ravel lowers it to the output tokens that remain |
| `temperature` | Float | No | From 0.0 to 2.0 |
| `reasoning` | String | No | `off`, `minimal`, `low`, `medium`, `high`, `xhigh` or `max` |
| `schema` | Map | No | A JSON Schema in the subset below. It asks for structured output |
| `timeoutMs` | Integer | No | The deadline of this call. Ravel lowers it to the time that remains |

An unknown field ends the evaluation as `host_request_invalid`. The request maps
onto the `CompletionRequest` of Rig:

- The messages become its `chat_history`.
- `maxTokens` becomes `max_tokens`, and `temperature` becomes `temperature`.
- `reasoning` becomes the portable `GenerationOptions.reasoning`.
- `schema` becomes `output_schema`.

Ravel leaves `on_unsupported` unset, so Rig refuses an option that the wire or
the model cannot honour. Ravel sends no tools, no documents and no provider
options.

| Field | When `status` is `ok` | When `status` is `failed` |
|---|---|---|
| `status` | `ok` | `failed` |
| `endpoint`, `model` | As requested | As requested |
| `text` | The text blocks, joined in order | `()` |
| `json` | The checked value when the request had a `schema`, else `()` | `()` |
| `finishReason` | `stop`, `length` or `other` | `()` |
| `usage` | `#{ inputTokens, outputTokens, totalTokens }`, each an integer or `()` | `()` |
| `reportedModel`, `responseId` | A string or `()` | `()` |
| `requestId` | A string or `()` | A string or `()` |
| `elapsedMs` | An integer | An integer |
| `error` | `()` | `#{ code, message, retryable, httpStatus, retryAfterMs }` |

| `error.code` | Cause | `retryable` |
|---|---|---|
| `unknown_endpoint` | Settings name no such endpoint | False |
| `endpoint_unavailable` | Its credential variable is unset or empty | False |
| `model_not_offered` | The endpoint lists models, and this model is not one of them | False |
| `unsupported_capability` | A schema for an endpoint without structured output | False |
| `unsupported_parameter` | The `UnsupportedOption` of Rig: the wire or the model cannot honour an option | False |
| `authentication` | The endpoint answered 401 or 403 | False |
| `rate_limited` | The endpoint answered 429. `retryAfterMs` holds its wait when it gave one | True |
| `provider_error` | Any other reply that reports a failure, or a turn that the provider ended with an error | As Rig classifies it |
| `network` | A transport failure with no reply | As Rig classifies it |
| `timeout` | The call reached its own deadline | True |
| `malformed_response` | A reply that does not decode, that stops short, that has no text, or whose JSON does not match the schema | True only when it stops short |
| `content_filtered` | The provider ended the turn with its content filter | False |
| `output_limit` | The text is longer than 1 MiB, or a successful response body is longer than 8 MiB or holds more than 262,144 JSON values | False |

Ravel makes no retry. The script decides whether to try again, with which
endpoint and which model, and how often. A call that fails before Ravel sends
it uses no budget. Those failures are the first four codes and
`unsupported_parameter`, because Rig refuses that option before it opens a
connection.

Messages and answers stay out of the notices and logs of Ravel. The notice
reports the endpoint, model, status, code, usage and elapsed time of each call.
A policy that wants the text in its diagnostics prints it. Ravel offers no
streaming. The script waits for the whole answer in any case, and a whole answer
keeps the output bound simple.

The schema subset for structured output:

| Keyword | Accepted value |
|---|---|
| `type` | One of `object`, `array`, `string`, `number`, `integer`, `boolean` and `null`, or an array of them |
| `properties` | A map from a name to a schema |
| `required` | An array of property names |
| `additionalProperties` | A boolean |
| `items` | A schema |
| `enum` | An array of strings, numbers, booleans or nulls |
| `title`, `description` | Strings. The check ignores them |

Any other keyword, or a nesting deeper than 32 levels, ends the evaluation as
`host_request_invalid`. Ravel parses the whole answer, trimmed, as JSON, and
checks it against the schema. A mismatch fails the call as `malformed_response`,
with the location in its message. A schema checks shape only, and the script
decides what the value means.

Ravel sends the schema natively where the endpoint supports structured output.
Rig 0.45.0 drops the schema with only a warning on a Chat Completions dialect
without structured output. Ravel therefore never sends a schema to such an
endpoint, and it fails the call as `unsupported_capability`.

On each of the three wires, Rig 0.45.0 rewrites the schema before it sends it.
It marks every property as required. It also adds `additionalProperties: false`
to each object that does not set the keyword. Ravel checks the answer against
the schema of the script, not against the rewritten schema. An endpoint can
still refuse a schema that the subset admits, such as one with
`additionalProperties: true`. The call then fails as `provider_error`, with the
status of the reply.

#### Harness runs

`ravel::run_harness(request)` runs a harness that settings name, through the
runner, and returns what happened. It never evaluates a policy.

| Field | Type | Required | Meaning |
|---|---|---|---|
| `harness` | String | Yes | The name of a harness entry in settings |
| `args` | Array of strings | Yes | The arguments after the program of the entry. No string holds a NUL |
| `stdin` | String | No | Text for standard input. Without it, standard input is at end of file |
| `timeoutMs` | Integer | No | The deadline of this run. Ravel lowers it to the time that remains |

The program of the run is the program of the entry, and the script supplies the
arguments. That is the 1.x `program` and `args` contract, with the program fixed
by the owner. The script decides where a prompt goes: into an argument or into
`stdin`. Ravel builds no argument that is specific to a harness.

A harness run has these properties:

- It runs noninteractively, in a new POSIX session, with no controlling
  terminal.
- Its working directory is `request.cwd`.
- It receives the environment of a managed process. That is the caller's
  environment with the same scrub, or the confined environment.
- It receives `RAVEL_EXIT_FILE` for a fresh result channel of its own, and the
  marker `RAVEL_POLICY_RUN`.
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

### LLM endpoints and credentials

The `settings.json` key `endpoints` is a map from a name to an entry. A name has
1 to 64 lowercase letters, digits and dashes. Its first character is a letter
or a digit. An unknown field refuses as `settings_invalid`.

| Field | Values | Meaning |
|---|---|---|
| `type` | `anthropic`, `openai` or `openai-compatible` | The API type |
| `apiKeyEnv` | String | The name of the environment variable that holds the credential. It is nonempty and has no `=`. It is required except for `openai-compatible`, where an endpoint with no `apiKeyEnv` needs no credential |
| `baseUrl` | String | Optional for `anthropic` and `openai`. Required for `openai-compatible` |
| `structuredOutput` | Boolean | Only for `openai-compatible`. The default is false |
| `models` | Array of nonempty strings | Optional. When it is present and not empty, calls can name only these models |

| `type` | Rig client and wire | Default base URL | Structured output |
|---|---|---|---|
| `anthropic` | `AnthropicConfig`, the Messages wire | `https://api.anthropic.com` | Yes |
| `openai` | `OpenAIConfig` with the OpenAI dialect, the Responses wire | `https://api.openai.com/v1` | Yes |
| `openai-compatible` | `OpenAIConfig` with a Ravel dialect from the baseline quirks of Rig, the Chat Completions wire | None | When `structuredOutput` is true |

Ravel builds each client with `connect`, on an HTTP client of its own. That
client implements the `HttpClientExt` interface of Rig over a reqwest client
that Ravel built. Ravel never uses the shared client or the environment
constructors of Rig.
No ambient `*_BASE_URL` or `*_API_KEY` variable therefore takes part. A base URL
uses `https`, or `http` to a loopback address: `localhost`, 127.0.0.0/8 or
`::1`. It has no user information, no query and no fragment.

The credential follows these rules:

- Only the Ravel CLI reads the named variable, at the time of each call.
- No host function returns it to the worker. Ravel writes it to no diagnostic,
  notice or log.
- Rig carries it as a `Secret`, which never serialises.
- It reaches only the base URL of its endpoint, in the header of its API type.
- A name that an endpoint uses as `apiKeyEnv` cannot also be a grant. A
  `policyEnv` grant or a `--policy-env` flag that names it refuses as
  `excluded_grant`, exit 2.
- An `apiKeyEnv` cannot name a variable that `env` reads without a grant:
  `HOME`, `PATH`, `TMPDIR`, `LANG` or a name of the form `LC_*`. Settings that
  try refuse as `settings_invalid`, exit 2.

A script names an endpoint and never a URL or a credential. It therefore cannot
send a credential to an endpoint that the owner did not configure.

These rules govern the path of an LLM call. An unconfined harness run and the
managed process keep the caller's environment, as in 1.x. When the credential
variable is in that environment, they receive it as any other caller variable.
Ravel does not scrub it, because a harness may authenticate with it. A confined
run or process receives only the confined environment, which excludes it.

The credential boundary covers what Ravel serves to the script. It is not a
filesystem sandbox. A measured read is not confined by path, so a policy that
reads a file that holds a credential receives the credential. On Linux, the
environment file of a process under `/proc` can hold one. That includes the file
of the Ravel CLI itself, which performs the read.

A list of refused paths would not remove this exposure, because the other files
and processes of the same user stay readable. An unconfined harness run that
prints its environment also returns the credential to the script in its output.

The HTTP client follows these rules:

- It ignores ambient proxy variables and system proxy settings.
- It follows no redirect, so a credential cannot reach another host.
- It makes no retry. Ravel switches off the default protocol retries of
  reqwest.
- It uses rustls with the trust store of the platform.
  Ravel builds the client on the reader thread, because loading the trust store
  can read files that the caller's environment names.
- Ravel removes the credential from every string of a response and of a failure
  before the script sees it, because an endpoint chooses those strings.
- Its connection deadline is no later than the deadline of the call.
- It bounds each response body before Rig decodes it. A successful body has at
  most 8 MiB, and a longer one fails the call as `output_limit`.
- It counts the JSON values of a successful body before Rig decodes it. A body
  with more than 262,144 values fails the call as `output_limit`. The count
  includes the keys of objects, and it builds no tree. The bytes alone do not
  bound the memory of the decode, because Rig keeps the whole document in
  `CompletionResponse.raw` and reads it again for the answer. A body of 8 MiB
  that holds `0,0,0,…` has four million values, and the decode of such a body
  held 586 MiB in the test. A body at the bound held 52 MiB.
- It reads at most 64 KiB of a failure body, and then it closes the connection.
  The call keeps the class of its status, and its message holds that part of
  the body.
- It counts the body bytes as they arrive. A declared length beyond the bound
  fails at once. A body without a declared length stops at the first byte
  beyond the bound.
- It decodes no compressed body, so the bound counts the bytes that Rig
  receives.
- It checks the text of the answer after Rig decodes it and after Ravel removes
  the credential from it. A text longer than 1 MiB fails the call as
  `output_limit`. The message names the length and holds none of the text.

The TLS crypto provider must build for all three archive targets with the
existing Zig glibc 2.17 toolchain. The design prefers `ring`. The increment that
adds TLS proves that build with `task release:smoke` before anything relies on
it.

### Harness entries

The `settings.json` key `harnesses` is a map from a name to an entry. Names
follow the rule for endpoints.

| Field | Type | Meaning |
|---|---|---|
| `program` | String | Required and nonempty. An absolute path, a `PATH` name or a relative path with a separator. Ravel resolves it as it resolves a selected program |
| `capabilities` | Array of nonempty strings | Optional labels that discovery repeats |

A settings problem names its place in the file, such as `endpoints.main.baseUrl`
or `budgets.llmCalls`, as the `location` of the refusal.

### Evaluation budgets and their composition

| Bound | Default | Source and range | Breach |
|---|---|---|---|
| Whole evaluation | 30,000 ms | `timeoutMs` or `--timeout-ms`, from 1,000 to 600,000 | `selection_timeout`, exit 124 |
| One host call | The time that remains | The `timeoutMs` of the call, lowered to the time that remains | The call fails as `timeout` |
| Delivered context | 262,144 bytes | `contextBytes` or `--context-bytes`, from 1 to 8,388,608 | `context_too_large`, exit 3 |
| One measured read | The smaller of 65,536 bytes and the context budget | `max_bytes`, up to the context budget | `source_too_large`, exit 3 |
| Measured sources | 256 | Fixed | `too_many_sources`, exit 3 |
| Worker messages | 1,048,576 bytes | Fixed | `message_too_large`, exit 3 |
| Policy diagnostics | 262,144 bytes | Fixed | `output_limit`, exit 3 |
| LLM calls | 8 | `budgets.llmCalls`, from 0 to 256 | `llm_budget_exhausted`, exit 3 |
| LLM output tokens | 32,768 | `budgets.llmOutputTokens`, from 0 to 10,000,000 | `llm_budget_exhausted`, exit 3 |
| LLM answer text | 1,048,576 bytes | Fixed | The call fails as `output_limit` |
| LLM response body | 8,388,608 bytes and 262,144 JSON values for a success. Ravel reads at most 65,536 bytes of a failure | Fixed | A success fails the call as `output_limit`. A failure keeps its class |
| Harness runs | 0 | `budgets.harnessRuns`, from 0 to 16 | `harness_budget_exhausted`, exit 3 |
| Harness output | 1,048,576 bytes for each stream | Fixed | The run ends as `output_limit` |
| Host calls at one time | 1 | Fixed by the synchronous script | None |
| Engine limits and worker heap | As in the policy worker section | Fixed | `script_limit` or `memory_limit`, exit 3 |

The bounds compose by these rules:

- Every budget belongs to one evaluation, which is one `start-session`. A nested
  session in a managed process has budgets of its own.
- The whole deadline runs from the launch of the worker to the judged result. It
  covers the import, `load_context`, `select` and every host call.
- The deadline of a call is the smaller of its `timeoutMs` and the time that
  remains. When the whole deadline passes during a call, the evaluation ends,
  not only the call.
- Ravel lowers the `maxTokens` of a call to the output tokens that remain.
  That value is the effective `maxTokens`.
- A call that Ravel sends uses one LLM call. It then uses its reported output
  tokens, or its effective `maxTokens` when the endpoint reports none. A sent
  call that failed reports no usage, so it uses its effective `maxTokens`. An
  endpoint can bill a request that Ravel then fails or abandons.
- A call with no LLM call or no output token left ends the evaluation before
  Ravel sends it. Ravel checks this before any other check of the call, so a
  call to an unknown endpoint ends the evaluation too. The script cannot catch
  it, because the worker hears nothing more.
- A harness run uses one run when the runner launches it.
- Teardown after the deadline or a cancellation adds at most the kill-grace of
  the runner and one second. It does not wait for a read in progress.
- Only the owner sets the three new budgets, in settings. Callers get no new
  flag. A script can read its budgets but cannot raise one.

### Cancellation and deadlines

An INT, TERM or HUP during the evaluation, or the end of the whole deadline,
stops the evaluation in this order:

1. The Ravel CLI records the first cause. A later cause does not repeat the
   stop.
2. It drops the Rig request in flight, which closes its connection.
3. It abandons a read in progress, and it does not wait for it.
4. It asks the runner to stop a harness run in progress. The runner escalates
   from TERM to KILL, clears the group and reaps the child.
5. It kills the worker and reaps it. The worker holds nothing that needs a
   graceful stop.
6. It removes the private directories, and it reports every cleanup failure.
7. It reads the cancellation latch again, and then it refuses.

Only one of steps 2 to 4 applies, because one host call runs at a time.

The final read of the latch applies the precedence of the
[completion contract](result-job-completion.md#outcome-and-diagnostic-order), as
in 1.x. A signal that Ravel consumed before that read wins over the deadline.
The refusal is then `selection_cancelled` with 128 plus its number, and Ravel
raises the same signal again. Among signals, the first consumed signal stays the
cause. The report keeps an earlier deadline as the original failure.

Without a signal, the deadline refuses as `selection_timeout`, exit 124. The
same final read follows every other refusal of the evaluation. A consumed signal
therefore also wins over a breach of a bound. No managed process launches after
a stop.

The worker also checks a stop flag at each Rhai operation, so a cooperative
script stops at once. The flag is set when the Ravel CLI closes its end of the
private socket. A native call that does not yield stops only when the
worker dies. Examples are `sleep` and the `pad` loop of Rhai 1.27.0. That is why
the worker is a process.

Dropping a request cancels local work only. A provider may finish, and may
bill, a request that it already accepted. Ravel cannot withdraw such a request.

### Diagnostics, notices and results

Stdout carries only the final result, as in 1.x. The output of a harness run
returns to the script, and it reaches no stream unless the script prints it.
The policy's `print` and `debug` output, the notices of Ravel and the runner
reports go to stderr.

The JSON routing notice moves to schema version 2:

- `worker` reports `path`, `version`, `buildId`, `rhaiVersion` and `rigVersion`.
  `bunVersion` is gone.
- `policy` adds `modules`: the canonical name, bytes and SHA-256 of the entry and
  of each module. The entry comes first, and each module follows in the order
  of its first import.
- `reads` lists every measured read with its `via`, its phase, its name, its
  bytes and its SHA-256. The `via` is `read_text` or `read_json`, and the phase
  is `load`, `context` or `selection`. A read that failed is no measured read,
  also when its bytes arrived and did not decode.
- `llmCalls` lists the endpoint, model, status, error code, usage and elapsed
  time of each call.
- `harnessRuns` lists the harness, status, exit code or signal, elapsed time of
  each run, and whether it sent a result.
- `bounds` adds `llmCalls`, `llmOutputTokens` and `harnessRuns`.

The JSON failure document keeps schema version 1. For an error in a script, its
`location` gains the form `LINE:COLUMN`, and its `source` names the file.

### Errors and refusals

Ravel keeps every 1.x code that still has a cause. It removes
`selection_unsettled` and `context_loader_unsettled`, because a Rhai function
cannot leave a promise unsettled.

| Code | Stage | Exit | Meaning |
|---|---|---|---|
| `policy_format_retired` | `authority` | 3 | The entry is TypeScript or JavaScript. Translate it with the migration guide |
| `policy_reentry_refused` | `authority` | 3 | `start-session` runs inside a harness run, where it cannot select |
| `policy_parse_failed` | `load` | 3 | A syntax error in the entry or a module, with its file, line and column |
| `policy_import_failed` | `load` | 3 | A module is missing, unreadable, too large, in a cycle or the 65th, or its top level failed |
| `policy_invalid` | `validation` | 3 | An export is missing or has the wrong arity, or the version is blank |
| `unsupported_version` | `validation` | 3 | `SCHEMA_VERSION` is not 3 |
| `selection_threw` | `selection` | 3 | `select` raised an uncaught error. The report has its file, line and column |
| `context_loader_failed` | `context` | 3 | `load_context` raised an uncaught error. The report has its file, line and column |
| `host_request_invalid` | `load`, `context` or `selection` | 3 | A host function received a malformed request |
| `script_limit` | `evaluation` | 3 | An engine limit, which `bound` names |
| `memory_limit` | `evaluation` | 3 | The worker heap reached 1 GiB |
| `llm_budget_exhausted` | `evaluation` | 3 | No LLM call or no output token remained |
| `harness_budget_exhausted` | `evaluation` | 3 | No harness run remained |
| `harness_cleanup_failed` | `evaluation` | 5 | The group of a harness run survived, or its reap failed |
| `worker_missing` | `worker` | 5 | Ravel cannot re-execute its own binary |
| `worker_identity_mismatch` | `worker` | 5 | The re-executed binary reports another build |
| `worker_failed` | `worker` | 5 | The worker died, broke the protocol or failed its teardown |

Settings problems in the new keys use `settings_invalid` and `excluded_grant`,
exit 2. A `--policy-env` flag that names a credential also refuses as
`excluded_grant`, exit 2. A policy refusal stays `policy_refused`, with the
owner's `policyCode`. Ravel never selects a fallback after a failure. The
defaults for the harness, the model, the effort and every recovery belong to the
policy. The host defaults are the ones in this specification.

### Re-entry and completion authority

A harness run executes its harness directly, through the runner. An LLM call is
an HTTP request. Neither invokes `start-session`, so neither evaluates a policy.

`start-session` refuses as `policy_reentry_refused` when `RAVEL_POLICY_RUN` is in
its environment. It refuses before it reads settings or a policy. The final
managed process never receives the marker, because Ravel scrubs `RAVEL_*` names
from it. Sessions that the final harness nests therefore stay allowed. A harness
run that clears its own environment escapes the marker, and then the budget and
the deadline still bound the work. Ravel 1.x ignores the marker.

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

The 2.0.0 installation holds these files:

- `bin/ravel`, the only executable.
- `share/ravel/examples/`, with `static.rhai`, `sample.rhai` and `llm.rhai`.
- The third-party notices for the Rust dependency tree.

The release removes `libexec/ravel`, the Bun worker and its source, the
TypeScript SDK and its declarations, and the worker build script and source
identity. It also removes the Bun pin and notices, and the typecheck, probe and
Bun hostile-runtime tasks. The archive manifest, the formula and the formula
test change to match. The formula test runs a Rhai policy.

A Taskfile task generates the notices from the Cargo metadata with a pinned
tool. It refuses a licence outside an allowlist. Rhai still lists `smartstring`,
which is MPL-2.0, as a dependency, although `compact_str` keeps its code out of
the build. The allowlist decision records that. The checks of the book run on a
checkout-local Bun as development tools. They are not part of the policy worker,
and the release carries no Bun.

### Migration

Ravel 2.0.0 is a clean cutover with no compatibility period. The precedence of
the entry lets a Grove session that Ravel 1.1.0 launched keep reading
`policy.ts`, while 2.0.0 reads `policy.rhai`.

| TypeScript 1.x | Rhai 2.0.0 |
|---|---|
| `~/.config/ravel/policy.ts` | `~/.config/ravel/policy.rhai` |
| `export const policy = definePolicy({ … })` | A module with `SCHEMA_VERSION`, `VERSION`, `select` and optionally `load_context` |
| `schemaVersion: 2` | `export const SCHEMA_VERSION = 3;` |
| `version` | `export const VERSION = "…";` |
| `select(request, context, host)` | `fn select(request, context)` |
| `loadContext(request, host)` | `fn load_context(request)` |
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
| `fetch` to a model API | `ravel::llm`, with an endpoint in settings |
| Code that launches a process | `ravel::run_harness`, with a harness entry in settings |
| `async` and `await` | Ordinary calls |
| `settings.json` keys | Unchanged, plus `endpoints`, `harnesses` and `budgets` |

The migration guide is a chapter of the book. It translates the shipped static
route table and the shipped sample policy. It also translates a policy with the
shape of the owner's: a harness table and the grant of the main repository of a
jj workspace. A fourth example shows an LLM-assisted policy.

While main holds both paths, a test runs each TypeScript policy and its
translation through the command-line seam over one table of requests. It
compares the routing notices, the argument vectors that a fake harness receives,
the delivered contexts and the refusals. The increment that removes the
TypeScript path first freezes those results as fixtures.

Grove's helpers move in Linkuistics/grove#24. They need these capabilities, and
Ravel's own tests prove each one without Grove code:

- `read_text` for the task file.
- `json_parse` and `json_encode`.
- The string functions of Rhai for the handle and the marker lines.
- `uuid_v4`.
- A module import by path.
- A wrapper that calls the `select` of another module.

### Rig upgrades

Rig has no 1.0 release, and it released seven minor versions from June to
October 2026. Ravel therefore keeps all use of Rig behind one internal module,
the LLM adapter. Its interface is one call. An endpoint entry and a checked
request go in, and a response or a classified failure comes out. An upgrade
changes the four Rig pins together and touches only that module.

The tests prove an upgrade at the command-line seam:

- A scripted fake endpoint covers each API type. Its cases are success,
  structured output, usage present and absent, 401, 429 with a wait, and 500.
  Further cases are an undecodable body, a body that stops short and a stall past
  the deadline. The body cases are a successful body beyond 8 MiB with little
  text and a 429 with a large body. A third body case has no declared length
  and is longer than the bound. A fourth body case is below 8 MiB and holds
  millions of small values, and a fifth has an answer text beyond 1 MiB. The last
  are an unsupported option and a redirect that Ravel does not follow.
- The fake endpoint checks the request that the mapping of Ravel sent. It checks
  the path, the credential header, the model, the token limit and the place of
  the schema.
- `rig-cassette` replays one recorded real exchange for each API type. A wire
  change in Rig shows as a replay mismatch.
- The opt-in smoke tasks then make one real call for each API type with real
  credentials.

A Taskfile task runs the first three for an upgrade, in that order. A new
recording is a separate opt-in step with real credentials. A Rhai upgrade
follows the same rule with the engine tests and the hostile classes.

## Requirements

### Requirement: Deterministic selection

Ravel SHALL select from a Rhai policy that makes no host call, with no network
access and no LLM endpoint configured.

#### Scenario: Route table

- **WHEN** a policy maps `kind` to a harness, a model and an effort in a table
- **THEN** the fake harness receives the argument vector from the table, and
  the notice reports no LLM call

### Requirement: Generic LLM interaction

Ravel SHALL let a policy build messages, call a model through `ravel::llm` and
interpret the answer, with no harness process.

#### Scenario: LLM-assisted selection

- **WHEN** `select` asks a fake endpoint to classify the prompt and maps the
  answer to a route
- **THEN** Ravel launches that route, the fake endpoint received one request,
  and no harness run occurred

#### Scenario: Another purpose

- **WHEN** `load_context` asks a fake endpoint for a summary and returns it in
  `facts`
- **THEN** `select` receives the summary in the delivered context

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
- **THEN** that invocation exits 3 with `policy_reentry_refused`, and the script
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

#### Scenario: Wrong role

- **WHEN** a policy calls `ravel::llm` with a message whose role is `tool`
- **THEN** Ravel exits 3 with `host_request_invalid`, and the report names the
  field and the line of the script

### Requirement: Script failures with locations

Ravel SHALL report a parse error and an uncaught runtime error with the file,
the line and the column.

#### Scenario: Syntax error in a module

- **WHEN** an imported module has a syntax error
- **THEN** Ravel exits 3 with `policy_parse_failed`, and `source` names that
  module

### Requirement: Endpoint failures are data

Each failure of an LLM call SHALL reach the script as a failed response with its
code. It SHALL NOT end the evaluation.

#### Scenario: Rate limit and fallback

- **WHEN** the first endpoint answers 429 and the policy then calls a second
  endpoint
- **THEN** the script sees `rate_limited` with `retryable` true, and the session
  uses the second answer

#### Scenario: Unsupported capability

- **WHEN** a policy sends a schema to an `openai-compatible` endpoint without
  `structuredOutput`
- **THEN** the call fails as `unsupported_capability`, and the fake endpoint
  receives nothing

#### Scenario: Oversized successful body

- **WHEN** the fake endpoint answers 200 with a short text and a metadata field
  of 9 MiB, without a declared length
- **THEN** the call fails as `output_limit`, and the fake endpoint sees its
  connection close before it sends the whole body

#### Scenario: Many small values

- **WHEN** the fake endpoint answers 200 with a body of 8 MB that holds four
  million values of `0` in a metadata field
- **THEN** the call fails as `output_limit` before Rig decodes the body, and the
  memory of the Ravel CLI stays below 128 MiB

#### Scenario: Oversized answer text

- **WHEN** the fake endpoint answers 200 with a body of 2 MiB whose answer text
  is longer than 1 MiB
- **THEN** the call fails as `output_limit`, and its message holds none of the
  text

#### Scenario: Large failure body

- **WHEN** the fake endpoint answers 429 with a body of 1 MiB
- **THEN** the call fails as `rate_limited`, and its message holds at most
  64 KiB of the body

### Requirement: Harness failures are data

Each outcome of a harness run SHALL reach the script. A cleanup failure SHALL
end the evaluation.

#### Scenario: Missing program

- **WHEN** a harness entry names a program that is not on `PATH`
- **THEN** `run_harness` returns `unavailable` with `harness_not_found`, and the
  run uses no budget

### Requirement: Deadlines and cancellation stop everything

At the deadline or on cancellation, Ravel SHALL stop the script, the LLM call
and the harness run. It SHALL reap every process and launch nothing. It SHALL
NOT wait for a read in progress. A consumed signal SHALL win over the deadline.

#### Scenario: A native call that does not yield

- **WHEN** `select` calls `sleep(600)`
- **THEN** Ravel exits 124 with `selection_timeout` within the deadline and its
  teardown bound

#### Scenario: Stalled endpoint

- **WHEN** the fake endpoint never answers
- **THEN** Ravel exits 124, and the fake endpoint sees its connection close

#### Scenario: A signal during an LLM call

- **WHEN** Ravel receives TERM while a call waits for a fake endpoint that
  never answers
- **THEN** Ravel exits 143 with `selection_cancelled` and raises TERM, the fake
  endpoint sees its connection close, and no managed process launches

#### Scenario: A signal after the deadline of an LLM call

- **WHEN** the deadline passes during a call to a stalled endpoint, and Ravel
  receives TERM before the teardown ends
- **THEN** Ravel exits 143 with `selection_cancelled`, and the report keeps
  `selection_timeout` as the original failure

#### Scenario: Descendants of a harness run

- **WHEN** Ravel receives TERM while a fake harness and its child sleep
- **THEN** Ravel exits 143, and neither process exists afterwards

#### Scenario: A signal after the deadline

- **WHEN** the deadline passes during a harness run whose fake harness ignores
  TERM, and Ravel receives TERM before the kill-grace ends
- **THEN** Ravel exits 143 with `selection_cancelled` and raises TERM, and the
  report keeps `selection_timeout` as the original failure

### Requirement: Resource exhaustion

Ravel SHALL end the evaluation at each engine limit, at the heap bound of the
worker and at each output bound. The script SHALL NOT catch a breach with `try`.
That holds for a breach in the script itself and for a call-depth, data-size or
variable breach inside the call of a function pointer. A breach of the
operations bound or of the heap bound SHALL end the evaluation, however the
script handles errors.

#### Scenario: Caught budget breach

- **WHEN** a policy wraps a ninth LLM call in `try`, with the default budget
- **THEN** Ravel exits 3 with `llm_budget_exhausted`

#### Scenario: Memory

- **WHEN** a policy grows an array of strings without end
- **THEN** Ravel exits 3 with `memory_limit` or `script_limit`, and the Ravel CLI
  survives to report it

#### Scenario: A breach inside a function pointer under `try`

- **WHEN** a policy wraps `[1].map(|x| ..)` in `try`, and the closure breaches
  the call depth, a data size or the variable bound
- **THEN** Ravel exits 3 with `script_limit`, and nothing launches

#### Scenario: An ordinary error inside a function pointer under `try`

- **WHEN** a policy wraps `[1].map(|x| throw "boom")` in `try`
- **THEN** the catch block runs with the thrown value, and the policy goes on

#### Scenario: A breach inside a comparer

- **WHEN** a policy recurses past the call depth inside the comparer of `sort`,
  `order`, `order_by` or `dedup`
- **THEN** Ravel exits 3 with `script_limit`, and nothing launches

### Requirement: Credential boundary

Ravel SHALL send the credential of an LLM endpoint only to the base URL of that
endpoint. No host function SHALL return the credential to the worker. Ravel
SHALL NOT write it to a diagnostic or a notice.

#### Scenario: Leak probes

- **WHEN** a policy calls `ravel::env` with the name of the credential, prints
  every value that it can reach, and makes one call
- **THEN** `env` throws `env_not_granted`, the key appears in no diagnostic or
  notice, and the fake endpoint sees it only in the credential header

#### Scenario: A flag grant of the credential

- **WHEN** a caller passes `--policy-env` with the `apiKeyEnv` name of an
  endpoint
- **THEN** Ravel exits 2 with `excluded_grant`, and no worker launches

#### Scenario: A credential under a base name

- **WHEN** an endpoint names `LC_MODEL_KEY` as its `apiKeyEnv`
- **THEN** Ravel exits 2 with `settings_invalid`, and no worker launches

### Requirement: Migration equivalence

Each translated policy SHALL give the same choices, arguments, contexts and
refusals as its TypeScript original.

#### Scenario: The owner-shaped policy

- **WHEN** the translation of the owner-shaped policy runs in a secondary jj
  workspace for every kind that it lists
- **THEN** each argument vector equals that of the TypeScript policy, including
  the `--add-dir` grant of the main repository

### Requirement: Entry precedence

Ravel SHALL evaluate `policy.rhai` when it exists. It SHALL refuse a lone
`policy.ts` with the migration remedy.

#### Scenario: Both files

- **WHEN** both `policy.rhai` and `policy.ts` exist in the configuration
  directory
- **THEN** Ravel evaluates `policy.rhai` and does not open `policy.ts`

## Test seams

The owner agreed one command-line seam in the requirements session. Every test
of this work runs the real `ravel start-session` against a sandbox home that
holds `policy.rhai` and `settings.json`. Two kinds of fake stand behind it:

- Fake harness scripts on a private `PATH`, as the existing tests use. They
  serve both the managed process and the harness runs.
- A deterministic local HTTP server that plays scripted responses or
  `rig-cassette` recordings. The settings of each test point an endpoint of each
  API type at it, with a dummy key and no network.

The real Rig client and the real worker run in every test. Opt-in Taskfile smoke
tasks make one real call for each API type and one real harness run, with real
credentials. The default test run never includes them. Each hostile class keeps
a firing control in the same test, as in 1.x. Policies at the same seam test the
pure functions, such as `json_encode` and `path_resolve`. There is no second,
in-process seam and no fake model inside the binary.

The seam cannot make the kernel hold a read. The nonblocking open removes the
stalls that a test can make, such as a FIFO. The FIFO case proves the open, and
its firing control is a blocking open of the same FIFO, which stalls. No test at
the seam proves that a stop never waits for a read. That property rests on the
reader thread and the stop order in this specification.

A signal after the deadline needs a teardown that lasts. On Linux, a test stops
the worker. A stopped worker takes the TERM of the teardown only when it
continues, so the Ravel CLI waits out its grace. macOS ends a stopped process on
TERM. The case
therefore runs on Linux only, and on macOS the stop order rests on the unit
tests of the supervision.

## Out of scope

- A verb that evaluates a policy without a launch. The owner kept it out of this
  workstream.
- Streaming answers to a script.
- Tools, agents and documents in an LLM call.
- More than one host call at a time.
- Live model lists.
- An explicit proxy setting. Ravel ignores ambient proxies.
- API types beyond the three.
- Modules embedded in Ravel, such as a successor to `ravel/sdk`.
- A regular-expression host function. The string functions of Rhai parse a
  handle and a marker line, and the tests prove it.
- A URL or a credential in a script.
- A filesystem sandbox. A measured read can read each file that the user can
  read.
- A compatibility period for TypeScript.
- The port of Grove's helpers, which Linkuistics/grove#24 carries.
- A port of the book checks that run on Bun.
