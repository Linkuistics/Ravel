# Ravel language

Ravel runs an LLM session in an agent harness as a supervised part of another
program. It routes a caller's request through the owner's configuration and
supervises the selected process. Then it returns the result that the session
sends. These terms separate configuration authority, session authority and
the meaning that a caller gives a result.

## Language

**Caller**: The person or program that invokes Ravel and interprets the opaque
result that Ravel returns. An example is an orchestrator that runs **Harness**
sessions as steps of its own work. The caller owns its task conventions and
continuation decisions.

**Owner**: The person who controls Ravel's **Configuration** for an account.
The owner and the caller can be the same person. Their configuration and task
responsibilities stay distinct.

**Session**: One `start-session` invocation. Ravel routes a **Caller**'s
request through the **Configuration script** to one **Managed process**. Ravel
supervises that process until it reports the **Opaque result** or a failure,
after the required cleanup attempts.
_Avoid_: run, job, POSIX session

**Kind**: The **Caller**'s open token that names the sort of **Session** that
it requests. Only the **Configuration script** gives it meaning.
_Avoid_: task kind, type

**Launcher CLI**: A one-session, process-lifetime consumer that supervises a
**Managed process**, reports its **Opaque result** or failure after required
cleanup attempts, then exits.

**Persistent runner**: A separate component whose host lifetime continues after
a managed launch finishes. Its contract for a host that continues is distinct
from the **Launcher CLI**'s one-session lifetime.

**Configuration**: The **Owner**'s **Configuration script** and **Settings**
together.

**Configuration script**: The owner's executable Rhai script that has the
authority to select a command or refuse the caller's request. Its default file
is `config.rhai`. Its author controls how it routes requests.
_Avoid_: owner policy, policy (unqualified)

**Settings**: The **Owner**'s values in `settings.json`, such as bounds, the
harnesses and environment grants. The **Configuration script** cannot change
them.
_Avoid_: owner settings

**Selection context**: The JSON data that the **Configuration script**'s
`select` receives. It is the **Caller**'s context document, or the context that
the script's own loader assembled. Ravel measures each source that the loader
reads through a **Host capability**. It is data, never authority.
_Avoid_: policy context, context (unqualified), session context

**Refusal**: A decision not to launch. The **Configuration script** itself makes
it, or Ravel makes it when it cannot accept that script's file, behaviour or
data. It is not a failure of the **Rhai worker** or of supervision.
_Avoid_: rejection, error

**Rhai worker**: The separate process, a second copy of the `ravel`
executable, that evaluates the **Configuration script** for the **Ravel CLI**.
It only computes, and it asks the Ravel CLI for each **Host capability**. It is
not a managed process and receives no session-result context.
_Avoid_: policy worker

**Host capability**: An operation that the **Configuration script** asks the
**Ravel CLI** to perform during evaluation. Examples are a measured read and a
**Harness run**. The Ravel CLI performs it and returns data to the script.
_Avoid_: host binding, SDK function

**LLM call**: A **Harness run** in which the **Configuration script** asks a model one
question and reads the answer as data. The **Harness** signs in with the
**Owner**'s own sign-in, and Ravel holds no model credential.
_Avoid_: API call, completion request, LLM endpoint

**Provider**: The owner's label for the origin of the model that a selection
runs, such as `anthropic`. Ravel reports it unchanged and never connects to it.
_Avoid_: vendor

**Evaluation budget**: A limit that the **Owner** sets for one **Session** on
**Host capabilities**, such as the number of **Harness runs**. The
**Configuration script** cannot raise it or catch its exhaustion.
_Avoid_: quota

**Ravel CLI**: The Rust `ravel` command-line program. During
`start-session` it checks the configuration script, supervises the managed process and
chooses the final result or failure to report.
_Avoid_: front, native front

**Runner**: The Rust library code in the Ravel CLI that launches and supervises
the managed job. It is not a separate service process. It is not the
undelivered **Persistent runner**.

**Managed process**: The direct child that the configuration script selects and that the
Ravel CLI supervises as a job. Its process group is the scope of job cleanup.

**Harness**: The agent program, such as Claude Code or Codex, in which an LLM
session works on the **Session**'s prompt. The **Configuration script** selects it. An
interactive harness does not end itself. Thus, the LLM sends the **Opaque
result** with `end-session`. Ravel treats the harness as any other **Managed
process**.
_Avoid_: agent runtime, test harness

**Harness run**: One execution of an owner-configured **Harness** that the
**Configuration script** requests during evaluation. The **Runner** supervises it as a
job with its own **Result channel**. It is not a **Session** and selects
nothing.
_Avoid_: nested session, sub-session

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
checks and reads the result.

**Confinement**: Optional restrictions on the filesystem access and environment
of a **Managed process** or a **Harness run**. Confinement does not apply to
the **Configuration script** or its **Rhai worker**.

## Flagged ambiguities

- **"Session"** alone means a Ravel **Session**. It never means a POSIX session
  (`setsid`), a login session or the LLM session in a **Harness**. Say "LLM
  session" for that. An unconfined **Managed process** stays in the
  **Caller**'s POSIX session, in a process group of its own. A confined launch
  creates a new POSIX session.
- **"Context"** alone is ambiguous. Say **Session-result context** for the
  inherited `RAVEL_EXIT_FILE` path, **Selection context** for the data that
  `select` receives, and working directory for the request's `cwd`.
- **"Kind"** is the **Caller**'s token. Grove passes its own task kind as the
  **Kind**, but Ravel neither knows nor checks Grove's kinds.
- **"Channel"**: the result channel is Ravel's private directory and completion
  file, through which `end-session` sends the result. It uses the runner's
  completion channel, which only signals that a file appeared.
- **"Worker"** means the **Rhai worker** process, never a thread.
- **"Provider"** means only the owner's model-origin label in a selection.
- **"LLM call"** means a **Harness run** that asks a model a question. Ravel
  sends no request to a model service itself.
- **`config_*` diagnostic codes** name the **Configuration script**.
  A restriction on a **Managed process** is confinement.
- **"Result"** means the **Opaque result**, never the **Managed process**'s
  exit status or the work a **Harness** did.

## Example dialogue

Caller author: “If the managed process sends `continue`, will Ravel run
the next task?”

Ravel maintainer: “Ravel completes the cleanup of the managed job and returns
those bytes. The caller decides whether they mean another task. The
configuration script chooses the command for each request. The Rhai worker
never receives the session-result context.”

Owner: “Can my configuration script ask a model which harness to use for a
task?”

Ravel maintainer: “Yes. The script makes an LLM call: a harness run of Claude
Code or Codex, which your settings name. The harness signs in with your own
sign-in, and the script reads the answer as data. Ravel never holds the
credential. The provider in the selection stays your own label.”
