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

## Decisions (running log)

- Placement. `json_parse`, `json_encode`, `uuid_v4`, `path_resolve` and `path_parent` run inside the worker. They register in the `ravel` namespace through the same `Bridge::namespace`, so a malformed argument ends the evaluation as `host_request_invalid` with the position, and a data failure is a thrown map. They send no frame, because the spec lets `json_encode` write any string Rhai can hold (16 MiB), and one worker message holds 1 MiB. Only `env` crosses the socket, because the worker has no environment and the front holds the caller's. This reads "use its host-call mechanism" as the registration, the validation and the thrown-map rules of the bridge, and not as a round trip for a pure function.
- `env` is a `host` call named `env`, answered on the runtime thread (no filesystem call). `Grants::lookup` in `environment.rs` is the single predicate for "readable" and the one place where `rhai-settings-discovery` puts the credential-name rule ahead of the base set and the grants.
- A value that is not UTF-8 throws `env_not_utf8`, because a Rhai string holds UTF-8 only, and a lossy value would change a name or path silently. The spec table gains the code.
- Randomness for `uuid_v4` comes from `getrandom` 0.4, which `tempfile` already locks, so `Cargo.lock` only gains an edge.
- Float form. `serde_json` writes floats in the shortest form that reads back (`1.0`, `1e-7`, `1e+21`, `9.223372036854776e+18`). The spec asks for no more, so the encoder keeps that form. A test round-trips each float and checks that it keeps its type. The spec names the form and the type rule.
- `path_parent` copies the algorithm of Node's `posix.dirname` and not the plain "remove the last segment". The brief demands equality with Node for absolute input. Node does not normalise there, so `/a//b` gives `/a/` and `//a` gives `//`. The spec states this. The fixed tables in the test file come from Node v26.12.1 (80 `resolve` rows and 16 `dirname` rows), and both unit tests in `helpers.rs` pin the same quirks.
- A relative `base` throws `path_not_absolute` even when `path` is absolute. The check does not depend on the other argument. `path_parent("")` throws too, where Node returns `.`.
- Rhai's `parse_json` is not the strict contrary of `json_parse`. It accepts a trailing comma, single-quoted strings and expressions, and it refuses `#{`. The spec sentence said "Rhai map syntax", so it now says what the test shows.
- Test scripts: Rhai forbids a `fn` inside a function, has no `Array::join`, trims in place and reserves `call`, `case` and a bare `_`. The test file defines helpers as global functions through `selecting_with`.

## Verification

- `cargo nextest run -p ravel --test rhai_helpers`: 24 of 24.
- Mutations run, each seen to fail its control and then undone: no version and variant bits (2 tests), no depth check in `json_parse`, a normalising `path_parent`, an absolute path that does not replace the base, every name readable, grants ignored, uppercase hexadecimal.
- `task check` passes: formatting, shellcheck, worker build and probes, runtime integrity, worker types, clippy over all targets, terminal recovery, then `cargo nextest run --locked --workspace` 517 of 517 (48 skipped by the suite's own filters). `task test` runs the same nextest command.
- `bash scripts/excerpts.sh check`: 8 passed, 0 failed. The book quotes none of the changed code.
- Not run: the Linux arm64 campaign (`rhai-worker` step of `scripts/native-linux/measure.sh`). `getrandom` is the one new dependency with a platform backend, so the Linux run is the check that a later session should add if it wants the proof.

## For adjacent work

- `rhai-settings-discovery-k14`: the credential-name rule goes first in `Grants::readable` (`crates/ravel/src/environment.rs`), and a test needs a caller variable named like an endpoint's `apiKeyEnv`, granted by `--policy-env`, that `ravel::env` still refuses. The `Session` already carries the grants.
