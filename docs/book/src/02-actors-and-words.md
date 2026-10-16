# The actors and their words

A request crosses three decisions: what work is wanted, which program will do
it, and whether the resulting [session](#caller-and-owner) completed successfully. Ravel assigns
those decisions to different participants. Knowing who makes each decision
lets you change a routing rule without changing the [caller](#caller-and-owner), and interpret a
session's outcome without mistaking a policy response for a completed job.

## Caller and owner

The **[caller](#caller-and-owner)** is the person or program invoking Ravel
and interpreting its returned result. It supplies the prompt, including the
ending instructions, and decides what to do afterwards. A **[session](#caller-and-owner)**
is one `ravel start-session` invocation, from routing the request to settling
the selected program's result or failure. A Ravel session is not a POSIX
session created by `setsid`, or a login session.

The caller supplies a **[kind](#caller-and-owner)**, an open token naming the
sort of session it wants. Ravel checks that the token is valid input; only the
owner's routing code gives it meaning. A caller might ask for `review`, but
Ravel has no registry of task kinds and does not know what reviewing entails.

The **[owner](#caller-and-owner)** is the person who controls Ravel's
configuration for the account. Their **[owner policy](#caller-and-owner)** is
executable TypeScript configuration that selects a program and its arguments
or declines the request. The caller and owner can be the same person, but
the roles remain distinct: choosing the work does not define the routing
rules. Ravel uses the account's `~/.config/ravel/policy.ts` by default;
`--config` explicitly names another entry. It does not search the working
repository for a policy.

A selection includes a program, arguments and descriptive labels such as
provider, model and effort. Those labels are owner text, not built-in model
integrations. The policy selects the actual command needed to invoke the
chosen program. Ravel runs that command without shell expansion or word
splitting; a shell is involved only if the policy selects one.

## Policy evaluation and supervision

The **[native front](#policy-evaluation-and-supervision)** is the Rust
`ravel` executable. During `start-session` it retains the caller's environment,
terminal and result output, evaluates the request, and supervises the selected
program. Its **[runner](#policy-evaluation-and-supervision)** is the Rust
library code that launches and supervises the job. The runner executes inside
the front; it is not another service running alongside it.

The **[policy worker](#policy-evaluation-and-supervision)** is a separate
process that evaluates the owner policy for the front. Its executable embeds
the Bun runtime so it can run TypeScript and imports without requiring a
separately installed runtime. “Worker” here means a process, not a Bun
`Worker` thread that policy code might itself create.

This separation lets policy run ordinary TypeScript while the native front
keeps supervision authority. Ravel gives the worker a bounded request and
a restricted environment, with no
[session-result context](#three-different-kinds-of-information). The
front checks the worker's response and requires confirmed worker termination
and reap before launching the selected program. Required worker cleanup must
succeed too. Receiving a valid selection alone does not authorise launch.
Only the native front supervises the managed job; policy evaluation is finished
before that job begins.

A **[refusal](#policy-evaluation-and-supervision)** is a decision not to
launch, made by owner policy or by Ravel when it cannot accept that policy's
behaviour, data or admission. Ravel does not substitute another route. A
failure of the worker or supervision, and a selection timeout, are separate
outcomes. A caller needs to distinguish “this request was refused” from “the
mechanism for evaluating or running it failed”.

## The managed process and its result

The **[managed process](#the-managed-process-and-its-result)** is the direct
child selected by owner policy and supervised by the native front. Its process
group defines the job-cleanup scope, including descendants that remain in
that group. If the selected program is an agent working on the prompt, the
book calls that program a **[harness](#the-managed-process-and-its-result)**.
That term describes an agent program, not a test harness; Ravel supervises it
through the same boundary as any other selected program.

The **[opaque result](#the-managed-process-and-its-result)** is the single
UTF-8 argument published with `ravel end-session`. Ravel returns it unchanged
after successful supervision and cleanup. The value is neither the managed
process's exit status nor the files or other work the program produced. Only
the caller gives the value meaning. An explicitly empty result is a publication;
exiting without publishing anything is not.

The **[result channel](#the-managed-process-and-its-result)** is the private
temporary directory and completion file through which `end-session` publishes
that value. The first complete publication wins; a later `end-session` does
not replace it. The channel carries a request to finish, not evidence that
the job has already been cleaned up. The native front combines publication
with confirmed reap, group cleanup and the other required cleanup before
returning success. Cancellation can suppress an already published value.

The runner's underlying completion channel only reports that a file appeared.
Ravel's result handling checks and reads the contents. Neither the file's
appearance nor the policy worker's selection is itself a successful session.

The participants' responsibilities can be compared directly:

| Participant | Decides or performs | Authority Ravel gives it |
|---|---|---|
| Caller | Supplies the task and ending instructions; interprets the result | Invocation and the decision about subsequent work |
| Owner | Writes routing code and settings | Configuration of which commands requests select |
| Policy worker | Evaluates that code and sends its response | Bounded request data and permitted environment; no session-result context |
| Native front, using its runner | Admits, launches, supervises and settles the session | Process supervision, terminal handling and the final result write |
| Managed process | Performs the selected work and publishes a result | Its own session-result context, inherited by its children |

## Three different kinds of information

The word “context” can hide three distinct inputs. The book names each one
so that request data cannot be mistaken for permission to end a session.

The **[session-result context](#three-different-kinds-of-information)** is
the inherited `RAVEL_EXIT_FILE` path identifying the enclosing session's result
channel. `end-session` uses that path; it reads no policy or settings and
accepts no target argument. Missing, empty or stale session-result context
causes it to fail. There is no persistent session lookup to fall back on.

In a nested launch, the outer managed process invokes a new `start-session`
as a caller. Ravel supplies a fresh path to the inner managed process, replacing
the outer path only in that child's environment. The outer managed process
keeps its own session-result context. Ordinary inheritance therefore directs
each completion command to its enclosing session rather than to a session
selected by an ID.

The **[policy context](#three-different-kinds-of-information)** is version-1
JSON data delivered to the policy's `select` function. It comes from the caller
or from the policy's optional context loader. It can describe the work and
the sources used to inform a choice. Reads through Ravel's provided host
operations are measured, recording what bytes were read and from where;
direct reads by policy code are possible but are not those measured reads.
Policy context is data for selection, not session-result authority or proof
that the data is trustworthy.

The caller's working directory is the request's `cwd`. It tells the policy
where the caller is working; it is neither of the two forms of context above.
The worker runs in its own execution environment, while the selected program
runs in the caller's working directory. A policy author uses the request's
`cwd` when the caller's location matters, rather than assuming the worker's
current directory is the same.

## Trust and confinement

Owner policy is trusted executable code. The worker's separation and restricted
inputs keep it out of the normal supervision and result-publication paths;
they do not make arbitrary owner code safe. Policy can access files or the
network and can start unrelated processes. Its detached children are outside
the worker cleanup guarantee.

**[Confinement](#trust-and-confinement)** is the optional restriction of the
managed process's filesystem access and environment. It is separate from
owner policy: selecting a command is routing, and restricting the command's
access is confinement. `--confine` applies to the managed process, not to the
policy worker, and Ravel has no unconfined fallback when confinement cannot
be provided.

Inherited session-result context separates normal nested use. It does not
authenticate code running as the same user. Such code can tamper with files
or launch unrelated processes. Likewise, descendants that leave the managed
process group are outside its cleanup, and an uncatchable death of the native
front can orphan the managed job. These limits matter when deciding what
guarantees a caller can rely on: a successful Ravel result confirms the defined
supervision and cleanup boundary, not that arbitrary same-user code has been
contained or that the caller's task was done correctly.

Chapter 3, “A complete session, end to end”, puts these roles together with a
small policy and a fake agent. The fake makes the routing, publication and
cleanup visible without introducing an agent account or a model service.
