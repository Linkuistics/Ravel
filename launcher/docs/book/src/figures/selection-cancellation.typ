// Cancellation during selection — UML sequence. The only editable source of this
// figure; the layout comes from _sequence.typ and the drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("worker", [Rhai \ worker], kind: "ravel"),
  part("harness", [evaluation \ harness]),
)

#let rows = (
  msg("caller", "cli", [start-session]),
  msg("cli", "worker", [Start worker. Check identity. Send request], create: true),
  note("cli", "worker", [The worker evaluates trusted configuration script. \ It has no session-result context.]),
  msg("worker", "cli", [Host call: run_harness]),
  msg("cli", "harness", [Start harness run with its own result channel], create: true),
  note("worker", "harness", [The script waits for data. The harness has no terminal. \ Its result channel cannot complete the outer session.]),
  div([Cancellation reaches the Ravel CLI]),
  msg("harness", "cli", [INT (the fake harness sends it to its parent)]),
  msg("cli", "harness", [Attempt to stop the harness run's group]),
  msg("cli", "cli", [Attempt harness reap, group confirmation \ and run-directory cleanup within the stop bound]),
  msg("cli", "worker", [Close protocol. Attempt bounded stop and wait]),
  msg("cli", "cli", [Drain bounded diagnostics. Attempt \ worker private-directory cleanup]),
  note("cli", "harness", [Ravel confirms reap and removal before it reports them as complete. \ Failed attempts stay as diagnostics. No X certifies teardown here. \ Ravel launches no final managed process and allocates no final-session channel.]),
  msg("cli", "cli", [Keep cancellation primary. Retain any \ original failure and teardown problems]),
  msg("cli", "caller", [Report cancellation and raise INT again \ (shell status 130). Return no result to the caller], dashed: true),
)

#render(sequence([Cancellation during selection — UML sequence], parts, rows, (
  [Blue: Ravel processes. Grey: caller and evaluation harness.],
  [Harness cleanup precedes worker teardown. The worker itself sends no signal.],
  [This example sends INT directly to the Ravel CLI. It is not keyboard delivery.],
  [HUP and TERM give shell statuses 129 and 143 by the same cancellation rule.],
  [If signal propagation fails, Ravel exits with the numeric status for that signal.],
  [Cancellation reports attempted teardown, not a guarantee that cleanup succeeded.],
), max-label: 145))
