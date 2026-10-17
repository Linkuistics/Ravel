# book-figure-settled-exit-status-k116

## Goal

Port the second state view on `_state.typ` and report its layout cost.

## Context

- Figure: `settled-exit-status`: 9 states, 8 transitions, two green, with very
  long outcome labels (the failure box lists eight exit codes over eight lines)
  and a 10-line key. The long boxes drive the layout, which flows left-to-right
  with outcomes to the right. The filename stays for stable links.
- Preserve the "What each view means" row: outcome precedence for
  `start-session`, not call order; cancellation, original failure, required
  cleanup, final write; cleanup is attempted before reporting; an original
  failure keeps its status if later cleanup fails.
- Every number in the exit-code boxes is checked against
  `crates/ravel/src` or chapter 6's status table, not copied blind.

## Done when

- `settled-exit-status.puml` is replaced by a committed `.typ`.
- Layout cost recorded as numbers, compared with the plan's cost table.
- `task docs:figures`, `task docs:book:build` and `task check` pass.
