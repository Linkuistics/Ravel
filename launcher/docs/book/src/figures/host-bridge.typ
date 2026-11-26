// One host call across the worker, CLI threads and runner.
#import "_sequence.typ": *
#import "_draw.typ": render
#let parts = (
  part("worker", [Rhai worker], kind: "ravel"),
  part("runtime", [CLI runtime \ thread], kind: "ravel"),
  part("reader", [CLI reader \ thread], kind: "ravel"),
  part("runner", [runner], kind: "ravel"),
  part("advisor", [fake-advisor]),
)
#let rows = (
  msg("worker", "runtime", [host frame: run_harness]),
  msg("runtime", "reader", [Dispatch call]),
  note("runtime", "reader", [Runtime continues to watch cancellation, deadline and worker.]),
  msg("reader", "reader", [Resolve advisor. Allocate its result channel.]),
  msg("reader", "runner", [Run harness]),
  msg("runner", "advisor", [Start in a new session], create: true),
  msg("advisor", "runner", [Output and result], dashed: true, role: "result"),
  msg("runner", "runner", [Stop group and reap]),
  destroy("advisor"),
  msg("runner", "reader", [Captured data], dashed: true),
  msg("reader", "reader", [Remove harness result channel]),
  msg("reader", "runtime", [Run map], dashed: true),
  msg("runtime", "worker", [reply frame], dashed: true),
  msg("worker", "worker", [Script receives stdout and result]),
)
#render(sequence([A harness call across the host bridge — UML sequence], parts, rows, (
  [Blue: Ravel. Grey: the harness. X: confirmed reap.],
  [The harness result channel is separate from the final session channel.],
), max-label: 120))
