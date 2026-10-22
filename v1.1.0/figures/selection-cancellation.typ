// Cancellation during selection — UML sequence. The only editable source of this
// figure; the layout comes from _sequence.typ and the drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI], kind: "ravel"),
  part("worker", [policy \ worker], kind: "ravel"),
)

#let rows = (
  msg("caller", "cli", [start-session]),
  msg("cli", "worker", [Start worker. Check identity. Send request], create: true),
  note("cli", "worker", [The worker evaluates trusted owner policy. \ It has no session-result context.]),
  div([Cancellation reaches the Ravel CLI]),
  msg("worker", "cli", [INT (the example below)]),
  msg("cli", "worker", [Close protocol. Attempt bounded stop and wait]),
  msg("cli", "cli", [Drain bounded diagnostics. Attempt \ worker private-directory cleanup]),
  note("cli", "worker", [Ravel confirms reap and removal before it reports them as complete. \ Failed attempts stay as diagnostics. No X certifies teardown here. \ Ravel launches no managed process and allocates no result channel.]),
  msg("cli", "cli", [Keep cancellation primary. Retain any \ original failure and teardown problems]),
  msg("cli", "caller", [Report cancellation and raise INT again \ (shell status 130). Return no result to the caller], dashed: true),
)

#render(sequence([Cancellation during selection — UML sequence], parts, rows, (
  [Blue: Ravel processes. Grey: caller.],
  [This example sends INT directly to the Ravel CLI. It is not keyboard delivery.],
  [HUP and TERM give shell statuses 129 and 143 by the same cancellation rule.],
  [If signal propagation fails, Ravel exits with the numeric status for that signal.],
  [Cancellation reports attempted teardown, not a guarantee that cleanup succeeded.],
), max-label: 145))
