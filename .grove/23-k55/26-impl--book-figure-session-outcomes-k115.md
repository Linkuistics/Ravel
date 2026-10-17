# book-figure-session-outcomes-k115

## Goal

Port the first UML state view, building the small state-machine module the
second state view will reuse, and report its layout cost.

## Context

- Figure: `session-outcomes`: 9 states (5 phase or outcome states in two
  semantic roles), 11 transitions with multi-line guard labels, two of them green,
  an initial marker, and a 10-line key. This is the densest unproven layout; two
  transitions fan out to four outcome states, and one phase has long guards
  between close nodes.
- New engine code: `_state.typ` (rounded state in a role, initial marker,
  transition with label and optional colour, label placement beside the edge).
  Decide at the start whether to hand-place coordinates or compute a simple
  layered layout from measured sizes. The sequence engine computed from text
  measurements and the C4 figures were hand-placed; pick on the evidence of which
  edges are hard.
- Preserve the "What each view means" row in `docs/book/figures.md`: cleanup is
  attempted on every path and an attempt need not succeed; the only green route
  needs a valid result, no cancellation, confirmed cleanup and a successful final
  write; the view groups operations rather than giving precedence.

## Done when

- `session-outcomes.puml` is replaced by a committed `.typ`; `_state.typ` exists.
- Layout cost recorded as numbers in the decision log, compared with the plan's
  cost table. If the cost is far above the estimate, say so and stop before the
  next state leaf rather than spending it; the owner decides.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
