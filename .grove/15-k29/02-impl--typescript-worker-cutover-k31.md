# typescript-worker-cutover-k31

## Goal

Remove the TypeScript path from the Rust code and the tests. Ravel evaluates only Rhai. A TypeScript policy refuses with the migration remedy.

## Context

- The input is "Policy entry and discovery", "Errors and refusals" and "Migration" in `docs/specs/policy-evaluation.md`.
- The TypeScript path lives in `crates/ravel/src/worker.rs`, `policy.rs`, `build.rs`, parts of `run.rs`, `environment.rs` and `cli.rs`, and in the tests. The Rhai path was added beside it, and the earlier leaves marked the new code.
- The tests in `crates/ravel/tests` and the unit tests use TypeScript policies as fixtures. Port each of them to a Rhai policy, or remove a test that only proves a TypeScript property. State each removal in the running log.
- The hostile classes of the Bun worker die with it. The Rhai hostile classes already exist, so check that each old class has a Rhai peer.
- If this leaf proves too big, decompose it. A natural order is the Rust code first, then the tests, then the help text.

## Done when

- A lone `policy.ts`, and a `--config` file ending in `.ts`, `.mts`, `.cts`, `.js`, `.mjs` or `.cjs`, refuse as `policy_format_retired`, exit 3. The remedy names `policy.rhai` and the migration guide. When both default files exist, Ravel evaluates `policy.rhai` and never opens `policy.ts`.
- A `--config` file with another extension refuses as `policy_unreadable`. The remedy asks for a `.rhai` file. A missing default entry refuses as `policy_missing`, and the message names `policy.rhai`.
- The Bun conversation, the policy snapshot validator, the worker source digest, the `libexec` lookup, the Bun version in the notice and the codes `selection_unsettled` and `context_loader_unsettled` are gone.
- The reserved environment names `BUN_*`, `NODE_OPTIONS`, `NODE_PATH`, `NODE_PRESERVE_SYMLINKS` and `NODE_CHANNEL_*` no longer refuse. The names `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*` keep their rules.
- The help text of `ravel start-session` and `ravel end-session` describes Rhai policies. Its examples use `policy.rhai`, and the `--policy-env` text no longer names Bun or Node.
- The routing notice has only schema version 2.
- No test uses a TypeScript policy except one that proves the retired-format refusals. `task check` and `task test` pass.
- `task test:native-linux` passes once on a Linux target for the changed worker lookup.

## Notes

- The worker identity of the Rhai path replaces the source digest of the TypeScript worker. Check that `build.rs` still gives the Rhai worker a build identity that changes with the source.
- The installed layout still holds the unused worker until the next leaf. That harms nothing.
- The book examples that show the old refusals of `NODE_OPTIONS` belong to the chapters that `rhai-book-use` rewrites. Run `task docs:examples` and report any example that this leaf breaks.
