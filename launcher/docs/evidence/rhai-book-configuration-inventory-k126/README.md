# Configuration inventory, k126

This is a source-derived configuration inventory for Ravel 2.0.0.
It records 236 individually assessed items in [inventory.json](inventory.json).
Each item has a stable ID, rule, exact source locations, reference lookup,
teaching location and result. There are 234 covered items and two corrected items.

The corrections are:

- CFG-223: chapter 4 calls `harness-table.rhai` a repository example.
  Installation supplies only `static.rhai`, `sample.rhai` and `llm.rhai`.
- CFG-200: the encoded host-read reply bound is 38,541,312 bytes.
  `host.rs:764` checks `FRONT_FRAME_BYTES`. The calculation in
  `frame.rs:32–63` selects the larger evaluation frame, rather than the
  19 MiB harness-reply calculation. The specification agrees at
  `docs/specs/configuration-evaluation.md:391–416`.

## Derivation and reconciliation

The review started with `docs/specs/configuration-evaluation.md` and
`docs/specs/skill-discovery.md`, then enumerated the current validators,
Rhai engine setup, namespace registrations, host replies, module resolver,
settings readers and installed example payload. It did not start from the
book's headings. Source fields were compared with the chapter 6 lookup and
chapter 4 explanations. Skill discovery has its own explanatory reference
subsection. The map records that location explicitly.

The review used the graph at verification tier 2, bounded symbol searches,
call traces and exact source reads. The 62-file source catalog is retained
in [source-scope.json](source-scope.json), with each candidate classified.
This catalog bounds source discovery. It is not a claim that every code path
or repository behaviour has been tested.

[coverage.json](coverage.json) records the final graph coverage check.
Its generation is `2026-11-13T15:58:08Z`. The two source scopes and exact
paths had no recorded coverage gap. Four Rhai examples and Cargo.lock had
no graph freshness record. The examples were read directly in full.
Cargo.lock was checked directly for the pinned Rhai dependency.
The graph signal is best effort and does not establish completeness.

No k125 measurement or historical k311 evidence was reused or changed.
The specifications, implementation and book agree on request schema 4.
Selection-context schema 1 and skill schema 1 are separate contracts.

## Item classes

| Class | Items |
|---|---:|
| entry | 5 |
| settings | 8 |
| environment | 4 |
| harness settings | 3 |
| budget settings | 1 |
| skill settings | 11 |
| exports and evaluation | 8 |
| request | 11 |
| request skill descriptor | 13 |
| effective bounds | 9 |
| selected | 8 |
| refused | 5 |
| context | 14 |
| measured reads | 10 |
| host signatures | 14 |
| host rules | 7 |
| JSON conversion | 8 |
| JSON Schema | 10 |
| harness discovery | 6 |
| remaining | 3 |
| harness request | 5 |
| harness result | 13 |
| imports | 9 |
| engine bounds | 12 |
| other fixed bounds | 10 |
| skill fixed bounds | 12 |
| installed examples | 3 |
| repository example | 1 |
| host data error | 3 |
| unavailable error | 2 |
| skill discovery | 8 |

## Frozen subjects and checks

All book corrections and inventory edits precede the recorded checks.
`subjects.sha256` and `subjects-after.sha256` identify the same frozen
measurement subjects. They include the source tree, example scripts,
specifications, book sources, documentation checkers, manifests, lockfile,
tool pins and this inventory. Only this session wrote these subjects.
Evidence output logs, this report and Grove retirement metadata are outputs,
so they are excluded from the subject hash set.

`checks.json` records each command, exit status and output-log path.
The commands use the existing Taskfile through `mise exec -- task`.
The example check targets chapters 4 and 6, the two corrected chapters.
The STE and prose-break checks use strict mode on the same chapters.
The excerpt check and mdBook build inspect the book as their tasks prescribe.
Passing these tools does not establish whole-book editorial proof.

## Limits

This inventory establishes configuration coverage at the recorded input
digests. It is a manual source-to-text comparison, not an independent runtime
test of each item. The item results distinguish existing coverage from fixes.
No runtime implementation changed.

This session does not establish comprehension, owner Read 2, the later
whole-book proof, Linux behaviour, a real subscription-harness run or published
rendering. Later lifecycle, component and workflow inventories retain their
own scope. There was no additional reviewer beside the scheduled proof leaf.
