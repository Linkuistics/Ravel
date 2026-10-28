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
