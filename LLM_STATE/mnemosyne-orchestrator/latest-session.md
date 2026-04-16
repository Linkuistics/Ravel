### Session 21 (2026-04-16T05:40:46Z) — Sub-H amendment + sub-E/sub-F contract updates

- **What was attempted:** Complete the sub-H amendment (Priority 1) by mapping all 7 legacy Claude Code skills to F-architecture equivalents, compiling cross-project requirements from sub-B/E/M/N, and folding the results into the sub-H brainstorm task description. Simultaneously update sub-E and sub-F planning state to reflect contracts locked by sub-N's completed 29-task plan.

- **What worked:**
  - Sub-H amendment fully resolved: all 7 skills mapped — 3 superseded by existing mechanisms (`begin-work`/`reflect` → `run_phase`; `setup-knowledge` → `adopt-project`), 3 collapse to expert dispatch/query (`curate-global`, `promote-global`, `explore-knowledge`), 1 requires new daemon command (`create-plan`). Cross-project responsibilities distributed across sub-N, sub-M, sub-E, sub-B.
  - Sub-H brainstorm task description (Priority 2) enriched with the full mapping, architectural constraints, cross-project responsibilities, and 6 design questions for the brainstorm to resolve. H's brainstorm is now unblocked.
  - Sub-E Stage 5 design clarified: parallel fan-out (not sequential) added as first key decision; `%ExpertConflict{}` definition tightened (absorb + non-trivial reject = conflict; scope-miss alone ≠ conflict); two distinct timeouts specified (5s retrieval pipeline abort vs 5min per-turn session reject-and-continue). Both backlog and memory updated.
  - Sub-F memory and backlog updated with sub-N's locked contracts: `ExpertRetrieval` behaviour signature (`retrieve/3`), `ActorSupervisor` public API must include `start_actor/2`, `stop_actor/1`, `send_message/2` in addition to the per-type variants, and message/event struct shapes (`%ExpertAbsorbCandidate{}`, verdict structs, `%Expert.*` events).

- **What didn't work:** Nothing — clean amendment session with no obstacles.

- **What this suggests trying next:**
  - Sub-H brainstorm is the next Priority 2 task and is now fully unblocked.
  - Sub-I amendment (Obsidian as daemon client) remains the only outstanding Priority 1 task.
  - Sub-B and sub-C task-list rewrites remain on the critical path for sub-F Task 0 readiness gate.

- **Key learnings:**
  - The skill-to-F-architecture mapping is clean: no skill survives as a harness slash command; the co-equal-actors principle routes everything through the client-socket boundary.
  - Sub-N's completion of its 29-task plan produced several binding contracts for sub-F (ActorSupervisor API shape, ExpertRetrieval signature) and sub-E (parallel fan-out, conflict definition, timeout split) that needed to propagate before sub-F implementation begins.
  - The two timeouts (retrieval pipeline vs per-turn session) are conceptually distinct — collapsing them would silently degrade per-expert timeouts into hard pipeline aborts.
