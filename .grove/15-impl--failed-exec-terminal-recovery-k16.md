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
