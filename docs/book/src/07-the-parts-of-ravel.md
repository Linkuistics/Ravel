# The parts of Ravel

Ravel separates the choice of a command from the supervision of that command.
The [owner policy](02-actors-and-words.md#caller-and-owner) chooses a program.
The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision)
checks that choice, runs the program and decides the final outcome.
The [caller](02-actors-and-words.md#caller-and-owner) interprets the result.
A [session](02-actors-and-words.md#caller-and-owner) is one
`start-session` invocation, from the request to the reported outcome.

This chapter explains the parts of the source that hold those roles. After it, you
can locate each part and explain why Ravel uses TypeScript and two Rust
packages. Chapter 8 follows one session through the functions. Chapter 9
explains the process and terminal mechanics.

The workspace contains two Rust packages: `ravel` and `keyed-launch`.
The policy worker is a separate TypeScript build within `crates/ravel`.
The table gives the source location of each part and the process in which it runs.

| Part | Source location | Where it runs | Responsibility |
|---|---|---|---|
| Ravel CLI | `crates/ravel/src/main.rs`, with the other executable modules | The `ravel` process | Check the request, coordinate the parts and report the outcome |
| [Policy worker](02-actors-and-words.md#policy-evaluation-and-supervision) | `crates/ravel/worker/src/` | A separate `ravel-policy` process | Evaluate the owner's TypeScript policy |
| Policy SDK | `crates/ravel/worker/sdk/index.ts` | Within the policy worker | Supply the types and helpers that a policy imports |
| [Local runner](02-actors-and-words.md#policy-evaluation-and-supervision) | `crates/keyed-launch/src/` | Within the Ravel CLI | Launch and supervise a local job |
| [Result channel](02-actors-and-words.md#the-managed-process-and-its-result) | `crates/ravel/src/completion.rs`, exported by `lib.rs` | Within the Ravel CLI and the separate `end-session` invocation | Carry one result through temporary files |

The [source-responsibility figure in chapter 6](06-reference.md#installed-layout)
shows the packages, the worker and the embedded SDK. The
[process-and-authority figure in chapter 2](02-actors-and-words.md#the-managed-process-and-its-result)
shows their process boundaries. The runner is library code, not another
service. `end-session` is another invocation of the same `ravel` executable,
not a request to a persistent daemon.

## The Ravel CLI coordinates the session

The CLI connects the policy's selection to the supervision of the job. Its
`choice.rs` prepares the request and obtains a checked policy response.
Its `run.rs` creates a fresh result channel and gives that channel to the
runner with the selected command. After supervision, the CLI combines the
result, cancellation and cleanup reports before it writes to standard output.

The `ravel` process keeps the caller's environment and the authority over the
terminal while policy code runs in the worker. It also reserves standard output
for the result. The standard output of the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
and Ravel's diagnostics go to standard error. A policy response cannot write the session's final result.

The package has a library target as well as an executable target. The
library exports `SessionChannel`, `publish_result` and `end_session`, with
the result constants. The executable uses this library and `keyed-launch`.
This division lets the result-channel tests compose a channel with a runner
without policy evaluation.

The CLI retains a coordination cost. Neither the worker nor the runner
knows all the conditions for a successful Ravel session. For example, a
runner can report a reaped child while the CLI still owes temporary-file
cleanup. A change to final output or failure precedence therefore belongs
in the CLI's coordination code.

| Decision | Owner | What the other parts receive |
|---|---|---|
| Which program to select | Owner policy | A command or a [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) |
| Whether the policy response is valid | Ravel CLI | A checked command, or no launch |
| How to launch and stop the job | Runner | Launch options from the CLI |
| Whether to return the result | Ravel CLI | Job observations and cleanup reports |
| What the result means | Caller | The result, unchanged |

## The worker executes TypeScript before the launch

Owner configuration is executable TypeScript. A policy can import modules,
use asynchronous functions and read files or network services. A static
table parser cannot preserve that contract. Bun supplies the language runtime
and module support. Ravel does not implement another TypeScript engine.

The build compiles the worker and its Bun runtime into `ravel-policy`.
An installed Ravel therefore needs no separate Bun or Node.js installation.
Rust owns supervision, while TypeScript supplies the policy execution
environment. This choice costs a second build tool, a larger executable,
the licence notices of the Bun runtime and checks on each delivery target.

The worker is a separate process so owner code does not execute inside the
process that supervises the session. In `worker.rs`, the CLI starts it with
a cleared environment, null standard input and a private socket at file
descriptor 3. The worker's standard output and standard error carry bounded
diagnostics. Length-prefixed JSON messages on the socket carry the protocol.

The CLI supplies only the base environment and permitted grants.
`environment.rs` excludes `RAVEL_*` from grants, so a nested caller cannot
pass its `RAVEL_EXIT_FILE` to the worker through this path. The CLI allocates
the new session's result channel only after policy evaluation succeeds.
Thus, the worker receives neither session's
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).

The worker stops before the managed process starts. A valid response does
not let it continue beside the managed process. The CLI closes the protocol
socket, ends the worker and confirms its reap. A parent reaps a child when it
collects the exit status of that child. The CLI also removes the worker's
private directory.

| Condition at the worker boundary | Source that enforces it | Consequence |
|---|---|---|
| The worker's identity matches before the policy request | `worker.rs`: `converse`, `verify_hello` | An incompatible worker receives no policy request |
| Response passes the checks | `choice.rs`: `judge` | Invalid data does not become a command |
| Worker reap and directory removal succeed, with no failed stop operation | `worker.rs`: `evaluate` | Failed required cleanup prevents a managed launch |
| Evaluation succeeds before executable resolution and launch | `choice.rs`: `evaluate_and_resolve`, then `run.rs`: `run` | The worker and the managed process do not run at the same time on this path |

This order closes the policy phase before the job phase starts. Thus the
worker's protocol messages and diagnostics cannot occur during the managed
launch. The order also lets the CLI stop the session when worker cleanup
fails, before the selected program can act.

The separate process is not a sandbox for hostile owner code. Policy is
trusted code. It can start detached children, which the worker cleanup
guarantee does not cover. Same-user code can also tamper with files.
Chapter 2 explains how these controls differ from the
[confinement of a managed process](02-actors-and-words.md#trust-and-confinement).

Three parts of Ravel control the worker's startup. The CLI filters
environment variables before the worker starts. The worker build disables dotenv, bunfig and
tsconfig autoloading. The worker starts in a private empty directory, then
changes to `/` before it registers embedded imports or loads owner policy.
These controls limit the files and variables that can change policy
evaluation without a request from the policy.

Package imports still need Bun's resolver. The build keeps package
autoloading enabled. `/package.json` and `/node_modules` stay reachable
inputs for the resolver.
The [module-resolution reference](06-reference.md#import-resolution)
gives the supported import rules and their limits. A runtime replacement
needs to preserve those rules as well as TypeScript syntax.

## The SDK describes the policy interface

The SDK supplies the types for a policy, its request and its
[policy context](02-actors-and-words.md#three-different-kinds-of-information).
It also supplies types for the selected command or refusal.
`definePolicy` returns its argument unchanged.
Its purpose is to let the TypeScript checker check a policy object and infer
the parameter types. The CLI still checks the data that crosses the worker
socket at runtime.

| SDK surface | Purpose | Limit |
|---|---|---|
| `Policy`, `SelectionRequest`, `Selected`, `Refused` | Describe selection inputs and outputs | Types do not replace runtime checks |
| Policy context and host types | Describe policy data and the host operations | Policy context grants no result authority |
| `definePolicy` | Check a policy object's TypeScript shape | It does not execute the policy |
| `readChoice` | Read names that the policy offers from `.ravel-choice` | The policy gives those names their command meanings |

The worker embeds `ravel/sdk` and `ravel/examples/static`. Exact imports of
those names use the embedded modules. The installation also supplies source
and declarations for configuration authors. An edit to those installed
source files does not replace the embedded modules.

The compatibility promise includes the supported owner policies, SDK use,
settings and policy discovery. The CLI still uses the default policy path
or an explicit `--config` path. Settings stay at their account path when
the explicit policy path changes. The
[policy chapter](04-writing-an-owner-policy.md) teaches this interface.
A change to Ravel's internal structure does not justify a new configuration
format.

A new command choice usually belongs in owner policy alone. A new SDK
field crosses more boundaries: its TypeScript declaration, worker messages
and Rust validation need to agree. The split leaves that coordination cost
with Ravel's developers.

## The runner owns the local job mechanics

`keyed-launch` contains the process-group setup, escalation, reap and
terminal recovery, with Rust launch types as their interface. It forwards
cancellation signals to the job. `Argv` holds a program and its arguments as
separate words. `Launch` adds the working directory, environment removals
and grants, an optional completion channel, signal state and escalation
durations. The runner spawns the command directly. A shell participates only
when the command selects one.

The runner does not choose a policy or interpret provider and model labels.
It also does not read the Ravel result. Its completion channel tells it only
that a file appeared. Its Rust result is `Ended` or `LaunchError`, not the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
that the caller receives.

| Runner interface | Information or action |
|---|---|
| `Channel` | A fresh completion path whose file appearance requests shutdown |
| `run` and `run_observed` | Supervise the job, with optional spawn and reap notifications |
| `run_noninteractive` | Isolate the child from the terminal and input, and send output to a file |
| `run_confined_observed` | Supervise with filesystem restrictions |
| `EntrySignals` | Supply the child's entry signal state |
| `Ended` | Report child status, the cause of its end, channel appearance, group state and retained operation reports |

The separate Rust package lets developers exercise job mechanics without
the TypeScript worker or Ravel's policy conventions. Its only production
dependency is `libc`, which supplies operating-system calls. A correction to
terminal recovery can stay inside the runner while the policy interface
stays unchanged. Chapter 9 explains those mechanics and their limits.

The code that calls the runner is its consumer. In Ravel, the Ravel CLI is
the consumer. It supplies the environment rules, completion channel and
escalation durations. It checks `Ended.group` before it acts on completion,
and it keeps the operation reports that `Ended` carries. The runner can
change process-wide signal state, and its return does not restore a host
for unrelated future work.

Chapter 10 has no text yet. These private packages do not establish a
supported component for another project or a contract for a
[persistent runner](01-what-ravel-is-for.md#decide-whether-ravel-fits).

## The result channel carries the value

The [result channel](02-actors-and-words.md#the-managed-process-and-its-result)
adds contents to the runner's file-appearance mechanism. `SessionChannel`
owns a private temporary directory, the completion path and an open
directory handle for reads. `end_session` obtains the destination from
`RAVEL_EXIT_FILE`. `publish_result` writes a complete temporary file, then
tries a hard link that cannot replace an earlier result.

The runner observes the file's appearance. The result-channel code reads
the contents and rejects invalid files. Its `result_after` checks the group
state and cancellation in the supplied `Ended`. An empty string is a sent
result. An absent file is no result.

| Part | What it knows | What it cannot establish alone |
|---|---|---|
| `end-session` | The inherited destination and whether the send succeeded | Whether supervision and cleanup succeeded |
| Runner | File appearance and child, group and terminal observations | Whether the file contains a valid Ravel result |
| `SessionChannel` | File contents and the supplied `Ended` | Whether that `Ended` belongs to its own launch |
| Ravel CLI | Which channel belongs to the launch, plus the other cleanup and signal reports | What the result means to the caller |

The Ravel CLI pairs the channel with the launch. The Rust type of
`SessionChannel::result_after` accepts an `Ended` from any launch.
`run.rs` passes the outcome of the launch that received that channel.
It then explicitly closes the channel and checks the other cleanup results.
A readable file alone cannot establish success.

A change to result storage therefore affects the result channel, the
runner's observation and the CLI's coordination. It does not require a new
policy choice. The result channel keeps its file format private, so owner
policy and callers do not depend on it. But the CLI still needs to combine
these observations in the correct order.
Chapter 8 follows that order and the failure paths.

## The program contract stays loose

The policy selects a program, Ravel runs it, and a program in the session
sends one value with `end-session`. Ravel expects a
[harness](02-actors-and-words.md#the-managed-process-and-its-result) in normal use,
but it requires no model-provider interface from the selected program.
The policy decides whether the command receives the prompt.

This separation appears in the code, not only in the examples. Rust checks
the selection's shape in `policy.rs`. Then `run.rs` constructs `Argv` from
the selected executable and argument words. The runner adds no prompt
argument. `end_session` accepts text and uses the inherited path.
The final CLI write returns that text unchanged after successful cleanup.

| Change | Part that owns the decision | Boundary that stays unchanged |
|---|---|---|
| Select a different harness or ordinary program | Owner policy | The runner still receives a program and argument words |
| Pass the prompt differently | Owner policy | Ravel still gives the caller's prompt to the policy unchanged |
| Give a result a new task meaning | Caller | Ravel still carries one opaque value |

The command still needs to be executable, and a successful session still
needs a sent result and successful cleanup. A program that exits with status
zero without a result does not complete that contract. The
[complete example in chapter 3](03-a-complete-session.md)
uses a fake agent to demonstrate the contract without a real model.

## The installed pair stays together

An installation contains `bin/ravel` and
`libexec/ravel/ravel-policy`. The CLI locates the worker relative to its own
canonical executable path. A symlink to the CLI therefore keeps the same
worker. The CLI does not select the worker through `PATH`.

Before it sends the policy request, the CLI checks the worker's protocol,
package version and source identity. The source identity is a SHA-256 digest
that both builds derive from the same worker sources and build inputs.
A missing or incompatible worker stops selection. This check pairs builds.
It does not authenticate files against the owner of the installation.

| Build input or output | How the parts stay together |
|---|---|
| Worker TypeScript, SDK, examples, two tsconfigs and worker build script | Rust `build.rs` and the worker build hash the same sorted input set |
| Protocol, package version and source identity | The worker sends them in its first message, and the CLI checks them |
| Installed SDK source and declarations | Configuration authors use the supplied interface alongside the embedded modules |
| Compiled Bun runtime | Installed execution needs no host TypeScript runtime |

This arrangement keeps an installed Ravel independent of another project's
installation. It also means that a worker-source change needs a rebuilt
pair. Chapter 12 explains delivery. The
[installed layout in chapter 6](06-reference.md#installed-layout)
lists the files and shows their deployment figure.

This chapter checks the source boundaries and their stated obligations.
It does not establish compatibility with every owner helper, independent
component use or safe concurrent use within another host. The
[evidence limits in chapter 1](01-what-ravel-is-for.md#understand-the-evidence-limits)
separate measured platforms from unqualified ones. Chapter 9 gives the
process and terminal guarantees within those limits.
