# rhai-book-cli-inventory-k125

## Goal

Derive the public CLI inventory from current argument definitions, dispatch and
output code. Map every item to the book and correct CLI coverage defects.

## Context

Use `docs/book/contract.md` section 3 and the parent brief. Retain a durable
inventory under `docs/evidence/`, with source and book locations, derivation
method, subject digests and limits. Historical k311 evidence stays frozen.

## Done when

- Every public verb, flag, option and argument is accounted for, including help,
  version, statuses, stdout/stderr/JSON notices and runtime environment reads,
  values set, values passed and values denied.
- Each item maps to explanatory book text; gaps are corrected or explicitly
  assigned to an existing live task with the exact defect.
- Source-derived enumeration is checked against actual public help, with retained
  command evidence and the distinction between inventory and comprehension.
- Relevant checks pass for any changes; evidence names its inputs and limits.

## Notes

Delivered `docs/evidence/rhai-book-cli-inventory-k125/`: source-derived command,
status, stream, notice and runtime-environment inventories mapped to chapter 6;
23 retained help/parser probes; 196 before/after subject digests; passing checks.
Corrected the current root help excerpt and the omitted `skills` wrapper help
and missing-subcommand behaviour. No CLI defect is handed forward.

Do not claim final proof or owner Read 2. Leave the remaining inventories and
whole-book proof live.
