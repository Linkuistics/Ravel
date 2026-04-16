# Memory — Sub-G Migration

## Sub-H dependency (2026-04-16)

Sub-H's 17-command daemon palette is **blocked on Phase 1 completing**. The
legacy skill surface (`begin-work`, `reflect`, `setup-knowledge`, `create-plan`,
`explore-knowledge`, `curate-global`, `promote-global`) is being dropped with
no backward-compatibility shim — sub-H assumes the daemon is live and the legacy
skills are gone. There is no forwarding layer.

**Atomic cut-over is required.** If any partial-state window exists (e.g.,
`LLM_STATE/` still present while the new structure is partially written), sub-H's
command palette is entirely inoperative during that window. Task 4 (migrate
Mnemosyne) must be designed and executed as a single atomic transaction, not a
gradual migration.
