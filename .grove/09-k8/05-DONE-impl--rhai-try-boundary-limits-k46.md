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

## Decisions (running log)

- **Option chosen: replace each array and map function that calls a function pointer.** Source evidence, Rhai 1.27.0:
  - `FnPtr::call_raw_with_extra_args` is the only wrapper of a limit error. Its 16 call sites are 12 in `packages/array_basic.rs` and 4 in `packages/map_basic.rs`.
  - The other `ErrorInFunctionCall` sites do not matter. `func/script.rs:415` passes a system exception through. `func/call.rs:644` wraps only `eval`, which the engine disables. `iter_basic.rs:80` wraps an arithmetic error that holds no limit.
  - The deprecated string-name forms (`map("f")`, `sort("f")`, `dedup("f")`, `index_of("f")` and others in `api/deprecated.rs`) call the original Rust functions directly. They are part of the class, and the k45 replacement of `dedup` and `sort` did not reach them.
- **Rejected: a custom `try` syntax.** `$ident$` does not push the catch variable on the parser's variable stack. A syntax that may change the scope pushes a search barrier, and the barrier hides every outer variable from the strict-variables check. The catch block then fails to compile, or the engine must drop strict variables. `parse_custom_syntax` in `src/parser.rs:2440` shows both.
- **Rejected: wait for a Rhai release.** No release is available, and the pin is `=1.27.0`. **Rejected: accept the gap.** The replacement is bounded and the spec states a SHALL.
- The replacements keep what the originals do. Only the last step differs: a non-catchable error comes back unwrapped, and a catchable one is wrapped as before. So `try` keeps its meaning for every ordinary error.
- **Built.** `rhai_worker/pointer_calls.rs` holds the replacements and `call`, the port of `call_raw_with_extra_args`. `callbacks::install` registers both modules. The registration declares the purity of the original, because Rhai defaults to pure.
  - Replaced: 18 array functions with a pointer, 12 string-name forms, and 4 map functions.
  - The graph server failed to connect. Every claim rests on direct reads of the Rhai 1.27.0 source.
- **A defect of k45 found and fixed.** Its `dedup` and `sort` replacements registered as pure, because `register_fn` defaults to pure. The originals are impure. A constant could therefore be the receiver and be changed. `callbacks.rs` now registers them impure, and the parity test has `const A = [3, 1, 2]; A.sort(..)`.
- **The string forms of `dedup` and `sort` were a hole of k45.** `dedup("f")` and `sort("f")` call the original Rust functions, which drop the error. They are replaced now.
- **Behaviour changes, all outside `try`'s own meaning.**
  - A pseudo error or a limit that comes back from a pointer call is no longer wrapped. `exit()` in a closure is an example. Rhai passes the same errors unwrapped through a script function.
  - `dedup` and `sort` are impure, as in Rhai.
  - Every other result, error text and catch value is the same. The parity test compares 74 scripts against Rhai's own engine.
- **Spec.** "Resource exhaustion" says SHALL again for call depth, data size and variables. "The policy worker" no longer lists the gap. Two scenarios were added. This undoes the narrowing of k45, so the owner has nothing to approve.
- **Verification.**
  - Each of call depth, string size, array size and variable count has a hostile test with `try` around `[1].map(..)`, in `engine.rs` and through the CLI in `rhai_limits.rs`. Each has a control with a harmless amount, and the control selects.
  - A second test runs every replaced function (37 calls) under `try` with a recursion. The original catches it, and the replacement ends the script.
  - Each mutation fails the named test.
    - No `install` fails all three new tests, and the CLI test too.
    - `retain` unregistered fails the list test.
    - `call` that wraps everything fails the list test and the engine test.
    - `map` without the index, `drain` declared pure and `sort` declared pure fail the parity test.
  - `task check`, `task test` and `task docs:excerpts` pass.
- **Not run.** `task test:native-linux` runs once as the last check of the increment `rhai-route-table-k8`. The review leaf `rhai-try-boundary-limits-k47` sits beside this one, so the increment is still open.
