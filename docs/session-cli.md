# The session CLI

Ravel's agreed extraction boundary is a **process manager and router**. This
checkout builds the command front, compiled policy worker, local runner and
[ephemeral result transport](session-channel.md) at 0.0.0. Installation and
distribution verification are the next increment.

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

## Inputs and exit behavior

`start-session` requires `--kind` and exactly one of `--prompt` / `--prompt-file`.
The owner's default is `~/.config/ravel/policy.ts`; `--config` names an explicit
entry, relative to the caller's cwd. Optional `--param NAME=VALUE`, `--context`,
`--task-file` and `--task-id` are caller data without Ravel task conventions.
The policy receives the prompt unchanged, including empty strings and newlines.
Use `ravel start-session --help` for bounds, environment grants and confinement.

The first `end-session` publication wins. A supplied result is success after
confirmed cleanup, even if the managed process subsequently exits nonzero.
A child exiting without a result is exit 1, including a natural exit 0. Missing
or stale enclosing context makes `end-session` exit 1. Cancellation by INT,
TERM or HUP returns that signal and no result. Cleanup/supervision failures
are reported on stderr and never yield a successful result. CLI errors exit 2,
policy refusals 3, worker failures 5, selection timeouts 124 and program failures
126/127. `--json` formats routing notices and refusals on stderr; stdout remains
the opaque result stream. Managed-process stderr remains its own stream.

The worker is paired by package version and source digest before any policy
is sent. Its generic embedded imports are `ravel/sdk` and
`ravel/examples/static`; `readChoice` reads `.ravel-choice`. Context-host reads
are measured, bounded regular-file reads. There is no run-lookup operation,
embedded caller adapter or persistent state directory. Obsolete `stateDir`
settings refuse as unknown keys, without opening that directory.

`task setup` installs a verified checkout-local Bun 1.4.2 and the frozen worker
build dependencies. `task build` builds the matching pair; `task typecheck`
checks both source and generated declarations. `task test:cli` uses actual
workers, temporary policies and fixture processes. `task check` rebuilds and
typechecks the pair, lints all targets and runs the full nextest suite.
