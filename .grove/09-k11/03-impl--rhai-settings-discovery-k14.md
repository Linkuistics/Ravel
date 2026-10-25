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
