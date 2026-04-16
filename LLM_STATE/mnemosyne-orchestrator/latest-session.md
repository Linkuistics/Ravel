### Session 22 (2026-04-16T07:03:13Z) — Sub-H brainstorm: fold 7 legacy skills into daemon TUI commands

- **What was attempted:** Execute the Sub-H brainstorm — design how the 7 legacy Claude Code skills fold into attached-client TUI actions under F's actor/dispatch/TUI architecture. Produce a design doc and sibling implementation plan.

- **What worked:**
  - Design doc produced at `docs/superpowers/specs/2026-04-16-sub-H-skills-design.md`. Architecture-first derivation yielded 17 daemon commands from daemon needs rather than mapping legacy skills 1:1. Key decisions: (1) Three legacy concepts disappear — `promote-global` (Tier 2 is expert-owned), `explore-knowledge` (expert query IS knowledge exploration), and the curate/promote distinction (unified into `knowledge.submit`). (2) Uniform protocol: reuse `send` with typed payloads; only `create_plan` and `adopt_project` are genuinely new NDJSON commands. (3) Command palette with user-configurable keybindings, modal which-key support, hot-reloaded from `tui-keybindings.toml`. (4) `mnemosyne cmd` CLI wrapper for external automation via the NDJSON socket. (5) Expert CRUD via filesystem + `$EDITOR`, with `DeclarationLoader` hot-reload as the single registration path. (6) Clean break at G's Phase 1 — no compatibility shim. (7) Local docs server hosted by the daemon; every command carries a description ≤80 chars and a help URL.
  - Sibling plan scaffolded at `LLM_STATE/sub-H-skills/` with 14 tasks: Task 0 readiness gate, 8 daemon-side Elixir tasks, 4 TUI-side Rust tasks, 1 integration/stubs task.
  - Cross-project propagation: Sub-M received Task 26 (`mnemosyne.tui_action` diagnostic contract — confirm `%Diagnostic{}` carries `fields: map()`, attach Handler, add `counter("mnemosyne.tui_action")` to metric catalogue). Sub-M adoption table updated from "queued" to "contract locked". Sub-E memory updated with explicit scoping boundary: curate command belongs to Sub-H, not Sub-E. README updated to reflect ten brainstorms complete and corrected sub-project table.
  - Orchestrator backlog, memory, and pipeline description updated: H marked done; recommended next steps updated to sub-I amendment as the sole remaining P1 blocker.

- **What didn't work:** Nothing — clean brainstorm session. Note: the backlog recorded results as "Session 21" (a numbering error by the work LLM — this is in fact Session 22, since session-log.md already contained 21 entries at session start).

- **What this suggests trying next:**
  - Sub-I amendment (Obsidian as daemon client) is the only remaining Priority 1 blocker; it gates the Sub-I brainstorm.
  - Sub-E, Sub-B, Sub-C task-list rewrites remain on the critical path for Sub-F Task 0.
  - Sub-H's 14-task implementation plan is ready but gated behind Sub-F Task 0.

- **Key learnings:**
  - Architecture-first derivation (what does the daemon need to expose?) produces a cleaner command surface than legacy-skills-first mapping — three legacy concepts vanished entirely because they were redundant with the new architecture.
  - The `mnemosyne cmd` CLI wrapper for socket automation was not in the original scope; it emerged naturally from the protocol design (NDJSON over a Unix socket is trivially scriptable).
  - The local docs server (daemon hosts help pages for all commands) removes the need for a separate docs deployment, keeps help always version-matched to the running daemon, and is accessible from TUI, Obsidian, and external tools alike.
