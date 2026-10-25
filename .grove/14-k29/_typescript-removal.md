# typescript-removal-k29 — brief

## Goal

Make Rhai the only format of owner policy. Remove the Bun policy worker, the TypeScript SDK, their declarations, and their build and release steps. A lone `policy.ts` refuses with a remedy that names the migration guide.

## Done when

- `ravel` evaluates only Rhai policies. A lone `policy.ts`, and a `--config` file with a TypeScript or JavaScript extension, refuse as `policy_format_retired`, exit 3.
- The TypeScript worker, the SDK, the declarations, the Bun pin, the Bun notices and every worker build, typecheck, probe and hostile-runtime task are gone.
- The installation holds `bin/ravel`, `share/ravel/examples/` and the third-party notices of the Rust dependency tree. The archive manifest, the formula and the formula test match, and the formula test runs a Rhai policy.
- The tests that used TypeScript policies now use Rhai policies or frozen fixtures.
- The spec set, the ADR set, `CONTEXT.md` and the README describe the one remaining path.
- `task check`, `task test` and `task release:smoke` pass.

## Decomposition

- `rhai-equivalence-fixtures` freezes the results of the equivalence test while the TypeScript path still exists. It comes first because no later leaf can recreate them.
- `typescript-worker-cutover` removes the TypeScript path from the Rust code and the tests, and it adds the retired-format refusals.
- `typescript-delivery-removal` removes the worker, the SDK and the release machinery, and it installs the examples.
- `rhai-spec-reconciliation` moves the remaining policy text of the old spec into the new one, and it removes what no longer describes anything.

Each leaf leaves main working. The cutover leaf can run before the delivery leaf, because an installed but unused worker harms nothing.

## Pointers

- ADRs: `docs/adr/owner-policy-is-a-rhai-script.md` and `docs/adr/policy-evaluation-precedes-the-launch.md`.
- Specs: `docs/specs/policy-evaluation.md` is the new design. Its sections "Installed layout and delivery" and "Migration" state the targets. `docs/specs/session-execution.md` holds the policy sections of 1.x.
- The leaf `rhai-example-policies` and its running log hold the equivalence test and the list of language differences.
- Skill for the reconciliation: `linkuistics:simplify-project`. It removes proven-dead code and artifacts, and it reconciles every consumer.
- Glossary: Owner policy, Policy worker and Host capability.

## Notes

- Book tools use a checkout-local Bun, for example `scripts/ste.ts` and `scripts/breaks.ts`. They are development tools and stay. The release carries no Bun.
- Rhai still lists `smartstring`, which is MPL-2.0, as a dependency, although `compact_str` keeps its code out of the build. The licence allowlist records that decision.
- A Taskfile task generates the notices from the Cargo metadata with a pinned tool. It refuses a licence outside an allowlist.
- The book chapters 7 to 11 describe what this node removes. The node `rhai-book-inside` rewrites them afterwards. The leaves of this node keep `task docs:examples` and `task docs:excerpts` passing for the chapters that still describe present code. If a check cannot pass until the rewrite, record which one and why in the running log.
- Search `docs/` for evidence files that describe the Bun runtime, such as `docs/runtime-evidence.md`. Remove a file only if nothing cites it and it describes nothing that still exists.
