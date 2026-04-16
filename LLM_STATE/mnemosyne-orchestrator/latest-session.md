### Session 23 (2026-04-16T07:34:54Z) — Absorb Sub-I P1 amendment + propagate Sub-H cross-cutting impacts

- **What was attempted:** Complete the last Priority 1 blocker: absorb F's architectural impact into Sub-I's brainstorm scope. Simultaneously propagate Sub-H's cross-cutting findings (17-command palette, `tui-keybindings.toml`, `mnemosyne cmd`, local docs server, atomic cut-over) into sub-A, sub-F, sub-G, and sub-N.

- **What worked:**
  - Sub-I P1 amendment fully absorbed. Backlog task status changed to `done`. Sub-I brainstorm scope expanded substantially: Obsidian reframed as a concrete daemon client (v1 via file observation; v1.5+ via sub-K plugin), and a complete data-surface inventory enumerated — vault-level tracked surfaces (`mnemosyne.toml`, `daemon.toml`, `routing.ex`, `plan-catalog.md`, `experts/*.md`, `knowledge/`, `archive/`, `.obsidian/`), per-project surfaces (visible via symlinks), runtime/ephemeral surfaces (gitignored), and the v1.5+ daemon event stream types. Seven brainstorm questions added. Sub-I is now unblocked.
  - Sub-A backlog and memory updated to include `tui-keybindings.toml` in the vault layout and init flow (hot-reloaded by the TUI; introduced by sub-H Session 21).
  - Sub-F Task 11 expanded: `create_plan` and `adopt_project` commands added explicitly to the daemon command set (both finalised in sub-H); `mnemosyne cmd` headless client documented as a second official NDJSON socket client. New Task 13a added as a placeholder for the local HTTP docs server (per-command help pages; provisionally sub-F's scope; scope must be confirmed before implementation). Memory updated: F-11 retitled to reflect two official clients; sub-H/sub-F relationship section added; scope boundary updated to name both clients as sub-H's implementation scope.
  - Sub-G Task 4 updated with an atomic cut-over constraint: `migrate-project.sh` must execute as a single atomic transaction (no partial-state window); sub-G memory initialised with the sub-H dependency rationale.
  - Sub-N memory updated with N-15: `expert.query` confirmed as the only NDJSON expert command; expert CRUD is filesystem + `$EDITOR` + `DeclarationLoader` hot-reload only; no additional socket command structs needed in sub-N.
  - Orchestrator memory updated: Sub-I status updated to "unblocked", amendments-absorbed list updated to include I, pipeline description updated to remove the amendment step and promote Sub-I brainstorm as the immediate next action.

- **What this suggests trying next:**
  - Sub-I brainstorm is now the immediate priority — P1 is clear. Produce the Obsidian coverage design doc with `.obsidian/` template design.
  - Sub-E, Sub-B, Sub-C task-list rewrites remain on the critical path toward Sub-F Task 0.
  - Task 13a scope confirmation (sub-F vs sub-H allocation for local docs server) should be resolved before sub-F implementation reaches Task 13.

- **Key learnings:**
  - The P1 amendment turned into a much richer scope expansion than the original brief suggested — the complete data-surface inventory (tracked, per-project, runtime, event stream) is itself the most valuable output, giving the sub-I brainstorm a precise checklist rather than an open-ended question.
  - The atomic cut-over requirement for sub-G (sub-H's command palette has no shim and is inoperative before Phase 1 completes) was not previously explicit in sub-G's scope; the triage surfaced it as a hard constraint on `migrate-project.sh` design.
  - Sub-N benefits cleanly from the sub-H confirmation: `expert.query` is the only command, CRUD is filesystem-only, so no new Task structs are needed. The boundary is now fully locked.
