# Result-job completion

This contract applies to the current Rhai path.
The linked acquisition, correction and native-entry receipts record earlier pinned revisions, including the retired TypeScript implementation.
Their observations qualify those revisions and targets. They do not establish conformance of the current Rhai evaluator.
Current command-line acceptance checks use a real Rhai configuration script and fake launched programs.

<!-- tc: pending because the completion contract exceeds 60 hard findings and this reconciliation changes other sections -->

An opaque result is a request to finish a managed job. A readable result file
does not establish that the matching job was reaped, that its group is gone,
or that cancellation and cleanup permit returning it. The
[session specification](session-execution.md#managed-process-and-result) owns
those promises. This document describes the completion decision that a native
consumer must currently compose. It selects no replacement interface or package.

<!-- tc: on -->

## One completion decision

The result channel belongs to one launch. Its consumer keeps that association
from allocation through supervision and the read. An ending from another launch
does not satisfy it. On a successful return, the runner establishes the reap,
the cause of the ending and the group state. The transport establishes whether a
publication occurred, and it checks the exact UTF-8 contents through its held
directory. The consumer joins these facts with explicit resource cleanup and
with every applicable cancellation observation.

The book's [chapter on following a session through the code](../book/src/08-follow-a-session-through-the-code.md#follow-cancellation)
explains the different knowledge of each participant. Its
[failure precedence](../book/src/08-follow-a-session-through-the-code.md#choose-the-final-outcome) is part of
the completion contract, including the report of diagnostics. Cancellation
suppresses even a result that the managed process published before it. Without cancellation,
the original result failure stays primary. Without an original failure, the
first cleanup failure wins, and then a surviving group prevents success. The
consumer reports every supplied cleanup failure and every diagnostic of a
surviving group.

A later exit status of the child does not invalidate a successfully published
result. An absent result stays distinct from an empty value.

For the CLI, successful settlement comes before the exact result write. Output
reservation and restoration are part of that settlement. The front joins three
cancellation sources in this order:

1. its selection and transition latch
2. the signal that the runner outcome consumed, from an ending or an error
3. the late latch of the runner

The final write can still fail, and it can write a prefix before it fails. The
[supervision decision](../adr/ravel-supervises-the-session.md) and
[job decision](../adr/the-launched-child-is-a-job.md) explain the kept
supervisor and process-group choices. This document changes neither.

## Acquisition and entry limits

Successful selection keeps a live guard for selection cancellation. An internal
composition could take that guard before it reserves the result output and
allocates its own channel. That composition would need to keep an optional
ending. It would also need to settle all the acquired resources of the selected
scope, even when no ending exists. These are existing points that enable
ownership. They are not an implemented interface for a complete operation.

The [current-source acquisition inventory](../result-job-source-evidence.md#acquisition-and-stopping-state-inventory)
identifies the limits of that transfer at the pinned source reference. The
corrections of C1 cover the cancellation that an error carries and the front
transfer without an ending. Its [checked observations](../completion-failure-design-evidence.md#c1-checked-observations)
record them.

<!-- tc: depart synonym-rotation because confirmed reap is an observed lifecycle fact and checked removal is a separate resource operation -->
Worker admission now needs a confirmed reap and a checked removal
of the private directory after evaluation. The [C2 checks](../completion-failure-design-evidence.md#c2-checked-observations)
cover reported failures of the wait and of the removal, and the preservation of
the original diagnostic.

The allocators of private directories now keep the recorded identity through
partial preparation, successful transfer and explicit checked cleanup. The
[C3 observations](../completion-failure-design-evidence.md#c3-checked-observations)
cover the original and cleanup reports, the protection of a replacement and the
admission. The [C4 observations](../completion-failure-design-evidence.md#c4-checked-observations)
check, on the measured host, that terminal ownership survives a failed exec.
They also check reportable recovery and the actual distinction between spawn and
reap. Their premises of an exclusive lease and a stable group identity stay
conditional.

These scoped observations do not establish complete native-entry conformance.
They also do not relax the worker order, cancellation, cleanup or terminal
promises of the session specification. The private substituted reports and
physical observations of C1–C4 stay distinct from entry, toolchain and target
assumptions that no check executed.

Direct native entry is separate from the selected-scope transfer of the CLI.
The caller supplies the transparent entry state, and the runner applies it to
the child. That state is not a capture or a restoration of the host state.
After return, the current runner keeps its host cancellation handlers and any
change that it made to an ignored SIGCHLD.

The caller must define these items:

- the capture instant
- the signal and terminal authority
- the policy for ignored and blocked signals
- the handling of late cancellation
- the lifetime of host restoration

Executable-entry transparency needs a capture before the runtime changes the
state. A native call-entry contract is a different choice. A synchronous call
gives no guarantee for concurrent embedding.

The agreed first independent native consumer is a launcher CLI. One invocation
supervises one managed launch, settles its result or failure after the required
cleanup, and exits. Its contract needs authority over the signal and terminal
state through that settlement. A persistent runner is a separate component with
its own contract for a continuing host. The initial launcher consumer does not
need to restore a host that resumes arbitrary work. Entry transparency,
cancellation, cleanup and terminal recovery stay required.

This lifetime choice implies no support for concurrent embedding. This document
selects no combined interface or public package. The
[native-entry evidence](../result-job-source-evidence.md#selected-scope-transfer-versus-native-entry)
and [failure counterexamples](../result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
name the smallest missing premises and the checks that are still owed.

<!-- tc: pending because the completion contract exceeds 60 hard findings and this reconciliation changes other sections -->

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

<!-- tc: on -->

### Worker teardown before admission

<!-- tc: depart synonym-rotation because confirmed reap is an observed lifecycle fact and checked removal is a separate resource operation -->
The evaluator SHALL require confirmed worker reap before returning a successful
evaluation. A successful Rhai conversation plus a teardown wait error is worker failure, with exit 5.
It cannot reach command resolution, successful selection or a managed launch. Any confirmed exit status meets the reap
prerequisite. It need not be status zero. A timeout, configuration refusal, protocol
failure or output-bound breach keeps its existing classification when cleanup
also fails, except that cancellation still wins.

Every teardown failure is
reported alongside that primary outcome and the separately captured configuration
diagnostics. A message cannot say “reaped” without that confirmation.

The evaluator closes its protocol end, attempts the existing bounded stop/kill
and wait, drains bounded diagnostics, and explicitly cleans its private
directory. No managed launch follows an unconfirmed reap or a failed required
directory cleanup. An unconfirmed reap is uncertainty, not evidence that the
worker remains alive or that it is gone. Failure reports the worker identity,
failed operation and recovery advice without pretending that an unsuccessful
wait can establish a successful cleanup. The Rhai worker cannot launch a child. Harness-run cleanup follows the
[configuration evaluation contract](configuration-evaluation.md#cancellation-and-deadlines).

### Cleanup after partial acquisition

Each allocator SHALL own a newly created directory until it successfully
transfers that resource. After acquisition produces a directory, a later
returned error causes explicit checked cleanup within that owner. This rule
covers the result-channel directory and the worker private directory. It
includes failures before the allocator can return a usable wrapper or held
reader. The outcome keeps the original error and every cleanup error. Each
cleanup error names the directory path and states whether the removal or the
identity record failed.

Without cancellation, an allocator failure stays primary. A cleanup failure
after an otherwise successful worker preparation or evaluation prevents
admission.

Ownership evidence must come before the allocator grants access to another
process. Cleanup compares the recorded owned identity with the current pathname.
Cleanup keeps a replaced directory. An owned directory that moved elsewhere
needs an explicit diagnostic and possibly a manual removal. If the allocator
cannot record the identity, it must report that it cannot safely confirm the
removal. It must not remove a possibly different directory only to hide the
original failure.

After an explicit cleanup attempt, the allocator disarms the removal by the
destructor. The destructor then cannot silently retry, remove a replacement or
lose the reported stopping state. Descriptors still close through their
existing ownership. The incumbent compares the identity and then removes by
pathname. This sequence is not atomic against a concurrent change of the
pathname. The [checked C3 controls](../completion-failure-design-evidence.md#c3-checked-observations)
cover moves and substitutions that complete before the cleanup observation, not
that concurrent interval.

<!-- tc: pending because the completion contract exceeds 60 hard findings and this reconciliation changes other sections -->

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

<!-- tc: on -->

| Situation | Primary outcome | Other information that must survive |
|---|---|---|
| Any applicable cancellation, with or without an ending | Selected INT/TERM/HUP, propagated after cleanup attempts. No result | Original operational failure, every cleanup/recovery error, known group survivor or unconfirmed lifecycle fact |
| No cancellation. Acquisition, conversation, result, spawn or supervision already failed | Original failure and its existing code/stage/status | Every later cleanup/recovery failure. Captured configuration diagnostics remain script output |
| No cancellation. Otherwise successful operation with cleanup/recovery failure | First required cleanup/recovery failure | All other cleanup/recovery failures. No candidate result escapes |
| No earlier failure. A matching ending reports a surviving group | Group failure | Survivor identity. No result |
| Matching reap/group/terminal/resource settlement succeeds, no cancellation | Exact published UTF-8 result | Empty publication differs from absence. Child's later exit status is not decisive |

The managed front keeps this cleanup order: mask restoration, channel close,
then output restoration. The front holds the reap and terminal failures of the
runner before those outer cleanups run. It also holds the local cleanup reports
of each allocator. An operational failure that existed before does not become a
later cleanup error.

Reports continue after a failure. A failed operation of the operating system
cannot appear as a fulfilled guarantee. If stderr is not writable, the emission
of a report can fail. That failure does not permit result success. It also does
not justify the silent discard of the report before an emission attempt.

Only a successful settlement reaches the final exact write and flush of stdout.
A failure of that write stays an error, and the write can have written a prefix.
The final cancellation join is the decision point of the settlement. A
cancellation joins at that point if the front observed it before, or if it is
pending for delivery through the existing transition. A signal that arrives
after that decision follows the restored disposition, and it can end the CLI
during the write. No atomic guarantee spans signal delivery and arbitrary output
writes, and no rollback removes a written prefix.

<!-- tc: pending because the untouched completion sections exceed this reconciliation's prose-conversion scope -->

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

<!-- tc: on -->

The launcher must establish its child-watching state before **any** child it
needs to reap, including the Rhai worker. Inherited ignored SIGCHLD or
SA_NOCLDWAIT cannot silently remove the reap prerequisite. Any repair is host
state owned for the launcher lifetime.

The managed child's supplied entry values
remain separate. Handler and mask transitions must be checked or retain their
failure reports. The transition requirements below govern current checks.

The four counterexamples are not a proof that every OS failure is
covered.

<!-- tc: pending because the untouched completion sections exceed this reconciliation's prose-conversion scope -->

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

<!-- tc: on -->

A single front-owned scope SHALL survive selection failure as well as success. Partial handler installation, reader creation and mask transitions retain their
known protection and every failure report through explicit settlement. No new
child is admitted after a failed transition.

Output-reader shielding is checked
before creation and lasts until each thread exits. The file-reader thread of a
Rhai evaluation takes the same shield. A failed block, creation or creator
restore refuses the evaluation before any host call runs.

Bounded diagnostic
collection and an abandoned configuration read may leave shielded readers alive until
CLI exit. No unbounded join or additional script-descendant supervision promise
follows.

<!-- tc: pending because the untouched completion sections exceed this reconciliation's prose-conversion scope -->

Failure of the parent-mask transition after spawn SHALL enter runner-owned
bounded teardown, actual wait and checked terminal recovery before returning.
It cannot become an early error return that abandons the spawned child. First
consumed cancellation survives later signals and every such failure. Simultaneous
pending standard signals supply no arrival chronology; only when no consumed
observation exists does fixed handled-signal order choose between pending ones.

Common finalisation SHALL preserve the original operational failure and every
secondary transition/cleanup report, including selection-failure and no-ending
paths. Unknown pending state cannot establish no cancellation or permit result
success. A pending set that could not be read leaves the handled signals blocked
until every report is written: restoring the mask first would deliver a queued
signal under its default action and end the launcher before the original failure
or any secondary report existed. Handlers remain present through the mask-restoration join; disposition
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
is not confirmed termination. A stop EPERM report may coexist with successful
completion only after actual reap, successful terminal recovery and an ESRCH
group-absence confirmation. The report remains in `Ended::operation_reports`
and the front's existing single JSON notice (`operationReports`), or its
non-JSON diagnostics; EPERM itself never establishes absence. A group stop
during watching that answers EPERM SHALL NOT end the watch by itself. On
Darwin, a leader inside its exit teardown leaves no signallable member before
`waitid` reports its exit. That report waits for the same three confirmations.
A refused stop to the direct child, or any other errno, before the observed
exit still ends the watch. Initiating
transition errors remain failures. Only an actual wait confirms reap; no result,
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
agreed and implemented, and its conformance is measured per target: each
measurement qualifies only the kernel, libc and compiler it names, and only as
far as its receipts say. The frozen 153-row matrix, its incumbent baseline and
the 310 protected test identities are measured on three targets:
aarch64-apple-darwin (Rust 1.99), aarch64-unknown-linux-gnu natively at
Rust 1.85.0, which was the declared minimum then, and x86_64-unknown-linux-gnu under QEMU user-mode
emulation. The x86_64 measurement is emulated and is never a qualification of an
x86-64 kernel: it cannot create user namespaces or honour SA_NOCLDWAIT, so its
rows and protected identities that need either are accounted for with a reason
and a control, not reconciled and not dropped. Each target's domain, receipts,
row-by-row reconciliation and limits are in the
[evidence note](../native-entry-development-evidence.md); this paragraph claims
nothing a receipt there does not carry. Native-entry conformance is measured
with immutable receipts on aarch64-apple-darwin and aarch64-unknown-linux-gnu
only; nothing about an x86-64 kernel is qualified. Qualification on a native
x86-64 kernel is deferred, not rejected, and tracked in
[Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1). Open and
unmeasured: any other kernel, libc or Rust version, a returning host and
independent adoption.

<!-- tc: on -->

## Test seams for the repair contract

Retain the agreed public CLI with a real Rhai configuration script and fake managed jobs,
transport/real-process boundary and runner/PTY boundary. Existing private process
waiting is the enabling point for cancellation followed by observation or reap
failure. The justified new private controls cover evaluator teardown reports before success admission, allocator-local post-creation failure and cleanup reports.
A restricted terminal-recovery control covers outcomes that a real PTY alone cannot force. Actual fixture processes are
still waited for. A reported failure is not permission to leak the fixture.

The [discriminating matrix](../completion-failure-design-evidence.md#discriminating-checks)
names the before/after expectations and controls. The repair must preserve the reference expectations for configuration, CLI classification, exact results, first-writer publication and nesting.
It must preserve combined-failure precedence/diagnostics, late cancellation, descendant/group cleanup, reap, terminal recovery and final-write behaviour.

Private fault controls cannot become public CLI switches or an
unbounded syscall abstraction. No behavioural instrument was built or executed
for the original design.

<!-- tc: depart synonym-rotation because checked observations name scoped tests and confirmation names an observed lifecycle fact -->
C1 now has the separate checked observations linked
above. Its recovery-report control does not establish actual terminal
recovery.

## Boundary criterion

A proposed interface at a higher level earns the matching-job guarantee only if
its owner creates or exclusively owns the association of channel and launch.
That owner must also check the actual ending. An interface that accepts an
arbitrary ending from the caller leaves the association with the caller. An
interface that returns candidate bytes before explicit cleanup also leaves the
completion decision with the caller.

An owner can settle only the resources and cancellation observations that it
owns or that it receives under an explicit transfer contract. A helper that
takes over the read and close calls does not receive the signal state of the
front or the output restoration. Their order and failure composition must stay
stated, even if they become internal to a combined module.

The [visual companion](../design/result-job/index.html#discussion) distinguishes
the observed composition from a proposed regrouping. That proposal stays
unselected. Source observation now supplies concrete failure and native-entry
limits, not a universal transfer guarantee. The resolution of those limits, the
recurring support cost and a working migration route still need assessment.
Public packaging and the execution of an independent consumer stay separate
questions.

## Evidence and checks

<!-- tc: depart synonym-rotation because fixed implementation reference names a revision and repair names corrective work -->
The nominal completion path is source-supported at the fixed implementation
reference named in the [component evidence](../component-evidence.md#source-reference-and-discovery).
The existing exact-result CLI case, cancellation CLI case and publish-before-
cancellation process case provide the partial observations recorded there.
They were not rerun for this design comparison.

The later acquisition observation kept that source reference and used one fresh
read-only exception-path review. It ran presentation checks only. Its
[evidence limits](../result-job-source-evidence.md#evidence-limits-and-continuation)
retain the closed comparison's funding/history and remaining cost, delivery,
search, runtime and whole-replacement lifecycle questions. Association support
does not establish complete settlement or runtime conformance.

Use the already agreed CLI with a real Rhai configuration script and fake launched
programs, transport/real-process tests, and runner/terminal tests. Any touched composition must retain first-writer and anchored-read checks, nesting and publication/cancellation overlap.
It must retain combined result/cleanup failure diagnostics and precedence, late-signal handoff, descendant cleanup, reap and terminal recovery. An unrelated ending is a discriminating interface case. Source
reasoning and diagrams establish no runtime preservation, external adoption
or human comprehension result.
