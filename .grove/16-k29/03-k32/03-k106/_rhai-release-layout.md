# rhai-release-layout-k106 — brief


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

## Decomposition

- `cargo-notices-k107` supplies the pinned Cargo notice generator, its allowlist and executable refusal controls. It is independently usable before the installed layout changes.
- `rhai-installed-layout-k108` moves install, archives, formula and smoke tests together, consumes the generated notice, and removes the retired delivery/campaign artifacts after reconciling their consumers. All original delivery acceptance criteria above remain here.

## Decisions (running log)

- 2026-10-28: The consumer inventory spans the dispatcher, installed smoke, archive manifest and controls, formula, target-floor smoke, doctor and historical campaign tooling. Split the independently verifiable Cargo notice generator from that coordinated layout change. This session implements only the first child. No delivery surface is removed until k108.
- 2026-10-28: k107 establishes `share/ravel/notices/THIRD-PARTY-NOTICES.md` as the installed notice path. Run `task notices` to generate `target/notices/THIRD-PARTY-NOTICES.md`, then copy that file unchanged into the staged archive/install prefix. k108 reconciles the exact manifest, formula, installed smoke, README and book handoff with that path. Existing Bun notices stay until the coordinated removal.

- `rhai-delivery-cutover-k110` delivers `bin/ravel` as the only executable, with `share/ravel/examples/{static,sample,llm}.rhai` and `share/ravel/notices/THIRD-PARTY-NOTICES.md`. Archives also keep LICENSE/README. `task notices` generates the notice at `target/notices/THIRD-PARTY-NOTICES.md`, and installation, archives and the Homebrew formula copy those bytes unchanged. `task install` uses `scripts/install.sh`; `task smoke` and target-floor checks use `scripts/installed-smoke.sh`. The worker/SDK/dispatcher, delivery Bun pin/notices and probes are removed; book Bun stays in `scripts/book-bun.sh`. k92 still owns configuration names; k33 owns spec reconciliation.

- Closure check on 2026-10-28: k110 completes the coordinated layout and removes all retired worker/SDK/delivery consumers. Required setup/check/test, archive/formula/install/target smokes and both book suites pass; k110 records the exact tests, target controls, limits, removal ledger and source/payload digest scope. Every acceptance criterion retained by this node is met. Final layout information is promoted upward and to k34; no additional leaf is required here.
