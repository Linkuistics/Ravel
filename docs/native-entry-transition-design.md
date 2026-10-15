# Checked native-entry transitions

This is a design for the incumbent one-session launcher, not implemented or
runtime-qualified behavior. The [completion contract](specs/result-job-completion.md#one-session-native-launcher-entry)
owns the required behavior. C1–C4 retain their existing scoped receipts and the
conditional C4 foreground/group-identity premises. A persistent runner that
returns to arbitrary host work needs a separate contract.

## Ownership and interface

Keep two different values: the immutable executable-entry record (valid
signals, ignore/default state, calling mask, capture failure), and mutable
launcher transition state (installed handlers, current mask, cancellation and
ordered operation reports). Repairing the launcher never edits the child record.
The initializer records failures using fixed storage without allocation or
formatting; main converts the stored operation/signal/error into a refusal.

The launcher front owns a signal scope before policy evaluation and retains it
through selection failure, managed supervision and final reporting. Choice
returns evaluation data or failure without destroying that scope. The scope
does not require an allocator/channel to exist. It therefore covers early
preparation errors and no-ending paths as well as successful selection.

Worker evaluation owns its child and diagnostic readers. The runner owns
managed-child handlers, wait/reap, group cleanup and the terminal lease. Both
owners return original failures, consumed cancellation and secondary reports;
neither infers lifecycle success from a substituted report. Final result
eligibility belongs to the front.

Use a fallible version of the existing Started observation to restore the
front's temporary calling mask. Preserve the existing infallible observed entry
points through adapters. If Started observation fails, the runner already owns
a spawned child: it must perform bounded stop/group cleanup/wait and terminal
recovery before returning that transition error. It must not return directly
from the callback with a live child. Started still means spawn-reported success.
This extends an existing lifecycle seam; it is not a public testing option or
a new generic syscall service. The additive API is frozen in the repair brief: observed entry points gain a separate prompt, synchronous fallible start check; existing infallible entry points adapt with an always-successful check.

## Transition protocol

The handled set is INT, TERM and HUP. Entry ignores are never installed over;
entry-blocked signals are never counted as launcher cancellation. Every signal
set construction, query, membership operation and mutation is checked. A failed
query gives uncertainty, never an empty set or default disposition.

| Transition and owner | Required effect | Failure, retained protection and settlement |
|---|---|---|
| T0 initializer: capture | Read the entry mask and each supported valid disposition once, before Rust startup changes SIGPIPE. Publish the complete immutable record only after every required query succeeds. | Store first capture error and refuse start before worker/job creation. Do not publish a partially valid record. No host transition has yet been authorised. |
| T1 front: begin scope | Before any child/thread, block handled signals in the calling thread; install checked selection handlers for nonignored signals; restore its previous mask with those handlers present. | Failed initial block starts no children/threads. Partial handler install retains the installed subset and the known blocked mask for explicit finalisation; no destructor silently restores it. All later recovery attempts and pending-query errors are retained. |
| T2 front/evaluator: watching | Query current SIGCHLD; install checked default with a clean action mask/flags if ignored or SA_NOCLDWAIT is set. Do this before worker creation, and verify again before the managed child. Keep repair separate from child entry state. | No child at that boundary on failure; original error and each cleanup report reach finalisation. Successful repair is launcher-lifetime state and need not be restored for a continuing host. |
| T3 evaluator: readers | Block handled signals and save the actual previous mask before creating either output reader. All created readers inherit the shield and keep it until thread exit. Restore the creating thread's saved mask with handlers present. | Failed block creates no reader. Use fallible thread creation; if the second creation or creator-mask restore fails, retain the first reader, stop/wait the worker, collect only to the existing deadline and report all failures. Reader lifetime does not require an unbounded join or worker-descendant cleanup. |
| T4 front to runner | Block handled signals; query pending; join selection latch and eligible pending signals before calling the runner. While blocked, transfer handler ownership; clear only stale runner state before installing the new handlers. | If any block/query/handler operation fails, no managed spawn. Retain known installed dispositions and current mask; finalise without an ending. Pending signals cannot be erased by a latch reset after eligibility resumes. |
| T5 runner: spawn and Started | Child restores only valid changeable ignore/default dispositions, then the captured child mask. Each child operation is checked; pre-exec uses fixed data and async-signal-safe operations. On spawn-reported success the fallible observer restores the parent's saved mask while runner handlers remain present. | Child reinstatement failure is a spawn error with errno and C4 checked recovery. Failed parent restore after Started becomes an active-supervision failure: stop/group-clean/wait/recover before returning. Preserve the first consumed signal and every report. |
| T6 runner: supervision | First-signal latch uses compare/exchange, not overwrite. Direct INT to a transparent launcher cancels even with a terminal. Child-directed terminal INT remains child status. | C1 carries consumed signal through every observation/reap/recovery error. No recorded/reaped group identity authorises later signalling. Return the ending only for actual confirmed lifecycle facts. |
| T7 front: common finalisation | Success, selection failure, failed spawn and no-ending error all keep handlers through explicit resource cleanup and mask restoration. Snapshot/join front, runner outcome and late runner observations in that precedence, adding eligible pending signals without replacing a consumed signal. | Every failed transition prevents result success. Cleanup continues for all owned resources; the earliest operational failure and all secondary reports survive cancellation precedence. Failed mask/pending operations cannot be treated as confirmation that no cancellation exists. |
| T8 front: exit | With handlers installed, restore the entry mask, then reblock main and take the final pending/latch snapshot. Readers remain shielded. Check disposition release under that block; release the final mask only at the exit boundary, before any result write. | Partial finalisation retains known protection and reports uncertainty; no result. Cancellation reports first, then resets the selected signal to default, unblocks that signal and raises it; check each step and use numeric fallback if signal death cannot be established. A newly arriving signal after the settlement decision follows the exit disposition; stdout has no atomic rollback. |

The T8 mask restoration/check cycle runs with handlers installed; the final
release to ignore/default dispositions occurs while main is blocked. On a
noncancelled path, final mask release is checked before stdout. If release
fails, retain the handlers that still exist, report the failure and exit without
a result. On cancellation, keep other handled signals blocked while resetting
and unblocking the selected one, so a later different pending signal cannot
replace the selected cancellation. There is no claim to recover from repeated
OS failure: uncertainty is reported, a numeric nonzero fallback exits, and the
process lifetime ends. No new child is admitted after any failed transition.

At no point does a surviving reader become eligible: shielding is permanent
for that thread. Collection may time out when a policy descendant holds a pipe;
the CLI settles and exits, terminating its own reader threads. It does not promise
to reap that descendant. Receipts must distinguish completed readers from still
blocked readers and prove the creating thread regained its mask.

## Alternatives and reason for the choice

Keeping the unchecked incumbent cannot support the stated failure contract.
Moving all cancellation and policy execution into the runner would require the
runner to know policy refusal, output/channel cleanup and CLI result precedence.
The chosen owner-local checks and one common front finaliser retain current
responsibilities and the existing worker/runner seams. A dedicated signal thread
would introduce another lifetime and routing protocol without removing the
need to shield readers or restore child state. It is outside this repair.

## Source evidence and support domain

Tier 2 graph discovery and an untruncated depth-one front trace (35 returned
callees) located the material paths. Exact source was read for the initializer,
cancellation, choice, worker capture, front, main and runner. Coverage generation
2026-10-15T03:01:03Z reports matching metadata and no recorded issues for these
Rust files; this is best effort, not completeness or execution evidence.

- [Entry capture](../crates/ravel/src/signal_state.rs#L66) treats failed action
  queries as nonignored and can still mark capture complete.
- [Choice](../crates/ravel/src/choice.rs#L61) currently owns/drops selection
  handlers on failure. [Front settlement](../crates/ravel/src/run.rs#L35) has a
  separate selection-failure branch and discards Started mask-restoration error.
- [Reader creation](../crates/ravel/src/worker.rs#L1226) already uses shielding;
  [the shield](../crates/ravel/src/cancellation.rs#L191) checks neither mask call.
  Collection at its existing deadline does not join potentially live readers.
- [Runner startup](../crates/keyed-launch/src/run.rs#L1031) installs handlers and
  repairs SIGCHLD after policy evaluation. Its repair tests ignore, not the
  independent flag; [child reinstatement](../crates/keyed-launch/src/run.rs#L464)
  discards action/set-building failures. Runner handler stores currently replace
  an earlier signal.

For Darwin arm64, the explicit signal domain is 1–31. For the declared GNU Linux
arm64/x86_64 targets it is 1–64 excluding glibc-reserved 32 and 33; KILL/STOP
are queried where supported but never changed or inserted as blockable signals.
Use the same domain for capture and reinstatement. An unexpected error querying
a required valid number refuses; a blanket EINVAL exemption would hide faults.
The producer verifies these domain assumptions against its target libc headers.

Linux v6.18 [exec](https://github.com/torvalds/linux/blob/v6.18/fs/exec.c#L1177)
calls [flush_signal_handlers](https://github.com/torvalds/linux/blob/v6.18/kernel/signal.c#L489),
which clears action flags even when preserving an ignore. Thus a standalone
SA_NOCLDWAIT case is a private post-entry fixture on that source domain, not an
exec-inherited Linux flag case. Actual executable ignored-SIGCHLD remains a
separate test. This source observation is not a Linux runtime qualification.
Apple's [execsigs source](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_sig.c)
resets caught actions but represents no-child-wait partly as process state.
This read does not establish standalone-flag survival on the measured Darwin
kernel. Claim no Darwin exec-inherited SA_NOCLDWAIT coverage: first record the
raw initializer flag in an executable-entry fixture; classify a cleared flag as
not inherited, never relabel the private post-entry fixture as inheritance.
Both platforms require private default-plus-SA_NOCLDWAIT watching controls.

Current local compiler is rustc 1.99.0, commit
b940084d7eb6a299eb4bfeb8e34901bc051e7ac4, aarch64-apple-darwin. K18's installed
std startup/exec source read is retained. Both digests were rechecked unchanged:
startup SHA-256 6c706c9a79d6a6ca3870a605800d7a74ba98cf41764c12cd4538a37812bba6aa;
exec SHA-256 1c0f874f1835e5befa6e8bdd65450903f475248b525d3b7b5e1ce861311a21d0.
Minimum Rust 1.85 and GNU Linux arm64/x86_64 remain unexecuted domains.
No broader host/toolchain support is established by this design.

## Frozen discriminator manifest

These are exact parameter families, not implemented tests. The producer freezes
their expansion before red/green, with one immutable row per concrete parameter.
Use public CLI/real policy/fake jobs for end-to-end cases; owner-local forked
processes for unreachable transitions; existing real-process/PTY seams for jobs.

| IDs and parameters | Required receipts and meaningful broken control |
|---|---|
| E01(signal INT/TERM/HUP/PIPE × disposition default/ignore × mask clear/blocked) | Controller sets state immediately before exec; raw initializer and managed child's first-code action/mask receipts agree. PIPE uses a raw/native helper before a helper runtime resets it. Break capture or child reinstatement separately; relevant parameter must fail. |
| E02(signal INT/TERM/HUP × pending yes/no, blocked at entry) | Controller queues while blocked before exec. Launcher must not cancel; child mask remains blocked. Pending is not promised to cross fork: parent pending and child blocked-state receipts are distinct. Control that counts entry-blocked pending as cancellation fails. |
| E03(SIGCHLD default/ignore) | Raw executable entry plus actual worker PID/wait and managed PID/group/wait receipts; both waits confirmed before admission/result. Disable pre-worker repair and show ignored case fails. |
| E04(default+SA_NOCLDWAIT, worker/managed boundary) | Private post-entry action fixture, raw query confirms flag immediately before checked watching. Real child exit/wait receipts; broken ignore-only repair fails. Darwin raw exec-flag observation is a separate reachability row, not proof borrowed from this fixture. |
| E05(direct launcher INT / PTY foreground INT, before/after handoff) | Controller uses actual launcher PID versus terminal VINTR write, records SID/PGIDs/tcgetpgrp and acknowledgements. Direct INT cancels; after-handoff terminal INT kills default child. Publish before child death distinguishes child status from invented launcher cancellation. No sleeps as ordering oracle. |
| E06(boundary selection-to-runner / Started / after-reap-before-front / selection-failure-before-finalise / no-ending-error-before-finalise × signal INT/TERM/HUP) | Fixed private acknowledgement barrier places actual signal at named boundary; record handler owner, main/reader masks, pending and consumed facts, child/reap/resource state and zero result. Remove the corresponding join or overwrite first latch and require failure. |
| E07(first TERM then HUP / first HUP then INT, runner-active and runner-error) | Barrier acknowledges consumption of first before second is sent; first survives outcome/error and late latch. Broken store-overwrite fails. No arrival-order claim for simultaneously pending standard signals; fixed INT/TERM/HUP order is used only when no consumed observation exists. |
| E08(reader completed / descendant-held-pipe reader still alive, success and selection failure) | Reader's actual mask receipt, creator before/after mask, bounded collect-deadline and reader state; send process-directed TERM at finalisation barrier. Main owns cancellation; no unbounded join. Controller owns and kills/waits its deliberately escaped pipe-holder. Break shielding and verify eligibility receipt fails. |
| F01 capture(mask query / valid action query / membership); F02 watching(query / repair) at worker and managed boundary | Finite private owner-local reported faults; capture faults injected into the raw initializer result boundary, not public environment switches. Before-child receipt proves no admission; original failure plus every report retained. Actual state queries/fixture cleanup separately recorded. |
| F03 shielding(block / first thread create / second thread create / creator restore); F04 selection and runner handlers(query / first install / later install / final restore) | Fixed masks and partial ownership receipts demonstrate no unshielded creation, no child after failed admission and retained subset protection. A successful physical operation followed by a substituted error is labelled separately from an operation deliberately omitted to model failed mutation. |
| F05 transition(block / pending query / membership / Started restore / final restore), selection-success / selection-failure / no-ending | Failed pending query never means no cancellation; actual queued TERM retained where query remains available. Started restore failure must produce bounded real reap/group/terminal cleanup, not an early return. Every failure prevents bytes; retain original refusal/spawn errno and secondary labels. |
| F06 child(action / mask-set construction / mask restore); F07 re-raise(default action / unblock / raise) | Child fault uses fixed pre-exec data and reports actual spawn/wait/C4 recovery. Re-raise faults execute in forked fixtures; parent distinguishes WIFSIGNALED from numeric 128+N fallback and asserts zero result. Disable the relevant check and require the report/result assertion to fail. |
| R01 protected existing expectations | Untouched CLI/configuration/exact UTF-8/empty/nested/first-writer/final-write, C1–C4 and runner process/PTY cases run through existing Taskfile tasks. Preserve physical operation versus substituted-report distinctions and C4 exclusive authority/no-reuse premises. |

Every barrier has a deadline and two-way acknowledgement. The frozen controller envelope is 30 seconds per case, including cleanup,
with each acknowledgement bounded by 5 seconds and the final 5 seconds reserved
for controller cleanup. This replaces the earlier illustrative 20-second recipe.
On timeout use only still-owned identities, bounded nonblocking wait and explicit
unresolved-identity reports; never extend the outer deadline. PTY owner saves/restores foreground/modes and closes
both ends; fixture PID handles remain owned through cleanup. Do not kill by a
recorded group number after direct-child reap. Tests involving escaped pipe
holders retain their own direct-child ownership. The producer may shorten
deadlines but may not replace receipt ordering with sleeps.

Freeze source revision, case expansion, toolchain/libc/kernel versions, runner
commands, protected expectations and exclusion notes before measuring. Digest
all subjects before and after each run, including fixtures/manifests/notes.
Use separate immutable per-case receipts for baseline, deliberately broken
control and repaired run. Every claimed repaired case needs a matching failing
control, with actual operations and substituted reports in separate fields.
Compare reruns row by row. Changing an instrument invalidates the affected run.
No product or behavioral control is built or executed by this design session.

## Implementation prerequisite: bounded error settlement

Implementation source verification at reference
`f3756b2de573133d9f70046928c61efdc075c5ce` found that the existing error
settlement cannot supply the bounded operation assumed by T3/T5 and the price
below. No production change or native fault case was executed by this read.
The required behavior and frozen additive Started interface remain unchanged.

| Existing path | Source observation | Consequence for the complete repair |
|---|---|---|
| Runner emergency wait | [watch](../crates/keyed-launch/src/run.rs#L1556) calls `signal(SIGKILL)` then blocking `reap`; [Child::reap](../crates/keyed-launch/src/run.rs#L1387) delegates to `Child::wait`. | Failure to deliver the signal, or delayed exit despite it, can prevent reaching terminal recovery. This is not a bounded cleanup primitive. |
| Runner post-kill polling | [Watch::Killed](../crates/keyed-launch/src/run.rs#L1599) has a polling interval, but no exit deadline. | A short polling interval is not a bound on cleanup. The one-second group-confirmation deadline applies only after reap. |
| Runner stop reports | [Process](../crates/keyed-launch/src/run.rs#L1347) returns no result from signalling or group killing; [kill](../crates/keyed-launch/src/run.rs#L1658) discards both syscall results. | Reusing this route loses required secondary reports; its emergency message claims SIGKILL was sent without confirmation. Checked stop outcomes are needed. |
| Worker cleanup | [stop](../crates/ravel/src/worker.rs#L1018) bounds its pre-kill grace, then [kill_and_wait](../crates/ravel/src/worker.rs#L1007) records a kill error and calls blocking `Child::wait` anyway. | A reader-creation failure cannot rely on this as a bounded worker wait. Existing collection deadlines do not bound the preceding wait. |
| Unknown-status terminal recovery | [Lease::reclaim](../crates/keyed-launch/src/run.rs#L863), with `status=None` and `failed_spawn=false`, takes foreground only from its direct child's group. A different holder can produce no report while modes remain unrestored. | Do not silently turn this into confirmed recovery. State the policy for unknown status and report unconfirmed recovery without granting new foreground authority. |

Rust documents that [Child::wait blocks until exit](https://doc.rust-lang.org/std/process/struct.Child.html#method.wait).
The source observations above establish possible unbounded paths and discarded
reports; they are not runtime demonstrations of a denied kill or kernel-stuck
child. Nor do they invalidate the narrower prior C1–C4 receipts. Those receipts
separate actual waits from substituted failure reports and do not establish a
deadline for every physical wait.

One fresh read-only reviewer inspected the frozen Started contract against the
runner and its existing wait-event tests. All findings were reconciled against
the exact source:

| Finding | Classification and disposition |
|---|---|
| Blocking emergency wait and deadline-free killed state cannot supply bounded cleanup. | **Valid and actionable.** An explicit error-settlement deadline and uncertainty outcome must be specified and priced before reuse. No deadline was selected by that prerequisite read. |
| Stop and group-kill failures are discarded. | **Valid and actionable.** Preserve individual operation reports alongside the initiating check error and first consumed cancellation. Existing LaunchError aggregation is sufficient for carrying text; it does not make the stop operations checked. |
| Emergency settlement omits the normal second group-kill attempt and post-reap group confirmation. | **Contract stated unclearly in the review prompt.** The existing emergency branch does attempt one group signal, so it is not evidence of no group-stop attempt. It supplies neither the ordinary repeated-kill sequence nor a group-absence receipt. Decide the required pre-reap stop protocol; error return may explicitly retain uncertainty and need not invent Group::Gone. Existing protected wait-event traces must remain reconciled individually. |
| Unknown-status reclaim can leave a foreign foreground holder and saved modes without reporting uncertainty. | **Valid and actionable for reporting.** Arbitrary takeover is not authorised. Preserve the existing C4 authority and no-reuse premises, and distinguish recovery attempted from recovery confirmed. |

The reviewer also identified reusable mechanisms: establish child/group/lease
ownership before Started; attach the callback's io::Error as source without
setting spawn errno; retain consumed cancellation through the existing typed
accessor; append recovery diagnostics; emit Reaped only for actual successful
wait. Entering ordinary watching after a failed check would call `lend` again,
contrary to the frozen error-path requirement.

K23 left the following decisions for the selected protocol below: the bound
and nonblocking observation strategy after
failed Started or worker-reader admission, treatment of stop/observation/wait
failures and EINTR/ECHILD without authorising signalling an uncertain or reaped
identity, the pre-reap group-stop sequence, and unknown-status terminal recovery
reporting. Their discriminators must distinguish physical operations from
substituted reports and fit the existing 30/5/5-second controller limits. The
frozen matrix has no newly authorised cleanup-failure expansion. Specify and
price any necessary amendment explicitly rather than silently adding cases.

Tier 2 graph lookup and exact source reads support this bounded claim. Coverage
generation `2026-10-15T03:17:48Z` matched metadata with no recorded gaps for the
material Rust paths and wait-event tests; this remains best effort. The worker
kill_and_wait trace resolved callees to unrelated similarly named functions, so
its implementation and actual Child calls were checked directly, not inferred
from those callee edges. The excluded `.cargo/config.toml` was read directly.
No exhaustive graph claim is made.

The complete repair remains live as one future producer. The material discovery
triggers the mandate's scope/funding continuation; it does not permit borrowing
the D≤20 phase-closure reserve or declaring the native repair complete.

## Bounded error-settlement protocol

This is the selected mechanism for the agreed failure contract, not implemented
behavior. Keep the frozen Started API, T0–T8, first consumed cancellation and
spawn-only errno classification. The callback io::Error remains the source;
cleanup reports never replace it. No result or new admission follows a failure.

### Deadlines and ownership

Use monotonic absolute deadlines, never restarted by EINTR, another signal or a
secondary failure. Runner error settlement has **2 seconds**, beginning when a
Started check fails or the first non-EINTR observation/wait/stop error occurs.
The ordinary Killed state also has a 2-second deadline beginning with its first
SIGKILL attempt; expiry becomes an error. A normal observed exit starts the same 2-second budget before its pre-reap
group stops if no deadline exists yet; every logical reap therefore has a
deadline even when it first encounters EINTR. If budgets overlap, use the earlier
deadline. This does not shorten configured normal channel grace or kill-grace.
The existing 20ms repeat-group-stop pause is inside this bound. Existing
post-reap group confirmation remains at most 1 second, query-only, after terminal
recovery; it is not a renewed child-wait budget. A recovery failure still skips
confirmation as the protected traces require.

Worker teardown has **2 seconds total** from reader-admission failure (or entry
to stop on other failure): retain the existing at-most-1-second graceful wait,
then SIGKILL and nonblocking observation for only the remainder. An observation
error may skip the remaining grace but cannot restart the deadline. Signal only
the worker PID: it shares the launcher's group. Diagnostic collection retains
its shared 1-second DRAIN_GRACE, beginning after teardown, capped at t0+3 seconds.
No reader join is required. The worker evaluation deadline remains unchanged.

At expiry attempt terminal recovery immediately, retain known child/group/lease
facts until that attempt and the final cancellation sample, then return an error
naming unresolved ownership. Rust Child drop is neither reap nor termination;
a surviving child/group may remain for the caller/operator after CLI exit.
There is no background reaper or resumed-host guarantee. Disarm all later numeric
signals once ownership is released or uncertain. Report attempted stop, confirmed
reap, group absence and foreground/mode recovery independently. No timeout may
manufacture Reaped, a status, Group::Gone or successful terminal recovery.
The deadlines bound user-space polling/retries, not scheduler stalls or an OS
syscall that never returns. Use nonblocking wait and TCSANOW recovery: no physical
Child::wait, blocking waitpid, terminal drain or unbounded retry reaches recovery.

### Checked operations and identity

Retain the Process seam and worker-local teardown boundary. Checked signalling
returns an ordered report for each group and direct-PID syscall, with target,
signal and errno. A successful kill means delivery was attempted successfully,
not exit. ESRCH is a no-target report, never a reap receipt; EPERM is retained.
Do not infer group absence from stop success. Group probes distinguish ESRCH
(absent), present/EPERM, and other query errors (unknown with the original errno).

Production runner observation uses waitid(WEXITED|WNOHANG|WNOWAIT), keeping the
unreaped child's PID reserved until all required group signals finish. The
logical reap operation uses waitpid(WNOHANG) under that same owner, decoding a
status only when the expected PID is returned. Poll at at most 10ms intervals,
clipped to the absolute remainder. Worker uses Child::try_wait under its sole
owner. A zero/None observation is still running, not an error or reap. EINTR is
recorded in the operation receipt and retried only within the same deadline,
without accumulating an unbounded diagnostic string; retain first errno and
retry count. Other errors remain secondary to the initiating failure. ECHILD
invalidates reservation immediately: no subsequent signal, no status or Reaped,
report unknown lifecycle and proceed to conditional terminal recovery. A wait
error that does not establish identity loss allows bounded retry while exclusive
wait ownership remains; a private substituted error must retain the separately
recorded physical wait fact and can never restore a released reservation.

Failed Started uses checked group-then-PID SIGKILL, followed by the existing two
group-only SIGKILL attempts separated by 20ms, all before reap and only while
reservation remains. Observe unreaped exit and bounded reap; if observation is
unavailable, the already completed stop sequence permits direct nonblocking reap.
Never call lend again. Legacy emergency observation failure retains its existing
single group-then-PID stop followed by logical wait, rather than adding a second
kill-group event to protected traces. This is a visible trade-off: uncertainty
can be returned; repeated group stops and confirmed absence are not claimed on
that legacy error path. Normal observed exit retains two group stops before reap.
At any confirmed reap, freeze elapsed and consumed cancellation before emitting
Reaped once, then recover. No signals follow confirmed reap, including after a
substituted failed report. On no-reap expiry sample cancellation before recovery;
T7 joins any later runner/front/pending observations without replacing the first.

### Unknown-status terminal recovery

Keep C4's exclusive foreground authority and no unrelated recorded-ID reuse
through handback premises, including after ECHILD/release. Query foreground;
if already owned, restore saved modes. If it equals the owned recorded child
group and the premises hold, attempt handback, query again, then restore modes
only if the launcher is confirmed foreground. A foreign holder with unknown
status is preserved and explicitly reported as unconfirmed foreground AND modes;
unknown status never enables the broader signal-death delegated-group rule.
Query, handback, saved-mode restore and temporary SIGTTOU restoration reports all
survive. A successful mode syscall is its operation confirmation, not a claim of
universal terminal recovery. Each terminal operation is attempted once, with EINTR retained rather than an
unbounded retry. Missing modes or ownership facts are errors. Failure
cannot lead to a new lend or successful result. No-lease launches require no
foreground operation. Existing known-status C4 behavior remains unchanged.

### Protected wait-event reconciliation

These are exact existing logical traces, not permission to use blocking waits.
The real adapter performs bounded physical polls inside the logical wait; new
operation receipts remain distinct from these existing event assertions.

| Existing case | Preserved trace/meaning |
|---|---|
| confirmed_reap_precedes_recovery: poll | poll → kill-group → wait → reaped → recover → confirm |
| same: recovery, confirmed / failed | poll → kill → wait → reaped → recover / poll → kill → wait → recover; no confirm |
| same: escalation, confirmed / failed | poll → poll → term → poll → kill → poll → kill-group → wait → [reaped only if confirmed] → recover → [confirm only if confirmed] |
| cancellation_forwards_or_kills_by_mode | interactive/noninteractive forward → poll → kill → poll → kill-group → wait → confirm; confined kill → poll → poll → kill-group → wait → confirm |
| check_wait_failure variants | optional forward → poll → kill (poll failure) or kill-group (observed exit) → wait → optional reaped → recover; preserve each early/at-reap signal tuple and both original reports |
| surviving group | Keep actual status beside Group::Present; no invented absence. |
| after-reap signal / elapsed / second sample | Freeze ending duration before observer/recovery; recovery signal stays late; drain reap sample without replacing the first. |
| reported recovery failure | Retain earlier observation/reap and cancellation reports, leave recovery signal late, and skip confirm in every failed-watch/failed-reap tuple. |
| error annotations | Preserve first cancellation, original spawn-only errno and every diagnostic. |

### Finite cleanup-failure amendment

Propose **S01–S05 only**, single-site rows, no Cartesian expansion with E/F/R.
This is an explicitly priced amendment awaiting funding/commissioning; k24 did
not authorise its construction. Existing F03/F05 real-process/PTY controls cover
the healthy cleanup path; S adds only the missing uncertainty discriminators.
Controls are private fixed enums at Process/worker/Lease owners, never installed
switches or a generic syscall facade. Use the existing CLI with real policy/fake
job, real-process and PTY boundaries. Each row records actual physical operation,
substituted report, monotonic timestamps, PID/group reservation state, cancellation,
all reports, Reaped count, foreground/modes and fixture cleanup separately.

| ID | Exact parameters and meaningful broken control |
|---|---|
| S01 | Failed Started, group-first kill / PID kill / first repeat-group kill / second repeat-group kill; each successful physical operation+reported EPERM and omitted operation+EPERM (8 rows). Actual other stops remain; break by discarding that report. |
| S02 | Failed Started, WNOWAIT observation EIO once / EINTR until deadline / ECHILD; final wait EIO once / EINTR until deadline / ECHILD; plus always-running until deadline (7 rows). Fixed real-operation receipt plus substituted outcome, never claim physical ECHILD or stuck kernel. On simulated release forbid further signals; break deadline or identity guard as appropriate. |
| S03 | Worker second-reader-create EAGAIN plus PID stop EPERM / kill EPERM / try_wait EIO / EINTR until deadline / ECHILD / always-running (6 rows). Stop/kill each have omitted and performed+reported modes (8 rows total). Break each report check, absolute deadline or identity guard. Retain original creation failure and every report, no Selected/start. |
| S04 | Unknown status after failed Started: foreground own / recorded child / unrelated group / foreground query EIO / handback EIO / mode restore EIO / SIGTTOU restore EIO (7 rows). Physical PTY operations then substituted EIO for error rows; existing C4 omitted-operation coverage remains. Unrelated holder is a controller-owned group under the C4 premises. Break the corresponding confirmation/report or foreign-holder guard. |
| S05 | Failed Started plus first consumed HUP then TERM at settlement, once with S02 always-running and once with S04 mode-report failure (2 rows). Acknowledge first consumption; break overwrite/report retention. |

The amendment has 32 concrete rows. Every row runs baseline, its named broken
control and repaired subject; a baseline already correct is reported as such.
No simulated error is labelled an actual failed physical wait. Controllers keep
independent direct-child handles for all real fixtures and escaped pipe holders;
they clean physically after product ownership is released, without reusing a
numeric group. Validate watchdog expiry, blocked acknowledgement and holder-cleanup
controls. Broken unbounded waits are killed by the outer controller, not awaited.

Reserve 0–25 seconds for setup/acks/case and 25–30 for cleanup; acknowledgements
each consume the SAME outer budget, never serial additions. Error settlement
uses at most 2 seconds, worker drain at most 1, runner confirmation at most 1.
Fixed ordinary fixtures use existing production grace/kill-grace values and must
show their sum plus setup/ack/confirmation fits the first 25 seconds in the frozen
manifest. Arbitrary legal user-configured grace values need not fit: if selected,
the exact remainder is max(0, setup+ack+graces+2+1−25) seconds, priced before any
run; no such long-grace row is commissioned here. Controller cleanup sends TERM
only while owned, uses at most 1 second before KILL, then WNOHANG until the 5-second
cleanup deadline. If even independent cleanup cannot confirm, record failure and
unresolved fixture identity, never a passing receipt or unbounded cleanup wait.

### Focused design review

One fresh-context reviewer challenged boundedness, identity safety and report
retention against the frozen contract and returned “No concrete finding.” No
finding requires classification and no second read was launched. This is one
aimed review, not measured execution or a proof of all OS behaviors. Tier 2
graph generation 2026-10-15T03:45:25Z reports matching metadata and no recorded
issue for the runner, worker and protected wait-event source; exact source was
read as well. Coverage remains best effort.

### Alternatives and reversal condition

Keeping blocking wait cannot bound recovery. A reaper thread would introduce a
second wait owner and a lifetime beyond error return without guaranteeing physical
reap; reject it here. Owner-local nonblocking wait preserves reservation and the
existing seams, at the cost of explicit unresolved-child errors. Reverse this
choice only if an actual supported-platform wait primitive cannot satisfy these
nonblocking ownership semantics; that earns a precise design continuation rather
than an implementation workaround or new target infrastructure.

## Complete repair price and handoff

The old P≤95/E≤80/RC≤4 estimate is replaced by the following complete proposal;
none is commissioned. Prices are conservative effort ceilings in minutes, not
measured spend. They cover the whole coherent repair, never just settlement.

| Work | P | E |
|---|---:|---:|
| Capture/domain and front scope through choice failure | 20 | 0 |
| Checked worker watching, readers and bounded teardown | 20 | 0 |
| Runner additive Started, child checks, checked stops/nonblocking settlement | 30 | 0 |
| Common finalisation, uncertainty, terminal and cancellation composition | 15 | 0 |
| Frozen E/F/R plus 32 S rows, barriers/cleanup controls, immutable manifests | 30 | 0 |
| Evidence and item-by-item subject/receipt reconciliation | 10 | 0 |
| Frozen E/F baseline/broken/repaired measurements | 0 | 35 |
| S baseline/broken/repaired and cleanup-controller validation | 0 | 15 |
| Protected test:list identities, CLI/session/runner/terminal/workspace/check | 0 | 25 |
| Bounded affected-row reruns and receipt reconciliation | 0 | 10 |
| **Complete producer** | **125** | **85** |

Fund producer RC≤5 for one focused review, coordination and retirement/commit;
mechanical planning/handoff D≤5/RC≤1; funding agreement recording/closure RC≤4.
After k25's conservative D≤10/RC≤2 charge, cumulative bounds are
D≤110/RC≤55/P≤155/E≤30/X=0 and balances D≤20 exclusively phase closure,
RC=0/P≤85/E≤90/X=0. The complete next cycle requires D≤5/RC≤10/P≤125/E≤85,
so the precise shortfall is **D≤5/RC≤10/P≤40**, with no additional E or X.
Recommend caps D≤135/RC≤65/P≤280/E≤120/X=0, retaining the original D≤20
reserve. Agreement charges precede allocating planning; actual unused allocation
is not spend. Cut only the funding/scope continuation inside k22 before the book.

Minimum Rust 1.85/Linux runtime qualification and the returning-host contract
remain outside this local repair price and remain live obligations; no phase
closure is earned. Source support is Darwin-host design evidence, not execution
of native cases. No product, fixtures or behavioral suite is built/run here.
