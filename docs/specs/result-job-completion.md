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
the runner outcome's consumed signal (ending or error) and the runner's late
latch, in that order.
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
identifies the limits of that transfer at the fixed source reference. C1's
error-carried cancellation and no-ending front transfer are now corrected and
[checked](../completion-failure-design-evidence.md#c1-checked-observations).
Worker admission is now gated on confirmed reap and checked post-evaluation
private-directory removal, with [C2 checks](../completion-failure-design-evidence.md#c2-checked-observations)
covering failed reported waits/removal and original diagnostic preservation.
Private-directory allocators now retain recorded identity through partial
preparation, successful transfer and explicit checked cleanup; [C3 observations](../completion-failure-design-evidence.md#c3-checked-observations)
cover original/cleanup reports, replacement protection and admission.
[C4 observations](../completion-failure-design-evidence.md#c4-checked-observations)
now check retained terminal ownership through failed exec, reportable recovery
and actual spawn/reap distinctions on the measured host. Their exclusive lease
and group-identity premises remain conditional. These scoped observations do not
establish complete native-entry conformance or relax the session specification's
worker ordering, cancellation, cleanup or terminal promises. C1–C4's private
substituted reports and physical observations remain distinct from unexecuted
entry/toolchain/target assumptions.

Direct native entry is separate from the CLI's selected-scope transfer.
Transparent entry state is supplied by the caller and applied to the child;
it is not host-state capture or restoration. The current runner retains its
host cancellation handlers and any ignored-SIGCHLD repair after return. The
caller must define the capture instant, signal/terminal authority, ignored and
blocked-signal policy, late cancellation handling and host restoration lifetime.
Executable-entry transparency needs capture before runtime changes; a native
call-entry contract is a different choice. No concurrent embedding guarantee
follows from a synchronous call.

The agreed first independent native consumer is a launcher CLI: one invocation
supervises one managed launch, settles its result or failure after required
cleanup, and exits. Its contract requires authority over signal and terminal
state through that settlement. A persistent runner is a separate component with
its own continuing-host contract. Restoring a host that resumes arbitrary work
is not the initial launcher consumer requirement; entry transparency, cancellation,
cleanup and terminal recovery remain required. No concurrent embedding support
is inferred from this lifetime choice, and no combined interface or public
package is selected here. The
[native-entry evidence](../result-job-source-evidence.md#selected-scope-transfer-versus-native-entry)
and [failure counterexamples](../result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
name the smallest missing premises and checks still owed.

## Required failure settlement

The following is the repair contract for the incumbent launcher, derived from
the unchanged session promises and the agreed one-session consumer. It is a
contract for corrections, **not a claim of complete executable conformance**.
C1–C4 have scoped checked evidence; terminal authority premises and native-entry
limits above still apply. The
[repair evidence and check plan](../completion-failure-design-evidence.md)
separate that source, proposed enabling points and checks not yet run.

### Cancellation across supervision failure

The runner SHALL retain a cancellation it consumed while it owned supervision
in either its successful ending or its failure outcome. An observation failure,
an emergency-reap failure, or a reap failure after observing exit cannot erase
that signal. A failure outcome remains a failure; it supplies no invented ending,
confirmed reap or group absence. Successful reap notification stays conditional
on a real confirmed reap. Group signalling stays before reap while the direct
child reserves the group identity; an uncertain or already-reaped identity is
never a licence to signal a possibly reused group.

The consumer joins its front cancellation, the runner outcome's consumed
cancellation, and the late runner latch in that order. The runner outcome may
be an error with no ending. A later signal cannot replace an earlier consumed
runner cancellation. Requeueing that earlier signal into the global latch is
not the transfer contract: it could overwrite a late signal or couple outcomes
to a future launch. Cancellation suppresses every candidate result, then the
launcher reports failures and propagates the selected signal after required
cleanup attempts. Supervision/reap and recovery failures remain diagnostics
even when cancellation is the primary outcome.

### Worker teardown before admission

The evaluator SHALL require confirmed worker reap before returning a successful
evaluation. A successful policy conversation plus a teardown wait error is
worker failure, with exit 5, and cannot reach command resolution, successful
selection or a managed launch. Any confirmed exit status meets the reap
prerequisite; it need not be status zero. A timeout, policy refusal, protocol
failure or output-bound breach keeps its existing classification when cleanup
also fails, except that cancellation still wins. Every teardown failure is
reported alongside that primary outcome and the separately captured policy
diagnostics. A message cannot say “reaped” without that confirmation.

The evaluator closes its protocol end, attempts the existing bounded stop/kill
and wait, drains bounded diagnostics, and explicitly cleans its private
directory. No managed launch follows an unconfirmed reap or a failed required
directory cleanup. An unconfirmed reap is uncertainty, not evidence that the
worker remains alive or that it is gone. Failure reports the worker identity,
failed operation and recovery advice without pretending that an unsuccessful
wait can establish a successful cleanup. Detached policy children remain outside
the existing worker guarantee.

### Cleanup after partial acquisition

Each allocator SHALL own a newly created directory until it successfully
transfers that resource. Once acquisition has produced a directory, a later
returned error triggers explicit checked cleanup within that owner. This covers
the confined workspace, result-channel directory and worker private directory,
including failures before a usable wrapper or held reader can be returned.
The outcome retains the original error and every cleanup error, including the
directory path and whether removal failed or identity could not be established.
An allocator failure remains primary in the absence of cancellation. Cleanup
failure on otherwise successful worker preparation/evaluation prevents admission.

Ownership evidence must precede access granted to another process. Cleanup
checks the recorded owned identity against the current pathname. A replaced
directory is preserved; an owned directory moved elsewhere requires an explicit
diagnostic and possible manual removal. If identity recording itself fails,
the allocator must report that it cannot safely confirm deletion. It must not
remove a possibly different directory merely to hide the original failure.
After an explicit cleanup attempt, destructor removal is disarmed so it cannot
silently retry, delete a replacement or erase the reported stopping state.
Descriptors still close through their existing ownership. The incumbent compares
identity and then removes by pathname; this is not atomic against concurrent
pathname mutation. The [checked C3 controls](../completion-failure-design-evidence.md#c3-checked-observations)
cover moves and substitutions completed before the cleanup observation, not that
concurrent interval.

### Terminal recovery after failed exec

The runner SHALL retain a terminal lease through a failed spawn as well as
through supervision. It saves foreground authority and terminal modes before
allowing an interactive handoff. It records the attempted child's group before
that child can take foreground; child-side memory alone does not convey that
fact to the parent. Failure to prepare or convey that record refuses before
handoff. Child-side handoff is checked and precedes exec. These operations must
respect the restricted pre-exec environment.

After failed exec, the runner attempts checked recovery before returning its
spawn error. The admitted launcher owns the failed-spawn lease exclusively:
no competing process or thread may change foreground between the recovery's
ownership observation and handback. The recorded group number also requires
that no unrelated group acquires that identity during this interval. Without
those premises, a query followed by handback is only a checked-holder policy,
not an unconditional guarantee against taking another group's foreground.
The CLI lifetime does not itself establish either premise.

Under that authority, if the launcher still owns foreground, it verifies/restores its
saved modes. If foreground is the group recorded for this attempt, it takes
foreground back and restores those modes. An unrelated holder is preserved and
reported; the record does not authorise arbitrary terminal theft or signalling
a group after its identity can be reused. Foreground-query, handback, mode and
temporary SIGTTOU-disposition restoration failures are all reported. Missing
ownership information is reported as unconfirmed recovery, not success.

The original spawn cause and its errno remain primary without cancellation:
the CLI's existing missing-program 127 and other failed-start 126 classification
remain intact. Recovery failure is secondary, and cancellation can still override
the primary outcome after all reports are retained. Successful spawning retains
the existing supervision, delegated-foreground recovery rules and real-reap
event order. A terminal recovery error on an otherwise successful supervision
also prevents result success; any consumed cancellation must survive that error.
The existing start notification means **spawn-reported success**, not confirmed
exec. A child can die before exec while the library reports successful spawn.
That path still requires actual reap, group cleanup and terminal recovery;
an attempted-group record establishes neither exec nor a successful ending.

### Outcome and diagnostic order

| Situation | Primary outcome | Other information that must survive |
|---|---|---|
| Any applicable cancellation, with or without an ending | Selected INT/TERM/HUP, propagated after cleanup attempts; no result | Original operational failure, every cleanup/recovery error, known group survivor or unconfirmed lifecycle fact |
| No cancellation; acquisition, conversation, result, spawn or supervision already failed | Original failure and its existing code/stage/status | Every later cleanup/recovery failure; captured policy diagnostics remain policy output |
| No cancellation; otherwise successful operation with cleanup/recovery failure | First required cleanup/recovery failure | All other cleanup/recovery failures; no candidate result escapes |
| No earlier failure; a matching ending reports a surviving group | Group failure | Survivor identity; no result |
| Matching reap/group/terminal/resource settlement succeeds, no cancellation | Exact published UTF-8 result | Empty publication differs from absence; child's later exit status is not decisive |

The managed front retains mask restoration, channel close, confined-directory
removal and output restoration order. Runner-owned reap/terminal failures and
allocator-local cleanup reports are retained before those outer cleanups; a
pre-existing operational failure does not become a later cleanup error. Reporting
attempts continue after a failure, but a failed OS operation cannot be presented
as a fulfilled guarantee. If stderr itself is unwritable, emission can fail;
that does not permit result success or justify silently discarding the report
before attempting emission.

Only successful settlement reaches the final exact stdout write/flush. A failure
of that write remains an error and may have written a prefix. The final
cancellation join is the settlement decision point: cancellation already
observed or pending for delivery through the existing transition joins then.
A newly arriving signal after that decision follows the restored disposition
and may terminate the CLI during writing. There is no atomic guarantee spanning
signal delivery and arbitrary output writes, and no rollback of a written prefix.

## One-session native launcher entry

The first native consumer SHALL own one process lifetime: capture, prepare,
launch, supervise, settle, report and exit. It has exclusive authority over
the relevant process-wide signal dispositions and controlling-terminal lease
for that lifetime. Concurrent embedding and repeated independent launches in a
host that resumes arbitrary work are outside this contract. A persistent runner
requires a separate continuing-host contract; no such component is delivered
by these corrections.
For failed-spawn terminal recovery, exclusive authority specifically excludes
competing foreground transfers and unrelated reuse of the recorded group
identity through handback. If a consumer cannot supply those premises, the
proposed handback mechanism is not established for that consumer. Source or
PTY checks cannot convert that environment assumption into a universal guarantee.

Executable-entry signal transparency requires the ignored dispositions and
calling thread's blocked mask to be captured before relevant runtime changes,
including Rust's SIGPIPE startup change. The native runner accepts these values;
it does not capture them. Failure to establish entry state refuses transparency.
The child receives the captured exec-surviving ignore/default state and mask,
not copies of arbitrary host handlers. Host cancellation ignores stay ignored;
signals blocked at executable entry are not reclassified as cancellation merely
because they are pending. They remain blocked in the child. Nonignored signals
delivered to the launcher follow its cancellation policy.

The launcher must establish its child-watching state before **any** child it
needs to reap, including a policy worker. Inherited ignored SIGCHLD or
SA_NOCLDWAIT cannot silently remove the reap prerequisite. Any repair is host
state owned for the launcher lifetime; the managed child's supplied entry values
remain separate. Handler and mask transitions must be checked or retain their
failure reports. The existing source's unchecked transitions are still premises
to verify; the four counterexamples are not a proof that every OS failure is
covered.

Auxiliary threads must remain ineligible to consume the launcher's handled
signals during the handoff; masking only the calling thread is insufficient if
another thread is eligible. Keep handlers present while restoring the launcher's
entry mask and joining pending/front/runner/late observations on both success
and failure. Restore required terminal state and temporary handoff masks or
dispositions within the launch. Process-lifetime acceptance of runner handlers
and SIGCHLD repair does not waive those cleanups; it only removes an arbitrary
continuing host's restoration obligation.

Terminal INT follows actual foreground ownership. Before handoff it may reach
the launcher; after handoff it reaches the child's group. A transparent launcher
also treats a nonignored INT delivered directly to itself as cancellation even
when a terminal exists. A child killed by terminal INT is not automatically a
launcher cancellation observation: retain the published-result versus later-child-
status rule. Ignored/blocked entry state still governs the child's response.

At the exit boundary, cancellation is reported and re-raised as the selected
signal, using the existing numeric fallback only if signal death cannot occur.
Other failures keep their CLI statuses. Success writes only the exact result
bytes after settlement and exits. No call-entry capture, complete restoration of
a returning host, independent adopting project, package selection or observed
independent-use result is inferred from this lifetime agreement.

### Checked native-entry transitions

The launcher SHALL retain an immutable, complete executable-entry record
separately from its own signal repairs. A required valid-signal query failure
prevents worker and managed launch; unsupported or runtime-reserved signal
numbers are excluded explicitly, never by treating every query error as absence.
Child restoration checks every required action and mask operation. Host watching
repair covers ignored SIGCHLD and independent no-child-wait state before either
child, without changing the child's captured entry values.

A single front-owned scope SHALL survive selection failure as well as success.
Partial handler installation, reader creation and mask transitions retain their
known protection and every failure report through explicit settlement. No new
child is admitted after a failed transition. Output-reader shielding is checked
before creation and lasts until each thread exits. Bounded diagnostic collection
may leave shielded readers alive until CLI exit; no unbounded join or additional
policy-descendant supervision promise follows.

Failure of the parent-mask transition after spawn SHALL enter runner-owned
bounded teardown, actual wait and checked terminal recovery before returning.
It cannot become an early error return that abandons the spawned child. First
consumed cancellation survives later signals and every such failure. Simultaneous
pending standard signals supply no arrival chronology; only when no consumed
observation exists does fixed handled-signal order choose between pending ones.

Common finalisation SHALL preserve the original operational failure and every
secondary transition/cleanup report, including selection-failure and no-ending
paths. Unknown pending state cannot establish no cancellation or permit result
success. Handlers remain present through the mask-restoration join; disposition
release occurs under a blocked calling mask with readers still shielded. Failed
release prevents result output. Cancellation is reported before checked default,
selected-signal unblocking and re-raise; failed propagation uses the existing
numeric fallback and retains the transition diagnostics. This is a required
correction contract, not observed conformance. The
[transition design and finite discriminator plan](../native-entry-transition-design.md)
give its source, support limits, alternatives and complete repair estimate.
The [bounded error-settlement protocol](../native-entry-transition-design.md#bounded-error-settlement-protocol)
selects nonblocking ownership-preserving observation. Runner error settlement
SHALL stop polling within two seconds of failed Started or the first settlement
error; ordinary post-SIGKILL watching has the same bound from its first kill
attempt. Worker failure teardown SHALL finish its attempts within two seconds,
including its existing graceful second; bounded reader collection adds at most
one second. Absolute deadlines never restart on EINTR or secondary failures.
These are polling/retry limits, not real-time guarantees against scheduler or
kernel stalls. Existing normal production graces remain unchanged.

Every required stop, wait and recovery operation SHALL retain its own outcome
alongside the initiating failure and first consumed cancellation. Stop success
is not confirmed termination. Only an actual wait confirms reap; no result,
status, Reaped event or group absence may be invented on expiry or ECHILD.
After confirmed reap, identity loss or release, numeric signals are forbidden.
All required group stops precede reap while identity is reserved. Expiry SHALL
proceed to checked conditional terminal recovery and common finalisation,
reporting any still-unresolved child/group ownership rather than blocking until
physical exit. This may leave a surviving process; the error must say so.

Unknown-status recovery SHALL preserve an unrelated foreground holder and
report unconfirmed foreground and saved-mode recovery. It may reclaim the
recorded child's foreground only under the existing exclusive-authority and
no-unrelated-ID-reuse premises, confirming launcher foreground before restoring
modes. It SHALL retain query, handback, mode and temporary disposition failures,
and SHALL NOT lend again after a Started check failure. No broader delegated
foreground takeover is authorised by unknown status. The required behavior is
agreed; this mechanism and its finite cleanup-failure controls remain unimplemented
and are commissioned as the complete native-entry-complete-repair-k28 after
the k26 funding agreement and k27 planning handoff; conformance remains unobserved.

## Test seams for the repair contract

Retain the agreed public CLI with real TypeScript policy and fake managed jobs,
transport/real-process boundary and runner/PTY boundary. Existing private process
waiting is the enabling point for cancellation followed by observation or reap
failure. The justified new private controls are evaluator teardown-report
substitution before success admission, allocator-local post-creation failure and
cleanup-report controls, and a restricted terminal-recovery outcome control where
a real PTY alone cannot force recovery failure. Actual fixture processes are
still waited for; a reported failure is not permission to leak the fixture.

The [discriminating matrix](../completion-failure-design-evidence.md#discriminating-checks)
names the before/after expectations and controls. Keep protected fixed-reference
expectations outside the repair: configuration, CLI classification, exact results,
first-writer publication, nesting, combined-failure precedence/diagnostics, late
cancellation, descendant/group cleanup, reap, terminal recovery and final-write
behavior. Private fault controls cannot become public CLI switches or an
unbounded syscall abstraction. No behavioral instrument was built or executed
for the original design. C1 now has the separate checked observations linked
above; its recovery-report control does not establish actual checked terminal
recovery.

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
