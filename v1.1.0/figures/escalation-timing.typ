// Result-triggered escalation and normal cleanup. This is the only editable
// source. The existing timing module lays out three distinct panels.
#import "_timing.typ": *
#import "_draw.typ": render

#let panel-a = panel(
  [A. The child outlasts both grace periods],
  (
    instant("sent", label: [Result \ sent]),
    instant("seen", label: [File \ observed]),
    instant("term", label: [TERM \ attempt]),
    instant("kill", label: [KILL \ attempt]),
    instant("exit", label: [Exit \ observed]),
  ),
  (
    lifeline("process", [Managed \ child], "external", (
      state([Runs], to: "term"),
      state([Ignores \ TERM], to: "kill"),
      state([Exit not yet \ observed], to: "exit"),
      state([Exited, \ unreaped], role: "note"),
    )),
    lifeline("runner", [Runner \ inside CLI], "ravel", (
      state([Waits for \ the file], to: "seen"),
      state([Grace], to: "term"),
      state([Kill grace], to: "kill"),
      state([Checks \ for exit], to: "exit"),
      state([Go to C]),
    )),
    lifeline("group", [Other group \ members], "external", (
      state([Can run], to: "term"),
      state([Can run], to: "kill"),
      state([Absence not yet confirmed]),
    )),
  ),
  msgs: (
    msg("runner", "process", "term", [TERM]),
    msg("runner", "group", "term", [TERM]),
    msg("runner", "process", "kill", [KILL]),
    msg("runner", "group", "kill", [KILL]),
  ),
  bounds: (
    bound("seen", "term", [2 seconds]),
    bound("term", "kill", [5 seconds]),
  ),
)

#let panel-b = panel(
  [B. The child exits during the first grace period],
  (
    instant("seen", label: [File \ observed]),
    instant("exit", label: [Child \ exits]),
    instant("noticed", label: [Exit \ observed]),
  ),
  (
    lifeline("process", [Managed \ child], "external", (
      state([Runs], to: "exit"),
      state([Exited, unreaped], role: "note"),
    )),
    lifeline("runner", [Runner \ inside CLI], "ravel", (
      state([Waits for the file], to: "seen"),
      state([Checks for exit], to: "noticed"),
      state([Go to C. No TERM or escalation KILL]),
    )),
    lifeline("group", [Other group \ members], "external", (
      state([Can still run. No new grace period]),
    )),
  ),
  bounds: (bound("seen", "exit", [Less than 2 seconds]),),
)

#let panel-c = panel(
  [C. Normal cleanup after either exit observation],
  (
    instant("first", label: [Group KILL \ attempt 1]),
    instant("second", label: [Group KILL \ attempt 2]),
    instant("reap", label: [Child \ reaped]),
    instant("recovery", label: [Terminal recovery \ completed, if lent]),
    instant("query", label: [Group query \ completes]),
  ),
  (
    lifeline("process", [Managed \ child], "external", (
      state([Exited, unreaped. Identity reserved], to: "reap", role: "note"),
    ), destroy: true),
    lifeline("runner", [Runner \ inside CLI], "ravel", (
      state([Final group stops], to: "second"),
      state([Reap], to: "reap"),
      state([Recover \ terminal], to: "recovery"),
      state([Query group \ with signal 0], to: "query"),
      state([Check \ outcome]),
    )),
    lifeline("group", [Other group \ members], "external", (
      state([Absence not yet confirmed], to: "query"),
      state([Only ESRCH \ confirms absence], role: "note"),
    )),
  ),
  msgs: (
    msg("runner", "group", "first", [KILL]),
    msg("runner", "group", "second", [KILL]),
  ),
  bounds: (
    bound("first", "second", [20 milliseconds]),
    bound("recovery", "query", [Up to 1 second]),
  ),
)

#render(timing([After a sent result — UML timing], (panel-a, panel-b, panel-c), (
  [Time runs left to right. Distances are not to scale. A and B are alternatives. Both continue at C.],
  [Blue: runner inside the Ravel CLI. Grey: child and group. Pale child state: exited but unreaped. X: confirmed reap.],
  [A signal arrow is an attempt, not proof of exit. The final group query sends no signal.],
  [Poll intervals: 500 milliseconds before escalation KILL, then 10 milliseconds. The scheduler can add delay.],
  [The first escalation KILL starts one 2-second retry bound for exit checks and reap attempts. Errors do not restart it.],
  [An expired bound or unconfirmed cleanup returns no result. C shows the normal path, not every failure path.],
  [Early exit during either grace skips later escalation. Descendants receive no new grace.],
  [After reap, Ravel sends no signal to that group ID. Query success or EPERM does not confirm absence.],
  [The CLI supplies the grace values. They are not a session deadline or a wall-clock guarantee.],
)))
