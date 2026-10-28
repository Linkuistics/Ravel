# rhai-only-runtime-k101


## Goal

Complete `typescript-worker-cutover-k31`: evaluate only Rhai, remove the unused TypeScript runtime path and prove the parent rollup.



## Context

- Previous children migrate live test and book consumers while retaining both runtimes. Read their running logs for removed tests and already-ported examples.
- Production removal starts with `worker.rs`, `policy.rs`, `build.rs`, `run.rs`, `environment.rs`, `authority.rs` and `cli.rs`; enumerate callers and all consumers before deleting symbols.
- The parent brief is the full acceptance contract, including fixture immutability, Rhai build identity, exact discovery/extension refusals, environment rules, notice schema and Linux check.

## Done when

- Every `Done when` of `typescript-worker-cutover-k31` holds. In particular, all six retired extensions and default precedence/refusal cases pass through the real CLI seam.
- Bun conversation, snapshot validator, source digest, libexec lookup, Bun notice field and unsettled codes are gone from runtime code. Rhai source changes still change its worker identity.
- Bun/Node environment names are accepted; `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*` retain their rules. Help describes Rhai and only routing notice schema version 2 remains.
- No executable TypeScript policy consumer remains except retired-format tests. Frozen observations are unchanged and independent Rhai checks remain, including real jj workspace coverage.
- `task check`, `task test`, `task docs:examples`, `task docs:excerpts` and both book checker tests pass. `task test:native-linux` passes once on a Linux target for the changed worker lookup.
- Reconcile moved source anchors and the complete parent acceptance list before retiring this child and closing the node.

## Notes

Installed unused worker artifacts are intentionally removed by the following `typescript-delivery-removal-k32`; spec reconciliation belongs to `rhai-spec-reconciliation-k33`. Preserve that ownership while keeping checks green. The book port restrictions in the parent remain binding for a missed example.
