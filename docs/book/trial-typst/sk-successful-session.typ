// A successful session — UML sequence. The ONLY editable source of this
// trial figure. `--input variant=long` adds a long label and an extra
// participant (the scalability probe); no other file differs.
#import "/src/figures/_sequence.typ": *
#show: figure-page

#let variant = sys.inputs.at("variant", default: "base")
#let long = variant == "long"
#let G = pal.green

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("worker", [policy worker], kind: "ravel"),
  part("managed", [managed process]),
  part("channel", [result channel \ (temporary files)], kind: "temporary", db: true),
) + if long { (part("audit", [audit sink \ (owner-supplied)]),) } else { () }

#let rows = (
  msg("caller", "cli", [start-session: kind and prompt]),
  msg("cli", "cli", [Check inputs and policy path]),
  div([Selection]),
  msg("cli", "worker", [Start in empty private directory], create: true),
  msg("worker", "worker", [Change to /]),
  msg("worker", "cli", [Identity; Ravel CLI checks the match], dashed: true),
  msg("cli", "worker", [Send policy request (no session-result context)]),
  msg("worker", "worker", [Import owner policy; select]),
  msg("worker", "cli", [Policy response on private socket], dashed: true),
  msg("cli", "cli", [Check response; confirm worker reap; \ remove worker's private directory]),
  destroy("worker"),
  msg("cli", "cli", [Resolve selected executable]),
  div([Launch and publication]),
  msg("cli", "channel", [Allocate fresh result channel], create: true),
  msg("cli", "managed", if long {
    [Launch in its own process group, with the session-result context and the narrowed environment the owner policy granted for this kind of session]
  } else {
    [Launch in its own process group; pass session-result context]
  }, create: true),
  msg("managed", "channel", [Publish opaque result via end-session], color: G),
  note("managed", "channel", [First complete publication wins. \ Publication does not wait for cleanup.]),
  div([Cleanup and delivery]),
  msg("cli", "managed", [Stop the managed process's group \ (grace and escalation if needed)]),
  msg("cli", "cli", [Confirm direct-child reap; recover terminal \ if lent; confirm process-group absence]),
  destroy("managed"),
  msg("cli", "channel", [Validate and read published result]),
  msg("channel", "cli", [Exact opaque value], color: G, dashed: true),
  msg("cli", "channel", [Remove result channel's temporary directory]),
  destroy("channel"),
) + if long { (msg("cli", "audit", [Record the final outcome]),) } else { () } + (
  msg("cli", "cli", [Decide the final outcome, including \ cancellation and cleanup]),
  msg("cli", "caller", [Write and flush opaque result; exit 0], color: G),
)

#sequence([A successful session — UML sequence], parts, rows, (
  [An X ends the pictured lifetime after reap or removal, not at process exit.],
  [Blue: Ravel processes. Grey: external participants. Gold cylinder: temporary files.],
  [Green arrows carry the opaque result.],
  [Successful unconfined path only; terminal recovery applies only if lent.],
  [Publication precedes cleanup; only the last arrow delivers the result to the caller.],
))
