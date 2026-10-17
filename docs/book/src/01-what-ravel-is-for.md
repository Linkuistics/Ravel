# What Ravel is for

Ravel runs
[LLM sessions in agent harnesses](02-actors-and-words.md#the-managed-process-and-its-result),
such as Claude Code and Codex, as supervised parts of another program. A
harness is the program in which an LLM works on a prompt: it passes the prompt
to the LLM and runs the commands the LLM asks for, such as shell commands. An
[orchestrator](02-actors-and-words.md#caller-and-owner), a program that runs
LLM sessions as steps of its own work, can ask Ravel for a
[session](02-actors-and-words.md#caller-and-owner), wait for its result, then
use that result to decide what to do next. The person who configures Ravel
chooses which harness runs each request. Ravel starts it, supervises it, and
returns its published result after cleanup succeeds.

Consider an orchestrator that asks an LLM to review a code change. The
orchestrator knows the files to review and what to do with the answer. The
person running it wants to choose the harness for their account. Ravel
separates those decisions: the orchestrator asks for a review, the person's
configuration selects a harness command, and Ravel runs it. An interactive
harness waits for more input when the LLM finishes its task. Ravel's
`end-session` command lets the LLM publish a result and ask Ravel to stop the
session, so the orchestrator can continue after cleanup.

## From a request to a result

The person or program making the request is the
[caller](02-actors-and-words.md#caller-and-owner). It invokes
`ravel start-session` with a prompt and a
[kind](02-actors-and-words.md#caller-and-owner): a token naming the sort of
work it wants. In the review scenario, the caller might use `review` as the
kind. That spelling is a convention the caller and configuration agree on;
Ravel has no built-in review category.

The [owner](02-actors-and-words.md#caller-and-owner) controls the account's
configuration. Their [owner policy](02-actors-and-words.md#caller-and-owner)
is TypeScript code that receives the request and either selects a program
and its arguments or refuses to launch. Ravel passes the prompt to that policy
unchanged. The policy decides how to put it into the selected command's
arguments. Ravel adds no task instructions.

One `start-session` invocation is a
[session](02-actors-and-words.md#caller-and-owner). For the length of the
session, Ravel supervises the program the policy selected, which runs as the
session's [managed process](02-actors-and-words.md#the-managed-process-and-its-result).
The harness runs the LLM session that works on the prompt. That LLM session
is not the Ravel session: Ravel supervises the harness process, not the LLM's
conversation inside it. Ravel supervises the harness as it would any other
process, so an ordinary program that uses `end-session` can stand in for it. The book's runnable examples use fake
agents in this way.

An interactive harness does not exit when its LLM finishes the task; it waits
for more input. The caller's prompt therefore tells the LLM to run
`ravel end-session RESULT` when it is done, and supplies the result values the
caller understands. That command
publishes one [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
and asks Ravel to end the session. “Opaque” means that Ravel carries
the value without interpreting it. If the caller chooses `continue` as a
value, Ravel returns those bytes; the caller decides whether to start more
work. `end-session` finds the enclosing session through its inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
It takes no session ID or target argument.

Publishing the result with `end-session` is a request to end the session, not
proof that cleanup succeeded. The managed process's process group is the
operating system's grouping of that process with its descendants. Ravel stops
the harness and any other processes still running in that group, and cleans
up the whole group. Before returning a successful result, Ravel confirms that
the managed process has ended and been reaped, that its process group is gone,
and that the session's temporary files have been removed. Reaping is the
parent collecting a finished child's exit status.
[Chapter 3, “A complete session, end to end”](03-a-complete-session.md#launch-the-managed-job),
explains those mechanics as a session reaches them.

The figure separates the caller's choice of work from the owner's choice of
program, with Ravel responsible for supervision between request and result.
The C4 system-context view treats Ravel as one system. Blue identifies Ravel,
grey the external caller and managed process, and slate the human owner.
Green arrows trace the opaque result. These shapes describe responsibilities,
not security boundaries.

![The owner configures Ravel through owner policy and settings. The caller sends a kind and prompt with start-session. Ravel launches and supervises the managed process, which publishes an opaque result with end-session. Ravel returns that result to the caller on standard output only after successful cleanup.](figures/system-context.svg)

[Open the system-context figure at full size.](figures/system-context.svg)

A successful `start-session` writes the exact result bytes to standard output,
without adding a newline. The managed process's standard output is redirected
to standard error, where its output and Ravel's diagnostics can be kept apart
from the result stream. The caller checks whether `start-session` succeeded
before interpreting that stream. A failure while writing the final result can
leave only a prefix, so bytes alone do not prove success.

Several outcomes carry no successful result. A
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision) means no
program is launched. If the managed process exits and no result was
published, the session has no result and fails, even if the exit status is
zero. Cancellation of the session suppresses a result even if it was already
published. A supervision or cleanup failure also prevents success. Conversely,
a session with a published result can succeed after cleanup even if the
managed process then exits with a nonzero status. The result is the value the caller's ending instructions asked for,
the exit status reports how the process ended, and the work is whatever the
program changed; none of the three stands in for another.

The table keeps the three outputs separate so that an exit status or a changed
file cannot be mistaken for the result the caller requested.

| What the session produces | What it tells the caller | What it does not prove |
|---|---|---|
| Opaque result | The value published with `end-session`, returned after successful supervision and cleanup | That the requested work is correct |
| Managed process's exit status | How that process ended | That it published a result or that the session succeeded |
| Work, such as changed files | What the selected program changed | That it published a result or completed cleanup |

## Decide whether Ravel fits

Ravel fits when your program needs to use an LLM session in an interactive
harness as a step in its own work. Your program can leave the choice of
harness to the account's owner policy and rely on the same ending command,
`end-session`, whichever harness that policy selects. Ravel supervises the
session and returns its explicit result after cleanup. Your program still owns the task and the decisions made
after the session ends.

Use this table to decide which part of your need Ravel meets and which part
still needs a caller, service or selected program.

| If you need… | Ravel's part | What remains outside Ravel |
|---|---|---|
| Different owners to choose different programs for the same request | Evaluate each owner's policy and run its selected command | The caller's task meaning and the owner's routing choices |
| A result after a program and its job have been cleaned up | Supervise the launch and return the explicit result after successful cleanup | The caller's interpretation and any further work |
| Task queues, schedules, retries or durable history | Execute a session when invoked | A caller or service that stores and schedules work |
| Model access, credentials, usage metering or model selection rules | Run the command chosen by policy | The selected program's model integration and the owner's selection rules |
| A host that keeps doing other work after a launch | The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) supervises one session and exits | A separate long-running host, which Ravel does not provide |

The Ravel CLI, the `ravel` command-line program, is a **launcher CLI**: one
invocation supervises one managed launch, reports its result or failure, then
exits. A **persistent runner** would be a
host that keeps running after a launch has finished. Ravel does not provide
one, and a program that exits after one session does not provide what such a
host would need.

Ravel has no persistent session store or history lookup. Its
[result channel](02-actors-and-words.md#the-managed-process-and-its-result)
uses temporary files for one session; it is not a record of past work. If a
caller needs a lasting record, that caller saves it. Owner policy is executable
code and can itself use files or network services; the absence of a Ravel store
does not restrict what that trusted code can do.

Grove, a task-tree driver, is one illustrative caller: an orchestrator that
runs LLM sessions in harnesses as steps of work on its task tree. It supplies its own
task kinds and interprets its own ending values. Those conventions belong to
Grove. Another caller can use different kinds and results without a Ravel
adapter.

The supervision guarantee also has limits. A descendant that moves itself out
of the managed process's process group is not reached by that group's cleanup,
and the session can succeed while it still runs. Detached children started by policy
are outside the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)'s
cleanup guarantee. If the Ravel CLI is killed abruptly, for example with SIGKILL,
which it cannot catch, the managed process or temporary files can be left
behind. Code running as the same user can tamper with files, including
Ravel's; keeping sessions separate is not a security check against that code.
[Chapter 2](02-actors-and-words.md#trust-and-confinement) explains why the
separation between these roles is useful within those limits.

## Choose a reading route

The book assumes everyday shell, Git and JSON use, the ability to read
TypeScript and Rust, and general operating-system knowledge. It teaches
Ravel's domain and the job-control details its behaviour depends on.

- To call Ravel from your own tool, read chapters 1–3, then
  [chapter 5, “Running sessions”](05-running-sessions.md).
  [Chapter 6, “Reference”](06-reference.md), is the lookup chapter.
- To write owner policy, read chapters 1–3, then
  [chapter 4, “Writing an owner policy”](04-writing-an-owner-policy.md),
  with [chapter 6](06-reference.md) for lookup.
- To change Ravel, read chapters 1–3, then chapter 7, “How Ravel is built”,
  chapter 8, “Following a session through the code”, and chapter 11,
  “Developing Ravel”. Chapter 9, “Jobs, signals and the terminal”, and chapter
  12, “Delivering Ravel”, provide the mechanics and delivery details.
- To reuse a component, read chapters 1 and 2, then chapter 10, “Using
  Ravel's components in your project”, with chapters 7 and 9 as background.
  Chapter 10 is unwritten until a supported extracted component and its
  consumer exist; Ravel's internal package boundaries are not, on their own, a
  supported interface for another project.

Reading the chapters in order is the learning route. The first practical
session uses a tiny policy and a fake agent: an ordinary program that publishes
a result with `end-session` in place of a harness running an LLM session. Learning
the protocol therefore needs no model service or personal agent account.
Installing and configuring real agents is outside the book's scope.

## Understand the evidence limits

How Ravel starts a session and cleans it up has been checked by recorded
measurements on aarch64 macOS with Rust 1.99, and on native aarch64 GNU/Linux
with Rust 1.85.0, the declared minimum. Each measurement covers only the
platform, kernel, C library and compiler it records. The x86_64 GNU/Linux
target has been built, linted at Rust 1.85.0 and run under QEMU user-mode
emulation, which translates x86-64 programs to run on the host's own kernel,
so no x86-64 kernel takes part. No x86-64 kernel is qualified. These are limits
on the evidence: the three targets do not have equivalent support
measurements. Chapter 12 explains the delivery and platform limits in detail.

The comparison shows why a built or emulated target does not carry the same
evidence as a measured native target.

| Target | Recorded execution | Compiler | Limit |
|---|---|---|---|
| aarch64 macOS | Native measurements | Rust 1.99 | Only the recorded platform and compiler |
| aarch64 GNU/Linux | Native measurements | Rust 1.85.0, the declared minimum | Only the recorded kernel, C library and compiler |
| x86_64 GNU/Linux | Built, linted and run under QEMU user-mode emulation | Rust 1.85.0 | Uses the host's kernel; no x86-64 kernel is qualified |

The book's reader checkpoints use the owner as a labelled stand-in for a
developer new to Ravel. Those reads can show the owner's results and
find inaccuracies; whether a new developer can use the book remains unknown.
The local book can be built and inspected, but GitHub's rendering remains
unchecked until the branch is pushed, and the published site until the first
release carrying the book.
