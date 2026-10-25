# rhai-try-boundary-limits-k48

**Integrates:** rhai-try-boundary-limits-k47

## Goal

Triage the committed review of `rhai-try-boundary-limits-k46`, apply the findings
that hold, and verify the result against its contract.

## Context

- Read the findings and question dispositions in `rhai-try-boundary-limits-k47`.
  Its reviewed producer is commit `630bb30b550bc27885c4537c238eac1b8779458d`.
- The review inspected source only. Its examples and additional mutation design
  are source-derived, not executed probes or passing tests.
- The parent brief and the producer's running log carry the agreed contract and
  prior verification. The relevant spec sections are "The policy worker" and
  "Resource exhaustion" in `docs/specs/policy-evaluation.md`.
- The parent increment still requires `task test:native-linux` as its final
  check before it closes.

## Done when

- Each review finding has a source-based disposition: apply, reject with
  evidence, or record an accepted trade-off through the appropriate contract.
- Applied corrections have discriminating verification, using the project
  Taskfile for required checks. Verification limitations are explicit.
- The implementation, specification and recorded guarantees agree at the
  resulting reviewable boundary. The parent close checks its own brief.

## Notes

The review owns no fix. Re-derive its findings before changing the code; its
list is evidence for triage, not a prescribed implementation checklist.

## Decisions (running log)

- **Read the findings from the commit of `rhai-try-boundary-limits-k47`**, and re-derived each one from the Rhai 1.27.0 source before changing code. The codebase graph server was not available, so every claim rests on direct reads.
- **F1 is real: a code defect.** `api/deprecated.rs:1428` and `:1472` call the Rust `drain` and `retain` directly, and those wrap the error of the named function for `try`. A listing of the 14 deprecated array functions in `api/deprecated.rs` shows that `drain` and `retain` were the only two without a replacement. Fixed: `drain_named` and `retain_named` in `pointer_calls.rs`, impure like the originals.
- **F2 is real: a code defect.** `order` and `order_by` are one export, and `array_basic.rs:1877` calls the original `sort_by` in Rust, so the replacement of the name `sort_by` cannot intercept it. Fixed: `order_by` in `callbacks.rs` sorts a copy with the replaced `sort_by` and returns it. Registered under both names, impure like the original. A search of the other `call_raw` sites finds only `dedup_by_comparer` (replaced) and `sort_by` (replaced), and the sorts without a comparer call no policy code.
- **Verification.**
  - The parity test now compares `drain("big")`, `retain("big")`, `order`, `order_by`, an empty array and a constant receiver against Rhai's own engine. The list test runs the four new calls under `try` with a recursion.
  - Worker seam: the `dedup`/`sort` tests in `engine.rs` take `order` and `order_by`, and a new test runs `drain("f")` and `retain("f")` under `try` for call depth and string size, each with a control that selects. CLI seam: `rhai_limits.rs` has the same for comparers and a new test for `drain`/`retain`.
  - Mutations, each seen to fail the named test: `drain_named` unregistered, `retain_named` unregistered, `order` unregistered, `order_by` unregistered, `order` declared pure, `order` sorting the receiver in place. The reviewer's mutation (`reduce_rev_with` moving `this` to slot 0 instead of 1) fails the parity test and the list test.
  - `task check` passes (447 tests, formatting, lint, worker build). `task docs:excerpts` passes.
- **Accepted trade-off, stated in the module docs: the list is still a source inventory.** Rhai is built without the `metadata` feature, so a test cannot enumerate the registered functions that take a pointer. The inventory rests on a read of the pinned source, and the pin `=1.27.0` keeps it valid. A Rhai upgrade must repeat the read, which `pointer_calls.rs` and the spec now say.
- **Accepted trade-off: the parity reference is the stock `Engine::new()`.** It has a call-depth limit and none of the strict options of the worker. The new scripts declare every variable and use no missing property, so they run the same in the worker engine. The worker seam tests cover the strict engine for the limits.
- **Behaviour changes the review listed stay as k46 recorded them** (`exit` unwrapped from a callback, `dedup`/`sort` impure, comparer errors now propagate). `order` and `order_by` join them: an ordinary comparer error now ends the selection, as for `sort`. The spec covers the comparers in "Resource exhaustion".
- **Spec.** "The policy worker" lists `drain("f")` and the comparer functions `order` and `order_by`. The scenario "A breach inside a comparer" names them.
- **No in-session reviewer spent.** Every fix has an executable seam test and a failing mutation.
- **Flaky test, not caused by this leaf.** `rhai_limits::a_native_call_that_does_not_yield_stops_at_the_deadline` failed 3 of 5 isolated runs on the changed tree, and 1 of 6 on the unchanged commit, at a machine load of about 16–19. Its control asserts that a worker start returns within a 1 s wall-clock bound. It passed in the final `task check`. Not changed here.
- **The parent increment stays open.** The brief asks for `task test:native-linux` as the last check, and it was not run: it is a Docker campaign with its own cost. A new leaf `rhai-route-table-linux-check` carries it.
