# rhai-try-boundary-limits-k47

**Reviews:** rhai-try-boundary-limits-k46

## Goal

Read the replacement of the array and map functions that call a function pointer. Read it as an attacker and as an owner. Find the `try` that still swallows a call-depth, data-size or variable breach. Find the ordinary policy whose result the replacement changes.

## Context

- Read the commit of `rhai-try-boundary-limits-k46` and its running log. The spec sections are "The policy worker" and the requirement "Resource exhaustion".
- The code is `crates/ravel/src/rhai_worker/pointer_calls.rs`, and `callbacks.rs` beside it. `engine.rs` installs them.
- The tests are in `pointer_calls.rs`, in the unit tests of `engine.rs` and in `crates/ravel/tests/rhai_limits.rs`.
- Rhai 1.27.0 is the pin. Its source is under `~/.cargo/registry/src/*/rhai-1.27.0`.
- The producer chose the replacement over a custom `try` syntax. The log gives the reason.

## Done when

- Each question below has an answer with evidence, or a finding.
- Questions to answer:
  - Is the list complete? The producer says the 16 call sites of `call_raw_with_extra_args` are the only wrapper of a limit error. The producer says the other `ErrorInFunctionCall` sites pass a system exception or sit behind the disabled `eval`.
    - Re-derive both claims from the source.
    - Check every native function that calls a `FnPtr`, a curried pointer, `call`, `Fn(..)`, an iterator or an operator. Check every package in the engine.
    - Find a path where `try` still catches a limit.
  - Is each replacement the same as its original while nothing fails?
    - Compare the argument order and the index as an extra argument.
    - Compare the binding of `this`, and its position when it moves into the arguments.
    - Compare the empty-array result, the caller name in the error and the `pure` flag.
    - Check the string fallback of `index_of("name")`. Check the dropped entry of a failed map `drain` and `retain`.
  - Does the replacement change the result of an ordinary policy?
    - A non-catchable error was wrapped and is now unwrapped. Examples are `exit` and a pseudo error.
    - `dedup` and `sort` are now impure.
    - The string forms of `dedup` and `sort` are now replaced.
  - Does a limit map to the right bound in `limit_failure`? The limit passes through a script function, a closure and a replaced function in a chain.
  - Does each new test have a control that can fail? The log lists five mutations of the producer.
    - Add one mutation that the producer did not make.

## Notes

- The parity test uses Rhai's own `Engine::new()` as the reference. Say if a script runs differently in the worker's engine, because the worker sets limits and the strict options.
- The producer ran no Linux check. The increment `rhai-route-table-k8` still needs `task test:native-linux` as its last check.
