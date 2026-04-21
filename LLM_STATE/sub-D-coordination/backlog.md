# Backlog — Sub-project D: Daemon Coordination

Implementation backlog for sub-project D. All tasks derive from §1–§7
of the design doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-D-coordination-design.md`.
Consult the spec before starting any task.

**Sibling plan scaffolded** in Session 18 of the orchestrator plan
(2026-04-16) immediately after the sub-D brainstorm. The spec is
frozen; this plan executes against it.

**Implementation runway:** sub-D has **no gate task**. All four modules
are pure Elixir with no dependency on sub-F's supervisor integration.
All four modules (`FreshnessTracker`, `FileIO`, `SingletonLock`,
`VaultGit`) are now implemented with test coverage. 36 tests pass,
0 failures.

**Adoption by siblings:** sub-B (`copy_back/2`), sub-E (`SafeFileWriter`),
sub-F (boot sequence, plan-catalog), and sub-N (expert absorb writes)
all integrate with `FileIO.write_safe/3`. Those adoption tasks live in
their respective sibling backlogs, not here. Sub-D delivers the modules;
siblings consume them.

## Task Backlog

**Sub-D is feature-complete for v1.** No remaining tasks. All four modules
(`FreshnessTracker`, `FileIO`, `SingletonLock`, `VaultGit`) are implemented
with full spec-branch coverage. Next natural step is sibling-plan adoption —
those tasks live in the siblings' respective backlogs.
