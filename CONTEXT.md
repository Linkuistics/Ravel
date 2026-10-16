# Ravel language

Ravel routes a caller's request through owner configuration and supervises the
selected process. These terms distinguish configuration authority, session
authority and the meaning a caller gives a result.

## Language

**Caller**: The person or program invoking Ravel and interpreting its returned
opaque result. Its task conventions and continuation decisions are caller-owned.

**Session**: One `start-session` invocation: a **Caller**'s request routed
through the **Owner policy** to one **Managed process**, supervised until its
**Opaque result** or failure is settled.
_Avoid_: run, job, POSIX session

**Kind**: The **Caller**'s open token naming the sort of **Session** it asks
for. Only the **Owner policy** gives it meaning.
_Avoid_: task kind, type

**Launcher CLI**: A one-session, process-lifetime consumer that supervises a
**Managed process**, settles its **Opaque result** or failure, then exits.

**Persistent runner**: A separate component whose host lifetime continues after
a managed launch settles. Its continuing-host contract is distinct from the
**Launcher CLI**'s one-session lifetime.

**Owner policy**: The executable TypeScript configuration authorised to select
a command or refuse the caller's request. Its author controls routing choices.

**Policy context**: The version-1 JSON data the **Owner policy**'s selection
receives: the **Caller**'s context document, or the context the policy's own
loader assembled, with every source read through the host measured. It is
data, never authority.
_Avoid_: context (unqualified), session context

**Refusal**: A decision not to launch, made by the **Owner policy** itself or by
Ravel when that policy's behaviour, output or admission cannot be accepted. It
is not a failure of the **Policy worker** or of supervision.
_Avoid_: rejection, error

**Policy worker**: The separate process evaluating the owner policy for the
native front. It is not a managed process and receives no session-result context.

**Managed process**: The direct child selected by the owner policy and supervised
as a job by the native front. Its process group is the scope of job cleanup.

**Harness**: The agent program an **Owner policy** selects to work on the
**Session**'s prompt, such as an AI coding agent's command-line tool. Ravel
treats it as any other **Managed process**.
_Avoid_: agent runtime, test harness

**Session-result context**: The inherited `RAVEL_EXIT_FILE` path identifying the
enclosing session's ephemeral completion channel. A nested session replaces it
only for that session's managed process.

**Opaque result**: The single UTF-8 argument published by `end-session` and
returned unchanged after successful supervision and cleanup. Only the caller
assigns it meaning; an explicitly empty result differs from no publication.

## Flagged ambiguities

- **"Session"** alone means a Ravel **Session**, never a POSIX session
  (`setsid`) or a login session. An unconfined **Managed process** stays in the
  **Caller**'s POSIX session, in a process group of its own; a confined launch
  starts a new POSIX session.
- **"Context"** alone is ambiguous. Say **Session-result context** for the
  inherited `RAVEL_EXIT_FILE` path, **Policy context** for the data selection
  receives, and working directory for the request's `cwd`.
- **"Kind"** is the **Caller**'s token. Grove passes its own task kind as the
  **Kind**, but Ravel neither knows nor checks Grove's kinds.
- **"Channel"**: the result channel is Ravel's private directory and completion
  file, through which `end-session` publishes. It is built on the runner's
  completion channel, which only signals that a file appeared.
- **"Worker"** means the **Policy worker** process, not a Bun `Worker` thread,
  which owner code may itself start.
- **"Policy"** means the **Owner policy**, never a security or sandbox policy;
  restricting a **Managed process** is confinement.
- **"Result"** means the **Opaque result**, never the **Managed process**'s
  exit status or the work a **Harness** did.

## Example dialogue

Caller author: “If the managed process publishes `continue`, will Ravel start
the next task?”

Ravel maintainer: “Ravel cleans up the managed job and returns those bytes. The
caller decides whether they mean another task. The owner policy chooses the
command for each request; the policy worker never receives the session-result
context.”
