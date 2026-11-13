# Rhai book component inventory: k128

All ten internal component obligations in `docs/book/contract.md` have
explanatory coverage in chapters 7–9. Source comparison found no teaching gap
that required a book correction. No runtime code changed.

`inventory.json` records the obligations, implementation locations, teaching
locations, assessment method and limits. It also identifies six responsibilities
across the two Rust packages: CLI coordination, the separate Rhai worker, the
CLI's host bridge, skill preparation, the runner and result transport.
The worker uses the same executable as the CLI. A package boundary does not
imply a process boundary.

| Obligation | Main teaching location |
|---|---|
| Caller semantics and owner script | Chapter 7, “The program contract stays loose” |
| Evaluation authority and worker reap before launch | Chapters 7–8, worker and host-bridge sections |
| Validation without fallback | Chapter 8, selection and refusal paths |
| Direct launch in caller context | Chapter 8, “Prepare the managed launch” |
| First complete, unchanged result | Chapters 7–8, result-channel and publication sections |
| Reap and confirmed group cleanup | Chapters 8–9, finish and pre-reap group-stop sections |
| Cancellation suppresses results | Chapters 8–9, cancellation and final-outcome sections |
| Terminal foreground and modes | Chapter 9, “Lend and recover the terminal” |
| Exact result bytes on stdout | Chapter 8, “Complete cleanup and write the result” |
| One installed executable and worker identity | Chapter 7, “The installed executable supplies both roles” |

## Method and scope

The assessment followed the current CLI's request through owner-entry resolution,
worker evaluation and teardown, program lookup, direct launch, publication,
supervision, terminal recovery and final settlement. It compared production
operations with explanatory prose, including adjacent failure paths.
Chapter 8 remains an orientation. This inventory does not claim complete source
reconstruction.

Tier 2 graph discovery used project
`Users-antony-Development-AgenticOrchestration-Ravel.gh-issue-2`.
File queries enumerated 62 CLI source files and 11 runner source files without
further pages. `source-scope.json` classifies every returned file as direct
evidence, supporting production source or private test/fixture material.
Enumeration is a scope check, not a claim that every source byte is explained.
Tests, models, dependencies and generated files do not silently become production
coverage.

Both-direction call traces and targeted source reads checked the boundaries.
Trace names sometimes matched unrelated symbols, so exact implementation reads
settled material claims. `coverage.json` records the generation, exact paths and
both source scopes. It reported no recorded issue. That is a best-effort index
signal, not proof of completeness.

Two boundaries deserve particular care. `SessionChannel::result_after` accepts
an `Ended` from any launch; the CLI pairs its channel with the correct launch.
Failed-spawn and unknown-status terminal recovery require exclusive foreground
authority and no unrelated reuse of the recorded group ID. The book already
states both limits.

## Validation records

All five checks passed: six complete examples in chapters 7–9, 22 source
excerpts across the book, strict STE with zero problems in those three chapters,
zero prose runs over 450 words and the local mdBook build. Six labelled
fragments were intentionally not executable examples. All 372 frozen inputs
remained unchanged, and all 369 inputs in the preceding lifecycle inventory
still match their recorded digests. The reference check resolved 103 source and
teaching locations and rejected both deliberately out-of-range references.

`checks.json` records the commands, times, exit statuses and individual logs for
the chapter 7–9 examples, source excerpts, strict STE, strict prose-break checks
and local book build. `reference-validation.json` records each inventory
reference and the line it resolves to. Line validity supports inspection; it
does not decide whether the explanation is correct.

`subjects.sha256` freezes the repository inputs before those checks.
`subjects-after.sha256` and `digest-validation.json` compare the same inputs
afterwards. The set includes the current Rust sources, manifests, lockfile,
specifications, book, fixtures and checker inputs, plus the inventory and its
scope classification. Reports, logs, graph output and Grove bookkeeping are
excluded from the measured subject set. Checkout-local tool and package files
from the preceding inventory are hashed again. Other external dependency caches
are represented by repository pins and lockfiles rather than recursively hashed.

`reconciliation.json` compares each input in the preceding lifecycle inventory
with current bytes. It preserves every changed path and both digests. The
assessment uses that record to detect drift, without reusing the earlier
runtime results or reader observations as new measurements. Frozen release
evidence remains unchanged.

## Limits and handoff

The examples use fake programs and no controlling terminal. They check the
documented examples, not every component failure branch or PTY recovery case.
This leaf adds no runtime qualification campaign or real-harness check.
Current local documentation checks run on native arm64 macOS. Existing
native-entry measurements cover named arm64 macOS and arm64 Linux environments;
the older Linux measurement used Rust 1.85.0. Native x86-64 Linux remains
unqualified.

The CLI must remain alive for supervision. Descendants that leave the job's
group, hostile same-user tampering and remote service work remain outside the
stated guarantees. Bounded waits can leave cleanup unconfirmed. Directory
identity checks do not make pathname changes atomic. Cancellation and final
output are not one atomic transaction; a failed write can leave a prefix.
The private runner retains process-wide signal changes after return and
supplies no supported embedding contract for a host that continues other work.

Coverage does not establish comprehension by a developer new to Ravel.
The fresh whole-book read, reconciliation of the accumulated inventories,
owner Read 2 and published-rendering checks remain with the final proof task
k130. This record discharges only the component inventory.
