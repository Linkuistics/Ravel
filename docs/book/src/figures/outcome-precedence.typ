// How the outcome is chosen when failures and signals combine — UML activity.
// The only editable source of this figure: the flow from the end of supervision
// to the exit status, top to bottom, each decision with its exit beside it, and
// the key. Nothing here has a coordinate: _activity.typ measures the text and
// places the nodes, and the drawing is _draw.typ's.
#import "_activity.typ": *
#import "_draw.typ": render

#let nodes = (
  initial("start"),
  action("cleanup", [Attempt every cleanup step, \ whatever happened before]),
  action("signals", [Take the first signal found, in this order: \ Ravel CLI latch, runner outcome, \ late runner latch, queued signal \ at the last check], limit: 320),
  action("report", [Report every cleanup error \ and a process group that survives]),
  decision("signal", [Signal found?],
    right: branch([[signal found]],
      outcome("cancelled", [Cancelled \ 129: HUP \ 130: INT \ 143: TERM], note: [Ravel returns no result. \ It reports the original \ failure, if there is one.])),
    down: branch([[no signal]])),
  decision("failure", [Original failure?],
    right: branch([[original failure]],
      outcome("original", [Original failure \ keeps its own exit status])),
    down: branch([[no failure]])),
  decision("cleanup-error", [Cleanup step failed?],
    right: branch([[cleanup error]],
      outcome("cleanup-failed", [5: first cleanup error])),
    down: branch([[no cleanup error]])),
  decision("group", [Process group survived?],
    right: branch([[group survived]],
      outcome("group-survived", [5: process group survived])),
    down: branch([[group absent]])),
  action("write", [Write the result and flush]),
  decision("written", [Write and flush succeed?],
    right: branch([[write or flush fails]],
      outcome("write-failed", [5: result write failed])),
    down: branch([[succeed]],
      outcome("succeeded", [0: Ravel delivers the exact result]))),
)

#render(activity([How the outcome is chosen — UML activity], nodes, (
  [Blue: steps. Grey: outcomes. A diamond is a decision.],
  [The label in square brackets is the guard of a flow.],
  [Ravel attempts all cleanup steps before it chooses an outcome.],
  [The decisions run in precedence order: the first that applies gives the outcome.],
  [Cancellation wins over every failure. An original failure keeps its own status.],
  [The write happens only when every earlier decision passes.],
)))
