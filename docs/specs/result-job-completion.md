# Result-job completion

An opaque result is a request to finish a managed job. A readable result file
does not establish that the matching job was reaped, that its group is gone,
or that cancellation and cleanup permit returning it. The
[session specification](session-execution.md#managed-process-and-result) owns
those promises. This document describes the completion decision that a native
consumer must currently compose. It selects no replacement interface or package.

## One completion decision

The result channel belongs to one launch. Its consumer retains that association
from allocation through supervision and reading; an ending from another launch
is insufficient. On successful return, the runner establishes reap, ending cause
and group state. The transport establishes whether publication occurred and validates the exact
UTF-8 contents through its held directory. The consumer joins these facts with
explicit resource cleanup and every applicable cancellation observation.

The [current component guide](../components.md#cancellation-and-partial-failure)
explains the participants' different knowledge. Its
[failure precedence](../components.md#error-and-signal-precedence) is part of
the completion contract, including diagnostic reporting. Cancellation suppresses
even a previously published result. Without cancellation, the original result
failure stays primary; otherwise the first cleanup failure wins, then a surviving
group prevents success. Every supplied cleanup failure and surviving-group
diagnostic is reported. A child's later exit status does not invalidate a
successfully published result; absence remains distinct from an empty value.

For the CLI, successful settlement precedes the exact result write. Output
reservation/restoration and confined-directory cleanup are part of that
settlement. The front joins its selection/transition cancellation latch,
the runner's interrupted ending and the runner's late latch, in that order.
The final write can still fail and can have written a prefix before failing. The
[supervision decision](../adr/ravel-supervises-the-session.md) and
[job decision](../adr/the-launched-child-is-a-job.md) explain the retained
supervisor and process-group choices; this document changes neither.

## Acquisition and entry limits

Successful selection retains a live selection-cancellation guard and the
prepared private workspace. An internal composition could take those together
before reserving result output and allocating its own channel. It would need
to retain an optional ending and settle all acquired selected-scope resources
even when no ending exists. These are existing ownership enabling points,
not an implemented complete-operation interface.

The [current-source acquisition inventory](../result-job-source-evidence.md#acquisition-and-stopping-state-inventory)
identifies the limits of that transfer. A cancellation consumed by the runner
can be lost when supervision subsequently fails without an ending. A successful
policy conversation does not check the worker teardown wait result before
returning selection. Partial directory acquisition returns no explicit removal
result. An interactive pre-exec terminal handoff followed by exec failure can
bypass terminal reclaim. These source-admitted cases prevent asserting complete
settlement on every failure path. They do not relax the session specification's
worker ordering, cancellation, cleanup or terminal promises, and have not been
observed at runtime in this investigation.

Direct native entry is separate from the CLI's selected-scope transfer.
Transparent entry state is supplied by the caller and applied to the child;
it is not host-state capture or restoration. The current runner retains its
host cancellation handlers and any ignored-SIGCHLD repair after return. The
caller must define the capture instant, signal/terminal authority, ignored and
blocked-signal policy, late cancellation handling and host restoration lifetime.
Executable-entry transparency needs capture before runtime changes; a native
call-entry contract is a different choice. No concurrent embedding guarantee
follows from a synchronous call.

A process-lifetime supervisor can explicitly accept those persistent changes;
a host that resumes arbitrary work needs an established restoration contract.
Neither native arrangement nor any public package is selected here. The
[native-entry evidence](../result-job-source-evidence.md#selected-scope-transfer-versus-native-entry)
and [failure counterexamples](../result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
name the smallest missing premises and checks still owed.

## Boundary criterion

A proposed higher-level interface earns the matching-job guarantee only if
its owner creates or exclusively owns the channel-to-launch association and
checks the actual ending. Accepting an arbitrary caller-supplied ending leaves
the association with the caller. Returning candidate bytes before explicit
cleanup leaves the completion decision with the caller too.

An owner can settle only the resources and cancellation observations it owns
or receives under an explicit transfer contract. Moving the read and close
calls into a helper does not transfer the front's signal state, confined
workspace or output restoration. Their ordering and failure composition must
remain stated even if they become internal to a combined module.

The [visual companion](../design/result-job/index.html#discussion) distinguishes
the observed composition from a proposed regrouping. That proposal remains
unselected: source observation now supplies concrete failure and native-entry
limits, rather than a universal transfer guarantee. Resolving those limits,
recurring support cost and a working migration route still need assessment.
Public packaging and independent consumer execution remain separate questions.

## Evidence and checks

The nominal completion path is source-supported at the fixed implementation
reference named in the [component evidence](../components.md#source-and-observation-evidence).
The existing exact-result CLI case, cancellation CLI case and publish-before-
cancellation process case provide the partial observations recorded there.
They were not rerun for this design comparison.

The later acquisition observation kept that source reference and used one fresh
read-only exception-path review. It ran presentation checks only. Its
[evidence limits](../result-job-source-evidence.md#evidence-limits-and-continuation)
retain the closed comparison's funding/history and remaining cost, delivery,
search, runtime and whole-replacement lifecycle questions. Association support
does not establish complete settlement or runtime conformance.

Use the already agreed CLI with real TypeScript policy and fake launched
programs, transport/real-process tests, and runner/terminal tests. Any touched
composition must retain first-writer and anchored-read checks, nesting,
publication/cancellation overlap, combined result/cleanup failure diagnostics
and precedence, late-signal handoff, descendant cleanup, reap and terminal
recovery. An unrelated ending is a discriminating interface case. Source
reasoning and diagrams establish no runtime preservation, external adoption
or human comprehension result.
