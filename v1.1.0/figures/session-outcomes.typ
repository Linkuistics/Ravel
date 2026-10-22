// Session phases and outcomes — UML state machine. The only editable source of
// this figure: the states in grid cells, the transitions and the key. Nothing here
// has a coordinate: _state.typ measures the text and places the cells, and the
// drawing is _draw.typ's.
#import "_state.typ": *
#import "_draw.typ": render

#let states = (
  initial("start", col: 2, row: 0),
  state("selecting", "phase", [Select], col: 2, row: 1),
  state("supervising", "phase", [Supervise], col: 2, row: 2),
  state("settling", "phase", [Do cleanup and \ decide the outcome], col: 2, row: 3, min-w: 250),
  state("cancelled", "outcome", [Cancelled], col: 1, row: 4),
  state("refused", "outcome", [Refused], col: 2, row: 4),
  state("missing", "outcome", [No result], col: 3, row: 4),
  state("writing", "phase", [Write the result], col: 0, row: 5),
  state("failed", "outcome", [Failed], col: 4, row: 5),
  state("succeeded", "outcome", [Succeeded], col: 0, row: 6),
)

#let trans = (
  tr("start", "selecting"),
  tr("selecting", "supervising", label: [Ravel confirms worker cleanup, \ checks the command and launches it], side: "left"),
  tr("selecting", "settling", label: [refusal, timeout, failure \ or cancellation before launch],
    out: "right", into: ("top", 80)),
  tr("supervising", "settling", label: [sent result and job termination, \ natural exit, failure or cancellation], side: "left"),
  tr("settling", "cancelled", label: [[cancellation \ selected]], out: ("bottom", "align"), side: "left"),
  tr("settling", "refused", label: [[refusal \ is primary]], out: ("bottom", "align")),
  tr("settling", "missing", label: [[missing result \ is primary]], out: ("bottom", "align")),
  tr("settling", "writing", role: "result", out: "left", into: "top", seg: 1, side: "right", t: 0.75,
    label: [[valid result and no cancellation. \ Ravel confirms reap, group absence and cleanup]]),
  tr("settling", "failed", label: [[other failure \ is primary]], out: "right", into: "top"),
  tr("writing", "failed", label: [write or flush fails]),
  tr("writing", "succeeded", role: "result", label: [write and flush succeed]),
)

#render(state-machine([Session phases and outcomes — UML state machine], states, trans, (
  [Blue: phases of Ravel's work, not processes.],
  [Grey rounded boxes: outcomes. The view shows no further transition.],
  [INT, TERM or HUP can cancel either phase. Ravel returns no result to the caller.],
  [Ravel attempts cleanup on every path. An attempt does not imply success.],
  [Ravel keeps one primary outcome and reports other failures beside it.],
  [No result means that no program sent a result, even if the managed process exited 0.],
  [An empty result is a value. A later nonzero managed exit does not invalidate it.],
  [Green: the only route to successful result delivery.],
  [A failed write may leave a prefix on stdout.],
  [This view groups operations. It is not the full failure-precedence algorithm.],
), col-gap: 30, row-gap: (56, 56, 56, 56, 100, 56)))
