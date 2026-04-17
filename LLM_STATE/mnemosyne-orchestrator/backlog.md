# Backlog — Mnemosyne Orchestrator
Tasks for merging LLM_CONTEXT functionality into Mnemosyne and building the persistent actor-daemon architecture committed in Session 9 (2026-04-14).

## Priority 1 — Implementation phase coordination

### Verify Sub-C Coordination Dispatches
- **Status:** done (2026-04-17) — re-verification required after Sub-C runs its own triage cycle
- **Dependencies:** none
- **Description:** The critical path monitor (2026-04-16) dispatched coordination to Sub-C's Received section with two instructions: (1) create a formal gate task for the pre-pivot Rust task-list rewrite, (2) create a model-configurability task (per-actor `model:` field → `--model` CLI flag on harness spawn). Sub-C's triage must have processed these dispatches into concrete backlog items.

  **Actions when this task runs (work phase):**
  - Dispatch a subagent to Sub-C's backlog to verify both tasks exist as discrete backlog items.
  - If either task is missing, write a direct coordination dispatch to Sub-C's Received section with explicit task wording.
  - Report status to orchestrator memory.

- **Results (2026-04-17):**
  - Subagent inspected `LLM_STATE/sub-C-adapters/backlog.md` and `memory.md`.
  - **Both dispatches present in Sub-C's `## Received` section** with explicit task wording from the 2026-04-16 critical-path audit (item #1: "Create a formal gate task (Task 0) for the Elixir backlog rewrite"; item #2: model-configurability — `model:` field → `--model` CLI flag).
  - **Neither dispatch has been promoted to a numbered backlog task yet.** Sub-C has not run a triage cycle since the dispatches landed. All 26 enumerated tasks remain in pre-pivot Rust framing (Cargo.toml, `src/harness/`, crossbeam-channel, `nix::killpg`, `std::process::Command`).
  - **No re-dispatch needed:** the dispatches already in Received use the same explicit wording the orchestrator would write. Re-dispatching would create duplicate noise.
  - **Implication for Task 3 (Sub-B + Sub-C Rewrite Monitoring):** Task 3's premise — "gate tasks exist and we can monitor their status" — is not yet true. Task 3 must wait for Sub-C to run its own work→reflect→compact→triage cycle to promote the Received items into numbered tasks. A re-verification of Sub-C is required at that point before Sub-F Task 0 readiness check can fire.
  - **Suggests next:** invoke a Sub-C work cycle (`run-plan.sh sub-C-adapters`) so its triage promotes the Received items. Until then, Stream 1 is blocked on Sub-C triage, not on missing dispatches. Streams 2 (Sub-R literature surveys) and 3 (Sub-E/Sub-N kickoff) remain independently unblocked.

### Dispatch Sub-R Literature Survey Kickoff (Tasks 1–3)
- **Status:** not_started
- **Dependencies:** none (Sub-R Tasks 1–3 are fully unblocked)
- **Description:** Sub-R's three literature survey tasks (Task 1: Datalog foundations, Task 2: ontology/folksonomy, Task 3: LLM ontology interfaces) have no cross-project dependencies and can start immediately. These gate Sub-R's Datalog engine implementation (Tasks 4+).

  **Actions when this task runs (work phase):**
  - Read Sub-R backlog via subagent to confirm Tasks 1–3 are correctly scoped and `not_started`.
  - Dispatch work-phase signals to Sub-R for Tasks 1–3 via its Received section.
  - Update orchestrator memory with Sub-R start status.

### Sub-B + Sub-C Rewrite Monitoring and Sub-F Task 0 Trigger
- **Status:** not_started
- **Dependencies:** Verify Sub-C Coordination Dispatches (gate task existence confirmed)
- **Description:** Sub-B gate task ("Rewrite downstream implementation tasks against Session-12 design doc") and Sub-C's formal gate task (to be verified) are the two preconditions for Sub-F Task 0 readiness check. Sub-D's `DaemonLock` and `FileIO` deliverables also gate Sub-F Tasks 1/4/6–12.

  **Actions when this task runs (work phase):**
  - Dispatch subagents to Sub-B, Sub-C, and Sub-D to check gate task and delivery status.
  - If Sub-B gate **and** Sub-C gate are both done: dispatch Sub-F Task 0 readiness check via Sub-F Received section.
  - If either is not done: record blocking status in orchestrator memory and carry task to next cycle.
  - Report Sub-D progress on `DaemonLock` + `FileIO` as secondary gate for Sub-F Tasks 1/4/6–12.

### Sub-E + Sub-N Independent Stream Kickoff
- **Status:** not_started
- **Dependencies:** none (both streams are independent of Stream 1)
- **Description:** Two independent implementation streams are unblocked: (1) Sub-E — Task 0 gate is `not_started` and independent of Sub-B/C, (2) Sub-N — Tasks 1–15 can start immediately (pure Elixir + fixtures; gated on Sub-F only at Task 16+).

  **Actions when this task runs (work phase):**
  - Dispatch subagent to Sub-E to verify Task 0 gate status and trigger work phase if ready.
  - Dispatch subagent to Sub-N to verify Tasks 1–15 scope and trigger work phase.
  - Update orchestrator memory with both streams' start status.

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
