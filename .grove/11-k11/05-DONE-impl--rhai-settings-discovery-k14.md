# rhai-settings-discovery-k14

## Goal

Add the three new owner settings keys and the discovery functions. An owner can name harnesses, LLM endpoints and budgets in `settings.json`. A policy can ask what exists, and what remains, and it cannot raise a bound.

## Context

- The input is `docs/specs/policy-evaluation.md`. Read "Discovery", "LLM endpoints and credentials", "Harness entries" and "Evaluation budgets and their composition".
- The ADR `docs/adr/owner-settings-name-the-llm-endpoints.md` records why the owner configures endpoints outside the script.
- The settings parser is `crates/ravel/src/settings.rs`, and the environment grants are in `environment.rs`. Unknown fields refuse as `settings_invalid`.
- The review finding F3 of `rhai-policy-k4` and its integration in `rhai-policy-k5` define the credential name rules. A credential name wins over every path that the policy can read.
- No network call happens in this leaf. Discovery reports configuration and availability, and never a recommendation.

## Done when

- The `endpoints` key parses. A name has 1 to 64 lowercase letters, digits and dashes, and its first character is a letter or a digit. The fields are `type`, `apiKeyEnv`, `baseUrl`, `structuredOutput` and `models`, with the rules of the spec tables.
- A base URL uses `https`, or `http` to a loopback address. It has no user information, no query and no fragment.
- The `harnesses` key parses. An entry has a `program` and optional `capabilities`.
- The `budgets` key parses, with the ranges of the budget table: `llmCalls` from 0 to 256, `llmOutputTokens` from 0 to 10,000,000 and `harnessRuns` from 0 to 16. The defaults are 8, 32,768 and 0.
- An `apiKeyEnv` name cannot be a grant. A `policyEnv` grant that names it refuses as `excluded_grant`, exit 2. A `--policy-env` flag that names it refuses the same way, before any worker starts.
- An `apiKeyEnv` cannot name `HOME`, `PATH`, `TMPDIR`, `LANG` or a name of the form `LC_*`. Settings that try refuse as `settings_invalid`, exit 2.
- `ravel::endpoints()` returns one map per endpoint in name order, with `name`, `type`, `available`, `reason`, `models` and `structuredOutput`. `available` is false when the credential variable is unset or empty, and `reason` is then `credential_unset`.
- `ravel::harnesses()` returns one map per entry in name order, with `name`, `program`, `available`, `resolved`, `reason` and `capabilities`. It resolves the program as a selected program resolves and runs nothing. `reason` is `not_found` or `not_executable`.
- `ravel::remaining()` returns `ms`, `llmCalls`, `llmOutputTokens`, `harnessRuns` and `sources`. The `limits` map of the request reports the effective values.
- Tests at the CLI seam cover each settings failure and each discovery function. They include the scenarios "A flag grant of the credential" and "A credential under a base name". The leak-probe scenario waits for the first LLM leaf.
- `task check` and `task test` pass.

## Notes

- Program resolution for a harness entry reuses `crates/ravel/src/program.rs`. The lookup is a filesystem call, so it runs on the reader thread of `rhai-measured-reads`.
- The harness-run budget stays 0 until the owner enables it. Nothing in this leaf runs a harness.

## Decisions (running log)

- Where the settings live. `crates/ravel/src/endpoints.rs` parses and holds one `endpoints` entry and one `harnesses` entry. A problem is a dotted location and a clause, and `settings.rs` turns it into `settings_invalid` with that location, so a refusal names `endpoints.main.baseUrl` or `budgets.llmCalls`. The budgets stay in `settings.rs` and `limits.rs` (`Budgets`), outside `Limits::to_json` and `to_request`. The notice and the TypeScript request therefore stay as they were, and `Bounds::from_limits` reads the budgets for the Rhai worker's `limits` map. The notice members for the new bounds stay with the leaf that uses them.
- The URL check uses the `url` crate (2.5.8, already in the local registry). It follows the WHATWG grammar, so `127.1` and `0x7f.1` normalise to 127.0.0.1, `LOCALHOST` lowercases, and `localhost.` stays refused. An empty user name with no password is no user information. The spec asks for no more. The HTTP client of the LLM leaf is reqwest on the same crate, so both read a URL one way.
- The spec was silent on four small rules, and the spec now states them. `apiKeyEnv` is a nonempty name with no `=` and no NUL. A `models` entry and a `capabilities` entry are nonempty strings. A `program` is nonempty. An `openai-compatible` endpoint with no `apiKeyEnv` is available, because it needs no credential.
- Credential and grant order. `Grants::read` takes the endpoints and refuses a flag that names a credential. `Settings::read` checks `policyEnv` after the whole file is parsed, because `endpoints` can follow `policyEnv`. `Grants::readable` returns false for a credential name before the base set and the grants. No settings or flag can reach that state, because both refuse earlier. A unit test builds the state by hand, so the rule holds if a later grant path forgets the check.
- Discovery placement. `endpoints()` and `remaining()` run on the runtime thread, because they read the environment and the clock and make no filesystem call. `harnesses()` runs on the reader thread through `program::resolve`, with the caller's `PATH` and working directory. The worker sends a frame with no argument for each, and the reply carries a `value` that the worker converts as any JSON.
- `remaining()` reports the budgets whole. No call spends an LLM call, an output token or a harness run in this leaf, so `rhai-llm-calls` and `rhai-harness-runs` subtract from `bounds`. `sources` counts down with `source_count()`, which `read` now shares, so the two cannot disagree.
- No review leaf is cut. The credential rules have a unit test and a CLI test each, and a mutation of each rule failed a test (below). The place where a credential reaches an HTTP request belongs to `rhai-llm-calls`, which owns the leak-probe scenario.

## Verification

- `cargo nextest run -p ravel --test rhai_settings`: 13 of 13. Unit tests: `endpoints::` (5), `settings::`, `environment::`, `limits::`.
- Mutations, each seen to fail its control and then undone: a flag grant of a credential not checked (2 tests), a base-name credential allowed (4), an empty credential counted as set (1), `not_found` and `not_executable` swapped (1), `sources` not counted down (1), the budgets ignored (3), a settings grant of a credential not checked (2), the credential rule dropped from `readable` (1).
- `task check` and `task test` pass: 541 of 541 (48 skipped by the suite's own filters). Two runs under a load average near 9 each failed one timing test (`partial_acquisition_cleanup_worker_timeout_cancel_int_wait`, then `rhai_limits::a_native_call_that_does_not_yield_stops_at_the_deadline`). Each passes alone, and a run with `--no-fail-fast` passed all 541.
- `bash scripts/excerpts.sh check`: 8 passed, 0 failed.
- Not run: the Linux arm64 campaign. `url` and its `idna` and ICU crates are pure Rust, but the `release:smoke` build with the Zig glibc 2.17 toolchain is the proof that they build for the three archive targets. The leaf that adds TLS runs it.

## For adjacent work

- `rhai-llm-calls`: `Endpoint` in `crates/ravel/src/endpoints.rs` holds `api`, `api_key_env`, `base_url`, `models` and `structured_output()`. `Session.directory` and `Host.directory` carry them to the front. Read the credential at the time of each call with `std::env::var_os`, and let `Host::remaining` subtract the calls it makes. `Host.bounds` already holds `llm_calls` and `llm_output_tokens`. The leak-probe scenario can use the caller variable of an endpoint's `apiKeyEnv` and `ravel::env`, which already throws `env_not_granted`.
- `rhai-harness-runs`: `discovery::harnesses` shows the lookup to reuse. `harnessRuns` defaults to 0, so a run refuses until the owner sets `budgets.harnessRuns`.
- `rhai-book-use` and the book chapter on settings: the keys `endpoints`, `harnesses` and `budgets` are new, and chapter 4 lists the keys of 1.x only.
