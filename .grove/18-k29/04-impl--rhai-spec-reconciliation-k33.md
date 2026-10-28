# rhai-spec-reconciliation-k33

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
  Literal names stay: `policy.rhai`, `--policy-env`, `policyEnv`, `RAVEL_POLICY_RUN` and the `policy_*` codes.
  The CLI help also says “owner policy”. A change to that text is a product change, so check it with the owner first.
