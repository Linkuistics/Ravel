### Session 24 (2026-04-16T09:26:27Z) — Sub-I Obsidian coverage brainstorm complete; all v1 brainstorms done

- **What was attempted:** Execute the Sub-I brainstorm — document which Obsidian features cover which Mnemosyne data surfaces and produce a `.obsidian/` template design for v1. Also propagate Sub-N cross-cutting impacts (DialogueRegistry call pattern, Expert.* event structs) into Sub-E and Sub-M.

- **What worked:**
  - Sub-I brainstorm complete. Design doc produced at `docs/superpowers/specs/2026-04-16-sub-I-obsidian-coverage-design.md` with 10 sections: full data-surface inventory (§2, 15+ surfaces mapped), three required community plugins — Dataview, Templater, Homepage (§3), two Templater templates for knowledge entries and expert declarations (§4), four starter Dataview views in `views/` directory (§5), `.obsidian/` file manifest with git-tracking policy (§6), symlink handling verification requirement (§7), first-open experience flow (§8), cross-sub-project requirements (§9), seven implementation tasks (§10).
  - Key design decisions landed: Obsidian is a pragmatic bridge to Phoenix LiveView, not the destination; no graph view; no CSS snippets; no custom dashboard (plan-catalog.md as homepage); zero shipped CSS. Non-markdown files (`.toml`, `.ex`) are plain-text-only.
  - Sibling plan scaffolded at `LLM_STATE/sub-I-obsidian-coverage/` with 7 tasks.
  - Sub-E ingestion backlog and memory updated with binding constraint: Stage 5 must use `Mnemosyne.Expert.DialogueRegistry` as stateless single-turn calls — one `%ExpertAbsorbCandidate{}` in, one verdict struct back, no multi-turn dialogue. Task 7 renumbered to 9 and a new task 7 inserted to make this explicit.
  - Sub-M observability memory updated: Sub-N's six `Expert.*` event structs (`ConsultationStarted`, `ConsultationCompleted`, `ConflictDetected`, `DialogueStarted`, `DialogueTurnCompleted`, `DialogueExpired`) absorbed into the sealed §4.1 event struct set (via backlog Task 24).
  - Orchestrator memory and backlog updated: Sub-I marked done, brainstorm count updated to 11 of 16, pipeline updated — P3 scope-cut decision is now the immediate next step (all v1-scope brainstorms complete).
  - README.md status section updated to reflect 11 brainstorms complete and Sub-I table entry updated.

- **What this suggests trying next:**
  - P3 v1 scope-cut decision is now fully unblocked — I was the last pending in-scope brainstorm. Run the scope-cut triage to lock v1 vs v1.5/v2 assignments and update memory.md.
  - Sub-E task-list rewrite remains on the critical path for Sub-F Task 0 readiness.
  - Sub-B and Sub-C downstream task-list rewrites (two remaining gate conditions for Sub-F Task 0).
  - Sub-L (Obsidian terminal spike) is independent and small; can interleave.

- **Key learnings:**
  - Reaching 11 of 16 brainstorms with all v1-scope work done is a milestone: the architecture phase is functionally complete. What remains before implementation is administrative consolidation (scope cut, task-list rewrites, readiness gate).
  - The `DialogueRegistry` single-turn constraint on Sub-E Stage 5 was surfaced during Sub-N brainstorm and needed explicit propagation — it's a hard API contract that changes the fan-out collector design in Stage 5.
  - Sub-M's Expert.* event absorption was a clean mechanical update; the sealed struct set is now complete for v1 (pending the scope cut confirming no additions).
