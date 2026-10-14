# Ravel language

Ravel routes a caller's request through owner configuration and supervises the
selected process. These terms distinguish configuration authority, session
authority and the meaning a caller gives a result.

## Language

**Caller**: The person or program invoking Ravel and interpreting its returned
opaque result. Its task conventions and continuation decisions are caller-owned.

**Owner policy**: The executable TypeScript configuration authorised to select
a command or refuse the caller's request. Its author controls routing choices.

**Policy worker**: The separate process evaluating the owner policy for the
native front. It is not a managed process and receives no session-result context.

**Managed process**: The direct child selected by the owner policy and supervised
as a job by the native front. Its process group is the scope of job cleanup.

**Session-result context**: The inherited `RAVEL_EXIT_FILE` path identifying the
enclosing session's ephemeral completion channel. A nested session replaces it
only for that session's managed process.

**Opaque result**: The single UTF-8 argument published by `end-session` and
returned unchanged after successful supervision and cleanup. Only the caller
assigns it meaning; an explicitly empty result differs from no publication.

## Example dialogue

Caller author: “If the managed process publishes `continue`, will Ravel start
the next task?”

Ravel maintainer: “Ravel cleans up the managed job and returns those bytes. The
caller decides whether they mean another task. The owner policy chooses the
command for each request; the policy worker never receives the session-result
context.”
