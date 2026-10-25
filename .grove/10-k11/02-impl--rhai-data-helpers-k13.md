# rhai-data-helpers-k13

## Goal

Add the pure data helpers and the environment read, and prove the capability list of Grove's helpers. A policy can then parse and encode JSON, make a UUID, resolve paths, read granted variables, and wrap another policy.

## Context

- The input is the section "Data helpers" of `docs/specs/policy-evaluation.md`. The section "Migration" lists the capabilities that Grove's helpers need.
- The bridge and the reads exist after `rhai-measured-reads`. Use its host-call mechanism for every function here.
- The environment rules are in `crates/ravel/src/environment.rs`. They merge the grants of `policyEnv` and of `--policy-env`, and they define the names that need no grant.
- The review finding F3 of `rhai-policy-k4` shows that an endpoint credential name must win over every readable path. `rhai-settings-discovery` owns that rule. This leaf must leave a clean place for it in `env`.
- The review noted that Grove's provider receipt has a fixed member order. `json_encode` sorts keys, so Grove's port joins encoded members. The spec states this.

## Done when

- `ravel::json_parse(text)` parses strict JSON and throws `json_invalid` on a failure.
- `ravel::json_encode(value)` writes the canonical form of the spec. It writes no whitespace and orders object members by code point. It writes the escapes of the spec in lowercase hexadecimal. It writes integers and shortest finite floats. For a string that Rhai can hold, it writes the bytes that `JSON.stringify` writes. A value it cannot encode throws `json_unencodable`.
- `ravel::uuid_v4()` returns a random version 4 UUID in lowercase hexadecimal, from the operating system.
- `ravel::path_resolve(base, path)` and `ravel::path_parent(path)` are lexical and read no file. For absolute inputs they equal `path.resolve` and `path.dirname` of Node. A relative base throws `path_not_absolute`.
- `ravel::env(name)` returns the caller's value, or `()` when the variable is unset. The readable names are `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*` and the granted names. Any other name throws `env_not_granted`. The worker itself has no environment.
- Tests at the CLI seam cover each function, with the edge cases of the spec. A table of path cases compares the results with Node's, written as fixed expectations.
- One test per capability of Grove's list proves it without Grove code. The list has these items: a measured read of a task file, and JSON parse and encode. It also has a handle and a marker line parsed with string functions, a UUID, a wrapper policy and a module import by path. The wrapper test imports a module, calls its `select` and changes the selection.
- `task check` and `task test` pass.

## Notes

- Rhai's own `to_json` writes Rust debug escapes, and its `parse_json` accepts Rhai map syntax. Neither is part of the contract. A test may show that the host functions replace them.
- Keep the conversion of integers and floats at the boundary exact. An integer beyond the signed 64-bit range becomes a float.
