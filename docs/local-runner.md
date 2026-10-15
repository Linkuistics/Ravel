# The local job runner

`crates/keyed-launch` is Ravel's private local runner. It has no dependency on
another project's Rust library, installation, task tree or policy worker. It is
not published as a separate Rust package. Ravel composes it with its own
command front and compiled policy evaluator. The private [session transport](session-channel.md) already composes
this runner with a result-carrying completion file without changing its API.
The [component guide](components.md#why-there-are-two-rust-packages) explains
what the split hides and which composition obligations remain with Ravel.

## Boundary

The caller supplies `Argv`: a program and arguments, each string one whole word.
The runner spawns them directly. A shell participates only if the caller names
one. `Launch` carries the working directory, environment scrub/grants, optional
completion channel and escalation durations.

| Interface | Responsibility |
|---|---|
| `Channel` / `signal` | Allocate a fresh path and signal it by creating a file; allocation itself writes no completion file. |
| `run` | Launch a foreground job and return its reaped status, ending, completion-channel observation and process-group result. |
| `run_observed` | Also notify the parent synchronously at successful spawn and confirmed reap. |
| `run_noninteractive` | Isolate stdin, descriptors and the controlling terminal; send output to the caller's regular file. |
| `run_confined_observed` | Limit filesystem access to writable roots, system runtime reads and explicitly granted read-only files. |
| `EntrySignals` / `reraise` | Preserve the entry signal state for transparent wrappers and report cancellation as a real signal death. |

Completion-channel appearance begins grace → SIGTERM → kill-grace → SIGKILL.
The runner reaps the child and restores the terminal. An ending includes whether
the process group was confirmed gone; callers must check that result before
acting on completion. Selection, task interpretation, record storage and the
decision to launch more work belong above this boundary.

The implementation and detailed contracts live in
[`src/lib.rs`](../crates/keyed-launch/src/lib.rs) and
[`src/run.rs`](../crates/keyed-launch/src/run.rs).

## Native entry and return

The runner changes process-wide signal dispositions. It preserves ignores for
its cancellation signals, replaces other TERM/HUP handlers, and also catches
INT when no terminal exists or `Launch.transparent` is supplied. Those handlers
persist after runner-core success or failure. An ignored SIGCHLD is repaired
for watching the child and is not restored on return. Child `EntrySignals`
are caller-supplied values applied before exec; they neither capture entry
state nor restore the native host. Executable-entry transparency requires
capture before runtime changes, while call-entry semantics need a separately
declared capture instant.

A native consumer must own the signal/terminal lifetime, choose transparency
and terminal-INT policy, handle late cancellation between launches and establish
any required host restoration. A process-lifetime supervisor can accept
persistent changes explicitly; a returning host cannot infer restoration or
concurrent safety from a synchronous call. Observers must return promptly and
must not panic. Actual foreground ownership gates interactive terminal lending;
confined launches instead use a new detached session.

Successful `Ended` and failed supervision carry different knowledge. An error
can lack confirmed reap/group absence. `LaunchError::consumed_signal()` retains
the first cancellation consumed during supervision, including observation or
reap failure. It returns `None` for unrelated errors; `raw_os_error()` remains
the primary spawn errno. Message/Debug retain original and secondary diagnostic
text. The final wait attempt drains the signal latch before observation/recovery;
a signal arriving after that sample remains the caller's late observation.
A failed exec retains its lease and checks foreground/mode recovery before
returning the original spawn errno. The private attempted-group pipe records
ownership before handoff, closes across exec and never authorises signalling
that recorded identity after failed spawn. Handback requires exclusive foreground
authority and no unrelated group-ID reuse through recovery; CLI lifetime alone
does not supply those premises. Failure never licenses result success. The
[source inventory](result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
states these conditional paths and checks still owed, and the
[completion spec](specs/result-job-completion.md#acquisition-and-entry-limits)
separates them from the CLI composition. They were not runtime-tested in that
source observation and do not waive the existing job/session promises.

The agreed first native consumer is a one-session launcher CLI that settles
its launch and exits. The [repair contract](specs/result-job-completion.md#required-failure-settlement)
requires consumed cancellation to survive the runner's error, checked terminal
recovery after failed exec, and explicit failure diagnostics. Its
[entry contract](specs/result-job-completion.md#one-session-native-launcher-entry)
requires capture before runtime changes and waitability before every child,
including the policy worker. [C1–C4 evidence](completion-failure-design-evidence.md#c4-checked-observations)
now distinguishes real PTY handoff/recovery, physically confirmed waits and
private recovery-report substitutions. Native entry capture, waitability and
checked handler/mask transitions still require conformance work. A persistent runner has its own
continuing-host contract, and no host-restoration or concurrency guarantee
is established here.

## Tests and authority

The transferred suite exercises completion-channel isolation, stale channels,
argv/environment delivery, escalation, signal forwarding, descendant cleanup,
noninteractive isolation, terminal modes and foreground recovery, confinement,
and injected wait errors. Subprocess roles marked ignored are fixtures invoked
by the ordinary tests; they are not omitted acceptance cases. The wait-error
instrument remains test-only under `tests/internal/wait_events.rs`.
`task test:terminal-recovery` runs the private runner PTY matrix, then passes its
exact error fields into the front's real conversion/settlement. `task check`
includes that paired task as well as the full workspace suite. The fixtures
separately restore the terminal and confirm child waits after injected failures.

[Cargo's guard](../.cargo/config.toml) force-clears session launch, signal,
exit, run-ID and state-directory controls for both legacy and renamed tools.
The [test helper](../crates/keyed-launch/tests/support/mod.rs) independently
scrubs those controls before subprocess launches and provides the scrub list for
runner launches, including when a test binary is invoked directly. Generic
production runner code retains caller-supplied environment semantics.

## Source provenance

The complete runner implementation and original suite were copied from
`crates/keyed-launch` at source commit
`639b9d63570ed87f81f3ced3d2851b7961006bf3` of
[Linkuistics/grove](https://github.com/Linkuistics/grove/tree/639b9d63570ed87f81f3ced3d2851b7961006bf3/crates/keyed-launch).
This copy retains the process and terminal implementation, source project's
supported Rust floor, Apache-2.0 license and locked dependency versions.
Caller-specific prose is generalized. Test changes add independent authority
guards and their regression case; they do not redesign the runner.

The source project's launch behavior is documented by its
[job decision](https://github.com/Linkuistics/grove/blob/639b9d63570ed87f81f3ced3d2851b7961006bf3/docs/adr/the-launched-child-is-a-job.md).
The active [job decision](adr/the-launched-child-is-a-job.md),
[executable specification](specs/session-execution.md) and
[runtime evidence](runtime-evidence.md) accompany this implementation. Historical references here establish
provenance; they introduce no executable dependency.
