# rhai-delivery-cutover-k110


## Goal
Deliver the coordinated Rhai-only installed layout and remove the retired worker/SDK/delivery machinery. This child retains every delivery acceptance criterion of k108/k106/k32.



## Context
Consume k107's `target/notices/THIRD-PARTY-NOTICES.md` at `share/ravel/notices/THIRD-PARTY-NOTICES.md`. k109 removes only disconnected campaign tooling and Linux historical-object staging; it changes no delivery consumer. Read the k108 brief and original k32/k106 inventory.

Move `scripts/release-common.sh`'s exact manifest and executable set, release-build/test, release-archives.test, release-doctor/test, release-smoke/target/qemu, release-formula.sh/test.rb/template, installed-check and Taskfile deps/probes/install/smoke/release:formula together. Reconcile check.sh's crates/ravel/scripts glob before deleting that directory. Then remove `crates/ravel/worker`, dispatcher, old installed smoke, worker-check and runtime-integrity. Keep checkout-local book Bun, LICENSE/README archive metafiles, frozen equivalence observations and historical receipts. Search docs evidence and delete only uncited files describing no surviving mechanism; k109 keeps cited native-entry evidence.

Do not rename configuration interfaces (k92), publish, or edit live owner configuration. Keep k34's final notice-path handoff in the brief chain.

## Done when
- Installation and exact archives have `bin/ravel` as their only executable, three examples (`static.rhai`, `sample.rhai`, `llm.rhai`) and Rust notices, with no `libexec/ravel`, Bun pin/notices or SDK.
- The real formula test evaluates Rhai. All removed delivery paths have their consumers classified and reconciled.
- `task setup`, `task check`, `task test`, `task release:test`, `task release:formula`, `task smoke`, `task release:smoke`, `task docs:examples`, `task docs:examples:test`, `task docs:excerpts` and `task docs:excerpts:test` pass. Use only unreleased archive builds and temporary prefixes.
- Exercise the three targets at their floors. Disclose unavailable checks and decompose precise remaining work instead of retiring unmet criteria. Promote final layout/notices information upward for k34.

## Notes
