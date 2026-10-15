# native-entry-complete-repair-k28 — brief

## Goal

Deliver and check the complete incumbent native-entry-to-settlement repair as
one working increment inside k22, before book-structure-k5. Capture, front,
worker and runner land together, with protected behavior and discriminating
receipts. This is correctness work on the one-session launcher CLI.

## Context

Read native-entry-complete-funding-k26's verbatim approval and
native-entry-complete-repair-k27's charges. The ancestor k22 brief owns the
unchanged additive Started API and exact E/F/R parameter expansion, paired
secondary failures and cancellation variants. docs/native-entry-transition-design.md
owns unchanged T0–T8, the selected bounded error-settlement protocol, protected
logical wait-event reconciliation, exactly 32 approved S01–S05 rows and the
complete price. docs/specs/result-job-completion.md owns required behavior;
docs/completion-failure-design-evidence.md owns the prior scoped C1–C4 receipts.
Source/toolchain verification belongs to this producer; prior design/source
observations are not native runtime conformance.

Approved phase caps are D≤135/RC≤65/P≤280/E≤120/X=0 minutes, with zero
production disruption. After k27's conservative D≤5/RC≤1 charge, cumulative
correctness charged bounds are D≤115/RC≤60/P≤155/E≤30/X=0. Remaining balances
are D≤20 exclusively phase closure, RC≤5/P≤125/E≤90/X=0. This complete
producer allocates P≤125/E≤85/RC≤5, D=X=0. P includes every source/control,
E/F/R and S instrument, bounded cleanup controller, manifest and evidence edit;
E includes baseline/broken/repaired, controller validation, protected checks,
affected reruns and item-by-item reconciliation. RC covers one narrow reviewer,
coordination and closure. Allocations are not spend. Full use would leave
D≤20 reserved/RC=0/P=0/E≤5/X=0; record actual charges at closure.

## Done when

- Implement the whole frozen T0–T8 protocol. Keep immutable executable-entry
  values separate from launcher repairs; checked watching precedes worker and
  managed child. Front scope survives choice failures; checked reader shielding
  lasts until thread exit, with bounded collection. Common explicit finalisation
  covers selection failure, failed spawn and no-ending errors. First consumed
  cancellation and all original/secondary reports survive; unknown queries never
  establish absence or permit success. Preserve C1–C4 and every protected logical
  wait-event trace individually.
- Add exactly run_observed_with_start_check(launch, observer, start_check) and
  run_confined_observed_with_start_check(launch, policy, observer, start_check)
  in keyed-launch beside existing exports. Preserve existing argument types and
  Result<Ended, LaunchError>, observer &mut dyn FnMut(LaunchEvent), and check
  &mut dyn FnMut() -> std::io::Result<()>. Existing infallible entry points adapt
  with an always-Ok check. Callbacks are synchronous, prompt and nonpanicking.
  Own child/group/lease before Started; emit Started once then check once after
  spawn-reported success, neither on failed spawn. Failed check retains io::Error
  as source with operation context and inspectable source errno, never the
  spawn-only raw_os_error accessor or Ended success. Reaped is infallible, once
  only on actual confirmed reap before recovery, including error cleanup.
- Apply the selected owner-local nonblocking settlement, without a generic
  syscall layer or background reaper. Use absolute 2-second runner error and
  post-KILL deadlines (earlier overlapping bound wins), and total 2-second worker
  teardown including graceful wait; retain separate existing at-most-1-second
  worker drain and query-only post-reap group confirmation. EINTR/secondary
  failure never restarts a deadline. Runner uses reserved WNOWAIT observation
  and bounded nonblocking reap, worker sole-owner try_wait. No blocking wait or
  terminal drain reaches recovery. Preserve checked individual group/PID stop
  reports, the specified pre-reap stop sequences and explicit unresolved ownership.
  ECHILD, actual reap or release disables subsequent numeric signalling; a
  substituted report cannot restore released ownership or invent a lifecycle fact.
- Failed Started always attempts bounded real child/group/wait/recovery before
  returning, never lends again, and preserves first cancellation plus every
  cleanup report. Unknown-status recovery preserves foreign foreground and
  reports unconfirmed foreground/modes; it cannot enable delegated takeover.
  Retain C4's exclusive foreground authority/no unrelated ID reuse through
  handback premises, checked foreground/mode/SIGTTOU operations and uncertainty.
  Polling bounds do not promise real-time behavior against scheduler/kernel stalls.
- Freeze exactly the ancestor's E01–E08/F01–F07/R01 expansion, including its
  specified fault modes, paired cases and cancellation variants, plus only the
  design's approved S01–S05 rows (8/7/8/7/2). No all-fault Cartesian product,
  long-grace row or target infrastructure is commissioned. Controls are private,
  fixed and owner-local; no installed switches. Use the agreed CLI/real-policy/
  fake-job, real-process and PTY seams. Record actual physical operations,
  substituted reports, fixture cleanup and product settlement separately.
- Before measurements freeze subjects/manifests, exact row identities, source,
  fixtures, scripts, reference expectations, exclusion notes and measured
  toolchain/libc/kernel domain. Digest all subjects before/after, one writer per
  receipt. Establish meaningful failing broken controls per claimed repair;
  report already-correct baselines honestly. Use acknowledged barriers with
  owner/mask/consumption facts; never sleeps as ordering evidence. Preserve PID/
  SID/PGID, foreground/modes, raw action/mask, pending/consumed signals, actual
  wait/reap and reader completion/aliveness receipts. Reconcile each row and
  rerun every affected case after subject/instrument changes; totals do not suffice.
- Keep each controller's 30-second outer budget, 5-second acknowledgement limit
  and final 5 seconds for cleanup, all within the same outer deadline. Validate
  watchdog expiry, blocked acknowledgement/failed release and escaped-holder
  cleanup controls before trusting runs. Own and boundedly stop/wait/close all
  fixture children, holders, pipes and PTYs on success, failure and timeout; no
  signalling reaped/released identities. Timeout/unresolved fixture cleanup is a
  failed receipt. Fixed production graces must fit the first 25 seconds. Longer
  legal graces need their exact priced timing remainder before execution, not
  silently enlarged limits. Broken unbounded controls are killed by their owner.
- Freeze test:list identities first and preserve every prior expectation through
  Taskfile test:cli, test:session, test:runner, test:terminal-recovery, test and
  check. Keep configuration, exact UTF-8/empty/nesting/first-writer/final-write,
  cancellation/diagnostic precedence and C1–C4 intact. Any new Taskfile selector
  wraps only these finite cases. Reconcile protected identities individually.
- Publish immutable per-case baseline/broken/repaired receipts and evidence
  linked from the design/contract, with actual failures, measured domain, support
  limits and spend. Use at most one narrow reviewer for failed-Started cleanup
  and first-signal/reader/finaliser composition; classify every finding. A second
  substantial review, material mechanism discovery, insufficient complete price
  or extra infrastructure earns only a precise funded/scope continuation before
  the book. Do not borrow reserved D, reset caps or narrow acceptance.
- Record actual charges and remaining balances in the brief chain. Preserve the
  receipts and price/fund/cut the next exact support remainder before the book;
  minimum Rust 1.85/Linux qualification is still owed and outside this local
  repair allocation. If renewal is needed, cut its precise human scope/funding
  continuation inside k22 before retirement, keeping unmet work live. Local
  success cannot close k22/correctness/root or advance book acceptance.

## Notes

No new mechanism discovery is allocated. The three existing ADRs remain current;
this repair selects no package, extraction, comparative target or migration.
Started means spawn-reported success, including possible pre-exec signal death.
Host-only receipts qualify only that host/compiler; verify signal-domain facts
against actual target libc headers and preserve executable-versus-private
SA_NOCLDWAIT distinctions. A returning host requires a separate contract.
K4/k6 comparison history, G2–G5/R-W, independent adoption and all root delivery/
documentation obligations remain live. No comparative gain is inferred.

## Decisions (running log)

- The human clarified ordinary-stop EPERM handling verbatim: **“Allow success
  after confirmed complete cleanup; retain the EPERM report (Recommended)”**.
  Darwin can report EPERM for a group containing only unreaped zombies. Retain
  the individual stop report, but actual reap, successful terminal recovery and
  an ESRCH group-absence confirmation may establish successful completion.
  This exception does not turn EPERM into absence or permit success with an
  unconfirmed lifecycle, recovery or group. Initiating Started/transition errors
  remain failures. Preserve the otherwise unchanged frozen API and case matrix.
- The human approved the necessary successful-outcome diagnostic extension
  verbatim: **“Carry reports in Ended and the JSON notice (Recommended)”**.
  Add an operation-reports field to Ended (existing struct literals need the
  field), while preserving launch argument types and Result<Ended, LaunchError>.
  Emit successful benign stop reports in the existing single JSON notice rather
  than extra runner stderr lines. Non-JSON consumers can report those same
  returned observations. This is the explicit amendment to the additive API.
- Freeze `docs/native-entry-cases.json` before native measurements: 153 concrete
  native/control rows plus the 310 existing test identities (including ignored
  fixtures). Paired and queued-TERM cases use the omitted primary mutation;
  ordinary mutation rows retain both omitted and performed/report modes.
  Development red/green logs under `.jj/k28-measure/` are not immutable matrix
  measurements or conformance receipts. The full source/instrument digests and
  controller validation remain required before those measurements.
- Spend the leaf's one narrow reviewer on failed-Started cleanup and
  first-signal/reader/finaliser composition. Both findings are valid and
  actionable: stale mask knowledge could permit disposition release after a
  failed final block; unknown-status recovery could silently preserve a foreign
  foreground without reporting unconfirmed modes/foreground. The first now has
  an observed failing development control and a passing correction. The second
  reporting correction still requires its physical PTY discriminator. No second
  reviewer has been used; executable controls must conclusively cover the fixes.
- Executable-entry fixtures must run the front on the actual initializer thread.
  The initial libtest-based controller correctly captured entry state but moved
  the front to a test thread, so thread-directed pending E02 signals remained on
  libtest's main thread. Reject those runs as E02 conformance evidence. The
  isolated cfg(test) binary now dispatches through its real main and records
  capturing/current thread identity; installed builds have no dispatch switch.
- On the measured Darwin host, E03's inherited ignored-SIGCHLD skip-repair mutant
  can still confirm both real waits: its raw no-child-wait flag is clear at exec.
  Do not invent an E03 baseline failure or call the private flag exec-inherited.
  The existing E04 private post-entry no-child-wait row is the independent failing
  pre-worker-repair discriminator in that domain. GNU/Linux and the Rust floor
  remain outside these local observations and still require qualification.
- The replacement private real-main entry run passed E01/E02/E03/E04/F01 in
  development. Both review corrections now have observed failing controls and
  corrected passing runs, including the physical foreign-foreground PTY case.
  These are construction evidence, not a frozen native-conformance campaign.
- The complete remaining instrument/controller/evidence work no longer fits
  this attempt's remaining producer balance. Keep the entire cross-layer repair
  live in this leaf, unsealed; do not retire a partial module or count development
  summaries as acceptance. `docs/native-entry-development-evidence.md` records
  the exact unfulfilled families, controls, provenance/receipt obligations and
  proposed remaining price. The manifest's mutable `phase` description was
  replaced with a purpose statement; its 153 row identities and 310 protected
  identities remain unchanged. No frozen measurement had begun.
- Conservatively charge this first attempt P≤70/E≤15/RC≤4, D=X=0, including
  source/control construction, failed and green development checks, the one
  reviewer, evidence/tree preservation and verification. These bounds include
  this attempt's ending work; they are not the full original allocation charged
  as spend. Cumulative bounds are D≤115/RC≤64/P≤225/E≤45/X=0. Remaining
  balances are D≤20 exclusively phase closure, RC≤1/P≤55/E≤75/X=0. A future
  attempt must add only its new charges, never charge this attempt again.
- Insert native-entry-qualification-funding-k29 before this leaf inside k22.
  Propose remaining P≤100/E≤70/RC≤10, D=X=0: exact shortfall P≤45/RC≤9,
  no D/E/X increase, proposed caps D≤135/RC≤74/P≤325/E≤120/X=0. The price
  includes the unchanged finite matrix, qualified bounded controllers, complete
  baseline/broken/repaired receipts, affected/protected checks, evidence and
  tree-owned review if needed. The proposal is not approval or spend; no
  renewed producer allocation is commissioned yet. Preserve reserved D, the
  support continuation and every root obligation. Force-stop leaves k28 live;
  no retirement, sealing commit or parent cascade is claimed.
- Fresh preservation checks observed a default-concurrency worker-startup timeout
  in the existing nofault-Unicode case, with actual cleanup confirmed. The same
  unchanged case passed alone and in sequential full runs. Retain all outputs;
  this does not prove the cause or qualify the native campaign. Sequential
  session (242), runner (78), workspace (320) and check (320) runs passed, with
  source digests unchanged during those checks.
- A clean-build control exposed a private-fixture prerequisite in the five new
  entry families. Mark only those new families explicit to test:native-entry;
  that task builds their private executable, includes ignored tests and excludes
  the two child-only roles. The revised task passed. Final ordinary check passed
  315 cases with the private executable hidden, retaining all 310 preexisting
  identities/ignored flags. This changes no required matrix acceptance.
- The tested cross-layer development increment is now an independently useful
  boundary; revise the earlier force-stop/unsealed plan. Decompose this item,
  keeping its entire acceptance contract in the k28 node, and retire/commit only
  its development child. Keep the full qualification producer live, preceded by
  k29 funding and a tree-owned review of the draft's partial-handler/final-mask
  composition. The one in-session reviewer is spent across this whole scope and
  must not reset through decomposition. No module is accepted separately.
- Replace this attempt's E≤15 charged bound with conservative E≤30, including
  failed/focused/sequential verification and fixture-prerequisite controls. This
  adds E≤15 only; P≤70/RC≤4, D=X=0 are unchanged and counted once across the
  development child/node. Cumulative bounds are D≤115/RC≤64/P≤225/E≤60/X=0;
  balances D≤20 exclusively phase closure/RC≤1/P≤55/E≤60/X=0. The unchanged
  remaining price P≤100/E≤70/RC≤10 now needs P≤45/E≤10/RC≤9 renewal:
  proposed caps D≤135/RC≤74/P≤325/E≤130/X=0. Full use leaves D≤20 reserved
  only. This replaces the earlier proposal; no renewal is approved or allocated.
- K29's human answered verbatim **“Approve the renewal (Recommended)”**: caps
  D≤135/RC≤74/P≤325/E≤130/X=0, no D/X increase, D≤20 closure reserve intact.
  After k29's conservative RC≤3 charge, cumulative bounds are
  D≤115/RC≤67/P≤225/E≤60/X=0; remaining D≤20 exclusively closure,
  RC≤7/P≤100/E≤70/X=0. K32 receives RC≤5, inspection only; any integration it
  earns and k31 share P≤100/E≤70/RC≤2, D=X=0, each charging actual work and
  passing the balance forward. The one in-session reviewer stays spent. This
  node's full acceptance contract, the unchanged matrix and the 30/5/5 envelope
  carry forward. Full use leaves D≤20 reserved/RC=0/P=0/E=0/X=0.

- K32's bounded source review found a late-runner-latch cancellation race;
  native-entry-development-k33 integrates that review before k31. Charge
  RC≤5, D=P=E=X=0, including review closure. Cumulative bounds are
  D≤115/RC≤72/P≤225/E≤60/X=0; remaining D≤20 exclusively phase closure,
  RC≤2/P≤100/E≤70/X=0 shared by k33/k31. Full acceptance, the fixed campaign,
  support qualification and all parent obligations remain live.

- K33 integrated k32's late-runner-latch finding: finalisation now consumes the
  runner latch after the final block, with two failing-then-passing private
  controls. Charge RC≤1/P≤10/E≤5, D=X=0. Cumulative bounds are
  D≤115/RC≤73/P≤235/E≤65/X=0; remaining D≤20 exclusively phase closure,
  RC≤1/P≤90/E≤65/X=0 for k31. Full acceptance, the fixed campaign, support
  qualification and all parent obligations remain live.

- K34 (inside k31) delivered the one qualified barrier controller (30/5/5, with its watchdog,
blocked-acknowledgement/failed-release and escaped-holder controls observed to
fail as designed), E06's 15 boundary joins and E07's four first-consumption rows,
each with its named break failing. These are development controls on
aarch64-apple-darwin, not the immutable campaign; no product defect was found and
no family is accepted. Charge P≤30/E≤15/RC≤1, D=X=0. Cumulative bounds are
D≤115/RC≤74/P≤265/E≤80/X=0 under caps D≤135/RC≤74/P≤325/E≤130/X=0; remaining
D≤20 exclusively phase closure, RC=0/P≤60/E≤50/X=0 for the rest of k31. The full
k28 campaign, supported-target qualification and all parent/root obligations
stay live; no node closes. K31's remaining children are k35 handoff/reader
controls, k36 fault families, k37 settlement controls and k38 campaign/receipts,
with a funding question k39 inserted ahead of k36 because the projected price
exceeds the remaining balance (a projection, not an allocation).

- K35 (inside k31) delivered E04's managed ignore-only mutant (at the runner,
observed to fail with the flag left set and ECHILD), E05's four direct/terminal
INT handoff rows on a real PTY with the invented-cancellation mutant detected,
and E08's four complete/escaped-reader rows with the controller owning and
killing its holder through an SCM_RIGHTS handoff and broken shielding failing the
eligibility receipt. These are development controls on aarch64-apple-darwin, not
the immutable campaign; no product defect was found and no family is accepted.
Charge P≤25/E≤10/RC=0, D=X=0. Cumulative bounds are
D≤115/RC≤74/P≤290/E≤90/X=0 under caps D≤135/RC≤74/P≤325/E≤130/X=0; remaining
D≤20 exclusively phase closure, RC=0/P≤35/E≤40/X=0 for the rest of k31. The full
k28 campaign, supported-target qualification and all parent/root obligations
stay live; no node closes. K31's remaining children are the k39 funding
question, k36 fault families, k37 settlement controls and k38
campaign/receipts.

- K39's human answered verbatim **“Renew full shortfall (Recommended)”** to the
  remainder recomputed from actual charges (P≤112/E≤84/RC≤13): add
  RC≤13/P≤77/E≤44, no D/X, so caps become D≤135/RC≤87/P≤402/E≤174/X=0, with
  the D≤20 closure reserve and zero production disruption intact. K39 charges
  RC≤3, D=P=E=X=0. Cumulative bounds are D≤115/RC≤77/P≤290/E≤90/X=0; remaining
  D≤20 exclusively phase closure, RC≤10/P≤112/E≤84/X=0, shared by k36–k38 and
  an optional RC≤5 tree-owned instrument review, each charging actual work. The
  unchanged matrix, amendments and 30/5/5 envelope carry forward; review
  integration, support qualification and narrower acceptance are not funded.
  The full k28 campaign and all parent/root obligations stay live.

K36 (inside k31) built the F02/F04/F05/F06 families, the four paired failures and
the two queued-TERM variants as development controls, each with a failing
broken subject, on aarch64-apple-darwin and not the immutable campaign. The
queued-TERM F05 rows found and repaired one product defect: after a failed final
pending query the incumbent finalisation restored the mask, so a queued TERM
killed the launcher under its default action before any report was written;
unknown pending state now keeps the handled signals blocked through settlement
(claiming no cancellation), with a unit control that fails without it. F06's
forked-child sites use the runner's off-by-default `native-faults` feature, only
in the isolated fixture target. The F03 paired row still uses the older worker
helper; routing it and C4's through the controller is k37's. Charge
P≤52/E≤24/RC=0, D=X=0. Cumulative bounds are D≤115/RC≤77/P≤342/E≤114/X=0 under
caps D≤135/RC≤87/P≤402/E≤174/X=0; remaining D≤20 exclusively phase closure,
RC≤10/P≤60/E≤60/X=0 for k37, k38 and an optional RC≤5 review. The full k28
campaign, supported-target qualification and all parent/root obligations stay
live; no node closes. K31's remaining children are k37 settlement controls and
k38 campaign/receipts.

K37 (inside k31) finished the settlement controls as development controls on
aarch64-apple-darwin, not the immutable campaign: S02's bound and identity
claims, F03/S03's report/deadline/identity claims, S04's seven recovery rows and
both S05 first-HUP-then-TERM rows, each with a named broken subject that fails
the claim it names, judged by named contract claims. The worker driver and C4
`pty_case` now run under the shared controller (one `poll` accessor added), and
a protected check exposed one instrument race in E05's acknowledgement, fixed
there. No product defect was found; no tree-owned review was cut and RC<=5 is
unspent. The 32 S identities (8/7/8/7/2) and 153 rows are unchanged and
individually reconciled in `docs/native-entry-development-evidence.md`. Charge
P<=30/E<=18/RC<=1, D=X=0. Cumulative bounds are
D<=115/RC<=78/P<=372/E<=132/X=0 under caps D<=135/RC<=87/P<=402/E<=174/X=0;
remaining D<=20 exclusively phase closure, RC<=9/P<=30/E<=42/X=0 for k38 and the
optional RC<=5 review. The full k28 campaign, supported-target qualification and
all parent/root obligations stay live; no node closes. K31's remaining child is
k38 campaign/receipts.

- K40 (inside k31) built the campaign tooling as development
  instruments on aarch64-apple-darwin, not the immutable campaign. Every native
  instrument now files one write-once record per judged (row, subject); a plan file
  expands the 153 rows to 327 expected records (143 rows with a failing broken
  subject, ten with a named reason); one driver freezes and digests every subject
  before and after, reconciles rows, tests and the 310 protected identities item by
  item and refuses a second writer; and each guarantee was seen to reject its
  deliberately wrong input. A development run reconciled 153 of 153 rows and 310 of
  310 protected identities (74 new), and `task check` passed 338 run/46 skipped.
  No product defect was found and production code is unchanged. Charge
  P<=25/E<=10/RC<=1, D=X=0. Cumulative bounds are
  D<=115/RC<=79/P<=397/E<=142/X=0 under caps D<=135/RC<=87/P<=402/E<=174/X=0;
  remaining D<=20 exclusively phase closure, RC<=8/P<=5/E<=32/X=0. K38 is now a
  node: the funding question, the incumbent subject with its baseline records and
  the frozen campaign (R01 per Taskfile task, docs reconciliation, support
  continuation) remain, and the projected price exceeds the remaining P. The full
  k28 campaign, supported-target qualification and all parent/root obligations
  stay live; no node closes.

- K41 (inside k38) found about 129 rows hostable on the incumbent once private
  seams are ported to operations it performs, and about 24 that target
  operations it never performs. The human answered verbatim **“I'm not actually
  concerned about the budget - just do what is required.”** K42 hosts every
  hostable row; nothing is narrowed. Budget caps no longer gate required work,
  so no further funding-question leaf is cut; charges are still recorded where
  charters ask, and X=0/zero production disruption still binds. K41 charges
  RC≤4, D=P=E=X=0. Bookkeeping: caps D≤135/RC≤88/P≤467/E≤177/X=0; cumulative
  D≤115/RC≤83/P≤397/E≤142/X=0; remaining D≤20 (phase closure),
  RC≤5/P≤70/E≤35/X=0 for k42 and k43. The full k28 campaign and all
  parent/root obligations stay live; no node closes.

- K44 (the first child of k42, inside k38) built the incumbent subject
  (`0515419edd29` exported from git, a private-seam patch, the repaired
  instruments copied verbatim and a baseline module), added the `baseline` class
  to the plan and driver with failing controls, and filed 46 front rows from a
  real run: 39 already-correct and 7 fails. Charge P≤22/E≤6/RC≤1, D=X=0, no
  reviewer spawned. Cumulative bounds D≤115/RC≤84/P≤419/E≤148/X=0 under caps
  D≤135/RC≤88/P≤467/E≤177/X=0; remaining D≤20 (phase closure),
  RC≤4/P≤48/E≤29/X=0 for k45, k46 and k43. K42 is a node (k44 landed; k45
  runner, worker and terminal rows and k46 fault rows remain; 107 rows
  unclassified). The full k28 campaign and all parent/root obligations stay
  live; no node closes.

- K45 (the second child of k42, inside k38) classified the 40 runner, worker
  and terminal rows from real incumbent runs: 30 observed (14 already-correct,
  16 fails) and 10 rejected premises (S01 x8, S02 always-running, S05 runner).
  With k44 the node stands at 24 fails, 52 already-correct, 10 rejected and 67
  unclassified (k46). The one in-session reviewer, spent here, corrected a
  first draft that had rejected S02, S03 and the S04 substitution on too
  narrow a premise; S02, S03 and S04 are hosted on the incumbent's own error
  paths. No product defect. Charge P≤20/E≤8/RC≤1, D=X=0. Cumulative
  D≤115/RC≤85/P≤439/E≤156/X=0 under caps D≤135/RC≤88/P≤467/E≤177/X=0;
  remaining D≤20 (phase closure), RC≤3/P≤28/E≤21/X=0 for k46 and k43. K42
  stays a node (k46 remains); the full k28 campaign and all parent/root
  obligations stay live; no node closes.
