# Native-entry development evidence and remaining qualification

K28 remains open as the complete repair node. Its development child,
`native-entry-development-k30`, delivers working source across executable capture,
the launcher front, worker and runner with partial development instruments. It
is **not** a completed native-conformance campaign or a release qualification.
The required contract remains
[result-job completion](specs/result-job-completion.md#checked-native-entry-transitions)
and the [selected protocol](native-entry-transition-design.md). The full
acceptance contract stays live as `native-entry-complete-repair-k28`;
no module is accepted as a separately completed repair. The development increment
is committed together; review and qualification remain live.

## Implemented development surface

The draft checks executable-entry capture, independent SIGCHLD watching repair,
front-owned handlers, permanent reader masks, fallible reader creation, explicit
finalisation, first-signal retention, child reinstatement, and bounded owner-local
stop/observation/reap. The two fallible Started APIs retain the callback error's
I/O source and run cleanup before returning. Unknown terminal status preserves
foreign foreground and reports uncertain foreground/modes. These are source
changes with partial tests, not proof of every required transition.

The human settled two interface/behavior questions during development:

- A reported group-stop EPERM may coexist with success only after actual reap,
  successful terminal recovery and an ESRCH group-absence confirmation. Retain
  the report. An initiating transition failure remains a failure.
- Add `Ended::operation_reports: Vec<String>` and carry those reports in the
  existing single JSON notice as `operationReports`. Existing struct literals
  must add the field. Launch arguments and `Result<Ended, LaunchError>` stay
  unchanged. The notice now follows supervision; non-JSON operation reports the
  same returned observations to stderr.

## What the development runs establish

The [archived development output](evidence/native-entry-development/) contains
the available logs, including failures. These runs were used while constructing
the repair. Subjects changed between runs, successful nextest output often
suppresses the physical per-case logs, and controller validation has not run.
They must not be promoted into immutable baseline/broken/repaired receipts.

| Surface | Available development observation | Missing qualification |
|---|---|---|
| Started APIs | Missing-API compile failure, then both real-process API checks passed; failed callback source/errno and actual events checked | Full live-child/PTY parent-mask faults and paired/cancellation cases |
| E01 | All 16 entry tuples passed, independent capture and reinstatement mutants detected, using the private real-main executable | Immutable raw per-case records, full action/mask telemetry, baseline campaign and controller qualification |
| E02 | All six tuples passed on the actual initializer thread; entry-blocked-pending mutant detected only in pending rows | Qualified acknowledged controller and immutable per-case campaign |
| E03 | Both real worker and managed waits confirmed for default/ignored exec entry; no-child-wait was clear at exec on this host | Other supported targets; a failing inherited-ignore baseline is not claimed |
| E04 | Private worker flag repair passed and skipped repair failed; managed private flag row passed; raw exec flag observation separate | Managed-boundary ignore-only mutant and qualified receipts |
| F01 | Three capture-result failures refused both admissions; ignored-error mutants failed the expectation | Qualified controller and per-operation physical/raw receipts |
| F03 / S03 | Six reader faults and eight worker-settlement faults passed development checks, including creator/reader masks and actual independent fixture cleanup | Matching lost-error/deadline controls, controller qualification, complete row receipts |
| F07 | Five propagation faults plus real TERM death distinguished numeric fallback from signal death | Controller validation and immutable per-case subjects/records |
| S01 / S02 | Eight stop-report and seven wait-uncertainty rows passed development assertions; S01 lost-report mutants detected | S02 deadline/identity mutants and qualified outer ownership/controller records |
| S04 | Foreign foreground physically preserved under substituted unknown status; missing uncertainty report failed before the fix and passed afterwards | Six other S04 rows, full matching broken controls and controller qualification |
| Regression development | A workspace run passed 314 cases before later entry/main/finalisation edits | Fresh checks and identity reconciliation for the current draft; R01 campaign remains separate |

The initial libtest entry fixture is rejected for E02: its front ran on a
different thread from the executable initializer, so thread-directed pending
signals stayed on the original thread. The replacement isolated binary executes
the fixture from real `main` and checks recorded/current thread identity. It is
compiled with `--cfg test` only for the final target, in a separate target
directory; installed builds have no fixture dispatch switch. The Taskfile's
`test:native-entry` builds this fixture and runs development checks only.

The early capture check initially reached Darwin's KILL query rejection before
the intended fault. The strengthened operation/signal/errno assertion exposed
that false positive. Darwin's explicit unchangeable KILL/STOP query exclusion
then allowed the intended required-query checks. Both outputs are retained.

The first complete workspace attempt also exposed direct runner EPERM stderr
reports splitting the protected single JSON notice. The approved `Ended` field
addresses that interface gap. Another development failure showed an informational
recovered worker-poll report incorrectly vetoing admission; the draft separates
retained reports from unresolved stopping. Old failed logs are not green results.

## One narrow review

K28 spent its one in-session review on failed-Started cleanup and the
first-signal/reader/finaliser composition. Both returned findings were classified
**valid and actionable** after rereading the source:

- Stale mask knowledge could authorise disposition release after a failed final
  block. The named failing control and corrected passing run are archived.
- Unknown-status recovery could preserve foreign foreground silently, without
  reporting uncertain modes/foreground. A real-PTY failing control and corrected
  passing run are archived.

These executable controls address those fixes. No second in-session reviewer
was used. Further substantial review must be a tree-owned `review-impl` leaf;
the approved remainder reserves RC≤5 for that route, not another spawned reader.
The review does not cover later code or unconstructed matrix controls.

## Current preservation checks

The archived [command results](evidence/native-entry-development/serial-development-checks.json)
and [final fixture checks](evidence/native-entry-development/fixture-development-checks.json)
record full return codes and timings. Source digests before/after these runs
match. These snapshots exclude generated/external dependencies and are development
provenance, not the required all-subject native-conformance freeze.

| Task | Observed development result |
|---|---|
| `test:native-entry` | Revised recipe passed: 20 front tests, three runner-native tests and both public Started checks; five entry-family functions explicitly included |
| `test:cli` | Passed at default scheduling |
| `test:session` | Default-scheduling run failed after 202 passes on a one-second worker-ready timeout in the unchanged Unicode case; focused rerun passed, sequential full run passed all 242 |
| `test:runner` | Sequential full run passed all 78 |
| `test:terminal-recovery` | Passed actual PTY checks and front receipt consumption |
| `test` | Sequential workspace run passed all 320 active cases before the new fixture-dependent entry families became explicit-only |
| `check` | Before fixture wiring correction: 320 passed. Final check with private executable hidden: 315 passed, 27 ignored; formatting, scripts, worker build/probes/types, runtime integrity, lint and PTY recovery passed |

The [missing-fixture broken control](evidence/native-entry-development/fixture-prerequisite-red.txt)
failed as expected. Five **new** entry-family functions now require the explicit
native task, which builds their private real-main executable and includes them
while excluding child-only roles. This does not change a preexisting test flag or
remove a required native row. Ordinary check succeeds without that executable.
The [identity and execution comparison](evidence/native-entry-development/final-protected-reconciliation.json)
records every preexisting identity/ignored flag and every nonignored case's pass
in the final workspace check; no total substitutes for the item records.

The Unicode timeout remains a failed run. Its real worker was terminated and its
private directory removed. Passing reruns do not establish the timeout's cause or
prove default-concurrency reliability. No expectation or timeout was enlarged.

## Tree-owned review of the development artifact

`native-entry-development-k32` read k30's artifact against the current source and
reported one finding, integrated by `native-entry-development-k33`: common
finalisation consumed the runner latch **before** it confirmed the final block.
The runner's handlers outlive supervision and write only their own latch, so a
TERM arriving between that read and the block was consumed, absent from the
pending set and invisible to the front latch. Finalisation could then release
dispositions and the mask and let the result be written. The contract already
required the final blocked latch/pending snapshot (T8), so this was an
implementation defect, not an unclear contract. The review made no other
concrete report-composition finding and states it is not exhaustive.

`Handlers::finalise` now attempts the final block first and consumes the runner
latch afterwards, whether or not the block succeeded. Front/outcome/late/pending
precedence is unchanged. Two private development controls discriminate the
ordering:

- A real trivial launch leaves the runner's handlers installed over the front's.
  A test-only seam then raises a real TERM on the finalising thread after the
  entry-mask restore and immediately before the final block. The unfixed order
  returned no cancellation and failed at that assertion; the corrected order
  returns SIGTERM. The same boundary is also run with the final block failing
  (a substituted report), which must keep both the failed-snapshot report and the
  signal; the unfixed order failed there too.
- Both controls assert the boundary was reached and that the front handler did
  not run, so a wrong boundary or wrong handler cannot satisfy them.

These are development controls on this host (aarch64-apple-darwin only), not
immutable matrix receipts. The signal and runner handler are physical; the
failing block in the second control is a substituted report. They do not replace
E06's acknowledged boundary joins or E07's first-consumption cases, which stay
live in k31, and they qualify neither Linux nor Rust 1.85. After the fix, `check`
passed 317 run/27 skipped, `test:native-entry` passed its 20 front, 3 runner and
2 Started checks, and `test:terminal-recovery` passed. The two controls are new
identities; no existing identity or expectation changed.

## Barrier controller, E06 and E07 (k34)

`native-entry-barrier-controller-k34` replaced the polling fixture loop with one
owner-local controller, `crates/keyed-launch/src/native_barrier.rs`, compiled
only into unit-test builds and included by `#[path]` into the ravel test binary.
The fixture side acts only when the controller names a private directory, so no
installed switch exists. One exchange is `READY` → controller action → `P`/`ACK`
(facts read fresh) → `R`/`RELEASED`, over two FIFOs the controller opens before
the fixture exists. Waits use absolute deadlines that EINTR never restarts:
work stops at 25 seconds of the 30-second case, a probe or release is bounded by
5 seconds, and the controller owns the reserved final 5 seconds. The ready record
is bounded by the work deadline rather than 5 seconds because it covers the
fixture's own preceding work. No sleep orders anything; the controller's
millisecond slices only decide how often a deadline is examined.

| Control (frozen 30/5/5) | Observed on this host |
|---|---|
| Watchdog expiry | Fixture silent and not exiting: no ready record, expiry at 25 s, group then child stopped by the owner and reaped, settlement not clean, total ≤ 30 s |
| Blocked acknowledgement and failed release | Probe and release each failed at 5 s (10 s total); fixture stopped and reaped; two faults recorded; settlement not clean |
| Escaped holder | Controller-owned holder in its own group killed and reaped on both an aborted and a naturally ended case; reap confirmed by the owner's own wait |
| Reaped identity | After a confirmed reap a signal request is refused and a second settlement neither signals nor invents a status |

**E06** runs the private real-main fixture, with a barrier at each named boundary
and the actual signal delivered by the controller between acknowledged
exchanges. The boundaries are the point before the runner's cancellation join;
the start check before the parent mask is restored; the observer's `Reaped`
event (the confirmed reap, before the front regains control); the point before
common finalisation after a policy refusal; and the same point after a failed
spawn that has no `Ended`. The fixture reports, read-only and fresh at each
probe, the owner of each handled signal's disposition (front, other, default,
ignore), the calling thread's mask, the process pending set, the front latch,
and whether the managed child started, was reaped and ended.

On the repaired product all 15 rows (five boundaries × INT/TERM/HUP) died by the
selected signal, wrote zero result bytes and reported the cancellation in the
single JSON notice, with the boundary facts below. Each row was then rerun with
only the corresponding join removed by a fixed private enum:

| Boundary | Observed facts on the repaired run | Join removed | Named failure |
|---|---|---|---|
| selection-to-runner | front owns the signal, consumed into the front latch; no child | admission check | child started although cancelled; final join still reports the signal |
| Started, before mask restore | mask blocked, signal pending, runner's handler; `End::Interrupted` and group gone | runner outcome | cancellation lost, result written |
| after confirmed reap | child reaped, runner's handler consumed it, front latch empty; `End::Exited` | late runner latch | cancellation lost |
| policy refusal before finalise | front owns and consumed it; no child; original refusal in diagnostics beside cancellation | front latch | cancellation lost |
| spawn failure, no ending | runner's handlers outlive the failed spawn and consumed it; original ENOENT in diagnostics | late runner latch | cancellation lost |

The main thread is the only thread eligible at these boundaries; E08 owns the
reader-mask receipts. The 15 + 15 runs took 32 seconds in one case and passed on five further
repeats of the unchanged subject; that is flake detection, not an immutable
receipt.

**E07** runs at the runner level because its seams are the runner's: the
keyed-launch test binary re-executes itself as a role process and the controller
acknowledges, by the handler's own run counter incremented *after* the latch
write, that each delivery has been latched. A first signal is latched and
acknowledged; a second is delivered and the latch must still hold the first; the
runner then consumes the first and acknowledges that consumption; the second is
delivered again and must neither replace the first nor reach the launch's result.
The runner-error variants arm the existing private wait-error seam only after
that acknowledgement, so the error path settles with the first consumption
already taken. Four breaks each let the later signal win and fail a named result:
latch store overwrite (both variants), consumption guard and reap join (active),
and error composition (error). The development run named the later signal in
every broken result and kept the first acknowledgement intact for every break
except the latch itself.

An instrument defect was found and corrected during this work: the handler
counter first moved before the latch write, so a probe could read an empty latch
after the counter advanced. The ordering was fixed and the runs repeated.

These are development controls on aarch64-apple-darwin, not immutable matrix
receipts, and qualify neither Linux nor Rust 1.85.

## Handoff and reader controls: E04 managed, E05, E08 (k35)

`native-entry-handoff-reader-controls-k35` ran the three families that need a
real PTY, a runner-level mutant seam or an escaped reader on the k34 controller,
without creating another one. Everything below is a development control on
aarch64-apple-darwin, not an immutable receipt, and qualifies neither Linux nor
Rust 1.85. No product defect was found and production semantics are unchanged:
every new seam is `cfg(test)`.

**E04 managed.** The managed boundary is the runner's own `restore_child_watching`,
which the ravel fixture links in non-test form, so its independent ignore-only
mutant lives at the runner. The role installs a private default plus
`SA_NOCLDWAIT` after entry, the runner's own query records the flags it saw
immediately before its checked watching, and a real child's exit and wait are the
receipt. The ravel-level managed row remains the end-to-end positive; the raw
exec-flag reachability row is untouched and never relabelled.

| Run | Runner saw flags | After the launch | Real child |
|---|---|---|---|
| Repaired | `SA_NOCLDWAIT` set | cleared | `Started, Reaped`, status 7, group gone |
| Ignore-only mutant | `SA_NOCLDWAIT` set | **still set** | `Started` only; wait fails ECHILD, reap unconfirmed |

**E05.** A launcher role on a PTY of its own (session and group leader, slave as
controlling terminal) runs the transparent runner under the controller. A direct
INT is `kill(launcher)`; a terminal INT is the VINTR byte written to the master.
*Before handoff* is the last parent act before the spawn (launcher group
foreground, no child); *after handoff* is the Started start check (child exec'd
and foreground). The child records its pid, publishes through the channel and
waits with the default INT disposition. Publication is confirmed through
acknowledged probes before the signal, never a sleep.

| Row | Facts at the barrier | Launcher latch after | Launch result |
|---|---|---|---|
| direct INT / before | pid = sid = pgrp = foreground, no child | INT | `Interrupted(INT)` |
| direct INT / after | foreground = child ≠ launcher group, published | INT | `Interrupted(INT)`, group gone |
| terminal INT / before | pid = sid = pgrp = foreground, no child | INT | `Interrupted(INT)` |
| terminal INT / after | foreground = child ≠ launcher group, published | **empty** | `Exited`, child status SIGINT, signalled, group gone, no cancellation |

The invented-cancellation mutant reports the child's own SIGINT death as the
launcher's cancellation; only the publish-then-terminal-INT row detects it. The
runner alone has no admission check, so the before-handoff rows still spawn and
then cancel the child; the front's admission join is E06's selection-to-runner row.
The first run of this family exposed a fixture defect, kept here: a process
exiting onto an undrained PTY blocks in tty close beyond SIGKILL. The controller
reported it as watchdog expiry with an unresolved identity at 30 s, and the row
now owns a bounded drain thread joined on every path.

**E08.** The private worker is this same executable in a worker role speaking the
real protocol. For escaped-holder rows it hands its stdout to the **controller**
over a private socket (`SCM_RIGHTS`); the controller spawns and owns a holder on
that pipe in its own group, acknowledges, and the worker exits. A process-directed
TERM is then delivered at the common finalisation barrier. The real Bun worker is
not used for E08: it cannot give the controller a descriptor, and the
reader/mask/finalisation behaviour does not depend on the worker implementation.

| Readers | Result | Reader masks (own thread) | Creator before/after | Collection | Holder |
|---|---|---|---|---|---|
| both complete | success / refusal | both block INT, TERM, HUP | equal | 2 of 2, immediate | none |
| one escaped | success / refusal | both block INT, TERM, HUP | equal | 1 of 2, window ≤ 1 s, took = window | killed and waited by controller |

Owners at the barrier: refusal rows keep the front's handler (latch consumed on
the main thread, recorded by a test-only handler-thread store); success rows have
the runner's handlers installed over it, so the signal is joined late and the
front latch is empty. The main-thread observation therefore covers refusal rows
only. Breaking only the shield (report success, never block) leaves cancellation
intact, because the latch is process-wide, and fails exactly the readers'
eligibility receipt: their own masks record no handled signal blocked.

Protected checks after the final edit: `task test:native-entry` (26 + 6 + 2),
`task check` (320 run / 36 skipped, formatting, clippy, worker, PTY recovery) and
`task test:terminal-recovery` passed. R01 reconciled by identity: all 310 prior
identities present with unchanged ignored flags; 46 new identities. The three new
rows passed five further unchanged repeats (flake detection only).

## Exact remainder before acceptance

The [case manifest](native-entry-cases.json) preserves 153 concrete native rows
and the 310 preexisting test identities, including ignored fixtures. A row's
presence is a required case identity, not an execution result. Keep the frozen
parameter products, mutation modes, four paired failures, two queued-TERM
variants and S counts 8/7/8/7/2 unchanged.

1. E04's managed mutant, E05's four direct/terminal INT handoff cases and E08's
   four complete/escaped-reader cases (k35), E06's 15 acknowledged boundary
   joins and E07's four first-consumption cases now have development controls,
   above; they are not immutable receipts. Receipts must distinguish
   front/runner/child ownership, pending versus consumed state and real reader
   eligibility. No sleeps can establish ordering.
2. Construct F02's six watching faults, F04's 14 selection/runner handler faults,
   F05's 20 finalisation/Started-mask faults, and F06's six pre-exec faults.
   Keep pre-exec controls fixed and allocation-free. Exercise the four paired
   cases and two pending-TERM variants through the agreed real-process/PTY/front
   seams. Retain original and secondary reports under cancellation precedence.
3. Complete matching broken controls for the implemented families, especially
   S02's bound/identity claims and F03/S03 report/deadline claims. Finish S04's
   seven-row recovery set and both S05 first-HUP-then-TERM rows. Do not substitute
   report rewriting for an irreversible physical operation.
4. Qualify every native controller: 30 seconds total, acknowledgements within
   five seconds, last five seconds reserved for cleanup. The shared barrier
   controller below has its three failing controls; the older C4/terminal and
   worker fixture helpers are not yet routed through it. Preserve and boundedly
   settle every owned child, holder, pipe and PTY on failure as well as success.
   Older C4 fixture mechanics are not automatically a qualified native controller.
5. Finish all instruments before measurement. Freeze source, binaries, helpers,
   manifests, reference expectations, exclusions and measured platform domain;
   digest every read subject before/after. Produce one immutable receipt per
   baseline/broken/repaired case with one writer. Keep actual operations,
   substituted reports, product settlement and fixture cleanup distinct. Rerun
   affected rows after edits and reconcile identities individually.
6. Complete R01 through the existing Taskfile tasks and preserve every previous
   expectation. Publish qualified evidence only when earned. Reconcile the
   approved operation-report API amendment throughout docs. Price and cut the
   separate Rust 1.85/Linux support remainder inside k22 before the book.

No complete baseline subject or frozen native measurement campaign currently
exists. No matrix family is accepted from the development summaries above.
The source development increment is sealed under k30. K28 remains an open node;
`native-entry-qualification-k31` carries the complete remaining acceptance work
after funding and the tree-owned review. Decomposition does not reset the one
in-session reviewer allowance.

## Domain and continuation price

Development ran on aarch64-apple-darwin, macOS 27.0.1 (26A434), Darwin 27.0.0,
rustc 1.99.0 (`b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`), libc 0.2.186.
Actual Darwin headers and pinned XNU source informed the explicit signal-domain
exclusions; source links remain beside the non-obvious calls. Those reads do
not qualify Linux or minimum Rust 1.85. No infrastructure/install work, returning
host contract or independent adoption was performed.

The original P≤125 estimate underpriced the instruments and their lifecycle
controllers. Working entry receipts additionally required rejecting a libtest
thread premise and constructing the permitted private real-main fixture. The
complete matrix and bounded-controller obligations cannot fit the remaining
P≤55 after this attempt's conservative P≤70 charge. This is an insufficient
complete-price continuation, not permission to narrow acceptance.

The **remaining** work is priced, in minutes, at P≤100 (80 for remaining finite
instruments/controllers/source corrections, 20 for immutable evidence and
documentation), E≤70 (15 controller validation, 30 native campaigns, 10 protected
checks, 15 affected reruns/reconciliation), RC≤10 (3 agreement/recording, 5 for
the tree-owned review, 2 remaining coordination/closure), D=X=0. Balances after
the recorded attempt were D≤20 reserved/RC≤1/P≤55/E≤60/X=0, so the exact
shortfall was **P≤45/E≤10/RC≤9**, with no D/X increase. The attempt charges
P≤70/E≤30/RC≤4, D=X=0, once across k30/k28; cumulative bounds before funding
were D≤115/RC≤64/P≤225/E≤60/X=0. The E≤30 bound replaces the earlier E≤15
preservation proposal, covering the additional failed/focused/sequential and
clean-fixture verification. It does not add another P≤70 or RC≤4 charge.

The human's answer in `native-entry-qualification-funding-k29` was **“Approve
the renewal (Recommended)”**. Approved phase caps are D≤135/RC≤74/P≤325/E≤130/X=0,
retaining zero production disruption and the original D≤20 phase-closure reserve.
K29 charges conservative RC≤3, D=P=E=X=0; cumulative bounds are
D≤115/RC≤67/P≤225/E≤60/X=0 and balances D≤20 exclusively phase closure,
RC≤7/P≤100/E≤70/X=0. The tree-owned review `native-entry-development-k32`
receives RC≤5; any integration it earns and `native-entry-qualification-k31`
share P≤100/E≤70/RC≤2, D=X=0. Full use leaves only D≤20 reserved; allocations
are not spend. The approval covers the unchanged matrix, amendments and 30/5/5
envelope, not new protocol, infrastructure or narrower acceptance. No
correctness/root closure, book acceptance, comparative gain or support
qualification follows. K4/k6, G2–G5/R-W and the root's delivery/documentation
work remain live.
