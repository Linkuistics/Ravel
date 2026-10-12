# Ravel supervises the session

start-session stays parent of its selected process until termination/reap and
process-group cleanup. end-session publishes one opaque value using inherited
context; Ravel gives that value no task-tree or continuation meaning. The caller
supplies ending instructions and interprets the returned value. There is no
persistent run record or observation protocol in Ravel.

An exec handoff was rejected because it leaves supervision and completion split
across the caller and selected executable. One owner of process termination lets
any caller use the same boundary. The price is a process for the session's
lifetime and nested job control when the caller is itself a supervisor.
Uncatchable front death can orphan its child, and is never successful completion.
Reopen an exec path only when a concrete consumer needs PID continuity and can
own the lost supervision contract.

The ephemeral channel carries a result rather than interpreting an exit status:
a natural child exit 0 conveys no opaque result and refuses. Atomic first-writer
publication preserves empty/multiline values and nested process context. An explicit
session target was rejected because it turns an enclosing-context action into
cross-session addressing. A persistent store was rejected by product scope;
caller provenance remains caller-owned. These constraints bind the
[executable specification](../specs/session-execution.md) and
[session channel](../session-channel.md).
