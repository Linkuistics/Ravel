# failed-exec-terminal-recovery-k16

## Goal

Correct and check C4 on the incumbent launcher: retain the terminal lease across
failed spawn, recover foreground/modes under its admitted ownership premises,
and report every recovery failure without losing the original spawn cause or
cancellation. Deliver a complete runner-to-CLI repair with real PTY evidence,
protected regressions and a useful independently checked stopping state.

## Context

- `terminal-entry-funding-k15` records the human's renewed funding and shared
  RC allowance. `completion-failure-contract-k8` supplies the existing design;
  this is its implementation/check slice, not further design or comparison.
- `docs/specs/result-job-completion.md`, especially terminal recovery, outcome
  order, native launcher premises and test seams, owns the required behavior.
  `docs/completion-failure-design-evidence.md#choosing-local-corrections` and
  `#discriminating-checks` supply the proposed attempted-group pipe, source
  limits and fixed C4 controls. C4/native rows there are plans, not observations.
- C1–C3 are checked incumbent repairs. Their protected evidence and immutable
  receipts are linked from that evidence document. Enter from k14's committed
  repair `f8d765d2`; do not regress error-carried cancellation, worker admission,
  bounded diagnostic composition or identity-aware partial cleanup.
- Product preservation reference remains
  `0bdcb1f9ae24db5e9dd499308dd791744e908eb0`. Keep normal reference expectations
  outside the repair being judged; corrected C1–C4 failure expectations derive
  from the unchanged specification and human agreement, not the faulty baseline.

## Ownership and enabling points

The existing runner owns the terminal lease, saved modes, pre-exec handoff,
spawn outcome and supervision recovery. The front already owns error
classification, cancellation joining and result suppression. Use the existing
runner/PTY and public CLI/real-policy/fake-job boundaries; recovery-report
substitution belongs narrowly inside the lease owner, with no public switch.

Implement k8's bounded mechanism: prepare a private CLOEXEC pipe only for an
owned interactive lease; convey the attempted group in a fixed-size record
after group creation and before checked foreground handoff. Pipe/complete-record
failure refuses before handoff. Retain the lease/reader through the spawn result,
close the parent's writer on both paths, and keep pipe descriptors out of the
exec'd program. Restricted child work is async-signal-safe, uses precomputed
descriptors/fixed data, handles EINTR and does not allocate or take locks.

Failed-spawn handback is conditional on exclusive authority from observation
through handback: no competing foreground transfer and no unrelated reuse of
the recorded group identity. CLI lifetime supplies neither premise by itself.
The record is not a group-ID reservation after reap; never signal that ID on
the spawn-error path. Under the admitted premises, restore modes when already
foreground; reclaim the recorded attempt's foreground and restore modes; preserve
an unrelated holder and report unconfirmed recovery. Missing/malformed ownership
information or query failure cannot be reported as successful recovery. Report
each failed query, handback, mode restoration and temporary SIGTTOU restoration.

Retain the existing successful-launch/delegated-foreground policy and actual
reap/event order. Checked recovery failure also blocks otherwise successful
supervision/result settlement. Original spawn errno and consumed cancellation
survive all secondary reports and the front's no-ending path; preserve existing
missing-program 127/other failed-start 126 classification absent cancellation.
Started means spawn-reported success, never exec confirmation.

Verify the actual source, used Rust toolchain/target and relevant spawn/pre-exec
implementation before relying on k8's pinned library observations. This is
implementation verification of the existing design, not authority for a new
mechanism, universal toolchain claim or graceful recovery from library panic.
If the mechanism needs material redesign, preserve the coherent work and obtain
explicit discovery/design/review funding before proceeding.

## Fixed discriminating cases

- Real PTY failed exec after handoff: observe the actual changed foreground
  group before exec fails and retain a parent-observed handoff receipt. A missing
  executable keeps ENOENT/127, emits no runner Started/Reaped event for the
  reported spawn failure, and restores launcher foreground and saved modes.
  Distinguish the library's physical failed-child wait from runner notifications.
- Before-handoff failure and background/no-lease controls: no handback is
  claimed or performed without owned authority. Under the admitted serialized
  lease, a real unrelated foreground holder is preserved with a recovery report;
  this does not establish an atomic query/handback or prevent arbitrary ID reuse.
- Lease-local recovery-report controls: after real handoff/failed exec, separately
  report foreground-query, handback, mode and temporary SIGTTOU-restoration
  failures, including combined failures. Also cover recovery failure after
  successful supervision. Preserve original errno/error, first cancellation,
  every secondary report and result suppression through the actual consumer.
- Pre-exec signal death: use k8's private async-signal-safe barrier after actual
  handoff, deliver terminal INT while blocked, then allow entry-mask restoration.
  Spawn may report success although exec never happened; keep actual reap,
  group cleanup and terminal recovery on the supervision path. Child signal
  status is not invented launcher cancellation, and no publication still refuses.
  This case alone does not qualify the full native-entry contract.
- Successful-path/private-descriptor controls: normal interactive and
  noninteractive launches retain their existing behavior, with no attempted-group
  pipe descriptor in the exec'd job. Keep the distinction between a real physical
  operation and a substituted recovery report explicit in the receipts.

Every fixture separately restores usable foreground/modes through an uninjected
cleanup path, kills/waits its actual children, closes endpoints and bounds reader
completion on failures too. Test-only barriers/controllers must not run on the
production pre-exec path. First show each fault control detects unchanged broken
behavior or an explicit broken control; expectations cannot come from the new
helper being tested. Do not leave a mutated source active during measurement.

## Done when

- The complete runner/front behavior above is implemented through existing
  interfaces; no new component, policy convention, public testing API or package
  is selected. New private instruments remain bounded to the stated owners.
- The fixed PTY and recovery cases have discriminating measured observations.
  Keep source support, simulated reports, actual handoff/wait/mode observations
  and unexecuted target/toolchain assumptions separate.
- Existing CLI with real TypeScript policy/fake jobs, runner process/PTY and
  C1/C2/C3 controls remain protected. Cover configuration/SDK/settings/discovery,
  exact empty/Unicode/multiline bytes, first writer, nesting/anchored reads,
  result plus child failure, cancellation precedence/late joining, descendants,
  combined cleanup failures and final-write behavior. Use existing Taskfile
  tasks for focused checks and `task check`; no real model service is required.
- Record per-case results, commands/status, immutable before/after input digests
  and measured domain in the existing evidence document and a linked receipt.
  Finish edits before measuring; confirm protected outcomes item by item rather
  than matching totals. Evidence does not turn conditional lease premises into
  an unconditional ownership guarantee.
- Use at most one narrow fresh reviewer for this newly produced repair within
  remaining RC. Reconcile every finding; executable controls can conclusively
  verify fixes. A second review or material redesign requires separately funded
  work, not invented RC or the reserved D allowance.
- Record actual conservative P/E/RC charges including instruments, source,
  evidence, review, coordination, retirement and commit. Pass unused balances
  forward; allocations are not spend. Stop at the first applicable bound with
  a precise remainder and renewed human funding if required.
- Place only the next exact supported native-entry continuation before
  `book-structure-k5`. Worker and managed-child waitability, entry capture,
  ignored/blocked signals and checked handler/mask transitions remain required.
  If that slice cannot be stated/priced from existing design without new D/RC,
  grow a precise human funding leaf instead. Do not let the book advance past
  unresolved consequential entry obligations.

## Notes

K15's human agreement raises only the correctness RC cap from 30 to 35 minutes.
Its agreement/closure charges RC≤3, D=P=E=X=0. Entering cumulative charged
bounds are D≤70/RC≤33/P≤105/E≤20/X=0. Remaining are D≤20 (all phase-closure
reserve)/RC≤2/P≤135/E≤100/X=0. This leaf prospectively allocates
P≤65/E≤35/RC≤2, D=X=0, one focused session, zero production disruption.
Full use would leave D≤20/RC=0/P≤70/E≤65/X=0; unused allocation is not spend.
New discovery/design is unfunded. No allocation or new leaf resets any cap.

K4 retains its fixed reference, one iteration/seven attempts,
R=1/8,F=4/4,N=3/6,D≤45/RC≤15/P=E=X=0; k6 separately retains
D≤45/RC≤10/P=E=X=0. Carry G2 support/upkeep/route cost, G3 delivery/use beyond
entry, G4 unread alternatives, G5 runtime and R-W lifecycle/compatibility debt.
No comparison restart, target/migration, extraction, independent adoption,
simplification gain or root completion follows from this correctness slice.
The first consumer remains a one-session launcher CLI; a continuing host or
persistent runner has a separate contract. Root independent-capability delivery,
unchanged configuration/complete sessions, supported target/migration decisions,
the human book structure and the practical post-proof reader checkpoint remain.

## Decisions (running log)

- Adopt the already approved k8 mechanism without reopening design. Implement
  inside the existing runner lease; use private cfg(test) subprocess/PTY controls
  and existing front conversion/settlement checks. First reproduce the lost
  foreground against unchanged product behavior, then retain the private
  attempted-group pipe through spawn and report checked recovery errors.
- Used compiler: Rust 1.99.0 (b940084d7), aarch64-apple-darwin. The installed
  rust-src and official 1.99.0 Unix spawn source are available for verification;
  measurements will be limited to this host, not the supported target set.
- Implementation sequence: failing real-PTY control; lease/pipe/error repair;
  discriminating handoff, report, signal-death and descriptor controls plus actual
  front settlement; one narrow reviewer; frozen measurements and protected
  `task check`; evidence, exact continuation/funding and retirement/commit.
- The one narrow fresh reviewer reported a valid fixture timeout defect: killing
  only the fixture group could strand its separately grouped children. Close the
  child's unused barrier endpoints, bound its pre-exec poll, retain a waited
  child owner for the foreign holder, and request cooperative fixture cleanup
  before any emergency termination. Deliberate controller-failure,
  barrier-timeout and fixture-timeout controls now confirm physical waits and
  uninjected terminal cleanup. This executable seam settles the fix; no second
  reviewer is commissioned. No product-logic finding was reported.
- Recovery also retains a physical foreground-query failure during lending,
  before its later recovery query. The /dev/null ENOTTY discriminator failed
  before that correction and passes after it. This extends checked reporting
  within the existing lease owner, without a syscall abstraction.
- Parent/child actual handoff receipts, missing/malformed records, failed
  handoff, real pipe-preparation EMFILE, background/no-terminal controls, actual
  unrelated-holder preservation, report substitutions, error-carried first
  consumed cancellation and pre-exec pending terminal INT remain separate cases.
  The paired private test task consumes the runner's exact message/errno/signal/
  Started fields through the actual front conversion and no-ending settlement.
- An isolated jj workspace at k14's `f8d765d2` receives only the new public CLI
  PTY test. Its frozen run fails specifically because the missing-interpreter
  spawn keeps 127 but leaves a different foreground group. The first baseline
  setup selected zero tests (wrong copy source); that run is retained as setup
  failure and earns no behavioral credit. The corrected run preserves every
  product source from the incumbent. No active product mutation is measured.
- Protected C1 rejected an extra group-confirmation call after reported recovery
  failure. Restore the incumbent stopping order (recover, return its error with
  consumed cancellation; confirm group only after successful recovery), keeping
  the old assertion unchanged. The first final paired measurement also found
  the descriptor probe inspecting already-closed unused no-controller ends;
  distinguish those from the two always-live attempted-group ends. Both failed
  measurements are retained; neither earns final acceptance credit.
- Final frozen acceptance passes: `task check` (290 workspace cases and all
  deterministic checks, including paired C4), focused session 136, protected
  runner 11 and CLI 13. The final paired transcript has 22 ordinary PTY cases,
  eight explicit broken controls, 48 exact-error consumer precedence branches
  and three no-publication consumer branches. All 283 prior workspace PASS
  identities and each focused identity are individually confirmed, not inferred
  from totals. All measured before/after manifests match. The linked compressed
  receipt preserves failures as well as the final passing observations.
- Conservative actual charged bounds for this leaf are P≤35/E≤10/RC≤2 minutes,
  D=X=0, including instruments, source/toolchain verification, evidence, one
  narrow reviewer, protected measurements and coordination/retirement/commit.
  Cumulative correctness charges are D≤70/RC≤35/P≤140/E≤30/X=0. Remaining are
  D≤20 (all phase-closure reserve)/RC=0/P≤100/E≤90/X=0. Unused P≤30/E≤25 of this
  leaf's allocation is not spend. Neither closure reserve nor a new leaf funds
  native discovery/design/review; place the precise human funding continuation
  before the book and leave all consequential entry obligations explicit.
