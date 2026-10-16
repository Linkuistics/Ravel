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

**Current measured status, per target:** see
[GNU/Linux immutable campaign (k52)](#gnulinux-immutable-campaign-k52). Sections
below that describe a leaf's own scope are historical for that leaf.

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

## Incumbent subject and front baseline (k44)

`native-entry-incumbent-front-k44`, the first child of the incumbent-baseline
node (k42), built the incumbent as its own frozen tree, added the `baseline`
verdict class to the plan and driver, and filed a classified baseline for the 46
rows the front's instruments host. It takes no immutable measurement, changes no
production code and finds no product defect.

**The incumbent subject.** `scripts/native-incumbent.sh` exports git revision
`0515419edd29` (the parent of the first repair commit) into
`target/native-incumbent/tree`, outside the repaired tree and its subjects, and
adds only:

| Addition | What it is |
|---|---|
| `seams.patch` | Private `cfg(test)` call sites in five product files (`main`, `run`, `cancellation`, `signal_state`, `worker`) and one nextest profile. Each places the repaired instrument's hook at the point in the incumbent's own flow that corresponds to it. None changes behaviour in a non-test build. |
| the repaired instruments, verbatim | `native_entry_tests.rs` and its `faults.rs`, `native_barrier.rs` and the manifest, copied from the repaired tree. They are never edited for the incumbent. |
| `instruments.patch` | Three adaptations to the copied front instrument: the module declaration for the baseline instruments, the `operation_reports` field that `Ended` lacks in the incumbent, and one fact string kept so the baseline can record it. |
| `files/` | The baseline instruments (`native_entry_tests/baseline.rs`, which reads every row's identity and parameters from the frozen manifest) and the observation half of the worker reader receipts. |

The compiled worker is the repaired tree's, because the incumbent's worker
sources, build script and compile script are identical and the build asserts
that from the two trees. A hunk that no longer applies is refused
(`--fuzz=0`). The source tree is reproduced exactly from the recipe; the debug
binaries are not bit-identical across rebuilds, so they are digested within each
run, before and after, like every other built subject. Twelve files differ from
git's export (five patched product files, the nextest profile, three instrument
sources, the manifest copy and the two files under `files/`), and
`task test:native-incumbent-controls` states that count.

**Placement is a judgement.** The repaired barriers sit where the repaired front
finalises. The incumbent has no common finalisation: its choice drops the
selection's handlers on a failed selection and the front then settles that failure
directly, so the "before finalise" barriers for a refused selection sit just
before that settlement. A signal delivered before the choice drops its handlers
would be latched; the incumbent's window is the gap after that, which the repaired
design closes. The rows below are classified at that placement, and the note
on each record carries the facts.

**The `baseline` class.** The plan states, per row, `observed` or
`rejected-premise` with a reason and the missing mechanism; a row with no entry is
unclassified. An observed row needs one `baseline` record, whose raw verdict the
driver classifies as `fails` (a claim was violated) or `already-correct`; it is
never required to fail. A rejected premise carries its reason and must have no
record. Failing controls (`task test:native-campaign-controls`) reject a missing
baseline record, an unclassified row, a rejected premise without a reason, an
unknown class, a record with no verdict and a rejected premise that has a
record, and they require both observed outcomes to be accepted.

**Harness controls.** An "already correct" reading is credited only because the
same instruments were seen to fail the incumbent when it was made wrong: a
managed child handed a wrong entry state fails E01's claims, and readers created
without the shield fail E08's eligibility receipt. Both run in
`native_baseline_control_*` and file no row.

**Rows hosted and their classification** (development run, aarch64-apple-darwin,
not an immutable receipt):

| Family | Rows | Already correct | Fails | Facts behind the failures |
|---|---:|---:|---:|---|
| E01 entry state | 16 | 16 | 0 | |
| E02 blocked pending | 6 | 6 | 0 | |
| E03 SIGCHLD at entry | 2 | 2 | 0 | The ignore row holds on Darwin because exec leaves no no-child-wait flag (raw flags 0), as the repaired tree's own E03 note records. |
| E04 flag boundaries | 3 | 1 | 2 | The worker row: the worker's reap is unconfirmed (`No child processes`, ECHILD) and its teardown fails. The managed row: the runner cannot wait (`cannot wait on the launched child: No child processes`). The raw row is an observation: flags 0, not inherited, so it is recorded as holding. |
| E06 boundary joins | 15 | 12 | 3 | The three refusal-before-finalise rows: the process is ended by the delivered signal at the boundary, before the front reports the original refusal or any cancellation. |
| E08 readers | 4 | 2 | 2 | The two policy-refusal rows, for the same reason; both success rows hold, with shielded readers, a restored creating mask and a bounded collection. |
| **Total** | **46** | **39** | **7** | |

The incumbent already holds E01, E02, E03's ignore row and E06/E08's success and
reaped/ENOENT/Started boundaries. It fails exactly where its handlers are absent
(after a refused selection) or where its watching repair does not cover the
no-child-wait flag. 107 rows remain unclassified and visible in the driver:
E05, E07 and S01–S05 (k45) and F01–F07, paired and cancellation (k46).

**What this does not establish.** There are no immutable per-case baseline
receipts; the campaign run is k43's. The baseline qualifies neither GNU/Linux nor
Rust 1.85. A seam is an observation or a call site, never a mechanism the
incumbent lacks: rows whose premise needs a missing mechanism are rejected
premises in k45 and k46, with the mechanism named.

## Runner, worker and terminal baseline (k45)

`native-entry-incumbent-runner-k45`, the second child of the incumbent-baseline
node (k42), classified the 40 runner-, worker- and terminal-level rows: E05 (4),
E07 (4), S01 (8), S02 (7), S03 (8), S04 (7) and S05 (2). Thirty are observed
from a real incumbent run and ten are rejected premises, each naming the
operation or mechanism the incumbent lacks. It takes no immutable measurement,
changes no production code and finds no product defect.

**What was hosted, and where the hooks sit.** The runner and terminal
instruments are `cfg(test)` code in the `keyed-launch` library and the worker's
in the ravel binary, so `native-incumbent.sh test` runs both in one nextest
invocation and the driver's one junit file names every test. The new
`files/.../run/baseline_tests.rs` ports the repaired E05 and E07 fixture roles,
controller flow and named claims (`E07::conforms`, `E05::axes`) and adds S02;
`terminal_tests.rs` gains the repaired S04/S05 role and claims; the worker
overlay gains S03. Every hook is a call site at an operation the incumbent
performs:

| Hook | Where in the incumbent's own flow |
|---|---|
| handler-run count | after the handler's latch store, so a controller that sees the count has seen the latch |
| `before_spawn` | immediately before `command.spawn()`, after the latch is cleared: E05's before-handoff boundary |
| `runner_active`, `first_consumed` | the supervision loop's first iteration after `lend`, and immediately after `take_interrupt` returned a signal |
| observation report | after the physical `waitid(WNOWAIT)`; E07's runner-wait-error, S02's observation rows and S04's unknown status substitute an errno there |
| reap report | in place of the physical wait, which is omitted so the reservation stays with the fixture, which reaps it afterwards (never the product); S02's wait rows and S04 |
| signal-attempt count | `kill()` and `kill_group`, counted only after a reported ECHILD |
| E05 after-handoff barrier | the `Started` notification, the incumbent's only post-spawn point and the place the repaired start check runs |
| before-recovery | S04's own-foreground precondition, placed at the recovery boundary because the incumbent lends again on every poll tick |
| second-reader creation, stop, kill, poll | S03's primary at `thread::spawn`, and the secondary fault at the teardown's own stop, kill and `try_wait` |

None adds a mechanism. `first_wins` latch and `invented` cancellation are
harness controls only: they make the incumbent right or wrong in the one way a
row names, to show the instrument can read both ways.

**Observed rows** (development run, aarch64-apple-darwin, not an immutable
receipt):

| Family | Rows | Already correct | Fails | Facts behind the failures |
|---|---:|---:|---:|---|
| E05 direct/terminal INT | 4 | 4 | 0 | Direct INT is the launcher's before and after handoff; terminal VINTR is the launcher's before handoff and the child's after, with the child's own status and no launcher cancellation invented from it. |
| E07 first consumed | 4 | 0 | 4 | The handler overwrites the latch with a plain store: after TERM then HUP the acknowledged latch is HUP (HUP then INT: INT) before the runner consumed anything, so the launch names the later signal in both the active and runner-wait-error variants. |
| S02 observation and wait | 6 | 2 | 4 | See below. |
| S03 second reader | 8 | 0 | 8 | The incumbent creates readers with `thread::spawn`, so the refused second creation is a panic (exit 101) with no failure reported; its teardown never runs, so the armed secondary fault is never reached, and the worker it spawned is left running (observed alive after the role died, then ended by the fixture). The claims that read the reported failure, and the one the row names, are unmet; `no-managed-start` and an empty result hold. |
| S04 unknown-status recovery | 7 | 6 | 1 | Foreground-unrelated: the foreign holder is preserved, but the incumbent gates its unconfirmed-foreground report on a failed spawn alone and treats a missing status like an ordinary exit, so `unconfirmed-foreground-reported` and `unconfirmed-modes-reported` fail while `foreign-foreground-preserved` and `saved-modes-not-applied-to-a-foreign-holder` hold. The other six hold, including retained query, handback, modes and SIGTTOU reports. |
| S05 terminal (S04 modes) | 1 | 1 | 0 | HUP is consumed before the second signal, TERM stays pending and unconsumed, and the mode report is retained. |

**S02: what the incumbent does with a substituted report.** The repaired rows
reach these faults through a failed Started check and a bounded settlement,
neither of which the incumbent has, but the incumbent performs both operations
(the `waitid(WNOWAIT)` observation and the reap) and each has an error path:
an observation failure sends SIGKILL and reaps, a reap failure ends the launch
with no status. The report is substituted there and the incumbent's response is
judged by the repaired row's named claims, with the repaired text restated by
meaning (the failed operation and its errno are named):

| Row | Result | Claims met or unmet |
|---|---|---|
| observation EIO once | already correct | The failure is reported and the child is SIGKILLed and reaped: `[Started, Reaped]`. |
| observation EINTR | fails | The first EINTR is fatal and the child is killed at once, in 1.5 ms: `absolute-deadline`, `deadline-expiry-reported` and `retries-reported` are unmet (there is no retry window). |
| observation ECHILD | fails | After the reported ECHILD the incumbent still signals the numeric group and PID (`no-signal-after-release`). |
| wait EIO once | fails | There is no retry: no Reaped, an error with no status (`reaped-after-recovered-error`). |
| wait EINTR | fails | As observation EINTR: no retry window, so the three deadline claims are unmet. |
| wait ECHILD | already correct | The ECHILD is reported, nothing is signalled after it, no Reaped is invented, and it ends well inside a second. |

The substituted EINTR and ECHILD are reports at the operation, never physical
EINTR or ECHILD; the physical `waitid` has run, and a substituted reap report
omits the physical wait. Not judged, because the fact cannot exist:
`start-check-source-retained` (no start check) and `ownership-released` (the
incumbent has no reservation state to read).

**S04 and S05's terminal row.** The repaired role reaches unknown status
through a failed Started check. These rows reach `Lease::reclaim(None, false)`
the way the incumbent itself can: observation fails, SIGKILL, and the reap
fails, so recovery is called without a status. The terminal operations, the
foreground and the modes are the real ones on the role's own PTY, and a
substituted recovery report follows its physical operation as in the repaired
rows. `start-check-source-retained` has no counterpart. The own-foreground row
places its precondition at the recovery boundary, a fixture-placed state rather
than one the incumbent's loop reaches. An earlier draft of this section
substituted `None` after a successful reap; a review found that this
manufactured a combination (a known reap with unknown status) the incumbent
cannot reach, and the rows were rehosted on its real path.

**Rejected premises**, verified against source with a control (the same patterns
counted in both trees: 0 in the incumbent and 13/5/9/2 in the repaired `run.rs`
for `start_check`, `stop_reports`, `ERROR_SETTLEMENT`, `operation_reports`;
`thread::Builder` 0 and `thread::spawn` 1 in the incumbent's `worker.rs`, the
reverse in the repaired):

| Rows | Missing operation or mechanism |
|---|---|
| S01 (8) | Checked stops with a per-attempt report, and the fallible start check that leads to them. `kill`, `Process::signal` and `Process::kill_group` return `()` and ignore the failure on purpose, so the one claim each row names (the faulted attempt's retained report) reads a fact that cannot exist. This is recorded as the reason, not as a failure. |
| S02 always-running (1) | An absolute error-settlement deadline over a child that is merely still running. The incumbent enters no error path for one: its supervision loop polls with `WNOHANG` every 500 ms and is a documented stall the caller closes, so there is no settlement to observe and no deadline to read. |
| S05 runner (1) | The start check, and the same deadline over an always-running observation while a second signal arrives. |

**Harness controls.** Each is seen to read the other way, so the "already
correct" readings are not an instrument that reads true everywhere and the
failures are not one that reads false everywhere: the incumbent with a
first-wins latch conforms E07's active and runner-wait-error rows while the
unmodified incumbent does not; the incumbent that reports the child's
terminal-INT death as its own cancellation fails E05's `attributed` axis; the
incumbent with its substituted report discarded fails S04's
`modes-report-retained` and `handback-report-retained`; and with S03's primary
absent the same armed secondary is reached by the incumbent's own teardown, so
"never reached" is a property of the primary and not of a hook that cannot
fire. They file no row.

**Result.** 40 of 40 classified: 30 observed (14 already correct, 16 fails) and
10 rejected premises. The development campaign (90 tests: 59 repaired, 31
incumbent) digested 321 subjects, unchanged after, and all 153 repaired
identities still reconcile. Across the node 24 rows fail, 52 are already
correct, 10 are rejected premises and 67 stay unclassified for k46.
`task test:native-incumbent-controls` now states 17 files differing from git's
export. F03 (k46) shares S03's primary and will observe the same panic at the
same site.

**What this does not establish.** The same limits as k44: no immutable
receipts, no GNU/Linux or Rust 1.85 qualification. The ten rejected premises
are an account of mechanisms absent from the pre-repair product, not a claim
that it behaves correctly under those faults. S02 and S03 failures are the
incumbent's responses to substituted reports and to a documented std panic, not
physical EINTR, ECHILD or thread exhaustion.

## Fault-family baseline (k46)

`native-entry-incumbent-faults-k46`, the third and last child of the
incumbent-baseline node (k42), classified the 67 fault rows: F01 (3), F02 (6),
F03 (6), F04 (14), F05 (20), F06 (6), F07 (6), the four paired failures and the
two queued-TERM variants. Forty-seven are observed from a real incumbent run (43
fail, 4 are already correct) and 20 are rejected premises, each naming the
operation the incumbent never performs and each checked by a control on the same
fixture. (k46 recorded 45 and 22; the [integration of its review
(k48)](#instrument-integration-k48) hosted the two F07 TERM-default rows.) It
takes no immutable measurement and changes no repaired production code; the
incumbent build gains seams, and one off-by-default feature.

**Where the hooks sit.** Every hook is a call at an operation the incumbent
already performs, with the report substituted after the physical operation (or
the operation omitted), through the same `faults::{query, mutation_c,
mutation_r}` and `native_hooks` sites the repaired product has. None adds a
check, a report or a repair.

| Rows | Operation, where the incumbent performs it |
|---|---|
| F01 | The initializer's mask query, TERM action query and TERM membership read. The initializer ran before `main`, so the row runs the incumbent's own `record()` once more with that one report replaced, keeping the real record for every signal but TERM. A failed mask query leaves the record unpublished (its checked path); the other two read as "not ignored" and "not blocked" (its unchecked path). |
| F03 | `shielded`'s block and restore (results unread) and the two `thread::spawn` calls (a panic on refusal, as in S03). |
| F04 selection and its final restore | `Handlers::install`'s query and installs (checked, reported as `cancellation_unavailable`) and `Handlers::drop`'s restore (unchecked). |
| F04 runner, F02 managed query | `install_termination_handler`'s `disposition` query and `signal()` installs, and `restore_child_watching`'s query: all unchecked. |
| F05 Started, F05 final restore | `restore_mask_at("Started-parent-restore")`, which the front swallows with `let _`, and `restore_mask_at("final-mask-restore")`, which is collected as a cleanup error. |
| F06 | `EntrySignals::reinstate` in the forked child: `sigemptyset` and the TERM `sigaction` (unchecked) and the final `pthread_sigmask` (the one result it checks). Reachable from the front fixture through the same off-by-default `native-faults` feature the repaired runner has, which the incumbent's private fixture build now also enables; no installed build does. |
| F07 | The handlers' `Drop` restore of the TERM entry action (the default action, in the default-entry state), then `reraise`'s unblock (`restore_mask`, ignored) and `raise` (ignored). |

**Observed rows** (development run, aarch64-apple-darwin, not an immutable
receipt). The named claims are the repaired rows' own; the table gives what the
incumbent met and did not.

| Rows | Already correct | Fails | What the incumbent did |
|---|---:|---:|---|
| F01 mask query | 1 | 0 | The unpublished record is refused as `signal_state_unavailable`, with no worker and no managed child. |
| F01 TERM query, TERM membership | 0 | 2 | The failure is invisible: the record is published, the worker and the managed child run, the launch succeeds. |
| F02 managed query | 0 | 1 | The report is ignored and the child admitted with no-child-wait still set, so its wait fails with ECHILD (the failure E04's managed row records). |
| F03 block, restore (4) | 0 | 4 | Neither mask result is read: admission proceeds, the managed command starts, and the launch ends with `session_result_missing`. No reader-admission report, mask failure or worker identity appears. `no-result-written` holds. |
| F03 first, second create (2) | 0 | 2 | The same panic as S03 (exit 101): no failure is reported, the worker is left running (observed, then ended by the fixture), `no-managed-start` and an empty result hold. |
| F04 selection query and installs (5) | 0 | 5 | Refused as `cancellation_unavailable` with the EIO text, before any worker or child, zero bytes: those claims hold. Unmet in every row: the handled signals are not blocked at the finalisation boundary; for the TERM rows the partial subset is not retained (the installed INT is released at once, and a performed TERM whose report failed is left installed unrecorded). |
| F04 INT final restore, selection and runner (4) | 0 | 4 | The restore is the unchecked `Drop`: nothing names the failed release, the launch that ran to completion writes its result (exit 0), and the mask is not held blocked. The physical owner afterwards is the operation's own. |
| F04 runner query and installs (5) | 0 | 5 | The report is ignored and the child admitted (`[Started, Reaped]`, exit 7); a performed install that was reported failed leaves the handler installed and the loop goes on to install the rest. |
| F05 final mask restore, selection-success and spawn-ENOENT (4) | 0 | 4 | Checked and collected: the original is kept, the result is not written, the exit is a failure, and the substituted errno's text is in the diagnostics. Unmet: the line does not name the failed transition (`ravel: Input/output error (os error 5)`). |
| F05 Started restore (2) | 0 | 2 | Swallowed: the child runs to its end and the launch writes its result with exit 0 (no `supervision_failed`, an `Ended` is produced). The mask is the operation's own. |
| F06 TERM restore, mask build (4) | 0 | 4 | Unchecked: the spawn succeeds, the child runs and exits 7. The child's own site record shows performed or omitted while it held the terminal; the terminal and modes come back. |
| F06 mask restore (2) | 2 | 0 | The one checked operation: the spawn fails with EIO as the actual errno, no Started, terminal and modes restored, no stray child. |
| F07 TERM default (2), unblock (2), raise omitted (1) | 0 | 5 | Nothing is reported: an omitted default restore leaves the incumbent's own handler installed so the raise ends by `exit(143)`; a performed default restore (report substituted) and a performed unblock end by the signal; an omitted unblock and an omitted raise end by `exit(143)`. No `cannot propagate` line. |
| F07 real signal death control | 1 | 0 | Raise ends the process by the real signal. |
| Paired F03 | 0 | 1 | The same incumbent path as S03's wait-EIO row: neither the creation failure nor the worker's wait report is written. |
| Paired F05 Started + recovery report, paired F06 + C4 report | 0 | 2 | The primary never exists (the Started restore and the TERM action restore are both swallowed), so `primary-retained` is unmet. The substituted handback report is the launch's only error and is retained: the incumbent returns it although the child ran to completion (F05 Started: `supervision_failed`, exit 5, nothing written). The F05 Started pair is one real front run: the front's own restore omitted and the runner's handback report substituted. |
| Queued TERM, F05 Started and F06 TERM action | 0 | 2 | The pending TERM is an actual signal under the parent's block and is joined at the tail (death by TERM, zero bytes, outcome names TERM, and for F06 the worker teardown confirmed). Unmet: the original Started or spawn failure was never raised (F05: the child ran to its end and an `Ended` was produced; F06: the child started and was then interrupted). |

**Judgement calls a reader should check.**

1. *F04's "handled signals blocked at finalisation".* All five selection rows
   and the four final-restore rows fail this claim, and for the query and
   INT-install rows it is the only unmet claim. The incumbent blocks the handled
   signals only at the pre-launch `block_and_check`, so at any point of a
   selection failure they are unblocked and its handlers are already dropped.
   The claim is in the repaired rows' contract (`retained subset protection`);
   the incumbent lacks that protection rather than mishandling the report.
2. *F05's label.* The repaired claim is the label `final mask release`; the
   incumbent reports the failure without naming the transition. This was
   restated as k45 restated S02's: the failed operation and its errno must be
   named. It is the only unmet claim in those four rows, so reading the claim
   as retention of the errno would make them already correct.
3. *F01 re-runs the initializer.* The substituted report is applied to the
   incumbent's own `record()` after `main` began; the real record is kept for
   every signal but TERM so SIGPIPE, which the Rust runtime has since ignored,
   is not handed to the managed child.
4. *The paired F05 Started row* is hosted end to end at the front, beside the
   cancellation rows and the single Started rows: the front's own
   `restore_mask_at("Started-parent-restore")` is the omitted primary and the
   runner's terminal-handback report is the substituted secondary, selected by
   a `native-faults`-only seam. k46 had hosted it at the runner with a fixture
   callback standing in for the front's swallowed restore, so the recorded
   failure was the callback's own choice; k48 replaced that (see below).
5. *F06's shared feature.* The front fixture reaches the forked child's sites
   through `native-faults`, which the incumbent's `keyed-launch` and `ravel`
   crates now declare as the repaired ones do.

**Rejected premises** (20), each verified on the same fixture: the selected
site is never reached (no receipt), while a hosted row on the same fixture
reaches its own.

| Rows | Missing operation or mechanism and its control |
|---|---|
| F02 worker (3) | A worker-side SIGCHLD query and default-action repair before the worker spawns: the incumbent's `worker.rs` never names SIGCHLD. The three sites are selected on the fixture, none is reached, and the worker is admitted (its ECHILD wait is E04's recorded failure). |
| F02 managed repair (2) | The no-child-wait repair of a default disposition: the incumbent repairs only `SIG_IGN`. On the runner role the repair site is reached for an ignored SIGCHLD and is not reached for default plus no-child-wait, and the query site is reached in both. |
| F05 final block, pending query, TERM membership (12: 3 outcomes x 4) | The common finalisation's signal snapshot. The incumbent's only block and pending query are the pre-launch `block_and_check`; each of the four sites selected on each outcome is never reached and the launch runs to its end. |
| F05 final mask restore on policy refusal (2), paired F05 refusal (1) | A mask restore on the selection-refusal path: the refusal settles without one (the handled signals were never blocked). The site is not reached on a refusal, which ends with its own exit 3, and is reached on the spawn-ENOENT path of the same fixture. |

**Harness controls**, none of which files a row: the F02 repair-site control
above; a checked child operation (mask restore) fails the spawn while the
unfaulted role runs the child and exits 7; the unfaulted F07 fixture dies by the
signal while the unblock-omitted one exits 143; the F07 custom-handler control;
and the rejected-premise control for F02 worker and F05. The hosted rows are
themselves positive controls: F01's mask query, both F06 mask-restore rows, the
F07 control and every held claim in the failing rows (the EIO text, the
refusal code, the owners, the site record) are readings the same instruments
return when the incumbent does it right.

**Result.** 67 of 67 classified: 47 observed (4 already correct, 43 fails) and
20 rejected premises. Across the node, with k44 and k45: 67 rows fail, 56 are
already correct and 30 are rejected premises, 153 of 153 by identity and none
unclassified or unobserved. (As k46 first recorded them, before k48's
integration, the figures were 45/22 and 65/56/32.) The development campaign
figures are in the k48 section. `task test:native-incumbent-controls` states 22
files differing from git's export.

**What this does not establish.** The same limits as k44 and k45: no immutable
receipts, no GNU/Linux or Rust 1.85 qualification, and a rejected premise is an
account of an operation the pre-repair product does not perform, not a claim that
it behaves correctly under that fault. The failures are the incumbent's responses
to substituted reports and to a documented std panic, not physical EIO, EAGAIN
or thread exhaustion.

## Instrument integration (k48)

`native-entry-incumbent-instrument-k48` integrated the committed findings of
`native-entry-incumbent-instrument-k47`, the fresh-context read of the whole
incumbent instrument before the frozen campaign. Each of its four findings was
reproduced from source before it was classified, and all four were real. It
takes no immutable measurement and changes no repaired production code or
repaired instrument; every change is to the incumbent's seams, its overlay and
the plan's baseline classification.

| Finding | What was wrong | Resolution |
|---|---|---|
| Worker fixture cleanup could kill a reused group (S03 ×8, F03 ×6, paired F03) | `s03_run` reaped the role, then probed and `SIGKILL`ed its numeric group; nothing reserved that group once the role was reaped | The role's exit is observed with `waitid(WEXITED|WNOHANG|WNOWAIT)`; the group is probed and killed while the unreaped leader still reserves it; the role is reaped last |
| The same cleanup's watchdog waited without a deadline | On timeout the group was killed and `child.wait()` then blocked, with the kill result ignored | The timeout settles through a bounded (2 s) unreaped wait, reports the kill errno and fails without waiting if the role is unconfirmed |
| The paired F05 Started row recorded the fixture's choice | The runner role's observer evaluated and discarded `Err(EIO)`; no front restore was performed and the report seam selected nothing | Hosted at the front: the real `restore_mask_at("Started-parent-restore")` omitted, and the runner's handback report substituted in the same run through a `native-faults`-only seam in `recovery_report`; the role's stand-in is removed |
| F07 TERM-default rows rejected an operation the incumbent performs | `Handlers::drop` reinstates the TERM entry action just before `reraise`, which is the default-action transition of the incumbent's propagation path; F04 hosts only INT's restore | Hosted at that restore; the two rows moved from rejected premise to observed |

**What the corrected instrument shows.** The development run of the corrected
incumbent was compared row by row with a run of the incumbent as k47 reviewed it
(same host, same revision, built from the committed patches). All 121 rows
observed before kept their verdict, and the 45 whose notes record their named
claims kept the same unmet and held lists. Other than the paired F05 Started
row and the two new ones, every row's note differed only in process IDs,
temporary paths and timings; the S03, F03 and paired F03 notes matched modulo
timings (the orphaned worker still survived where it did before), so the cleanup
corrections changed no observation. The paired F05 Started row is still
a fail on the same single unmet claim (`primary-retained`: the front swallows
its Started restore, so there is no primary to retain), now measured at the
real restore. The two TERM-default rows are new observations, both fails: with
the restore omitted the incumbent's handler is still installed and the process
ends by `exit(143)` with no report, and with the restore performed and its report
substituted the process ends by the real signal with no report (the repaired
claim is the exit code and the `cannot propagate` line).

**Controls added.** `native_baseline_control_s03_an_unreaped_role_reserves_its_group`
shows on this host that an ended, unreaped group leader keeps the group present
(a live member answers the probe), keeps it reserved once only the leader is
left (`EPERM`, never `ESRCH`) and releases it only at the reap;
`native_baseline_control_s03_settlement_is_bounded` shows the bounded wait
returning at its deadline for a role that has not ended. The F07 control now also
shows the omitted TERM-default restore changing the unfaulted ending from signal
death to `exit(143)`; the custom-handler control is kept and now states that it
locates the default action at the `Drop`, not at `reraise`.

**Result.** 153 of 153 by identity, none unclassified or unobserved: 67 rows
fail, 56 are already correct and 30 are rejected premises. The development
campaign (`task test:native-campaign`, aarch64-apple-darwin) executed 112 tests
(59 repaired, 53 incumbent), none failed, digested 327 subjects unchanged after,
and reconciled all 153 rows against 327 records with no baseline problem.
`task test:native-incumbent-controls` still states 22 files differing from git's
export, and `task test:native-campaign-controls` passes.

**What this does not establish.** The same limits as k44 to k46: no immutable
receipts and no GNU/Linux or Rust 1.85 qualification. The failures are the
incumbent's responses to substituted reports, not physical EIO. The k47 review
re-derived most rows from source and took the rest (E01–E05, E06's other twelve,
E08's two selection-success rows) from this note; k48 did not re-derive those
either, and reran them only as part of the development run above.

## Frozen campaign and R01 (k43)

`native-entry-campaign-run-k43` took the immutable measurement. It changed no
instrument, plan, manifest, script or production code after k48; the only edits
before the freeze were the four documents named below, made first because the
completion spec and the transition design are digested subjects. Everything it
measured was on **aarch64-apple-darwin, macOS 27.0.1 (26A434), rustc 1.99.0,
cargo-nextest 0.9.148, libc crate 0.2.186**. Nothing here qualifies GNU/Linux or
Rust 1.85.

**Subjects.** The driver (`task test:native-campaign`) built the private fixtures
and the frozen incumbent, digested 327 subjects (source tree, manifest, plan,
the two documents, the built fixture and worker, every workspace test binary and
the incumbent tree and binaries) before and after, and found every digest
unchanged. A second digest taken after the R01 tasks (below) is also identical,
so the six Taskfile tasks did not move a subject either. The incumbent is git
revision `0515419edd29` (tree and revision are in `domain.json`). The campaign
ran on parent revision `804699c370ab` with the four document edits as working-copy
changes, pinned by the work-item commit rather than a second commit.

**Measurement.** Front 35 tests, runner 22, Started API 2 (the repaired and
broken subjects), and 53 for the incumbent baseline: 112 executed, 112 passed,
none from outside the plan, no record from a test that did not run. The 153
manifest rows reconciled **153 of 153** item by item against 450 write-once
records (327 repaired/broken and 123 baseline). Each row has one receipt
(`receipts/`, mode 0444), with its expected subjects, observed verdicts and
baseline class. Every one of the 143 rows with a failing broken subject has that
subject observed to fail, and the ten rows without one carry the recorded reason
in the plan.

| Baseline class (incumbent `0515419edd29`) | Rows |
|---|---|
| Fails | 67 |
| Already correct | 56 |
| Rejected premise (the incumbent never performs the operation, or its fixture premise is absent) | 30 |

The repaired product conformed on all 153. No product defect was found in this
measurement and production code is unchanged. The 56 already-correct rows pass
before the repair; their repaired and broken-subject records stay explicit and
none is counted as repair evidence. The failures are the incumbent's response to
substituted reports (not physical EIO), as k44 to k48 recorded.

**R01.** The 310 protected identities and their ignored flags (20 ignored
fixtures) were listed first from the frozen manifest and listing
(`frozen-listing.json`: 310 of 310 preserved, 74 new identities counted apart).
Then `test:cli`, `test:session`, `test:runner`, `test:terminal-recovery`, `test`
and `check` each ran once, sequentially, and exited 0:

| Task | Tests | Protected identities in scope, run and passed |
|---|---|---|
| `test:cli` | 13 passed, 1 skipped | 13 of 13 non-ignored (14 in scope) |
| `test:session` | 241 passed, 29 skipped | 219 of 219 (227 in scope) |
| `test:runner` | 97 passed, 17 skipped | 71 of 71 (83 in scope) |
| `test:terminal-recovery` | 6 + 1 passed | the two private PTY selections only; no identity join |
| `test` | 338 passed, 46 skipped | 290 of 290 (310 in scope, 20 ignored fixtures) |
| `check` | formatting, shellcheck, worker build/probes/integrity/types, clippy, terminal recovery, then 338 passed, 46 skipped | 290 of 290 |

338 run plus 46 skipped is 384, which is the 310 protected identities and the 74
new ones. The join (`r01-join.sh`, kept beside the logs) matches each non-ignored
protected identity to its own `PASS` line in the task's log, so a count alone
never stands in for an identity. It was seen to reject its wrong inputs: a log
with one identity's `PASS` line removed (12 of 13), a log with an appended
`FAIL` line, an empty log, and a scope that matches nothing. The join is a
post-run reading of the logs; it is not part of the driver and was written after
the driver was frozen, so it is evidence of this run and not an instrument edit.

**Documentation.** The sweep for `Ended::operation_reports` (JSON
`operationReports`) and the retained-`EPERM` rule enumerated every document naming
`Ended`, `operationReports` or `EPERM` outside the grove and the archived
evidence. The completion spec and the transition design already stated both
amendments. Four documents were silent and were reconciled before the freeze:
[local runner](local-runner.md), [session CLI](session-cli.md), the component
guide's job row and consumer list, and the settlement sequence diagram. The
remaining documents that name `Ended` either predate the amendment as source
inventory of the fixed reference (source evidence) or do not list `Ended`'s
fields. Controls: the same pattern found the amendment in the spec, the design
and `crates/ravel/src/run.rs`, and found none of it in the four documents at
their parent revision, so a clean read there would have been a read of a broken
instrument. The spec's closing status paragraph still describes "a subset of
development controls"; it is a digested subject and was left unedited so the
recorded digests describe the tree that was measured. The support continuation
reconciles it when it freezes again.

**One handling note.** `junit/front.xml` was 1.2 MiB and the version control
refuses to snapshot a new file above 1 MiB, so it is stored as `front.xml.gz`
(byte-identical when decompressed; the original's SHA-256 is
`8a067bc2...8101`, with the other three in `campaign-junit-original.sha256`).
`tests.json` and `reconciliation.json` were derived from the original before it
was compressed.

**What this establishes, and what it does not.** On this host and compiler the
repaired product holds every claim of all 153 rows, the 143 failing broken
subjects each fail the claim they name, the incumbent baseline is classified for
all 153, and R01's 310 identities and ignored flags are preserved through the six
Taskfile tasks. It does **not** qualify GNU/Linux (aarch64 or x86_64) or the
minimum Rust 1.85, a returning host, an independent adopter, the comparative
questions (K4, k6, G2 to G5, R-W) or the book. A local pass is not supported-target
qualification, and the 30 rejected premises are properties of the incumbent
rather than of the repaired product.

**These receipts describe the tree before the minimum-Rust lint correction.**
The correction ([k51](#minimum-rust-lint-correction-k51)) edits four source sites
afterwards, so the digests recorded here are of the pre-edit tree. They stay as
measured and are not edited; the immutable campaign on the corrected tree is the
run leaf's.

## GNU/Linux and minimum-Rust continuation: development increment (k49)

`native-entry-linux-qualification-k49` became a node when its first session found
a production-source finding at the declared minimum Rust that its own charter
sends to a separate correction run. This section records the increment that
session delivered: the Linux environments, the ports of the instruments and
driver, and **development** measurements. Nothing here is an immutable receipt
and nothing here qualifies a target. The immutable runs follow the correction,
because any production edit invalidates the digests they freeze. Development
artefacts are kept under
[`docs/evidence/native-entry-linux-development/`](evidence/native-entry-linux-development).

### Environments

Each target runs in a Docker image built by `scripts/native-linux/Dockerfile`
(Ubuntu 26.04.1 pinned by digest, **rustc 1.85.0**, the exact floor of
`rust-version = "1.85"`, cargo-nextest 0.9.148 and Task 3.54.0 as on Darwin,
bubblewrap, a C compiler; each tool's SHA-256 is in the image and in the
campaign's `domain.json`). `scripts/native-linux.sh` stages a copy of the
working tree (none of the host's build output) and the checkout's git objects,
so the frozen incumbent revision is exported exactly as on Darwin. The Docker VM's
kernel is 7.0.14-linuxkit, aarch64, glibc 2.43. `task native-linux:image` and
`task test:native-linux` are the entry points.

Four container settings were each measured to be necessary, and each is a
property of running the product's sandbox in a container, not of the product:

| Setting | What happens without it |
|---|---|
| `seccomp=unconfined` | Docker's profile refuses `unshare(2)`; every confined check fails (`confinement_unusable`). `scripts/release-smoke.sh` records the same finding. |
| `systempaths=unconfined` | Docker masks parts of `/proc`; the kernel refuses a new procfs mount in a user namespace, so bubblewrap fails (`Can't mount proc`). 18 protected failures remained with the first setting alone. |
| `--init` | A `sleep` PID 1 never reaps reparented orphans, so every group-absence check reads zombies as present. 8 protected failures. |
| uid 1000, not root | As the release smoke's floor container runs; permission bits bind. |

With all four the plain protected suite on aarch64 Linux is 338 passed, 46
skipped: the same 384 identities as k43's Darwin run. The `hostile` tests need
`task probes` first, as `task check` runs it.

### What differs from Darwin

Three Darwin premises do not exist on this kernel. Each was repaired in an
instrument and none in production code.

| Premise | Linux fact | Where |
|---|---|---|
| E03: skipping the worker's SIGCHLD repair is harmless at an ignored-SIGCHLD entry, because exec clears Darwin's no-child-wait state. | An ignored SIGCHLD is itself the no-zombie state (a wait returns ECHILD once children are gone) and exec preserves `SIG_IGN`, clearing only action flags: [`flush_signal_handlers`, v6.18](https://github.com/torvalds/linux/blob/v6.18/kernel/signal.c#L489); [sigaction(2)](https://man7.org/linux/man-pages/man2/sigaction.2.html). The skip-repair subject fails ECHILD, so `E03__disposition=ignore` is a failing discriminator of its own and the incumbent genuinely fails it. | plan override, instrument assertion |
| E04 raw executable entry: observe whether SA_NOCLDWAIT reaches entry. | Action flags are cleared at exec, so it cannot. The row now asserts the flag is clear **after a read-back-confirmed pre-exec injection** (an unconfirmed injection refuses the spawn), so "cleared" can only mean exec cleared it. It is never relabelled as inheritance. | plan override, `raw_entry` |
| Incumbent S03 driver: a group holding only unreaped zombies answers `EPERM`. | Measured on this kernel: success for the signal-0 probe and for SIGKILL, ESRCH only after the reap. The driver and its control read live members from `/proc/<pid>/stat` (state `Z`/`X` excluded) on Linux, and keep the probe elsewhere. | incumbent overlay |

The plan gains a platform dimension rather than a second plan:
`platforms.<name>.rows` replaces a base expectation for one exact row, with its
reason. The 153 identities and 327 expected records are unchanged, and the
driver's selftest shows the Darwin and Linux plans differ on exactly the two
overridden rows and that an override applies only on its platform. A Darwin
development campaign with the edited instruments reconciles 153 of 153 and its
receipts equal k43's immutable ones item by item (0 of 153 differ; incumbent
67/56/30).

**Handled-signal domain** (`scripts/native-linux/signal-domain.c`, built and run
on each target): `_NSIG` 65, `SIGRTMIN` 34, `SIGRTMAX` 64; the kernel's
`rt_sigaction` answers for 1 to 64 and refuses 65; glibc's `sigaction` refuses
32 and 33 with EINVAL. The product's domain (1 to 64 excluding 32 and 33 on GNU
Linux) is exactly that.

### aarch64 Linux development campaign

One `task test:native-campaign` run on aarch64-unknown-linux-gnu (kernel
7.0.14, glibc 2.43, rustc 1.85.0): 112 tests executed, none failed, no subject
moved, **153 of 153 rows reconciled**. The incumbent baseline is 68 fails, 55
already correct, 30 rejected premises, against Darwin's 67/56/30: the one row
that changes class is `E03__disposition=ignore` (already correct on Darwin, fails
here), and two other receipts differ only in the order their observed records
are listed. Elapsed 1 minute 11 seconds; the 30/5/5 controller envelope was
never approached.

R01 through the six Taskfile tasks, joined identity by identity as k43 did
(`scripts/native-r01.sh`, with the join's failing controls):

| Task | Result | Protected identities |
|---|---|---|
| frozen listing | 310 of 310 preserved, 74 new | 310 |
| `test:cli` | exit 0 | 13 of 13 (14 in scope) |
| `test:session` | exit 0 | 219 of 219 (227 in scope) |
| `test:runner` | exit 0 | 71 of 71 (83 in scope) |
| `test:terminal-recovery` | exit 0 | the two private PTY selections |
| `test` | exit 0 | 290 of 290 (310 in scope, 20 ignored) |
| `check` | **exit 201, at the lint step** | 0 of 290 (the nextest step never ran) |

### Minimum Rust 1.85: the lint step fails

`task check`'s lint step fails at the declared minimum on **every** platform.
The Darwin campaign at Rust 1.99 could not see it. Clippy 0.1.85 (both 1.85.0
and 1.85.1) reports four deny-level errors, all of which clippy 1.86.0
accepts; a scratch copy with the four sites rewritten is clean for
`--workspace --all-targets -D warnings` and for `native-faults`
([the log on the unmodified tree](evidence/native-entry-linux-development/minimum-rust-clippy/clippy-1.85-unmodified-tree.txt)
and [the proposed repair, not applied](evidence/native-entry-linux-development/minimum-rust-clippy/proposed-repair-not-applied.patch)):

| Site | Lint | Kind of file |
|---|---|---|
| `crates/keyed-launch/src/run.rs:534` | `clippy::precedence` (`bits \| 1 << n`) | production library |
| `crates/ravel/build.rs:72` | `clippy::format_collect` | production build script |
| `crates/ravel/src/context.rs:723` | `clippy::format_collect` | production library |
| `crates/ravel/tests/cli.rs:451` | `clippy::format_collect` | protected test file |

Rust 1.85.1 builds the workspace with `--all-targets` and the `native-faults`
fixture with no compiler error or warning. No API newer than 1.85 is used by the
repaired code or its instruments. The repair is a source edit in production
files, so by the charter it is its own failing-first correction run and is not
made here.

The repair is made and measured in [the next section](#minimum-rust-lint-correction-k51).

An earlier check on the Darwin host reported 1.85 clippy clean. That reading was
a broken instrument: `rustup run 1.85 cargo clippy` resolved to Homebrew's
clippy 0.1.99 driving rustc 1.85.1. The genuine 0.1.85 driver
(`~/.rustup/toolchains/1.85-*/bin/cargo-clippy`) reports the four errors above.
Any lint check at a pinned toolchain must name its clippy version.

### x86_64 under user-mode emulation

An x86_64 Linux kernel is not available to the campaign. The target is run under
QEMU user-mode emulation on the aarch64 kernel above, and every result in this
subsection is **emulated** and qualifies nothing about an x86-64 kernel.

*Getting a faithful guest.* Docker Desktop's registered QEMU (10.2.3) maps a
64-bit guest above 2^47 on this VM's 48-bit address space, and rustc's allocator
then faults before doing any work (rustc 1.85.0 and 1.93.1, and under no
allocator setting or CPU model tried). `scripts/release-smoke.sh` documents the
same finding and its fix: a guest base of 2^47 (`-B 0x800000000000`) through a
private binfmt_misc instance with the pinned QEMU. `scripts/native-linux-qemu.sh`
reuses that mechanism with the guest as the namespace's root, at CPU model
Nehalem (the floor the release smoke emulates). The guest's address space and
vDSO are checked before use. rustc, cargo, nextest and the pinned Bun x64 all
run under it; the workspace builds, and the plain protected suite runs in
about 93 seconds.

*What the emulator cannot do* (`scripts/native-linux/emulation-probe.c`, one
source built for both targets):

| Operation | aarch64 Linux, native | x86_64, emulated |
|---|---|---|
| threads per process, as the host counts them | 1 | **2** (a second thread, named `qemu`) |
| `unshare(CLONE_NEWUSER)` | succeeds | **EINVAL** (it needs a single-threaded caller) |
| wait under SIGCHLD = SIG_DFL + SA_NOCLDWAIT | ECHILD | **the child's exit status** |
| wait under SIGCHLD = SIG_IGN | ECHILD | ECHILD |
| `kill(-pgid, 0)` for one unreaped zombie | success | success |

So no emulated process can create a user namespace, which the product's
bubblewrap confinement needs, and the emulator does not honour SA_NOCLDWAIT.

*Protected suite under emulation.* `cargo nextest run --workspace`: 338 run, 261
passed, 77 failed, 46 skipped. 74 of the 77 need a user namespace
(`confinement_unusable`: 68 partial-acquisition cases, 3 `ravel::cli` `--confine`
cases, 2 `keyed-launch::confinement`, 1 confined-cancellation case); 1
(`the_child_sees_its_arg0_while_its_program_is_what_runs`) reads the process's
own argv from `ps`, which under the emulator is the emulator's; 2 are matrix
instruments (below). The 74 are an emulation limit, not a product fact, and no
protected expectation was changed.

*Matrix under emulation.* `task test:native-campaign` with the platform
`linux-emulated`: 112 tests executed, **146 of 153 rows reconciled** to the
Linux expectations and **7 emulation-limited**, each with its reason and
control, 3 expected test failures and no unexpected one, no subject moved.
The driver accounts for a limited row as its own status and rejects a limit
whose row did not diverge, a divergence the plan did not list and a test
failure it did not expect (selftest). The seven:

| Rows | What was observed | Cause |
|---|---|---|
| E04 worker, E04 managed, E04 raw | the skip-repair and ignore-only subjects still confirm the wait; the raw row's test stopped before it ran | the emulator does not honour SA_NOCLDWAIT (probe above) |
| E07 ×4 | `broken:guard` conformed (the runner reported the first signal although the later-wins guard was removed); the remaining E07 rows did not execute | **not isolated**: the runner's loop and signal delivery run inside the emulator, and no native x86-64 run exists to separate the emulator from the architecture |

The incumbent baseline under emulation is 66 fails, 57 already correct, 30
rejected; the two E04 rows whose class moves are among the seven and carry no
meaning. Elapsed 5 minutes 26 seconds, no controller timeout.

### What this increment establishes

Linux and emulation environments that reproduce, a driver and plan that carry a
platform dimension and an emulation accounting with failing controls, and
development measurements that say: the repaired product reconciles 153 of 153 on
aarch64 Linux at Rust 1.85.0 with three Darwin premises replaced by measured
Linux ones; its protected suite passes there; the declared minimum's lint step
does not; and under emulation 146 of 153 rows agree while seven cannot be
reproduced and 74 protected identities cannot run. It does **not** establish an
immutable receipt for any Linux target, `task check` at the minimum, or anything
about a native x86-64 kernel.

## Minimum-Rust lint correction (k51)

`native-entry-minimum-rust-lint-correction-k51` is the failing-first correction
the k49 charter asked for: `task check`'s lint step must pass at the declared
minimum Rust, and nothing else may change. Its logs, the instruments and the
receipts are under
[`docs/evidence/native-entry-minimum-rust-lint-correction/`](evidence/native-entry-minimum-rust-lint-correction).

### Red first, on the unmodified tree

The tree was `15cb3f675cec` with no edit. The same command
(`cargo clippy --locked --workspace --all-targets -- -D warnings`, and again with
`--features native-faults`) was run under four toolchains, each log naming the
clippy that `cargo clippy` really dispatched to:

| Toolchain | clippy | Result |
|---|---|---|
| rustc 1.85.0 | 0.1.85 (`4d91de4e48`) | **fails**, exit 101 |
| rustc 1.85.1 | 0.1.85 (`4eb161250e`) | **fails**, exit 101 |
| rustc 1.86.0 | 0.1.86 | clean |
| rustc 1.93.1 | 0.1.93 | clean |

Clippy stops dependents behind a failing crate, so the deny-level run reports
`clippy::precedence` in `keyed-launch` and `clippy::format_collect` in the build
script and then stops. To see every site on the unmodified tree, a second run with
`RUSTFLAGS="-W clippy::all"` demotes the lints to warnings and prints all four,
at the lines named in the finding above, for 1.85.0 and 1.85.1
(`red/*-enum.txt`). The red is therefore the declared minimum's: the same tree and
command are clean one minor release later.

*Instrument finding.* The first four readings were all clean. `cargo` looks a
subcommand up in `~/.cargo/bin` (rustup's proxies) before `PATH`, so putting a
toolchain's `bin` first on `PATH` still ran the default toolchain's clippy, 0.1.93
(`cargo clippy --version`, run under that `PATH`, printed it). The harness
(`lint.sh`) now selects the toolchain with `RUSTUP_TOOLCHAIN` and writes
`cargo clippy --version` at the top of every log. This is the same trap k50 met
with Homebrew's clippy, one layer further down, and the header is the control.

### The repair

| Site | Change |
|---|---|
| `crates/keyed-launch/src/run.rs:534` | `bits \| 1 << (signal - 1)` becomes `bits \| (1 << (signal - 1))`. Parentheses only. |
| `crates/ravel/build.rs` `hex` | the `map(format!).collect()` becomes a `String::with_capacity` loop with `write!`, the idiom `crates/keyed-launch/src/channel.rs` already uses for its own `hex`. |
| `crates/ravel/src/context.rs` `hex` | the same. |
| `crates/ravel/tests/cli.rs:451` | `length.iter().map(..).collect::<String>()` becomes `length.map(..).concat()`. Only how the octal escape string is built differs; the line before and the assertions after are untouched. |

No `#[allow]` was added. No test file other than the one protected line changed,
no test was added (so the 310 identities, and the 74 new ones, are the same
names), and nothing under `docs/evidence/native-entry-campaign` or the incumbent
subject changed.

### Behaviour is unchanged

- **Precedence site.** `-Zunpretty=mir` and `-Zunpretty=hir` of the `keyed-launch`
  library on nightly 1.97, before and after, are byte-identical (18620 and 3829
  lines, `precedence-site-compiled-form.txt`). The control is a deliberate fault,
  `signal - 1` to `signal - 2`: it changes MIR by four lines and HIR by two, so the
  comparison is seen to go dirty. *Object code is not evidence here:* the same
  fault left both object files byte-identical, because `EntrySignals::new` is
  generic and is monomorphised only in the crates that call it. MIR and HIR are
  taken before monomorphisation, which is why they are the instrument.
- **Both `hex` functions.** `hex-differential.py` extracts `fn hex` from the base
  revision and the working tree (not retyped copies), compiles old and new side by
  side with rustc 1.85.0 and compares them on every single byte `0..=255`, the
  whole run, the empty slice and 64 pseudo-random lengths: 644 comparisons, 0
  differ. The mutant (`{byte:02X}`) differs in 221.
- **The protected test's fixture.** The same script extracts the statements that
  compute the length prefix and the `format!` that builds the worker script, and
  compares the full script text for every `hello` length 0 to 1023 and the test's
  own `hello`: 1025 comparisons, 0 differ; the mutant (`03x`) differs in 993.

### Green, and the protected expectations

| Reading | Result |
|---|---|
| clippy 0.1.85 (1.85.0 and 1.85.1), `--workspace --all-targets -D warnings`, and with `native-faults` | clean (`green/`) |
| clippy 0.1.86 and 0.1.93, the same | clean |
| `cargo fmt --all --check` at 1.85.0 and 1.93.1 | clean |
| aarch64-apple-darwin, `scripts/native-r01.sh run` (Homebrew rustc 1.99) | 310 of 310 preserved, 74 new identities; `test:cli` 13 of 14 in scope (1 ignored fixture), `test:session` 219 of 227 (8), `test:runner` 71 of 83 (12), `test` 290 of 310 (20); subject digests identical before and after |
| aarch64 GNU/Linux, rustc/clippy 1.85.0, kernel 7.0.14-linuxkit, glibc 2.43 | `cargo build --all-targets`, clippy (both feature sets) and fmt exit 0; R01 through all six tasks exit 0, **including `check`**: eight steps passed, 338 of 338 tests, 290 of 290 identities, digests identical before and after |

The Linux row is the first time `task check` passes at the declared minimum. The
join's failing controls (`task test:native-r01-controls`) pass.

**One failure to report.** In the Darwin R01 run, `task check` exited 201: the
nextest step failed `worker::native_tests::native_f03_first_create`, whose
`first-create-ignored` broken subject was refused with `selection_timeout` at the
1000 ms bound. The identical suite had passed 338 of 338 in `task test` 90
seconds earlier in the same run, and all other `check` steps passed. On the same
tree (digests re-verified) `task check` alone then passed, three further
whole-suite runs passed (338 of 338 each), and the case alone passed 15 of 15
quiet and 15 of 15 beside one busy loop per CPU
(`darwin-check-reruns/summary.txt`). The failed receipt is kept as it was, in
`r01-darwin-aarch64-apple-darwin/check.txt`, and the join counted it
(`failure_lines: 2`). **The cause is not established**: this is one failure in
six whole-suite runs, not reproduced, on a host where other processes (indexers,
the terminal) were active. `.config/nextest.toml` already names this class: a
protected one-second deadline must not compete with other work. It is
independent of the four edited sites (a deadline, and the edit changes only
digest rendering and a parenthesis), but "independent by inspection" is not a
reproduction, so the immutable run should be taken on a quiet host and must treat
a repeat as a finding.

### What this establishes, and what it does not

At the declared minimum Rust the workspace builds, lints and formats clean, with
behaviour shown unchanged at each edited site, the protected suite and R01
unchanged on aarch64-apple-darwin and on aarch64 GNU/Linux, and `task check`
passing on both. It is **not** an immutable campaign receipt: the frozen matrix
was not rerun on the corrected tree, nothing about x86_64 is measured, and the
k43 and k50 receipts describe the tree before this edit.

## GNU/Linux immutable campaign (k52)

`native-entry-linux-campaign-run-k52` took the immutable measurement on the
corrected tree, as k43 did for Darwin: the frozen 153-row matrix, its incumbent
baseline and the 310 protected identities through the six Taskfile tasks, on
three targets. Every receipt is under
[`docs/evidence/native-entry-linux-campaign/`](evidence/native-entry-linux-campaign),
one directory per target, each with its `domain.json`, `subjects-before.json` and
`subjects-after.json`, `rows/` (write-once records), `receipts/` (one per row),
`reconciliation.json`, the nextest logs and the R01 joins.
`compare-receipts.py` and its output `compare-receipts.txt` are the item-by-item
comparison against k43 quoted below. The k43, k50 and k51 directories are
untouched; they describe earlier trees.

### What was measured, and on what

The tree is parent revision `6674fd024c10` with this leaf's working-copy edits:
the two digested documents (made once, before any run; they state scope and point
here, and claim no outcome) and one instrument edit (below). Each target ran from
that one tree, one at a time, with nothing else running.

| | aarch64-apple-darwin | aarch64-unknown-linux-gnu | x86_64-unknown-linux-gnu, **emulated** |
|---|---|---|---|
| Kernel, libc | Darwin 27.0.0 (macOS 27.0.1, 26A434), system libc, libc crate 0.2.186 | 7.0.14-linuxkit (Docker Desktop VM), glibc 2.43 (Ubuntu 26.04.1) | the same aarch64 VM kernel under QEMU user-mode 10.2.3, `-B 0x800000000000 -cpu Nehalem`, same glibc userland |
| Compiler | rustc 1.99.0 | **rustc 1.85.0**, the declared minimum | **rustc 1.85.0** |
| Tools | nextest 0.9.148, task 3.54.0 | nextest 0.9.148, task 3.54.0, tool digests in `domain.txt` | the same versions, x86_64 builds |
| Subjects digested before and after | 336, none moved | 336, none moved | 336, none moved |
| Build, clippy `-D warnings`, `fmt --check` | not part of this run (k51 ran them on the host) | pass, pass, pass | pass, pass, pass |
| Matrix rows reconciled | **153 of 153** | **153 of 153** | **146 of 153**; 7 emulation-limited |
| Tests executed / failed unexpectedly | 112 / 0 | 112 / 0 | 112 / 0 (3 expected failures, the limited rows) |
| Baseline (incumbent `0515419edd29`) | 67 fail, 56 already correct, 30 rejected | 68 fail, 55 already correct, 30 rejected | 66 fail, 57 already correct, 30 rejected |
| R01 | 310 of 310 preserved, 74 new; six tasks exit 0 | 310 of 310 preserved, 74 new; six tasks exit 0 | 310 of 310 preserved, 74 new; **see below** |
| Wall time (campaign / R01) | 1 min 57 s / 5 min 12 s | 1 min 13 s / 2 min 42 s | 5 min 59 s / 4 min 24 s |

Darwin's and aarch64 Linux's six R01 tasks each exit 0 and join identity by
identity: `test:cli` 13 of 13 non-ignored (14 in scope), `test:session` 219 of
219 (227), `test:runner` 71 of 71 (83), `test` and `check` 290 of 290 (310, 20
ignored fixtures), `test:terminal-recovery` its two private PTY selections. The
338 run plus 46 skipped are the 310 protected identities and the 74 new ones on
both. `task check` passes at 1.85.0 on aarch64 Linux, its first immutable
pass at the declared minimum.

### The first aarch64 run found an instrument defect, and was not used

The first immutable aarch64 run (`attempt-1-aarch64-linux-incumbent-guard/`)
failed: the campaign step exited 201 at the incumbent build, and R01 exited 1
because the incumbent was never built to be digested. The incumbent driver
refuses to build unless its worker sources are byte-for-byte the repaired
tree's, and `crates/ravel/build.rs` is one of them. k51 had rewritten `hex` there
(same output for all 256 byte values, shown in k51), so the guard, which had
never run since, correctly said the files differ. Nothing had exercised the
campaign between k43 and this run (k50 ran before the edit, k51 ran R01 only).

The repair is one instrument edit, in `scripts/native-incumbent.sh`: the one
sanctioned difference is pinned as the SHA-256 of the normal-format `diff` of the
incumbent's `build.rs` against the repaired tree's. Any other difference in
`build.rs`, or any difference in the rest of the worker, is still refused, and the
selftest was extended with three controls seen to behave: the unmodified
incumbent is accepted, a `build.rs` identical to the repaired tree's is
accepted, and the same file with one further line is rejected
(`bash scripts/native-incumbent.sh selftest`). Because `scripts/` is a digested
subject, the edit invalidated that run and any earlier digest, so the Darwin,
aarch64 and x86_64 runs reported here were all taken after it and none before.
The failed run's directory is kept as the finding, unaltered.

### aarch64 Linux, Rust 1.85.0, native

All 153 rows reconcile. 151 receipts equal k43's Darwin receipts field for field
(verdicts, subjects, baseline class, status); the two that differ are exactly the
two rows whose Darwin premise does not exist on this kernel and which carry their
platform expectation and reason in the plan: `E03__disposition=ignore` (an
ignored SIGCHLD is itself the no-zombie state, and exec keeps SIG_IGN, so the
skip-repair subject fails ECHILD and the incumbent baseline moves from
already-correct to fails) and `E04__boundary=raw-executable-entry` (exec always
clears action flags, so the row asserts the flag is clear after a
read-back-confirmed injection). These are the same two k50 found in
development; the immutable run reproduces them. No new difference appeared. The
handled-signal domain read from the target's own headers and kernel is 1 to 64
with glibc reserving 32 and 33 (`signal-domain.txt`).

### x86_64 Linux under user-mode emulation (recorded as emulated)

The matrix and the suite build and run under the emulator, and 146 of 153 rows
reconcile to the Linux expectations. It is **not** a qualification of an x86-64
kernel: no kernel, signal, process-group or PTY semantics of that architecture
have been observed, and nothing here may be cited as such.

The seven emulation-limited rows each carry a reason and a control in their
receipt (`limit`), and are neither reconciled nor dropped:

| Rows | Observation | Cause and control |
|---|---|---|
| E04 worker, E04 managed (2) | the skip-repair and ignore-only subjects still confirm the wait | QEMU does not honour SA_NOCLDWAIT: `emulation-probe.c`, one source built for both targets, gives `ECHILD` natively and the child's exit status here, and `ECHILD` for SIG_IGN in both |
| E04 raw executable entry (1) | not executed; its test stopped at the worker row | the same limit; no observation of entry flags exists for this target |
| E07 TERM-then-HUP runner-active (1) | `broken:guard` conformed: the runner reported the first signal though the retaining guard was removed | **not isolated**: the loop and its signal queueing run inside the emulator; native aarch64 reconciles all four E07 rows with every break failing, and a native x86-64 run that would separate emulator from architecture does not exist |
| E07 other three (3) | not executed; the test stopped at the first row | the same |

**R01 under emulation is not a join and says so.** The six tasks stop under
nextest's default fail-fast after two failures (34 of 338 run), so those logs are
not a partition of the identities. They are kept as measured, and are not the
account of what cannot run. A supplementary run, in the same container and after
the campaign's subjects were digested again, took `cargo nextest run --locked
--workspace --no-fail-fast` (the Taskfile's `test` command with one flag), kept
under `supplement-no-fail-fast/`: 338 run, **261 passed, 77 failed**, 46 skipped,
identical to k50's development figures. Joined against the manifest, 215 of the
290 non-ignored protected identities pass and 75 do not; the other two failures
are new matrix instruments. The 77 failures, classified from their own output
(`failure-classification.json`):

| Cause | Failures | How it is known |
|---|---|---|
| a user namespace cannot be created (`unshare(CLONE_NEWUSER)` is `EINVAL` for every emulated process: a second host thread) | **74** | 69 show `confinement_unusable`, 3 show bwrap's own "Creating new namespace failed"; 2 (`ravel::cli`, exit 2 where 5 is asserted before any diagnostic is captured) are **inferred**: both pass `--confine` and both pass natively on aarch64 |
| the child's argv as `ps` reads it is the emulator's | 1 | `the_child_sees_its_arg0_while_its_program_is_what_runs` |
| the matrix instruments of the two limited families | 2 | E04 managed and E07, as above |

These 74 are the protected identities that cannot run here; every one of them
passes natively on aarch64 Linux at the same compiler. No protected expectation
was changed or weakened to obtain any figure.

### Darwin, on the corrected tree

`task test:native-campaign` and `native-r01.sh run` on the host, into new
directories, equal k43's item by item: 153 receipts and 450 rows with 0
differing (free text, pids and list order excluded; `compare-receipts.py`), the
same 112 tests, the same R01 counts. One thing in k43's directory does not match
its own table: its stored `test-cli.txt.r01.json` reads scope 0, while its log
holds 13 `PASS` lines and its note says 13 of 13. This run's join of the same
task is 13 of 14 in scope (one ignored fixture), as k43's table says. k43's
directory is not edited. The `check` step passed (338 passed, 46 skipped); the
selection-deadline miss k51 recorded did not reproduce in this run, and is not
explained by it.

### Support limits, per target

- **aarch64-apple-darwin** (Darwin 27.0.0, rustc 1.99.0): the matrix and R01 are
  measured with immutable receipts, on the kernel and compiler named.
- **aarch64-unknown-linux-gnu** (7.0.14-linuxkit in Docker Desktop, glibc 2.43,
  rustc 1.85.0, the declared minimum): the matrix and R01 are measured with
  immutable receipts, with two rows holding platform expectations. This is one
  kernel and one libc; another Linux kernel or glibc is not measured, and the
  container needs four settings (`seccomp=unconfined`, `systempaths=unconfined`,
  `--init`, a non-root user) which are properties of running the product's
  sandbox in a container.
- **x86_64-unknown-linux-gnu**: built, linted at 1.85.0, and run **under QEMU
  user-mode emulation only**. 146 rows reconcile, 7 are limited with reason and
  control, and 74 protected identities cannot run; the numbers did not move from
  k50's. Nothing about an x86-64 kernel is established. Native qualification is
  deferred, not rejected, and tracked in
  [Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1).
  Native-entry conformance is measured with immutable receipts on
  aarch64-apple-darwin and aarch64-unknown-linux-gnu only.
- **Not measured on any target**: other kernels, libc versions and Rust versions
  (only 1.85.0 and 1.99.0 were run), a returning host, independent adoption, and
  the root obligations this campaign never owned.

### Charges

This leaf charges P≤30 / E≤30 / RC=0 minutes, X=0, D=0 (no in-session reviewer
was spent; the guard repair is covered by an executable control). Machine time
was about 28 minutes: the failed run 2, Darwin 7, aarch64 4, x86_64 12, the
supplement 3. k50 and k51 recorded no separate charge, so this note does not sum
a node total; the node's projection was P≤45 / E≤60 / RC≤4.

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
   above); the incumbent subject and its baseline now exist for all 153 rows
   (k44, k45 and k46, above), and the frozen campaign has since run on the
   host (k43, above) and on both Linux targets (k52, above).
6. Complete R01 through the existing Taskfile tasks and preserve every previous
   expectation. Publish qualified evidence only when earned. Reconcile the
   approved operation-report API amendment throughout docs. Price and cut the
   separate Rust 1.85/Linux support remainder inside k22 before the book.

The frozen native campaign now exists for aarch64-apple-darwin and Rust 1.99
([above](#frozen-campaign-and-r01-k43)); the development summaries earlier in
this note remain development evidence. The support continuation ran the same
matrix on GNU/Linux and Rust 1.85 (k52, above); qualification on a native
x86-64 kernel is deferred to
[Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1).
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

`native-entry-campaign-funding-k41` read the incumbent product `0515419edd29`
before pricing the remaining campaign. Its source performs most of the matrix's
faulted operations but discards their results, so about 129 of the 153 rows can
be hosted once private test seams are ported to it. About 24 rows target an
operation the incumbent never performs (it has, for example, no
finalisation-time pending query) and are rejected premises whatever the
funding. The read is source-only and family-level;
`native-entry-incumbent-baseline-k42` verifies each row. The human's answer was
**“I'm not actually concerned about the budget - just do what is required.”**
Budget caps no longer gate required work; zero production disruption still
binds and no acceptance is narrowed. After k41's RC≤4, cumulative bounds are
D≤115/RC≤83/P≤397/E≤142/X=0 under caps D≤135/RC≤88/P≤467/E≤177/X=0, kept as
bookkeeping, with D≤20 for phase closure and RC≤5/P≤70/E≤35/X=0 remaining for
k42 and k43.
