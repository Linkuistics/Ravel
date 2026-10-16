# writing-an-owner-policy-k85

## Goal

Draft part B of chapter 4 in `docs/book/src/04-writing-an-owner-policy.md` under
`docs/book/contract.md` (the named human structure brief and prose/examples
contract). Complete the chapter after part A, retaining its working examples
and teaching progression. This is the planned second drafting session, not a
correction run.

## Context

Read the document-node brief, including its part A handoff and guide-disposition
table, and the entire chapter. Part A is `writing-an-owner-policy-k73`.
Append these reserved headings, in order, without reshaping part A:

1. `Supply policy context`: version-1 data, `loadContext`, host reads and bounds,
   measured sources, source attribution versus authentication, and refusals.
2. `Set bounds and grant environment names`: all settings keys/defaults/ranges,
   CLI replacements/additional grants, unknown keys including obsolete
   `stateDir`, and every never-grantable environment name/pattern. Derive the
   inventories from the settings reader, SDK and session specification.
3. `Resolve policy imports`: literal embedded modules, relative helpers,
   package resolution, enabled package.json autoloading and disabled ambient
   startup/resolver inputs, with the accepted limits.
4. `Route to Claude Code and Codex`: check each program's own help at its
   recorded installed version/date; show actual flags and caller-supplied ending
   instructions, validate with fake `claude`/`codex`, and explain terminal use
   where applicable. No real agent execution or personal policy routes/models
   in the book. Point to agent documentation for installation/accounts and to
   chapter 9 for terminal depth (plain-text forward mention until it exists).
5. `Copy an existing policy`: preserve source and destination edits, copy
   missing files without overwriting, reconcile helpers/imports/choices/bounds/
   grants, remove obsolete storage settings and validate every route/refusal.

The existing installed-declaration recipe uses the checkout's pinned checker
as a separate development tool; normal installed execution needs none. Both
recipes passed all three part A policies and rejected a numeric program.
The sample's `careful` choice sets `high` even where its default is `xhigh`.
Those are checked behaviours to preserve, not suggestions to fix product code.

## Done when

- Part B is taught in the reserved order; the whole chapter builds and passes
  `task docs:examples`, with every complete example under the real-policy,
  fake-program seam and every defined route/refusal exercised.
- Configuration fields, bounds, refusal codes and environment rules are
  enumerated from their subjects and taught; chapter 6's future lookup scope
  is recorded. Complete the node brief's guide-disposition rows, retaining the
  dated personal handoff outside the book.
- Real-agent flag provenance states the actual checked date/version; automated
  examples claim fake execution only. Reader comprehension and unavailable
  renderings/publication remain unknown, not inferred from mechanical checks.
- Copy-edit is already queued immediately after this leaf. Under the editorial
  family rule cut no duplicate successor; retire and commit this leaf normally.

## Decisions (running log)

- Keep the approved five-section order and all of part A. The named contract
  supplies the reader/outcome, ordered plan and emphasis/exclusions; no new
  structure agreement is needed. Add complete context, settings/grants and
  agent-route examples under the existing checker rather than product code.
- On 2026-10-16 AEDT the installed help reports Claude Code 2.1.301 and
  codex-cli 0.163.0 (the latter supersedes the brief's earlier 0.162.1 check).
  Use interactive routes with explicit effort for Claude and explicit
  sandbox/approval/inline/no-daemon options for Codex, leaving model choice to
  each agent's configuration and labelling it honestly. Only fake executables
  run the routes; version/help invocations are the only real-agent commands.
- Enumerate all six bounds and all never-grantable patterns from the readers,
  including HARNESS_DISPATCH_*. Teach source attribution separately from host
  measurement and authentication. Leave the dated personal handoff in its guide
  for the already-planned migration leaf to preserve outside the book.
- The one aimed fresh-context reviewer checked context attribution/measurement
  and import resolution against the implementation. Both findings are valid
  and actionable, mechanical qualifications: reserve the single-key `$ravel`
  object even in nested JSON data, and distinguish unsupported unsigned-integer
  schema versions from invalid negative/fractional versions. Direct reads of
  context.rs confirm both; no new mechanism or second review is needed.
- Initial example validation caught five invented bracket-format diagnostic
  excerpts; replace them with actual stable output (source/location for the
  OS-dependent missing-file error). Source verification also corrected the
  worker-output refusal to `output_limit` and retained its bounded diagnostic
  capture qualification. Runtime behaviour and product source are unchanged.

## Verification

- Structure: the named contract states the software-engineer reader and
  configure/typecheck/validate outcome, the ordered chapter plan, and the
  emphasis/exclusions. The five reserved part B headings follow part A in order;
  comparison with the parent revision confirms part A is unchanged except for
  one scope clarification: its no-runtime-types statement now says "introduced
  so far", since the later settings example uses process.env. Its examples and
  teaching progression are unchanged. Guide sentence groups have destinations in the document-node
  brief; chapter 6's lookup scope is also in the parent book brief so its sibling
  can read it. The dated handoff remains in the unchanged guide pending migration.
- Technical truth: Tier 2 graph discovery, assemble's outbound trace and exact
  snippet, then direct source reads of SDK, host, validators, settings, limits,
  environment, worker startup and build switches. Coverage generation
  2026-10-16T04:22:29Z reported no recorded gaps and matching metadata on those
  source paths. That is best-effort coverage, not proof of graph completeness.
  The inventories are the three settings keys; six bounds; all exclusions in
  environment::excluded; the Context/Assessment/SourceRecord/MeasuredSource
  and host shapes; and context/settings/bound refusal codes in their readers.
  No ADR decision changed: this draft teaches the existing contracts.
- `task docs:examples` passed: 7 complete examples, 0 failed, 0 skipped,
  4 labelled fragments not run, 0 page errors. Six examples belong to chapter 4,
  one to chapter 3. Every defined route and explicit policy refusal is exercised;
  built-in failures are representative cases, not all filesystem conditions.
  `task docs:book:build` passed without a warning. Logs from this local run are
  under ignored `target/k85-typecheck/`.
- SHA-256 digests of 167 enumerated source/configuration/document inputs under
  crates, scripts, docs/book, docs/adr and docs/specs, plus root manifests,
  Taskfile, CONTEXT and owner-configuration, matched before and after the
  successful examples/build run. Subsequent prose clarifications changed
  no extracted example bytes: a fresh extraction matched every example file,
  command, expected stream and status (only source-line locators excluded),
  and the final book build passed again.
- All six extracted chapter policies, including routes.ts, typechecked against
  both generated checkout declarations and target/book-install declarations
  using the pinned checker and strict checkout options. The settings example
  uses process.env and therefore adds the checkout's Bun development types;
  installed execution requires no checker or separately installed runtime.
- Real programs were invoked only for version/help. Automated routes ran fakes.
  Reader comprehension, real-agent permissions/obedience, terminal interaction,
  editor/GitHub rendering and publication remain unknown. This draft did not
  inspect a rendered browser page or claim those checks.

## Handoff

Copy-edit `writing-an-owner-policy-k86` is already the next live sibling, so
no successor is added. The document node remains open for copy-edit, art and
proof; there is no parent close cascade and no review-kind Creator line to set.
