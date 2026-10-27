# typescript-delivery-removal-k32

## Goal

Remove the Bun worker, the TypeScript SDK and their build and release machinery. Install the Rhai examples and generate the third-party notices of the Rust dependency tree. Make the archives, the formula and the smoke tests match.

## Context

- The input is "Installed layout and delivery" in `docs/specs/policy-evaluation.md`.
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

- Parts I and II proof requires the final third-party-notice path to reach the book.
  Chapter 6 and `installed-files.typ` currently name the notices without a path.
  Record the final path here for `rhai-book-inside-k34` to reconcile in chapter 6 and the shared installed-file figure.
  Keep the generated notices, archive manifest, formula and installed smoke checks consistent with that path.
- The release step `brew upgrade ravel` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. The release leaf of the root brief handles that. Do not change the step here.
- `task release:major` refuses outside the default colocated jj workspace. Do not run a release from a leaf in this node.
- The archive manifest check is exact. A file that the new layout adds or drops needs an edit of the manifest and of its control tests.
