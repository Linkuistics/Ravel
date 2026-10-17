# Ravel language

Ravel runs an LLM session in an agent harness as a supervised part of another
program. It routes a caller's request through owner configuration and
supervises the selected process. Then it returns the result that the session
sends. These terms separate configuration authority, session authority and
the meaning that a caller gives a result.

## Language

**Caller**: The person or program that invokes Ravel and interprets the opaque
result that Ravel returns. An example is an orchestrator that runs **Harness**
sessions as steps of its own work. The caller owns its task conventions and
continuation decisions.

**Owner**: The person who controls Ravel's policy and settings for an account.
The owner and the caller can be the same person. Their configuration and task
responsibilities stay distinct.

**Session**: One `start-session` invocation. Ravel routes a **Caller**'s
request through the **Owner policy** to one **Managed process**. Ravel
supervises that process until it reports the **Opaque result** or a failure,
after the required cleanup attempts.
_Avoid_: run, job, POSIX session

**Kind**: The **Caller**'s open token that names the sort of **Session** that
it requests. Only the **Owner policy** gives it meaning.
_Avoid_: task kind, type

**Launcher CLI**: A one-session, process-lifetime consumer that supervises a
**Managed process**, reports its **Opaque result** or failure after required
cleanup attempts, then exits.

**Persistent runner**: A separate component whose host lifetime continues after
a managed launch finishes. Its contract for a host that continues is distinct
from the **Launcher CLI**'s one-session lifetime.

**Owner policy**: The executable TypeScript configuration that has the authority
to select a command or refuse the caller's request. Its author controls how it routes
requests.

**Policy context**: The version-1 JSON data that the **Owner policy**'s
selection receives. It is the **Caller**'s context document, or the context
that the policy's own loader assembled. Ravel measures each source that the
loader reads through the host. It is data, never authority.
_Avoid_: context (unqualified), session context

**Refusal**: A decision not to launch. The **Owner policy** itself makes it, or
Ravel makes it when it cannot accept that policy's file, behaviour or data. It
is not a failure of the **Policy worker** or of supervision.
_Avoid_: rejection, error

**Policy worker**: The separate process that evaluates the owner policy for the
Ravel CLI. It is not a managed process and receives no session-result context.

**Ravel CLI**: The Rust `ravel` command-line program. During
`start-session` it checks the policy, supervises the managed process and
chooses the final result or failure to report.
_Avoid_: front, native front

**Runner**: The Rust library code in the Ravel CLI that launches and supervises
the managed job. It is not a separate service process. It is not the
undelivered **Persistent runner**.

**Managed process**: The direct child that the owner policy selects and that the
Ravel CLI supervises as a job. Its process group is the scope of job cleanup.

**Harness**: The agent program, such as Claude Code or Codex, in which an LLM
session works on the **Session**'s prompt. The **Owner policy** selects it. An
interactive harness does not end itself. Thus, the LLM sends the **Opaque
result** with `end-session`. Ravel treats the harness as any other **Managed
process**.
_Avoid_: agent runtime, test harness

**Session-result context**: The inherited `RAVEL_EXIT_FILE` path that
identifies the ephemeral completion channel of the enclosing session. A nested
session replaces it only for that session's managed process.

**Opaque result**: The single UTF-8 argument that a program in the session
sends with `end-session`. After successful supervision and cleanup, Ravel
returns it unchanged. Only the caller assigns it meaning. An explicitly empty
result is not the same as no result.
_Avoid_: publish, publication, submit (`end-session` sends the result, and
Ravel keeps the first result that it receives)

**Result channel**: Ravel's private temporary directory and completion file
that carry the **Opaque result** for one launch. The result channel uses the
runner's completion channel, which reports only that the file appeared. Ravel
validates and reads the result.

**Confinement**: Optional restrictions on the managed process's filesystem access
and environment. Confinement does not apply to the **Owner policy** or its
**Policy worker**.

## Flagged ambiguities

- **"Session"** alone means a Ravel **Session**. It never means a POSIX session
  (`setsid`), a login session or the LLM session in a **Harness**. Say "LLM
  session" for that. An unconfined **Managed process** stays in the
  **Caller**'s POSIX session, in a process group of its own. A confined launch
  starts a new POSIX session.
- **"Context"** alone is ambiguous. Say **Session-result context** for the
  inherited `RAVEL_EXIT_FILE` path, **Policy context** for the data selection
  receives, and working directory for the request's `cwd`.
- **"Kind"** is the **Caller**'s token. Grove passes its own task kind as the
  **Kind**, but Ravel neither knows nor checks Grove's kinds.
- **"Channel"**: the result channel is Ravel's private directory and completion
  file, through which `end-session` sends the result. It uses the runner's
  completion channel, which only signals that a file appeared.
- **"Worker"** means the **Policy worker** process, not a Bun `Worker` thread,
  which owner code may itself start.
- **"Policy"** means the **Owner policy**, never a security or sandbox policy.
  A restriction on a **Managed process** is confinement.
- **"Result"** means the **Opaque result**, never the **Managed process**'s
  exit status or the work a **Harness** did.

## Example dialogue

Caller author: “If the managed process sends `continue`, will Ravel start
the next task?”

Ravel maintainer: “Ravel completes the cleanup of the managed job and returns
those bytes. The caller decides whether they mean another task. The owner
policy chooses the command for each request. The policy worker never receives
the session-result context.”
