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

## Fault families, paired and queued-TERM controls (k36)

`native-entry-fault-families-k36` constructed the remaining F families on the
k34 controller: F02 (6), F04 (14), F05 (20), F06 (6), the four paired failures
and the two queued-TERM variants. Everything here is a development control on
aarch64-apple-darwin, not the immutable campaign; it qualifies neither Linux nor
Rust 1.85, and no family is accepted.

**Mechanism.** One private fixed-site hook per owner. A *mutation* site either
omits the physical operation (a failed mutation that changed nothing) or
performs it and then reports EIO; a *query* site always performs the physical
query and substitutes only the report. An irreversible operation is never
simulated by rewriting its report. Each row writes a receipt that keeps the
operation performed, the actual result/errno, the reported EIO and the actual
state afterwards (handler owners, mask, pending, SIGCHLD flags) as separate
fields. The row's one broken subject ignores that failure at its site. The
front/worker sites are `cfg(test)` in `ravel`. The runner's sites are `cfg(test)`
in `keyed-launch`, plus one off-by-default Cargo feature, `native-faults`, that
only the isolated fixture target enables (`task test:native-entry`) so F06's
forked-child sites are reachable from the front's real-main fixture; installed
and release builds never enable it. The child's pre-exec sites use two atomics
and one fixed two-byte `write` (site; performed, failed, foreground bits), with
no allocation or formatting.

| Rows | Test (owner) | Observed on the repaired product | Broken subject |
|---|---|---|---|
| F02 worker ×3 | `native_f02_worker_watching_faults_refuse_before_any_worker` (ravel fixture) | `worker_failed` “cannot establish worker watching”, no worker waited, zero bytes; omitted repair left no-child-wait set, performed repair cleared it | Worker admitted; its own later ECHILD teardown failure replaces the watching refusal |
| F02 managed ×3 | `native_f02_managed_watching_faults_refuse_before_any_child` (runner role) | “cannot query/repair child watching”, no Started, spawn errno accessor `None` (spawn-only), no stray child, same omit/perform flag states | Child admitted (`Started`) |
| F04 selection ×5 | `native_f04_selection_handler_faults_fail_admission_with_the_partial_subset_owned` | `cancellation_unavailable` before any worker; the front owns exactly the installed subset (query: none; INT omit: none; INT perform: INT; TERM omit: INT; TERM perform: INT, TERM) with the handled signals still blocked at finalisation | Admission proceeds, worker runs |
| F04 runner ×5 | `native_f04_runner_handler_faults_fail_admission_with_the_partial_subset_owned` (runner role) | “cannot install runner handler for signal N”, no Started; the runner installs TERM, HUP, INT in that order, so the partial subset is TERM/HUP for an INT fault and none or TERM for a TERM fault | Child admitted |
| F04 INT final restore ×4 (selection refusal, runner completed launch) | `native_f04_int_final_restore_failure_retains_ownership_and_blocks_the_result` | The release failure keeps the installed owner (omit) or the entry action (perform) and keeps the mask blocked; refusal or ending retained, “disposition release” reported, zero bytes; the completed launch's child ran, was reaped and the group gone | Failure not reported, result written |
| F05 finalisation ×18 | `native_f05_finalisation_failures_keep_the_original_and_never_write_a_result` | Three outcomes × block omit/perform, pending query, TERM membership, final mask restore omit/perform: original outcome kept, secondary label present, zero bytes; the block/restore rows' physical mask matches the operation, not the report | Secondary label absent; queued-TERM rows die by the TERM with no report |
| F05 Started ×2 | `native_f05_started_restore_failure_cleans_up_the_live_child_and_terminal` (PTY lease) | Started once, confirmed Reaped before return, no Ended, terminal foreground back with the launcher, no child left (`waitid` WNOWAIT), `supervision_failed` with the group-stop reports kept; omit left the handled signals blocked, perform restored them | The launch completes and writes its result |
| F06 ×6 | `native_f06_child_restoration_faults_fail_the_spawn_and_recover_the_terminal` (PTY role) | Spawn fails with EIO as the actual spawn errno, no Started, the child's own record shows performed/omitted while it was the terminal's foreground group, foreground and modes restored, no stray child | Child runs and exits 7 |
| Paired F03 | `native_paired_f03_second_create_keeps_the_worker_wait_report` | Second reader creation (EAGAIN) and the worker's try_wait EIO both in the failure | Worker reports discarded; the pair assertion rejects it |
| Paired F05 Started + recovery, F06 + C4 recovery | `native_paired_started_restore_and_child_action_keep_the_recovery_report` (runner role) | Primary and the substituted terminal-handback report both returned; the handback physically ran | Recovery report discarded |
| Paired F05 refusal | `native_paired_f05_refusal_keeps_its_exit_beside_the_final_restore_failure` | Refusal's own code and exit 3 kept; “final mask release” reported beside it | Secondary absent |
| Queued TERM, F05 Started | `native_f05_started_restore_failure_with_queued_term_joins_at_finalisation` (front) | TERM pending behind the handoff block, never consumed; joined at finalisation: death by TERM, original Started failure in diagnostics | Failure never exists to retain |
| Queued TERM, F06 | `native_f06_action_failure_with_queued_term_joins_at_finalisation` (front, feature seam) | Same transfer with the spawn failure retained; worker teardown confirmed | Spawn failure absent |
| Runner side of both | `native_queued_term_stays_pending_through_a_failed_started_or_spawn` | The failed launch consumed nothing; the signal joins only when the parent restores its mask | — |

**One product defect, repaired.** The queued-TERM F05 rows (pending query and
TERM membership) showed that, after a failed final pending query, the incumbent
finalisation still restored the mask. The queued TERM was then delivered under
its restored default action and killed the launcher before the original failure
or any secondary report was written: exit by SIGTERM, empty diagnostics, and
byte-for-byte what the ignore-the-failure subject also did, so the control could
not discriminate until the product was corrected. Unknown pending state now keeps
the handled signals blocked through settlement and the launcher ends on the
reported failure (“pending cancellation state unknown; handled signals retained
blocked”). It claims no cancellation. The unit test
`an_unknown_pending_set_keeps_the_mask_blocked_through_the_reports` fails
without the correction, and the six queued rows now differ from their broken
subject. The pre-repair observation above is the only baseline failure claimed.

**Already-correct baselines.** No other row was run against a pre-repair
subject: the sites did not exist before this leaf, and the rows establish that
the current product fails closed at each. No baseline failure is invented for
them. Each still has a failing broken subject, as tabulated. Immutable
baseline/broken/repaired receipts, per-row digests and the one-writer record are
the campaign's (k38).

**Limits.** The paired F05 Started + terminal-recovery case is asserted at the
runner's API, where the C4 report seam is; the front forwards the runner error's
text whole, as the F05 Started rows show. The runner-side rows run in a role
process owned by the shared controller; the front-side rows run in the private
real-main fixture. On Darwin the group-stop EPERM reports for already-reaped
zombie groups appear in these rows' diagnostics and are the approved retained
benign reports. Protected checks: `task test:native-entry` (35 front, 11 runner,
2 Started API checks) and `task check` (326 run, 45 skipped) passed; R01
reconciled by identity: all 310 prior identities present with unchanged ignored
flags, 61 new identities of which 15 are this leaf's.

## Settlement controls (k37)

`native-entry-settlement-controls-k37` finished the settlement family: S02's
bound and identity claims, F03/S03's report and deadline claims, S04's seven
unknown-status recovery rows and both S05 first-HUP-then-TERM rows, plus the
F03 broken subjects. Everything is a development control on aarch64-apple-darwin,
not the immutable campaign; it qualifies neither Linux nor Rust 1.85 and no
family is accepted. The 32 S identities (8/7/8/7/2) and the 153-row manifest are
unchanged.

**Judged by claims, not by the instrument.** Each row's observations are kept as
facts (physical operation, substituted report, elapsed time, ownership state,
events, foreground and modes) and judged by a list of *named claims* taken from
the contract. The same list judges the repaired product and the row's one named
broken subject, and a broken subject counts as detected only when it violates
the claim it names. A broken run that fails for any other reason (a timeout, a
crash, a different claim) is not a detection. Every fixture's own cleanup is
asserted on every row, repaired or broken.

| Subject | One defect each | Seam (all `cfg(test)`; none reaches an installed build) |
|---|---|---|
| report | the injected failure is ignored, so its report never exists | existing `broken_report`, worker `drop_wait_reports`, `reader_reports`, terminal `DROP_REPORTS` |
| deadline | the bounded observation, wait or worker poll may outlive its absolute bound by 1 s | `deadline_slack` at the runner observation and reap loops and the worker poll loop |
| identity | ECHILD no longer releases ownership or disarms numeric signalling | `identity_guard_broken` at `exited`, `reap`, `stop_target` and the worker poll |
| takeover / uncertainty / handback / restore | unknown status borrows the signal-death rule; the unconfirmed foreground and modes reports are never made; the recorded child's group is not handed back; saved modes are not restored | `terminal_tests::break_*` in `Lease::reclaim` |
| overwrite | a later consumed signal replaces the first | the existing `first_latch_broken("error")` |

The deadline rows assert both bounds: a repaired row must run its whole
2-second bound (a row that returned early exercised nothing) and finish inside
2.5 seconds, and the 1-second slack subject must overrun it. ECHILD rows must
settle at once (under 1 second) with ownership reported unresolved and no Reaped,
and the identity subject must fail by reaping, or by waiting out the deadline.
After release a later numeric stop on the released identity is requested; the
control records an attempt (it never performs one) only when the guard is broken.

**S04 and S05 on a PTY.** One role runs unknown status after a failed Started
check on its own PTY. Physical operations run and only the error rows substitute
a report afterwards. The fixture stands in for a child that left the terminal
raw, so mode restoration is observable, and it settles its own foreground, modes
and any foreign holder (killed and reaped by the fixture, bounded) before the
claims are judged, so a violated claim never leaves the PTY dirty. S05 on the
terminal acknowledges the first consumption by the runner latch being empty at
the moment TERM is raised during the recovery reports; S05 on the runner uses
the shared controller, acknowledging HUP's consumption at the settlement barrier
(latch empty, consumed=HUP), delivering TERM, and acknowledging its latching by
the handler's own run counter before release.

**Owners moved onto the controller.** The worker fixture driver (F03/S03 and
every older admission case) and the C4/terminal `pty_case` no longer run their
own 8-second wait and group kill or blocking `wait`. Each adopts its fixture in
the shared 30/5/5 controller (`finish`, or `abort` on the cooperative-abort
path) and a non-clean settlement is a failed receipt. One small accessor was
added to the controller, `poll`, a non-blocking exit check that records the
status once so a later settlement never signals a reaped identity.

**One instrument defect found and corrected.** E05's direct and terminal-INT
rows read the launcher's latch in the acknowledgement probe immediately after
the signal was sent. Delivery is asynchronous, so under the full-workspace load
of `task check` the probe once read an empty latch for a row whose launcher did
receive the INT (the launch outcome was `Interrupted(INT)`); the same row passed
alone. Where the launcher is the target the controller now polls acknowledged
probes until the handler's own run counter is seen or a 3-second window ends, so
an absence is a fact. Rows where the launcher must not receive the signal are
unchanged. The earlier red check log is the failing observation; no baseline
failure is claimed otherwise.

**Reconciliation.** Each manifest identity, its test, its named broken subject
and the claim that subject must fail. Rows sharing a test are separate
iterations of that test and each is asserted (S01's eight and S02's seven).
Every identity below is present in `cargo nextest list` at this commit.

| Manifest identity | Test | Named broken subject | Claim it must fail |
|---|---|---|---|
| `F03__operation=reader-block__mode=omit-EIO` | `native_f03_block_omitted` | reader failure ignored | no-managed-start |
| `F03__operation=reader-block__mode=perform-report-EIO` | `native_f03_block_performed` | reader failure ignored | no-managed-start |
| `F03__operation=first-create__mode=omit-EAGAIN` | `native_f03_first_create` | reader failure ignored | no-managed-start |
| `F03__operation=second-create__mode=omit-EAGAIN` | `native_f03_second_create` | reader failure ignored | no-managed-start |
| `F03__operation=creator-restore__mode=omit-EIO` | `native_f03_restore_omitted` | reader failure ignored | no-managed-start |
| `F03__operation=creator-restore__mode=perform-report-EIO` | `native_f03_restore_performed` | reader failure ignored | no-managed-start |
| `S01__operation=group-first__mode=omit-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 1, omitted) | stop-report-retained attempt=1 |
| `S01__operation=group-first__mode=perform-report-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 1, performed) | stop-report-retained attempt=1 |
| `S01__operation=PID__mode=omit-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 2, omitted) | stop-report-retained attempt=2 |
| `S01__operation=PID__mode=perform-report-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 2, performed) | stop-report-retained attempt=2 |
| `S01__operation=group-repeat-first__mode=omit-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 3, omitted) | stop-report-retained attempt=3 |
| `S01__operation=group-repeat-first__mode=perform-report-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 3, performed) | stop-report-retained attempt=3 |
| `S01__operation=group-repeat-second__mode=omit-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 4, omitted) | stop-report-retained attempt=4 |
| `S01__operation=group-repeat-second__mode=perform-report-EPERM` | `native_s01_retains_each_stop_report_and_distinguishes_physical_operations` | report discarded (attempt 4, performed) | stop-report-retained attempt=4 |
| `S02__operation=observation-EIO-once` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | report ignored | observation-error-reported |
| `S02__operation=observation-EINTR-until-deadline` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | bound outlived by 1 s | absolute-deadline |
| `S02__operation=observation-ECHILD` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | ECHILD keeps identity | identity-released-without-reap |
| `S02__operation=wait-EIO-once` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | report ignored | wait-error-reported |
| `S02__operation=wait-EINTR-until-deadline` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | bound outlived by 1 s | absolute-deadline |
| `S02__operation=wait-ECHILD` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | ECHILD keeps identity | loss-does-not-wait-out-deadline |
| `S02__operation=always-running-until-deadline` | `native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership` | bound outlived by 1 s | absolute-deadline |
| `S03__operation=PID-stop-EPERM__mode=omit__primary=second-reader-EAGAIN` | `native_s03_stop_omitted` | reports discarded | stop-report-retained |
| `S03__operation=PID-stop-EPERM__mode=perform-report__primary=second-reader-EAGAIN` | `native_s03_stop_performed` | reports discarded | stop-report-retained |
| `S03__operation=PID-kill-EPERM__mode=omit__primary=second-reader-EAGAIN` | `native_s03_kill_omitted` | reports discarded | kill-report-retained |
| `S03__operation=PID-kill-EPERM__mode=perform-report__primary=second-reader-EAGAIN` | `native_s03_kill_performed` | reports discarded | kill-report-retained |
| `S03__operation=try-wait-EIO__mode=report__primary=second-reader-EAGAIN` | `native_s03_poll_eio` | reports discarded | wait-report-retained |
| `S03__operation=EINTR-until-deadline__mode=report__primary=second-reader-EAGAIN` | `native_s03_poll_eintr` | bound outlived by 1 s | absolute-deadline |
| `S03__operation=ECHILD__mode=report__primary=second-reader-EAGAIN` | `native_s03_poll_echild` | ECHILD keeps identity | signalled after reported ECHILD |
| `S03__operation=always-running__mode=report__primary=second-reader-EAGAIN` | `native_s03_running` | bound outlived by 1 s | absolute-deadline |
| `S04__recovery=foreground-own` | `native_s04_foreground_own` | mode restoration skipped | saved-modes-restored |
| `S04__recovery=foreground-child` | `native_s04_foreground_child` | handback skipped | recorded-child-group-handed-back |
| `S04__recovery=foreground-unrelated` | `native_s04_foreground_unrelated` | takeover (a); uncertainty dropped (b) | (a) foreign-foreground-preserved; (b) unconfirmed-foreground-reported |
| `S04__recovery=foreground-query-EIO` | `native_s04_foreground_query_eio` | report ignored | query-report-retained |
| `S04__recovery=handback-EIO` | `native_s04_handback_eio` | report ignored | handback-report-retained |
| `S04__recovery=modes-EIO` | `native_s04_modes_eio` | report ignored | modes-report-retained |
| `S04__recovery=SIGTTOU-restore-EIO` | `native_s04_sigttou_restore_eio` | report ignored | sigttou-report-retained |
| `S05__signals=HUP-then-TERM__secondary=S02-always-running` | `native_s05_runner_hup_then_term_with_always_running_keeps_the_first_and_the_report` | later consumption replaces the first | consumed names TERM, not HUP |
| `S05__signals=HUP-then-TERM__secondary=S04-modes-EIO` | `native_s05_terminal_hup_then_term_keeps_the_first_signal_and_the_mode_report` | mode report ignored | modes-report-retained |

**Already-correct baselines.** None of these rows was run against a pre-repair
subject: the S sites are bounded by the k28 repair itself, and these rows
establish that the repaired product holds each claim and that its single named
defect does not. No baseline failure is invented. Immutable
baseline/broken/repaired receipts, digests and the one-writer record remain
k38's.

**Limits.** The S02/S03 ECHILD and always-running rows substitute the report and
retain the physical reservation; they never claim a physical ECHILD or a stuck
kernel, and the independent fixture owner reaps afterwards. Polling bounds
guard user-space retries, not scheduler or kernel stalls. The paired F05 Started
and F06 rows remain asserted at the runner's API (k36). The terminal rows exercise
unknown status through the runner's own recovery, not the front. Protected checks:
`task test:native-entry` (35 front, 20 runner, 2 Started API checks) and
`task check` (336 run, 46 skipped) passed; R01 reconciled by identity: all 310
prior identities present with unchanged ignored flags, 72 new identities in all.
No tree-owned instrument review was cut: every broken subject was observed
failing the claim it names, the seams are `cfg(test)`-only and equivalent to the
incumbent in non-test builds, and the reserved RC≤5 stays unspent.

## Campaign tooling and row identity (k40)

`native-entry-campaign-tooling-k40` built the measurement instrument that the
immutable campaign needs, and nothing it does not. It takes no immutable
measurement and finds no product defect. Before it, the instruments judged whole
families inside single test functions and carried no manifest identity, so a
per-row receipt did not exist to freeze.

**Row records.** Every native instrument now files one record per judged
(row, subject) at the point it judges it, through `native_barrier::row` (test
builds only; inert unless a private `K40_ROW_DIR` names a directory). A record is
`<id>@<subject>.json`, created exclusively with mode 0444, so a second record for
one row and subject panics instead of replacing the first, and an unsafe name is
refused. It carries the manifest id, the subject, `conforms`, the judging test's
name and process and a bounded note. Production code and installed builds are
unchanged.

| Subject | Meaning of `conforms` |
|---|---|
| `repaired` (also `repaired:front`, `repaired:runner`) | The product as built holds every claim the row's assertions make. Two names exist only where one row has an end-to-end front record and a runner-level record |
| `broken:<name>` | The named defect was *not* detected. It is `false` only when the broken subject violated the claim it names; a broken run that fails any other way is recorded `true`, because that is not a detection |

Records are made before the assertion that would panic, so a violated claim is a
recorded verdict, and the test's own failure follows. Where a test already made
many assertions about one subject they are run under a recording wrapper rather
than rewritten; the assertions are unchanged.

**The plan.** [`native-entry-campaign-plan.json`](native-entry-campaign-plan.json)
says which subjects each manifest row must have and with which verdict. A row is
resolved by exact id, then by the first matching pattern of its family, then by
the family default, and the 153 rows expand to 327 expected records. 143 rows
have at least one failing broken subject. Ten do not, each with its reason:
E02 `pending=no` (three negative controls: the entry-blocked-pending mutant is not
discriminating without a pending signal), E03 (two rows: Darwin exec clears the
private flag, so skipping the worker repair still confirms both waits and the E04
worker row is the discriminator), E04 raw-executable-entry (an observation of
whether the flag reaches the entry, with the measured flag state in its note),
E05 (three positive rows; only PTY-VINTR after handoff has a defined
discriminator) and F07's real-signal-death control. The plan is a subject of
every campaign: it is digested before and after like the source.

**The driver.** `scripts/native-campaign.sh` (Taskfile `test:native-campaign`)
freezes and digests every subject before and after a run, runs the three finite
Taskfile selections (`native-entry:front`, `native-entry:runner`,
`native-entry:start`, which `test:native-entry` is now composed of) under a
separate nextest profile each so their junit files cannot overwrite each other,
and reconciles item by item. Its subjects are the source tree (`Cargo.*`,
Taskfile, `.cargo`, `.config`, `scripts`, `crates` without the worker's
dependency tree), the manifest, the plan, the transition design and the
completion spec, the two built private executables and every workspace test
binary; 123 digests in the development run. The measured domain
(toolchain, nextest, task, kernel, libc crate, parent revision) is written beside
them. The campaign directory must not exist, the driver is the only writer of
every file in it except the write-once instrument records, a second receipt
writer is refused, and a moved, appeared or vanished subject fails the run.

It reconciles in both directions and reports rather than totals: each row's
expected subjects against the observed ones (missing, unexpected, wrong
verdict), manifest identities with no record, record identities outside the
manifest, every failed test, and every record whose judging test did not appear
as passed in the junit. `r01` reconciles the 310 protected identities against a
nextest listing one by one, by presence and ignored flag, counting new identities
apart so they can never stand in for a missing one.

**Controls, each seen to fail.** `task test:native-campaign-controls` runs the
driver against deliberately wrong inputs and requires the named rejection:
a missing record, an identity outside the manifest, a broken subject that
conformed, a record the plan does not expect, a negative control that violated,
a second writer for one receipt, a failed test, records from a test that never
ran, a missing protected identity masked by a new one, a changed ignored flag, and
a changed, vanished and appeared digest. The same run asserts that the real plan
resolves all 153 real rows. Two unit tests assert that the recorder is
write-once, read-only and refuses names that could leave its directory.

Two defects in the instrument were found by these controls and corrected before
any run was credited: a `jq` selector that read the wrong `.` inside a
`select(...)` and, separately, a second-writer refusal that exited only a pipe's
subshell inside an `if` and so was lost. Neither reached a measurement.

**Development run.** `task test:native-campaign DIR=...` on aarch64-apple-darwin
built the fixtures, digested 123 subjects (unchanged after), ran 59 tests
(35 front, 22 runner, two Started API checks), all passed, and reconciled
**153 of 153** rows against 327 records, with no unexpected record, no unknown
identity and no record from a test that did not run. `r01` found all 310
protected identities with unchanged ignored flags and 74 new identities.
`task check` passed 338 run/46 skipped. The
[archived summary](evidence/native-entry-development/k40-development-campaign/)
holds the reconciliation, the per-row table, the test list, the digests and the
domain. **This is a development run**: the instruments and recorder changed
between runs, the baseline subject does not exist, and nothing here is an
immutable per-case baseline/broken/repaired receipt.

**What this does not establish.** There is still no incumbent subject, so every
baseline, and every claim that a row fails before the repair, is unmeasured.
The 143 failing broken subjects are deliberate mutations of the repaired code,
not the incumbent product. The tooling qualifies neither GNU/Linux nor Rust 1.85.

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
2. F02's six watching faults, F04's 14 handler faults, F05's 20 finalisation
   and Started-mask faults, F06's six pre-exec faults, the four paired cases and
   the two pending-TERM variants now have development controls (k36, above); they
   are not immutable receipts.
3. S02's bound/identity claims, F03/S03's report/deadline claims, S04's seven
   rows and both S05 rows now have development controls with a named failing
   broken subject each (k37, above); they are not immutable receipts. Do not
   substitute report rewriting for an irreversible physical operation.
4. Qualify every native controller: 30 seconds total, acknowledgements within
   five seconds, last five seconds reserved for cleanup. The shared barrier
   controller has its three failing controls (k34); the worker fixture driver and
   the C4/terminal `pty_case` now run under it (k37). Preserve and boundedly
   settle every owned child, holder, pipe and PTY on failure as well as success.
   A fixture child that the role itself owns (a foreign holder, a C4 child in
   another group) is settled by the role's own bounded cleanup, and only the
   cooperative abort or the controller's group kill reaches it on a failed case.
5. Finish all instruments before measurement. Freeze source, binaries, helpers,
   manifests, reference expectations, exclusions and measured platform domain;
   digest every read subject before/after. Produce one immutable receipt per
   baseline/broken/repaired case with one writer. Keep actual operations,
   substituted reports, product settlement and fixture cleanup distinct. Rerun
   affected rows after edits and reconcile identities individually. The row
   recorders, plan, digest driver and its failing controls now exist (k40,
   above); the incumbent subject with its baseline records and the frozen
   campaign run remain.
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

`native-entry-fault-families-funding-k39` repriced the remainder after k34 and
k35 from their actual charges. K35's rows cost about 2.8 P each, against the
0.5–0.8 P assumed when k31 was decomposed, and F02, F04 and F06 have no fault
seams yet. The remaining price is P≤112/E≤84/RC≤13. Projections: fault families
P≤58/E≤31/RC≤2, settlement controls P≤27/E≤18/RC≤1, campaign and receipts
P≤27/E≤35/RC≤2, the funding session RC≤3 and an optional RC≤5 instrument review.
The human's answer was **“Renew full shortfall (Recommended)”**. Approved caps
are D≤135/RC≤87/P≤402/E≤174/X=0, with zero production disruption and the D≤20
closure reserve intact. After k39's RC≤3, balances are D≤20 exclusively phase
closure and RC≤10/P≤112/E≤84/X=0 shared by the remaining children. Review
integration, support qualification and narrower acceptance are not funded.
