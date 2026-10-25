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
