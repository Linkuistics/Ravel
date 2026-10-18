# The launched child is a job

The local runner puts its direct child in a new process group in the caller's
session, and supervises the whole group. It gives the child the controlling
terminal only when the caller owns that terminal's foreground; after termination
it owes recovery of foreground ownership and saved modes. Entry signal masks/dispositions
are restored for the child, including ignores inherited by a transparent wrapper.
Ravel uses this runner for the selected process, separately from policy evaluation.

The [acquisition evidence](../result-job-source-evidence.md#c4-interactive-pre-exec-handoff-followed-by-exec-failure)
identifies the original failed-exec path after foreground handoff. The runner
now retains the lease and a private attempted-group record through spawn,
checks foreground/mode recovery and retains every recovery failure beside the
original errno or cancellation. [C4 checks](../completion-failure-design-evidence.md#c4-checked-observations)
exercise that behavior on the measured host. Failed-spawn handback requires
exclusive foreground authority and no unrelated reuse of the recorded group ID
through handback; the record never authorises signalling that ID after reap.
CLI lifetime alone supplies neither premise. This conditional policy does not
establish an atomic query/handback or complete native-entry conformance.

A new session (setsid) was rejected for interactive children: it loses the caller's
controlling terminal and cannot receive a foreground handoff. A process group is
all escalation needs, while retaining terminal access. Reconsider setsid only for
noninteractive detached processes whose terminal ownership is deliberately absent.
Signalling only the child's PID was rejected because descendants could survive
completion. Waiting a second grace for descendants after the child exits was
rejected: descendants receive time during their parent's shutdown, then surviving
group members are terminated before the direct child is reaped, preventing group-ID
reuse from redirecting that signal.

The runner normally reclaims from its direct child's group. When the child died
by a signal, it may reclaim a delegated foreground group too: a supervisor killed
without cleanup can otherwise leave a raw-mode orphan reading the terminal.
The accepted residue is that a nested job-controlling shell can lose that terminal
after such a death. Job control activates from observed terminal ownership, not
an owner flag. Noninteractive launches instead isolate terminal/stdin/descriptors.

A descendant that deliberately leaves the group is outside group cleanup.
Uncatchable supervisor death can orphan the managed job; a caller must not infer
successful completion from such a death. Real runner process/PTY tests cover
escalation, descendants, cancellation, foreground and mode recovery. The transferred
[source observations](../component-evidence.md#local-runner-source-provenance) explain the original
terminal experiments; [the active spec](../specs/session-execution.md) sets Ravel's
result obligation after confirmed cleanup.
