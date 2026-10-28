# typescript-build-removal-k105


## Goal
Remove the retired TypeScript worker from normal development setup, builds and checks while preserving working book checks and release consumers until k106.

## Context

Read parent k32 for the complete delivery contract. k104 gives the book its own Bun setup. Reconcile `Taskfile.yml`, `scripts/check.sh` and `scripts/native-linux/measure.sh`: no worker dependency install, worker build/probe/typecheck or Bun runtime-integrity prerequisite in ordinary setup/build/check/native-entry. Remove obsolete public worker/probe/hostile and campaign task entries where no supported consumer needs them. Keep delivery-only helpers until k106 changes their consumers.

Move `scripts/examples.test.sh` and `scripts/excerpts.test.sh` fixtures to surviving source paths, including their generated `node_modules` rejection fixture. Check the current source: k101 already moved some fixtures. Preserve controls proving exclusions, stale builds and executable freshness; retired-worker dirtiness controls may remain as deliberate negative input if clearly labelled.

The current native Linux measurement calls `task setup && task worker && task probes`. Change it to the remaining development prerequisites and preserve source digests and current Rhai/native checks. No new claim about frozen native campaigns; historical receipts stay intact.

## Done when

- Normal setup/build/check/native-entry no longer install dependencies, build, probe or typecheck the retired worker.
- `task setup`, `task check`, `task test`, both book checker suites and complete examples/excerpts pass.
- The Linux task recipe contains only current prerequisites; verify on Linux when available and state any environment limitation.
- Installation/release consumers still have the helpers they need until k106; record exactly which files remain for that child.

## Notes

## Decisions (running log)

- 2026-10-28: Use the mandated k32 design and this leaf's bounded implementation contract. The implementation sequence is: capture the current task expansion and book-suite baseline; remove normal development worker prerequisites and obsolete task entry points; move the remaining generated-path fixture; run setup/build/check/test, both book suites and complete examples/excerpts, then the current Linux arm64 measurement. Keep the book-owned Bun fetch and all historical campaign receipts. No runtime, installed-layout or configuration-interface change belongs here.
- 2026-10-28: Graph discovery is unavailable: `codebase-memory-mcp cli list_projects` refuses because a pre-coordination or unverified generation is active. Use exact source and configuration searches instead; no graph completeness claim is made.
- 2026-10-28: Consumer searches show `scripts/release-build.sh` still calls `task deps` and `task probes`, and `release:formula` calls `probes`. Retain these two tasks as explicitly delivery-only until k106; remove the unsupported `worker`, `typecheck`, `test:hostile`, frozen-incumbent/campaign/R01 and `runtime:integrity` public entries. Keep their scripts and worker source for the delivery transition. Normal setup/build/check/native-entry invoke none of them.
- 2026-10-28: k101 already moved both suites' source excerpts to `crates/ravel/src/run.rs`. Move only the generated `node_modules` rejection fixture to `scripts/node_modules/pkg/index.js`. Preserve the deliberately newer retired-worker scratch input in `control_stale_source`: it proves the Rust executable's freshness check ignores that obsolete input.
- 2026-10-28: The full before/after task inventory removes exactly the ten intended unsupported entries, with no additions. Verbose dry runs against the parent's Taskfile and the current Taskfile make the control visible: the former expands `deps` and `worker` under normal development entry points, while the latter expands only book setup, Cargo and `check.sh`. Release callers still reference the retained `deps` and `probes`. macOS `task setup`, `task build` and `task check` pass; check runs 602 tests, with 48 skipped. Linux arm64 minimum-Rust build/lint/format, setup, Rhai and native-entry checks pass; complete test and book runs continue.
- 2026-10-28: Final macOS verification passes through `mise exec -- task setup build check test docs:examples:test docs:examples:mutants docs:excerpts:test docs:examples docs:excerpts`. Check and test each pass 602 cases with 48 skipped; their passing identities match item by item. The example and excerpt suites preserve every baseline control identity and kill all 30 and 24 respective mutations. All 22 complete book examples and nine source excerpts pass. Changed shell files, including `scripts/native-linux/measure.sh`, pass explicit shellcheck. The recorded Taskfile, Cargo manifests/lock/toolchain, top-level shell scripts, Linux measurement script and example fixture-page digests match before and after. Local diagnostic files are under `target/k105/` and `target/k105-book-baseline.log`; they are scratch outputs, not committed campaign evidence.
- 2026-10-28: `mise exec -- task test:native-linux PLATFORM=arm64 OUT=target/k105/linux-arm64` passes every step on Docker's native Linux arm64 kernel: minimum-Rust build/clippy/fmt, book-only setup, 258 Rhai tests, native-entry groups of 35/22/2, and check/test (602 passed and 48 skipped each). Check/test identities match item by item. The measurement's complete source digest maps match. The fresh container has no `crates/ravel/worker/node_modules`, `target/libexec/ravel` or `target/probes/ravel` after the run, proving it did not depend on host worker build residue. Removed the successful run's container after collecting its results. No x86-64 or frozen-campaign qualification is claimed.

### Files retained for k106

`crates/ravel/worker/` (including SDK, declarations configuration, manifests and fixtures),
`crates/ravel/scripts/dispatch.sh`, `crates/ravel/scripts/installed-smoke.sh`,
`scripts/worker-check.sh`, `scripts/installed-check.sh` and
`scripts/runtime-integrity.test.sh` stay for the installation/release transition.
`Taskfile.yml` retains `deps`, `probes`, `install`, `smoke` and the release tasks;
`scripts/release-build.sh` still calls the first two. k106 must reconcile
`scripts/check.sh`'s `crates/ravel/scripts/*.sh` lint glob if it removes that directory.
Release scripts, tests and `scripts/templates/ravel.rb.tmpl` retain their existing
layout; this leaf makes no new claim that those delivery checks pass.

Retired executable tooling also stays for k106's coordinated deletion:
`scripts/native-incumbent.sh`, `scripts/native-incumbent/`,
`scripts/native-campaign.sh` and `scripts/native-r01.sh`. Their public tasks are
gone. k106 also reconciles the campaign profiles in `.config/nextest.toml`, the
old git-object staging in `scripts/native-linux.sh` and dependencies/comments in
`scripts/native-linux/Dockerfile`. Preserve the current Linux environment and
its measurement, the checkout-local book tools, frozen equivalence fixtures and
historical campaign receipts.
