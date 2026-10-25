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

## Decisions (running log)

- Reviewed producer commit `630bb30b550bc27885c4537c238eac1b8779458d` against its parent and unchanged current implementation. This is an inspection-only review: no tests, builds, lint, formatting, executable probes or code mutations run. The producer's check results remain reported evidence.
- The codebase-memory CLI could not start because a pre-coordination or unverified generation was active. No project, generation or coverage result was available. Used direct source inspection of the bounded implementation and pinned Rhai dependency instead.
- Two actionable omissions established from source: deprecated array `drain`/`retain` string overloads still wrap limit errors, and array `order`/`order_by` still discard comparer errors through a direct Rust call to the original sorter. Record these as separate findings and hand them to an integration leaf.
- All review questions have dispositions below. Created adjacent integration `rhai-try-boundary-limits-k48` for independent triage. It keeps `rhai-route-table-k8` open, so this review does not close the increment or claim its pending Linux check.

## Findings

### F1 — P2: String-name `drain` and `retain` still make a limit catchable

**Location:** `crates/ravel/src/rhai_worker/pointer_calls.rs:66`.

The deprecated-function registrations stop after `reduce_rev`; neither `drain`
nor `retain` has a replacement taking `&str`. The pointer overloads at lines
64–65 take `FnPtr`, and do not replace the string signatures. Rhai's standard
array package still registers both deprecated functions.

In the pinned dependency, `src/api/deprecated.rs:1428` and `:1472` call the
original Rust `drain` and `retain` directly after constructing `FnPtr`. Those
calls bypass engine name lookup. The originals at
`src/packages/array_basic.rs:1982` and `:2129` invoke
`call_raw_with_extra_args`, whose final step wraps every error in a catchable
`ErrorInFunctionCall` (`src/types/fn_ptr.rs:653`).

This source-derived selection body therefore continues after a call-depth
breach, with `fn up(n) { up(n + 1) }` defined in the policy:

```rhai
let a = [1];
try { a.drain("up"); } catch (e) { }
#{ status: "selected", program: "recording-agent", args: [],
   provider: "p", model: "m", effort: "e", reason: "after a breach" }
```

Replacing `drain` with `retain` follows the same path. A named function that
breaches string size, array size or variable count is wrapped in the same way.
The operations latch does not observe these errors. The failed operation is
still prevented, and this finding does not establish a heap or deadline
escape; it violates the required refusal after the breach.

The replacement-list test at `pointer_calls.rs:766` has neither string-name
call. Its assertion that its list proves no replacement is missing is therefore
too strong. The parity list also omits these signatures, and the worker/CLI
tests exercise only `map`. Integration should independently triage this path
and extend verification to the omitted overloads, including successful controls.

### F2 — P2: `order` and `order_by` still discard the comparer failure

**Location:** `crates/ravel/src/rhai_worker/callbacks.rs:29`.

Only `sort`, `sort_by` and their supported string form are registered here.
Rhai's `src/packages/array_basic.rs:1870` exports another comparer consumer
under both `order` and `order_by`. Its implementation clones the receiver and
calls the original Rust `sort_by` directly at line 1877. That original drops a
comparer error through `.ok()` at line 1577. Registering a replacement named
`sort` or `sort_by` cannot intercept a direct Rust call.

With `up` as above, either of these statements in `select` reaches the call
depth and then returns normally, allowing a subsequent selection:

```rhai
let a = [1, 2];
a.order(|x, y| up(0));
// The same consumer is exported as a.order_by(...).
```

No `try` is needed. String-size, array-size and variable errors raised by that
comparer are also discarded. Operations remain covered by the latch. This is
the same refusal gap that motivated the sorter replacement, through two
additional exported names, and it contradicts Resource exhaustion's SHALL.

Neither new test list contains `order` or `order_by`. Integration should
re-derive this consumer and its aliases, then verify both harmless comparers
and breached comparers through the worker/CLI seam. This finding does not
prescribe how to implement that correction.

## Answers to the review questions

**Completeness and wrapping.** The direct-site count is correct: the pinned
source has twelve calls to `call_raw_with_extra_args` in `array_basic.rs` and
four in `map_basic.rs`. A direct-site list is not an exported-consumer list.
The extra deprecated string consumers in F1 and the Rust delegation in F2 are
the material omissions found in this bounded read.

The other enabled constructions of `ErrorInFunctionCall` agree with the
producer's explanation: `src/func/script.rs:410` passes `Exit` through, then
passes system exceptions through before wrapping ordinary errors; all the
runtime engine-limit variants are system exceptions
(`src/types/error.rs:369`). The construction in `src/func/call.rs:644` belongs
to `eval`, which the worker disables at `engine.rs:154`. The iterator
construction at `src/packages/iter_basic.rs:80` wraps a fixed arithmetic error
for a zero step. The additional constructions in `src/grain/vm/mod.rs` are
behind the dependency's `grain` feature; this project enables only default
features and `compact_str` for Rhai and runs the AST engine.

The package inventory was derived from `Engine::new`, `pkg_std.rs` and
`pkg_core.rs`: language, arithmetic, strings, iterators, function pointers,
bit fields, logic, math, arrays, blobs, maps and time. The debugging package is
feature-gated and not requested. Reading the callback/native-dispatch sites
in these packages exposed `order_by`, rather than assuming the sixteen wrapper
sites covered every callback consumer. Basic function-pointer helpers inspect
names; the evaluator handles `Fn`, curry and `call`.
`src/func/call.rs` routes direct, method, curried and map-attached pointer calls
to script/native dispatch without adding the unconditional wrapper. The
shared `FnPtr::call_raw` also retains curry handling for the replacements.
Collection equality, membership and the string index fallback use native
operator dispatch; their error branches propagate errors or handle a missing
equality function, rather than wrapping a limit. The no-comparer `dedup`
delegates to a native equality pointer; it does not accept a policy closure.
No additional exploitable limit-swallowing path was established by this read.

**Successful-call parity.** The replacement `call` copies the three attempts
of `src/types/fn_ptr.rs:610`: original arguments with `this`, then moving
`this` into the argument list when the missing-function signature matches,
then adding the extra arguments. Moving `this` inserts a clone and leaves
currying to `call_raw` in both implementations. The array callbacks move it to
slot 0, reducers to slot 1, and `for_each` never moves it. Index arguments are
the original forward index, including in reverse reduction. `zip` has two
value arguments and no `this`. Map callbacks keep the key argument and move
the value from `this` into slot 1. In particular, the replacement preserves
the original's key/value order even though some parity-test lambda parameter
names suggest the reverse.

Empty collections preserve their original unit, false, true, -1, empty
collection or initial-accumulator outcomes as applicable. The offset helper
matches `eval/target.rs:18`, including negative starts and starts past the end.
Error caller labels match the originals, including the surprising `filter`
label for object-map `map` and `index_of` for `find`. Purity matches each
original declaration, including impure deprecated string `map`/`filter` and
the corrected impure comparer functions. No additional successful-call
result mismatch was established for the signatures actually replaced.

**String fallback and partial mutation.** `index_of_named_from` follows both
deprecated overloads: it tries a valid function name first, falls back only
for the matching missing-function signature (possibly one wrapper deep), and
otherwise propagates the error. Value fallback uses native `==`, with false
for differing types that lack equality and an error for equal types that lack
it. It preserves the empty-array early return and start offset. The new map
`divide` matches `map_basic.rs:411` and `:475`: earlier retained entries and
every later entry survive; the entry whose callback failed is dropped, and
earlier drained entries do not return to the receiver. The array functions
retain the failing element and preserve earlier removals. These are deliberate
parity details, not proposed fixes.

**Ordinary policy changes.** The change is broader than limit errors: `call`
returns every non-catchable error unwrapped. In particular,
`try { [1].map(|x| exit(42)); } catch (e) { }` no longer continues through the
catch block. `Exit` propagates to the outer evaluator instead of becoming a
catchable wrapper; its value can replace the subsequent script result
(`src/eval/stmt.rs:985`). This behavior difference is explicitly recorded by
the producer. Other pseudo errors follow the same classification, though a
normal function's `return` is consumed at the function boundary. The changed
purity rejects mutation of a constant receiver as upstream does. The newly
replaced string `sort`/`dedup` also propagate ordinary comparer errors where
upstream discarded them, consistent with the already accepted pointer-form
change. These visible changes do not add a third finding. The parity suite
does not independently prove their intended behavior: it excludes exit and
most comparer-error cases.

**Bound mapping through a chain.** For the replaced path, script calls pass
system exceptions through, the replacement avoids wrapping them, and an outer
script function again passes them through. Even if a wrapper remains,
`engine.rs:540` recursively unwraps function/module errors.
`limit_of` at line 413 maps stack overflow, variables, operations and the data
labels to their bounds, and `selection_failure` invokes `limit_failure` with
that bound. The new engine and CLI hostile cases assert the bound name for
call depth, string size, array size and variables. The missing paths in F1/F2
lose the error before this mapping can help; they do not mislabel it.

**Controls and an additional mutation.** The worker and CLI hostile tests pair
each amount with a harmless control, and assert a specific bound; the CLI also
checks that the recording harness never ran. The list test's upstream engine
is a useful firing control for the listed wrapper paths: upstream catches the
recursion while the replacement errors. It skips that control assertion for
`sort`/`dedup` because upstream drops those errors. The parity test supplies
ordinary successful comparisons, receiver-mutation comparisons and selected
error comparisons. Its reference engine has only a small call-depth limit;
it does not use the worker's strict variables, missing-map-property policy or
other configured limits. Its generated prelude and named functions are
declared, and the illustrated scripts do not depend on missing properties.
The worker/CLI tests provide the separate strict-engine evidence for `map`,
not for every parity script.

An additional mutation to run in integration is changing `reduce_rev_with`'s
`move_this` from `Some(1)` to `Some(0)` in `pointer_calls.rs:353`. The existing
parity script with initial accumulator 5 and `|acc, x, i| acc * 10 + x + i`
is discriminating: its first reverse step changes from `5 * 10 + 4 + 3` to
`4 * 10 + 5 + 3`, and the later steps diverge too. This checks argument order
with a noncommutative reducer, beyond the producer's reported mutations.
This is a source-derived mutation design, not an executed mutation result.
The review procedure forbids editing test/production code and running tests,
so executing it belongs to integration. More urgently, expanding the inventory
is needed: a mutation of an unlisted consumer cannot be detected by the current
listed-consumer tests, as F1/F2 demonstrate.

## Verification limits and handoff

Coordinates refer to producer commit
`630bb30b550bc27885c4537c238eac1b8779458d` and the unchanged local Rhai 1.27.0
source under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rhai-1.27.0/`.
`Taskfile.yml` exists. The producer reports `task check`, `task test` and
`task docs:excerpts` passing; these were not reproduced here. Linux verification
remains due before the parent increment closes. No executable exploit or
mutation ran, no graph completeness claim is made, and no subagent was used.
F1 and F2 warrant an adjacent integration leaf for independent triage, fixes
and verification. The review changes only task notes and task-tree structure.
