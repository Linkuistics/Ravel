# Completion failure repair evidence

The [completion specification](specs/result-job-completion.md#required-failure-settlement)
states the required corrections and one-session native launcher entry contract.
This document holds source locations, proposed private enabling points and
discriminating checks. Product source is unchanged; a design and rendered
diagram establish no runtime conformance. The earlier
[A0–A15 inventory and C1–C4](result-job-source-evidence.md#acquisition-and-stopping-state-inventory)
remain observations at fixed reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`.

## Obligation and source map

| Gap / obligation | Current source evidence | Local repair and hidden decision |
|---|---|---|
| C1 / O7, O6, O9 | [`watch`](../crates/keyed-launch/src/run.rs#L1197) consumes cancellation; both error exits lack it in [`Failed`](../crates/keyed-launch/src/run.rs#L1142). [`supervise`](../crates/keyed-launch/src/run.rs#L1160) returns only its error. [`run`](../crates/ravel/src/run.rs#L32) has no ending on that branch. | Runner retains its consumed signal in its existing opaque error, alongside the original message/errno. Front reads that observation before converting the error and uses it in the existing join. No global requeue or fabricated `Ended`. |
| C2 / O2, O3, O9 | [`evaluate`](../crates/ravel/src/worker.rs#L316) stores the result of [`stop`](../crates/ravel/src/worker.rs#L873), then admits the successful conversation without checking it. [`evaluate_and_resolve`](../crates/ravel/src/choice.rs#L118) resolves only after evaluation returns. | Evaluator gates successful evaluation on confirmed teardown. Keep conversation judgment, bounded output, timeout and cancellation precedence; retain wait/removal errors as secondary operational diagnostics when a conversation already failed. |
| C3 / O9 | [`Confined::prepare`](../crates/ravel/src/confinement.rs#L28), [`SessionChannel::allocate`](../crates/ravel/src/completion.rs#L33) and worker preparation acquire a directory before later fallible work. Destruction alone reports no removal outcome. | Each allocator settles its local partial resource, returning original failure plus checked cleanup reports. Use the existing identity-protection rule of [`SessionChannel::cleanup`](../crates/ravel/src/completion.rs#L77); retain captured policy output separately. No new public filesystem abstraction. |
| C4 / O8, O9 | [`hold_before_spawn`](../crates/keyed-launch/src/run.rs#L606) saves a lease; [`run_with_output`](../crates/keyed-launch/src/run.rs#L793) can hand foreground to the child before failed exec returns through `?`. [`reclaim`](../crates/keyed-launch/src/run.rs#L650) occurs only inside supervision and returns no checked outcome. | Runner keeps that lease through the spawn result and makes recovery reportable. Identify this attempt's group before handoff; preserve spawn errno while appending recovery failures. Keep existing delegated-foreground policy for successful launches. |
| Native entry / O2, O7, O8 | [`record`](../crates/ravel/src/signal_state.rs#L66) runs before Rust startup; [`EntrySignals`](../crates/keyed-launch/src/run.rs#L429) supplies child state. Host handlers and ignored-SIGCHLD repair persist. | CLI lifetime owns host signal authority until exit. Capture before runtime changes; establish waitability before the worker as well as before the managed child; keep required mask/terminal cleanup and late joining. No arbitrary continuing-host restoration. |

The source was discovered with Tier 2 graph verification in project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-14T05:10:00Z`. Qualified searches completed their pages. Both-direction,
depth-one traces of front `run`, worker `evaluate`, runner `run_with_output`
and `supervise` completed without truncation. Exact snippets established the
material branches. Same-name edges into unrelated tests, scripts or other
`restore` methods were not taken as execution evidence. Coverage of every
relied-on source/document path initially reported matching metadata and no
recorded issue; that best-effort signal proves no completeness property.
Mermaid and changed/unindexed documents are read directly at closure.

## Choosing local corrections

Three arrangements were considered for the error information, without reopening
the component comparison. Retaining today's error/latch behavior fails C1/C2's
existing promises. Relatching a consumed runner signal avoids an error accessor
but merges a completed operation's information with late arrival/global state;
the caller can still lose which launch consumed it. A new complete-operation
API would demand the unsettled acquisition/lifetime/package design and spend
outside this correctness increment. Extend the existing runner error with its
consumed cancellation and keep resource-local checked reports. Success APIs,
package boundaries and CLI policy routing remain unchanged.

The runner error's signal accessor is meaningful outcome data, not a test
switch. Its primary errno remains the spawn cause only. Recovery diagnostics
must not replace it or require parsing a message to learn cancellation. A
private vector or equivalent local report can retain secondary errors; this
design does not require a public universal error taxonomy. At the front, render
all operational cleanup reports through the existing stderr reporting boundary,
including in JSON mode as existing cleanup reporting does. Do not add JSON
schema fields, change normal notices, or label Ravel cleanup as captured policy
stdout/stderr. Preserve original code/stage/status when adding error-context
text on newly corrected failure paths.

For C4, merely calling existing `reclaim(None)` after spawn failure is
insufficient: the parent lease's child field is still zero, and the saved
attributes/recovery syscalls can fail silently. Nor may recovery take any
foreign foreground group. The smallest bounded mechanism proposed is a private
close-on-exec pipe prepared only for an owned interactive lease. In the child,
after the process group exists and before `tcsetpgrp`, write one fixed-size
record containing this attempt's group ID. The parent retains the reader until
the spawn outcome. Failure to create the pipe or write the complete record
fails before handoff. Check `tcsetpgrp`; an error is a spawn failure. In the
parent, close its write end after spawn on both paths and inspect the record;
successful spawn supplies the child ID too. No pipe or test descriptor reaches
the exec'd managed program. Pre-exec work uses only async-signal-safe operations
on precomputed descriptors and fixed data, with EINTR handled and no locks or
allocation.

On failed spawn, use the record and actual foreground to decide whether this
attempt owns the handback **under the exclusive failed-spawn lease premise**:
no competing foreground transfer or unrelated reuse of the recorded group
identity occurs from observation through handback. CLI process lifetime alone
does not establish that premise. Without it, the query/set pair is only a
checked-holder policy, not an unconditional no-theft guarantee. The record
is not a durable authority receipt after reap. Already-own foreground needs mode restoration;
matching attempted group permits handback; unrelated foreground is preserved
with an unconfirmed-recovery diagnostic. The record is **not** a group-ID
reservation after reap: never send a signal to that ID on the error path.
Malformed/missing receipt or query failure retains the spawn error and reports
the ownership/recovery uncertainty. Failure reports must name every failed
handback, mode restoration and temporary SIGTTOU restoration attempt. This
mechanism adds one small private pipe and report protocol, not a new launcher
component. Its portability and actual recovery still need PTY checks.

Rust's [pre-exec contract](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.pre_exec)
requires child-safe work and says child memory changes are invisible to the
parent. In the inspected Rust
[1.85.0 implementation](https://github.com/rust-lang/rust/blob/1.85.0/library/std/src/sys/pal/unix/process/process_unix.rs)
and [1.98.1 implementation](https://github.com/rust-lang/rust/blob/1.98.1/library/std/src/sys/process/unix/unix.rs),
the ordinary pre-exec/exec error branch waits for the failed child before
returning the spawn error; wait/pipe failures can panic. This is pinned source
evidence, not a public guarantee about every future Rust toolchain. The repair
must confirm its used toolchain and targets, and cannot promise graceful
settlement across library panic or uncatchable death. A `Started` event remains
**spawn-reported success**, not confirmed exec. In the pinned implementation,
EOF on the spawn error channel also returns success if the child dies before
exec. For example, terminal INT may be pending after handoff and terminate the
child when its entry mask is restored. The attempted-group record supplies
no extra exec confirmation. Keep supervision/reap/recovery on this successful-
spawn-report path; it is distinct from a reported exec error.

### Specification and reference mismatches

The reference is protected as source/provenance, not treated as an oracle for
its known faults. The required expected differences are: signal propagation
with C1's no-ending error; refusal before resolution/launch in C2; original
partial-acquisition error plus checked cleanup diagnostics in C3; and terminal
recovery/reporting on C4. Normal cases retain their protected expectations.
The current selection-cancellation wording claims worker reap even when wait
is unconfirmed; change that wording only on the unconfirmed branch and retain
the original cancellation classification/signal.

The inventory also exposes unchecked handler/mask/terminal calls. In particular,
runner SIGCHLD repair currently occurs **after** policy evaluation; it cannot
establish worker waitability. The native contract therefore requires waitability
before the worker too. A real ignored-SIGCHLD/SA_NOCLDWAIT entry case must
discriminate that premise; it is not an observed runtime failure. No generic
“all OS failures repaired” claim follows from C1–C4. Failure of identity capture,
signal installation/restoration, output diagnostics or library panic must stay
explicit rather than normalised away in a comparison.

## Discriminating checks

These are construction/execution plans, not instruments delivered by this
design. Expected outcomes derive from O1–O10 and the agreements, held outside
the repair under judgment at the fixed reference. First demonstrate each fault
control fires against the unchanged behavior or a deliberate broken control;
then exercise the repaired path. A test that simply uses its new helper to
compute its own expected outcome is not a discriminator.

| Case and enabling point | Controlled condition and discriminator | Required observation and fixture responsibility |
|---|---|---|
| C1: existing private [`Process`/`FakeProcess`](../crates/keyed-launch/tests/internal/wait_events.rs#L9) | Set TERM before observation error; repeat with emergency reap success/failure and normal-exit observation followed by reap failure. Inject a second signal at failed reap and a late signal after recovery. Include no cancellation and differing earlier front cancellation. | Error preserves first consumed signal and original failures, no invented ending; Reaped only on successful wait. Existing group-before-reap event order survives. Front no-ending settlement propagates TERM with empty result stdout, keeps earlier front signal precedence, reports all cleanup errors and preserves the late signal separately. Fake process has no OS child; serialize its existing process-global latch. |
| C1: front settlement and public CLI / real policy / fake job | Feed retained runner error cancellation into the real no-ending settlement path; combine with channel-close/mask/output failure. Public CLI controls cover ordinary selection cancellation, managed cancellation and publication before cancellation. | No result, correct signal/primary precedence, all secondary diagnostics. Keep real fixture PID/group and wait them on every test exit; no claim that nominal CLI tests alone inject an OS wait error. |
| C2: new private evaluator teardown-outcome control | Perform actual worker stop/wait first, then substitute a reported wait failure before the success gate. Invoke front/selection composition in an isolated private fixture; managed program writes a start sentinel. Repeat successful conversation, policy refusal, timeout, output overflow and cancellation. | No successful evaluation, resolution/selection or start sentinel after unconfirmed reported reap; success becomes worker failure 5. Existing failure classifications remain primary except cancellation. Wait error and captured diagnostics survive, no false “reaped”. Control stores the real wait receipt so it never leaks the worker or destroys the diagnostic-bound control. No production env/CLI switch. |
| C3: new private allocator post-creation/cleanup-report controls | Fail after actual directory acquisition at confined preparation, held-reader open and appearance-channel allocation, and worker preparation. Then inject removal failure. Repeat worker cleanup failure after otherwise successful evaluation. | Original failure code/source/status plus exact failed-cleanup path/operation; all outer acquired resources are settled. No admission after successful worker conversation plus removal failure. Suppress false success and preserve manual-cleanup advice. Disarm local destructor after the checked attempt. |
| C3: existing real-directory identity boundary | Rename an owned directory and replace its name; include symlink replacement and identity-read failure before transfer. | Preserve replacement contents, report the moved/unknown-owned directory, never infer removal from a destructor. Fault controller's separate fixture guard removes actual debris once assertions finish; do not mask the returned report by using successful cleanup as the injected result. |
| C4: existing real [`in_pty`](../crates/keyed-launch/tests/job.rs#L328) boundary | Launch a missing executable through the runner's real pre-exec path; capture an actual changed foreground group before exec fails. Contrast a failure before handoff and a background launcher that owns no lease. Under the admitted exclusive lease, observe an unrelated holder and verify refusal to take it; do not infer atomic query/handback from that control. | Failed spawn has no Started/Reaped event claimed by the runner, keeps ENOENT classification, then restores launcher foreground and saved modes. Parent-observed handoff receipt plus actual `tcgetpgrp` establishes that this exercised C4, not only a missing-program test. No handback by background/pre-handoff controls; unrelated ownership is preserved under the stated serialization/identity premise. |
| C4/native: pre-exec death after observed handoff | In a private PTY fixture, use an async-signal-safe test barrier after real handoff, deliver terminal INT while blocked, then allow entry-mask restoration. The controller/fixture reap every child. | Spawn may report success and emit Started even though exec never occurred. Reaped still requires actual wait; group and terminal recovery occur through supervision. Child signal status is not invented launcher cancellation or exec confirmation, and missing publication still refuses. Keep this separate from the failed-exec error case. |
| C4: narrow private recovery-outcome control at the lease | After real failed exec/handoff, report failure from foreground query, handback, mode restore or temporary SIGTTOU restoration. Successful-supervision recovery error also checked. | Spawn/supervision primary and cancellation observation survive; each recovery error is reported and blocks success. Use a separate uninjected PTY fixture recovery to leave foreground/modes usable, kill/wait fixture children, close endpoints and bound reader completion. Nothing test-only runs in production pre-exec. |
| Native CLI entry at real process / PTY boundaries | Supply ignored and blocked INT/TERM/HUP and SIGPIPE, ignored SIGCHLD and SA_NOCLDWAIT; compare launcher-targeted INT with terminal-generated INT after foreground handoff. Signal during transition and after reap, including before no-ending failure settlement. | Capture-before-runtime child state, ignored/blocked semantics, worker and managed-child waitability, foreground-directed INT, late joining and result-or-signal exit. Keep signal cleanup failures explicit; runtime coverage is target/toolchain-specific. No inference of returning-host restoration. |
| Protected regression / final write | Existing real-policy CLI and transport cases: config/SDK/settings/discovery, direct argv, exact empty/Unicode/multiline bytes, first-writer, nesting/anchored regular reads, published result plus child failure, descendant cleanup, combined result/close failure, unwritable output and write failure after settlement. | Preserve codes/stages/statuses, diagnostic association and bytes; final write happens only after successful cleanup. A prefix on a failed final write is an accepted existing boundary, not a result to normalise into success. Use Taskfile tasks; retain complete per-case logs and untouched expectations. |

The evaluator outcome control needs no replacement OS-wait framework. A
scope-limited test-only report substitution at the existing teardown boundary
allows the real front admission path to be exercised. Allocator controls belong
inside their owners after creation, not in global filesystem behavior. Terminal
recovery reports need a narrow lease-local control; real handoff/exec/foreground
observations still come from a PTY. Test-process physical cleanup and injected
reported failure are separate receipts. No complete syscall abstraction,
public testing flag, owner-policy convention or packaging change is justified.

No formal model was built. Error-carried observations can be represented by
types, but source paths and the planned wait/PTY/failure controls answer whether
the real operations and diagnostics meet the contract. A diagram is not a
checked state machine and a model of those reports would not confirm an OS reap,
entry mask or terminal recovery. Runtime checks remain owed.

## Evidence limits and remaining work

This is correctness design on the incumbent, not an extraction or a supported
target/migration. Native delivery and an actual adopting project remain
unestablished. The closed comparison's reference/frame/history is unchanged:
one iteration, seven attempts, R=1/8, F=4/4, N=3/6, charged D≤45 and RC≤15
minutes, P=E=X=0. K6 separately charged D≤45 and RC≤10, P=E=X=0. No comparative
iteration, judgment, variant or catalogue read occurred here.

Carry G2 six-month upkeep/support and route cost, G3 delivery/use beyond entry,
G4 unread alternatives, G5 unexecuted runtime domains, and R-W compatible-runtime
and new-lifecycle debt. These repairs cannot establish simplification gain or
root completion. The root still owes independently useful capability delivery,
unchanged configuration/complete sessions, later supported target/migration
work, human book structure and the practical post-proof reader checkpoint.
Detailed funding and the next useful work order stay in the Grove brief/task
tree; they are not durable behavior of the tool.
