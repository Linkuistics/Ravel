# configuration-names-k113


## Goal
Use configuration names for public diagnostics and JSON output, preserving
behaviour and frozen pre-cutover observations.



## Context
Follows `configuration-names-k112`. Inspect refusals, notices, selection handling,
worker/front/host capabilities, comparison normalization and all output consumers.

## Done when
- Output fields are `config`, `configEnvironment`, `configCode`; Ravel-owned codes
  and locations use `config_` and `config.`. Prefixes are `config stdout:` and
  `config stderr:`. Messages and remedies use configuration terms.
- Failure documents use schema version 2. Routing notices remain 2 and script
  schema remains 3. Reconcile producers, consumers, examples and schema statements.
- Frozen pre-cutover fixtures are unchanged. Explicit tested comparison
  translation accounts for renamed fields/messages without hiding behaviour
  changes; the existing mutation control still fires.
- Text/JSON refusals and notices have CLI coverage. Preserve exact opaque results,
  caller-owned values, cancellation and cleanup behaviour.
- Current consumers and executable examples agree; `task check`, `task test`,
  complete examples, checker controls and excerpts pass. Record platform limits.

## Notes
The next child owns remaining source/document filename and prose reconciliation.
Do not rename arbitrary owner-supplied codes or results. The parent contract is
the completion checklist; use direct source fallback if the graph stays unavailable.

## Decisions (running log)

2026-10-28: The graph CLI cannot start because a pre-coordination or unverified
generation is active. Use direct source reads and literal inventories for this
leaf; no graph completeness claim is available. The existing parent contract
settles the design, including the three schema numbers and no compatibility
aliases. Implement in place in the driver-provided jj workspace.

2026-10-28: First add failing CLI checks for the renamed refusal/notice fields,
text prefixes, locations and unchanged owner data. Then rename the producers and
their consumers, explicitly translate only known historical comparison fields
and messages, and run the full required checks. Frozen fixture bytes stay exact.
The following k114 leaf retains filenames and broader narrative reconciliation.

2026-10-28: Rename at each producer rather than rewriting serialized output:
the refusal formatter emits schema 2 and `configCode`, routing notices keep
schema 2 and emit `config`/`configEnvironment`, and Rhai keeps schema 3.
Ravel-owned refusal codes, loader locations, stream prefixes and diagnostic
wording change at construction. Owner codes, messages, remedies and streams
remain values. CLI tests explicitly carry old tokens through those values.

2026-10-28: Translate frozen observations on the expected side only. The
translation renames the environment-grant and owner-code fields, the stable
refusal code, its exact Ravel-owned message prefix, and the two exact remedies
changed in the shipped static/table examples. It does not recursively replace
strings. The new helper test preserves unknown remedies, caller arguments,
owner codes and diagnostics. A null-refusal regression check prevents mutable
JSON indexing from constructing an object. The original fixture and script
mutation controls both pass. All frozen fixture paths have an empty jj diff.

2026-10-28: Spent the leaf's one in-session reviewer on the bounded translation,
formatter, notice/refusal constructors and CLI tests. The fresh reviewer used
source/diff reads, reported no concrete findings, changed no files and ran no
tests. There are no findings to classify. No second reviewer is needed.

2026-10-29: The first `task check` passed 611 tests (48 skipped), formatting,
shell checks, lint, fake harness smoke checks and terminal recovery. A later
`task test` run alongside example controls hit the existing one-second timeout
fixture before its script printed `selection timeout ready`; its worker never
completed startup. A focused retry while the controls were still active failed
the same way. After those controls finished, the exact case passed alone with
no source change. Run the complete suite again without concurrent campaigns;
retain both failures in the validation account rather than treating them as
successful measurements. No runtime deadline or assertion was weakened.

2026-10-29: Final diagnostic review removed a stale TypeScript `process.exit`
explanation from the worker-ended-without-result message; Rhai exposes no such
operation. It also corrected a wrapped worker label to the glossary's “Rhai
worker”. These are diagnostic text edits only. Run `task check` once more
against this final source; the separate full `task test` already passed 611
tests with 48 skipped after the concurrent checks stopped.

## Validation

2026-10-29, aarch64 macOS, pinned Rust 1.99.1 through `mise exec -- task`:

- Final `task check`: formatting, shellcheck, book Bun integrity, fake harness
  smoke, clippy, terminal recovery and all 611 enabled tests passed; 48 skipped.
- Separate `task test`: 611 passed, 48 skipped. The two earlier startup-timeout
  failures and the successful isolated retry are recorded above. Final `check`
  ran the same full suite after the last diagnostic-text edits and passed.
- Final `task docs:examples`: 22 passed, none failed or skipped; 57 explicitly
  labelled fragments are not executed by this checker.
- `task docs:examples:test` passed; `task docs:examples:mutants` detected all
  30 removed safeguards. The fixture comparison's unchanged effort/refusal
  mutation controls and its changed-fixture controls passed in both full suites.
- Final `task docs:excerpts`: all nine passed. `task docs:excerpts:test` passed
  its controls and detected all 24 removed safeguards.
- `task docs:book:build` passed without a warning. `task smoke` passed direct
  and symlinked installed CLI checks in a fresh temporary prefix, including
  retired-format refusal and exact, empty, multiline and flag-like results.
- SHA-256 inventories of `crates/`, `scripts/`, `docs/`, root Cargo manifests,
  Taskfile, toolchain, README and glossary matched at the check boundaries.
  The inventories and command logs are in ignored `target/k113-*` files.
  Documentation-only edits between campaigns were recorded separately; the
  final inventory stayed unchanged through the final check and example run.
- `jj diff --name-only -- crates/ravel/tests/equivalence/fixtures` is empty:
  no frozen observation was rewritten or regenerated.

No Linux campaign or cross-target release qualification ran in this leaf.
The graph generation remained unavailable, so discovery evidence is direct
source inspection rather than graph coverage. Remaining filenames, private
identifiers and broader prose reconciliation belong to the live k114 sibling;
the parent node does not close here. L63 in `docs/documentation-lessons.md`
records the stale retained-name explanations found during this rename.
