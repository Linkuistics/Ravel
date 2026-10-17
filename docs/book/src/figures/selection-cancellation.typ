// Cancellation during selection — UML sequence. The only editable source of this
// figure; the layout comes from _sequence.typ and the drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI], kind: "ravel"),
  part("worker", [policy worker], kind: "ravel"),
)

#let rows = (
  msg("caller", "cli", [start-session]),
  msg("cli", "worker", [Start worker; check identity; send request], create: true),
  note("cli", "worker", [The worker evaluates trusted owner policy. \ It has no session-result context.]),
  div([Cancellation reaches the Ravel CLI]),
  msg("worker", "cli", [SIGINT (the example below)]),
  msg("cli", "worker", [Close protocol; attempt bounded stop and wait]),
  msg("cli", "cli", [Drain bounded diagnostics; attempt \ worker private-directory cleanup]),
  note("cli", "worker", [Reap and removal must be confirmed before claiming them. \ Failed attempts remain diagnostics; no X certifies teardown here. \ No managed process is launched. \ No result channel is allocated.]),
  msg("cli", "cli", [Keep cancellation primary; retain any \ original failure and teardown problems]),
  msg("cli", "caller", [Report cancellation; re-raise INT \ (shell status 130); no result bytes], dashed: true),
)

#render(sequence([Cancellation during selection — UML sequence], parts, rows, (
  [Blue: Ravel processes. Grey: caller.],
  [This example sends INT directly to the Ravel CLI; it is not keyboard delivery.],
  [HUP and TERM give shell statuses 129 and 143 by the same cancellation rule.],
  [A failed signal propagation uses the corresponding numeric exit as fallback.],
  [Cancellation reports attempted teardown, not a guarantee that cleanup succeeded.],
), max-label: 300))
