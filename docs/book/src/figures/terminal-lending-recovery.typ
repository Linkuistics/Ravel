// Terminal lease through spawn and recovery, using the existing sequence module.
// Dividers identify alternatives explicitly; no new layout or drawing primitive.
#import "_sequence.typ": *
#import "_draw.typ": render

#let parts = (
  part("cli", [Ravel CLI \ (runner inside)], kind: "ravel"),
  part("child", [child / \ managed group]),
  part("tty", [controlling \ terminal]),
)

#let rows = (
  note("cli", "tty", [Ordinary unconfined launch. /dev/tty is available. The CLI group holds foreground. \ Without a terminal, or in detached mode, there is no lease.]),
  note("cli", "tty", [An early setup failure enters B before the later setup steps complete. \ It need not reach the group record, terminal handoff or exec attempt.]),
  msg("cli", "tty", [Query foreground. \ Save terminal modes]),
  msg("cli", "child", [Spawn child], create: true),
  msg("child", "child", [Create process group]),
  msg("child", "cli", [Write attempted group ID \ through private pipe]),
  msg("child", "child", [Suppress SIGTTOU]),
  msg("child", "tty", [Make child group foreground \ with tcsetpgrp]),
  msg("child", "child", [Restore entry dispositions, \ then mask. Attempt exec]),
  note("cli", "tty", [The pipe carries data, not an exec acknowledgement. Its descriptors close across exec. \ A background CLI lends only after it gains foreground, possibly at a later poll.]),

  div([A. Spawn reports success]),
  msg("cli", "cli", [Check Started. Restore \ the parent entry mask]),
  note("cli", "tty", [Started does not prove exec. A failed Started check takes bounded cleanup with no new lend. \ The steps below show the normal supervision path.]),
  msg("cli", "child", [Observe exit without reap]),
  msg("cli", "child", [Attempt final group stops \ while identity is reserved]),
  msg("cli", "child", [Reap the child]),
  destroy("child"),
  msg("cli", "tty", [Query foreground for recovery]),
  note("cli", "tty", [Recovery requires a lease that held foreground. Otherwise, take no foreground and restore no modes.]),
  div([A1. Normal exit: choose by current holder]),
  note("cli", "tty", [Child group: reclaim foreground. CLI group: keep it. \ Another group: preserve that holder and its modes.]),
  div([A2. Signal death: choose by current holder]),
  note("cli", "tty", [Reclaim from another positive holder except the POSIX session leader's group. \ Keep the CLI's own foreground. A nested shell can lose foreground under this rule.]),
  msg("cli", "tty", [If CLI ownership is confirmed, \ restore saved modes]),
  msg("cli", "cli", [After recovery, query group \ absence with signal 0]),

  div([B. Instead: failed spawn or unknown child status]),
  note("cli", "tty", [No reap is promised here. Recovery requires a lease that held foreground. \ Handback requires exclusive foreground authority and no unrelated group-ID reuse through recovery.]),
  msg("cli", "tty", [Query foreground for recovery]),
  note("cli", "tty", [Recorded child group: attempt handback. CLI group: keep it. \ Unrelated holder: preserve it and report recovery uncertainty. \ No valid record supplies no authority over another group.]),
  msg("cli", "tty", [If CLI ownership is confirmed, \ restore saved modes]),
  msg("cli", "cli", [Keep original failure and \ recovery reports]),
)

#render(sequence([Lend and recover the terminal — UML sequence], parts, rows, (
  [Blue: the Ravel CLI, with its runner inside. Grey: child group and terminal.],
  [A and B are alternative outcomes. B can begin before exec. A1 and A2 are alternative recovery rules within A.],
  [The X belongs only to A: confirmed child reap. It does not mark the group's disappearance or a reap in B.],
  [Another group can hold delegated foreground. The holder notes show when Ravel preserves it or reclaims it.],
  [Handback and mode restoration temporarily ignore SIGTTOU, then restore its prior disposition. Ravel checks these operations.],
  [Foreground query and handback are not atomic. CLI lifetime establishes neither of B's two conditions.],
  [After failed spawn, the attempted group ID never authorises numeric signals. Unknown status does not permit wider takeover.],
  [Recovery errors return no result to the caller. Cancellation can override the original failure, but keeps recovery reports.],
), max-label: 220))
