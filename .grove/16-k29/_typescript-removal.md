# typescript-removal-k29 — brief

## Goal

Make Rhai the only format of owner policy. Remove the Bun policy worker, the TypeScript SDK, their declarations, and their build and release steps.
A lone `policy.ts` refuses with a remedy that names `policy.rhai` and chapter 4 of the book.
The owner removed the migration guide in `rhai-book-use-k27` and chose a one-off manual conversion of their configuration.

## Done when

- `ravel` evaluates only Rhai policies. A lone `policy.ts`, and a `--config` file with a TypeScript or JavaScript extension, refuse as `policy_format_retired`, exit 3.
- The TypeScript worker, the SDK, the declarations, the Bun pin, the Bun notices and every worker build, typecheck, probe and hostile-runtime task are gone.
- The installation holds `bin/ravel`, `share/ravel/examples/` and the third-party notices of the Rust dependency tree. The archive manifest, the formula and the formula test match, and the formula test runs a Rhai policy.
- The tests that used TypeScript policies now use Rhai policies or frozen fixtures.
- `task docs:examples`, `task docs:excerpts` and the tests of both checkers pass. No book example or checker runs a TypeScript policy or needs the worker.
- The spec set, the ADR set, `CONTEXT.md` and the README describe the one remaining path.
- `task check`, `task test` and `task release:smoke` pass.

## Decomposition

- `rhai-equivalence-fixtures` freezes the results of the equivalence test while the TypeScript path still exists. It comes first because no later leaf can recreate them.
- `typescript-worker-cutover` removes the TypeScript path from the Rust code and the tests, and it adds the retired-format refusals.
- `typescript-delivery-removal` removes the worker, the SDK and the release machinery, and it installs the examples.
- `rhai-spec-reconciliation` moves the remaining policy text of the old spec into the new one, and it removes what no longer describes anything.

Each leaf leaves main working. The cutover leaf can run before the delivery leaf, because an installed but unused worker harms nothing.

## Pointers

- ADRs: `docs/adr/configuration-script-uses-rhai.md` and `docs/adr/configuration-evaluation-precedes-the-launch.md`.
- Specs: `docs/specs/configuration-evaluation.md` is the new design. Its sections "Installed layout and delivery" and "Migration" state the targets. `docs/specs/session-execution.md` holds the policy sections of 1.x.
- The leaf `rhai-example-policies` and its running log hold the equivalence test and the list of language differences.
- Skill for the reconciliation: `linkuistics:simplify-project`. It removes proven-dead code and artifacts, and it reconciles every consumer.
- Glossary: Owner policy, Policy worker and Host capability.

## Notes

- The delivery children set the final Rust notice path to `share/ravel/notices/THIRD-PARTY-NOTICES.md`. `task notices` generates `target/notices/THIRD-PARTY-NOTICES.md`; k108 integrates it with installation, archives and the formula. `rhai-book-inside-k34` reconciles chapter 6 and the shared installed-file figure with this path after the layout lands.

- `typescript-worker-cutover-k31` closes with k101. Runtime evaluation is Rhai-only, with the complete retired-extension/default-discovery table, ordinary Bun/Node grants, routing schema 2 and source-sensitive Rhai identity. Frozen equivalence observations are unchanged; current runtime tests and all complete book examples use Rhai. macOS check/test pass 602 cases each, both book checkers and their controls pass, and Linux arm64 passes current Rhai/native-entry/check/test tasks. The real jj workspace comparison runs on macOS; the Linux image uses its documented no-jj skip. The old incumbent/R01 campaign was retired from `test:native-linux` after it failed on the intentional runtime/test contraction; original campaign receipts remain untouched. k32 owns the unused delivery worker, SDK, build/release machinery, installed-smoke/formula TypeScript fixtures and obsolete campaign tools. k33 owns spec reconciliation. k101 also removes chapter 6's obsolete JSON-marker reservation and stale frame-source pointers as direct runtime-removal consequences; later book leaves keep their wider narrative duties.

- F18 in `rhai-book-use-k74` adds `configuration-names-k92` immediately after this node.
  This node deliberately completes the removal under the current interface names. The following leaf performs the coordinated rename.
  Freeze the pre-rename observations first. Do not pre-empt that rename piecemeal or regenerate fixtures after it.
- Book tools use a checkout-local Bun, for example `scripts/ste.ts` and `scripts/breaks.ts`. They are development tools and stay. The release carries no Bun.
- Rhai still lists `smartstring`, which is MPL-2.0, as a dependency, although `compact_str` keeps its code out of the build. The licence allowlist records that decision.
- A Taskfile task generates the notices from the Cargo metadata with a pinned tool. It refuses a licence outside an allowlist.
- The book chapters 7 to 11 describe what this node removes. The node `rhai-book-inside` rewrites them afterwards. The leaves of this node keep `task docs:examples`, `task docs:excerpts` and the tests of both checkers passing. `typescript-worker-cutover` ports each complete example of chapters 7 to 11 that runs a TypeScript policy. The port changes the policy, and it leaves the prose alone. A moved excerpt is re-anchored in the leaf that moves it. No leaf of this node may retire with a failing book check.
- Search `docs/` for evidence files that describe the Bun runtime, such as `docs/runtime-evidence.md`. Remove a file only if nothing cites it and it describes nothing that still exists.

- `rhai-delivery-cutover-k110` delivers `bin/ravel` as the only executable, with `share/ravel/examples/{static,sample,llm}.rhai` and `share/ravel/notices/THIRD-PARTY-NOTICES.md`. Archives also keep LICENSE/README. `task notices` generates the notice at `target/notices/THIRD-PARTY-NOTICES.md`, and installation, archives and the Homebrew formula copy those bytes unchanged. `task install` uses `scripts/install.sh`; `task smoke` and target-floor checks use `scripts/installed-smoke.sh`. The worker/SDK/dispatcher, delivery Bun pin/notices and probes are removed; book Bun stays in `scripts/book-bun.sh`. k92 still owns configuration names; k33 owns spec reconciliation.
