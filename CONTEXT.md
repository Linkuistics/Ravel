# Ravel language

Ravel runs an LLM session in an agent harness as a supervised part of another
program: it routes a caller's request through owner configuration, supervises
the selected process and returns the result the session publishes. These terms
distinguish configuration authority, session authority and the meaning a
caller gives a result.

## Language

**Caller**: The person or program invoking Ravel and interpreting its returned
opaque result, such as an orchestrator that runs **Harness** sessions as steps
of its own work. Its task conventions and continuation decisions are
caller-owned.

**Owner**: The person controlling Ravel's policy and settings for an account.
The owner and caller can be the same person; their configuration and task
responsibilities remain distinct.

**Session**: One `start-session` invocation: a **Caller**'s request routed
through the **Owner policy** to one **Managed process**, supervised until its
**Opaque result** or failure is reported after required cleanup attempts.
_Avoid_: run, job, POSIX session

**Kind**: The **Caller**'s open token naming the sort of **Session** it asks
for. Only the **Owner policy** gives it meaning.
_Avoid_: task kind, type

**Launcher CLI**: A one-session, process-lifetime consumer that supervises a
**Managed process**, reports its **Opaque result** or failure after required cleanup attempts,
then exits.

**Persistent runner**: A separate component whose host lifetime continues after
a managed launch finishes. Its continuing-host contract is distinct from the
**Launcher CLI**'s one-session lifetime.

**Owner policy**: The executable TypeScript configuration authorised to select
a command or refuse the caller's request. Its author controls routing choices.

**Policy context**: The version-1 JSON data the **Owner policy**'s selection
receives: the **Caller**'s context document, or the context the policy's own
loader assembled, with every source read through the host measured. It is
data, never authority.
_Avoid_: context (unqualified), session context

**Refusal**: A decision not to launch, made by the **Owner policy** itself or by
Ravel when it cannot accept that policy's file, behaviour or data. It
is not a failure of the **Policy worker** or of supervision.
_Avoid_: rejection, error

**Policy worker**: The separate process evaluating the owner policy for the
Ravel CLI. It is not a managed process and receives no session-result context.

**Ravel CLI**: The Rust `ravel` command-line program. During
`start-session` it checks the policy, supervises the managed process and
chooses the final result or failure to report.
_Avoid_: front, native front

**Runner**: The Rust library code executing inside the Ravel CLI that launches
and supervises the managed job. It is not a separate running service or the
undelivered **Persistent runner**.

**Managed process**: The direct child selected by the owner policy and supervised
as a job by the Ravel CLI. Its process group is the scope of job cleanup.

**Harness**: The agent program, such as Claude Code or Codex, in which an LLM
session works on the **Session**'s prompt; the **Owner policy** selects it. An
interactive harness does not end itself, which is why the LLM publishes the
**Opaque result** with `end-session`; Ravel treats the harness as any other
**Managed process**.
_Avoid_: agent runtime, test harness

**Session-result context**: The inherited `RAVEL_EXIT_FILE` path identifying the
enclosing session's ephemeral completion channel. A nested session replaces it
only for that session's managed process.

**Opaque result**: The single UTF-8 argument published by `end-session` and
returned unchanged after successful supervision and cleanup. Only the caller
assigns it meaning; an explicitly empty result differs from no publication.

**Result channel**: Ravel's private temporary directory and completion file
carrying the **Opaque result** for one launch. The runner's underlying completion
channel reports file appearance; Ravel validates and reads the result.

**Confinement**: Optional restrictions on the managed process's filesystem access
and environment. Confinement does not apply to the **Owner policy** or its
**Policy worker**.

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
