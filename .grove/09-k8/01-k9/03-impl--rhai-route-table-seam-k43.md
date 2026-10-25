# rhai-route-table-seam-k43

## Goal

Make a Rhai route table work end to end through the real `ravel start-session`, with the TypeScript path unchanged beside it.

## Context

- The parent brief `rhai-policy-worker-k9` lists every `Done when` item for the seam. This leaf carries the items that need `start-session`: the entry rules, admission of the entry file, the result checks, the routing notice of schema version 2, the failure document and the CLI tests.
- `rhai-engine-worker-k42` provides the worker role, the converter and the front function that evaluates a source. `choice.rs`, `authority.rs` and `run.rs` call it when the entry takes the Rhai path.
- Entry rules: with no `--config`, `policy.rhai` wins over `policy.ts` when both exist. A `.rhai` file given to `--config` takes the Rhai path. A `policy.ts` alone and every other extension keep the TypeScript path. `authority.rs` currently admits only one personal default.
- The notice of the Rhai path carries `worker` with `path`, `version`, `buildId`, `rhaiVersion` and `rigVersion`, with no `bunVersion`. It reports no host call. The failure document keeps schema version 1, and `location` takes the form `LINE:COLUMN`.

## Done when

- Every item of the parent `Done when` that `rhai-engine-worker-k42` did not cover holds.
- Tests at the real CLI seam cover the route table, the unknown field, both-files precedence, the parse failure, a missing export, a wrong version, abstention and an uncaught error. The sandbox helper for Rhai policies lives in `crates/ravel/tests/support`.
- A `policy.rhai` defining `load_context` refuses as `policy_invalid` with the message that this build does not support it yet.
- The TypeScript tests still pass. `task check` and `task test` pass. `task docs:excerpts` and `task docs:examples` pass, and `task test:native-linux` runs once for the worker on Linux.

## Notes

- Mark the new code so that the removal increment can find the code it keeps and the code it removes.
