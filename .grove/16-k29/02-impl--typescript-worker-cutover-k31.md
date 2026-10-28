# typescript-worker-cutover-k31

## Goal

Remove the TypeScript path from the Rust code and the tests. Ravel evaluates only Rhai. A TypeScript policy refuses with the migration remedy.

## Context

- The input is "Policy entry and discovery", "Errors and refusals" and "Migration" in `docs/specs/policy-evaluation.md`.
- The TypeScript path lives in `crates/ravel/src/worker.rs`, `policy.rs`, `build.rs`, parts of `run.rs`, `environment.rs` and `cli.rs`, and in the tests. The Rhai path was added beside it, and the earlier leaves marked the new code.
- The tests in `crates/ravel/tests` and the unit tests use TypeScript policies as fixtures. Port each of them to a Rhai policy, or remove a test that only proves a TypeScript property. State each removal in the running log.
- The hostile classes of the Bun worker die with it. The Rhai hostile classes already exist, so check that each old class has a Rhai peer.
- `crates/ravel/build.rs` embeds the digest of the worker source, and `scripts/examples.sh` (`require_fresh_build`) checks for that digest and for `libexec/ravel/ravel-policy`. Removing the digest therefore breaks the checker of the book examples. This leaf owns the repair of `scripts/examples.sh` and `scripts/examples.test.sh`.
- Chapter 9 of the book holds a complete example that writes `policy.ts` and runs `--config policy.ts`. It expects exit 0. After this leaf the command refuses with exit 3, so the example needs a Rhai policy of the same behaviour.
- If this leaf proves too big, decompose it. A natural order is the Rust code first, then the tests, then the help text. Every child leaves main working. No child removes the TypeScript path while a test, a book example or a checker still runs a TypeScript policy.

## Done when

- A lone `policy.ts`, and a `--config` file ending in `.ts`, `.mts`, `.cts`, `.js`, `.mjs` or `.cjs`, refuse as `policy_format_retired`, exit 3.
  The remedy names `policy.rhai` and chapter 4, "Write a configuration script". Draft `rhai-book-use-k88` gave chapter 4 that title on 2026-10-28. The owner removed the migration guide in `rhai-book-use-k27`.
  When both default files exist, Ravel evaluates `policy.rhai` and never opens `policy.ts`.
- A `--config` file with another extension refuses as `policy_unreadable`. The remedy asks for a `.rhai` file. A missing default entry refuses as `policy_missing`, and the message names `policy.rhai`.
- The Bun conversation, the policy snapshot validator, the worker source digest, the `libexec` lookup, the Bun version in the notice and the codes `selection_unsettled` and `context_loader_unsettled` are gone.
- The reserved environment names `BUN_*`, `NODE_OPTIONS`, `NODE_PATH`, `NODE_PRESERVE_SYMLINKS` and `NODE_CHANNEL_*` no longer refuse. The names `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*` keep their rules.
- The help text of `ravel start-session` and `ravel end-session` describes Rhai policies. Its examples use `policy.rhai`, and the `--policy-env` text no longer names Bun or Node.
- The routing notice has only schema version 2.
- No test uses a TypeScript policy except one that proves the retired-format refusals. `task check` and `task test` pass.
- The freshness check of `scripts/examples.sh` no longer needs the worker digest or `libexec/ravel/ravel-policy`. It refuses a `ravel` binary older than its Rust sources. `scripts/examples.test.sh` keeps a control that fires for a stale binary, and it drops the controls that belong to the old worker pair.
- Every complete example of the book in chapters 7 to 11 that runs a TypeScript policy now runs a Rhai policy. The behaviour, the exit code and the output stay the same. This leaf edits only the policy file and the `--config` argument of such an example. The prose waits for `rhai-book-inside`.
- Every book excerpt that this leaf moves is re-anchored on its new lines. `task docs:examples` and `task docs:excerpts` pass at the end of this leaf.
- `task test:native-linux` passes once on a Linux target for the changed worker lookup.

## Notes

- `rhai-equivalence-fixtures-k30` captured the TypeScript observations once in `crates/ravel/tests/equivalence/fixtures/{static,sample,harness-table}.json`. Keep these files unchanged and retain the independent Rhai fixture checks in `rhai_examples.rs`, including the real secondary jj workspace and its skip message. Remove the live TypeScript comparisons, their source constants and the ignored one-shot generator together with the `equivalence:freeze` Taskfile task. The earlier leaf's running log records the capture digests and the additional `context.encodedBytes` exclusion: each live count is checked before path normalization instead.
- The worker identity of the Rhai path replaces the source digest of the TypeScript worker. Check that `build.rs` still gives the Rhai worker a build identity that changes with the source.
- The installed layout still holds the unused worker until the next leaf. That harms nothing.
- The book examples that show the old refusals of `NODE_OPTIONS` belong to the chapters that `rhai-book-use` rewrites. Run `task docs:examples` and repair any example that this leaf breaks, as the done condition says. Chapter 9 is the known case. Search chapters 7, 8, 10 and 11 for others.
- A repaired example in a chapter that `rhai-book-inside` rewrites later is a mechanical port. Do not rewrite its prose here. Record each port in the running log, so that `rhai-book-inside-k35` and `rhai-book-inside-k36` know which examples already run Rhai.
