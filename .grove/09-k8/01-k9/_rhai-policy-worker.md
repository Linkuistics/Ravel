# rhai-policy-worker-k9 — brief

## Goal

Make a Rhai route table work end to end through the real `ravel start-session`. The policy needs no host function. Ravel evaluates it in a policy worker, which is Ravel's own binary in an internal role.

## Context

- The input is `docs/specs/configuration-evaluation.md`. Read these sections first: "Policy entry and discovery", "The policy module", "Request, context and selection", "Conversion at the boundary", "The policy worker" and "Errors and refusals".
- The TypeScript path in `crates/ravel/src/worker.rs`, `policy.rs` and `run.rs` stays. The Rhai path is new code beside it. The existing frame protocol in `frame.rs` and the supervision of the worker (the private socket, the bounded stop, the diagnostics bound) are the starting point.
- Check first how much of `worker::evaluate` serves both workers. If the supervision cannot serve the Rhai worker as it stands, decompose this leaf. Make the first child a behaviour-preserving extraction that the TypeScript tests keep green.
- The worker re-executes Ravel's own binary. On Linux it executes the running image. On macOS it executes the canonical path of the Ravel executable. The role accepts no command-line input, and without the socket and the handshake it exits with status 2.
- The worker reports the build identity of its binary. A different identity refuses as `worker_identity_mismatch`, and a worker that cannot run refuses as `worker_missing`. Both exit 5.
- The hygiene of the TypeScript worker stays. The worker has a private mode-0700 empty directory, null stdin, an empty environment, and the socket at fd 3. It inherits no other descriptor.

## Done when

- A `policy.rhai` with a route table selects a program. The fake harness receives the arguments from the table, and the notice reports no host call.
- The entry rules hold, as the spec table says. With no `--config`, `policy.rhai` wins when both files exist. A `.rhai` file given to `--config` takes the Rhai path. A `policy.ts` alone and every other extension keep the TypeScript path.
- Ravel admits the entry only as a regular UTF-8 file of at most 1 MiB. It admits a symlink by its real file. `policy_missing` and `policy_unreadable` name the file.
- The module contract holds. `SCHEMA_VERSION` must be 3, `VERSION` must be a nonblank string, and `select(request, context)` must exist. The failures are `policy_parse_failed`, `policy_invalid` and `unsupported_version`, each with file, line and column where they apply.
- A policy that defines `load_context` refuses as `policy_invalid` with the message that this build does not support it yet. The leaf `rhai-measured-reads` removes that refusal.
- The engine runs in strict mode. An undefined variable fails when the script compiles. An absent map property fails with a location, and a property that holds `()` stays readable. `eval` is off. `print` and `debug` go to the diagnostics channels of the worker.
- The request map has every documented field. An optional field the caller did not give holds `()`. The `limits` map reports the effective bounds, with the defaults of the spec for the budgets that later leaves enforce.
- One converter turns Rhai values into JSON and back, with the rules of the spec table. It names the location of a refused value, such as `result.args[2]`.
- The result checks of the 1.x selection apply. A return of `()` refuses as `selection_abstained`. An uncaught error refuses as `selection_threw`. An unknown field refuses as `selection_malformed` with its location.
- The routing notice of the Rhai path has schema version 2 and the members that exist now. The failure document keeps schema version 1, and its `location` takes the form `LINE:COLUMN`.
- Tests at the real CLI seam cover the route table and the unknown field. They also cover both-files precedence and the parse failure. The last cases are a missing export, a wrong version, abstention and an uncaught error. The new sandbox helper for Rhai policies lives in `crates/ravel/tests/support`.
- The TypeScript tests still pass. `task check` and `task test` pass.

## Notes

- Add Rhai `=1.27.0` with the `compact_str` feature. Do not enable `unchecked`.
- The next leaf sets the fixed engine limits and the heap bound. Leave the engine at its defaults here, except for the strict mode.
- Mark the new code so that the removal increment can find the code it keeps and the code it removes.

## Decomposition

`worker::evaluate` mixes two things. The first is generic supervision of a child with a private socket: the private directory, the socket pair, the descriptor hygiene, the diagnostics capture, the bounded stop, the teardown report and the cancellation order. The second is the Bun protocol: locating `ravel-policy`, the hello with `bunVersion`, and the evaluate, context and select frames. The supervision cannot serve a second worker until the first part is separate from the second. So this leaf is a node with three children, in order.

- `worker-supervision-extraction` moves the generic supervision into a function that takes a launch description and a conversation. The TypeScript path calls it with its own launch and conversation. Behaviour stays the same.
- `rhai-engine-worker` adds the internal role of Ravel's own binary, the strict Rhai engine, the converter, the module contract and the worker side of a Rhai protocol. It adds the front side that launches the role through the extracted supervision. Unit tests cover it.
- `rhai-route-table-seam` joins the pieces to `ravel start-session`: the entry rules, the validation and result checks, the routing notice of schema version 2, the sandbox helper and the CLI tests.

Each child leaves `task check` and `task test` passing.
