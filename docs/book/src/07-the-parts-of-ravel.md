# The parts of Ravel

Ravel separates the choice of a command from the supervision of that command.
The [configuration script](02-actors-and-words.md#caller-and-owner) chooses a program.
The [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision)
checks that choice, runs the program and decides the final outcome.
The [caller](02-actors-and-words.md#caller-and-owner) interprets the result.
A [session](02-actors-and-words.md#caller-and-owner) is one
`start-session` invocation, from the request to the reported outcome.

This chapter explains the parts of the source that hold those roles. After it, you
can locate each part and explain why Ravel uses Rhai and two Rust
packages. Chapter 8 follows one session through the functions. Chapter 9
explains the process and terminal mechanics.

The workspace contains two Rust packages: `ravel` and `keyed-launch`.
The Rhai worker is a separate process of the same `ravel` executable.
The table separates source locations from process boundaries: a package or
library does not always correspond to a separate process.

| Part | Source location | Where it runs | Responsibility |
|---|---|---|---|
| Ravel CLI | `crates/ravel/src/main.rs`, with the other executable modules | The `ravel` process | Check the request, coordinate the parts and report the outcome |
| [Rhai worker](02-actors-and-words.md#evaluation-and-supervision) | `crates/ravel/src/rhai_worker/` | A second `ravel` process | Evaluate the owner's Rhai configuration script |
| Host capabilities and data helpers | `crates/ravel/src/rhai_worker/host.rs`, `helpers.rs` | The Ravel CLI performs effects; the Rhai worker computes | Supply the operations in the `ravel` namespace |
| [Local runner](02-actors-and-words.md#evaluation-and-supervision) | `crates/keyed-launch/src/` | Within the Ravel CLI | Launch and supervise a local job |
| [Result channel](02-actors-and-words.md#the-managed-process-and-its-result) | `crates/ravel/src/completion.rs`, exported by `lib.rs` | Within the Ravel CLI and the separate `end-session` invocation | Carry one result through temporary files |

The [source-responsibility figure in chapter 6](06-reference.md#installed-layout)
shows the packages, the worker and the host capabilities. The
[process-and-authority figure in chapter 2](02-actors-and-words.md#the-managed-process-and-its-result)
shows their process boundaries. The runner is library code, not another
service. `end-session` is another invocation of the same `ravel` executable,
not a request to a persistent daemon.

## The Ravel CLI coordinates the session

The CLI connects the configuration script's selection to the supervision of the job. Its
`choice.rs` prepares the request and obtains a checked configuration script response.
Its `run.rs` creates a fresh result channel and gives that channel to the
runner with the selected command. After supervision, the CLI combines the
result, cancellation and cleanup reports before it writes to standard output.

The `ravel` process keeps the caller's environment and the authority over the
terminal while configuration script code runs in the worker. It also reserves standard output
for the result. The standard output of the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
and Ravel's diagnostics go to standard error. A configuration script response cannot write the session's final result.

The package has a library target as well as an executable target. The
library exports `SessionChannel`, `publish_result` and `end_session`, with
the result constants. The executable uses this library and `keyed-launch`.
This division lets the result-channel tests compose a channel with a runner
without configuration script evaluation.

The CLI retains a coordination cost. Neither the worker nor the runner
knows all the conditions for a successful Ravel session. For example, a
runner can report a reaped child while the CLI still owes temporary-file
cleanup. A change to final output or failure precedence therefore belongs
in the CLI's coordination code.

The table separates command choice, job supervision and result interpretation,
so each decision has one owner.

| Decision | Owner | Information at the boundary |
|---|---|---|
| Which program to select | Configuration script | A command or a [refusal](02-actors-and-words.md#evaluation-and-supervision) |
| Whether the configuration script response is valid | Ravel CLI | A checked command, or no launch |
| How to launch and stop the job | Runner | Launch options from the CLI |
| Whether to return the result | Ravel CLI | Job observations and cleanup reports |
| What the result means | Caller | The result, unchanged |

## The worker evaluates Rhai before the launch

The configuration script is a Rhai module. The Ravel CLI starts a second copy
of its own executable in an internal worker role. The role name
`ravel-rhai-config-worker` is an internal process identity, not an installed
filename or a command for owners to invoke.

The worker has an empty environment, null standard input and a private socket
at file descriptor 3. It only computes. The Ravel CLI performs measured reads,
environment access and harness runs when the script requests a host capability.
The worker receives no session-result context. Its standard output and standard
error carry bounded diagnostics, and length-prefixed JSON frames carry the
protocol on the socket.

The CLI checks the worker identity before it sends the entry and request.
It checks the returned data through `config::judge_result`. The worker must
end and be reaped, and its private directory must be removed, before a managed
process can launch. Failed required cleanup prevents that launch.

The separate process lets the CLI stop evaluation at its deadline and keeps a
worker abort or heap exhaustion outside the supervisor. The script remains
trusted executable configuration: host reads can expose owner data to it.
[Chapter 4](04-write-a-configuration-script.md) explains the language and its
operations. [Chapter 2](02-actors-and-words.md#trust-and-confinement) separates this authority
from the confinement of a managed process.

## The Rhai module defines the configuration interface

The module exports `SCHEMA_VERSION`, `VERSION`, `select` and an optional
`load_context`. There is no TypeScript SDK or installed declaration file.
The worker supplies the data helpers in the `ravel` namespace. Effects cross
the socket to the Ravel CLI as host capabilities.

| Interface | Purpose | Limit |
|---|---|---|
| `select(request, context)` | Select a command or refuse | Ravel checks the returned map before any launch |
| `load_context(request)` | Assemble selection context | Selection context grants no result authority |
| `ravel::read_text`, `ravel::read_json` | Ask the CLI to read and measure a file | Settings bound the reads and delivered data |
| `ravel::run_harness` | Ask the CLI to supervise a configured harness | Settings name the harness and bound its runs |
| `ravel::json_check` | Check data against a schema | It supplies no execution authority |

The default entry is `config.rhai`, or `--config` names an explicit Rhai file.
Settings stay at their account path when the explicit entry changes. A new
route usually belongs in the configuration script alone. A new host capability
crosses the worker protocol, CLI implementation and validation boundaries.

## The runner owns the local job mechanics

`keyed-launch` contains the process-group setup, escalation, reap and
terminal recovery, with Rust launch types as their interface. It forwards
cancellation signals to the job. `Argv` holds a program and its arguments as
separate words. `Launch` adds the working directory, environment removals
and grants, an optional completion channel, signal state and escalation
durations. The runner spawns the command directly. A shell participates only
when the command selects one.

The runner does not choose a configuration script or interpret provider and model labels.
It also does not read the Ravel result. Its completion channel tells it only
that a file appeared. Its Rust result is `Ended` or `LaunchError`, not the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
that the caller receives.

The table locates the runner interfaces through which the Ravel CLI supplies
launch controls and receives job observations.

| Runner interface | Information or action |
|---|---|
| `Channel` | A fresh completion path whose file appearance requests shutdown |
| `run` and `run_observed` | Supervise the job, with optional spawn and reap notifications |
| `run_noninteractive` | Isolate the child from the terminal and input, and send output to a file |
| `run_confined_observed` | Supervise with filesystem restrictions |
| `EntrySignals` | Supply the child's entry signal state |
| `Ended` | Report child status, the cause of its end, channel appearance, group state and retained operation reports |

The separate Rust package lets developers exercise job mechanics without
the TypeScript worker or Ravel's configuration script conventions. Its only production
dependency is `libc`, which supplies operating-system calls. A correction to
terminal recovery can stay inside the runner while the configuration script interface
stays unchanged. Chapter 9 explains those mechanics and their limits.

The code that calls the runner is its consumer. In Ravel, the Ravel CLI is
the consumer. It supplies the environment rules, completion channel and
escalation durations. It checks `Ended.group` before it acts on completion,
and it keeps the operation reports that `Ended` carries. The runner can
change process-wide signal state, and its return does not restore a host
for unrelated future work.

Ravel supports no separate component. Its runner serves the Ravel CLI, which
supervises one session and then exits. Another program invokes the Ravel CLI as a child
process and leaves supervision to it. These private packages do not
establish a contract for a
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

The table shows why a successful send or a runner observation alone cannot
establish a successful Ravel session.

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
configuration script choice. The result channel keeps its file format private, so owner
configuration script and callers do not depend on it. But the CLI still needs to combine
these observations in the correct order.
Chapter 8 follows that order and the failure paths.

## The program contract stays loose

The configuration script selects a program, Ravel runs it, and a program in the session
sends one value with `end-session`. Ravel expects a
[harness](02-actors-and-words.md#the-managed-process-and-its-result) in normal use,
but it requires no model-provider interface from the selected program.
The configuration script decides whether the command receives the prompt.

This separation appears in the code, not only in the examples. Rust checks
the selection's shape in `config.rs`. Then `run.rs` constructs `Argv` from
the selected executable and argument words. The runner adds no prompt
argument. `end_session` accepts text and uses the inherited path.
The final CLI write returns that text unchanged after successful cleanup.

The table shows which program, prompt and result decisions can change while
the interfaces between the parts stay unchanged.

| Change | Part that owns the decision | Boundary that stays unchanged |
|---|---|---|
| Select a different harness or ordinary program | Configuration script | The runner still receives a program and argument words |
| Pass the prompt differently | Configuration script | Ravel still gives the caller's prompt to the configuration script unchanged |
| Give a result a new task meaning | Caller | Ravel still carries one opaque value |

The command still needs to be executable, and a successful session still
needs a sent result and successful cleanup. A program that exits with status
zero without a result does not complete that contract. The
[complete example in chapter 3](03-a-complete-session.md)
uses a fake agent to demonstrate the contract without a real model.

## The installed executable supplies both roles

An installation contains one executable, `bin/ravel`. The CLI resolves its own
executable to start the Rhai worker, so a symlink to the CLI uses the same
binary for both roles. The worker is not selected through `PATH`.

The initial worker message carries its protocol, package version and source
identity. The CLI checks those values before it sends the configuration request.
The identity detects a worker mismatch; it does not authenticate the installation
against its owner. A change to the Rhai worker needs one rebuilt executable.

Chapter 11 explains delivery. The
[installed layout in chapter 6](06-reference.md#installed-layout)
lists the executable, example scripts and Rust notices.

This chapter checks the source boundaries and their stated obligations.
It does not establish compatibility with every owner helper, independent
component use or safe concurrent use within another host. The
[evidence limits in chapter 1](01-what-ravel-is-for.md#see-what-we-tested-and-what-remains-unchecked)
separate measured platforms from unqualified ones. Chapter 9 gives the
process and terminal guarantees within those limits.
