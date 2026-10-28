// An evaluation harness run and the later managed launch — UML sequence.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("caller", [caller]),
  part("cli", [Ravel CLI], kind: "ravel"),
  part("worker", [Rhai worker], kind: "ravel"),
  part("run", [claude harness run]),
  part("managed", [managed process]),
)
#let rows = (
  msg("caller", "cli", [start-session]),
  msg("cli", "worker", [Evaluate configuration script], create: true),
  msg("worker", "cli", [run_harness over private socket: claude, args and stdin]),
  msg("cli", "cli", [Find program. Check run budget]),
  msg("cli", "run", [Launch in a new POSIX session, \
    with its own result channel and RAVEL_POLICY_RUN], create: true),
  msg("run", "run", [Read stdin, write answer on stdout, exit]),
  msg("cli", "cli", [Complete run group cleanup]),
  destroy("run"),
  msg("cli", "worker", [Return status, exitCode and stdout as a map], dashed: true),
  msg("worker", "worker", [Script reads answer and selects]),
  msg("worker", "cli", [Selection], dashed: true),
  msg("cli", "cli", [Check selection and reap worker]),
  destroy("worker"),
  msg("cli", "caller", [Write text routing notice on standard error]),
  msg("cli", "managed", [Launch selected program], create: true),
)
#render(sequence([Harness run during evaluation — UML sequence], parts, rows, (
  [Blue: Ravel processes. Grey: external participants.],
  [An X ends the pictured lifetime after cleanup or reap.],
  [The run's answer goes to the configuration script. The managed process is a later launch.],
  [In the Claude example the run exits 0. It sends no end-session result.],
  [The later managed process sends the final result prime.],
  [With \-\-json, the notice follows the supervision attempt.],
), max-label: 130))
