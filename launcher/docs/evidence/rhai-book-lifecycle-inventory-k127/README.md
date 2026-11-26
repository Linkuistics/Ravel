# Rhai book lifecycle inventory: k127

The inventory covers 134 outcomes and limits across the contract's 16 lifecycle classes.
The book covered 133 items, and one required a correction.
Chapter 8 now says that `--exit-dir` overrides `TMPDIR` for the managed result channel.
No runtime code changed.

## Derivation

The primary sources were `docs/book/contract.md:354`, `docs/specs/session-execution.md` and `docs/specs/result-job-completion.md`.
The assessment followed the CLI's session path through selection, worker teardown, launch, publication and final settlement.
It also followed harness runs, cancellation, terminal recovery and failed launches.
The assessment compared each outcome with the current implementation and the book.

`inventory.json` records each rule, its contract class and exact source, specification, reference and teaching locations.
`contractClasses` maps each contract bullet to its inventory rows.
Rows LIFE-128 through LIFE-133 concern evidence limits, so their sources include the contract rather than an unrelated runtime function.
Reference and teaching locations can coincide when the book explains a limit in one place.

The graph supplied Tier 2 discovery for project `Users-antony-Development-AgenticOrchestration-Ravel.gh-issue-2`.
File queries returned all 62 CLI source files and 11 runner source files without further pages.
`source-scope.json` classifies those files.
Supporting modules and test fixtures are not separate lifecycle outcomes.
Symbol searches, call traces and source reads followed the relevant paths.
The assessment checked exact source when trace names produced unrelated matches.

`coverage.json` records the graph generation, exact paths and both source scopes.
It reported no recorded coverage issue for these paths or scopes.
That result is a best-effort index report, not proof that discovery is complete.
`reference-validation.json` retains the resolved line for each row's references.
This mechanical check establishes path and line validity.
The source comparison establishes the mapping.

## Correction and checks

LIFE-134 records the correction at `docs/book/src/08-follow-a-session-through-the-code.md:439`.
The previous wording omitted the directory override.
`crates/ravel/src/run.rs:93` and `docs/specs/session-execution.md:115` establish the choice.

All five affected checks passed on native arm64 macOS with Rust 1.99.1:

| Check | Result |
|---|---|
| Chapter 8 examples | 2 passed. The checker omitted 3 fragments |
| Source excerpts | 22 passed |
| Chapter 8 strict STE | 0 problems |
| Chapter 8 strict breaks | 0 runs over 450 words |
| Book build | Passed with pinned mdBook 0.5.4 |

`checks.json` records commands, times, exit statuses and log paths.
The examples use fake programs.
They do not exercise every inventory row.
The source comparison was not a new runtime campaign.

## Digests and earlier evidence

`subjects.sha256` freezes 369 inputs before the checks.
The set includes current sources, specifications, book files, examples, checker inputs, manifests, pins and this inventory.
It also includes the earlier digest records used by `reconciliation.json`.
The input set excludes reports, logs, graph output, reference-validation output and Grove metadata.
`subjects-after.sha256` and `digest-validation.json` record the comparison after the checks.

`reconciliation.json` compares every earlier subject with the current bytes:

| Earlier record | Unchanged | Changed |
|---|---:|---:|
| CLI inventory k125 | 193 | 3 |
| Configuration inventory k126 | 295 | 1 |
| Gates k123, before | 351 | 3 |
| Gates k123, after | 351 | 3 |
| Frozen book evidence k311 | 273 | 15 |

The reconciliation retains each path and both digests.
It does not reuse runtime measurements or reader observations.
The frozen k311 evidence remains unchanged.

## Limits retained

The current checks measured native arm64 macOS only.
Existing native-entry evidence covers named arm64 macOS and arm64 Linux environments.
The earlier Linux measurement used Rust 1.85.0.
Native x86-64 Linux remains unqualified despite builds, lint checks and QEMU execution.
These facts qualify no other kernel, C library or compiler combination.

Cleanup depends on the CLI staying alive and on descendants remaining in the managed process group.
Same-user tampering is outside the channel's authentication guarantee.
Directory identity checks do not make pathname removal atomic.
Bounded waits can leave cleanup unconfirmed.
Stopping local harness work does not prove that a remote service stopped.
The final signal decision and output write are not atomic, and a failed write can leave a prefix.

The private runner serves one CLI session and is not a supported embedding library.
Real harness flags remain limited to their recorded versions and dates.
The contract retains the 2026-10-16 checks of Claude Code 2.1.301 and codex-cli 0.163.0.
Chapter 4 labels later help checks dated 2026-10-26.
This leaf ran no new real-harness check.

Whether a developer new to Ravel can use the book remains unknown.
Owner stand-in reads do not establish general comprehension.
This inventory does not complete owner Read 2 or the whole-book proof.
Local build success does not establish GitHub rendering or a published copy of this branch.
