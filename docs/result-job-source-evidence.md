# Result-job acquisition and native-entry evidence

The current front has enough local information to associate a successfully
allocated channel with its own selected launch and to attempt the explicit
selected-scope cleanups. That is narrower than a completion owner that can
settle **every** acquisition and failure path. Source-admitted paths lose a
cancellation observation or do not establish a required prerequisite at the
fixed reference inspected here. The later
[C1 repair and checked observations](completion-failure-design-evidence.md#c1-checked-observations)
retain cancellation across runner errors and no-ending front settlement; the
later [C2/C3 checks](completion-failure-design-evidence.md#c3-checked-observations)
cover worker admission and partial-directory ownership. Later
[C4 checks](completion-failure-design-evidence.md#c4-checked-observations)
cover failed-exec terminal recovery under its authority premises; the inventory
and counterexamples below remain fixed-reference evidence. A direct
native runner call also retains changes to its host's signal dispositions.

This record supports the [completion contract](specs/result-job-completion.md),
not a replacement API, extraction or comparative selection. Required behaviour
remains in the [session specification](specs/session-execution.md). The
[acquisition view](design/result-job/index.html#diagram-acquisition) and
[native-entry view](design/result-job/index.html#diagram-native-entry) illustrate
the inventory below.

## Scope and provenance

The inspected product source matches fixed reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`: `jj diff --from` that reference
`--to @ --summary crates` returned no changes. This is a current-source
observation. No product code, executable seam, behavioural test, equivalence
campaign or live exposure was introduced or run.

Tier 2 graph verification used project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-14T04:09:13Z`. Qualified-symbol searches completed their relevant
pages; direct outbound traces of front `run`, runner `run_with_output` and
worker `evaluate`, and both directions for runner `run_with_output`, were
untruncated. Exact snippets and direct reads supplied control-flow evidence.
Some graph edges resolve unrelated same-named methods: edges are navigation,
not proof of execution. Coverage checked the nine source files cited below
and the two source-directory scopes: metadata matched, with no recorded gaps.
That best-effort result is not completeness proof.

The inventory classifies ordinary returning `Result` branches, acquired
resources and signal transitions in front composition, selection handoff,
worker teardown and selected-job entry/supervision. It does not enumerate
policy-protocol internals, all OS interleavings, panics, allocation aborts or
uncatchable process death. The latter cannot establish successful completion.
Rows distinguish checked cleanup from attempted or destructor-only cleanup.

## Acquisition and stopping-state inventory

| State / stopping branch | Acquired resources and owner | Signals and local knowledge | Cleanup / permitted outcome in source |
|---|---|---|---|
| A0: entry recording, exit-parent validation or request preparation fails | Front owns prepared values only; no selected job, result channel or output reservation | Initializer's recorded ignored set/mask; selection handlers are not installed yet | Immediate refusal via `?`; entry-recording failure refuses transparency. No settled result. |
| A1: confined preparation fails before a private directory exists | Front preparation owns canonical grants and backend checks | No selection guard yet | Immediate confinement refusal; nothing selected or launched. |
| A2: confined preparation fails after directory creation | `Confined::prepare` still owns a local `TempDir`; no `Confined` is returned | Front never receives the directory or a cleanup result | Local destructor cleanup only; original refusal returns directly. A cleanup error cannot be supplied to `settle` on this path. |
| A3: workspace prepared; worker lookup or handler installation fails | Front owns optional `Confined`; handler installation owns any already-saved dispositions | Lookup installs no handlers; partial handler installation drops its local guard and attempts saved-disposition restoration | Front explicitly removes `Confined` and calls selection-path `settle` with the front latch. Partial restoration syscall results are discarded. No job or output. |
| A4: worker preparation/spawn fails | Evaluator owns private directory, protocol socket ends/duplicate and command pipes; front retains selection guard and workspace | Selection latch may override the refusal | Evaluator's local descriptors/private directory drop; selection drops handlers; front removes workspace and settles refusal. No managed launch. Worker-directory removal is destructor-only. |
| A5: worker runs; evaluation refuses, expires, breaks or is cancelled | Evaluator owns worker child, socket and bounded capture; selection guard remains installed | Cancellation is checked during conversation and again after teardown | Socket closes; worker receives stop/kill and a wait attempt; diagnostics are collected and private directory drops. Failure returns; front settles with workspace cleanup. A wait error is not a confirmed reap. |
| A6: worker conversation succeeds and teardown completes | Same worker resources; choice resolution follows evaluation | Final selection latch check can still refuse | A successful wait normally precedes `Selected`. However `evaluate` also returns success when its stored wait result is `Err`; see C2. |
| A7: `Selected` reached; output reservation fails | Front owns `Selected`'s choice/source/live handlers and optional workspace; reservation may briefly own a saved fd | Front latch remains active; no transition block or runner yet | Reservation's local guard drops if duplication/redirection fails. Front still attempts mask restore, workspace removal, output cleanup if acquired, drops handlers, joins signals and settles. No channel or job. |
| A8: output reserved; channel allocation fails | Front owns output guard/workspace/handlers; allocator may own a local directory and held fd | Front latch remains active; no runner yet | Partial channel allocation uses local destructor cleanup; front receives only allocation error. Outer output/workspace cleanup still runs. Directory-removal failure inside allocation is unavailable to settlement. |
| A9: channel allocated; transition check cancels | Front owns output, channel, workspace, handlers and same-launch association | `block_and_check` checks latch/pending handled signals not blocked at executable entry; blocks this thread even on cancellation | No spawn. Front restores mask while handlers remain, explicitly closes channel, removes workspace, restores output, drops output/handlers, joins latches and settles cancellation. |
| A10: confined command construction fails | Same front resources; backend has not entered runner core | Transition mask remains blocked; front guard still owns dispositions | No `Started`/`Ended`; front mask restoration and all acquired-resource cleanup still run. Runner handlers may never have been installed. |
| A11: runner core preparation/spawn fails | Runner owns optional terminal fd/lease and command; front retains channel/output/workspace/guard | Runner installs handlers and may repair SIGCHLD before fallible descriptor setup/spawn; latch clears immediately before spawn | No `Started`/`Ended`. Front restores mask, checks late runner latch and cleans its resources. Runner does not guard host-disposition restoration; an interactive child can hand itself foreground before exec then fail, without a reclaim call on this branch (C4). |
| A12: spawn succeeds; supervision fails | Runner owns child/group/terminal lease; front knows `Started` but stores no `Ended` | `Started` restores front mask, discarding that callback's restore result; `watch` can consume cancellation before failing | Wait-observation failure sends KILL and attempts reap; reap failure returns unknown status. `Reaped` fires only on confirmed reap. `supervise` attempts terminal reclaim but returns only `LaunchError`, with no group confirmation or consumed signal (C1). Front refuses and cleans remaining resources. |
| A13: own `Ended` returned | Runner has reaped direct child, attempted terminal recovery, queried group; front owns ending/channel association | Ending includes interruption through reap; later signal stays in runner latch | Front validates result against this ending. Interrupted/live-group/invalid/absent result refuses. Explicitly empty publication is successful candidate data. Child's later exit code is not decisive. |
| A14: result candidate or failure; final cleanup | Front still owns all acquired selected-scope resources and guard | Mask restore occurs while a handler is present; guard drops before front/ending/late join | Attempt mask restore → channel close → private removal → output restore, drop output (best-effort restore again) → drop handlers. Collect all supplied cleanup results; join front, interrupted ending, late runner in that order. |
| A15: settlement / output | Front owns candidate/failure plus supplied cleanup reports; no reusable job owner is returned | Applicable *available* cancellation wins; C1 can make a consumed one unavailable | Report every supplied cleanup error and surviving-group diagnostic. Cancellation > original result failure > first cleanup failure > group. Success alone reaches exact stdout write/flush; final write failure is `result_write_failed` and can leave partial output. |

The inventory precedes and defines the diagrams. “Cleanup” does not mean every
system call succeeded. Handler installation/restoration, transition mask
blocking and terminal handoff/restoration have unchecked syscall results in
the inspected code; a successful `Ended` contains no terminal-restoration
receipt. These are premises to discriminate, not new waivers of O2/O7/O8/O9.

## Exact source anchors

| Evidence | Source locations and what they establish |
|---|---|
| F: front composition | [`run`](../crates/ravel/src/run.rs#L32), [`settle`](../crates/ravel/src/run.rs#L250), [`ResultOutput`](../crates/ravel/src/run.rs#L315): immediate preparation exits, guarded selected closure, explicit cleanup order, signal precedence, diagnostic and final-write boundary |
| S: selection transfer | [`choose_prepared`](../crates/ravel/src/choice.rs#L60), [`Selected`](../crates/ravel/src/choice.rs#L98), [`evaluate_and_resolve`](../crates/ravel/src/choice.rs#L118): success moves live guard and source; failure does not return the guard; evaluation precedes resolution |
| H: entry / handlers | [`record`](../crates/ravel/src/signal_state.rs#L66), [`restore_mask`](../crates/ravel/src/signal_state.rs#L135), [`entry`](../crates/ravel/src/signal_state.rs#L146), [`Handlers`](../crates/ravel/src/cancellation.rs#L60): executable-entry capture, thread-mask restoration, child entry value, live selection dispositions and first front signal latch |
| W: worker teardown | [`evaluate`](../crates/ravel/src/worker.rs#L316), [`stop`](../crates/ravel/src/worker.rs#L873): wait attempt occurs before returning; its result does not gate the successful conversation arm |
| P: workspace acquisition | [`Confined::prepare`](../crates/ravel/src/confinement.rs#L28), [`remove`](../crates/ravel/src/confinement.rs#L131): directory exists before later fallible checks; explicit removal is available only after successful preparation |
| T: result acquisition | [`SessionChannel::allocate`](../crates/ravel/src/completion.rs#L33), [`result_after`](../crates/ravel/src/completion.rs#L56), [`close/cleanup`](../crates/ravel/src/completion.rs#L73): held-directory/matching-ending checks, destructor-only partial allocation, identity-protected explicit cleanup |
| R: native entry and terminal | [`install_termination_handler`](../crates/keyed-launch/src/run.rs#L317), [`restore_child_watching`](../crates/keyed-launch/src/run.rs#L365), [`EntrySignals`](../crates/keyed-launch/src/run.rs#L429), [`Lease`](../crates/keyed-launch/src/run.rs#L592), [`run_observed`](../crates/keyed-launch/src/run.rs#L735), [`run_confined_observed`](../crates/keyed-launch/src/run.rs#L783), [`run_with_output`](../crates/keyed-launch/src/run.rs#L793): persistent host changes, child-state input, observed events, confined pre-entry refusal and spawn lifetime |
| J: ending / error | [`supervise`](../crates/keyed-launch/src/run.rs#L1160), [`watch`](../crates/keyed-launch/src/run.rs#L1197): successful ending versus error payload, consumption of cancellation, cleanup/reap/reclaim/confirmation order |
| CLI: reporting | [`main`](../crates/ravel/src/main.rs#L29), [`report_failure`](../crates/ravel/src/main.rs#L172): CLI receives success/failure after front settlement and propagates supplied cancellation; cannot reconstruct information discarded below it |

## Counterexamples and the smallest missing premises

### C1: cancellation consumed before a supervision error

Take a selected launch with no earlier front-latched signal. `watch` consumes
TERM through `take_interrupt`, stores it locally and forwards it. The next
`child.exited()` fails. Even if the emergency reap succeeds, `Failed` carries
only error/status; `supervise` returns only the error. Front `ended` stays
`None`, and the consumed runner latch can be empty. With no later signal,
front settlement selects `supervision_failed`, not cancellation. Result bytes
are suppressed, but required signal propagation is not established. The
normal-exit-then-reap-error branch can lose the same local interruption.

Smallest missing premise: either this combination cannot occur in the supported
environment (not established), or the consumed cancellation must survive the
error boundary. The cheapest discriminating observation, when funded, is the
existing private `Process` wait-error seam with interruption before an injected
observation/reap failure, then the current front's no-ending settlement. This
session did not add or run that case. Error cleanup also does not confirm group
absence, so no caller may treat the failure as successful teardown.

### C2: worker wait error does not gate selection success

For an otherwise successful conversation, `evaluate` stores `stop`'s
`io::Result<ExitStatus>`, drains diagnostics, drops the private directory and
returns `Ok(Evaluation)` without checking that stored result. `stop` can return
the final `child.wait()` error. Resolution and `Selected` can therefore follow
without a **confirmed** reap. An error does not prove a live worker; it does
mean the required confirmation is absent. Naming a selected-job completion
owner cannot supply this upstream fact.

Smallest missing premise: successful evaluation must imply confirmed worker
reap, or a separately justified equivalent observation. Cheapest next
discriminator is a funded teardown-error observation at the evaluator boundary,
checking that no selected launch is admitted. There is no claimed existing
worker-wait injection seam; changing that boundary needs its own authorization.

### C3: partial acquisition cannot report all cleanup results

After private directory creation, a later confined-preparation check can fail;
after session directory creation, opening the held directory or allocating the
appearance channel can fail. Each function returns the original error and
drops its local temporary directory. Front settlement receives neither that
resource nor a removal outcome. The smallest missing premise for explicit
all-acquisition cleanup is a checked failure-path cleanup report or a justified
guarantee that destructor removal cannot fail. The source establishes neither.
No result is returned on these paths; potential debris and missing diagnostics
must not be presented as successful settlement. A funded partial-allocation
failure plus cleanup-failure observation discriminates that premise.

### C4: interactive pre-exec handoff followed by exec failure

When the parent owns foreground, `hold_before_spawn` supplies the terminal fd.
The child `pre_exec` calls `tcsetpgrp` before exec. If handoff succeeds and exec
then fails, `command.spawn()` returns an error; the `?` bypasses `supervise`
and `Lease::reclaim`. The lease has no destructor reclaim. Front output/mask/
directory cleanup cannot recover this terminal ownership on its own. A valid
selection whose executable disappears before spawn is one way to reach the
exec-failure branch. This is a source-admitted conditional trace, not a measured
PTY failure. Cheapest discriminator is a funded real PTY launch with failed
exec after handoff, checking foreground recovery through the existing runner
terminal boundary. It must also distinguish a failure before handoff.

## Selected-scope transfer versus native entry

At successful `Selected`, a prospective internal composition can move the
choice, source, live guard and optional prepared workspace together **before**
reserving output or allocating a channel. That owner would then retain its
own channel-to-launch association, store an optional ending, perform every
acquired selected-scope cleanup, retain supplied failure diagnostics and hand
settled bytes to the CLI's final-write boundary. These enabling points exist;
the combined owner does not. This limited ownership description neither
transfers A0–A6 nor supplies C1/C2/C3/C4's missing observations. It is not a
supported complete-operation API or evidence of improvement.

A direct native caller has a different entry contract. `EntrySignals::new`
accepts a caller-provided ignored set and mask. It neither captures them nor
restores the host on return. Executable-entry transparency requires capture
before the Rust runtime changes SIGPIPE, as the front initializer does. A host
wanting call-entry semantics must define that different instant explicitly.
Child exec-surviving ignored/default state is not a copy of host custom handlers.

The runner replaces nonignored TERM/HUP handlers, and INT when there is no
terminal or `transparent` is supplied. It retains those handlers after success
and runner-core failure. An ignored SIGCHLD is repaired to default and latched
for future calls, without host restoration. A direct host with a custom TERM
handler can thus receive a successful ending and find the runner handler still
installed. This is a concrete restoration counterexample independent of C1.
The CLI selection guard restores its saved INT/TERM/HUP dispositions; it does
not cover SIGCHLD, and its front process ends rather than resuming arbitrary
host work. Front's cancellation latch also is not reset by guard installation.

Native reuse must consequently declare an exclusive process-wide signal and
terminal lifetime, a cancellation policy (including terminal INT), capture
instant, ignored/blocked-signal treatment, late-latch consumption between
launches, and who restores any host state. The calling thread's mask and other
threads' signal eligibility are separate concerns. The runner provides no
concurrent-embedding guarantee. A returning host cannot infer restoration from
“synchronous”; a process-lifetime supervisor may accept persistent dispositions
only under an explicit caller contract. Neither arrangement is selected here.
Actual foreground ownership gates interactive lending; confined jobs instead
start a detached session. Panic-free prompt observers remain a caller obligation.

## Evidence limits and continuation

The later [failure repair design](specs/result-job-completion.md#required-failure-settlement)
and [discriminating check plan](completion-failure-design-evidence.md#discriminating-checks)
address these source-admitted gaps under the agreed one-session launcher
lifetime. They establish requirements and proposed local enabling points,
without repairing or runtime-testing the source observed here.

The useful stopping state is a documented incumbent with explicit entry and
failure limits. Internal channel/job association is structurally possible;
complete signal/resource settlement is unsupported without the premises above.
Runtime conformance, independent delivery/use and human usability remain
unobserved. Existing CLI/real-policy/fake-program, transport/real-process and
runner/PTY seams remain the acceptance boundaries; diagrams and source reads
are not those checks.

The closed comparison retains one iteration, seven attempts, R=1/8, F=4/4,
N=3/6, charged D≤45 minutes and RC≤15 minutes, P=E=X=0. This observation
does not reset or reopen it. G1 now has concrete failure/entry limits, not a
universal transfer solution. G2 six-month upkeep/cost, G3 independent use and
delivery beyond entry, G4 unread comparison scope, G5 runtime checks, and R-W
compatible runtime/new lifecycle debt remain. A later comparative judgment
requires prospective funding with the fixed reference/frame/history. A human
scope/funding continuation precedes the book-structure agreement; root capability,
configuration/session, book and post-proof practical reader obligations remain.
