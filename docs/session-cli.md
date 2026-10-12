# The session CLI

Ravel's agreed extraction boundary is a **process manager and router**. This
checkout currently contains the local runner and private result transport
([the session channel](session-channel.md)); the front and compiled policy
worker are the next implementation increment.

The public CLI has two verbs:

| Verb | Responsibility |
|---|---|
| `ravel start-session` | Route the caller's request through the owner's policy, start the selected process, supervise its lifecycle and return its result. |
| `ravel end-session RESULT` | From inside the session, pass one opaque argument back to its enclosing `start-session` and request that the session end. |

`end-session` finds its enclosing session through inherited process context. It
takes no session ID or target path. Completion uses an ephemeral channel for that
session, isolated from nested sessions. Ravel terminates and reaps the managed
process before `start-session` returns the supplied argument. The result is
returned unchanged on stdout, distinguishable from session output and diagnostics.
Ordinary `--help` and `--version` flags remain available.

The caller supplies the prompt, including instructions to use `end-session` and
the result values it understands. Ravel passes the prompt unchanged. A caller may
choose values such as `continue` or `force-stop`; Ravel gives them no meaning.
Caller task trees, continuation decisions and durable provenance stay outside
this boundary.

Ravel has no persistent store, record/observation commands, historical lookup API,
importer or history migration. Existing dispatch history remains with its original
tool. The generic policy/SDK handles routing and applicable caller context without
database access. The owner configuration format and selection choices remain;
obsolete storage settings are removed during configuration copying.

Development checks use the existing real-worker/fake-agent and local process
seams. Test result round trips, nested/stale context, cancellation, reaping,
terminal recovery and confinement, then verify the independently installed pair.
Use focused checks during edits and the full relevant suite at handoff. Taskfile
commands use cargo nextest with its reporting visible and save formatting/lint
logs with concise summaries; inspect failure details
only when needed. No real model launches or new testing framework are required.
