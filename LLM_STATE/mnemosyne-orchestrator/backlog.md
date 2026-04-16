# Backlog — Mnemosyne Orchestrator
Tasks for merging LLM_CONTEXT functionality into Mnemosyne and building the persistent actor-daemon architecture committed in Session 9 (2026-04-14).

## Priority 1 — Decisions and gates

- **Status:** done
- **Dependencies:** all in-scope brainstorms complete ✓ (A, B, C, D, E, F, G, H, I, M, N done; K, L eliminated)
- **Description:** All in-scope v1 brainstorms are now complete (Sub-I was the last one, finished Session 23). Decide what's actually in v1 vs. deferred to v2. Update memory.md with the v1 cut. Adjust dependent implementation plans accordingly.
- **Results:** V1 scope cut locked. Two version tiers (no v1.5):
  - **V1** (12 sub-projects): A, B, C, D, E, F, G, H, I, M, N, R
  - **V2** (3 sub-projects): O, Q, P
  - **Eliminated** (2): K (Obsidian plugin client), L (Obsidian terminal spike)
  - **Key decisions:**
    - Sub-R (knowledge ontology) elevated to v1 — tag-vocabulary drift compounds from day one; richer resolver drops in behind sub-N's `ScopeMatcher` interface.
    - Model configurability absorbed by Sub-C for v1 — per-actor `model:` field honored, `--model` passed to claude CLI, sensible defaults (Opus for plan phases, Haiku for fact extraction/ingestion/expert absorb). Sub-O (v2) adds multi-adapter + local models.
    - Sub-K and Sub-L eliminated entirely — no sunk cost (no design docs or sibling plans).
  - **Immediate pipeline:** Sub-R brainstorm (can parallel task-list rewrites) → sub-E/sub-B/sub-C task-list rewrites (C also absorbs model configurability) → sub-F Task 0 readiness check → Task 1+ implementation.

## Priority 2 — Remaining sub-project brainstorms

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

### Sub-R — Knowledge ontology (v1)
- **Status:** not_started (v1, brainstorm needed)
- **Dependencies:** sub-N done (satisfied). Sub-R does not block sub-N initial implementation — sub-N ships exact-string tag matching behind the `ScopeMatcher` interface, and sub-R's richer resolver drops in behind the same interface. Elevated to v1 because tag-vocabulary drift compounds from day one.
- **Description:** **Surfaced during sub-N brainstorm Session 16.** When sub-N locked tag-based scope matching for Stage 5, the user observed that tag-vocabulary drift is inevitable (e.g., `rust` vs `rustlang` vs `Rust`) and that the right long-term answer is an ontology — and that this should be handled as a separate research task, not inline with sub-N. Sub-R is that research brainstorm. **Elevated to v1** — drift compounds from day one once experts accumulate knowledge; retrofitting an ontology after vocabulary has already diverged is harder than starting with one.
  Cross-cutting consumers of the ontology:
  - **Sub-N expert scope matching** — the `ScopeMatcher` becomes ontology-aware (synonyms, hierarchy, polysemy)
  - **Sub-E Stage 2 classification** — tag inference from extracted facts needs a canonical vocabulary
  - **Sub-F fact extraction** — concern keywords in `routing.ex` rules should match a canonical vocabulary
  - **Sub-H TUI skills** — displaying tags to the user needs consistent presentation
  Scope for the brainstorm (research-heavy):
  - **Formal ontology vs folksonomy vs hybrid**: what's the right model for a single-developer knowledge system?
  - **Synonym handling**: `rust` ≡ `rustlang` — dictionary-driven? user-editable? auto-inferred?
  - **Hierarchy**: `rust ⊂ systems-programming ⊂ programming` — what does this buy for scope matching and retrieval?
  - **Polysemy**: `async` in Rust vs JavaScript vs Python — how does context disambiguate?
  - **Vocabulary evolution**: how does the ontology grow as new domains appear without breaking existing tags?
  - **Hooks already reserved in v1**:
    - Sub-A's vault layout has a reserved `tag-vocabulary.md` file path (currently unused)
    - Sub-N's `ScopeMatcher` is a behaviour-shaped interface that a richer resolver can drop into
  - **Literature survey**: 40 years of ontology research in AI/KR; what's the minimum viable ontology for Mnemosyne's scale (single user, <10k knowledge entries initially)?
  - **Integration with existing tools**: Obsidian's tag system, Dataview queries — what's preserved, what's superseded?
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-R-knowledge-ontology-design.md` and sibling plan at `LLM_STATE/sub-R-knowledge-ontology/`.
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
