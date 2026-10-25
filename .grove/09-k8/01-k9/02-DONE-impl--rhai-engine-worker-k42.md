# rhai-engine-worker-k42

## Goal

Add the Rhai policy worker as a unit that tests exercise without `ravel start-session`. Ravel's own binary gains an internal role. The role runs the strict Rhai engine, checks the module contract and calls `select`. The front side launches the role through `worker::supervise`.

## Context

- The parent brief `rhai-policy-worker-k9` has the whole goal. The spec sections to read are "The policy module", "Request, context and selection", "Conversion at the boundary" and "The policy worker".
- `worker::supervise` and `worker::Launch` come from `worker-supervision-extraction-k41`. `Launch` takes a program, an `arg0`, an environment and a remedy.
- The role is chosen by `arg0`, because the role accepts no command-line input. A fixed `arg0` marks it. Any argument beyond `arg0`, a missing socket at fd 3 or a failed handshake exits with status 2. Verify that `Command::arg0` and `/proc/self/exe` on Linux and the canonical `current_exe` on macOS give the worker the same binary.
- The front reads the entry source and sends it in the evaluate frame, because the worker reads no file. This leaf has no import, so one source is enough. `rhai-local-capabilities` adds imports.
- The default `Engine` of Rhai installs a file module resolver. Replace it with the dummy resolver, and cite the Rhai 1.27.0 source for the choice.
- The build identity of the binary needs a build-script digest of the crate sources. The Bun worker keeps its own `BUILD_ID`.

## Done when

- Rhai `=1.27.0` with `compact_str` is a dependency. The `unchecked` feature stays off.
- The worker role, the protocol (hello, evaluate, result) and the front launch are in new modules beside the TypeScript path. A comment marks what the removal increment keeps and what it removes.
- One converter turns Rhai values into JSON and back with the rules of the spec table and names the location of a refused value, such as `result.args[2]`.
- The engine runs in strict mode, with `eval` off, `print` to the worker's stdout and `debug` to its stderr. The engine keeps the default limits.
- The module contract is checked in the worker. `SCHEMA_VERSION` must be 3, `VERSION` a nonblank string, `select` must have arity 2, and a defined `load_context` refuses as the spec says. Parse failures, invalid exports and a wrong version come back with file, line and column.
- The request map has every documented field. An absent optional field holds `()`. `limits` holds the effective bounds.
- Unit tests cover the converter, the contract and one conversation through `supervise` against the real role. `task check` and `task test` pass.

## Notes

- The next leaf, `rhai-route-table-seam`, joins this unit to `choice.rs`, the entry rules and the notice. Keep the front API small: a function that takes the entry source, the request and the launch description, and returns what the judge needs.
- The fixed engine limits and the heap bound belong to `rhai-engine-limits`.

## Decisions (running log)

- The new code is the directory `crates/ravel/src/rhai_worker/`. The names are `mod.rs` (constants, bounds, wire types), `convert.rs`, `engine.rs`, `locate.rs`, `role.rs` and `front.rs`. The directory is not called `rhai`, because that name would clash with the `rhai` crate at the crate root.
- The module comment and the comment at the top of `worker.rs` mark what the removal increment keeps and removes. `build.rs` keeps the TypeScript digest beside the new `RAVEL_BUILD_ID`, which digests the crate sources, the runner sources, `Cargo.toml`, `build.rs` and `Cargo.lock`.
- Protocol: `hello`, `evaluate`, then `selection` or `failure`. The front sends the entry source and the request in the evaluate frame. The worker adds `schemaVersion` and builds `limits` from its own copy of the bounds. The LLM and harness budgets hold the spec defaults (8, 32,768 and 0) until the leaves that enforce them read settings.
- `worker.rs` changed in three small ways. `CHANNEL_FD` and `Conversation::failed` are `pub(crate)`. `receive` and `broken` gained generic forms, `receive_for` and `broken_for`, that take the remedy. The Bun path still calls the old names, so its behaviour is the same.
- Rhai 1.27.0 functions cannot see the constants of their own module by name, even without strict mode. A function reads one as `global::NAME`, and only when it is called through the module that `Module::eval_ast_as_new` returned. So the engine runs the top level once as a module, registers it as the static module `policy` and calls `policy::select(request, context)`. A call on the AST with `eval_ast(false)` cannot reach the constants. I added the `global::` rule to the spec. The example-policy leaf must write its route tables that way.
- Strict mode is `set_strict_variables(true)` plus `set_fail_on_invalid_map_property(true)`. `eval` is off with `disable_symbol`. The dummy resolver replaces the file resolver, with the source cited in `engine.rs`.
- Rhai 1.27.0 gives no position for a valid but wrong declaration, such as a blank `VERSION` or a `select` with one parameter. `EvalContext` has no position and `ScriptFnMetadata` has none. The `internals` feature would expose them, but the spec pins the features. So `locate.rs` scans the source for top-level `const NAME` and `fn NAME`, skipping comments and strings. A missing export has no location.
- The check order is `SCHEMA_VERSION` (missing or wrong type is `policy_invalid`, another integer is `unsupported_version`), then `VERSION`, then a public `select` of arity 2, then any `load_context`. The failure of a top level that raises is `policy_import_failed`, as the error table says.
- The converter refuses nesting deeper than 100 levels, at the location of the value. `serde_json` refuses to read a frame nested past 128 levels, so a deeper value would have broken the protocol instead of refusing at its location. I added the bound to the spec.
- A result frame over the message bound becomes a `message_too_large` failure in the worker, because the front reads no frame past that bound. The seam leaf maps that code.
- The context argument of `select` is `request.context` as the front sent it. The seam leaf attaches the measured sources, because it owns the `Measured` values.
- The role exits with status 2 for input beyond `arg0`, for a descriptor 3 that is no socket (`fstat`) and for a failed handshake. The evaluation runs on a thread with a 64 MiB stack. `main` compares the whole `argv[0]` with the role name, so a path that ends in that name is no role.
- Identity check on macOS: the launch uses the canonical `current_exe`. A unit test runs it with `Command::arg0` and shows that the child is the same image under the role name. On Linux the program is `/proc/self/exe`, and the same test runs there.
- Tests: 47 new, in the unit modules and in `tests/rhai_role.rs`, which drives the real binary with the role `arg0` over a socket pair. The unit test `one_conversation_runs_a_route_table_through_supervise` runs the same role code behind `supervise`, through a script that execs the test binary, because a test binary cannot take the role `arg0`. Removing the argument check made the matching test fail, so the control fires.
- `task check` and `task test` pass: 386 tests passed and 48 were skipped. `task docs:excerpts` passes. I did not run `task docs:examples`, because no quoted code changed.
- The dead-code expectation on `mod rhai_worker` in `main.rs` marks that the front half has no caller. The seam leaf removes it when it joins the front to `start-session`.
- Linux check: the 39 unit tests and the 8 role tests pass in the arm64 GNU/Linux container (`scripts/native-linux.sh`, toolchain 1.99.1). That includes the test that `/proc/self/exe` with `Command::arg0` runs the same image under the role name. The container needs `RUSTUP_TOOLCHAIN=1.99.1-aarch64-unknown-linux-gnu`, because the unprivileged user cannot sync the toolchain file. The full `task test:native-linux` run stays with the end of the increment.
