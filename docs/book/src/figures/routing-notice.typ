// The review policy's selection and routing notice — UML sequence.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI], kind: "ravel"),
  part("worker", [policy worker], kind: "ravel"),
  part("agent", [fake-agent]),
)
#let rows = (
  msg("caller", "cli", [start-session: kind review and prompt]),
  msg("cli", "worker", [Evaluate owner policy], create: true),
  msg("worker", "cli", [Selection: program fake-agent, args. \
    provider local, model fake, effort low], dashed: true),
  msg("cli", "cli", [Check selection and reap worker]),
  destroy("worker"),
  msg("cli", "caller", [Text routing notice on standard error: provider local, \
    model fake, effort low for kind \"review\"]),
  msg("cli", "agent", [Launch fake-agent with args], create: true),
  note("cli", "agent", [Labels go to the notice, not to the program.]),
)
#render(sequence([Selection and routing notice — UML sequence], parts, rows, (
  [Blue: Ravel processes. Grey: external participants.],
  [An X ends the worker lifetime after reap.],
  [With \-\-json, the JSON notice replaces the line after the supervision attempt.],
), max-label: 150))
