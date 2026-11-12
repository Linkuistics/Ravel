// One review from the caller's request to its next action — UML sequence.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [orchestrator]),
  part("ravel", [Ravel CLI], kind: "ravel"),
  part("harness", [Claude Code]),
)
#let rows = (
  msg("caller", "ravel", [start-session: review and prompt]),
  note("caller", "ravel", [The prompt permits approved or changes-needed.]),
  msg("ravel", "ravel", [Configuration selects Claude Code]),
  msg("ravel", "harness", [Launch with prompt], create: true),
  msg("harness", "harness", [Review files and write review-notes.md]),
  msg("harness", "ravel", [end-session changes-needed], role: "result"),
  msg("ravel", "ravel", [Stop group and complete cleanup]),
  msg("ravel", "caller", [Status 0 and changes-needed], role: "result", dashed: true),
  msg("caller", "caller", [Read notes and request fix]),
)
#render(sequence([One code review — UML sequence], parts, rows, (
  [Blue: Ravel. Grey: caller and harness. Green: the sent and returned result.],
  [The prompt also permits approved. The caller chooses what either result means.],
  [Ravel returns a result only after successful cleanup.],
), max-label: 145))
