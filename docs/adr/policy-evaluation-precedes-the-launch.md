# Policy evaluation precedes the launch

Ravel evaluates the owner policy in a policy worker: a separate process that
Ravel reaps before the selected process launches. The worker is Ravel's own
binary in an internal role, and it runs the Rhai script. It only computes. The
Ravel CLI performs every host capability for it: measured reads, the
environment and harness runs through the runner. An LLM call is a harness run.
Owner policy never executes inside the process that supervises the session.

The [policy evaluation specification](../specs/policy-evaluation.md) owns the
protocol, the capabilities and the bounds. The ordering of worker and job stays
required. The Ravel CLI admits a selection only after it reaped the worker and
removed its private directory, as the
[C2 checks](../completion-failure-design-evidence.md#c2-checked-observations)
record. Every host call of the evaluation also ends before the launch. The
worker sends its result only after the reply to its last host call. The
waitability of the worker and the managed child at entry is part of the agreed
[native-entry contract](../specs/result-job-completion.md#checked-native-entry-transitions).

The trade-off is a hard stop against a cheaper bridge. The owner rejected a Rhai
thread inside the Ravel CLI. A thread cannot stop a native call that does not
yield. The Rhai 1.27.0 source has two: `sleep`, and `pad` with an empty padding
string. Rhai bounds each value but not the heap, and on a thread a memory
exhaustion or a stack overflow aborts the supervisor without cleanup.

A process lets the Ravel CLI kill the worker at the deadline, whatever the
script runs. It confines a memory bound, a stack overflow or an abort to the
worker. The price is a request and reply protocol for every host call, and the
worker lifecycle code that 1.x already carries. Reopen the thread design only
when Rhai can interrupt every native call and Ravel can bound the heap of one
thread.

The design also rejected two other shapes. A second binary under `libexec`
needs a pairing check and a second installed file, and its isolation is no
better than re-execution. Host capabilities inside the worker would let a kill
of the worker orphan harness jobs. The worker would also hold the caller's
environment and its files.
