# rhai-engine-limits-k45

**Integrates:** rhai-engine-limits-k44

## Goal

Triage the review of `rhai-engine-limits-k10`, apply the findings that hold,
and verify the resulting implementation against its contract.

## Context

- Read the findings and question dispositions in `rhai-engine-limits-k44`
  from its committed review artifact. The reviewed implementation is commit
  `90abaf1faa05fd4700782da8e7a31840894c3f41`.
- The producer's running log and the parent brief carry the implementation
  decisions and recorded verification. The review inspected source only.
- The relevant specification is `docs/specs/policy-evaluation.md`, especially
  "The policy worker", "Evaluation budgets and their composition",
  "Cancellation and deadlines" and "Resource exhaustion".

## Done when

- Each review finding has a source-based disposition: apply, reject with
  evidence, or record an accepted trade-off through the appropriate contract.
- Every applied correction has discriminating verification. Required project
  checks run through the Taskfile, and any remaining limitation is explicit.
- The implementation, specification and recorded guarantees agree at the
  resulting reviewable boundary.

## Notes

The review owns no fix. Re-derive each finding before changing the code; its
list is evidence to triage, not a prescribed implementation checklist.

## Decisions (running log)

- Source of the findings: the committed review `rhai-engine-limits-k44`. Each finding was re-derived against the pinned Rhai 1.27.0 source and by test. No subagent reviewer was spent. The graph server failed to connect, so every claim rests on direct source reads.
- **F1 is real. It is a real issue and a real trade-off.** Source evidence: `dedup_by_comparer` (`array_basic.rs:1322`) maps a comparer error to `false`. `sort_by` (`:1577`) maps it to a comparison by type.
  - Other sites in the array and map packages do not swallow a limit. Three (`:821`, `:913`, `:2277`) swallow only `ErrorFunctionNotFound`. The rest return the error.
  - So `dedup` and `sort` are the whole class that needs no `try`.
- **Applied.** `rhai_worker/callbacks.rs` replaces `dedup` and `sort`/`sort_by`. The replacements return the first error of the comparer. They sit in the engine's own namespace, which Rhai searches before its packages.
  - With the install line commented out, the limit test and the ordinary-error test fail. The parity control passes in both states. It shows equal results and equal argument order.
  - Behaviour change, stated in the spec: a `throw` in a comparer now fails the selection, as it does in `map` and `filter`.
- **Not applied. Accepted visibly.** `try` around the call of a function pointer still swallows a call-depth, data-size or variable breach. Example: `try { [1].map(|x| up(0)) } catch (e) {}`.
  - Rhai wraps the error in `ErrorInFunctionCall` (`fn_ptr.rs:653`). `is_catchable` returns true for the wrapper, whatever it holds.
  - Disabling `try` is not possible. The spec's scenario for the LLM budget wraps a call in `try`.
  - No hook sees these limits. A fix replaces `try` itself, or every function that calls a pointer. That is a redesign. It is not work for a leaf that applies findings.
  - The leaf `rhai-try-boundary-limits-k46` carries the stronger enforcement.
- **Spec reconciled to what holds.** "The policy worker" now says that the closure-and-`try` path loses the refusal and keeps the protection. It also names the `dedup`/`sort` replacement.
  - "Resource exhaustion" keeps its SHALL for `try` used directly. The operations bound and the heap bound end the evaluation however the script handles errors. The other limits inside a function-pointer call MAY be swallowed by an enclosing `try`.
  - This narrows a SHALL. The owner must approve it. The close-out says so.
  - A new scenario covers the comparer path.
- **F2 is real. The arithmetic of the review holds.** Measured: one `while` iteration costs 6 operations. The loop passes at 16.0M iterations and fails at 16.8M. The old test ran 50M iterations per phase, so the top level alone passed the bound.
  - The new test runs 10M iterations per phase, about 60M operations. It shows that each phase alone selects. It shows that the pair refuses.
  - Any cost from 5 to 10 operations per iteration keeps that shape.
  - Mutation check: the hook counts Rhai's per-evaluation count instead of the captured counter. The new test fails. The old test still passed.
- **F3 is real.** New test: `a_front_that_closes_the_socket_during_the_evaluation_stops_a_cooperative_script` in `rhai_role.rs`.
  - It starts the real role and shows `loop { }` still running with the channel open. It drops the socket of the front. It requires status 2 and the text "the front asked the evaluation to stop".
  - Mutation check: the watcher does not store the flag. The role then stops with status 1 and "the result could not be sent: Broken pipe". The status and the text discriminate, so the test has no timing assertion.
  - A guard kills the role if an assertion fails.
- Review questions without a finding keep the answers of the review. These cover allocator accounting, a breach during a reply, native calls that outlive the front, and in-place growth below the heap bound. This session found no reason to change one.
