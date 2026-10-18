# What Ravel is for

Ravel runs
[LLM sessions in agent harnesses](02-actors-and-words.md#the-managed-process-and-its-result),
such as Claude Code and Codex, as supervised parts of another program. A
harness is the program in which an LLM works on a prompt. It gives the prompt
to the LLM and runs the commands that the LLM requests, for example shell
commands. The person who configures Ravel chooses which harness runs each
request. Ravel starts that harness and supervises it. After cleanup succeeds,
Ravel returns the result that the session sent.

A harness such as Claude Code or Codex can run under the subscription of its
user. Thus, a caller can use that subscription for LLM work instead of paid
API access. The cost can be much lower for work that the subscription covers.
The [Claude Code documentation](https://code.claude.com/docs/en/authentication)
and [Codex documentation](https://learn.chatgpt.com/docs/auth) explain their
access options and link to their plans and terms. This paragraph agrees with
those pages as they were on 18 October 2026.

An [orchestrator](02-actors-and-words.md#caller-and-owner) is a program that
runs LLM sessions as steps of its own work. It can request a
[session](02-actors-and-words.md#caller-and-owner) from Ravel and wait for
its result. Then it uses that result to decide its next step.

For example, an orchestrator can ask an LLM to review a code change. The
orchestrator knows the files to review and what to do with the answer. The
person who operates the orchestrator wants to choose the harness for their
account. Each participant has a different responsibility in that review:

| Participant | Responsibility in the review |
|---|---|
| Orchestrator | Choose the work to review and interpret the result |
| Person who configures Ravel | Choose the harness command through their policy |
| Ravel | Run the selected command and supervise it until cleanup completes |

An interactive harness waits for more input when the LLM completes its task.
Ravel's `end-session` command lets the LLM send a result and ask Ravel to
stop the session. Thus, the orchestrator can continue after cleanup.

## From a request to a result

The person or program that makes the request is the
[caller](02-actors-and-words.md#caller-and-owner). The caller invokes
`ravel start-session` with a prompt and a
[kind](02-actors-and-words.md#caller-and-owner). A kind is a token that names
the sort of work that the caller wants. In the review example above,
the caller can use `review` as the kind. The caller and the owner
policy agree on that token. Ravel has no built-in review category.

The [owner](02-actors-and-words.md#caller-and-owner) controls the
configuration of the account. Their
[owner policy](02-actors-and-words.md#caller-and-owner) is a configuration
file with TypeScript code. Its default path is
`$HOME/.config/ravel/policy.ts`. The policy receives the request. Then it
selects a program and its arguments, or it refuses to launch a program.

The policy can use the kind to choose a harness and the arguments that set
the model and the effort level of that harness. The `provider`,
`model` and `effort` fields of a selection are labels for the notice that
Ravel writes when the session starts. These labels do not configure the
harness. The policy puts the actual options in the arguments of the command.

The caller can also use the kind to choose instructions for the prompt. If
the harness supports skills, the prompt can tell the LLM which skill to load.
A skill is a set of instructions for a particular task. This use of the kind
is a caller convention. Ravel gives the prompt to the policy unchanged and
adds no task instructions. The policy decides whether and how to include the
prompt in the arguments of the selected command.

One `start-session` invocation is a
[session](02-actors-and-words.md#caller-and-owner). During the session, Ravel
supervises the program that the policy selected. This program is the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
of the session. The harness runs the LLM session that works on the prompt.
That LLM session is not the Ravel session. Ravel supervises the harness
process, not the conversation of the LLM in that process.

The contract with the selected program is loose. The policy selects a
program, Ravel runs it, and a program in the session sends one value with
`end-session`. Ravel needs nothing more from the program. The selected
program does not have to use an LLM or understand a prompt.

The table separates what Ravel needs from the harness use that this book
teaches. The runnable examples use ordinary programs as fake agents.

| Part of a session | What Ravel needs | What this book expects from a harness |
|---|---|---|
| Selected command | The policy selects a program and its arguments | The program runs an LLM session |
| Prompt | The caller supplies `--prompt` or `--prompt-file`. Ravel gives that text to the policy unchanged. The policy decides whether the program receives it. | The policy passes the prompt to the harness |
| Result | A program in the session sends one value with `end-session` | The LLM follows the caller's instructions and tells the harness to run that command |

An interactive harness does not exit when its LLM completes the task. It
waits for more input. Thus, the caller's prompt tells the LLM to run
`ravel end-session RESULT` when the task is complete. `RESULT` is a
placeholder for the text to send. The caller's prompt gives the permitted
values and tells the LLM when to use each value.

For example, the caller can choose the text `continue` as a result value. The
prompt can tell the LLM to run `ravel end-session continue` when the review is
complete. The command sends that text to Ravel and asks Ravel to end the
session. After successful supervision and cleanup, Ravel returns the text
unchanged. The caller can interpret `continue` as permission to start its
next task. Ravel gives `continue` no meaning and does not start a next task.

The book calls this text an
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result),
because Ravel does not interpret it. `end-session` finds the enclosing session
through its inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
It takes no session ID or target argument.

When `end-session` sends the result, it requests the end of the session. This
request does not prove that cleanup succeeded. A process group is an
operating-system unit that contains the managed process and its descendants.
Ravel stops the harness and each other process that remains in that group.
Then it completes the cleanup of the group.

To reap a child process, its parent collects the exit status of that child.
Before Ravel returns a successful result, it confirms that the managed
process ended and that Ravel reaped it. Ravel also confirms that the process
group is gone and that it removed the temporary files of the session.
[Chapter 3, “A complete session, end to end”](03-a-complete-session.md#launch-the-managed-job),
explains these mechanics at the point where a session needs them.

The table shows the steps after the LLM completes its task. The caller
receives a successful result only after Ravel stops the managed process and
its process group and completes the cleanup.

| Step after the LLM completes its task | Who acts and what happens |
|---|---|
| Send | The LLM tells the harness to run `ravel end-session RESULT`. This command sends a value and requests the end of the session. |
| Stop | Ravel stops all processes that still run in the process group of the harness. |
| Confirm cleanup | Ravel confirms that it reaped the managed process, that the group is gone and that it removed the temporary files of the session. |
| Return | After successful supervision and cleanup, Ravel returns the result unchanged. The caller interprets the result and decides its next step. |

The figure shows Ravel between an orchestrator that requests work and a
harness that runs an LLM session. The owner chooses the harness. Ravel
supervises the harness until cleanup lets the result return. Other callers
and selected programs use the same boundary.

The C4 system-context view shows Ravel as one system. These shapes show
responsibilities, not security boundaries.

![The owner configures which harness runs the caller's request. An orchestrator or a person sends a kind and a prompt with start-session. Ravel launches and supervises the harness, or another selected program, as the managed process. The LLM tells the harness to run end-session, which sends an opaque result and asks Ravel to stop. Ravel stops the group members that remain. Ravel returns the result to the caller on standard output only after successful cleanup.](figures/system-context.svg)

[Open the system-context figure at full size.](figures/system-context.svg)

After a successful session, Ravel writes the result unchanged to standard
output. Ravel does not interpret the result and adds no newline. Ravel redirects the standard output of the
managed process to standard error. Thus, the caller can keep the output of
the managed process and the diagnostics of Ravel separate from the result
stream.

The caller checks whether `start-session` succeeded before it interprets that
stream. A failure during the write of the final result can leave only the
start of the result on standard output. Thus, the output alone does not prove
success.

Some outcomes return no result to the caller:

- A [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) means
  that Ravel launches no program.
- If the managed process exits and no program sent a result, the session has
  no result and fails. This is also true when the exit status is zero.
- Cancellation of the session suppresses the result, even if a program sent
  the result before the cancellation.
- A supervision failure or a cleanup failure also prevents success.

But a session with a sent result can succeed after cleanup, even if the
managed process then exits with a nonzero status. The result is the value
that the caller's instructions to end the session asked for. The exit status
tells how the process ended. The work is what the program changed. None of these three
outputs can replace another.

The table keeps the three outputs separate. Thus, you do not confuse an exit
status or a changed file with the result that the caller requested.

| What the session produces | What it tells the caller | What it does not prove |
|---|---|---|
| Opaque result | The value sent with `end-session`, returned after successful supervision and cleanup | That the requested work is correct |
| Managed process's exit status | How that process ended | That it sent a result or that the session succeeded |
| Work, such as changed files | What the selected program changed | That it sent a result or that the cleanup completed |

## Decide whether Ravel fits

Ravel fits when your program needs to use an LLM session in an interactive
harness as a step of its own work. Your program can let the owner policy of the
account choose the harness. In each harness that the policy selects, the LLM
uses the same command, `end-session`, to send the result. Ravel supervises the
session and returns its explicit result after cleanup. Your program still owns
the task and the decisions after the session ends.

Use this table to find which part of your need Ravel meets. The other parts
need a caller, a service or a selected program.

| If you need… | Ravel's part | What stays outside Ravel |
|---|---|---|
| Different owners to choose different harnesses for the same LLM task | Evaluate the policy of each owner and run the command that it selects | The caller's task meaning and the owner's policy decisions |
| An interactive harness to finish as a step of your program | Accept the result that `end-session` sends, stop the group members that remain and return the result after successful cleanup | The caller's instructions to end the session, the interpretation of the result and all further work |
| Task queues, schedules, retries or durable history | Run a session when a caller invokes Ravel | A caller or service that stores and schedules work |
| Model access, credentials, usage measurement or rules for model selection | Run the command that the policy chose | The harness's model integration and the owner's selection rules |
| A host that continues to do other work after a launch | The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) supervises one session and then exits | A separate host that continues to run after each launch, which Ravel does not provide |

The Ravel CLI, the `ravel` command-line program, is a **launcher CLI**. One
invocation supervises one managed launch, reports its result or failure, and
then exits. A **persistent runner** is a host that continues to run after a
launch is complete. Ravel does not provide one. A program that exits after one
session does not give what such a host needs.

Ravel has no persistent session store and no history lookup. Its
[result channel](02-actors-and-words.md#the-managed-process-and-its-result)
uses temporary files for one session. It is not a record of past work. If a
caller needs a permanent record, that caller saves it. Owner policy is
executable code and can itself use files or network services. Ravel has no
store, but the absence of a store does not restrict what that trusted code
can do.

Grove, a task-tree driver, is one illustrative caller. It is an orchestrator
that runs LLM sessions in harnesses as steps of the work on its task tree. It
supplies its own task kinds and interprets its own result values. These
conventions belong to Grove. Another caller can use different kinds and
results without a Ravel adapter.

The supervision guarantee has limits. This table shows the consequence of
each limit for the caller.

| Condition | Limit of the guarantee |
|---|---|
| A descendant leaves the process group of the managed process | Group cleanup does not reach it. The session can succeed while that descendant continues to run. |
| Policy starts detached children | The cleanup guarantee of the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision) does not cover them. |
| The Ravel CLI stops abruptly, for example because of SIGKILL | The managed process or temporary files can remain. The Ravel CLI cannot catch SIGKILL. |
| Other code runs as the same user as Ravel | That code can tamper with Ravel's files. The separation of sessions is not a security check against that code. |

[Chapter 2](02-actors-and-words.md#trust-and-confinement) explains why the
separation of these roles is useful within these limits.

## Choose a route through the book

The book assumes everyday use of shell, Git and JSON, the ability to read
TypeScript and Rust, and general operating-system knowledge. It teaches the
domain of Ravel and the job-control details that the behaviour of Ravel
depends on.

Choose the route that serves your task. The table gives the chapter order
and the chapters for further detail.

| Your task | Read in this order | Further detail |
|---|---|---|
| Call Ravel from your own tool | Chapters 1–3, then [chapter 5, “Run sessions”](05-running-sessions.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Write an owner policy | Chapters 1–3, then [chapter 4, “Write an owner policy”](04-writing-an-owner-policy.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Change Ravel | Chapters 1–3, then 7, “How Ravel is built”, 8, “Follow a session through the code”, and 11, “Develop Ravel” | Chapter 9, “Jobs, signals and the terminal”, and chapter 12, “Deliver Ravel” |
| Reuse a component | Chapters 1 and 2, then chapter 10, “Use Ravel's components in your project”, when it exists | Chapters 7 and 9 for background |

Chapter 10 has no text until a supported extracted component and its consumer
exist. The internal package boundaries of Ravel alone are not a supported
interface for another project.

The learning route is to read the chapters in order. The first practical
session uses a small policy and a fake agent. A fake agent is an ordinary
program that sends a result with `end-session`. It takes the place of a
harness that runs an LLM session. Thus, you can learn the protocol without a
model service or a personal agent account. The installation and the
configuration of real agents are outside the scope of the book.

## Understand the evidence limits

Recorded measurements checked how Ravel starts a session and completes its
cleanup on two platforms. One platform is aarch64 macOS with Rust 1.99. The
other platform is native aarch64 GNU/Linux with Rust 1.85.0, the declared
minimum. Each measurement applies only to the platform, kernel, C library and
compiler that it records.

The project built the x86_64 GNU/Linux target, linted it at Rust 1.85.0 and
ran it only under QEMU user-mode emulation. This emulation translates x86-64
programs to run on the kernel of the host. Thus, no x86-64 kernel takes part.
No x86-64 kernel is qualified.

These facts are limits of the evidence. The three targets do not have the
same evidence. Chapter 12 explains the limits of delivery and of the platforms
in detail.

The table shows why a built or emulated target does not have the same
evidence as a measured native target.

| Target | Recorded execution | Compiler | Limit |
|---|---|---|---|
| aarch64 macOS | Native measurements | Rust 1.99 | Only the recorded platform and compiler |
| aarch64 GNU/Linux | Native measurements | Rust 1.85.0, the declared minimum | Only the recorded kernel, C library and compiler |
| x86_64 GNU/Linux | Built, linted and run under QEMU user-mode emulation | Rust 1.85.0 | Uses the kernel of the host. No x86-64 kernel is qualified. |

The reader checkpoints of the book use the owner as a labelled stand-in for a
developer new to Ravel. These reads can show the owner's findings, for
example errors in the book. They cannot show whether a new developer can use
the book. You can build and examine the local book. Nobody can check how GitHub shows the
book until the project pushes the branch. The book on GitHub Pages stays
unchecked until the first release that contains the book.
