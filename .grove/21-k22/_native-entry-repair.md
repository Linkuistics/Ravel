# native-entry-repair-k22 — brief

## Goal

Deliver the complete checked incumbent native-entry-to-settlement repair on the
existing one-session launcher, with discriminating receipts and protected
behavior. This is one useful increment: capture, front, worker and runner changes
cannot independently satisfy the contract. Keep them together and green.

## Context

Read native-entry-repair-k21 and native-entry-transitions-k20, the transition
protocol and priced table in docs/native-entry-transition-design.md, and
`docs/specs/result-job-completion.md#one-session-native-launcher-entry`.
The latter owns behavior; the design owns T0–T8 and E01–E08/F01–F07/R01.
Existing capture and reader shielding are retained mechanisms to check.
The three existing ADRs remain current; no package/extraction is selected.

Current approved phase caps after k26: D≤135/RC≤65/P≤280/E≤120/X=0 minutes,
zero production disruption. Cumulative correctness charged bounds are
D≤110/RC≤59/P≤155/E≤30/X=0. Remaining balances are D≤25 (D≤5 for the
mechanical planning handoff and D≤20 exclusively phase-closure reserve), shared
RC≤6/P≤125/E≤90/X=0. K26's verbatim approval and charges are recorded below.

The original complete P≤95/E≤80/RC≤4 estimate is suspended: source verification
did not establish the existing bounded error-settlement mechanism it assumed.
The original allocation is not a renewed entitlement. K25 supplies the selected
bounded protocol and complete replacement price. K26 approves that full scope
and the exact 32-row S amendment; native-entry-complete-repair-k27 alone receives
D≤5/RC≤1, P=E=X=0, to commission one complete producer from actual balances.
The producer price is P≤125/E≤85/RC≤5, D=X=0, including controls, measured
red/broken/green, protected checks, evidence and review/closure. No product work
is commissioned by this requirements session. Do not borrow reserve, reset caps
or silently enlarge controls.

## Done when

- Implement T0–T8 as one coherent repair, preserving immutable child entry
  values separately from launcher repairs, checked watching before both children,
  front scope through choice failure, permanent reader shielding and bounded
  collection, first consumed cancellation and common explicit finalisation.
  Every failed required operation prevents result success; retain original error
  plus all secondary reports. No child after failed admission. No signalling a
  recorded group after confirmed reap. Preserve C1–C4 and C4's exclusive foreground
  authority/no unrelated group-ID reuse premises. Started remains spawn-reported
  success, including possible pre-exec signal death.
- Add only the fallible Started interface frozen below, retaining all existing
  infallible entry points and event expectations. A check failure must enter
  bounded real stop/group cleanup/wait/terminal recovery before returning.
- Build and freeze exactly the concrete parameter expansion below plus the
  approved 32 S01–S05 rows in docs/native-entry-transition-design.md before
  measuring red/broken/green. Keep controls private and owner-local, fixed enums
  or fixed fixture data, without installed switches or a generic syscall layer.
  Exercise public CLI/real policy/fake job, real-process and PTY seams as agreed.
  Compare immutable per-case receipts with a meaningful failing broken control
  for each claimed repair. Some already-correct baseline cases may pass red;
  say so and use a targeted broken control, never invent baseline failure.
- Freeze subject manifests/digests before execution and confirm after; include
  source, fixtures, scripts, reference expectations and exclusion notes actually
  read by each run. One writer per receipt. Reconcile item by item; rerun every
  affected case after any instrument/subject change. A count alone is insufficient.
- Run the unchanged protected expectations through Taskfile test:cli,
  test:session, test:runner, test:terminal-recovery, test and check. Freeze
  test:list identities first and reconcile every prior identity individually.
  A new Taskfile selector is allowed only as a wrapper of these finite native
  cases. No suite expectations may be weakened to fit the repair.
- Preserve receipts for PID/SID/PGID, foreground owner/modes, raw action/mask,
  consumed/pending observations, actual wait/reap, reader completion/aliveness,
  physical operation and report substitution separately. Every fixture has
  bounded controller/child/pipe-holder/PTY cleanup on success, failure and timeout.
  Use acknowledged barriers, never sleeps as ordering evidence. Retain original
  errors when cleanup also fails; no ambiguous timeout is a passing receipt.
- Publish evidence linked from the design/contract with actual measured domain,
  red/broken/green subjects, controls, failures, limits and spend. One narrow
  reviewer may challenge failed-Started cleanup and first-signal/reader/finaliser
  composition only within the revised producer's funded allowance; the original
  RC≤4 allocation is suspended. Classify every finding. A second substantial review,
  material protocol discovery or extra runtime infrastructure earns only a precise
  funded/scope continuation before book-structure-k5. Do not claim phase closure.

## Frozen additive interface

Add `run_observed_with_start_check(launch, observer, start_check)` and
`run_confined_observed_with_start_check(launch, policy, observer, start_check)`
in keyed-launch, exported beside existing functions. Existing argument types and
return `Result<Ended, LaunchError>` remain. `observer` remains
`&mut dyn FnMut(LaunchEvent)`; `start_check` is
`&mut dyn FnMut() -> std::io::Result<()>`. Existing observed entry points adapt
with an always-Ok check. Existing run/noninteractive entry points retain behavior.
The callbacks remain synchronous, prompt and nonpanicking by contract.

After spawn-reported success, the runner owns child, group identity and lease
before either callback. Emit Started once, then invoke start_check once before
normal watching/admission of result. Failed spawn invokes neither. Reaped remains
infallible, emitted once only on actual confirmed reap, before terminal recovery,
also during error cleanup. A failed start_check retains its io::Error as source
with operation context, does not populate the spawn-only raw_os_error accessor,
and never produces an Ended success. Its source raw errno remains inspectable
through Error::source; no public LaunchError constructor is needed.

On failed check, initiate existing bounded escalation/error settlement, retain
first already-consumed runner signal and any subsequently consumed signal only
if no first exists, terminate the live group before reap, confirm wait and recover
the terminal. Preserve every stop/wait/recovery report beside the initiating
check error. If an OS cleanup operation fails, return explicit uncertainty rather
than inventing reap or success; the callback error itself never authorises early
return with an unattempted live-child cleanup. Do not lend a reclaimed terminal
again on this error path. The front retains its saved mask/scope and performs
common finalisation even when the callback's restore failed. Join front, outcome
consumed signal, late runner and eligible pending in T7 order; keep earlier
operational errors and all reports under cancellation precedence.

## Frozen finite case expansion

Use one immutable row per tuple below, suffixed with explicit parameter names.
All products mean full Cartesian expansion unless the restriction is stated.
E families use no injected faults. Baseline default signals are unblocked with
no pending signal unless a tuple says otherwise. The production executable has
no fault/barrier environment interface. Private fixture builds expose only the
named owner-boundary acknowledgements/operation outcomes.

| Family | Exact expansion |
|---|---|
| E01 | INT, TERM, HUP, PIPE × default, ignore × clear, blocked. Raw initializer and native child's first-code mask/action receipts, PIPE before helper runtime. Independently break capture and reinstatement. |
| E02 | INT, TERM, HUP × pending yes, no; named signal blocked at exec. Parent pending and child blocked mask recorded separately; do not assert pending crosses fork. Break exclusion of entry-blocked cancellation. |
| E03 | SIGCHLD default, ignore at actual exec; each row confirms actual worker and managed waits. Break pre-worker repair. |
| E04 | Private default+SA_NOCLDWAIT × worker, managed admission boundary; raw query immediately before watching, actual wait after. Separately one raw Darwin executable-entry flag-reachability observation; cleared flag means not inherited. Linux exec flags are source-cleared, never relabel private injection as inheritance. Break ignore-only repair. |
| E05 | direct launcher INT, PTY VINTR × before handoff, after handoff; default/unblocked INT. Before handoff terminal signal reaches launcher; after handoff terminal signal reaches child group. Publish then signal child for after-handoff child-status discriminator. Record actual foreground acknowledgements and distinguish launcher cancellation. |
| E06 | selection-to-runner, Started-before-parent-mask-restore, after-confirmed-reap-before-front, policy-refusal-before-finalise, spawn-ENOENT-no-ending-before-finalise × INT, TERM, HUP. Named barrier acknowledges current owner/masks before controller signals and releases. Break corresponding join individually. |
| E07 | TERM then HUP, HUP then INT × runner-active, runner-wait-error. Acknowledge first consumption before second delivery; use existing private wait-error seam. Break first-latch compare/exchange. No chronological claim about simultaneous pending standard signals. |
| E08 | both readers complete, one reader held by escaped pipe-holder × successful selection, policy refusal. Keep both reader masks recorded and send process-directed TERM at finalisation barrier; creator's restored mask recorded. Controller owns escaped holder and kill/wait. Break permanent shielding and check actual thread eligibility, not statistical delivery. |
| F01 | capture mask query, required valid TERM action query, TERM membership query. Substitute initializer result with fixed EIO; receipt records real query separately. No child admission. |
| F02 | worker, managed boundary × watching action query, default-action repair. Precondition is private default+SA_NOCLDWAIT (E03 independently covers executable ignore). EIO on query; repair rows each omitted mutation+EIO and successful physical repair+substituted EIO. |
| F03 | reader mask block, first thread create, second thread create, creator-mask restore. Thread faults use fixed EAGAIN, other faults EIO. Mutation faults each omitted operation and successful physical operation+reported error; creation faults omit only. Record retained first reader and actual bounded worker wait. |
| F04 | selection handlers, runner handlers × action query, first install INT, later install TERM, final restore INT. EIO. Query uses substituted report; mutations each omitted and performed+substituted. Capture partial installed subset/main mask. Use no-child boundary for admission failures and actual finalisation boundary for final restore. |
| F05 | selection-success, policy-refusal, spawn-ENOENT-no-ending × finalisation block, pending query, TERM membership, final mask restore; plus selection-success × Started parent mask restore only. EIO; query/membership substitute result, mutations each omit and perform+substitute. Queue actual TERM under block before queries. Verify query uncertainty never becomes empty cancellation; Started row uses live real child/group and PTY lease, not a fake Process alone. |
| F06 | child TERM action restore, child mask-set construction, child mask restore × omitted operation+EIO, successful operation+substituted EIO. Fixed pre-exec data only; actual spawn errno/wait/foreground/modes and C4 recovery. No allocation/formatting in pre-exec. |
| F07 | selected TERM default action, selected TERM unblock, TERM raise. Forked fixture with first TERM already consumed and no pending TERM. Each omitted mutation+EIO; also performed+substituted EIO for default/unblock only. Successful raise cannot report afterwards: separate unmodified real-signal-death control. Parent checks WIFSIGNALED versus numeric 128+TERM fallback, zero result. |
| R01 | Every frozen existing CLI/session/runner/terminal-recovery/workspace test identity, through existing Taskfile tasks. Preserve configuration, UTF-8/empty/nesting/first-writer/final-write and C1–C4 expectations. |

Each fault is single-site; no all-fault Cartesian product. To check retained
secondary failures, add exactly these paired cases: F03 second-create failure +
worker wait report, F05 Started restore failure + terminal recovery report,
F05 policy-refusal final-restore failure + original refusal, F06 action error +
C4 recovery report. Reuse existing owner report seams. Cancellation variants
are exactly F05 Started restore and F06 action failure with eligible TERM queued
under the parent's known handoff block, then joined during front finalisation.
They assert pending-to-finalisation transfer, not pre-spawn handler consumption
while blocked; E07 independently proves first-consumed retention.
Record actual operation versus report;
substituted successful wait is never called an actual failed wait.

Barriers carry fixed boundary IDs above over private pipes: fixture writes ready
with owner/state facts, controller sends signal or selects the fixed fault,
fixture acknowledges consumption where required, controller releases. Use a
30-second outer per-case watchdog, 5-second acknowledgement deadline and
5-second controller cleanup deadline; production escalation and worker collection
limits remain their existing configured values. If a legal existing limit cannot
fit, record and price the exact timing remainder before execution rather than
silently extending the matrix/budget. On timeout stop/wait owned launcher/children,
close PTY/pipes and collect reader state; never signal a reaped identity. Validate
controller timeout, escaped-holder cleanup and failed barrier release with private
fixture controls before trusting passing runs.

For every F operation, the broken control ignores that operation's failure;
for E families use the named broken check/join/capture or shielding mutation.
Freeze mutation subjects separately from incumbent and repaired subjects.
Irreversible process operations are not simulated by rewriting a report.

## Notes

Host-only receipts qualify only that host/compiler. Verify actual libc signal
ranges against target headers. Linux runtime and minimum Rust 1.85 remain
unqualified; no infrastructure/install work is priced here. Keep existing support
obligations explicit and cut the exact next continuation before the book on
closure, even if implementation succeeds. A returning host is a different
contract. K4 reference 0bdcb1f9ae24db5e9dd499308dd791744e908eb0, one iteration/
seven attempts, R=1/8,F=4/4,N=3/6,D≤45/RC≤15 remain; k6 D≤45/RC≤10 remains.
G2 six-month cost, G3 delivery/use beyond entry, G4 unread alternatives, G5 target
execution and R-W runtime/lifecycle debt persist. No comparative gain, extraction,
independent adoption, parent/root completion or book acceptance follows.

## Decisions (running log)

- Source verification found that the design's proposed reuse of existing bounded
  error settlement is unsupported. Runner emergency recovery performs unchecked
  signalling followed by blocking Child::wait; the killed polling state has no
  deadline. Worker kill_and_wait likewise performs blocking wait after its grace.
  This is source evidence, not an observed hung process or a failed native case.
- The one permitted narrow reviewer inspected the existing runner against the
  frozen Started contract. Its boundedness, lost stop reports and uncertain
  terminal-recovery findings are actionable prerequisites. The emergency-group
  finding needs a precise distinction between attempted group termination and
  confirmed group absence; do not infer that the contract requires confirmation
  on every error. Full findings and classifications belong in the transition
  design's implementation-prerequisite section.
- Treat this as the mandate's material-protocol-discovery stopping condition.
  Preserve the complete repair as one future producer; do not split it by module,
  waive boundedness, enlarge the frozen controls or spend reserved D. Decompose
  only to deliver the independently useful source prerequisite and a precise
  human scope/funding continuation. No product repair or native measurement has
  begun, and the node cannot close.
- Charge P≤15/RC≤4, D=E=X=0 for the prerequisite session k23, including this
  node's initial source verification and one reviewer. Cumulative correctness
  bounds are D≤100/RC≤49/P≤155/E≤30/X=0; remaining D≤20 exclusively phase
  closure reserve, shared RC≤1/P≤85/E≤90/X=0. K24 carries the exact proposed
  renewal. No complete replacement producer is commissioned before revised
  protocol/pricing; this node's original estimate is suspended, not renewed.
- K24's human answered verbatim **“Approve the bounded renewal (Recommended)”**.
  Caps become D≤130/RC≤55/P≤240/E≤120/X=0; the D≤20 phase-closure reserve
  remains untouched. K24 charges conservative RC≤4, D=P=E=X=0, including
  agreement recording/tree/verification/retirement/commit. Cumulative bounds are
  D≤100/RC≤53/P≤155/E≤30/X=0; remaining D≤30 (10 for protocol/pricing and
  20 exclusively closure reserve), shared RC≤2/P≤85/E≤90/X=0.
  `native-entry-settlement-k25` alone receives D≤10/RC≤2, P=E=X=0 for the
  next design/pricing continuation inside this node.
  No replacement producer, control amendment, parent close or book advancement
  is authorised by this renewal alone.

K25 selects the unimplemented bounded settlement protocol in the existing design
and required spec: owner-local nonblocking observation, absolute 2-second runner
error/post-KILL and total worker teardown bounds, separate existing 1-second
worker drain/group confirmation, checked individual stops and explicit unresolved
ownership. ECHILD/reap/release disables numeric signalling. Unknown status grants
no foreign-foreground takeover; all recovery uncertainty and original/secondary
reports survive to T7. Existing protected logical wait-event traces are reconciled
individually. Exactly 32 S01–S05 rows are proposed and priced, not commissioned;
frozen API/T0–T8/E/F/R remain. One focused reviewer returned no concrete finding.
No product/control or native behavior was constructed/executed.

K25 conservatively charges D≤10/RC≤2, P=E=X=0 including verification/retirement/
commit. Cumulative correctness bounds become D≤110/RC≤55/P≤155/E≤30/X=0;
remaining D≤20 exclusively phase closure, RC=0/P≤85/E≤90/X=0. The complete
producer price is P≤125/E≤85/RC≤5; necessary planning is D≤5/RC≤1 and
funding-agreement recording/closure RC≤4. Exact shortfall is D≤5/RC≤10/P≤40,
with no E/X increase: propose caps D≤135/RC≤65/P≤280/E≤120/X=0. Allocations
are not spend. native-entry-complete-funding-k26 inside k22 is the next precise
human continuation before the book. The repair stays one complete future producer;
none is commissioned from insufficient balances and no reserve is borrowed.
Minimum-Rust/Linux qualification, returning-host, k4/k6/G2–G5/R-W and all root
obligations remain live; no parent/phase/root close or comparative gain follows.

K26's human answered verbatim **“approved”** to adding D≤5/RC≤10/P≤40,
raising caps to D≤135/RC≤65/P≤280/E≤120/X=0 while retaining zero production
disruption and the original D≤20 phase-closure reserve. Approval explicitly
covers the design's exact 32 S01–S05 rows at the existing CLI/real-policy/fake-job,
real-process and PTY seams, unchanged T0–T8/additive Started API, and the 30/5/5
controller envelope. Longer legal graces still require their exact priced timing
remainder before execution; no blanket matrix or target infrastructure is funded.

K26 charges conservative RC≤4, D=P=E=X=0 including recording/tree/verification/
retirement/commit. Cumulative charged bounds are D≤110/RC≤59/P≤155/E≤30/X=0;
remaining D≤25 (5 planning, 20 exclusively phase closure), RC≤6/P≤125/E≤90/X=0.
native-entry-complete-repair-k27 is the sole next planning handoff inside k22,
allocated D≤5/RC≤1, P=E=X=0. Full planning use leaves D≤20 reserved/RC≤5/
P≤125/E≤90/X=0, sufficient for the full P≤125/E≤85/RC≤5 producer. Full use of
both would leave D≤20 reserved/RC=0/P=0/E≤5/X=0; projections are not spend.
Record actual planning charges before commissioning that producer. The complete
repair and all support/comparison/root obligations remain live; k22 cannot close.
