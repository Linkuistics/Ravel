# Completion failure repair evidence

The [completion specification](specs/result-job-completion.md#required-failure-settlement)
states the required corrections and one-session native launcher entry contract.
This document holds source locations, proposed private enabling points and
discriminating checks. C1 is now implemented with the
[C1 observations below](#c1-checked-observations). C2 admission and diagnostic
preservation now have [checked observations](#c2-checked-observations);
C3 now has [checked allocator/consumer observations](#c3-checked-observations).
C4 and native entry remain required work. A design or rendered diagram establishes no runtime
conformance. The earlier
[A0–A15 inventory and C1–C4](result-job-source-evidence.md#acquisition-and-stopping-state-inventory)
remain observations at fixed reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`.

## Obligation and source map

| Gap / obligation | Current source evidence | Local repair and hidden decision |
|---|---|---|
| C1 / O7, O6, O9 | [`watch`](../crates/keyed-launch/src/run.rs#L1217) samples at every final wait attempt and carries its first signal through `Failed.error`. [`supervise_reported`](../crates/keyed-launch/src/run.rs#L1177) composes that error with recovery reports. [`run`](../crates/ravel/src/run.rs#L32) reads the error observation before conversion even without an ending. | `LaunchError::consumed_signal()` retains numeric cancellation separately from message/errno. Front joins front, outcome, late and reports superseded errors. No global requeue, fabricated `Ended`, or failed-wait group assertion. Checked C1 evidence is below; actual checked reclaim remains C4. |
| C2 / O2, O3, O9 | [`evaluate`](../crates/ravel/src/worker.rs#L317) retains conversation and teardown reports, then checks confirmed wait and required removal before success. [`evaluate_and_resolve`](../crates/ravel/src/choice.rs#L118) resolves only after evaluation returns. | Evaluator gates successful evaluation on confirmed teardown. Keep conversation judgment, bounded output, timeout and cancellation precedence; retain wait/removal errors as secondary operational diagnostics when a conversation already failed. |
| C3 / O9 | [`Confined::prepare`](../crates/ravel/src/confinement.rs), [`SessionChannel::allocate`](../crates/ravel/src/completion.rs) and worker preparation retain a private [`PrivateDirectory`](../crates/ravel/src/private_directory.rs) from creation through transfer/cleanup. | Disarm pathname-only TempDir cleanup before identity capture, preserve original cause plus every checked cleanup report, and compare recorded device/inode/type before removal. Unknown identity refuses deletion; explicit attempts disarm Drop. Anchored reads remain separate. C3 evidence below limits the snapshot guarantee. |
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
the component comparison. Retaining the reference's error/latch behavior fails C1/C2's
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
The reference's selection-cancellation wording claimed worker reap even when
wait was unconfirmed. C2 uses teardown-attempt wording for cancellation, timeout
and overflow, retaining the existing cancellation classification/signal.

The inventory also exposes unchecked handler/mask/terminal calls. In particular,
runner SIGCHLD repair currently occurs **after** policy evaluation; it cannot
establish worker waitability. The native contract therefore requires waitability
before the worker too. A real ignored-SIGCHLD/SA_NOCLDWAIT entry case must
discriminate that premise; it is not an observed runtime failure. No generic
“all OS failures repaired” claim follows from C1–C4. Failure of identity capture,
signal installation/restoration, output diagnostics or library panic must stay
explicit rather than normalised away in a comparison.

## Discriminating checks

These originated as construction/execution plans. C1/C2 instruments and observed
outcomes are recorded below; C3 instruments now have observations too. C4 and
native-entry rows remain plans. Expected outcomes
derive from O1–O10 and the agreements, held outside
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
entry mask or terminal recovery. Runtime checks outside the C1/C2 observations below remain owed.

## C1 checked observations

The incumbent now preserves runner-consumed cancellation across observation/reap
errors and transfers it to front settlement without an ending. `LaunchError`
stores the first numeric signal separately from its message and primary errno.
Both final wait routes drain the latch before observers or recovery, preserving
the earlier consumed signal. Emergency-wait error text remains beside the
observation error. `supervise_reported` composes recovery reports with either
watch outcome; production forwards its existing unit reclaim as successful.
It does **not** detect real recovery syscall failures or implement C4.

The front's production conversion passes message, errno and consumed-signal
accessor values through `supervision_failure`, retaining the observation before
converting the error. Both outcomes feed the same front/outcome/late join. The
private front controls exercise that conversion and join before no-ending
`settle`, using fixed numeric observations; the cross-crate accessor connection
is source-checked, while injected `Process` controls check the error payload.
They do not inject an actual OS wait failure through the public CLI. Cancellation
reports the original failure and all available outer-cleanup reports through
the existing stderr boundary, including JSON invocations. Captured policy output
keeps its separate labels. Only successful settlement can reach the result write.

The [measured receipts](cancellation-failure-transfer-checks.txt) retain every
selected case status, successful parameter receipt, failed assertion and input
digest. On `aarch64-apple-darwin`, Rust 1.99.0 and nextest 0.9.148, each full
control run held all 138 tracked project source/input files fixed before/after.
These are project input manifests, not a freeze of generated binaries, runtime
caches or external dependency/tool installations. The initial narrow-manifest
exploratory controls were superseded by those full runs. Evidence/tree prose was
written after measurement; no behavior was edited while a run read its subjects.

| Command / controlled break | Observed status and discrimination |
|---|---|
| `task test:runner CLI_ARGS="-E 'test(wait_events)' --no-fail-fast"`, erase signal attachment while retaining the accessor | Task 201 / nextest 100. Each observation/confirmed-reap, observation/failed-reap, final-reap, recovery-report and error-annotation test fails missing HUP; existing nominal controls still pass. |
| `task test:runner CLI_ARGS="-E 'test(cancellation_survives_observation_failure) \| test(cancellation_survives_final_reap_failure)' --no-fail-fast"`, omit both final latch reads | Task 201 / nextest 100. All three routes fail with TERM still pending before observer or recovery. |
| `task test:runner CLI_ARGS="-E 'test(cancellation_survives_observation_failure_with_failed_reap)'"`, discard emergency-wait text | Task 201 / nextest 100. The distinct emergency-wait diagnostic assertion fails. |
| `task test:session CLI_ARGS="-E 'test(runner_error_conversion_joins)'"`, discard consumed signal in conversion | Task 201 / nextest 100. Retained HUP observation is absent. |
| `task test:session CLI_ARGS="-E 'test(run::tests)' --no-fail-fast"`, discard superseded primary report | Task 201 / nextest 100. Original stdout failure and supervision diagnostics are missing; classification/success and group controls pass. |
| Restore each correction; `task test:runner CLI_ARGS="-E 'test(wait_events)' --success-output immediate"` | Exit 0. All 11 cases pass, including every parameter case below and existing group/reap/recovery/duration/late-signal controls. |
| `task test:session CLI_ARGS="-E 'test(run::tests)' --success-output immediate"` | Exit 0. All four cases pass, including precedence, diagnostics, 127/126/5, success and surviving-group controls. |
| `task test:cli` | Exit 0. All 11 real-policy/fake-job cases pass, including configuration/settings/context, exact values, nesting/first-writer, cancellation/descendants and unwritable stdout/cleanup. |
| `task check` | Exit 0. Formatting, shell checks, worker build/probes, runtime integrity, worker types and workspace lint pass; all 141 selected workspace tests pass. Fourteen ignored fixture roles remain outside direct selection. Its full-workspace nextest already supplies `task test`; no redundant full run was added. |

The wait controls check each of three routes: failed observation with successful
emergency reap, failed observation with failed emergency reap, and observed exit
with failed final reap. **Every route** separately checks HUP/no second signal,
HUP/TERM at reap, no earlier signal/TERM at reap, and neither signal. Returned
signals are respectively HUP, HUP, TERM and absent. All preserve their original
operation text, plus distinct emergency-wait text when present; recovery receives
confirmed status only after successful reap. `Reaped` occurs only there. The
trace asserts forward/poll/KILL/wait/recovery for uncertain observation and
forward/poll/group-kill/wait/recovery after known exit, with `Reaped` inserted
only for confirmed wait and no group-confirmation claim on error. Latches are
serialized and drained before recovery; FakeProcess owns no real child.

Recovery-report controls separately cover successful watch and each failed-watch
route, both with and without HUP. The original wait error and reported recovery
error remain together; HUP remains the outcome signal. TERM inserted only during
recovery stays late in every case. No `Ended` or group confirmation is invented.
Error annotations separately retain first HUP, primary ENOENT and secondary text;
unrelated EACCES errors carry no cancellation.

Front no-ending controls check INT/HUP/TERM → INT, absent/HUP/TERM → HUP,
absent/absent/TERM → TERM, and all absent → original supervision failure 5.
Cancellation preserves original observation, emergency-wait and recovery text,
plus each mask/channel/private-directory/output cleanup report. The same refusal
serializes its signal for JSON; reports use the shared stderr boundary. Missing
executable, other failed start and post-start error controls retain 127, 126 and
5 with exec stage and original code/message. Ordinary successful settlement
preserves empty, Unicode and multiline bytes. Existing real-process/PTY cases
in the workspace run protect physical descendant cleanup, group-before-reap,
terminal recovery and signal propagation; private error controls do not prove
those syscalls failed in the real process.

The C2 and C3 observations below cover worker admission and allocator ownership.
C4 checked terminal recovery/failed exec and native entry remain mandatory. Started remains spawn-reported
success, not exec confirmation. C4's exclusive-lease/identity assumptions and
the distinction from a returning host remain conditional. These local checks
establish neither all OS failure behavior, cross-target qualification, final-write
atomicity, independent adoption nor a component target. A failed final write may
still have emitted its accepted existing prefix.

## C2 checked observations

The evaluator now closes its protocol endpoint, attempts worker stop/kill/wait,
drains bounded policy diagnostics and explicitly closes its private directory
before deciding admission. A judged success reaches resolution only after
confirmed wait and successful required removal. Nonzero exit still confirms
reap. Failed conversation classifications remain primary except cancellation;
output overflow retains its existing precedence and keeps the original EOF,
timeout or refusal as context. Cancellation retains first-signal precedence and
superseded metadata/remedies. Operational reports name the worker/PID, operation,
error and cleanup path; policy stdout/stderr remain separate bounded byte streams.
Recovered operation reports travel through choice to the front's existing stderr
reporting boundary without a new JSON schema field.

The private test-only report transform runs after actual teardown/removal and
before that production decision. Ignored subprocess roles exercise real
`choose_prepared` and `run`, including `Selected` absence, missing-program 5
rather than resolution's 127, absent start sentinel and empty product stdout.
Input-receipt logs are explicitly excluded from cancellation-output assertions.
A fake protocol worker provides readiness/protocol/EOF/nonzero controls; ordinary
choices and complete sessions use the real compiled TypeScript worker. Physical
wait status, PID, elapsed bound and directory removal precede each injected
report. Fixture capture uses files and bounded polling; its timeout control
confirms emergency kill and real reap. No public fault switch or owner-policy
failure convention implements the report transform.

The [measured receipts](worker-reap-admission-checks.txt) retain named per-case
outcomes, before/after SHA-256 input manifests, commands and status. Long runs of
identical `x` policy bytes use a reversible repetition notation, verified back
to each raw log's digest. Every measured input manifest matched before/after.
These manifests freeze tracked project sources/fixtures/configuration, Cargo.lock,
Taskfile, scripts and task/spec expectations, plus the measurement wrapper.
Generated binaries/caches, installed dependencies/tools and external runtime
state are not frozen. The host is macOS arm64 with Bun 1.4.2 and nextest 0.9.148;
this is not cross-target or native-entry qualification.

| Instrument | Observation |
|---|---|
| `task test:list` with C2 filter | All 35 named parameter/instrument cases are listed. Two ignored subprocess roles are invoked by the controls. The mandated `test:session --list` form was rejected by nextest; the new reusable listing task uses `nextest list`. |
| Additive-seam red, corrected fixture | Intended resolution/gate and lost-report assertions fail. An earlier unknown-settings fixture failure is recorded but earns no conformance evidence. |
| Final gate-removal mutant | Missing-program resolution returns `program_not_found` instead of `worker_failed`. |
| Final diagnostic-loss mutant | Original primary classification remains, but worker identity/secondary reports and cancellation output assertions fail. |
| Reviewer-derived EOF/deadline + overflow + cancellation controls | Red loses original conversation information and claims stopping; corrected cases retain all metadata and use truthful attempt wording. |
| Restored focused C2 | All 35 cases pass: nine success-gate combinations, refusal/deadline/protocol/EOF/overflow combinations, recovered poll, actual nonzero exit, INT/TERM/HUP/late/real-teardown cancellation, mixed conversation/overflow/cancellation, exact empty/Unicode/multiline sessions, and fixture deadline/reap. |
| `task test:cli` | All 12 cases pass. Existing 11 assertions remain protected; the combined stdout/confined-cleanup stimulus now targets only the confined directory. A separate real-permission case verifies checked worker-removal refusal plus outer cleanup reports. |
| Focused C1 runner/front | All 11 wait-event and four front controls pass, preserving first signal, original/secondary reports and no-ending settlement. |
| `task check` | Formatting, scripts, worker build/probes, runtime integrity, types and lint pass; all 177 selected workspace tests pass, including real process/PTY cases. Sixteen ignored fixture roles remain outside direct selection. This supplies `task test`; no identical full rerun is needed. |

One fresh reviewer returned three actionable findings: false timeout stop wording,
blocking fixture capture/wait without an effective deadline, and original EOF/
deadline loss under overflow. Each is corrected with a discriminating executable
control; no second reviewer or architectural redesign was commissioned. An
oversubscribed parallel C2/CLI run produced real pre-readiness timeouts and is
not credited green. A scoped nextest scheduling reservation now prevents C2
fixtures from competing within a run, without retries or deadline widening;
subsequent measurements run sequentially. A mechanical Clippy warning was fixed
before the successful workspace run. Sources restored after the final mutants
match that workspace's measured product hashes; the focused rerun independently
confirms every C2 item.

At the C2 boundary, C3 partial preparation/identity cleanup remained owed;
the [later C3 observations](#c3-checked-observations) address that slice. C4
checked failed-exec/terminal recovery and native waitability/signal transitions
remain owed. Inherited auto-reap,
detached policy children, library panic and uncatchable death are not discharged
by substituted reports. No complete failure-path conformance, extraction,
independent adoption, migration or simplification gain follows.

## C3 checked observations

Each of the three allocators now owns its directory immediately after creation,
disarms `TempDir` pathname cleanup, and records device/inode/type before worker
or managed-process access. A local preparation error explicitly settles that
owner before returning; only success transfers it. Confined and session owners
retain the same identity for later explicit cleanup and best-effort Drop. Worker
ownership lasts through real teardown, bounded capture and required removal.
Reap plus required removal still gates successful evaluation before resolution.
A failed wait remains unconfirmed even if a directory-removal attempt succeeds.

Cleanup disarms its destructor before any identity/removal check. Missing/moved,
replaced, symlinked and unknown identities refuse deletion, preserving unrelated
contents and supplying the exact owned path, operation, uncertainty and advice
to inspect that path and locate the moved directory. The code neither knows nor
invents its new location. Original acquisition classification, source, remedy
and detail remain primary without cancellation. The session I/O wrapper retains
the original kind and typed source; errno is accessible on the original I/O
source, while `raw_os_error()` on a contextual `io::Error` wrapper is `None`.
Real appearance-channel errors now retain that I/O source in `LaunchError`;
its existing spawn-only `raw_os_error()` accessor is unchanged.

[Measured receipts](partial-acquisition-cleanup-checks.txt.gz) are a gzip-compressed
UTF-8 ledger retaining commands,
status, named outcomes and before/after SHA-256 subjects. `green-physical-final` additionally
retains every case's physical acquisitions/identity/cleanup attempts and separately
substituted reports, outside captured product stderr. All measured manifests
match before/after. Tracked project inputs, Cargo.lock and the measurement wrapper
are frozen; generated binaries/fixtures/caches, installed tools/dependencies and
host state are not. The receipts name collection snapshots of the generated
orchestration scripts separately from measured subjects. Long `x` runs have a
reversible encoding, expanded and digest-checked before storage. macOS arm64,
Bun 1.4.2 and nextest 0.9.148 are the executed domain, not cross-target qualification.

| Instrument | Observation |
|---|---|
| Fixed nextest listing and restored focused run | All 97 selected C3 cases pass; one ignored process-isolated role is invoked by the controls. Creation failures acquire no fictional path. Identity-capture failures leave deletion explicitly unconfirmed. Post-creation failures execute actual acquisition before substitution. |
| Allocator preparation and transferred ownership | Confined canonical/runtime/protected/tmp controls, real runtime validation, separate session reader/channel controls, and worker pair/duplicate/descriptors/missing or nonexecutable spawn controls preserve original error and all local reports. Every acquired descriptor closes. No worker failure reaches Selected/start; channel failure follows selection but prevents managed start/result. |
| Real identity fixtures | Every allocator preserves prior moves, directory replacements and symlink targets, before transfer and at later cleanup. Failed removal leaves real observable debris until fixture cleanup. Restoring a moved owned inode to its original name after explicit failed cleanup does not rearm Drop. |
| Actual channel error | Moving the directory before the underlying Channel allocator produces a real ENOENT; the original NotFound kind, typed source and errno survive checked cleanup context. This is distinct from post-success EIO substitution. |
| Failure composition | Refusal/deadline/protocol/overflow combine physical C3 residual ownership with reported C2 wait/poll failure. First INT/TERM/HUP and late choice override retain primary metadata, bounded policy output, local/outer cleanup and wait reports. Actual wait precedes substitutions; nonzero exit confirms reap but failed removal still gates admission. |
| Four distinct mutants | Diagnostic loss fails cleanup advice/report assertions; bypassed identity fails replaced/symlinked ownership or admission; unconditional destructor removal erases real debris and the restored moved-path sentinel; omitted removal gate admits Selected or permits missing-program resolution 127 instead of worker failure 5. All mutations are restored. |
| No-fault complete sessions | Selected/start and exact empty/Unicode/multiline result bytes survive cleanup; a confined complete session passes too. Existing anchored reads, first-writer publication and nesting remain covered. |
| Protected checks | CLI 12, C2 35, C1 front 4 and runner wait-events 11 all pass, sequentially. The scheduling reservation includes C3, with no deadline widening or retry-to-success. |
| Workspace | `task check` passes formatting, scripts, worker build/probes, runtime integrity, typechecking, lint and all 283 selected tests; 17 ignored fixture roles remain outside direct selection. Each focused named outcome is independently matched to the workspace outcome, rather than comparing totals. |

The one reviewer found real channel error conversion loss, missing outer-confined
worker controls and missing conversation/C2/C3 combinations. Each was verified
and corrected with executable discriminators; no second reviewer or redesign was
commissioned. Earlier CLI-argument, error-kind/cancellation-code and sentinel
placement fixture mistakes, plus one compile-only failure, are retained but earn
no conformance credit. A missing-program gate mutant initially exposed a weak
fixture assertion; strengthening it against the fixed worker-5 contract makes
both missing and valid program controls fail the final mutant.

Snapshot comparison/removal is not atomic under concurrent pathname mutation.
Private removal/wait substitutions do not establish actual OS syscall failure;
physical residual fixtures and the real ENOENT observation establish only their
named operations. Fixture-parent removal after assertions never changes the
reported production outcome. Panic, uncatchable death, detached policy children,
native auto-reap and unchecked signal/mask/terminal premises remain outside these
checks. C4 and native entry remain required. No extraction, supported component
target, independent adoption, migration, comparison restart or simplification
gain follows from C3 success.

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
