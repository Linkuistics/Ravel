### Session 25 (2026-04-16T10:32:16Z) — V1 scope cut: lock version tiers, update backlog and README

- **What was attempted:** Complete the Priority 1 "decisions and gates" task — determine the final v1 vs. v2 scope, record the decision in backlog.md and memory.md, and update README.md to reflect the locked tiers.

- **What worked:**
  - Priority 1 task status changed from `not_started` to `done` in backlog.md, with full results block documenting the decisions.
  - V1 locked at 12 sub-projects: A, B, C, D, E, F, G, H, I, M, N, R.
  - V2 locked at 3 sub-projects: O, Q, P.
  - K and L eliminated (no sunk cost — no design docs or sibling plans).
  - Sub-R (knowledge ontology) elevated to v1; rationale recorded: tag-vocabulary drift compounds from day one, and sub-A's `tag-vocabulary.md` slot and sub-N's `ScopeMatcher` interface are already hooks waiting for it.
  - Model configurability absorbed by Sub-C for v1 (per-actor `model:` field, `--model` flag on CLI spawn); Sub-O deferred to v2.
  - Immediate pipeline made explicit: Sub-R brainstorm → sub-E/B/C task-list rewrites (C absorbs model configurability) → sub-F Task 0 readiness check → Task 1+ implementation.
  - memory.md Sub-projects section updated: sub-project table now shows two tiers, K/J/L correctly annotated, recommended ordering updated with sub-R as a new parallel-workable design task.
  - README.md updated: "What's shipping and when" section aligned to v1/v2 (no v1.5), sub-R added to v1 capabilities list, MoM referenced as v1.5+ in mixture-of-experts section, status table updated to include sub-R pending and reflect completed brainstorms.

- **What didn't work / what was skipped:**
  - No implementation started; this was a pure scope-decision session.
  - Sub-R brainstorm not started here (correctly deferred — it's next in pipeline).

- **What this suggests trying next:**
  - Run Sub-R brainstorm (knowledge ontology, research-heavy; can parallel sub-B/C/E task-list rewrites).
  - Sub-E task-list rewrite (Rust framing → Elixir/BEAM, post sub-F amendment).
  - Sub-B and sub-C task-list rewrites (gate tasks in their sibling plans; two conditions remaining for sub-F Task 0).
  - Sub-C must absorb model-configurability requirement during its rewrite.

- **Key learnings / discoveries:**
  - The v1.5 tier was eliminated entirely — collapsing to clean v1 and v2 removes an ambiguous intermediate state that would have complicated scheduling.
  - Sub-R's elevation to v1 was the non-obvious scope decision: it was originally surfaced during sub-N as a "maybe v2" item, but the argument that drift compounds from the first knowledge entry is compelling and irreversible.
