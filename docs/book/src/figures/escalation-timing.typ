// What happens after end-session sends the result — UML timing. The only editable
// source of this figure: two panels (the process still runs after the grace
// period, and the process exits inside it), each with its instants, its
// lifelines, its signals and its duration constraints, and the key. Nothing here
// has a coordinate: _timing.typ measures the text and spaces the instants, and
// the drawing is _draw.typ's.
#import "_timing.typ": *
#import "_draw.typ": render

#let panel-a = panel(
  [A. The process still runs after the grace period],
  (
    instant("result", label: [end-session sends \ the result]),
    instant("seen", label: [Runner sees \ the result]),
    instant("term", label: [Runner \ sends TERM]),
    instant("kill", label: [Runner \ sends KILL]),
    instant("reap", label: [Runner reaps \ the process]),
  ),
  (
    lifeline("process", [Managed \ process], "external", (
      state([Runs], to: "term"),
      state([Runs. \ Ignores TERM], to: "kill"),
      state([Ended, not \ reaped], to: "reap", role: "note"),
    ), destroy: true),
    lifeline("runner", [Runner], "ravel", (
      state([Waits for \ the result], to: "seen"),
      state([Grace period: \ waits for the exit], to: "term"),
      state([Second grace period: \ waits for the exit], to: "kill"),
      state([Sees the exit. Stops \ the rest of the group], to: "reap"),
      state([Confirms that \ the group is gone]),
    )),
    lifeline("group", [Rest of the \ process group], "external", (
      state([Runs], to: "term"),
      state([Runs. \ Ignores TERM], to: "kill"),
      state([Ended], role: "note"),
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
    bound("term", "kill", [up to 5 seconds]),
  ),
)

#let panel-b = panel(
  [B. The process exits inside the grace period],
  (
    instant("result", label: [end-session sends \ the result]),
    instant("seen", label: [Runner sees \ the result]),
    instant("exit", label: [Process \ exits]),
    instant("noticed", label: [Runner sees the exit. \ Sends KILL to the group]),
    instant("reap", label: [Runner reaps \ the process]),
  ),
  (
    lifeline("process", [Managed \ process], "external", (
      state([Runs], to: "exit"),
      state([Ended, not \ reaped], to: "reap", role: "note"),
    ), destroy: true),
    lifeline("runner", [Runner], "ravel", (
      state([Waits for \ the result], to: "seen"),
      state([Grace period: \ waits for the exit], to: "noticed"),
      state([Stops the rest \ of the group], to: "reap"),
      state([Confirms that \ the group is gone]),
    )),
    lifeline("group", [Rest of the \ process group], "external", (
      state([Runs], to: "noticed"),
      state([Ended], role: "note"),
    )),
  ),
  msgs: (
    msg("runner", "group", "noticed", [KILL]),
  ),
  bounds: (
    bound("seen", "noticed", [less than 2 seconds]),
  ),
)

#render(timing([After end-session sends the result — UML timing], (panel-a, panel-b), (
  [Time goes from left to right. The distances are not to scale.],
  [Blue: the runner. Grey: the managed process and the rest of its process group.],
  [Pale: that part has ended. An X marks the reap of the process.],
  [Each signal goes to the process and to the rest of its group.],
  [A wait ends when the process exits. Then the runner sends no signal to the process.],
  [The runner looks twice each second. A step can start up to 0.5 second late.],
  [The runner stops the rest of the process group before it reaps the process.],
)))
