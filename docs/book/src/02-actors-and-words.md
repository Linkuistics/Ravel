# Who does what in a Ravel session

Remember the code review of chapter 1. An orchestrator asked for a review of
two files. Its prompt permitted two results, `approved` and `changes-needed`.
The owner's configuration chose Claude Code for the kind `review`. Ravel
launched Claude Code, and the LLM sent `changes-needed` with
`ravel end-session`.

Ravel stopped Claude Code, completed cleanup and returned
`changes-needed`. Then the orchestrator read the notes and requested a `fix`
session.

Three different participants made three different decisions in that review:

| Decision in the review | Who made it | The general rule |
|---|---|---|
| Review two files, and permit `approved` or `changes-needed` | The orchestrator, which is the caller | The [caller](#caller-and-owner) chooses the work, the prompt and the permitted results |
| Use Claude Code for the kind `review` | The owner, through the configuration script | The [owner](#caller-and-owner) chooses the program for each request |
| `changes-needed` is the result, after Claude Code stopped and cleanup completed | Ravel | Ravel decides whether the [session](#caller-and-owner), one run of `ravel start-session`, completed successfully |

If you know who makes each decision, you can change a rule in the
configuration script, and the caller does not change. You can also interpret
the outcome of a session. You do not confuse a response from the
configuration script with a completed session. This chapter names each
participant and each term that the book uses for it.

You do not run anything in this chapter. Each code block is a fragment of the
review, and its label says what it omits. Chapter 3 gives the first session
that you run.

## Caller and owner

The **caller** is the person or program that invokes Ravel and interprets the
result that Ravel returns. An orchestrator is one such caller. An orchestrator
is a program that runs LLM sessions in
[harnesses](#the-managed-process-and-its-result) as steps of its own work. A
harness is an agent program, such as Claude Code or Codex, in which an LLM
works on a prompt. The caller supplies the prompt, with the instructions to
end the session. The caller also decides the next step after the session.

A **session** is one `ravel start-session` invocation. It lasts from the
request until Ravel reports the result or the failure of the selected
program. A Ravel session is not a POSIX session that `setsid` creates. It is
also not a login session or the LLM session in a harness. Ravel supervises
the harness process, not the conversation of the LLM.

The caller supplies a **kind**. A kind is an open token that names the sort of
session that the caller wants. Ravel checks that the token is not empty and
gives it no meaning. The configuration script of the [owner](#caller-and-owner)
interprets it. For example, a caller can request `review`. But Ravel has no
registry of task kinds and does not know what a review includes.

The **owner** is the person who controls the **configuration** of Ravel for
the account. The configuration has two parts:

| Part | Default path | What it does |
|---|---|---|
| **Configuration script** | `$HOME/.config/ravel/config.rhai` | Rhai code that selects a program and its arguments, or refuses the request |
| **Settings** | `$HOME/.config/ravel/settings.json` | Values that the script cannot change, such as the deadline, limits, environment grants, harnesses and budgets |

The caller and the owner can be the same person, but their roles stay
different. The choice of the work does not set the rules that select a
command. The table shows which decisions stay separate, even when one person
has both roles.

| Role | Decides | Leaves to the other role |
|---|---|---|
| Caller | The requested work and the meaning of the result | Which command the configuration script selects |
| Owner | The configuration script that selects a command or refuses a request, and the settings | The caller's task and next step |

What is Rhai? Rhai is a small computer language for programs that are written
in Rust. Ravel contains the Rhai engine, so your configuration script runs in
Ravel's own binary, and you install no separate language runtime. Rhai looks
like a mix of JavaScript and Rust, and you do not declare types. A Rhai script
changes nothing outside itself, except through functions that Ravel gives it.
Ravel also limits how long a script runs and how much data it makes.

[Chapter 1](01-what-ravel-is-for.md#the-configuration-script-chooses-a-program)
shows a short configuration script and links to
[the Rhai Book](https://rhai.rs/book/), Rhai's own manual.
[Chapter 4](04-write-a-configuration-script.md) teaches the Rhai that a
configuration script needs.

By default, Ravel uses the account's configuration script,
`$HOME/.config/ravel/config.rhai`. `--config PATH` names a different script.
Ravel resolves that path relative to the caller's working directory. Ravel
does not search the caller's repository for a configuration script.

What does the configuration script give back? When it chooses a program, it
returns a **selection**. A selection is the map that the script's `select`
function returns with `status: "selected"`. It names a program and its
arguments. It also has the labels `provider`, `model`, `effort` and `reason`,
which describe the choice.

The **provider** is the owner's label for the model's origin. Ravel reports
it unchanged and never connects to it. The labels do not configure the
program. The configuration script puts the actual options, such as a model
option, in the arguments. Ravel runs that command without shell expansion or
word splitting. A shell runs only if the script selects one.

Then why does a selection have labels? They tell the caller what the
configuration script chose. After selection and before the launch, the Ravel
CLI writes the text **routing notice** as one line on the caller's standard
error. The caller, or the person who watches the terminal, reads the choice
there. Standard output stays free for the result. For example, a script that
returns `provider: "local"`, `model: "fake"` and `effort: "low"` for the kind
`review` gives the routing notice below.

Fragment: the routing notice of that selection. This block omits the other
lines on standard error.

```text
ravel: starting provider local, model fake, effort low for kind "review"
```

With `--json`, a JSON routing notice replaces that line, after the
supervision attempt.
[Chapter 5](05-running-sessions.md#read-notices-separately-from-results)
teaches both forms.

The [system-context figure in chapter 1](01-what-ravel-is-for.md#from-a-request-to-a-result)
shows these roles around Ravel. An orchestrator can choose the work, and the
owner chooses the harness. Ravel supervises that harness as the
[managed process](#the-managed-process-and-its-result), the program that the
configuration script selected for the session.

## Evaluation and supervision

<!-- tc: depart synonym-rotation because launch names the managed launch, while start names other process creation -->
The **Ravel CLI** is the `ravel` command-line program, written in Rust. During
`start-session`, it keeps the caller's environment, terminal and standard
output. It evaluates the request and supervises the selected
program. Its **runner** is the Rust library code that launches and supervises
the managed process. The runner runs in the process of the Ravel CLI. It is
not a separate service.

The **Rhai worker** is a separate process that evaluates the configuration
script for the Ravel CLI. The Ravel CLI launches a second copy of its own
executable, in an internal role, as a child process. Here, “worker” means a
process, not a thread. You install no separate language runtime.

Why does the Ravel CLI use a separate process? The parent stays in control
while the script runs:

| Reason | What the separate process permits |
|---|---|
| The deadline | When the deadline expires, the parent ends evaluation and can signal the worker process. Rhai cannot interrupt some of its own functions, such as `sleep`. A kernel stall can delay the worker's exit. |
| Host requests | The worker only computes. The parent performs each read or harness run that the script asks for and sends the data back. |
| Isolation of a failure | If the script exhausts the memory of the worker or overflows its stack, only the worker stops. The parent reports the failure and launches nothing. |

The worker does not run at the same time as the managed process. The Ravel
CLI waits until the worker exits, reaps it and removes its private directory.
Only then does it launch the selected program.

The worker has an empty environment, and the configuration script has no
direct filesystem, network or process access. A **host capability** is an
operation that the script asks the Ravel CLI to perform. These operations
include measured file reads and harness runs. The Ravel CLI controls each
**host call**, the script's request for an operation, and returns its data to
the script.

Ravel gives the worker a request with a size limit. The worker gets no
[session-result context](#three-different-kinds-of-information), the
`RAVEL_EXIT_FILE` path with which a program sends the result of a session.
Thus, the configuration script cannot end a session. The Ravel CLI checks the
worker's response. Only the Ravel CLI supervises the
[managed process](#the-managed-process-and-its-result). Evaluation and all
its host calls end before Ravel launches that process.

Do you want an LLM to help the configuration script choose the harness,
model, effort or prompt additions? The script can ask one through a harness
run.

A **harness run** is one execution of a program from the `harnesses` entry of
the settings. The configuration script requests it with `ravel::run_harness`
during evaluation. An **LLM call** is a harness run that asks a model a
question. The harness uses the owner's sign-in, and Ravel holds no model
credential. The script reads the answer as data and decides what it means.

An **evaluation budget** limits work during one evaluation of the
configuration script. For example, the owner sets the number of harness runs
in the settings. The script cannot raise that number or catch its
exhaustion. One deadline covers evaluation. Compare purposes and completion
rules:

| Execution | Purpose | How it ends |
|---|---|---|
| Rhai worker | Evaluate the configuration script | Return a selection or refusal, then exit |
| Evaluation harness run | Return data to the configuration script, such as an LLM answer | Natural exit or its own `end-session` result, followed by cleanup |
| Final managed process | Perform the work of the caller's session | Send the session result with `end-session`, followed by cleanup |

Each harness run has its own result channel. Its result reaches only the
configuration script, and cannot complete the outer session. The final managed
process receives a different channel after evaluation ends.

A **refusal** is a decision not to launch the final managed process. The
configuration script makes it, or Ravel makes it when it cannot accept the
file, behaviour or data of the script. Ravel does not try a different route.
A failure of the worker, a failure of supervision and a selection timeout are
different outcomes. A caller needs to tell a refusal from a failure during
evaluation or supervision. Evaluation harness runs can occur before the script
refuses.

<!-- tc: depart synonym-rotation because confirmation establishes completed cleanup, while checks also cover input validation -->
A valid selection alone is not sufficient to start the launch. Ravel also
confirms the cleanup conditions in this table before it launches the
selected program.

| Condition before launch | What the Ravel CLI confirms |
|---|---|
| Host-call completion | Every host call ended. Each harness run completed its required cleanup. |
| Worker exit and reap | The worker ended, and the Ravel CLI collected its exit status |
| Removal of the private directory | The Ravel CLI removed the worker's private directory |

## The managed process and its result

The **managed process** is the direct child that the configuration script
selects and that the Ravel CLI supervises. A process group is a set of
processes that can receive a signal as one unit. Cleanup applies to that
group, with all descendants that stay in it.

A **harness** is the agent program, such as Claude Code or Codex, in which an
LLM session works on the prompt. The harness gives the prompt to the LLM and
runs the commands that the LLM requests, for example shell commands. The
configuration script selects the harness command. Here, “harness” means the
agent program, not a test harness. Ravel supervises it as any other selected
program.

The configuration script decides whether to pass the prompt to that program.
Ravel does not require an LLM or a program that understands prompts. An
ordinary program can take the place of the harness and send a value with
`end-session`, as the examples do.

An interactive harness waits for more input when the LLM completes its task.
Thus, the caller's instructions to end the session tell the LLM to run
`ravel end-session` with a result. Follow that command to its result. The
program that Ravel launched, usually a harness that runs an LLM, runs
`ravel end-session`. Ravel then stops that program and the rest of its
process group. After cleanup, the sent value becomes the result of
`ravel start-session`.

The **opaque result** is text that a program in the session sends as one
UTF-8 argument with `ravel end-session`. In the prompt's instructions to end
the session, the caller gives the permitted values and tells when to use each
value. In the review, the orchestrator permitted `approved` and
`changes-needed`, and it gave a next action for each value. Ravel gives this
text no meaning. After successful supervision and cleanup, Ravel returns the
text unchanged. Then the caller decides its next step.

Fragment: the command that the review prompt asks the LLM to run when it finds
a problem. This block omits the prompt and the harness.

```sh
ravel end-session changes-needed
```

The LLM runs this command in a shell that the harness starts. That shell
inherits the path of the session's result file. In your own shell outside a
session, the same command fails with status 1, because it finds no such path.

The result is separate from the exit status of the managed process. It is
also separate from the files or other work that the program produced, such as
`review-notes.md`. A program can send an empty result. If no program sends a
result, the session has no result, even when the managed process exits with
status zero.

The **result channel** is the private temporary directory and completion file
through which `end-session` sends that value. Ravel makes it before the
launch and puts the path of the file in the `RAVEL_EXIT_FILE` environment
variable of the managed process.

Ravel keeps the first complete result that it receives. That first `end-session` requests shutdown. A
second `end-session` can still run before the shutdown stops its process.
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
successful session. The selection from the Rhai worker is also not a
successful session.

The figure shows why a response from the configuration script cannot complete
a session. The response reaches the Ravel CLI on a path that is separate from
the opaque result of the managed process. A harness that runs an LLM session
has the managed-process role. An ordinary program can take its place.

This C4 container view shows the processes and the temporary store of Ravel.
These are not operating-system containers. The result channel holds
temporary files, not a database. The
outline around them groups the software of Ravel, not a sandbox.

Steps 1 and 2 show the order: Ravel completes every host call and worker cleanup
before the final managed process starts. The participants do not all run at the same time.
The figure omits evaluation harness runs and their result channels.
Those runs return data to the configuration script through host replies, as the table above shows.

![The Ravel CLI launches a second copy of its own executable as the Rhai worker, with an empty environment and no session-result context. The worker evaluates the Rhai configuration script and sends host calls and its selection or refusal through a private socket. The CLI controls host calls and returns their replies. Every host call and worker cleanup ends before the final managed launch. This view omits evaluation harness runs and their separate channels. The final managed process sends its result through its own channel, and the CLI returns it to the caller only after successful cleanup.](figures/process-authority.svg)

[Open the process-and-authority figure at full size.](figures/process-authority.svg)

The table shows that the Ravel CLI supervises the session and decides its
final outcome. The caller and the owner keep their separate decisions.

| Participant | Decides or does | What Ravel lets it do |
|---|---|---|
| Caller, such as an orchestrator | Supplies the task and the instructions to end the session. Interprets the result. | Start the session and decide the next step |
| Owner | Writes the configuration script and the settings | Configure which commands the requests select |
| Rhai worker | Evaluates the configuration script and sends its response | Receive bounded request data and ask for host capabilities, with no environment or session-result context |
| Ravel CLI, with its runner | Checks the request, launches and supervises the program, and decides the outcome | Supervise processes, control the terminal and write the final result |
| Managed process, such as a harness | Runs the LLM session and the commands that it requests, which include `end-session`. Another selected program can take its place. | Use its own session-result context, which its children inherit |

## Three different kinds of information

The word “context” can mean three different inputs. The book names each one.
Thus, you do not confuse request data with permission to end a session.

The **session-result context** is the inherited `RAVEL_EXIT_FILE` path. It
identifies the result channel of the enclosing session. How does it reach
`end-session`? Ravel puts the path in the environment of the program that it
launches. A child process gets a copy of the environment of its parent,
unless the parent changes it. If each parent leaves the variable unchanged,
the shell and the `ravel end-session` that it starts have the same path.

`end-session` reads that path from its own environment. It reads no
configuration and accepts no target argument. It fails with status 1 if the
path is missing, empty or stale, that is, from a session that already ended.
Ravel has no persistent session lookup that it can use in place of that path.

In a nested launch, the outer managed process invokes a new `start-session`
as a caller. Ravel gives a new path to the inner managed process. This path
replaces the outer path only in the environment of that child. The outer
managed process keeps its own session-result context. Thus, with normal
inheritance, each `end-session` goes to its enclosing session, not to a
session that an ID selects.
[Chapter 1](01-what-ravel-is-for.md#how-end-session-finds-its-session) shows
both paths from a real nested run.

The table shows why an `end-session` with the inner path ends the inner
session. The outer process keeps its own path for the result.

| Process that runs `end-session` | Inherited session-result context | The result goes to |
|---|---|---|
| Outer managed process and its ordinary children | Path of the outer session | Result channel of the outer session |
| Inner managed process and its ordinary children | New path of the inner session | Result channel of the inner session |

The **selection context** is JSON data that the `select` function of the
configuration script receives as `context`. It comes from the caller's
context document, which `--context PATH` names, or from the optional
`load_context` function of the script. It can describe the work and the
sources that inform a choice. The script reads files through `ravel::read_text`
and `ravel::read_json`. For each successful read, Ravel records the canonical
name, the byte count and the SHA-256 digest of the bytes that it read.

The configuration script cannot read a file directly. A harness program can
read files, but Ravel does not measure those reads for the script. Selection
context is data for selection. It gives no permission to send a session
result, and it does not prove that the data is trustworthy.

The working directory of the caller is the `cwd` of the request. It tells the
configuration script where the caller works. It is not one of the two forms of
context above. The selected program starts in the working directory of the
caller. An owner uses the `cwd` of the request when the location of the
caller is important. The owner does not assume that the current directory of
the worker is the same.

The Ravel CLI starts the worker in an empty private directory.
The configuration script cannot change that directory or use it to read files.
Relative measured reads use `request.cwd`.
Relative imports use the directory of the file that imports them.
After the worker ends and Ravel reaps it, Ravel removes its private directory.

The table identifies the directory for each path operation or program execution.

| Operation | Directory used |
|---|---|
| Relative `--config` path | Caller's working directory |
| Relative import in the configuration script | Directory of the real file that imports the module |
| Relative measured read | Caller's `request.cwd` |
| Evaluation harness run or final managed process | Caller's working directory |

Use this table to keep selection data and the working directory separate from
the inherited path for the result.

| Information | Form | Purpose |
|---|---|---|
| Session-result context | Inherited `RAVEL_EXIT_FILE` path | Directs `end-session` to the result channel of the enclosing session |
| Selection context | JSON data | Informs the selection of the configuration script. Gives no permission to send a result. |
| Caller's working directory | `cwd` of the request | Names where the caller works. The selected program starts there. |

## Trust and confinement

The owner controls the configuration script and the settings. The separate
worker computes with restricted inputs, while the Ravel CLI performs its host
capabilities. A measured read can reach any regular file that the user can
read. A configured harness can access files and the network within its
permissions. These capabilities do not make arbitrary owner code safe.

**Confinement** is the optional restriction of the filesystem access and the
environment of the managed process. It is separate from the configuration
script. The script selects a command, and confinement restricts the access of
that command. `--confine` applies to the managed process and to evaluation
harness runs. It does not restrict measured reads or the Rhai worker. If the
request asks for confinement and Ravel cannot provide it, Ravel does not
launch the process unconfined.

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
these roles in one session with a small configuration script and a fake agent.
You make its directory, `~/ravel-first-session`, and save its two files in it.
The fake agent makes the selected command, the sent result and the cleanup
visible. It needs no agent account and no model service.
