// Cancellation after publication — UML sequence. The only editable source of this
// figure; the layout comes from _sequence.typ and the drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("managed", [managed process]),
  part("channel", [result channel \ (temporary files)], kind: "temporary", db: true),
)

#let rows = (
  note("cli", "channel", [Selection and launch have succeeded. Worker cleanup was confirmed. \ The managed process holds this session's RAVEL_EXIT_FILE.]),
  msg("managed", "channel", "Publish \"candidate\" via end-session", role: "result"),
  note("managed", "channel", [First complete publication wins. \ end-session success does not confirm delivery.]),
  div([Cancellation reaches the Ravel CLI]),
  msg("managed", "cli", [SIGINT (the example below)]),
  msg("cli", "managed", [Attempt to stop the managed process's group]),
  msg("cli", "cli", [Attempt reap and terminal recovery if lent; \ check whether the group is gone]),
  msg("cli", "channel", [Attempt result channel cleanup]),
  msg("cli", "cli", [Attempt remaining required cleanup]),
  note("cli", "channel", [A failed attempt is reported, not treated as confirmed cleanup. \ No X certifies process or directory removal on every path.]),
  msg("cli", "cli", [Keep cancellation primary; suppress candidate; \ retain original failure and cleanup reports]),
  msg("cli", "caller", [Report cancellation; re-raise INT \ (shell status 130); no result bytes], dashed: true),
)

#render(sequence([Cancellation after publication — UML sequence], parts, rows, (
  [Blue: Ravel process. Grey: external participants. Gold cylinder: temporary files.],
  [Green carries a published value, but no green arrow reaches the caller.],
  [The managed process's group includes descendants that stay in that group.],
  [INT goes directly to the Ravel CLI here; terminal Ctrl-C can reach only the group.],
  [HUP and TERM use shell statuses 129 and 143; failed propagation uses numeric fallback.],
  [Cleanup problems cannot turn cancellation into success or permit result delivery.],
), max-label: 340))
