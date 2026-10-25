# rhai-policy-increments-k3

## Goal

Cut the agreed `rhai-policy` design into the smallest independently useful
increments, and order them by dependency. Each increment leaves main working.

## Context

- The spec and the ADR set from `rhai-policy-k2`, and any review of them.
- The running log of `plan-k1`. Main makes no release until 2.0.0. The
  workstream ends with the release, the Homebrew upgrade and the conversion of
  the owner's local configuration.

## Done when

- The tree holds each increment as ordered leaves or nodes, and each body names
  its input.
- The last increments cover the documentation, the removal of the TypeScript
  path, the release, the Homebrew upgrade and the local conversion.

## Notes

- Give the upgrade leaf the two consequences in the running log of `plan-k1`.
  The upgrade keeps Ravel 1.1.0 and `policy.ts` until the driver restarts.
- The book changes for 1.1.0 ran through the editorial kinds: draft, copy-edit,
  art and proof. Consider the same pipeline for the Rhai chapters.

## Decisions (running log)

**The cut follows product behaviour.** An increment ends when a policy author can do one more useful thing and main still works. The order has eleven steps:

1. The toolchain.
2. A deterministic Rhai policy.
3. Local capabilities.
4. LLM calls.
5. Harness runs.
6. The example policies.
7. The first half of the book.
8. The removal of TypeScript.
9. The second half of the book.
10. The release and the upgrade.
11. The local conversion.

The release and the upgrade are separate leaves. A new schema and its only reader stay in one increment.

**The TypeScript path stays until one increment removes it.** The cutover rule of `plan-k1` and the spec say that main keeps both paths. The extension of the entry selects the path. The refusal `policy_format_retired` therefore arrives with `typescript-worker-cutover`, and not earlier. The interim refusal of `load_context` in `rhai-policy-worker` is deliberate and temporary.

**The Rig dependency has no leaf of its own.** The spec says that the increment which adds TLS proves the archive build. A dependency leaf alone would link nothing, so the proof would be empty. `rhai-llm-anthropic` carries the proof as its first step and stops for the owner if a target fails.

**The equivalence results freeze first.** `rhai-equivalence-fixtures` runs before any removal, because the TypeScript side cannot produce the results afterwards.

**The book changes in two nodes.** The reader checkpoints of the contract already split the book into Parts I and II, and Parts III and IV. Parts I and II describe the owner contract, and the removal does not change it. Parts III and IV describe the worker, the installation and the release, and the removal changes all three. A single pipeline after the removal would leave the executable book checks failing between the two steps. A single pipeline before it would describe code that does not exist yet.

Each node has two draft leaves because chapters 4 and 6 are long. The draft stages cut copy-edit, art and proof, as the editorial procedure says.

**The CLI help and the README move with the removal.** A separate documentation leaf would describe a state that main does not have. The cutover leaf changes the help text with the behaviour, and the reconciliation leaf changes the README.

**The release leaves carry three consequences.** The upgrade keeps Ravel 1.1.0 and `policy.ts` until the driver restarts, as `plan-k1` says. Ravel 1.1.0 refuses an unknown key in the shared `settings.json`, so the conversion adds no new key until the restart. This session found that in `crates/ravel/src/settings.rs`.

`task release:major` refuses outside the default colocated jj workspace, and the work lives in a secondary workspace. The release leaf asks the owner how the work lands on main. The release task also runs `brew upgrade`, so the release leaf sets `HOMEBREW_NO_INSTALL_CLEANUP`. The upgrade leaf then checks the result.

**One review follows.** The cut is a decomposition that 28 leaves build on. A fresh `review-planning` leaf reads it before the first increment runs. It stands at the front of the new entries, so it runs first. The review cuts the integration only if it has findings. This session had no provider evidence in its environment, so the review has no `**Creator:**` line.

**The first draft leaf of each book node needs the owner.** The contract needs a change before the Rhai chapters, and a change to the contract is made with the owner. The leaf body says so.

**Verification.** The controlled-English checker reports no hard finding in each file that this session wrote. No code ran, because planning writes no code. This session read the spec, the design review and its integration, the glossary, the code layout, the Taskfile, the release scripts and the book contract. It did not run any build or test.
