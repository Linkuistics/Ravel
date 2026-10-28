# rhai-release-layout-k106


## Goal
Deliver only `bin/ravel`, the three Rhai examples and Rust dependency notices; remove the worker, SDK and all remaining delivery/campaign machinery after reconciling consumers.

## Context

The parent k32 retains the original full file/consumer inventory and acceptance contract. Read it and `docs/specs/policy-evaluation.md` under “Installed layout and delivery”. k104 preserves book Bun setup in `scripts/book-bun.sh`; k105 removes development prerequisites. This child owns the remaining cohesive layout change: install/smoke, release-build/smoke/doctor/common, exact archive manifest and controls, formula template/test, pinned Cargo-notice generation with licence allowlist (including the explicit smartstring MPL-2.0 decision), and deletion of retired artifacts once consumers move.

Remove `crates/ravel/worker`, dispatcher, old installed smoke and wrapper/runtime integrity helpers only after consumers move. Remove obsolete native incumbent/campaign/R01 executable tooling and reconcile Dockerfile dependencies and live references. Keep historical campaign receipts. Search docs evidence files and delete only when uncited and describing no surviving mechanism. Keep checkout-local Bun for book tools; releases carry no Bun.

The three installed examples are `static.rhai`, `sample.rhai` and `llm.rhai`. Record the final notice path in this task and promote it to the k29/root brief for `rhai-book-inside-k34`, which owns chapter 6 and the shared installed-file figure. k33 owns spec reconciliation; k92 owns the later coordinated configuration rename. Do not rename that interface here, regenerate frozen equivalence observations, publish a release or change the owner's live files.

## Done when

- Every parent k32 acceptance criterion holds, including exact archive layout, Rust notices and allowlist, Rhai formula test and no `libexec/ravel`.
- `task release:test`, `task release:formula`, `task smoke` and `task release:smoke` pass. Run only unreleased builds/smokes, never `task release:major` from this workspace.
- `task check`, `task test`, `task docs:examples`, `task docs:examples:test`, `task docs:excerpts` and `task docs:excerpts:test` pass.
- All removed paths and names have their consumers classified and reconciled. Disclose any target-specific check that cannot run; use decomposition for precise remaining work rather than retiring unmet criteria.
- The final notice path reaches the later book node through the brief chain; frozen observations and historical receipts remain intact.

## Notes
