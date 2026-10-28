# typescript-delivery-removal-k32 — brief

## Goal

Remove the Bun worker, the TypeScript SDK and their build and release machinery. Install the Rhai examples and generate the third-party notices of the Rust dependency tree. Make the archives, the formula and the smoke tests match.

## Decomposition

- `book-bun-toolchain-k104` moves the checkout-local Bun fetch out of the worker dispatcher, with offline integrity controls. It preserves the book tools before their current fetch owner is removed.
- `typescript-build-removal-k105` removes worker prerequisites from normal setup/build/check and native Linux checks, and moves the book checker fixtures off retired source paths. Delivery consumers may retain the unused worker until the next child.
- `rhai-release-layout-k106` changes installation, archives, notices, smoke tests and the formula together, then removes the worker/SDK/dispatcher and retired campaign tooling after reconciling their last consumers.

## Decisions (running log)

- 2026-10-28: Split at the actual dependency boundaries above. The book fetch is a verified first increment; build/check removal and the installed-layout change each require their own fresh session. This session implements only k104. Keep frozen equivalence observations and historical campaign receipts unchanged.
- 2026-10-28: k104 delivers `scripts/book-bun.sh` and `book:bun:integrity`; setup uses that book-owned fetch. It preserves Bun 1.4.2, the existing three package digests and the default `target/tools/bun` path, with `BOOK_TOOLS_DIR` support. Setup/build, all requested book checks and repository check pass (602 tests passed, 48 skipped). The worker dispatcher is unchanged. k105 now removes development prerequisites; k106 removes the remaining dispatcher, its delivery-only Bun pins/notices and the old installed layout. The final Rust-notice path is still k106's decision and must reach k34.
- 2026-10-28: k105 removes TypeScript prerequisites from normal setup/build/check/native-entry and the current Linux measurement, and removes unsupported worker/typecheck/hostile/frozen-campaign task entries. Both book suites use surviving Rust source; the generated-path rejection fixture now uses `scripts/node_modules`. macOS setup/build/check/test, complete examples/excerpts and both checker suites/controls pass. Native Linux arm64 minimum-Rust, setup, Rhai, native-entry, check and test pass without worker dependencies or compiled worker/probe output and with unchanged source digests. `deps` and `probes` remain explicitly delivery-only because release-build/formula still call them. k105's task records the exact retained helper/campaign paths for k106, including the script lint glob, Docker staging/dependencies and campaign profiles to reconcile. Delivery checks and the installed layout remain k106's work.

## Context

- The input is "Installed layout and delivery" in `docs/specs/configuration-evaluation.md`.
- The files to remove include the directory `crates/ravel/worker` (source, SDK, declarations, sample, `bun.lock` and `package.json`), `scripts/worker-check.sh`, `crates/ravel/scripts/dispatch.sh`, `crates/ravel/scripts/installed-smoke.sh` and `scripts/runtime-integrity.test.sh`. Search first, because other files may depend on them.
- Consumers to reconcile are the Taskfile (`setup`, `deps`, `worker`, `typecheck`, `build`, `probes`, `test:hostile`, `install`, `smoke`, `runtime:integrity`), `scripts/check.sh`, `scripts/installed-check.sh`, the release scripts (`release-build.sh`, `release-smoke*.sh`, `release-doctor.sh`, `release-common.sh`) and their tests. Also reconcile `scripts/templates/ravel.rb.tmpl`, `scripts/release-formula.test.rb`, `scripts/native-linux/Dockerfile` and `scripts/native-incumbent`.
- The checkers of the book also depend on files that this leaf removes. `scripts/examples.test.sh` quotes `crates/ravel/worker/sdk/index.ts` as its excerpt fixture. `scripts/excerpts.test.sh` quotes the same file and a `node_modules` path under `crates/ravel/worker`. The comments of `scripts/book.sh` and `scripts/figures.sh` name `dispatch.sh`. Point each fixture at a file that survives, and keep its control firing.
- `rhai-equivalence-fixtures` and `typescript-worker-cutover` leave the book examples on Rhai policies. This leaf must keep them passing.
- Find where the book tools fetch their Bun. The scripts `ste.sh`, `breaks.sh` and `book.sh` need it. Keep that fetch, and move it out of `dispatch.sh` if the removal would take it.
- The examples come from `rhai-example-policies`. The archives install them under `share/ravel/examples/` as `static.rhai`, `sample.rhai` and `llm.rhai`.
- Use the skill `linkuistics:simplify-project`. Prove that a file is dead before you remove it, and reconcile every consumer.
- `native-incumbent-guard-build-rs-k51` (done) made the native-entry campaign pass again on Linux arm64. The frozen incumbent shares the compiled TypeScript worker, and `scripts/native-incumbent.sh` reads `crates/ravel/worker` and `dispatch.sh`. This leaf must therefore retire the campaign from `task test:native-linux` or build the incumbent's worker separately. Its running log states the two ways.
- If the leaf proves too big, decompose it. A natural cut is the build and test machinery first, then the release scripts and the formula.

## Done when

- The installation holds `bin/ravel`, `share/ravel/examples/` and the third-party notices. `libexec/ravel` does not exist.
- The Bun pin and the Bun notices are gone from the release. The book tools keep their checkout-local Bun.
- A Taskfile task generates the notices from the Cargo metadata with a pinned tool. It refuses a licence outside an allowlist, and the allowlist records the `smartstring` decision.
- The archive manifest, the release checks, the formula template and the formula test match the new layout. The formula test runs a Rhai policy.
- `task setup` no longer installs worker dependencies. `task check` no longer builds, probes or typechecks a worker.
- `task release:test`, `task release:formula`, `task smoke` and `task release:smoke` pass.
- `task docs:examples`, `task docs:examples:test`, `task docs:excerpts` and `task docs:excerpts:test` pass. A fixture or a checker that quoted a removed file now quotes a file that survives.
- `task check` and `task test` pass.

## Notes

- k106 decomposed at the independently verifiable Cargo-notice boundary. k107 adds the generator; k108 retains every coordinated install/archive/formula/smoke/removal criterion. The final notice path is `share/ravel/notices/THIRD-PARTY-NOTICES.md`, generated by `task notices` at `target/notices/THIRD-PARTY-NOTICES.md`. Carry that path to k34's chapter 6 and installed-file figure reconciliation.

- k101 removed the runtime path and brought forward retirement of the frozen incumbent/R01 from `test:native-linux`, after a real Linux run failed on the now-Rhai fixture and deliberately retired Bun test identities. That task now runs current Rhai, native-entry, check and test tasks with source digests; it makes no new frozen-campaign claim. Keep historical campaign receipts intact when removing the old tooling. The remaining executable TypeScript policies are delivery-only fixtures in `crates/ravel/scripts/installed-smoke.sh` and `scripts/templates/ravel.rb.tmpl`, plus the retired probe exercise in `scripts/release-formula.test.rb`. Reconcile or delete them with their existing delivery ownership. Current runtime tests and complete book examples use Rhai.

- Parts I and II proof requires the final third-party-notice path to reach the book.
  Chapter 6 and `installed-files.typ` currently name the notices without a path.
  Record the final path here for `rhai-book-inside-k34` to reconcile in chapter 6 and the shared installed-file figure.
  Keep the generated notices, archive manifest, formula and installed smoke checks consistent with that path.
- The release step `brew upgrade ravel` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. The release leaf of the root brief handles that. Do not change the step here.
- `task release:major` refuses outside the default colocated jj workspace. Do not run a release from a leaf in this node.
- The archive manifest check is exact. A file that the new layout adds or drops needs an edit of the manifest and of its control tests.

- `rhai-delivery-cutover-k110` delivers `bin/ravel` as the only executable, with `share/ravel/examples/{static,sample,llm}.rhai` and `share/ravel/notices/THIRD-PARTY-NOTICES.md`. Archives also keep LICENSE/README. `task notices` generates the notice at `target/notices/THIRD-PARTY-NOTICES.md`, and installation, archives and the Homebrew formula copy those bytes unchanged. `task install` uses `scripts/install.sh`; `task smoke` and target-floor checks use `scripts/installed-smoke.sh`. The worker/SDK/dispatcher, delivery Bun pin/notices and probes are removed; book Bun stays in `scripts/book-bun.sh`. k92 still owns configuration names; k33 owns spec reconciliation.

- Closure check on 2026-10-28: k110 completes the coordinated layout and removes all retired worker/SDK/delivery consumers. Required setup/check/test, archive/formula/install/target smokes and both book suites pass; k110 records the exact tests, target controls, limits, removal ledger and source/payload digest scope. Every acceptance criterion retained by this node is met. Final layout information is promoted upward and to k34; no additional leaf is required here.
