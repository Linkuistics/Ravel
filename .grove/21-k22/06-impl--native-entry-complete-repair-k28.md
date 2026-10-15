# native-entry-complete-repair-k28

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
