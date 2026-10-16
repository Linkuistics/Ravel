# What Ravel is for

Ravel lets a tool ask for work to be done without choosing the program that
will do it. The tool supplies a request; the person who configures Ravel
chooses how requests are routed. Ravel starts the selected program, supervises
it, and returns its published result after cleanup succeeds.

Consider a tool that asks for a code review. The tool knows the files to review
and what to do with the answer. The person running it wants to choose the
reviewing program for their account. Ravel separates those decisions: the
tool asks for a review, the person's configuration selects a command, and
Ravel runs that command, supervises it until it has ended and been cleaned up,
and returns its result. The same tool can use different programs under
different owners' configurations.

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
An AI coding agent is one possible program; the book calls
such an agent program a [harness](02-actors-and-words.md#the-managed-process-and-its-result).
Ravel also works with ordinary programs that use its completion command.

The caller's prompt supplies the ending instructions, including the result
values it understands. From within the session, `ravel end-session RESULT`
publishes one [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
and asks Ravel to end the session. “Opaque” means that Ravel carries
the value without interpreting it. If the caller chooses `continue` as a
value, Ravel returns those bytes; the caller decides whether to start more
work. `end-session` finds the enclosing session through its inherited
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).
It takes no session ID or target argument.

Publication is the start of completion, not proof that completion succeeded.
Before returning a successful result, Ravel confirms that the managed process
has ended and been reaped, that its process group is gone, and that the
session's temporary files have been removed. Reaping is the parent collecting
a finished child's exit status. The process group is the operating system's
grouping of the managed process with its descendants, and Ravel cleans up the
whole group. Chapter 3, “A complete session, end to end”, explains those mechanics
as a session reaches them.

A successful `start-session` writes the exact result bytes to standard output,
without adding a newline. The managed process's standard output is redirected
to standard error, where its output and Ravel's diagnostics can be kept apart
from the result stream. The caller checks whether `start-session` succeeded
before interpreting that stream. A failure while writing the final result can
leave only a prefix, so bytes alone do not establish success.

Several outcomes carry no successful result. A
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision) means no
program is launched. A managed process that exits without publishing a result
has not completed the protocol, even if its exit status is zero. Cancellation
of the session suppresses a result even if it was already published. A
supervision or cleanup failure also prevents success. Conversely, a published
result can succeed after cleanup even if the managed process later exits
nonzero. The result is the value the caller's ending instructions asked for,
the exit status reports how the process ended, and the work is whatever the
program changed; none of the three stands in for another.

## Decide whether Ravel fits

Ravel fits when your tool needs an owner-selected program, one supervised
session and an explicit result. The tool can leave routing to the account's
configuration and use the same completion protocol for each selected program.
It still owns the task and the decisions made after the session ends.

| If you need… | Ravel's part | What remains outside Ravel |
|---|---|---|
| Different owners to choose different programs for the same request | Evaluate each owner's policy and run its selected command | The caller's task meaning and the owner's routing choices |
| A result after a program and its job have been cleaned up | Supervise the launch and return the explicit result after successful cleanup | The caller's interpretation and any further work |
| Task queues, schedules, retries or durable history | Execute a session when invoked | A caller or service that stores and schedules work |
| Model access, credentials, usage metering or model selection rules | Run the command chosen by policy | The selected program's model integration and the owner's selection rules |
| A host that continues doing arbitrary work after a launch | The CLI supervises one session and exits | A separate continuing-host contract, which is not delivered here |

Ravel is a **launcher CLI**: one invocation supervises one managed launch,
settles its result or failure, then exits. A **persistent runner** would be a
host that keeps running after a launch settles. Ravel does not provide one,
and its one-session lifetime does not supply the contract such a host needs.

Ravel has no persistent session store or history lookup. Its
[result channel](02-actors-and-words.md#the-managed-process-and-its-result)
uses temporary files for one session; it is not a record of past work. If a
caller needs a lasting record, that caller saves it. Owner policy is executable
code and can itself use files or network services; the absence of a Ravel store
does not restrict what that trusted code can do.

Grove, a task-tree driver, is one illustrative caller. It supplies its own task
kinds and interprets its own ending values. Those conventions belong to Grove.
Another caller can use different kinds and results without a Ravel adapter.

The supervision guarantee also has limits. A descendant that moves itself out
of the managed process group is not reached by that group's cleanup, and the
session can succeed while it still runs. Detached children started by policy
are outside the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)'s
cleanup guarantee. If Ravel is killed abruptly, for example with SIGKILL,
which it cannot catch, the managed process or temporary files can be left
behind. Code running as the same user can tamper with files; Ravel's session
separation does not authenticate that code.
[Chapter 2](02-actors-and-words.md#trust-and-confinement) explains why the
authority boundaries are useful within those limits.

## Choose a reading route

The book assumes everyday shell, Git and JSON use, the ability to read
TypeScript and Rust, and general operating-system knowledge. It teaches
Ravel's domain and the job-control details its behaviour depends on.

- To call Ravel from your own tool, read chapters 1–3, then chapter 5,
  “Running sessions”. Chapter 6, “Reference”, is the lookup chapter.
- To write owner policy, read chapters 1–3, then chapter 4, “Writing an owner
  policy”, with chapter 6 for lookup.
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
session uses a tiny policy and a fake agent, so learning the protocol needs
no model service or personal agent account. Installing and configuring real
agents is outside the book's scope.

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

The book's reader checkpoints use the owner as a labelled stand-in for a
developer new to Ravel. Those reads can establish the owner's results and
find inaccuracies; whether a new developer can use the book remains unknown.
The local book can be built and inspected, but GitHub's rendering remains
unchecked until the branch is pushed, and the published site until the first
release carrying the book.
