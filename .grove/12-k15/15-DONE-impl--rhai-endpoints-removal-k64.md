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

## Decisions (running log)

- **The harness parser moved to `src/harness_entries.rs`.** `src/endpoints.rs` is gone. The new module holds `Harness`, `Harnesses`, `Problem`, `parse_harnesses` and the field, name and string checks they use. It needs no `url` and no `environment`. Its two tests keep the location style `harnesses.claude.program`. One is the entry test. The other is the name rule, which now runs on harness names, because the endpoint test was its only other subject. `settings.rs` calls it as `harness_entries::parse_harnesses`.
- **`endpoints` is an unknown key.** The settings reader has no arm for it, so it refuses as `settings_invalid` at the location `endpoints`, with the message `holds the unknown key "endpoints"`. The remedy no longer lists the key. The new test `the_endpoints_key_is_an_unknown_key_and_the_refusal_names_it` covers three files, one of them with `harnesses` before it. It checks the code, the location, the file name and the message.
- **`ravel::endpoints()` is gone from every layer.** The `endpoints` arm of the front's host call, the `endpoints` function of `discovery.rs`, the `endpoints` name in the namespace loop of `bridge.rs`, `Directory::endpoints` and the wire table in the `rhai_worker` module doc. A policy that calls it ends with `selection_threw` and `Function not found: ravel::endpoints ()`. I ran that by hand first. The test `a_policy_that_calls_ravel_endpoints_fails_with_an_unknown_function` asserts the code and the message, and its control calls `ravel::harnesses()` in the same position and succeeds.
- **`excluded_grant` lost its credential clause only.** `Grants` had a `credentials` list, `not_credential()` and an `Endpoints` argument on `Grants::read`. All three are gone, and so is the check that `Settings::read` ran after the whole file. Nothing else in that rule changed, and `hostile.rs` still proves the `NODE_OPTIONS` class. A variable that an endpoint once named as its `apiKeyEnv` is now an ordinary grant. The test `a_variable_that_an_endpoint_once_named_as_its_credential_is_an_ordinary_grant` shows `--policy-env MAIN_KEY` reaching `ravel::env`. `--policy-env` help text named the same clause, and I removed it there too. The first sweep missed it because my grep cut each line at 200 characters, and the enumeration below found it.
- **`url` is gone from the workspace, and no crate keeps it.** Only `endpoints.rs` used it. `Cargo.lock` held `url` before this leaf and holds none now, and the lockfile lost 274 lines, which were `url` and its `idna`, `icu_*` and `percent-encoding` branch. `cargo tree --workspace --target all -e normal,build,dev -i NAME` answers "did not match any packages" for `url`, `idna`, `idna_adapter`, `percent-encoding`, `form_urlencoded` and `icu_normalizer`. The same command answers `rhai v1.27.0` for the control.
- **Tests that went with the endpoint types.** In `endpoints.rs`: the base URL test, the name rule on endpoint names, the field table of an endpoint and the credential-under-a-base-name test. In `environment.rs`: two credential tests. In `settings.rs`: the credential grant test, and the endpoint cases of two others. In `tests/rhai_settings.rs`: the endpoint cases of the settings table, and the settings grant. Also the flag grant, the credential under a base name and the `env` refusal of a credential. Last, the two `endpoints()` discovery tests. `harnesses()` and `remaining()` keep their tests. The discovery test for module top level, `load_context` and `select` now counts harness entries and `harnessRuns`. Its expected arguments are `[",1,0", "1", "0", "1"]`.
- **Glossary and spec need no change.** The spec holds no `endpoint` token at all. `CONTEXT.md` names "LLM endpoint" once, as a term under `_Avoid_` for **LLM call**, which says what the owner removed. The ADR names endpoints twice, as two rejected shapes. All three stay.
- **The sweep.** I enumerated every token that contains `endpoint`, in any case, in the 3,645 tracked files that `jj file list` names, outside `.grove`. That is 41 tokens in 15 files, counted before the fix to `cli.rs`. Classification, with no leftover in code or help text now:
  - Not this feature: `crates/keyed-launch` (pipe and descriptor endpoints, three files), `docs/book/src/11-deliver-ravel.md` (the GitHub Pages REST endpoint, four), `docs/completion-failure-design-evidence.md` (protocol endpoints), `docs/research/layout-draw-separation-a.md` (`bendpoints` and a graph endpoint), `scripts/release-notes.sh` and `scripts/release-prepare.sh` (a revision endpoint).
  - Says what the owner removed: `CONTEXT.md`, the ADR, and the lines of `docs/design/rhai-policy` (`README.md` and `diagrams.json`) that say the design removed the LLM endpoints.
  - This leaf's own words: the three lines in `settings.rs` and the tests of `tests/rhai_settings.rs` that name the removed key and function.
  - A hit that was a leftover: `cli.rs`, the `--policy-env` help. Fixed.
  - Controls. The same pipeline over `cli.rs` as it stood before this leaf (`jj file show -r @-`) prints `endpoint`, so the instrument finds the one real leftover. The same pipeline over `cli.rs` now prints nothing. A mutated pattern, `endpoit`, over `CONTEXT.md` prints nothing, although `CONTEXT.md` holds the token. The first control fired on a subject that I know is wrong, so the clean read of `cli.rs` is evidence.
  - `.grove`: the finished leaves and briefs describe how endpoints were built and removed, and I left them as history. Three live files name the word, and none needs a change. `20-impl--owner-policy-conversion-k39.md` says the owner's configuration "gains no endpoints". `13-k22/03-impl--rhai-llm-call-scenarios-k67.md` says an earlier test used a fake provider endpoint. The root `_BRIEF.md` says the node removes the endpoint settings. The brief of the closed node `rhai-local-capabilities-k11` still says its leaf parsed an `endpoints` key. That records what the node built, so I left it.
- **No review leaf is cut, and no reviewer was spent.** The change is a deletion of a feature that the compiler and the tests cover. The one claim the compiler cannot prove is the unknown-function failure, and a test with a control covers it.
- **`~/.config/ravel` is untouched.**

## Verification

- `cargo nextest run -p ravel --test rhai_settings`: 11 of 11, after the unknown-function failure was seen by hand.
- `task check` passed on the final sources. It runs format, shellcheck, worker build and probes, runtime integrity, worker types, `clippy --all-targets`, terminal recovery, then `cargo nextest run --locked --workspace`. The result is 546 passed, 48 skipped and none failed. `task test` gave the same totals. The total fell from the 555 of k63 by the removed endpoint tests, less the three that this leaf added.
- `task docs:excerpts`: 8 passed, 0 failed. The book quotes none of the changed code, and the one prose mention of `environment.rs` (RAVEL_* grants) is still true.
- The three dead-code warnings about `live`, `reset_peak` and `peak` in `rhai_worker/heap.rs` come from the test build. This leaf removed no reference to them, so they are not its change, and they do not fail `task check`.
