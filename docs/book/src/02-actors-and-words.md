# The actors and their words

Each request involves three decisions: what work is wanted, which program will
do it, and whether the resulting [session](#caller-and-owner) completed
successfully. Ravel assigns those decisions to different participants. Knowing
who makes each decision lets you change a routing rule without changing the
[caller](#caller-and-owner), and interpret a session's outcome without
mistaking a policy response for a completed session.

## Caller and owner

The **caller** is the person or program invoking Ravel and interpreting its
returned result. It supplies the prompt, including the ending instructions,
and decides what to do afterwards. A **session** is one `ravel start-session`
invocation, from routing the request to settling the selected program's result
or failure. A Ravel session is neither a POSIX session created by `setsid` nor
a login session.

The caller supplies a **kind**, an open token naming the sort of session it
wants. Ravel checks that the token is not empty and gives it no meaning; the
[owner](#caller-and-owner)'s routing code interprets it. A caller might ask for
`review`, but Ravel has no registry of task kinds and does not know what
reviewing entails.

The **owner** is the person who controls Ravel's configuration for the
account. Their **owner policy** is executable TypeScript configuration that
selects a program and its arguments or refuses the request. The caller and
owner can be the same person, but the roles remain distinct: choosing the work
does not define the routing rules. Ravel uses the account's
`~/.config/ravel/policy.ts` by default; `--config PATH` names another policy
file instead, resolved against the caller's working directory. Ravel does not
search the working repository for a policy.

A selection includes a program, arguments and descriptive labels such as
provider, model and effort. Those labels are owner text, not built-in model
integrations. The policy selects the actual command needed to invoke the
chosen program. Ravel runs that command without shell expansion or word
splitting; a shell is involved only if the policy selects one.

The [system-context figure in chapter 1](01-what-ravel-is-for.md#from-a-request-to-a-result)
places these roles around Ravel: the caller chooses the work and the owner
controls routing.

## Policy evaluation and supervision

The **native front** is the Rust `ravel` executable. During `start-session` it
keeps the caller's environment, terminal and standard output, evaluates the
request, and supervises the selected program. Its **runner** is the Rust
library code that launches and supervises the managed process. The runner
executes inside the native front; it is not another service running alongside
it.

The **policy worker** is a separate process that evaluates the owner policy
for the native front. Its executable embeds the Bun runtime, so it can run
TypeScript policy and the modules that policy imports without a separately
installed runtime. “Worker” here means a process, not a Bun
`Worker` thread that policy code might itself create.

This separation lets policy run ordinary TypeScript while the native front
keeps supervision authority. Ravel gives the worker a size-limited request and
a restricted environment, with no
[session-result context](#three-different-kinds-of-information). The native
front checks the worker's response, then launches the selected program only
after confirming that the worker has ended and been reaped and that the
worker's private directory has been removed. Receiving a valid selection alone
does not authorise launch. Only the native front supervises the
[managed process](#the-managed-process-and-its-result); policy evaluation is
finished before that process starts.

A **refusal** is a decision not to launch, made by owner policy, or by Ravel
when it cannot accept that policy's behaviour, data or admission. Ravel does
not substitute another route. A failure of the worker or supervision, and a
selection timeout, are separate outcomes. A caller needs to distinguish “this request was refused” from “the
mechanism for evaluating or running it failed”.

## The managed process and its result

The **managed process** is the direct child selected by owner policy and
supervised by the native front. Its process group defines the job-cleanup
scope, including descendants that remain in that group. If the selected
program is an agent working on the prompt, the book calls that program a
**harness**. That term describes an agent program, not a test harness; Ravel
supervises it through the same boundary as any other selected program.

The **opaque result** is the single UTF-8 argument published with
`ravel end-session`. Ravel returns it unchanged after successful supervision
and cleanup. The value is neither the managed
process's exit status nor the files or other work the program produced. Only
the caller gives the value meaning. An explicitly empty result is a publication;
exiting without publishing anything is not.

The **result channel** is the private temporary directory and completion file
through which `end-session` publishes that value. The first complete
publication wins; a later `end-session` does not replace it. The channel
carries a request to finish, not evidence that the job has already been
cleaned up. Before returning success, the native front also requires a
confirmed reap, a process group that no longer exists, and removal of the
session's temporary files. Cancellation suppresses the value even if it was
already published.

The runner's underlying completion channel only reports that a file appeared.
Ravel's result handling checks and reads the contents. Neither the file's
appearance nor the policy worker's selection is itself a successful session.

The figure shows why a policy response cannot complete a session: it reaches
the native front through a separate path from the managed process's opaque
result. It uses C4's container level, which shows the processes and the store
inside the single Ravel box of chapter 1. Here, a container means a running
unit or a store, not an operating-system container. Boxes name processes or
the external caller; the cylinder represents temporary files, not a database.
The enclosing outline groups Ravel's software, not a sandbox. Blue marks
Ravel's processes, grey external participants and gold the result channel;
labels identify them without colour. Arrowheads follow the labelled actions
or data, with green tracing the opaque result. Steps 1 and 2 show ordering:
the worker is cleaned up before the managed process starts. The figure shows
every participant across a session; it does not show which processes run at
the same time.

![The native front evaluates the request through a separate policy worker, which evaluates trusted owner policy without session-result context and sends a selection or refusal through a separate socket. Before launching the managed process, the native front confirms that the worker ended and was reaped and its private directory was removed. The runner executes inside the native front and supervises the managed process. That process receives session-result context and uses end-session to publish into the temporary result channel. The native front validates and reads the publication, then writes the opaque result to the caller only after successful cleanup.](figures/process-authority.svg)

[Open the process-and-authority figure at full size.](figures/process-authority.svg)

The responsibility table locates final supervision and result settlement in
the native front, while the caller and owner retain their separate decisions.

| Participant | Decides or performs | Authority Ravel gives it |
|---|---|---|
| Caller | Supplies the task and ending instructions; interprets the result | Invocation and the decision about subsequent work |
| Owner | Writes routing code and settings | Configuration of which commands requests select |
| Policy worker | Evaluates the owner policy and sends its response | Bounded request data and permitted environment; no session-result context |
| Native front, using its runner | Admits, launches, supervises and settles the session | Process supervision, terminal handling and the final result write |
| Managed process | Performs the selected work and publishes a result | Its own session-result context, inherited by its children |

## Three different kinds of information

The word “context” can mean three distinct inputs. The book names each one so
that request data cannot be mistaken for permission to end a session.

The **session-result context** is the inherited `RAVEL_EXIT_FILE` path
identifying the enclosing session's result channel. `end-session` uses that
path; it reads no policy or settings and accepts no target argument. It fails
if the path is missing or empty, or stale: left over from a session that has
already ended. There is no persistent session lookup to fall back on.

In a nested launch, the outer managed process invokes a new `start-session`
as a caller. Ravel supplies a fresh path to the inner managed process, replacing
the outer path only in that child's environment. The outer managed process
keeps its own session-result context. Ordinary inheritance therefore directs
each completion command to its enclosing session rather than to a session
selected by an ID.

The mapping shows why a completion command that inherits the inner path ends
the inner session, while the outer process keeps its own publication path.

| Process invoking `end-session` | Inherited session-result context | Publication goes to |
|---|---|---|
| Outer managed process and its ordinary children | Outer session's path | Outer session's result channel |
| Inner managed process and its ordinary children | Fresh inner session's path | Inner session's result channel |

The **policy context** is version-1 JSON data delivered to the policy's
`select` function. It comes from the caller or from the policy's optional
context loader. It can describe the work and the sources used to inform a
choice. Reads through Ravel's provided host operations are measured: Ravel
records each source's canonical name, byte count and the SHA-256 digest of the
bytes actually read. Direct reads by policy code are possible but are not
measured. Policy context is data for selection, not session-result authority
or proof that the data is trustworthy.

The caller's working directory is the request's `cwd`. It tells the policy
where the caller is working; it is neither of the two forms of context above.
The selected program starts in the caller's working directory. A policy author
uses the request's `cwd` when the caller's location matters, rather than
assuming the worker's current directory is the same.

The native front creates the worker's private directory, which is empty, and
starts the worker there. The worker changes to `/` before it registers its
embedded modules or imports owner policy, and policy code can change the
worker's working directory again. During worker cleanup, before it launches
the selected program, the native front removes the worker's private directory.

Use this comparison to keep selection data and a working location separate
from the inherited path that directs result publication.

| Information | Form | Purpose |
|---|---|---|
| Session-result context | Inherited `RAVEL_EXIT_FILE` path | Directs `end-session` to the enclosing session's result channel |
| Policy context | Version-1 JSON data | Informs the owner policy's selection; supplies no session-result authority |
| Caller's working directory | Request's `cwd` | Names where the caller is working; the selected program starts there |

## Trust and confinement

Owner policy is trusted executable code. The worker's separation and restricted
inputs keep it out of the normal supervision and result-publication paths;
they do not make arbitrary owner code safe. Policy can access files or the
network and can start unrelated processes. Its detached children are outside
the worker cleanup guarantee.

**Confinement** is the optional restriction of the managed process's
filesystem access and environment. It is separate from owner policy: selecting
a command is routing, and restricting the command's access is confinement.
`--confine` applies to the managed process, not to the policy worker. When
confinement is requested but cannot be provided, Ravel does not launch the
process unconfined instead.

Inherited session-result context keeps nested sessions apart while each
process uses the path it inherited. It does not authenticate code running as
the same user. Such code can tamper with files or launch unrelated processes.
Likewise, descendants that move out of the managed process's process group are
outside its cleanup, and if the native front is
killed with SIGKILL, which it cannot catch, the managed process can be left
running without a supervisor. These limits matter when deciding what
guarantees a caller can rely on: a successful Ravel result confirms the defined
supervision and cleanup boundary, not that arbitrary same-user code has been
contained or that the caller's task was done correctly.

Chapter 3, “A complete session, end to end”, puts these roles together with a
small policy and a fake agent. The fake makes the routing, publication and
cleanup visible without introducing an agent account or a model service.
