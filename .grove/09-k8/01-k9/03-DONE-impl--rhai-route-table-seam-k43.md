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

## Decisions (running log)

- `PolicyEntry` gains `format: Format`, either `TypeScript` or `Rhai { source }`. The front reads a Rhai entry once, bounded at 1 MiB plus one byte, and sends exactly those bytes. `authority::resolve` picks the path: a `.rhai` file given to `--config` takes the Rhai path, and with no `--config` an existing `policy.rhai` wins over `policy.ts`. "Exists" uses `symlink_metadata`, so a dangling `policy.rhai` refuses as `policy_missing` and does not fall back. With neither file, `policy_missing` names `policy.rhai`.
- The worker reply now carries the policy's `VERSION`: `Reply::Selection { version, result }`. The notice needs it, and the first leaf of the pair did not report it.
- `select` gets its own context argument, `selectContext` in the evaluate frame, which holds the caller's document with the measurement attached (`context::deliver`). `request.context` stays the raw document, as the spec table says. The front delivers the context before it starts the worker, so a `context_too_large` refusal comes before any policy error. The TypeScript path orders them the other way, because its worker loads first. A Rhai policy has no `load_context` yet, so the order has no other visible effect.
- The judge reuses the 1.x result checks. `policy::judge_result` takes a `Language` that selects the words of the remedy, so a Rhai refusal does not point at `ravel/sdk`. `ScriptFailure` becomes a `Refusal` in `choice::script_refusal`. A position becomes `LINE:COLUMN` in `location`, and a converter path such as `result.args[1]` stays in `location` when there is no position.
- The Rhai worker gets an empty environment, because the spec says it has none. A `--policy-env` grant is still validated and reported in `policyEnvironment`. It is for the `env` host function of a later increment, which reads the caller's environment from the front.
- The notice of the Rhai path has schema version 2, `worker` with `path`, `version`, `buildId` and `rhaiVersion`, and no other new member. I left out `rigVersion`, because Rig is not a dependency before `rhai-llm-calls`, and a member with no value would be invented data. That leaf adds it with the other members.
- The marks for the removal increment are `(typescript)` and `(rhai)` in the doc comments of `authority.rs`, `choice.rs`, `policy.rs` and `run.rs`. The sandbox helper in `tests/support/rhai.rs` stays.
- The notice of the TypeScript path and of the Rhai path both show `operationReports` of `stop attempt … Operation not permitted` when run in the Claude Code sandbox on macOS. The TypeScript path does the same on the same machine, so it is the environment and not this leaf.
- Controls: breaking the precedence rule makes `both_files_prefer_policy_rhai_and_never_open_policy_ts` and `a_dangling_policy_rhai_is_missing_and_does_not_fall_back_to_policy_ts` fail. A `policy.ts` of mode 000 proves that the Rhai path never opens it.
- Gates on macOS: `task check` (format, lint and 405 tests) and `task test` (405 passed, 48 skipped) pass. `task docs:excerpts` passes with 8 excerpts. `task docs:examples` passes with 14 examples, after a `touch` of `main.rs` that cleared its stale-build guard, which compares file times and cannot see that a test file does not enter the binary.
- Linux: `task test:native-linux PLATFORM=arm64` fails in its rustup steps with `could not create temp file /opt/rustup/tmp/…: Permission denied`, the container problem the previous leaf logged, and its Bun `worker-setup` step fails too. So I ran the worker's tests in the arm64 GNU/Linux container by hand with `RUSTUP_TOOLCHAIN=1.99.1-aarch64-unknown-linux-gnu`. The 39 Rhai unit tests, the 8 role tests and 18 of the 19 CLI tests pass there, which proves the `/proc/self/exe` launch behind the real `start-session`. The one failure is `a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path`, because that container run has no built Bun worker, and it passes on macOS. The whole campaign needs the container's toolchain repaired, which this leaf does not own.
