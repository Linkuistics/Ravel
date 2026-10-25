# rhai-endpoints-removal-k64

## Goal

Remove the `endpoints` settings key, `ravel::endpoints()` and the endpoint types. Settings keep the `harnesses` and `budgets` keys. After this leaf, `rhai-llm-calls-k15` meets its "Done when" except the upgrade task.

## Context

- The input is "Harness entries" and "Discovery" in `docs/specs/policy-evaluation.md`. `harnesses()` and `remaining()` stay.
- The module `src/endpoints.rs` holds the parsers of both endpoints and harness entries. Keep the harness parser, and move it to a module that has a fitting name.
- `rhai-settings-discovery-k14` built the `endpoints` key and `ravel::endpoints()`. Read its running log for the places it touched.
- The dependency `url` exists to check the base URL of an endpoint. Check whether anything else needs it.
- `excluded_grant` keeps the names that 1.x excludes, such as `NODE_OPTIONS`. Only its clause for an `apiKeyEnv` goes.
- Ravel 1.1.0 refuses an unknown key in `settings.json`. That concerns the owner's file, and this leaf does not touch it.

## Done when

- `endpoints` is an unknown key in `settings.json`, and the refusal names it. `ravel::endpoints()` is gone. A policy that calls it fails with an unknown function.
- The endpoint types, their parsers and their tests are gone. The harness parser and its tests remain, with the location style `harnesses.claude.program`.
- The discovery map of `harnesses()` stays as it is. Tests still cover `harnesses()` and `remaining()`.
- `url` is gone from `Cargo.toml` if nothing else needs it, and the running log says which crate keeps it otherwise.
- The glossary and the spec need no change. If either still names an endpoint, record the place in the running log and fix it.
- The TypeScript path works as before. `task check` and `task test` pass.

## Notes

- Sweep for the claim "no endpoint remains". Enumerate every token that matches `endpoint` in tracked files, then classify each one. Run a control that finds a known hit. The spec and the ADR may use the word to say what the owner removed.
- Do not touch `~/.config/ravel`. The owner's file never held an `endpoints` key.
