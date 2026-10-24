# The launched child is a job

The local runner puts its direct child in a new process group in the caller's
POSIX session. It supervises the whole group. It gives the child the
controlling terminal only when the caller owns the foreground of that terminal.
After termination, the runner owes recovery of foreground ownership and saved
modes. The runner restores the entry signal masks and dispositions for the
child, including ignores that a transparent wrapper inherited. Ravel uses this
runner for the selected process, separately from policy evaluation.

The [acquisition evidence](../result-job-source-evidence.md#c4-interactive-pre-exec-handoff-followed-by-exec-failure)
identifies the original failed-exec path after foreground handoff. The runner
now retains the lease and a private record of the attempted group through the
spawn. It checks the recovery of foreground and modes. It retains every
recovery failure beside the original errno or cancellation.
[C4 checks](../completion-failure-design-evidence.md#c4-checked-observations)
exercise that behaviour on the measured host.

Failed-spawn handback requires exclusive foreground authority. It also requires
that no unrelated process reuses the recorded group ID through the handback.
The record never authorises a signal to that ID after the reap. The lifetime of
the CLI alone supplies neither premise. This conditional policy does not
establish an atomic query and handback.

The native-entry campaigns measured conformance on aarch64 macOS and aarch64
GNU/Linux only. Nothing qualifies a native x86-64 Linux kernel
([receipts](../native-entry-development-evidence.md#gnulinux-immutable-campaign-k52)).

The design rejected a new POSIX session (`setsid`) for interactive children.
Such a child loses the controlling terminal of the caller, and it cannot
receive a foreground handoff. A process group is all that escalation needs, and
the child keeps terminal access. Reconsider `setsid` only for noninteractive
detached processes that deliberately have no terminal ownership. The design
rejected a signal to the PID of the child alone, because descendants could
survive completion.

The design also rejected a second grace for descendants after the child exits.
Descendants receive time during the shutdown of their parent. Then the runner
terminates the surviving group members before it reaps the direct child. This
order prevents a reuse of the group ID from redirecting that signal.

The runner normally reclaims the terminal from the group of its direct child.
When a signal killed the child, the runner may also reclaim a delegated
foreground group. Otherwise a supervisor killed without cleanup can leave a
raw-mode orphan that reads the terminal. The accepted residue is that a nested
job-control shell can lose that terminal after such a death. Job control
activates from observed terminal ownership, not from an owner flag.
Noninteractive launches instead isolate the terminal, stdin and descriptors.

<!-- tc: depart synonym-rotation because confirmed cleanup is the spec's term for an observed outcome, while checks names an examination -->
A descendant that deliberately leaves the group is outside group cleanup. An
uncatchable death of the supervisor can orphan the managed job. A caller must
not infer successful completion from such a death. Real runner tests with
processes and PTYs cover escalation, descendants, cancellation, and foreground
and mode recovery. The transferred
[source observations](../component-evidence.md#local-runner-source-provenance)
explain the original terminal experiments.
[The active spec](../specs/session-execution.md) sets the result obligation of
Ravel after confirmed cleanup.
