# Rhai book workflow inventory: k129

The current workflow surface has 56 public tasks and the internal `release`
implementation. All have explanatory destinations in chapters 10–11.
`inventory.json` maps each declaration to its exact Taskfile block, purpose and
book section. It also classifies every one of the 77 files under `scripts/`
as implementation, control, fixture, template or evidence infrastructure.

Source comparison found one teaching gap. Chapter 10 listed `notices:test`
without explaining what its real-tool controls establish. It now explains
dependency selection, separate notices, licence refusals, atomic output,
lockfile refusal and the pinned tool requirement. No runtime code changed.

## Method and scope

Public membership comes from `mise exec -- task --list-all --json`.
`task-list.json` preserves each returned name, description and source location.
The inventory reconciles that list with every two-space task declaration in
`Taskfile.yml`. Task omits internal tasks even with `--list-all`, so the
source reconciliation adds internal `release` explicitly. Its three wrappers
remain separate public entries.

The script roster comes from a complete filesystem enumeration under
`scripts/`, reconciled with `rg --files scripts`. Each file is assigned a
role and owning tasks. Fixtures, templates, shared helpers and Linux probes
are accounted for rather than silently dropped by a chosen filename pattern.
Source spans identify the complete file for those classifications; Taskfile
spans identify individual recipes. Book spans identify explanatory sections,
not merely the task-name navigation table.

The assessment compared source inputs, effects, order, controls, refusals and
recovery with those sections. The inventory includes setup, builds, tests,
handoff checks, book checks and their controls, local installation, archives,
formula testing, notes, preflight, cuts, publication, promotion and same-version
recovery. Script-only details identify tagged book build/publish/verify/live
operations and Linux environment helpers. The book names evidence instruments
and links their records; it does not teach a historical campaign as a supported
workflow.

Tier 2 coverage checks used graph project
`Users-antony-Development-AgenticOrchestration-Ravel.gh-issue-2`.
`coverage.json` preserves the exact-path responses and script scope result,
including generation metadata. The partial example-control script was read
directly in full. Direct reads also covered the reported arithmetic-guard
lines in the two QEMU helpers, the excluded SVG fixture, notices template and
`.cargo/config.toml`. Checkout-local tool files excluded from indexing are
hashed as data. The graph signal is best effort; filesystem enumeration and
source reads establish this bounded inventory.

## Validation records

`checks.json` records commands, times, exit statuses and individual logs for
strict STE and prose-break checks in chapters 10–11, the book's source excerpts
and the local mdBook build. These checks validate the documentation edit;
they do not rerun every workflow being described.

`reference-validation.json` resolves every inventory source and teaching
span, and records deliberately invalid reference controls.
`membership-validation.json` compares public tasks, all declarations, the
script roster and book navigation names item by item. Omitted-task,
omitted-script, duplicate-task and changed-navigation controls must fail.
Range validity and membership do not decide whether prose explains a workflow.

`subjects.sha256` freezes all inputs before the final checks.
`subjects-after.sha256` and `digest-validation.json` compare the same paths
afterwards. The set extends the preceding component inventory with every
script, licence policy, the current inventory, task capture and this explanation.
It retains the preceding Rust, book, fixture, manifest, tool, package and font
inputs. Logs, reports, graph output, Grove bookkeeping and unrelated workspace
files are outside the measured subject set. External dependency caches are
represented by repository pins and lockfiles, not recursively hashed.

`reconciliation.json` records the current digest of every input in the
preceding component inventory and every difference from its recorded value.
This detects drift; it does not reuse the preceding runtime results as new
observations. The changed chapter is intentional. Frozen release evidence is
not rewritten.

## Limits and handoff

No release, remote publication, installation replacement, real harness call or
platform campaign was performed for this inventory. Source inspection accounts
for those workflows without causing their effects. The local documentation
checks run on native arm64 macOS. They add no Linux or real-harness qualification;
native x86_64 Linux remains unqualified.

This record establishes workflow traceability, not comprehension, complete
source reconstruction, an independent editorial review or a whole-book proof.
The fresh whole-book read, accumulated-inventory reconciliation, published
rendering checks and owner Read 2 remain with final proof task k130.
