# Backlog — Mnemosyne Orchestrator
Tasks for merging LLM_CONTEXT functionality into Mnemosyne and building the persistent actor-daemon architecture committed in Session 9 (2026-04-14).

## Priority 1 — Decisions and gates

- **Status:** not_started
- **Dependencies:** all in-scope brainstorms complete ✓ (A, B, C, D, E, F, G, H, I, M, N done; K, L pending but v1.5+)
- **Description:** All in-scope v1 brainstorms are now complete (Sub-I was the last one, finished Session 23). Decide what's actually in v1 vs. deferred to v1.5/v2. Update memory.md with the v1 cut. Adjust dependent implementation plans accordingly.
  Sub-N (domain experts) is **in v1** — F relies on ExpertActor shipping alongside PlanActor. Without experts, Query messages have no interesting targets.
  Sub-O (MoM) and sub-P (team mode) are **v1.5+ and v2+ by design** — not candidates for v1 scope cut.
  Sub-K (Obsidian plugin client) remains v1.5+ per F's TUI-first commitment.
  Sub-I and sub-L are small documentation/spike work that can land wherever convenient.
  **Immediate pipeline after this decision:** sub-E/sub-B/sub-C task-list rewrites → sub-F Task 0 readiness check → Task 1+ implementation.
- **Results:** _pending_

## Priority 2 — Remaining sub-project brainstorms

### Sub-L — Obsidian terminal plugin spike (independent)
- **Status:** not_started
- **Dependencies:** none (prerequisite for K but independent of other sub-projects)
- **Description:** Small investigation. Evaluate existing Obsidian terminal plugins (obsidian-terminal, obsidian-execute-code, others) for PTY control, streamed output capture, clean termination, and integration with external processes. Recommend fork, build-new, or use-existing for K's scope.
  Follow the guivision + OCR evidence pattern for any UI-inside-Obsidian testing.
  Output: design doc (short) at `docs/superpowers/specs/YYYY-MM-DD-sub-L-obsidian-terminal-spike.md`.
- **Results:** _pending_

### Sub-O — Mixture of models (v1.5+)
- **Status:** not_started (v1.5+)
- **Dependencies:** F done; sub-N done; v1 implementation landed (or close)
- **Description:** **Reserved for v1.5+.** F reserved the schema hooks (`[harnesses.*]` daemon config section, `model:` actor field, `[fact_extraction].model` config). Sub-O implements:
  - Multi-adapter harness layer (multiple concurrent adapters in the daemon)
  - Per-actor model selection (routing resolves `model:` to the appropriate adapter + model combo)
  - Local-model adapters (Ollama, llama.cpp, similar)
  - Cost telemetry (tokens consumed per actor, per session, per model)
  - Economic discipline: users can override defaults, but the defaults should be sensible (expensive models for plan actors, cheap/local for fact extraction)
  Not started until v1 is implemented.
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-O-model-mixing-design.md` and sibling plan.
- **Results:** _pending_

### Sub-Q — Vector-store infrastructure (v1.5+)
- **Status:** not_started (v1.5+)
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
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-Q-vector-store-design.md` and sibling plan at `LLM_STATE/sub-Q-vector-store/`. Reserved for v1.5+; do not land in v1.
- **Results:** _pending_

### Sub-R — Knowledge ontology (v1.5+)
- **Status:** not_started (v1.5+)
- **Dependencies:** sub-N done (satisfied). Sub-R does not block sub-N v1 — v1 uses exact-string tag matching behind the `ScopeMatcher` interface, and sub-R's richer resolver drops in behind the same interface.
- **Description:** **Surfaced during sub-N brainstorm Session 16.** When sub-N locked tag-based scope matching for Stage 5, the user observed that tag-vocabulary drift is inevitable (e.g., `rust` vs `rustlang` vs `Rust`) and that the right long-term answer is an ontology — and that this should be handled as a separate research task, not inline with sub-N. Sub-R is that research brainstorm.
  Cross-cutting consumers of the ontology:
  - **Sub-N expert scope matching** — the `ScopeMatcher` becomes ontology-aware (synonyms, hierarchy, polysemy)
  - **Sub-E Stage 2 classification** — tag inference from extracted facts needs a canonical vocabulary
  - **Sub-F fact extraction** — concern keywords in `routing.ex` rules should match a canonical vocabulary
  - **Sub-Q vector-store** — tag-based retrieval filters need vocabulary consistency
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
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-R-knowledge-ontology-design.md` and sibling plan at `LLM_STATE/sub-R-knowledge-ontology/`. Reserved for v1.5+; do not land in v1.
- **Results:** _pending_

### Sub-K — Obsidian plugin client (v1.5+)
- **Status:** not_started (v1.5+)
- **Dependencies:** F done; sub-L complete; v1 daemon implementation stable
- **Description:** Design the Obsidian plugin that consumes F's NDJSON client protocol and provides an Obsidian-integrated UI alternative to the Rust TUI. Command palette, plan-state panel via Dataview, terminal integration for hosting harness sessions, multi-plan dashboards.
  K does not replace the Rust TUI. Both coexist.
  Output: design doc at `docs/superpowers/specs/YYYY-MM-DD-sub-K-obsidian-plugin-design.md` and sibling plan.
- **Results:** _pending_

### Sub-P — Team mode (v2+)
- **Status:** not_started
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
