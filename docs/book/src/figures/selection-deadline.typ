// Selection deadline and the later signal variant, one authoritative sequence.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("cli", [Ravel CLI \ latch], kind: "ravel"),
  part("runner", [runner], kind: "ravel"),
  part("run", [fake-advisor \ and its group]),
  part("worker", [Rhai worker], kind: "ravel"),
)
#let rows = (
  msg("cli", "cli", [Deadline expires after 1 s. Host::settle sets StopRequest.]),
  msg("cli", "runner", [Stop harness run]),
  msg("runner", "run", [TERM to group and run]),
  msg("run", "run", [Trap writes note]),
  msg("run", "cli", [Signal variant: trap sends TERM. Latch keeps it.]),
  msg("runner", "run", [After 5 s: KILL to group and run]),
  msg("runner", "run", [Two group KILL attempts, reap and group query]),
  msg("cli", "worker", [TERM, then KILL after 1 s if needed, then reap]),
  msg("cli", "cli", [Remove private directories. Read latch again.]),
  note("cli", "worker", [No signal: selection_timeout, 124. With TERM: selection_cancelled, 143.]),
)
#render(sequence([Deadline during a harness run — UML sequence], parts, rows, (
  [Blue: Ravel. Grey: harness run and group. Cleanup steps are attempts.],
  [Signal arrows are attempts. The second example adds the TERM to the CLI.],
  [No final managed process starts.],
), max-label: 140))
