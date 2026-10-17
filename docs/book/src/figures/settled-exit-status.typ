// From the final outcome to exit status — UML state machine. The only editable
// source of this figure: the steps in one column, an outcome beside each, the
// transitions and the key. Nothing here has a coordinate: _state.typ measures the
// text and places the cells, and the drawing is _draw.typ's.
#import "_state.typ": *
#import "_draw.typ": render

#let states = (
  initial("start", col: 0, row: 0),
  state("cancellation", "phase", [Check cancellation], col: 0, row: 1),
  state("original", "phase", [Check the original outcome], col: 0, row: 2),
  state("cleanup", "phase", [Check required cleanup], col: 0, row: 3),
  state("writing", "phase", [Writing the result], col: 0, row: 4),
  state("succeeded", "outcome", [0: exact result delivered], col: 0, row: 5),
  state("cancelled", "outcome", [Cancelled \ 129: HUP \ 130: INT \ 143: TERM], col: 1, row: 1),
  state("failure", "outcome", [Original failure retained \ 1: no result \ 2: invalid inputs or settings \
    3: refusal by policy, or of its \ file, behaviour or data \
    5: worker, supervision or result failure \ 124: selection deadline \
    126: pre-start execution failure \ 127: program or interpreter not found], col: 1, row: 2, limit: 320),
  state("cleanup-failed", "outcome", [5: cleanup, recovery or group failure], col: 1, row: 3, limit: 320),
  state("write-failed", "outcome", [5: result write or flush failed], col: 1, row: 4, limit: 320),
)

#let trans = (
  tr("start", "cancellation"),
  tr("cancellation", "cancelled", label: [[cancellation selected]], out: ("right", "align")),
  tr("cancellation", "original", label: [[no cancellation]]),
  tr("original", "failure", label: [[original failure]], out: ("right", "align")),
  tr("original", "cleanup", label: [[valid candidate result]]),
  tr("cleanup", "cleanup-failed", label: [[required cleanup failed \ or group survives]]),
  tr("cleanup", "writing", role: "result", label: [[reap, group absence and \ required cleanup confirmed]]),
  tr("writing", "write-failed", label: [write or flush fails]),
  tr("writing", "succeeded", role: "result", label: [write and flush succeed]),
)

#render(state-machine([From the final outcome to exit status — UML state machine], states, trans, (
  [Refinement of chapter 3's final-outcome step for start-session.],
  [Blue: steps in deciding the outcome. Grey: outcomes.],
  [Applicable cleanup is attempted before reporting an outcome.],
  [This order expresses precedence, not the implementation's call order.],
  [Original failures keep their status when later cleanup also fails.],
  [Cancellation suppresses a published result and retains other diagnostics.],
  [Green: the only success route. Empty publication is a valid value.],
  [A later nonzero child exit does not invalidate that value.],
  [A failed final write can leave a prefix; a late signal can interrupt it.],
  [end-session, help and version invocations are covered by the table, not this view.],
), col-gap: 190, row-gap: (56, 56, 56, 72, 56)))
