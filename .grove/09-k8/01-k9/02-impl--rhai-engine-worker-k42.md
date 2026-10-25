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
