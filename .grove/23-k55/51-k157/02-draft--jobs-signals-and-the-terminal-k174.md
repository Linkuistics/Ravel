# jobs-signals-and-the-terminal-k174

## Goal

Complete part B of chapter 9 in
`docs/book/src/09-jobs-signals-and-the-terminal.md` under
`docs/book/contract.md`. Read the whole part A and preserve its section order.
Add the three reserved second-level sections, in this order:

1. “Lend and recover the terminal”.
2. “Preserve the entry signal state”.
3. “Know the guarantees and their limits”.

These sections deliver the terminal, native-entry and failure-limit scope in
the document node brief. Part A's job, escalation, cleanup, cancellation and
confinement sections are already present. No empty headings await completion.

## Context

- Read `jobs-signals-and-the-terminal-k158` for source evidence, obligation
  and outcome inventories, guide destinations, review scope and checks.
- Teach controlling terminal, foreground group and saved modes where needed.
  Explain ordinary recovery, delegated foreground after signal death, normal
  exit with another foreground holder, and failed exec. Preserve the exclusive
  foreground-authority and no-unrelated-ID-reuse conditions. A recorded group
  after failed spawn never grants authority to signal that number.
- Teach signal dispositions and masks across exec, the capture instant, the
  runner's persistent process-wide changes, and the CLI's checked restoration.
  Distinguish child entry values from restoration of a returning host. Explain
  what the runner does not restore. Started means spawn-reported success, not
  confirmed exec. Re-derive the facts from current source.
- State quoteable guarantees, each with its limit: same-user tampering, abrupt
  CLI death, descendants that leave the group, detached policy children,
  concurrent embedding, and unqualified native x86-64 Linux. Do not present a
  persistent runner or native x86-64 qualification as delivered functionality.
- Part A distinguishes ordinary unconfined CLI execution without a terminal
  from `run_noninteractive`, a separate detached runner API without a channel.
  Preserve that distinction. Its cancellation section links chapter 5's two
  sequences. Its normal cleanup table signals before reap and queries after
  reap. Only ESRCH confirms absence. EPERM retention needs actual reap,
  successful terminal recovery and group absence.
- Part A's complete fake sends `finished`, ignores TERM, and forces KILL.
  Fourteen complete book examples passed on Darwin arm64. Fifty-four selected
  runner cases passed, including PTY, noninteractive and Seatbelt cases. This
  is no Linux measurement, interactive CLI example or universal security proof.
- Part A's single review found no issue in its assigned pre-reap order scope.
  It did not review part B. Apply this leaf's own ordinary review allowance.

## Done when

- The whole chapter fulfils the node's scope. O8 is taught in full, O6 and O7
  agree across both halves, and the ten-obligation and lifecycle inventories
  are re-derived with their destinations and outstanding owners recorded.
- Every signal/terminal/cleanup claim is checked against the subject and the
  relevant tests. Each guide statement replaced by part B has a destination.
- The terminal sequence's participants, branches, modes and conditions are in
  the node brief's art handoff or this leaf's log. Art owns its drawing.
- Complete examples pass `task docs:examples`; terminal-only fragments state
  what they omit. Book build, strict STE and strict prose-break checks pass.
- Inspect later siblings before any cut. Copy-edit already follows this leaf,
  so do not add another copy-edit. Retire and seal this leaf when complete.
