# rhai-engine-limits-k10

## Goal

Make the Rhai worker safe against a hostile or careless script. Each engine limit, the heap bound, the output bounds and the deadline end the evaluation with the specified code. The Ravel CLI survives and reports it.

## Context

- The input is the worker created by `rhai-policy-worker`. Read its running log first.
- The spec sections are "The policy worker", "Evaluation budgets and their composition" and "Cancellation and deadlines". The table of fixed engine limits is in "The policy worker".
- Rhai 1.27.0 has two known faults that the spec cites. `sleep` blocks the thread for a time that the script chooses. `pad` with an empty padding string loops for ever. Rhai 1.27.1 would fix the second fault, but it is not released.
- The engine limits of Rhai differ between debug and release builds. Set every limit explicitly, so that tests and releases agree.
- Rhai passes limit errors through the boundary of a script function, and `try` does not catch them. A host latch must still refuse when another path swallows one.
- The existing hostile tests in `crates/ravel/tests/hostile.rs` show the pattern. Each hostile class keeps a firing control in the same test.

## Done when

- The worker sets the fixed limits of the spec table: operations, call depth, expression depth, string size, array size, map size, variables and script functions. The worker evaluates on one thread with a stack of 64 MiB.
- A breach ends the evaluation as `script_limit`, and the report names the bound. A script cannot catch it with `try`.
- A counting allocator in the worker enforces the heap bound of 1 GiB. A breach reports `memory_limit`, and the worker exits.
- The worker checks a stop flag at each Rhai operation, so a cooperative script stops at once.
- Tests at the real CLI seam prove each class. The classes are `sleep(600)` at the deadline and the `pad` loop. The others are an array that grows without end, deep recursion, a large string, an endless loop and a `try` around a breach. The `sleep` case exits 124 within the deadline and the teardown bound.
- Tests prove the output bounds. A policy that prints without end ends as `output_limit`. A message above 1 MiB ends as `message_too_large`.
- Tests prove cancellation. INT, TERM and HUP during the evaluation kill and reap the worker, and nothing launches. The exit status is 128 plus the signal number.
- A test proves that a worker killed from outside refuses as `worker_failed`.
- A test guards the closure case of rhaiscript/rhai#1193. Ravel calls named script functions, so a closure that a host calls after the script ends must not occur. The test shows that no such call exists.
- The worker identity checks have tests where the seam allows them. The checks are `worker_missing` and `worker_identity_mismatch`.
- `task check` and `task test` pass. `task test:native-linux` passes once for the worker on a Linux target.

## Notes

- A script cannot reach the heap bound before a size limit fires unless it builds many small values. Pick the hostile cases so that each limit has a case that reaches it first.
- Keep the worker free of a credential and of any file or process access. This leaf adds no host function.
