# Backlog — Mnemosyne Orchestrator
Tasks for merging LLM_CONTEXT functionality into Mnemosyne and building the persistent actor-daemon architecture committed in Session 9 (2026-04-14).

## Priority 1 — Implementation phase coordination

### Implementation Critical Path Monitor
- **Status:** done
- **Dependencies:** All v1 brainstorms complete (satisfied 2026-04-16)
- **Description:** All twelve v1 brainstorms are done (A, B, C, D, E, F, G, H, I, M, N, R). Implementation phase begins. Orchestrator role: dispatch subagents to monitor critical path and surface blockers.

  **Critical path (ordered):**
  1. **Sub-B and Sub-C task-list rewrites** — pre-pivot Rust framing must be removed (gate tasks already in each sub-project's backlog). These two directly unblock Sub-F Task 0.
  2. **Sub-E task-list rewrite** — same Rust-framing issue; gate task in Sub-E backlog. Independent of B/C.
  3. **Sub-C model-configurability absorption** — per-actor `model:` field → `--model` CLI flag on harness spawn; required for cost control (Opus for plan phases, Haiku for ingestion/extraction).
  4. **Sub-R Tasks 1–3 (literature surveys)** — can run immediately, no cross-project dependencies; Sub-R is fully unblocked.
  5. **Sub-F Task 0 readiness check** — triggers once B+C rewrites land; unblocks Sub-F's 28 implementation tasks (largest block in v1).

  **Actions when this task runs (work phase):**
  - Dispatch subagents to Sub-B, Sub-C, Sub-E, Sub-F, Sub-R to report task-list and gate status.
  - Identify which sub-projects are unblocked vs. waiting.
  - Dispatch coordination signals (via Received section) to sub-projects if ordering adjustments are needed.
  - Verify Sub-R adoption stubs landed in all nine sibling backlogs (N, E, F, H, A, M, I, G, and one of B/C).

  **Output:** updated orchestrator memory reflecting current implementation status, which sub-projects are running, which are blocked, and the next unblocking event.
- **Results:** Completed 2026-04-16. Dispatched six parallel subagents (Sub-B, Sub-C, Sub-E, Sub-F, Sub-R, plus cross-backlog Sub-R stub checker).

  **Findings:**
  - **Sub-B:** Gate task exists ("Rewrite downstream implementation tasks against Session-12 design doc"), `not_started`. 22 impl tasks remaining, 4 done (amendments/spikes). **Unblocked** — can start anytime.
  - **Sub-C:** ⚠️ No formal gate task — rewrite requirement is prose only in backlog header. No model-configurability task exists. 26 tasks all `not_started`. Two coordination dispatches sent to Sub-C Received section.
  - **Sub-E:** Task 0 gate exists, `not_started`. 28 impl tasks (1 `in_progress` = F amendment). **Unblocked** — independent of B/C.
  - **Sub-F:** Task 0 readiness gate exists, `not_started`. **Blocked** on B+C rewrites + Sub-D deliverables. 30+ tasks all `not_started`. Sub-A/Sub-M conditions CLEARED.
  - **Sub-R:** 3 done (design amendments), 18 `not_started`. Tasks 1–3 (literature surveys) **fully unblocked**, can start immediately in parallel.
  - **Sub-R adoption stubs:** Verified complete — all 8 target siblings (F, N, E, A, C, H, M, I) have stubs, plus bonus stub in Sub-B. Sub-G/Sub-D correctly have none.

  **Actions taken:** Dispatched coordination to Sub-C backlog (create formal gate task; create model-configurability task). Updated orchestrator memory with implementation status snapshot.

  **Next unblocking events:** (1) Sub-B gate execution → partially unblocks Sub-F Task 0. (2) Sub-C gate creation + execution → fully unblocks Sub-F Task 0 (with Sub-D). (3) Sub-R Tasks 1–3 → unblocks Sub-R implementation (independent track).

## Priority 2 — v2 sub-project brainstorms (deferred)

### Sub-O — Mixture of models (v2)
- **Status:** not_started (v2)
- **Dependencies:** F done; sub-N done; v1 implementation landed (or close)
- **Description:** **Reserved for v2.** V1 ships per-actor model selection within the single Claude Code adapter (absorbed by Sub-C). Sub-O extends this with:
  - Multi-adapter harness layer (multiple concurrent adapters in the daemon)
  - Local-model adapters (Ollama, llama.cpp, similar)
  - Cost telemetry (tokens consumed per actor, per session, per model)
  - Economic discipline: users can override defaults, but the defaults should be sensible (expensive models for plan actors, cheap/local for fact extraction)
  Not started until v1 is implemented.
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-O-model-mixing-design.md` and sibling plan.
- **Results:** _pending_

### Sub-Q — Vector-store infrastructure (v2)
- **Status:** not_started (v2)
- **Dependencies:** sub-N done (satisfied). Sub-Q does not block sub-N v1 — the `ExpertRetrieval` behaviour chokepoint means sub-Q can land as a drop-in `SemanticRetrieval` strategy without touching sub-N.
- **Description:** **Surfaced during sub-N brainstorm Session 16.** When sub-N locked v1 on keyword+section retrieval, the user signaled that a vector DB as a Mnemosyne-wide capability is appealing and should be treated as cross-cutting infrastructure, not a sub-N-internal strategy. Sub-Q is the brainstorm that investigates how Mnemosyne should provide vector-store infrastructure as a shared daemon service consumable by multiple sub-projects.
  Potential consumers (cross-cutting):
  - **Sub-N expert retrieval** — semantic similarity on knowledge files within an expert's scope
  - **Sub-F vault-catalog search** — "find me plans or experts related to X" at the router level
  - **Sub-E Stage 3 contradiction detection** — current sub-E uses deterministic heuristics; semantic similarity could improve precision
  - **Plan actors searching Tier 1/2 knowledge directly** — beyond expert-mediated access
  - **Sub-O local models** — local embedding models share runtime deps with local inference models, so there's natural overlap with the mixture-of-models sub-project
  Scope for the brainstorm:
  - Embedding model selection: Ollama? llama.cpp? bundled ONNX model? What accuracy/size/CPU trade-offs matter?
  - Storage layout: ETS + flat binary? Lightweight SQLite with an extension? Where does the index live relative to `<vault>/runtime/`?
  - Update policies: on knowledge file write, how does the index stay in sync without blocking writes?
  - Crash recovery: partial embed states on daemon restart
  - Similarity function: cosine vs dot product; approximate vs exact nearest-neighbor
  - Write-time vs read-time embedding: batch nightly reindex vs on-demand?
  - First-time vault setup: embedding N thousand files takes time; what's the UX?
  - Integration boundary: a behaviour module that sub-N (and others) implements against — what does the API look like?
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-Q-vector-store-design.md` and sibling plan at `LLM_STATE/sub-Q-vector-store/`. Reserved for v2; do not land in v1.
- **Results:** _pending_

### Sub-P — Team mode (v2)
- **Status:** not_started (v2)
- **Dependencies:** F done; v1 implementation landed; optionally sub-O
- **Description:** **Reserved for v2+.** F reserved the schema hooks (`[peers]` daemon config section, `<peer>@<qualified-id>` syntax in qualified IDs). Sub-P implements:
  - Network transport for cross-daemon dispatch: BEAM distribution (`Node.connect/1`) or custom TCP
  - Peer discovery: static peer list in config, or mDNS, or DHT
  - Cross-daemon authentication: shared secret, TLS, cookie-based BEAM auth
  - Shared-vault conflict resolution: git-based sync, CRDT-based live sync, or a central vault service
  - Multi-user identity in daemon config
  - Distributed experts: should experts be replicated, partitioned, or centralized?
  - Curation workflow: whose expert accepts a dispatched candidate when multiple users have matching experts?
  A substantial brainstorm at v2 milestone.
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-P-team-mode-design.md` and sibling plan.
- **Results:** _pending_
