# The actors and their words

Each request has three decisions:

- which work the caller wants
- which program does the work
- whether the [session](#caller-and-owner) completed successfully

Ravel gives these decisions to different participants. If you know who makes
each decision, you can change a rule in the owner policy without a change to
the [caller](#caller-and-owner). You can also interpret the outcome of a
session, and you do not confuse a policy response with a completed session.

## Caller and owner

The **caller** is the person or program that invokes Ravel and interprets the
result that Ravel returns. An orchestrator is one such caller. An orchestrator
is a program that runs LLM sessions in
[harnesses](#the-managed-process-and-its-result) as steps of its own work. The
caller supplies the prompt, with the instructions to end the session. The
caller also decides the next step after the session.

A **session** is one `ravel start-session` invocation. It starts with the
request and ends when Ravel reports the result or the failure of the selected
program. A Ravel session is not a POSIX session that `setsid` creates. It is
also not a login session or the LLM session in a harness. Ravel supervises
the harness process, not the conversation of the LLM.

The caller supplies a **kind**. A kind is an open token that names the sort of
session that the caller wants. Ravel checks that the token is not empty and
gives it no meaning. The policy code of the [owner](#caller-and-owner)
interprets it. For example, a caller can request `review`. But Ravel has no
registry of task kinds and does not know what a review includes.

The **owner** is the person who controls the configuration of Ravel for the
account. Their **owner policy** is executable TypeScript configuration. It
selects a program and its arguments, or it refuses the request. The caller
and the owner can be the same person, but their roles stay different. The
choice of the work does not set the rules that select a command.

The table shows which decisions stay separate, even when one person has
both roles.

| Role | Decides | Leaves to the other role |
|---|---|---|
| Caller | The requested work and the meaning of the result | Which command the owner policy selects |
| Owner | The policy that selects a command or refuses a request | The caller's task and next step |

By default, Ravel uses the account's policy file,
`$HOME/.config/ravel/policy.ts`. `--config PATH` names a different policy
file. Ravel resolves that path relative to the working directory of the
caller. Ravel does not search
the repository of the caller for a policy.

A selection contains a program, arguments and descriptive labels in the
`provider`, `model` and `effort` fields. These labels are text from the owner, not
built-in model integrations. The policy selects the actual command that
starts the chosen program. Ravel runs that command without shell expansion or
word splitting. A shell runs only if the policy selects one.

The [system-context figure in chapter 1](01-what-ravel-is-for.md#from-a-request-to-a-result)
shows these roles around Ravel. An orchestrator can choose the work, and the
owner chooses the harness. Ravel supervises that harness as the managed
process.

## Policy evaluation and supervision

<!-- tc: depart synonym-rotation because launch names the managed launch, while start names other process creation -->
The **Ravel CLI** is the `ravel` command-line program, written in Rust. During
`start-session`, it keeps the environment, the terminal and the standard
output of the caller. It evaluates the request and supervises the selected
program. Its **runner** is the Rust library code that launches and supervises
the managed process. The runner runs in the process of the Ravel CLI. It is
not a separate service.

The **policy worker** is a separate process that evaluates the owner policy
for the Ravel CLI. Its executable contains the Bun runtime. Thus, it can run
TypeScript policy and the modules that the policy imports, without a
separately installed runtime. Here, “worker” means a process. It is not a Bun
`Worker` thread, which policy code can itself create.

This separation lets policy run ordinary TypeScript, while only the Ravel CLI
supervises. Ravel gives the worker a request with a size limit and a
restricted environment. The worker gets no
[session-result context](#three-different-kinds-of-information). The Ravel
CLI checks the response of the worker. Only the Ravel
CLI supervises the [managed process](#the-managed-process-and-its-result).
Policy evaluation is complete before that process starts.

A **refusal** is a decision not to launch. The owner policy makes it, or Ravel
makes it when it cannot accept the file, behaviour or data of the policy.
Ravel does not try a different route. A failure of the worker, a failure of
supervision and a selection timeout are different outcomes. A caller needs to
tell a refusal from a failure during evaluation or supervision.

<!-- tc: depart synonym-rotation because confirmation establishes completed cleanup, while checks also cover input validation -->
A valid selection alone is not sufficient to start the launch. Ravel also
confirms both cleanup conditions in this table before it launches the
selected program.

| Condition before launch | What the Ravel CLI confirms |
|---|---|
| Worker exit and reap | The worker ended, and the Ravel CLI collected its exit status |
| Removal of the private directory | The Ravel CLI removed the worker's private directory |

## The managed process and its result

The **managed process** is the direct child that owner policy selects and that
the Ravel CLI supervises. Cleanup applies to its full process group, with all
descendants that stay in that group.

A **harness** is the agent program, such as Claude Code or Codex, in which an
LLM session works on the prompt. The harness gives the prompt to the LLM and
runs the commands that the LLM requests, for example shell commands. Owner
policy selects the harness command. Here, “harness” means the agent program,
not a test harness. Ravel supervises it as it supervises any other selected
program.

The policy decides whether to pass the prompt to that program.
Ravel does not require an LLM or a program that understands prompts. An
ordinary program can take the place of the harness and send a value with
`end-session`, as the examples do.

An interactive harness waits for more input when the LLM completes its task.
Thus, the caller's instructions to end the session tell the LLM to run
`ravel end-session`. This command sends a result. Then Ravel stops all
processes that still run in the process group of the managed process. Ravel
completes the cleanup before it returns the result.

The **opaque result** is text that a program in the session sends as one
UTF-8 argument with `ravel end-session`. In the prompt's instructions to end
the session, the caller gives the permitted values and tells when to use
each value. For example, the caller can choose `continue` to mean that it can
start its next task. Ravel gives this text no meaning. After successful supervision and
cleanup, Ravel returns the text unchanged. Then the caller decides its next
step.

The result is separate from the exit status of the managed process. It is
also separate from the files or other work that the program produced. A
program can send an empty result. If no program sends a result, the session
has no result, even when the managed process exits with status zero.

The **result channel** is the private temporary directory and completion file
through which `end-session` sends that value. Ravel keeps the first complete
result that it receives. That first `end-session` requests shutdown, which
normally ends the harness. A second `end-session` races that shutdown.
For example, another process in the group can send a result before Ravel
stops it. Ravel keeps the first result, even if the second command succeeds.

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

The table shows the short interval in which another process can still send
a result. These waits end early if the managed process exits.

| Point after Ravel detects the result | What Ravel does |
|---|---|
| First 2 seconds | Gives the managed process time to exit |
| After 2 seconds, if the managed process still runs | Sends TERM |
| Up to 5 seconds after TERM | Waits for the managed process to exit |
| After that wait, if the managed process still runs | Sends KILL |
| After the managed process exits | Stops any group members that remain, then reaps the managed process |

The result channel carries a request to end the session. That request does
not prove that cleanup is complete.

Before the Ravel CLI returns success, it confirms the reap, the absence of the
process group and the removal of the session's temporary files.
Cancellation suppresses the result, even if Ravel received it before the
cancellation.

The result channel uses the completion channel of the runner. That completion
channel only reports that a file appeared. Ravel's code for the result
channel checks and reads the contents. The appearance of the file is not a
successful session. The selection from the policy worker is also not a successful
session.

The figure shows why a policy response cannot complete a session. The
response gets to the Ravel CLI on a path that is separate from the opaque
result of the managed process. A harness that runs an LLM session has the
managed-process role. An ordinary program can take its place.

This C4 container view shows the processes and the temporary store of Ravel.
These are not operating-system containers. The result channel holds
temporary files, not a database. The
outline around them groups the software of Ravel, not a sandbox.

Steps 1 and 2 show the order: Ravel completes the cleanup of the worker
before the managed process starts. The participants in the figure do not all
run at the same time.

![An orchestrator or a person requests a session, and the Ravel CLI sends the request to a separate policy worker. The worker evaluates trusted owner policy without session-result context and sends a selection or a refusal through a separate socket. The Ravel CLI confirms that the worker ended, that it reaped the worker and that it removed the worker's private directory. Then the runner in the Ravel CLI launches and supervises the selected harness or other program. The LLM tells its harness to run end-session, which uses the inherited session-result context to send the result into the temporary result channel. The Ravel CLI reads and checks the sent result, stops the group members that remain and writes it to the caller only after cleanup succeeds.](figures/process-authority.svg)

[Open the process-and-authority figure at full size.](figures/process-authority.svg)

The table shows that the Ravel CLI supervises the session and decides its
final outcome. The caller and the owner keep their separate decisions.

| Participant | Decides or does | What Ravel lets it do |
|---|---|---|
| Caller, such as an orchestrator | Supplies the task and the instructions to end the session. Interprets the result. | Start the session and decide the next step |
| Owner | Writes the policy code and the settings | Configure which commands the requests select |
| Policy worker | Evaluates the owner policy and sends its response | Receive request data with a size limit, in a restricted environment, with no session-result context |
| Ravel CLI, with its runner | Checks the request, launches and supervises the program, and decides the outcome | Supervise processes, control the terminal and write the final result |
| Managed process, such as a harness | Runs the LLM session and the commands that it requests, which include `end-session`. Another selected program can take its place. | Use its own session-result context, which its children inherit |

## Three different kinds of information

The word “context” can mean three different inputs. The book names each one.
Thus, you do not confuse request data with permission to end a session.

The **session-result context** is the inherited `RAVEL_EXIT_FILE` path. It
identifies the result channel of the enclosing session. `end-session` uses
that path. It reads no policy or settings and accepts no target argument. It
fails if the path is missing, empty or stale, that is, from a session that
already ended. Ravel has no persistent session lookup that it can use in
place of that path.

In a nested launch, the outer managed process invokes a new `start-session`
as a caller. Ravel gives a new path to the inner managed process. This path
replaces the outer path only in the environment of that child. The outer
managed process keeps its own session-result context. Thus, with normal
inheritance, each `end-session` goes to its enclosing session, not to a
session that an ID selects.

The table shows why an `end-session` with the inner path ends the inner
session. The outer process keeps its own path for the result.

| Process that runs `end-session` | Inherited session-result context | The result goes to |
|---|---|---|
| Outer managed process and its ordinary children | Path of the outer session | Result channel of the outer session |
| Inner managed process and its ordinary children | New path of the inner session | Result channel of the inner session |

The **policy context** is version-1 JSON data that the `select` function of
the policy receives. It comes from the caller or from the optional
policy-context loader of the policy. It can describe the work and the sources
that inform a choice. Ravel measures each read through the host operations
that it provides. For each source, it records the canonical name, the byte
count and the SHA-256 digest of the bytes that it read.

Policy code can also read data directly, but Ravel does not measure these
reads. Policy context is data for selection. It gives no permission to send a
result, and it does not prove that the data is trustworthy.

The working directory of the caller is the `cwd` of the request. It tells the
policy where the caller works. It is not one of the two forms of context
above. The selected program starts in the working directory of the caller. A
policy author uses the `cwd` of the request when the location of the caller is
important. The author does not assume that the current directory of the
worker is the same.

The Ravel CLI creates the private directory of the worker, which is empty, and
starts the worker there. The worker changes to `/` before it registers its
embedded modules or imports owner policy. Policy code can then change the
working directory of the worker again. During worker cleanup, before it
launches the selected program, the Ravel CLI removes the private directory of
the worker.

The table separates the start location of the worker from the location where
policy code starts to run. It also shows that the removal of the private
directory occurs before the launch.

| Point in policy evaluation | Fact about the working directory |
|---|---|
| Worker startup | The Ravel CLI starts the worker in its empty private directory |
| Before policy import | The worker changes to `/`. Then policy code can change its working directory. |
| Worker cleanup, before the managed launch | After the worker ends and the Ravel CLI reaps it, the Ravel CLI removes the private directory of the worker |

Use this table to keep selection data and the working directory separate from
the inherited path for the result.

| Information | Form | Purpose |
|---|---|---|
| Session-result context | Inherited `RAVEL_EXIT_FILE` path | Directs `end-session` to the result channel of the enclosing session |
| Policy context | Version-1 JSON data | Informs the selection of the owner policy. Gives no permission to send a result. |
| Caller's working directory | `cwd` of the request | Names where the caller works. The selected program starts there. |

## Trust and confinement

Owner policy is trusted executable code. The worker is separate and gets
restricted inputs. Thus, policy has no part in the normal paths of supervision
and of the result. But this separation does not make arbitrary
owner code safe. Policy can access files or the network and can start
unrelated processes. Its detached children are outside the cleanup guarantee
of the worker.

**Confinement** is the optional restriction of the filesystem access and the
environment of the managed process. It is separate from owner policy. Owner
policy selects a command, and confinement restricts the access of that
command. `--confine` applies to the managed process, not to the policy worker.
If the request asks for confinement and Ravel cannot provide it, Ravel does
not launch the process unconfined.

The inherited session-result context keeps nested sessions separate while
each process uses the path that it inherited. It is not a security check
against code that runs as the same user. That code can tamper with files or
launch unrelated processes. Also, descendants that leave the process group of
the managed process are outside its cleanup. If KILL stops the Ravel CLI,
the managed process can continue to run without a supervisor. The Ravel CLI
cannot catch KILL.

These limits are important when you decide what a caller can trust. A
successful Ravel result confirms the supervision and cleanup that this
chapter describes. It does not confirm that Ravel contained arbitrary
same-user code. It also does not confirm that the work for the task of the
caller is correct.

[Chapter 3, “A complete session, end to end”](03-a-complete-session.md), uses
these roles in one session with a small policy and a fake agent. The fake
agent makes the choice of command, the sent result and the cleanup visible. It
needs no agent account and no model service.
