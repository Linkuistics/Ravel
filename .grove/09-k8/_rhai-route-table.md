# rhai-route-table-k8 — brief

## Goal

Let an owner write a Rhai policy that selects a harness from a route table. The real `ravel start-session` evaluates it in a policy worker and launches the selected program. The TypeScript path stays beside it.

## Done when

- A `policy.rhai` with a route table selects a program, and the fake harness receives the argument vector from the table.
- The notice reports no host call, because this increment offers none.
- A hostile script cannot hold the Ravel CLI. Each engine limit, the heap bound and the deadline end the evaluation with the specified code.
- The TypeScript path works as before, and `task check` and `task test` pass.

## Decomposition

- `rhai-policy-worker` makes the Rhai path work end to end with no host call.
- `rhai-engine-limits` makes that path safe against a hostile script.

The first leaf must come first, because the second one tests the worker that the first one creates. Hostile classes are a leaf of their own, because each class needs a firing control and a discriminating test.

## Pointers

- ADRs: `docs/adr/owner-policy-is-a-rhai-script.md`, `docs/adr/policy-evaluation-precedes-the-launch.md` and `docs/adr/ravel-supervises-the-session.md`.
- Spec: `docs/specs/policy-evaluation.md`. Read the sections "Policy entry and discovery", "The policy module", "Request, context and selection", "Conversion at the boundary", "The policy worker" and "Cancellation and deadlines".
- Spec of the TypeScript path: `docs/specs/session-execution.md`. Its policy sections stay current until the cutover.
- Glossary: Owner policy, Policy worker, Policy context, Refusal and Evaluation budget, in `CONTEXT.md`.
- Test seam: the real `ravel start-session` against a sandbox home. Fake harness scripts on a private `PATH` stand behind it. The existing `crates/ravel/tests/support` and `crates/ravel/tests/hostile.rs` show the pattern, including the firing control.

## Notes

- Main keeps both policy paths until the removal increment. The extension of the entry selects the path, and the default entry prefers `policy.rhai`. Only the removal increment adds the refusal `policy_format_retired`.
- Pins: Rhai `=1.27.0` with the `compact_str` feature. Never enable the `unchecked` feature.
- The routing notice of the Rhai path uses schema version 2. This increment reports the members that exist. Later increments add `modules`, `reads`, `llmCalls`, `harnessRuns` and the new `bounds`.
- Run `task docs:excerpts` and `task docs:examples` after a change to code that the book quotes. Fix an excerpt that moved. The prose rewrite waits for the documentation increments.
- Run `task test:native-linux` once for the worker on Linux, as the last check before the increment closes. The worker executes the running image on Linux and the canonical path on macOS.
- Rhai 1.27.0 defaults to a lower expression depth in a debug build than in a release build, so a test policy with deeply nested expressions fails to parse under `cargo test` until the engine sets its own limits. `rhai-engine-limits` sets the fixed table, so its tests must not depend on the build profile.
- The Rhai path is joined to `start-session`. `choice.rs` evaluates it through `rhai_worker::front`, and `policy::judge_result` applies the result checks. Hostile-script tests go through the real CLI with the helper in `crates/ravel/tests/support/rhai.rs`.
