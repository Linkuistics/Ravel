// Cancellation after a sent result — UML sequence. The only editable source of this
// figure; the layout comes from _sequence.typ and the drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("managed", [managed \ process]),
  part("channel", [result channel \ (temporary files)], kind: "temporary", db: true),
)

#let rows = (
  note("cli", "channel", [Selection and launch succeeded. Ravel confirmed worker cleanup. \ The managed process holds this session's RAVEL_EXIT_FILE.]),
  msg("managed", "channel", "Send \"candidate\" with end-session", role: "result"),
  note("managed", "channel", [Ravel keeps the first complete result. \ end-session success does not confirm delivery.]),
  div([Cancellation reaches the Ravel CLI]),
  msg("managed", "cli", [INT (the example below)]),
  msg("cli", "managed", [Attempt to stop the managed process's group]),
  msg("cli", "cli", [Attempt reap and terminal recovery if lent. \ Check whether the group is gone]),
  msg("cli", "channel", [Attempt result channel cleanup]),
  msg("cli", "cli", [Attempt other required cleanup]),
  note("cli", "channel", [Ravel reports a failed attempt. It does not treat it as confirmed cleanup. \ No X certifies process or directory removal on every path.]),
  msg("cli", "cli", [Keep cancellation primary. Return no result to the caller. \ Retain original failure and cleanup reports]),
  msg("cli", "caller", [Report cancellation and raise INT again \ (shell status 130). Return no result to the caller], dashed: true),
)

#render(sequence([Cancellation after a sent result — UML sequence], parts, rows, (
  [Blue: Ravel process. Grey: external participants. Gold cylinder: temporary files.],
  [Green carries a sent value, but no green arrow reaches the caller.],
  [The managed process's group includes descendants that stay in that group.],
  [INT goes directly to the Ravel CLI here. Terminal Ctrl-C can reach only the group.],
  [HUP and TERM use shell statuses 129 and 143. If signal propagation fails, Ravel exits with the numeric status.],
  [Cleanup problems cannot turn cancellation into success or permit result delivery.],
), max-label: 145))
