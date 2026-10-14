# Ravel's component contracts

Ravel evaluates an executable owner policy, launches its selected command, and
returns one opaque result after ending the managed job. Understanding those
three acts explains the language and package boundaries better than counting
files. Start with the responsibility table, then follow a successful session
and cancellation in the [visual companion](design/components/index.html).
The [glossary](../CONTEXT.md) distinguishes the caller, owner policy, policy
worker, managed process and session-result context.

This is an explanation of the current implementation, grounded in the
[evidence below](#source-and-observation-evidence). It chooses no replacement
architecture. The [completion invariant](specs/result-job-completion.md)
explains the matching-job and settlement obligations behind the boundary
comparison; its regrouping proposal remains unselected.
The [session specification](specs/session-execution.md) owns
behavioural promises; the [CLI](session-cli.md), [configuration](owner-configuration.md),
[runner](local-runner.md) and [result-channel](session-channel.md) guides own
their detailed interfaces. The tables here connect those contracts to owners.

## Responsibilities before packages

| Responsibility | Interface and guarantee | Caller assumptions | Hidden decisions, state and effects | Dependencies and failure composition |
|---|---|---|---|---|
| Admit configuration and request | Prepare a request from CLI data, settings and the admitted policy path; do not infer a caller task convention | Caller supplies the prompt; the owner maintains a readable TypeScript policy and supported settings | Front owns path resolution, defaults, validation and reading prompt/caller-context bytes | Filesystem, caller cwd/HOME, settings and limits; invalid inputs stop before a selected job starts ([E1](#e1-admission)) |
| Evaluate executable policy | Import the admitted entry, call optional `loadContext`, then `select`; report data over a private framed socket | Owner code is trusted executable configuration; arbitrary direct I/O is possible, while host reads have measured/bounded semantics | Worker owns the policy object, frozen request/context, host-read ledger and language execution; front owns deadline, output bound, identity admission and judgment | Compiled Bun worker, embedded SDK, owner imports, front protocol; refusal/throw/malformed data/timeout never selects a fallback ([E2](#e2-policy)) |
| Resolve and compose a selected session | Turn the judged command into exact argv and a supervised job; return only settled result bytes | Selected program resolves in the caller's execution context; ending instructions and result meanings come from the caller | Native front owns selected-route data, output reservation/redirection, signal transitions, session allocation and explicit cleanup | Admission, policy evaluation, executable lookup, result transport and runner; any cancellation or supervision/cleanup failure prevents success ([E3](#e3-composition)) |
| Supervise a local job | Accept `Argv` and `Launch`; return reaped status, ending cause, channel appearance and group confirmation | Caller supplies environment scrub/grants, channel name, escalation and any transparent entry signal state; caller checks `Group::Gone` | Runner owns process-group setup, signal forwarding, escalation, reap, terminal foreground/mode recovery and confinement backend | OS process/terminal facilities; does not judge opaque results or routing; launch/wait failures are errors and surviving groups are explicit outcomes ([E4](#e4-job)) |
| Carry one result through completion | Allocate a private channel; publish once; read after the matching launch; explicitly close | Consumer pairs the channel with its own `Ended`, rejects cancellation and handles cleanup errors; child receives only its enclosing context | Transport owns atomic publication, anchored bounded regular-file reads and directory-identity cleanup; runner observes appearance only | Filesystem and appearance channel; missing publication differs from an empty result; invalid/stale context and invalid files refuse ([E5](#e5-result)) |
| Deliver the matched executable pair | Build/install a native front with its matching compiled worker, SDK and notices | Build uses the pinned runtime and matching identity inputs; execution uses one installed prefix | Build scripts and Rust build step own shared source identity and layout; no installed host TypeScript runtime is required | Rust/Bun development tools and release tooling; missing/mismatched worker refuses before policy is handed over ([E6](#e6-delivery)) |

These responsibilities do not imply six packages. Several are coordinated
inside the native front, and policy evaluation crosses a process boundary.

## Languages, packages and runtime participants

The workspace has two Rust packages, `ravel` and `keyed-launch`, both marked
`publish = false`. The TypeScript worker is a separate build unit inside
`crates/ravel/worker`, compiled into the installed `ravel-policy` executable.
The SDK and examples are embedded modules and also ship as source/declarations
for configuration authors. See [workspace/package evidence](#e6-delivery).

The `ravel` package has two Rust targets with different roles. Its executable
contains request admission, policy judgment, session composition and CLI
reporting. Its library exports the result transport (`SessionChannel`,
`publish_result`, `end_session` and associated constants). The executable uses
that library and `keyed-launch`; the library itself depends on `keyed-launch`.
This is a source-package distinction, not an extra running service.

`keyed-launch` runs within the native front's process. The separate policy
worker runs first and is waited for before the selected managed process starts.
`end-session` is another invocation of the native executable, usually launched
by the managed process. It publishes to the inherited path and returns; the
already-running front performs shutdown. There is no persistent session daemon.

### Why TypeScript exists

Configuration is executable TypeScript, including asynchronous functions,
imports and ordinary filesystem/network computation. It is not a static routing
file that Rust can parse and replace without changing the configuration contract.
Bun supplies that execution environment; Rust retains the parent's environment,
terminal and supervision authority, admits the worker's identity, judges its
serialized data and enforces bounds. The worker gets neither the caller's
completion authority nor the selected managed process's channel.

That division has delivery and resolver costs: a second build tool, a compiled
runtime, a framed protocol and controls split across native startup, compile
flags and worker code. Their rationale and accepted limits live in the
[policy-worker decision](adr/policy-evaluation-precedes-the-launch.md).
The unchanged-configuration promise requires continued execution of existing
TypeScript policies even if future internal code is regrouped or rewritten.
A Rust-only implementation is therefore an open compatibility and runtime
question, not a conclusion of this map.

### Why there are two Rust packages

The runner hides OS job mechanics behind argv, launch options and an ending.
It does not know policy paths, provider/model labels, Ravel result meanings or
caller task trees. Keeping that responsibility separate currently permits
runner tests and native Rust consumers to exercise job semantics without the
policy worker. Its sole production dependency is `libc`.

The price is an explicit composition contract: `Launch` leaves scrubbing,
signal transparency, completion-channel choice and interpretation of `Ended`
with its consumer. Ravel adds its own result validation, explicit temporary
cleanup and CLI status rules. Moving those obligations into another package
would need to preserve their owner and ordering. The existing split is an
evidenced responsibility boundary; its current packaging is not a verdict on
the best future arrangement or on public reuse.

## Scoped behavioural obligations

The human's compatibility promise and the active specification authorise these
requirements. This map covers the component question, not all legacy consumers
or every execution. “Source” denotes inspected implementation; “observed”
denotes the three named tests rerun below. An existing acceptance seam is a place
to check a promise, not a claim that it was exercised in this investigation.

| ID | Required outcome and provider guarantee | Operational owner; consumer/environment assumption | Evidence and existing acceptance seam |
|---|---|---|---|
| O1 | Existing owner TypeScript policy, SDK use, settings and discovery keep working; explicit policy entry/default admission stays unchanged | Native admission + worker/SDK; owner supplies supported files/imports, caller supplies cwd/HOME | Source E1/E2; `cli` context/settings cases, authority/settings unit cases, `task typecheck`, installed `task smoke` |
| O2 | Policy execution has no session-result authority; identity is admitted before policy handover and the worker is waited for before the managed launch | Native worker coordinator; trusts owner policy, not arbitrary protocol values; OS supplies wait/descriptor semantics | Source E2/E3; context/environment, identity/deadline CLI cases and `task test:hostile` |
| O3 | Policy decisions, explicit refusals, exceptions, malformed values and limits are judged without fallback; prompt/params remain caller data | Front judgment + worker host; framed socket is protocol, stdout/stderr are bounded diagnostics | Source E2/E3; `cli` refusal/context/deadline cases; exact-prompt case observed |
| O4 | Selected argv executes directly in the caller's context, with requested confinement or refusal, and with a fresh nested result context | Front resolves/assembles; runner launches; owner supplies command words, caller grants runtime files when needed | Source E3/E4; `cli` nested/confined cases and runner argv/environment cases |
| O5 | First complete publication wins; empty, multiline and Unicode values remain exact; stale/missing context and invalid result files refuse | Result transport; managed code publishes through inherited context; same-user tampering is outside authentication guarantees | Source E5; publication/regular-file unit cases, nested/stale process and CLI cases; exact bytes observed |
| O6 | Result success requires the matching launch's reap and confirmed group cleanup; published result does not make the child's later status decisive | Runner enforces lifecycle; transport checks `Ended`; front settles remaining errors | Source E3–E5; exact-result and published-then-cancelled cases observed; runner descendants/wait-error tests |
| O7 | INT/TERM/HUP cancellation suppresses results, including already-published values, and propagates as a signal after cleanup | Front signal transition + runner; signal delivery can overlap selection, spawn, supervision and settling | Source E3/E4/E5; selection/managed cancellation and published-result cancellation observed; signal-transition unit cases |
| O8 | Interactive foreground and saved terminal modes are recovered, with descendant-group cleanup before reap | Runner; caller's actual terminal ownership determines handoff; descendants deliberately leaving the group are outside cleanup | Source E4 and [job ADR](adr/the-launched-child-is-a-job.md); existing real-process/PTY runner tests, not rerun here |
| O9 | Only result bytes reach stdout, after explicit cleanup; child stdout and diagnostics use stderr; cleanup/write errors cannot become success | Front output reservation + settling; result stdout must be writable, OS permits descriptor/mode restoration | Source E3; exact separation observed; unwritable-output/cleanup CLI case and settling unit cases |
| O10 | Installed execution uses the matched front/worker pair and shipped SDK without another project's installation or a host Bun/Node | Build/install owner + front identity admission; matching installed prefix is intact | Source E6; fresh local build observed; installed/symlink smoke and target archive checks not rerun here |

Detailed bounds and exit-code assignments remain in the
[specification](specs/session-execution.md#request-context-and-selection) and
[CLI guide](session-cli.md#inputs-and-exit-behavior). No difference from these
promises is authorised by the component analysis.

## Composition and coordination

The [package/process overview](design/components/index.html#diagram-composition)
separates in-process calls from IPC, configuration and filesystem observations.
The following textual relations are the checkable meaning of its arrows.
These relations describe the inspected nominal composition; only the named
scenario outcomes were observed at runtime. The
[acquisition evidence](result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
qualifies failure-path discharge: worker reap is attempted but not checked
before successful selection. Runner errors now retain consumed cancellation;
the [C1 checks](completion-failure-design-evidence.md#c1-checked-observations)
cover its no-ending transfer without establishing all failure-path guarantees.

| Relation | Kind and direction | Information/guarantee carried | Obligations |
|---|---|---|---|
| R1 | CLI: caller → front; result: front → caller | Request and opaque bytes; caller retains meaning | O1, O3, O9 |
| R2 | Configuration: owner files → admission/worker | Admitted entry and supported settings; imports resolve from admitted files | O1 |
| R3 | IPC: front ↔ worker | Identity, evaluate/context/select exchanges and judged data over fd 3; no result path | O2, O3 |
| R4 | Order: worker wait completes → managed spawn | Selection completion alone does not authorise overlapping the worker and job | O2, O4 |
| R5 | Call: front → transport and runner | Fresh channel paired with launch; exact argv, scrub/grants, signals and escalation | O4, O6, O7 |
| R6 | Shared file: publisher → channel; channel → runner/transport | First-writer result; appearance for runner, bounded anchored contents for transport | O5, O6 |
| R7 | Order: group cleanup → reap → terminal/group confirmation → front settle → result stdout | No successful publication to the caller while a managed group survives or cleanup/cancellation prevents it | O6–O9 |
| R8 | Build data: shared identity inputs → native front and worker | Protocol, package version and source identity must agree before policy handover | O2, O10 |

The contract owner is the human; the implementation enforcers are in the tables.
Historical co-change was not sampled, so no co-change arrow or claim is made.
No durable replicated store participates in these inspected paths; the shared
mutable state is the ephemeral channel and OS process/terminal state.

### A successful operation

The [successful-session sequence](design/components/index.html#diagram-success)
follows the exact-result CLI fixture. Admission prepares the request. The front
starts a cleared-environment worker in a private directory, admits its hello,
and coordinates loading, optional context and selection. The worker moves to
`/` before registering embedded modules or importing the owner entry. Judgment
checks the reported policy/context/command. Evaluation closes the socket,
waits for the worker and collects bounded diagnostics; executable lookup follows.

The front reserves result output, allocates the fresh channel and calls the
runner. The fixture prints ordinary child output, invokes `end-session` with
its value, then exits 7. Publication makes a fully written inode appear using
a no-clobber hard link. The runner observes appearance or natural exit, kills
remaining group members before reap, recovers the terminal and confirms group
state. The front reads through the held directory, closes session resources,
restores output and settles errors/signals before writing the exact result.

The rerun checked ordinary, empty, multiline, option-looking and Unicode results,
unchanged multiline prompt data, stderr separation and an empty temporary
directory afterward. This fixture does not force the 2-second grace to expire;
the runner's escalation tests are the separate seam for that branch.

### Cancellation and partial failure

The [cancellation sequence](design/components/index.html#diagram-cancellation)
shows two phases of the same request composition. During selection, the native
handlers latch TERM, evaluation stops/waits for the worker and the managed job
is never reached. During managed execution, the runner forwards cancellation,
ends the group, reaps and returns an interrupted ending; the front cleans up
and propagates the signal without result stdout. The CLI fixture checked both
phases, temporary cleanup, and disappearance of the managed PID/group.

A separate transport/runner fixture publishes first and then sends TERM to its
supervisor. It checks an interrupted ending with `Group::Gone`, rejects reading
a result and closes the channel. This observed case supports the race's lower
composition layer; it does not exercise every late-signal window of the CLI.
`run` additionally joins front, runner and late runner latches, then passes the
selected signal to `settle`.

That join includes `LaunchError::consumed_signal()` when supervision fails
without an ending. The first runner-consumed signal survives observation/reap
failure, and the front reads it before converting the error. Cancellation also
reports the original operational error and all supplied cleanup diagnostics.
The [C1 evidence](completion-failure-design-evidence.md#c1-checked-observations)
records injected failure controls and the checked regressions.
The [acquisition inventory](result-job-source-evidence.md#acquisition-and-stopping-state-inventory)
also distinguishes destructor-only partial-directory cleanup, unchecked worker
reap, and an interactive failed-exec path that bypasses terminal reclaim. These
are source-admitted limits under O2/O7/O8/O9, not authorised changes to them or
measured runtime failures. Ordinary selected-scope cleanup still runs on
output/channel/launch failure without an ending.

The [failure repair contract](specs/result-job-completion.md#required-failure-settlement)
now assigns each correction to its existing resource owner and retains original
failure/cancellation precedence plus secondary diagnostics. The
[discriminating checks](completion-failure-design-evidence.md#discriminating-checks)
remain plans for C2/C3/C4 and native entry. C1's checked transfer changes its
current-source limit; it selects no component arrangement.

These participants have different local knowledge. The publisher knows only
the inherited destination and whether publication succeeded. The runner knows
channel appearance, child/group state and its signal latch, not result contents.
The transport can inspect file contents and an `Ended`, but cannot prove that
ending belongs to its channel. The front knows that pairing and the remaining
cleanup/output/signal results. The caller receives bytes only after the front
joins those observations. A readable file alone is insufficient evidence.

### Error and signal precedence

`run` takes the front's latched signal first, then the runner outcome's consumed
signal (interrupted ending or error), then a late runner signal. When cancellation
supersedes a failure, `settle` first reports that original failure, keeping policy
output separately labelled. It reports every supplied cleanup error
and any surviving-group diagnostic before selecting the primary outcome.
Cancellation wins over result and cleanup failures. Without cancellation, an
existing result failure stays primary; with an otherwise successful result, the
first cleanup error wins, then a surviving group prevents success. For the
managed path, cleanup results are supplied in mask-restoration, channel-close,
confined-directory removal and output-restoration order. Only a successful
settlement reaches the final result write, whose failure is still an error.

Preserving “no success on failure” alone would miss that contract. A boundary
change must also retain diagnostics and replay cancellation plus channel-close
failure, and result failure plus channel-close failure. The existing
`cancellation_overrides_prelaunch_failure_and_reports_every_cleanup_error`
unit case checks combined cancellation/result/cleanup failure; the
`cancellation_reports_a_surviving_group_and_suppresses_the_result` case checks
the cancellation/group diagnostic. The noncancelled combined-result/close case
is a source-supported precedence rule and a check owed by a touched implementation,
not an observed case of the three process tests above.

If worker judgment refuses, there is no managed launch. If result allocation or
output reservation fails, front cleanup still runs. Invalid contents, a surviving
group, explicit cleanup failure or result-output failure cannot produce success.
Uncatchable front death, same-user tampering and descendants leaving their group
remain the accepted limits in the specification and ADRs. This manual local
obligation replay checks the represented order and information; it is not a
proof of all admissible interleavings.

The [native-entry contract](specs/result-job-completion.md#acquisition-and-entry-limits)
is separate: child entry-signal values do not restore a returning host's
dispositions. Runner termination handlers and SIGCHLD repair persist; the CLI's
selection guard restores its saved INT/TERM/HUP only. No native restoration or
concurrent-embedding guarantee is inferred from the package boundary.

## An independent-change scenario

Consider changing how a result is stored/published while retaining first-writer
publication, exact bytes, nested isolation and success-after-cleanup. This is a
design probe, not a requested behavioural change or an implemented alternative.
The change needs the transport's allocate/publish/read/close contract, the
runner's appearance-only observation, and the front's channel-to-launch pairing,
explicit cleanup and error/cancellation precedence. Atomic appearance must still
mean a complete value; an empty published value must remain distinguishable
from no publication. The policy's routing logic need not change.

This reveals a concrete current reasoning cost. The consumer must understand
`SessionChannel::channel`, `Launch.channel`, `Ended.group/end/signalled`,
`SessionChannel::result_after/close`, and front settling. The Rust interface
accepts an unrelated `Ended`; correctness depends on the consumer following
the pairing protocol. The production front follows it and the inspected tests
exercise it. This is a caller obligation, not evidence of a current bug.
The same mechanism is assembled independently in the process fixture
([E7](#e7-change-footprint)), exposing the protocol a future consumer would face.

## Boundary questions and retained costs

| Concern | Evidence or remaining question | Consequence for separate reasoning |
|---|---|---|
| TypeScript/native boundary | Executable owner policy is required; native authority and worker protocol are source-verified. Equivalent support in a different runtime has not been established | Removing TypeScript syntax/support would break configuration. Keep the runtime/authority rationale visible; reopen implementation choice for concrete delivery/support pressure |
| Generic job reuse | Runner's public Rust interface avoids policy conventions; private package, Unix mechanics and explicit caller obligations remain | An external consumer example and delivery/support arrangement are still required before claiming independent usability |
| Result-aware job reuse | Pairing, cancellation, explicit cleanup and output settlement span transport, runner and front | A new package alone may export the same protocol. Compare ownership/interface arrangements before extraction |
| Worker lifecycle versus job lifecycle | Worker uses a private protocol, bounded diagnostics, descriptor isolation and pre-policy startup controls; managed job uses foreground/group supervision | Similar spawn/wait code does not establish an interchangeable contract. No generic-worker extraction is selected |
| State/effects and composition cost | The result channel and OS state have distinct readers/owners; no consumer can infer all success conditions from its own local state | Hiding file representation is useful only if cancellation and cleanup knowledge also have a responsible owner |

The first bounded comparison should address **the consumer's responsibility for
joining result publication with its own supervised ending and cleanup**. It is
directly on the complete-session path and also constrains reuse of a capability
currently inside `ravel`. The current composition remains the baseline.
Compare retaining it with arrangements that change interface/ownership,
including regrouping; do not assume a package split removes this reasoning.
The process bounds and judging frame live in the Grove continuation, not here.

This is a prioritisation supported by the scoped source/change evidence, not
proof that a redesigned boundary will be clearer. Human comprehension and
external consumer usability remain unassessed. Existing configuration and
session semantics, terminal recovery and runtime controls remain obligations.

## Source and observation evidence

The source reference is commit
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`. Links below point to the current files;
retrieve the anchor with `jj file show -r <commit> <path>` when they change.
Graph discovery used project
`Users-antony-Development-Ravel.simplify-and-componentize`, Tier 2 verification,
generation `2026-10-14T03:03:14Z`. Searches and traversals used qualified names;
their relevant pages were untruncated. Exact snippets and direct source checks
verify the material claims, including top-level worker startup and cross-process
wiring. One reported edge from `result_after` to a test `Scratch::new` did not
match the source and was discarded. Graph edges are navigation, not runtime
evidence. Coverage checks reported no recorded gap and matching metadata for
the cited current source paths; that is best-effort, not completeness proof.
Coverage refreshed at generation `2026-10-14T03:28:35Z` while production source
remained unchanged. The graph partially parsed the new Mermaid presentation
(composition lines 1–34, sequence frontmatter lines 1–8, and cancellation line 24);
the complete diagram sources were read directly and checked in the browser.

### E1: Admission

[choice.rs](../crates/ravel/src/choice.rs) `prepare` (43–58),
[authority.rs](../crates/ravel/src/authority.rs) `resolve` (34–117),
[settings.rs](../crates/ravel/src/settings.rs) `Settings::read` (30–93),
[inputs.rs](../crates/ravel/src/inputs.rs) `Inputs::read` (73–132), and
[environment.rs](../crates/ravel/src/environment.rs) `Grants` and grant exclusions.

### E2: Policy

[choice.rs](../crates/ravel/src/choice.rs) `choose_prepared`,
`evaluate_and_resolve`, `judge`;
[worker.rs](../crates/ravel/src/worker.rs) `locate`, `evaluate` (316–529),
`converse` (572–628), `verify_hello`;
[worker main.ts](../crates/ravel/worker/src/main.ts) startup (1–80),
`converse`, `load` (196–223), `assemble`, `select` (304–330).
The [runtime evidence](runtime-evidence.md) owns resolver/startup probe results
and upgrade limits; those probes were not rerun here.

### E3: Composition

[main.rs](../crates/ravel/src/main.rs) `main` (29–57) routes both verbs;
[run.rs](../crates/ravel/src/run.rs) `run` (32–247), `settle` (250–294)
and output reservation/restoration; [program.rs](../crates/ravel/src/program.rs)
`resolve`; [confinement.rs](../crates/ravel/src/confinement.rs) `Confined::prepare`.

### E4: Job

[runner lib.rs](../crates/keyed-launch/src/lib.rs) public contract;
[runner run.rs](../crates/keyed-launch/src/run.rs) `Launch`, `Ended`,
`run_observed`, `run_noninteractive`, `run_confined_observed`,
`supervise` (1160–1195) and `watch` (1197–1307).

### E5: Result

[ravel lib.rs](../crates/ravel/src/lib.rs) exports;
[completion.rs](../crates/ravel/src/completion.rs) `SessionChannel`,
`result_after` (56–70), cleanup/read, `end_session` (135–145),
`publish_result` (150–188) and regular-file bounds.

### E6: Delivery

[workspace manifest](../Cargo.toml), [ravel manifest](../crates/ravel/Cargo.toml),
[runner manifest](../crates/keyed-launch/Cargo.toml),
[build.rs](../crates/ravel/build.rs), [dispatch.sh](../crates/ravel/scripts/dispatch.sh),
[Taskfile](../Taskfile.yml) and worker identity admission. Existing install/archive
contracts are documented in the specification and [delivery guide](RELEASING.md).

### E7: Change footprint

[run.rs](../crates/ravel/src/run.rs) composes the product;
[session_process.rs](../crates/ravel/tests/session_process.rs) `supervisor_fixture`
(134–193) composes transport/runner without policy evaluation. Its caller-supplied
scrub and escalation make the enabling choices visible. This is a test consumer,
not evidence of adoption by an external project.

### Observed scenarios and limits

On 2026-10-14, on this macOS host, `task setup` and `task build` succeeded using
the pinned development runtime and locked Rust dependencies. The existing
`task test:session` seam was run with this exact nextest filter:

```sh
task test:session -- -E 'test(routed_result_is_exact_prompt_is_unchanged_and_child_stdout_is_separate) | test(cancellation_during_selection_launches_nothing_and_during_session_reaps_group) | test(cancellation_reaps_the_child_and_discards_an_already_published_result)'
```

All three named cases passed; 74 other cases were outside the filter.
[cli.rs](../crates/ravel/tests/cli.rs) supplies the real-policy/fake-program
fixture, including `env_clear`, temporary HOME/TMPDIR and fixture PATH;
[session_process.rs](../crates/ravel/tests/session_process.rs) supplies the
publish-then-cancel process fixture. No personal policy or real model service
was used.

Before and after that run, SHA-256 covered `Cargo.toml`, `Cargo.lock`,
`Taskfile.yml`, the built front/worker, and every file under `crates`, `scripts`
and `.cargo` excluding dependency `node_modules` and generated `target` trees.
The sorted subject-list digest was
`1c97953a8012bf60c9f9c3a4cef63d14c63a11fa29c3db937220b695a52b72b6`;
the identical before/after digest-list identity was
`c5be21debad205bd914bfaf71de19a086f79a542c757827620732df25f8d8d8d`.
The full anchor identifies source fixtures and expectations; pinned lockfiles
identify development dependencies, whose installed bytes were not separately
digested. Fresh test binaries were compiled by nextest for this run.

The two existing combined-cancellation settling unit cases named under
[error precedence](#error-and-signal-precedence) were also rerun through
`task test:session` with a filter for their exact names: both passed, with 75
other cases outside that filter. An initial `--lib` selection selected no cases
because these tests belong to the executable target; that empty run supplies
no evidence. The successful rerun used the same subject list, with identical
before/after digest-list identity
`5ed2dcc7b29bf30f2e2ee53c135ce3560c949d6393040cc5d7782a8f096e3655`.
That identity differs from the first run because the Taskfile now includes the
local presentation task; production source and expectations were unchanged.

These observations do not establish all late-signal windows, terminal recovery,
hostile-runtime resistance, confinement, installed layouts, platform floors,
unknown owner helpers or complete legacy compatibility. Their existing checks
remain part of any affected implementation handoff. The visual explanation's
source fidelity can be checked mechanically/manually; human usefulness requires
the later practical reader checkpoint.
