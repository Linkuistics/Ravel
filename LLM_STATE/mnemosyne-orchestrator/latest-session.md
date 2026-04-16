### Session 18 (2026-04-16T04:41:21Z) — Sub-G migration brainstorm + Phase 0 artifact retirement

- **What was attempted:** Brainstorm for Sub-G (migration: LLM_STATE/ + v0.1.0 transition). Design the migration path from LLM_STATE/ + LLM_CONTEXT/ to `<project>/mnemosyne/` + the BEAM daemon. Also executed Phase 0 — immediate artifact retirement.

- **What worked:**
  - Full brainstorm completed with 12 design decisions captured in decision trail
  - Design doc written at `docs/superpowers/specs/2026-04-16-sub-G-migration-design.md` (491 lines, 8 sections + appendix)
  - Sibling plan scaffolded at `LLM_STATE/sub-G-migration/` with 8 tasks across 3 phases
  - Phase 0 executed immediately: deleted `src/`, `Cargo.toml`, `Cargo.lock`, `adapters/claude-code/`, `defaults/`, `eval/`, `tests/` — all v0.1.0 artifacts
  - Key discovery: `eval/` is NOT standalone — it depends on the root crate via `mnemosyne = { path = ".." }`, so it was deleted rather than preserved

- **What didn't work:** Nothing — clean brainstorm session

- **What this suggests trying next:**
  - Sub-H amendment and Sub-I amendment are the last pending Priority 1 tasks
  - Sub-G Task 2-3 (write `migrate-project.sh`) can proceed independently of the daemon — the script can be written and tested before the daemon is ready
  - Sub-B/sub-C task-list rewrites remain on the critical path for Sub-F Task 0

- **Key learnings:**
  - eval/'s dependency on the root crate makes it non-standalone — corrects the earlier memory entry that said "eval stays as Rust"
  - Per-project staggering with Mnemosyne-first is the natural ordering: the rename IS the daemon switch-over event, no intermediate states needed
  - The migration script and daemon adoption are cleanly separable: different lifecycles, different error models
  - G imposes no design changes on any sibling sub-project — the post-migration layout conforms to what A, B, F, E, N already expect
