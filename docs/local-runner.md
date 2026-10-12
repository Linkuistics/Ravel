# The local job runner

`crates/keyed-launch` is Ravel's private local runner. It has no dependency on
another project's Rust library, installation, task tree or policy worker. It is
not published as a separate Rust package. Ravel will compose it with its own
command front and policy evaluator; those components are not present in this
increment. The private [session transport](session-channel.md) already composes
this runner with a result-carrying completion file without changing its API.

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

## Tests and authority

The transferred suite exercises completion-channel isolation, stale channels,
argv/environment delivery, escalation, signal forwarding, descendant cleanup,
noninteractive isolation, terminal modes and foreground recovery, confinement,
and injected wait errors. Subprocess roles marked ignored are fixtures invoked
by the ordinary tests; they are not omitted acceptance cases. The wait-error
instrument remains test-only under `tests/internal/wait_events.rs`.

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
Generic executable-owned decisions, the specification and runtime evidence will
accompany Ravel's front/worker increment. Historical references here establish
provenance; they introduce no executable dependency.
