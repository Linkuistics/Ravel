# reference-chapter-k94

## Goal

Draft **part B** of chapter 6 in `docs/book/src/06-reference.md` under the
structure brief and prose contract at `docs/book/contract.md`: configuration
inventory 2 and installed paths. Complete the chapter after part A without
restructuring its existing lookup entries.

## Context

- Part A is `reference-chapter-k75`; its decision log records the source
  enumeration, counts, checks and evidence limits. The document node brief
  carries its current handoff. Read the whole chapter before filling it.
- Fill the eight reserved second-level headings: **Policy entry and settings**;
  **Bounds**; **Policy and request**; **Selection and refusal**;
  **Policy context and host operations**; **SDK and module resolution**;
  **Shipped sample**; **Installed layout**. Remove their placeholders and the
  opening statement that the chapter awaits this draft.
- Enumerate from current subjects: settings/authority, all six bounds, the
  policy and request fields, selected/refused shapes, policy-context document
  and nested fields, host operations, SDK exports, embedded imports and module
  resolution, the static example and copied sample, and installed bin/libexec,
  SDK/declaration/example/notice paths. Read the source and specification named
  in the node brief; check installation paths against the install/manifest
  scripts. Derive counts from these sources rather than this task.
- The Selection and refusal section also owns diagnostic-code lookup entries:
  enumerate the full CLI/admission/worker/evaluation/context/selection/runtime
  code surface, with stages, statuses and remedies. Part A already supplies
  the JSON envelope, ten stages and every normal status, not that code list.
  Preserve chapter 4 proof's `message_too_large` context-stage case for an
  oversized loader failure report as well as policy/selection messages.
- Each entry begins with its literal name and supplies local semantics,
  defaults/limits and consequences, then an anchored teaching link to chapters
  3–5. Use tables for parallel fields and labelled blockquotes for asides.
  Report missing teaching or discrepancies; do not quietly treat an earlier
  chapter as technical evidence.
- Retain settings validation before flag replacement; no fallback route;
  the caller/delivered-policy-context distinction; worker grants versus managed
  inheritance; JSON notice timing and conditional `operationReports`; original
  error/cancellation precedence; exact result bytes and signal termination.
- The installed-layout section is the home for art's reusable C4 component
  and UML deployment views. State the source/installation relationships in
  prose from source; art owns the figures themselves.
- No real-agent flags were copied into part A. If bringing them into scope,
  honour their dated-check requirement; fake routes establish argument delivery,
  not actual agent execution or permission behaviour. Chapter 10, native
  x86-64 qualification, human comprehension and remote rendering remain the
  named omissions/unknowns where relevant.

## Done when

- Every configuration inventory and installed-path item has a self-contained
  entry with a teaching destination; the subject enumeration and its counts
  are recorded here. Part A's whole-chapter heading structure remains usable.
- The whole chapter passes the book build, complete-example and figure checks;
  links/anchors resolve. Record outcomes and their actual scope. There is no
  source-excerpt claim to check unless this draft adds one.
- The already queued `reference-chapter-k95` copy-edit leaf is next; grow no
  duplicate copy-edit leaf. Update its body only with specific new handoffs
  that this draft earns. Retire and commit this draft in the ordinary way.

## Notes

This is the planned second drafting part, not an editorial correction rerun.

## Decisions (running log)

- The named structure brief `docs/book/contract.md` meets all three draft
  preconditions: “The reader is a software engineer with no background in
  Ravel's context”; its ordered section plan assigns chapter 6 lookup of
  “any CLI input, setting, bound, request, selection or context shape,
  environment variable or installed path”; Emphasis names successful cleanup,
  authority, fake-agent validation and limits, and Exclusions keeps personal
  configuration, history and evidence campaigns outside the book. Part B
  retains the eight headings already established by part A.
- Use field tables for closed shapes and a diagnostic-code table with stage,
  status, remedy and anchored teaching per row. Existing complete teaching
  examples remain the executable examples; this draft adds no code excerpt or
  real-agent flags. The sample's `careful` choice means exactly effort `high`,
  including where that lowers an `xhigh` static route.
- Tier 2 graph discovery uses the nearest project
  `Users-antony-Development-Ravel.simplify-and-componentize`, ready at coverage
  generation `2026-10-16T06:43:12Z`. The bounded settings and worker function
  searches are fully paged; a broad initial lookup was narrowed before use.
  `choice.evaluate_and_resolve` has nine outbound and one inbound rows at
  depth one, untruncated. Exact source reads, the context-validator snippet
  and literal diagnostic enumeration supply material evidence. Coverage on
  25 named paths and the front/SDK/example scopes reports matching metadata
  and no recorded issue; this is best effort, not completeness proof.
- `reference-chapter-k95` already holds the next copy-edit stage. Do not grow
  a duplicate. The installation relationships are prose in this draft;
  reusable C4 component and UML deployment figures remain art's charter.

## Subject-derived inventory

Enumerated against this checkout on 2026-10-16. Part A's CLI/environment
inventory remains in `reference-chapter-k75`; these are part B's subjects.

| Surface | Enumeration and count | Subject |
|---|---|---|
| Entry/settings | Default entry, explicit entry and canonical admission; fixed settings location with three optional keys: timeoutMs, contextBytes, policyEnv. Unknown-key and whole-file validation are local. | `authority.rs`, `settings.rs`, `choice.rs::prepare`, `inputs.rs` |
| Bounds | Six: selectionMs, contextBytes, sourceBytes, sources, messageBytes, diagnosticsBytes; six corresponding notice names and origins. | `limits.rs`, worker host and message handling |
| Policy | Four fields: schemaVersion, version, select, loadContext. | SDK `Policy`, `policy.rs::Validator::validate`, worker `load` |
| Request | Nine fields: schemaVersion, kind, prompt, cwd, params, taskFile, taskId, context, limits; last three caller-data optionals are taskFile/taskId/context. | SDK `SelectionRequest`, `choice.rs::request`, `inputs.rs` |
| Selection/refusal | Seven Selected fields and four Refused fields, all locally described; selection union and loader refusal path. | SDK, `policy.rs::selected`/`loader_refused`, `choice.rs::deliver` |
| Diagnostic codes | 49 distinct front codes, each with stage/status/remedy/teaching. Enumerated all lowercase underscore string literals in front Rust modules outside unit-test modules; classified four non-codes: clap's prompt_file, private cfg(test) spawn_missing/spawn_other, and remove_dir_all's operation label. All remaining tokens equal the code-table rows. | `crates/ravel/src/*.rs`, excluding *_tests files; constructors and wrapper arguments read in authority/inputs/settings/environment/limits/policy/context/choice/worker/program/run/cancellation/signal_state/main/confinement |
| Policy context | Six caller/loader fields plus delivered measured; two Assessment fields, four SourceRecord fields, four MeasuredSource fields, three ReadSource fields. Eleven executable-looking top-level names receive the data-only explanation; unknown fields still refuse generally. | SDK types, `context.rs` field arrays and validation, worker `assemble`/host ledger |
| Host | Four public members: diagnostic, signal, readText, readJson; last two only available while loading. Each read's two-field wrapper and shared three-field source object are described. | SDK SelectHost/ContextHost and `worker/src/host.ts` |
| SDK | 19 exports: 16 types/interfaces, two functions and CHOICE_FILE. readChoice covers absence, empty/multiple names, duplicates, offered names, byte/file rules and both helper refusal codes. | Every export declaration in `worker/sdk/index.ts` |
| Imports | Two exact embedded module names; file-relative, ordinary package, package aliases, no-file imports and explicit autoload controls. | `worker/src/main.ts`, `scripts/dispatch.sh` under the crate, session specification and worker ADR |
| Static/sample | Six static exports (two types, agent, routes, selectRoute, policy); five route keys. Copied sample has two choices, one-choice cardinality, exact high override and its own policy version. | `worker/examples/static.ts`, `worker/sample/policy.ts` |
| Installation | Nine installed pair/SDK/example/notice files; archive manifest adds README.md and LICENSE for eleven paths total. Two executables; keyed-launch is linked into the front, not a third process. | `crates/ravel/scripts/dispatch.sh` build/install_pair, `scripts/release-common.sh::archive_manifest`, workspace/package manifests, worker registration |

The literal diagnostic comparison passes, and replacing the message_too_large
row with a wrong code makes the same comparison fail. That token is also
present in chapter 4's teaching and policy validator unit tests. Initial
verification scratch commands named a nonexistent test file and misparsed a
quoted heredoc delimiter; neither result was credited. The corrected checks
use the actual unit-test module and extract only the manifest heredoc body.
SDK/static export names equal their table entries; all eleven manifest paths
occur in the chapter.

No contradictory statement was found in the overlapping chapter 4/5 teaching.
The installation links teach obtaining/keeping the pair and using declarations;
individual licence/notice contents are lookup facts here, not new licence
teaching. Detailed delivery teaching remains chapter 12's planned scope.
No dated real-agent flags were added or rechecked.

## Validation and handoff

- `task docs:book:build`: exit 0, no warnings.
- `task docs:examples`: 13 complete examples passed, 0 failed, 0 skipped;
  six labelled fragments not run, zero page errors. No new example was added.
- `task docs:figures`: seven existing figures match. This draft adds no
  figures; the node's art handoff retains the component/deployment views.
- Every chapter-6 local Markdown link was checked against generated HTML:
  276 occurrences, 31 unique destinations across four other chapters and
  this chapter; every file and fragment exists.
- The 3,442 files enumerated by `rg --files --hidden -g '!.jj/**'
  -g '!.git/**' -g '!.grove/**'`, sorted before SHA-256 hashing, compare
  unchanged before/after all checks. This includes chapter, contract,
  source, scripts and manifests; build/cache output is ignored. Temporary
  receipts are under `target/reference-k94/`. No product source changed.
- Follow-up coverage on manifests, literal-classification sources, teaching,
  specs, ADRs and Taskfile at generation `2026-10-16T06:45:52Z` reports
  matching metadata and no recorded issue. Direct reads remain the evidence;
  graph coverage is not a completeness proof.
- Structure review retained part A's whole-chapter order and completed the
  eight reserved sections with local semantics and anchored teaching. There
  is no source excerpt to validate. No in-session reviewer was used: no
  architectural choice or unexpected uncheckable claim required one. The
  ADR set is unchanged. Copy-edit and art keep their own charters.
- `jj diff --check` is not supported by the installed jj; it supplied no
  validation evidence. Direct whitespace inspection of the changed prose
  passes. The supported book gates above all pass.
- No review leaf names this producer, and copy-edit is already queued as
  `reference-chapter-k95`; no successor is duplicated. That live sibling
  keeps the document node open, so no ancestor closes in this retirement.
- Mechanical coverage does not establish human comprehension, terminal
  interaction, real-agent operation/permissions, native x86-64 qualification
  or remote rendering. Chapter 10 remains deliberately unwritten; the owner's
  stand-in read and future release rendering retain their separate scope.
