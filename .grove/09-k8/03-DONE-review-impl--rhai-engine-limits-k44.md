# rhai-engine-limits-k44

**Reviews:** rhai-engine-limits-k10

## Goal

Read the engine limits, the heap bound and the stop mechanism of the Rhai worker as an attacker would. Find the hostile script, the race or the unsafe step that the tests of `rhai-engine-limits-k10` do not reach.

## Context

- Read the commit of `rhai-engine-limits-k10` and its running log. The spec sections are "The policy worker", "Evaluation budgets and their composition" and "Cancellation and deadlines".
- The code is in `crates/ravel/src/rhai_worker/`: `heap.rs` (a global allocator that ends the process), `engine.rs` (limits, the `Latch`, error mapping), `role.rs` (the channel watcher and the arm of the allocator) and `choice.rs` (`limit_refusal`).
- The tests are `crates/ravel/tests/rhai_limits.rs` and the engine unit tests.

## Done when

- Each question below has an answer with evidence, or a finding.
- Questions to answer:
  - Is `heap.rs` sound? The allocator counts in the front too. Check `realloc` and `alloc_zeroed`. Check the breach path: `write` to fd 3, then `_exit`. Check a second thread that breaches while the first reports. Check a breach during the write of a normal reply.
  - Which limit breaches can a script still swallow? The log says the latch covers only the operations bound. Find a way to swallow a call-depth, data-size or variable breach that the log does not name. Say whether the swallowed breach lets the script do harm that the limit was meant to stop.
  - Does the stop watcher stop a script that the front abandoned? Look for a close of the socket that does not wake the read. Look for a script that outlives the SIGTERM.
  - Can a script make the progress hook or the latch count wrong? One example is a native call that runs many operations in one step.
  - Does any hostile class have a test whose control cannot fail?

## Notes

- Push and index assignment grow a container without a size check in Rhai 1.27.0. The heap bound is the backstop. Say if a script can use that gap in a way that matters. One way is to hold the worker below the heap bound and above the deadline.

## Decisions (running log)

- The reviewed producer is commit `90abaf1faa05fd4700782da8e7a31840894c3f41`.
  The checkout began clean on its empty successor. This review inspects source
  and the producer's recorded verification; it runs no tests, builds, linters
  or formatters and changes no production or test code. `Taskfile.yml` exists.
- Graph discovery could not start: `codebase-memory-mcp cli list_projects`
  refused because a pre-coordination or unverified generation is active. There
  are no graph results, generation, or coverage claims. Direct source reads
  cover the worker, supervisor, framing, conversion, refusal mapping, tests,
  specification and the pinned local Rhai 1.27.0 source.
- A second error-suppression mechanism is established: Rhai's `dedup` discards
  callback errors itself, without a script `try` or the catchable wrapper that
  the implementation log describes. Non-operation limits can therefore be
  followed by a successful selection. This is F1; resource containment and the
  promised refusal are distinct questions.
- The test for a combined operation budget exhausts that budget in its first
  phase. It cannot detect a reset between the top level and `select` (F2).
  Separately, the cancellation tests cannot detect removal of the socket
  watcher, because they exercise native sleep and supervisor signals (F3).
- All five review questions have a source-based disposition below. F1–F3
  warrant an adjacent `integrate-review-impl` leaf for independent triage.
  No allocator soundness defect or escape from a live CLI's deadline was
  established. The review changes only task notes and the task tree.

## Findings

### F1 — P2: Native callback consumers discard limit errors without `try`

**Location:** `crates/ravel/src/rhai_worker/engine.rs:123`, `:149` and `:168`.

The engine installs Rhai's standard array package but records only operations
breaches. Rhai 1.27.0's `dedup_by_comparer` discards every callback error with
`unwrap_or(Dynamic::FALSE)`. Its `sort` comparator similarly discards errors
with `.ok()`. This is a separate mechanism from the catchable wrapper described
in the producer log: it needs no `try` at all.

A policy of this shape reaches the call-depth limit and then selects:

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "limit-probe";
fn up(n) { up(n + 1) }
fn select(request, context) {
    let a = [1, 2];
    a.dedup(|x, y| { up(0); false });
    #{ status: "selected", program: "recording-agent", args: [],
       provider: "p", model: "m", effort: "e", reason: "after a breach" }
}
```

**Source evidence:** In the pinned Rhai source, `src/packages/array_basic.rs:1314`
calls `FnPtr::call_raw` and substitutes `false` for an error. At `:1576`, the
sort comparator also drops the error. `src/types/fn_ptr.rs:466` dispatches a
linked script closure to `call_script_fn`; `src/func/script.rs:304` rejects the
excess call level. After `dedup` returns, Ravel's latch has recorded nothing,
and `evaluate_with` accepts the selection. These are source-derived examples,
not executed probes.

The same `dedup` path can discard a string-size failure from
`let s = "x"; loop { s += s; }` inside its comparator, or a variable-limit error
from a comparator containing too many local declarations. A direct `try` is
not required in either case. Disabling `try` alone therefore cannot restore
the contract.

The inspected examples do not exceed the protected call depth or retain the
oversized string: the failed operation is prevented and its locals unwind.
They do, however, launch after a breach that the leaf and the normative
resource-exhaustion requirement say must end evaluation. The implementation's
new caveat at `docs/specs/policy-evaluation.md:253` does not reconcile the
unchanged SHALL at `:1081`. Integration needs to decide how to enforce that
contract, or obtain and record an explicit change to it, and cover this native
error-discard path. This finding does not assert a heap or deadline escape.

### F2 — P2: The operation-composition test passes when counters reset

**Location:** `crates/ravel/src/rhai_worker/engine.rs:967`.

`the_operations_bound_counts_the_top_level_and_select_together` uses
50,000,000 loop iterations in each phase and assumes that iterations equal Rhai
operations. They do not. The loop condition evaluates expressions, and each
body evaluates an assignment and its operand. Rhai tracks these separately
(`src/eval/expr.rs:216`, `src/eval/stmt.rs:202`, and the conditional while-loop
evaluation at `:536`). The top level alone exceeds 100,000,000 operations, so
`select` is never reached and the final `script_limit` assertion still passes.

A regression that uses Rhai's per-evaluation progress count instead of the
captured aggregate counter would therefore pass this test while resetting the
budget between module evaluation and selection. There are no isolated-phase
success controls in this test to expose that mistake. The current captured
counter appears to compose correctly; the defect is in the claimed proof.

Use workloads that each select successfully in isolation, then show their
combination refuses. Verify that resetting the counter between phases makes
that combined case fail. The integration owns those tests and their execution.

### F3 — P2: No process test distinguishes a working socket stop watcher

**Locations:** `crates/ravel/src/rhai_worker/role.rs:118`,
`crates/ravel/src/rhai_worker/engine.rs:1127`, and
`crates/ravel/tests/rhai_limits.rs:377`.

The engine unit test writes `stop_flag` directly. The real cancellation test
runs `sleep(600)`, which cannot observe that flag, and the supervisor sends TERM
and then KILL. The native-call deadline test has the same limitation. The role
test that closes its socket does so before sending an evaluate frame
(`crates/ravel/tests/rhai_role.rs:223`), before `watch` starts.

Deleting the watcher, or leaving its flag unset, therefore preserves all these
outcomes. In the case the watcher was added to cover—a cooperative policy whose
front disappears without sending a signal—the worker would keep running until
its operations bound. There is no discriminating process test for that
guarantee in the inspected suites.

Add a bounded process case that establishes evaluation is running, closes the
front's socket or kills the front without a cleanup signal, and observes prompt
worker exit. Keep a live-channel control and a firing check that suppresses the
watcher's store. The process must be cleaned up on a failed assertion too. This
is a verification gap, not an observed failure of the current watcher.

## Answers to the review questions

**Allocator accounting and soundness.** No unsafe-memory defect was established
in the current call path. `heap.rs:116` reserves the requested allocation size
and refunds it on a null return. `alloc_zeroed` does the same while delegating
zeroing to `System`. Growing `realloc` reserves only the difference and refunds
it on failure; successful shrinking releases the difference. Failed shrinking
keeps the old allocation counted. `dealloc` releases after delegating to
`System`. The allocator also counts in the CLI but remains unarmed there.
This counts requested live allocation bytes, not allocator metadata, resident
memory, or the evaluation thread's stack.

`arm` prepares and leaks the report before starting either worker thread.
Thread creation supplies publication of that already-initialised memory;
the relaxed pointer and length loads are not evidence of an uninitialised
frame in this topology. This reasoning depends on the existing single call to
`arm` before thread creation, rather than supporting concurrent re-arming.
`breach` allocates nothing, handles positive short writes, stops on a write
error, and calls `_exit(4)`. A second breaching thread sleeps while the first
reports and exits the entire process. If the peer has closed or a write fails,
a complete `memory_limit` report is not guaranteed, but worker death and the
supervisor's bounded stop remain available.

**Breach during a normal reply.** The evaluation thread must join before the
role builds and writes the normal reply (`role.rs:99–105`). The watcher only
reads, sets the flag and drops its state. Thus there are not two allocating
reply writers in this implementation. `write_frame` serialises the JSON body
before it writes the length header (`frame.rs:58`). A heap breach while
serialising the final reply therefore sends the prepared failure before a
normal header has been sent. The raw emergency write is not generally safe to
interleave with arbitrary concurrent protocol writers; no such writer was
found in this bounded inspection.

**Swallowed limits.** F1 adds a native error-discard mechanism to the already
recorded closure/`try` gap. Call depth and variable checks reject the operation;
string growth using `+=` is checked before mutation in Rhai's fast operator
(`src/func/builtin.rs:904`). Swallowing those failures loses the refusal, rather
than proving those particular resource checks can be exceeded. Parse-time
expression and function-count failures cannot be caught by executing policy
code. The operations latch still refuses after callback error suppression.

**Watcher and abandonment.** The production supervisor drops the extra worker
endpoint before conversing and drops `front_end` before teardown
(`worker.rs:596`, `:624`). The pre-exec descriptor handling replaces fd 3 and
closes higher descriptors on exec. The role's `try_clone` duplicates the worker
endpoint, not the front endpoint, so it does not itself prevent peer EOF. No
production descriptor path that keeps the front endpoint open was established.
The missing process discriminator is F3.

An inherited ignored or blocked TERM can let a native call outlive TERM, but
`worker.rs:1276` escalates to KILL after its grace. A dead front cannot perform
that escalation: `sleep(600)` or the empty-padding loop can survive front KILL
even after the watcher sets the flag. The existing ADR
`ravel-supervises-the-session` explicitly accepts orphaning after uncatchable
front death. This is not a newly established violation of its cleanup
contract, and this review does not claim that the watcher stops native calls.

**Progress accounting and oversized containers.** The hook's private counter
survives the two engine entry calls and ignores Rhai's reset count. It fires
again after a swallowed operations error. Native work is not one hook callback
per inner operation: array sorting, data-size walks, sleep and padding can run
between callbacks. No script path that resets the private counter or clears
the latch was established. F2 concerns the test, not a demonstrated counter
reset in production.

In-place growth can keep an oversized container below 1 GiB, then enter native
work beyond the configured evaluation deadline. For example, build an array
with more than 1,048,576 small integers using `push`, then call `sleep(600)`.
The heap bound does not end that case, and the progress hook does not preempt
the sleep. The CLI's socket deadline and process escalation still end it; they
do not depend on the container size. Sorting a large in-place container is
another source of native work between progress checks. This gap matters for
the meaning of the size limits and makes process supervision essential; it
does not establish a way to keep a live supervising CLI past its stop bound.

**Controls.** F2 is a concrete false-positive proof: a per-phase budget is
enough to pass the supposed aggregate-budget test. F3 identifies the other
missing discriminator. The CLI operation, recursion, doubling, heap, output,
message and native-call cases have nearby successful controls; those controls
do not establish coverage of the different paths in F1. The returned-closure
guard has negative conversion assertions but no same-test successful JSON
control. Other conversion and CLI route tests provide that positive evidence;
no additional functional finding is based on this arrangement.

## Verification limits

The inspection used the exact producer commit and the unchanged current
production/test source, its running log, the brief chain, glossary, cited ADRs,
the relevant policy specification and local Rhai `=1.27.0` dependency source
under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rhai-1.27.0/`.
Line coordinates refer to that commit and pinned dependency. No exploit,
mutation check, test, build, lint or formatter ran. No subagent was used.
The producer's reported macOS passes and partial Linux results remain recorded
evidence, not results independently reproduced by this review. The Linux task
failures recorded by the producer remain unresolved. Integration owns triage,
all fixes and all resulting verification.
