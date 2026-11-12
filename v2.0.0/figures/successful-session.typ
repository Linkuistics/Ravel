// A successful session — UML sequence. The only editable source of this figure:
// participants, rows and the key. The layout comes from _sequence.typ and the
// drawing from _draw.typ.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("worker", [Rhai \ worker], kind: "ravel"),
  part("managed", [managed \ process]),
  part("channel", [result channel \ (temporary files)], kind: "temporary", db: true),
)

#let rows = (
  msg("caller", "cli", [start-session: kind and prompt]),
  msg("cli", "cli", [Check inputs and configuration script path]),
  div([Selection]),
  msg("cli", "worker", [Start in empty private directory], create: true),
  msg("worker", "cli", [Identity. Ravel CLI checks the match], dashed: true),
  msg("cli", "worker", [Send evaluation request (no session-result context)]),
  msg("worker", "worker", [Import configuration script and select]),
  msg("worker", "cli", [Script response on private socket], dashed: true),
  msg("cli", "cli", [Check response and confirm worker reap. \ Remove the private directory of the worker]),
  destroy("worker"),
  msg("cli", "cli", [Resolve selected executable]),
  div([Launch and result]),
  msg("cli", "channel", [Allocate fresh result channel], create: true),
  msg("cli", "managed", [Launch in its own process group. \ Pass session-result context], create: true),
  msg("managed", "channel", [Send opaque result with end-session], role: "result"),
  note("managed", "channel", [Ravel keeps the first complete result. \ end-session does not wait for cleanup.]),
  div([Cleanup and delivery]),
  msg("cli", "managed", [Stop the managed process's group \ (grace and escalation if needed)]),
  msg("cli", "cli", [Confirm direct-child reap. Recover terminal \ if lent. Confirm process-group absence]),
  destroy("managed"),
  msg("cli", "channel", [Validate and read sent result]),
  msg("channel", "cli", [Unchanged opaque value], role: "result", dashed: true),
  msg("cli", "channel", [Remove result channel's temporary directory]),
  destroy("channel"),
  msg("cli", "cli", [Decide the final outcome from the result, \ cancellation and cleanup]),
  msg("cli", "caller", [Write and flush opaque result. Exit 0], role: "result"),
)

#render(sequence([A successful session — UML sequence], parts, rows, (
  [An X ends the pictured lifetime after reap or removal, not at process exit.],
  [Blue: Ravel processes. Grey: external participants. Gold cylinder: temporary files.],
  [Green arrows carry the opaque result.],
  [Successful path only. Recover the terminal only if Ravel lent it.],
  [This chapter's configuration script makes no host call or evaluation harness run.],
  [The sent result precedes cleanup. Only the last arrow delivers the result to the caller.],
), max-label: 125))
