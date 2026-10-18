# Follow a session through the code

This chapter follows the [session](02-actors-and-words.md#caller-and-owner)
from chapter 3 through the code that returns `finished`.
The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision)
coordinates the path. The [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
evaluates the [owner policy](02-actors-and-words.md#caller-and-owner).
The [runner](02-actors-and-words.md#policy-evaluation-and-supervision)
controls the managed job inside the Ravel CLI process.

This is an orientation to one path, not complete source coverage.
It omits the full policy-context protocol, confinement backends, platform
system calls, test instruments and delivery code.
[Chapter 7](07-the-parts-of-ravel.md) explains the component boundaries.
Chapter 9, “Jobs, signals and the terminal”, explains the job-control details.

## Trace the example

The [first session in chapter 3](03-a-complete-session.md#run-the-first-session)
supplies `--kind demo`, `--config policy.ts` and a prompt argument.
The policy chooses `fake-agent` and passes the prompt as one argument.
The fake prints the prompt, calls `ravel end-session -- "finished"`, then exits.
No policy-context loader or confinement participates in this path.

The [caller](02-actors-and-words.md#caller-and-owner) receives `finished`
without a newline on standard output and status 0.
The fake's printed line and Ravel's diagnostics appear on standard error.
The example check runs the checkout build in scratch with private directories
and a cleared environment. It has no terminal, so it does not demonstrate
terminal handover or recovery.

The table gives the whole path before the individual decisions.
Paths below are relative to the checkout root.

| Step | Code location | What passes to the next step |
|---|---|---|
| Parse the command | `crates/ravel/src/main.rs`, `main` | `RunArgs` for `run::run` |
| Check inputs and policy path | `crates/ravel/src/choice.rs`, `prepare` | `Prepared`, with inputs and the policy entry |
| Evaluate and check the policy | `choice::choose_prepared`, `worker::evaluate` | A checked command after worker cleanup |
| Find the program | `crates/ravel/src/program.rs`, `resolve` | The executable path and original program name |
| Prepare output and channel | `crates/ravel/src/run.rs`, `run` | A `Launch` with arguments and its own result channel |
| Launch and supervise | `crates/keyed-launch/src/run.rs` | `Ended`, after reap and terminal recovery, with group state |
| Check the result | `crates/ravel/src/completion.rs`, `result_after` | The result from that launch |
| Complete cleanup and decide | `run`, `Handlers::finalise`, `settle` | A result that can reach the final write |
| Return the result | `run`, then `main` | Unchanged output, then status 0 if the write and flush succeed |

The managed process calls a second `ravel` process for `end-session` during
supervision. That process sends the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
through the [result channel](02-actors-and-words.md#the-managed-process-and-its-result).
It does not return the result to the original caller or wait for job cleanup.

## Prepare the request

`main` in `crates/ravel/src/main.rs` parses the arguments with
`Cli::try_parse_from`. The definitions in `crates/ravel/src/cli.rs` distinguish
`StartSession` from `EndSession`.
For `start-session`, `main` calls `run::run`. For `end-session`, it calls
`completion::end_session` directly, without policy evaluation.

`run` checks the saved entry signal state and begins cancellation handlers.
These handlers retain a signal that arrives during selection.
Next, `choice::prepare` reads the settings, prepares the inputs and resolves
the policy path. The order matters: settings supply defaults that input
validation needs before the worker exists.

Excerpt from `crates/ravel/src/choice.rs`: these first statements of
`prepare` establish the inputs and the policy entry.

```rust
    let home = std::env::var_os("HOME");
    let settings = Settings::read(home.as_deref())?;
    let inputs = Inputs::read(args, &settings)?;
    let entry = authority::resolve(args.config.as_deref(), &inputs.cwd, home.as_deref())?;
```

`Prepared` holds these checked inputs and the entry. It contains no policy
decision. In the example, `authority::resolve` resolves `policy.ts` relative
to the caller's working directory. It requires a readable regular file and
a real path that the worker can import exactly.
It does not search for another policy when this path fails.

The prompt remains caller data. The request that `choice::request` constructs
contains the unchanged prompt, the kind, the working directory, parameters
and effective limits. Ravel does not append instructions to end the session.
The example's policy chooses to pass the prompt to the fake.

## Evaluate and check the policy

`choice::choose_prepared` locates the worker that matches the CLI, then calls
`evaluate_and_resolve`. That function calls `worker::evaluate` with the
request and a callback, `choice::judge`, that checks the worker's responses.
The Rust coordinator owns the checks. TypeScript executes the policy.

`worker::evaluate` in `crates/ravel/src/worker.rs` creates the worker's
private directory and socket. It starts the worker with a restricted
environment, null standard input and separate pipes for diagnostics.
The worker receives no
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
Its standard output is not the selection protocol.

The exchange has these steps in the example:

| Order | Actor and function | Action |
|---|---|---|
| 1 | Worker, `crates/ravel/worker/src/main.ts` | Changes to `/`, registers embedded modules and sends `hello` |
| 2 | Rust, `worker::converse` | Checks the worker identity, then sends the policy path and request |
| 3 | Worker, `converse` via `load` | Imports the policy and sends its description |
| 4 | Rust, `choice::judge` | Checks that description, then requests selection |
| 5 | Worker, `converse` via `select` | Calls the policy's `select` and sends its returned value |
| 6 | Rust, `choice::judge` | Checks the selected command with `policy::selected` |
| 7 | Rust, `worker::evaluate` | Closes the socket, ends and reaps the worker, collects diagnostics and removes its private directory |

The example has no [policy context](02-actors-and-words.md#three-different-kinds-of-information).
Thus, `judge` skips the optional context exchange.
The worker supplies `undefined` as the policy's context argument.
The policy returns `fake-agent` with the prompt in its argument array.

`worker::evaluate` returns success only after confirmed worker reap and
successful required cleanup. A valid command alone does not permit launch.
The runner cannot enforce this order because it has no worker to inspect.
The Ravel CLI enforces it before the runner call.

After that return, `evaluate_and_resolve` checks cancellation and calls
`program::resolve`. Here, it searches the caller's `PATH` for `fake-agent`.
The example's `bin` directory supplies that executable.
The `Choice` retains both the selected command and the executable
path. The command's labels supply the routing notice, not a model integration.

## Prepare the managed launch

`run` first calls `ResultOutput::reserve`. This function checks that standard
output is writable, saves its descriptor, then redirects standard output to
standard error. The saved descriptor does not pass through exec to the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result).
This arrangement keeps the fake's diagnostics separate from the result.

Next, `SessionChannel::allocate` creates a fresh temporary directory and
channel path. It retains an open directory descriptor for the later read.
Allocation creates no result file. The channel belongs to this launch alone.

`run` builds the argument vector from the checked command.
The excerpt preserves the difference between the executable path and the
original program name used as argument zero.

Excerpt from `crates/ravel/src/run.rs`: the argument vector uses the
resolved path and each argument that the policy supplied.

```rust
        let argv = Argv::new(
            OsString::from(&choice.executable.path),
            choice.command.args.iter().map(OsString::from).collect(),
        )
        .with_arg0(OsString::from(&choice.executable.program));
```

Ravel performs no shell expansion or word splitting.
The prompt remains one argument. The fake's shebang selects its shell
interpreter, as chapter 3 explains.

The `Launch` holds this argument vector, the channel, environment changes,
entry signal state and escalation times.
For this unconfined path, `run` calls
`keyed_launch::run_observed_with_start_check`.
The runner removes old result-context variables, applies grants, then sets
the new `RAVEL_EXIT_FILE` last.
Thus, a nested session gives its own child a fresh destination.
The unconfined child otherwise inherits the caller's environment and working
directory.

In `crates/keyed-launch/src/run.rs`, `run_with_output` creates the process
group and spawns the command. If Ravel owns the foreground, the child takes
the terminal during spawn, before exec.
After spawn, `run_with_output` calls `supervise_started`.
The `Started` event records spawn-reported success, not proof of exec.
The Ravel CLI's start check restores its entry signal mask before ordinary
supervision continues. The parent can also lend the terminal during `watch`,
after that check, when Ravel owns the foreground.

## Send the result

The fake calls `ravel end-session -- "finished"`.
In this second Ravel process, `main` calls `end_session` in
`crates/ravel/src/completion.rs`. That function reads `RAVEL_EXIT_FILE` and
passes the path and result to `publish_result`.
The function name retains the source's terminology. Its action is to send
one value to the enclosing session.

`publish_result` checks the byte bound and the parent directory, then writes
the value into a temporary file in that directory.
It adds the completion name only after the write and flush succeed.

Excerpt from `crates/ravel/src/completion.rs`: the temporary file receives
the complete value before the completion name exists.

```rust
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(result.as_bytes())?;
    temporary.flush()?;
```

The next operation, `fs::hard_link`, adds a name for that file.
It does not replace a name that already exists. This is the first-result rule.
The runner can observe a complete file, rather than a partly written value.

| Channel state | Effect of `end-session` |
|---|---|
| No completion name | Adds the name for the complete temporary file |
| A valid result already exists | Checks the prior result and leaves it unchanged |
| Missing or empty inherited path, stale directory, or invalid prior result | Reports failure |

An empty string is still a sent result. Newlines and Unicode remain in the
value. A second call cannot replace the first value, but it races with
shutdown and removal of the channel. It can therefore fail or never run.

The file operation requests completion. It does not signal the managed group
or perform the enclosing session's cleanup. The original Ravel process owns
those actions.

## Finish the managed job

`supervise_started` calls `supervise_reported`, which calls `watch`.
These functions are in `crates/keyed-launch/src/run.rs`.
`watch` checks cancellation, the child's state and channel appearance.
If the child continues after the result appears, `watch` applies the grace,
TERM and KILL stages from [chapter 3](03-a-complete-session.md#send-the-result-and-finish-the-job).

The example can exit before `watch` observes the file.
That path needs no result-triggered escalation.
`supervise_reported` checks channel appearance again when it constructs
`Ended`, after the job cleanup steps.
Thus, a rapid child exit does not by itself lose the result.

The final group stop precedes reap on the normal path through `watch`.

Excerpt from `crates/keyed-launch/src/run.rs`: after it observes the child's
exit, `watch` stops the group before it collects the child's exit status.

```rust
    child.bound(Instant::now() + ERROR_SETTLEMENT);
    child.kill_group();
    let reaped = child.reap();
```

The direct child's PID reserves the group's identity until reap.
The runner uses that interval to stop descendants that remain in the group.
After `watch` returns, `supervise_reported` recovers a lent terminal and
confirms whether the group is gone. It returns an `Ended` with separate
fields for the child status, cancellation, channel appearance and group state.

The Ravel CLI needs those separate facts. A later nonzero child status does
not invalidate a sent result. A group that remains or failed required recovery
prevents success. Descendants that leave the group remain outside this
cleanup guarantee.

## Read the result

Back in `run`, `allocated.result_after(ending)` receives the `Ended` from
this channel's runner call. The association belongs to `run`: the Rust type
alone does not prove that an arbitrary `Ended` belongs to this channel.

`result_after` first checks the group and cancellation, then channel
appearance. It reads the contents only after those checks.

| Observation | Result of the transport check |
|---|---|
| Group remains present | Error |
| Runner reports cancellation | Error |
| Runner observed no completion file | `None` |
| Valid empty completion file | `Some(String::new())` |
| Valid nonempty completion file | `Some(result)` |

`read_result` opens the file relative to the held directory descriptor with
`openat`. A replacement of the directory pathname cannot redirect this read.
The open refuses a final symlink and cannot block on a FIFO.
`read_regular` requires a regular file, enforces the byte bound and checks
UTF-8. These checks matter because the managed process can change files.

In the example, the value is `Some("finished")`.
`run` converts `None` to `session_result_missing`, even if the child's exit
status is zero. The transport returns candidate text to `run`, not to the
caller. Resource cleanup and the final cancellation decision still follow.

## Clean up and write the result

After the runner returns, `run` reports its operation reports.
Without `--json`, each report has the form `ravel: <report>` on standard
error. With `--json`, `operationReports` belongs to the selection notice.
An operation report does not alone decide success, as the
[macOS example in chapter 3](03-a-complete-session.md#run-the-first-session) explains.

`run` then attempts these operations in order:

| Order | Operation | Purpose |
|---|---|---|
| 1 | `handlers.restore_entry_mask()` | Restores the entry mask while handlers still protect the transition |
| 2 | `channel.close()` | Removes the result channel's temporary files |
| 3 | `confined.remove()`, when applicable | Removes the private confined workspace |
| 4 | `output.restore()` | Restores the caller's standard output |
| 5 | `handlers.finalise(runner_signal)` | Joins cancellation observations and completes signal transitions |
| 6 | `settle(...)` | Reports failures and decides whether the result can proceed |

The example has no confined workspace, so that cleanup contributes success
without a removal. The other operations still run.
Channel cleanup checks the directory's recorded identity before removal.
It does not delete a replacement directory. This pathname check is not an
atomic defence against concurrent changes by same-user code.

`settle` is the step that decides the end of supervision from the collected
result, cleanup outcomes, group state and cancellation.
It reports failures, gives cancellation priority and rejects unsuccessful
cleanup. It does not perform those cleanup operations itself.
On this successful path it returns `finished` to `run`.

Excerpt from `crates/ravel/src/run.rs`: only a successful `settle` reaches
this final write and flush.

```rust
    io::stdout()
        .write_all(result.as_bytes())
        .and_then(|()| io::stdout().flush())
        .map_err(|error| failed("result_write_failed", error))?;
    Ok(())
```

`main` converts this success to status 0.
Ravel writes the result unchanged and adds no newline.
A failed final write can leave only the start of the value on standard
output. The invocation then fails, so output bytes alone do not prove success.
Signals that arrive after the final cancellation decision can still stop
the Ravel CLI during output. The write is not atomic with that decision.
