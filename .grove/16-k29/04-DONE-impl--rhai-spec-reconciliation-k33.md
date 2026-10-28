# rhai-spec-reconciliation-k33

<!-- tc: off because the inherited task and acceptance criteria are instructions from earlier Grove sessions -->

## Goal

Make the spec set, the ADR set, the glossary and the README describe the one remaining policy path. Remove text that describes TypeScript policies, the Bun worker or the 1.x contract of the worker.

## Context

- The spec `docs/specs/policy-evaluation.md` says that the work which removes the TypeScript path moves the policy sections of `docs/specs/session-execution.md` into it.
- The ADR set is a minimum coherent set. Read `ADR-FORMAT.md` of the grove skill for the rules. Edit, merge or remove a record in place, and chase every citation.
- `CONTEXT.md` defines Owner policy, Policy worker and Host capability for the new design. Check every other term for stale text.
- The README says that a TypeScript policy of the owner chooses the command. Its installation text names `libexec/ravel/notices/`.
- Evidence files under `docs/` record earlier work, such as `docs/runtime-evidence.md` for the Bun runtime. Keep a file that a live document still cites. Remove a file only after you prove that nothing cites it and that it describes nothing that exists.
- Use the skill `linkuistics:simplify-project`.

## Done when

- `docs/specs/session-execution.md` keeps what still describes the session. Its policy text lives in `docs/specs/policy-evaluation.md`, and no rule exists twice.
- The three specs, the five ADRs and `CONTEXT.md` agree with the code. No document names `policy.ts`, Bun or the SDK except to explain the migration or the history.
- The README describes Rhai policies and the new installation layout.
- Every link between these documents resolves.
- A search for the stale terms finds only the places that explain the migration. The search has a positive control and a cross-tree control, as the verification rules require.
- `task check` passes. The prose follows the technical-communication rules.

## Notes

- Parts I and II proof checked `environment::LAUNCH_CONTROLS` against chapters 5 and 6.
  The list has six exact names, not every `RAVEL_*` name.
  Correct the broad scrub claim in the policy-evaluation spec's re-entry section.
  `start-session` refuses an inherited `RAVEL_POLICY_RUN` before evaluation, so the final managed process cannot inherit that marker.
  Each evaluation harness run gets a fresh marker and its own result path.
  Keep these rules separate from the wider reserved classes for policy environment grants.
- The design views in `docs/design/rhai-policy` stay. Check that they still match the spec.
- Do not edit the book. The nodes `rhai-book-use` and `rhai-book-inside` own it.
- The changelog entry for 2.0.0 belongs to the release leaf.
- Draft `rhai-book-use-k88` changed the reader-facing terms in `CONTEXT.md` and the book on 2026-10-28, after owner finding F10.
  “Owner policy” is now “configuration script”, “policy worker” is “Rhai worker” and “policy context” is “selection context”.
  The term map is in the brief of `rhai-book-use-k84`. Bring the specs, ADRs and README to the same terms, or record why one keeps the old term.
  This leaf matches the pre-rename literal names. F18 in `rhai-book-use-k74` now authorises a product rename after this node.
  `configuration-names-k92` owns that coordinated change and the final current spec and ADR filenames.
  The owner has requested the help terminology change. No further permission is needed for that already authorised correction.

<!-- tc: on -->

## Decisions (running log)

- Bootstrap resolved this mandated leaf from its filename and root-to-leaf briefs.
  The graph CLI refused startup because a pre-coordination or unverified generation was active.
  Direct source reads therefore supply the implementation evidence.
- Preserve the executable contract, identifiers, frozen fixtures and book.
  This leaf changes documentation ownership and descriptions only.
  The coordinated product rename remains with `configuration-names-k92`.
- The canonical owners are the configuration evaluation spec, the session spec and the completion spec.
  Configuration discovery, request/context/selection checks, grants, worker protocol and delivery belong to `policy-evaluation.md`.
  `session-execution.md` retains managed launch, result, confinement and delivery-floor promises through links to their canonical rules.
  `result-job-completion.md` retains detailed settlement and native-entry requirements.
- Keep all five ADRs: each states a distinct current decision.
  Keep cited evidence, including the former runtime records, as scoped historical observations.
  No evidence file qualifies for deletion without losing a live citation or provenance.
- The source separates reserved configuration grant classes from six exact launch controls.
  State those rules separately and explain the fresh harness-run marker and result channel.
- The prose baseline found existing hard findings in the session and completion specs and the supervision ADR.
  A preparatory commit converts the affected prose before reconciliation.
  Untouched completion sections retain explicit pending markers under the large-document rule.

## Preservation ledger

| Surface | Consumers | Disposition and destination | Verification |
|---|---|---|---|
| CLI, settings, identifiers, examples and release files | Callers and owners | Preserve executable bytes and literal names | Source reads, unchanged executable diff, `task check` |
| Configuration rules in the session spec | Spec readers and source comments | Fold unique current facts into the Rhai spec and reconcile links | Section inventory, source checks and link check |
| Managed launch, result and confinement | Callers and completion contract | Keep in session spec | Before/after contract comparison |
| Five ADRs and glossary | Maintainers | Keep decisions and adopt configuration vocabulary | Read whole set, check links |
| README and design views | Users and maintainers | Match the current runtime and installed layout | Source comparison, stale-term inventory |
| Cited evidence and frozen fixtures | Provenance and acceptance checks | Keep with existing historical scope | Inbound citation search |

- The baseline `mise exec -- task check` passed all 602 tests, with 48 skipped.
  Its complete log is `target/check.OVlthW`.
- The session spec now delegates configuration and delivery rules to the Rhai spec.
  The Rhai spec explicitly retains caller input bounds, context shapes, selection checks, settings, grants and worker identity/protocol rules.
  These no longer depend on a reader consulting the removed 1.x contract.
- Existing spec anchors remain as explicit HTML anchors where headings adopt the glossary terms.
  The design views retain their paths, show the Rhai worker and use the actual request-map shape for `run_harness`.
- The completion spec distinguishes earlier fixed-revision receipts from current Rhai acceptance checks.
  It keeps native-entry limitations and detailed failure settlement.
  No new executable conformance claim follows from this documentation change.
- The single fresh reviewer reported two actionable contract gaps, and both were valid.
  The equivalence requirement omitted intentional differences and comparison exclusions already required by `rhai_examples.rs`.
  The refusal table also omitted `unsupported_version` at the context stage, as `context.rs` implements.
  The spec now states both distinctions. These are documentation corrections covered by existing executable checks, so no second review is needed.
- The terminology review found an algorithmic handback rule that must not become a configuration script.
  It now says “conditional handback rule”. This applies the existing glossary lesson, rather than introducing a new authoring rule.
- The final bounded documentation inventory covers README, CONTEXT, all specs, all ADRs and the Rhai design views.
  `target/k33-token-inventory.json` enumerates every inline token and prose token from those surfaces.
  The stale-term candidates classify as literal interfaces/citations, retired-format discovery and migration, rejected alternatives, historical observations, or retained book tooling.
  No candidate describes an active TypeScript evaluator, separate installed worker or SDK.
  Existing configuration literals and stable citation slugs deliberately remain for k92.
- The stale-reference instrument uses the same `rg -n --hidden --fixed-strings` flags throughout.
  `task probes` is absent from the current documentation scope and present in `docs/runtime-evidence.md`.
  `task notices` supplies the positive control in the current scope.
  A planted stale reference fires, and deliberately absent variants make both controls fail.
  These results distinguish a clean scope from a broken search.
- The local-link check passed every scoped outgoing citation and all relative Markdown citations into the canonical documents.
  Its absent-fragment control failed as required.
  The design manifest resolves every diagram source, including the unchanged state and outcome diagrams.
  `target/k33-links.json` records each checked citation. This check makes no external URL or rendered-diagram claim.
- The final prose check has no hard findings in the maintained documentation scope.
  Its log is `target/k33-prose.log`.
  Six explicit pending regions preserve untouched parts of the large completion contract.
  They do not exclude this leaf's new contract text from the prose check.
- Final `mise exec -- task check` passed formatting, script checks, book-tool integrity, fake harness checks, Rust lint and terminal recovery.
  All 602 workspace tests passed, with 48 skipped.
  Logs: `target/k33-final-check.log` and the full check log `target/check.PU6CRz`.
- The four book tasks passed: `docs:examples`, `docs:excerpts`, `docs:examples:test` and `docs:excerpts:test`.
  They checked 22 complete examples and nine excerpts. The excerpt checker killed all 24 of its mutants.
  `target/k33-book-checks.log` retains the checker controls and their expected failures.
  Fragments remain outside complete-example execution, as the checker reports.
- Final local-link validation passed 101 citations, and all seven design sources resolve.
  The deliberately absent fragment still fails. No rendered-view or external-link result is claimed.
- Every one of the 3,586 tracked subjects outside `.grove/` retained its individual digest throughout final verification.
  `target/k33-subjects-before.json` and `target/k33-subjects-after.json` record that comparison.
  The subjects include code, manifests, recipes, documentation, frozen fixtures and preserved receipts.
  They exclude machine state and dependency caches. No code, book, fixture or delivery recipe changed in this leaf.
- The parent removal criteria now hold across k30, k31, k32 and this reconciliation.
  k110 records successful installation, three-target release smoke and real formula checks.
  Those delivery results remain predecessor evidence, rather than a claim of rerunning them here.
  The final `task check` runs the same workspace nextest command as `task test`.
  The fresh book checks cover the parent node's checker requirements.
- Retire this leaf and close `typescript-removal-k29`.
  Promote current documentation ownership, exact launch-control scope and comparison exclusions to the root brief.
  Keep the next leaf responsible for coordinated configuration identifiers and final spec/ADR filenames.
  Neither producer has a live review leaf, so no Creator receipt is required.
