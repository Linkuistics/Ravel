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
a new generic syscall service. The exact additive Rust name belongs to planning.

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

Every barrier has a deadline and two-way acknowledgement. Outer controller
deadline is 20 seconds for one case; on timeout send TERM, then KILL after at
most 2 seconds, wait at most 5 seconds and report unresolved identity rather
than signal a reused group. PTY owner saves/restores foreground/modes and closes
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
| Blocking emergency wait and deadline-free killed state cannot supply bounded cleanup. | **Valid and actionable.** An explicit error-settlement deadline and uncertainty outcome must be specified and priced before reuse. No deadline was selected here. |
| Stop and group-kill failures are discarded. | **Valid and actionable.** Preserve individual operation reports alongside the initiating check error and first consumed cancellation. Existing LaunchError aggregation is sufficient for carrying text; it does not make the stop operations checked. |
| Emergency settlement omits the normal second group-kill attempt and post-reap group confirmation. | **Contract stated unclearly in the review prompt.** The existing emergency branch does attempt one group signal, so it is not evidence of no group-stop attempt. It supplies neither the ordinary repeated-kill sequence nor a group-absence receipt. Decide the required pre-reap stop protocol; error return may explicitly retain uncertainty and need not invent Group::Gone. Existing protected wait-event traces must remain reconciled individually. |
| Unknown-status reclaim can leave a foreign foreground holder and saved modes without reporting uncertainty. | **Valid and actionable for reporting.** Arbitrary takeover is not authorised. Preserve the existing C4 authority and no-reuse premises, and distinguish recovery attempted from recovery confirmed. |

The reviewer also identified reusable mechanisms: establish child/group/lease
ownership before Started; attach the callback's io::Error as source without
setting spawn errno; retain consumed cancellation through the existing typed
accessor; append recovery diagnostics; emit Reaped only for actual successful
wait. Entering ordinary watching after a failed check would call `lend` again,
contrary to the frozen error-path requirement.

The missing decisions are the bound and nonblocking observation strategy after
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

## Initial repair estimate and handoff

**This estimate is suspended pending the bounded-settlement prerequisite above.**
It records the original design price, not a current complete supported price.
Implementation has not commissioned a narrower replacement repair.

One focused fresh-context reviewer read the ownership/transition protocol and
relevant cases against the required signal/failure contract. It returned
“No concrete findings within the assigned transition-review scope.” There are
no findings to classify. This is one narrow read, not proof of the protocol or
runtime conformance. A subsequent self-read clarified T8's already-described
ordering in the table: handlers remain installed for mask restoration and the
final pending join; disposition release comes afterwards under the block.

Design estimates in minutes, against P≤100/E≤90 shared unused balances:

| Work | P | E |
|---|---:|---:|
| Checked capture/domain and front scope including choice failure | 20 | 0 |
| Checked worker watching/readers and partial thread failure | 15 | 0 |
| Checked runner handoff/fallible Started/child operations/first latch | 20 | 0 |
| Common finalisation/reporting/re-raise and C1–C4 composition | 15 | 0 |
| Finite fixture/barrier controls, manifest and evidence preparation | 20 | 0 |
| Measured red/broken/green case families | 0 | 35 |
| Untouched protected CLI/C1–C4/runner and workspace/check workflows | 0 | 25 |
| Receipt reconciliation and bounded rerun allowance | 5 | 20 |
| **Complete production/check estimate** | **95** | **80** |

Allow RC≤4 for the producer's one narrow review, coordination and retirement/
commit; unused allocations are not spend. The focused transition review found
no concrete issue within its assigned scope. The estimate does not include new
Linux infrastructure or minimum-toolchain installation.
Those domains remain explicitly unqualified; unchanged release support promises
remain debt rather than waived requirements. If those executions are required
to close this phase, price their precise continuation before claiming closure.

The design session cannot cut implementation leaves. If supported after review,
the next planning leaf merely freezes the additive observation contract and
parameter expansion and commissions this complete vertical increment from
actual remaining balances. It must not discover a second mechanism under a
nominally mechanical allowance. A substantive new protocol/review need instead
earns a precise human funding/scope continuation, with no reserve borrowing.
