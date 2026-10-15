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
the proposed remainder reserves time for that route, not another spawned reader.
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

## Exact remainder before acceptance

The [case manifest](native-entry-cases.json) preserves 153 concrete native rows
and the 310 preexisting test identities, including ignored fixtures. A row's
presence is a required case identity, not an execution result. Keep the frozen
parameter products, mutation modes, four paired failures, two queued-TERM
variants and S counts 8/7/8/7/2 unchanged.

1. Finish E04's managed mutant; E05's four direct/terminal INT handoff cases;
   E06's 15 acknowledged boundary joins; E07's four acknowledged first-consumption
   cases; and E08's four complete/escaped-reader cases. Receipts must distinguish
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
   five seconds, last five seconds reserved for cleanup. Observe watchdog,
   blocked-acknowledgement/failed-release and escaped-holder cleanup controls.
   Preserve and boundedly settle every owned child, holder, pipe and PTY on
   failure as well as success. Current helper cleanup is development code;
   older C4 fixture mechanics are not automatically a qualified native controller.
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

Propose the following **remaining** work, in minutes: P≤100 (80 for remaining
finite instruments/controllers/source corrections, 20 for immutable evidence and
documentation), E≤70 (15 controller validation, 30 native campaigns, 10 protected
checks, 15 affected reruns/reconciliation), RC≤10 (3 agreement/recording, 5 for
the tree-owned review, 2 remaining coordination/closure), D=X=0. Current remaining
balances after the recorded attempt are D≤20 reserved/RC≤1/P≤55/E≤60/X=0.
The exact shortfall is therefore **P≤45/E≤10/RC≤9**, with no D/X increase.
Proposed phase caps are D≤135/RC≤74/P≤325/E≤130/X=0. Full use would leave
only D≤20 reserved; prices and proposed caps are not approval or spend. The
attempt charges P≤70/E≤30/RC≤4, D=X=0, once across k30/k28; cumulative bounds
are D≤115/RC≤64/P≤225/E≤60/X=0. The E≤30 bound replaces the earlier E≤15
preservation proposal, covering the additional failed/focused/sequential and
clean-fixture verification. It does not add another P≤70 or RC≤4 charge.

The human funding continuation is `native-entry-qualification-funding-k29`,
ordered before the open k28 node inside k22. It must obtain explicit agreement
before committing a renewed producer allocation. No correctness/root closure,
book acceptance, comparative gain or support qualification follows from this
attempt. K4/k6, G2–G5/R-W and the root's delivery/documentation work remain live.
