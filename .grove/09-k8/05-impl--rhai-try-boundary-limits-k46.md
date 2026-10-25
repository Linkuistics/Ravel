# rhai-try-boundary-limits-k46

## Goal

Make a call-depth, data-size or variable breach end the evaluation even when a `try` surrounds the call of a function pointer. Today the script can swallow the refusal. The worker still prevents the operation that breached the limit.

## Context

- `rhai-engine-limits-k45` found and accepted this gap. Read its running log first. The spec says what holds today in "The policy worker" and in the requirement "Resource exhaustion", where the requirement says a breach MAY be swallowed.
- The cause is in Rhai 1.27.0. `FnPtr::call_raw_with_extra_args` wraps every error in `ErrorInFunctionCall` (`src/types/fn_ptr.rs:653`), and `EvalAltResult::is_catchable` returns true for that wrapper whatever it holds (`src/types/error.rs:320`). So `try { [1].map(|x| up(0)) } catch (e) { }` continues. No hook reports these limits.
- `try` stays in the language. The spec's scenario for the LLM budget wraps a call in `try`.
- The operations bound already refuses through the latch in `crates/ravel/src/rhai_worker/engine.rs`. `crates/ravel/src/rhai_worker/callbacks.rs` shows how a replacement function enters the engine's own namespace.
- Weigh these options and their cost:
  - Replace `try` with a custom syntax. The syntax refuses a wrapped limit.
  - Replace each array and map function that takes a function pointer.
  - Wait for a Rhai release. The release must make the wrapper follow its inner error.
  - Accept the gap and keep the spec as it is.

## Done when

- The chosen option has a recorded reason. If it is to accept the gap, the leaf says so and retires with no code change.
- If the gap closes, each of call depth, string size, array size and variable count has a hostile test. The test puts `try` around a function-pointer call. Each test has a control that selects, and a mutation check that makes the test fail.
- The requirement "Resource exhaustion" says SHALL again for those limits, and "The policy worker" no longer lists the gap.
- `task check` and `task test` pass.

## Notes

- The review of `rhai-engine-limits-k10` asked for this stronger enforcement. It changes how `try` behaves. Give it a review leaf beside it.
