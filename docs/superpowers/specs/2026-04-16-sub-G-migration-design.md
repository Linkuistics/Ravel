# Sub-project G — Migration: LLM_STATE/ + v0.1.0 transition

**Status:** design complete
**Brainstorm date:** 2026-04-16
**Parent plan:** `LLM_STATE/ravel-orchestrator/`
**Sibling implementation plan:** `LLM_STATE/sub-G-migration/`

## §1. Purpose and scope

Design the migration path from the current `LLM_STATE/` + `LLM_CONTEXT/`
layout to `<project>/ravel/` + the BEAM daemon. Two deliverables:

1. **`migrate-project.sh`** — a one-shot shell script that restructures
   a single project's filesystem from `LLM_STATE/` to
   `ravel/project-root/`, handles per-project specifics, and commits
   the result. Runs independently of the daemon. Lives in the Ravel
   repo at `scripts/migrate-project.sh`.

2. **Migration ordering and constraints** — the documented rules for
   when each project migrates and what preconditions must hold.

### In scope

- Per-project filesystem restructure (`LLM_STATE/` →
  `ravel/project-root/`)
- `phase.md` → `plan-state.md` rename (content unchanged)
- `related-plans.md` deletion (vault catalog replaces this concept)
- Ravel v0.1.0 artifact retirement (Rust CLI, legacy plugin, eval,
  defaults, tests)
- APIAnyware-MacOS knowledge directory relocation and docs move
- Migration ordering across five projects
- Cross-sub-project requirements documentation

### Out of scope

- Vault creation (Sub-A's `ravel init`)
- Project adoption into the vault (Sub-A's `ravel adopt-project`)
- `plan-state.md` schema enforcement beyond the rename (daemon's job at
  first boot)
- `run-plan.sh` or LLM_CONTEXT changes (none needed)
- The daemon itself (Sub-F + Sub-B)
- `~/.ravel/` cleanup (v0.1.0 has no real users; hardcoded path only
  existed in the deleted Rust CLI code)
- `.gitignore` updates (no generated files introduced; vault `.gitignore`
  is Sub-A's responsibility)

## §2. Migration ordering

Four phases, each triggered by an explicit precondition.

### Phase 0 — Ravel artifact retirement (immediate)

**Precondition:** none. These are dead artifacts with no dependents.

**Actions:**
- Delete `src/`, `Cargo.toml`, `Cargo.lock`, `adapters/claude-code/`,
  `defaults/`, `eval/`, `tests/`
- Single git commit

This is independent of the `LLM_STATE/` → `ravel/` rename and can
execute at any time.

### Phase 1 — Ravel migrates (daemon switch-over)

**Precondition:** the Ravel daemon's phase runner (Sub-B inside
Sub-F) can drive plans from `ravel/project-root/`.

**Actions:**
- Run `migrate-project.sh` on Ravel
- The daemon takes over plan driving for Ravel
- `LLM_CONTEXT/run-plan.sh` is no longer invoked for Ravel plans
- LLM_CONTEXT itself is untouched — the four dependents still use it

The rename IS the switch-over event. Ravel stops using
`run-plan.sh` and starts using the daemon. No intermediate states, no
symlink bridges, no LLM_CONTEXT changes.

### Phase 2 — Dependent projects migrate (one at a time)

**Precondition per project:** Phase 1 complete. The user decides when
each project is ready. No technical gate beyond Phase 1.

**Actions per project:**
- Run `migrate-project.sh` on the project
- Run `ravel adopt-project` to create the vault symlink
- The daemon now drives this project's plans
- `run-plan.sh` no longer invoked for this project

Order among the four dependents is unconstrained.

### Phase 3 — LLM_CONTEXT retirement

**Precondition:** all five projects are daemon-driven. No project
invokes `run-plan.sh` anymore.

**Actions:**
- LLM_CONTEXT directory can be archived or deleted
- This is a human decision, not a script operation

## §3. Per-project restructure operations

### §3.1 Common operations (all projects)

The script performs these in order for every project:

1. **Create `ravel/`** directory at project root
2. **Create `ravel/knowledge/`** (empty for most projects)
3. **Identify the root plan** — the `LLM_STATE/` directory that becomes
   `project-root/`
4. **Move root plan** → `ravel/project-root/`
5. **Move child plans** — all other `LLM_STATE/` plan directories nest
   under `ravel/project-root/`, preserving their relative structure
6. **Rename `phase.md` → `plan-state.md`** in every plan directory
   (recursive). Content unchanged — still a bare phase string. Schema
   enforcement is the daemon's job at first boot.
7. **Delete `related-plans.md`** in every plan directory (recursive).
   Vault catalog replaces this concept.
8. **Delete `LLM_STATE/`** — must be empty after moves. Hard error if
   not.
9. **Git add + commit** — single commit with specific `git add` paths
   (never `git add -A`)

### §3.2 Per-project specializations

#### Ravel

- **Root plan:** `LLM_STATE/ravel-orchestrator/` →
  `ravel/project-root/`
- **Children:** all 8 `LLM_STATE/sub-*` directories →
  `ravel/project-root/sub-*/`
  - `sub-A-global-store`, `sub-B-phase-cycle`, `sub-C-adapters`,
    `sub-D-coordination`, `sub-E-ingestion`, `sub-F-hierarchy`,
    `sub-M-observability`, `sub-N-domain-experts`
- **Artifact retirement** (Phase 0, executed separately):
  - Delete `src/`, `Cargo.toml`, `Cargo.lock` (Rust v0.1.0 CLI)
  - Delete `adapters/claude-code/` (legacy Claude Code plugin)
  - Delete `defaults/` (v0.1.0 config format)
  - Delete `eval/` (evaluation framework, depends on deleted root crate)
  - Delete `tests/` (Rust integration tests for retired CLI)
- **Preserved:** `docs/`, `spikes/`, `README.md`, `LICENSE`, `TODO.md`

#### APIAnyware-MacOS

- **Root plan:** `LLM_STATE/core/` → `ravel/project-root/`
- **Children:** `LLM_STATE/targets/racket-oo/` →
  `ravel/project-root/targets/racket-oo/`
- **Knowledge:** `knowledge/` (top-level) → `ravel/knowledge/`
- **Docs relocation:**
  - `LLM_STATE/new-language-guide.md` → `docs/`
  - `LLM_STATE/overview.md` → `docs/`
  - `LLM_STATE/project-workflow.md` → `docs/`
  - `LLM_STATE/targets/template.md` → `docs/`
- **Caveat:** repo carries uncommitted WIP on `main`. Script must
  `git add` specific paths only — never `git add -A` or `git add .`.

#### TestAnyware (formerly GUIVisionVMDriver)

- **Root plan:** `LLM_STATE/vision-pipeline/` →
  `ravel/project-root/`
- **Children:** `LLM_STATE/ocr-accuracy/` →
  `ravel/project-root/ocr-accuracy/`
- **Knowledge:** none (create empty `ravel/knowledge/`)
- **Caveat:** same uncommitted-WIP caveat as APIAnyware-MacOS.

#### Modaliser-Racket

- **Root plan:** `LLM_STATE/modaliser/` → `ravel/project-root/`
- **Children:** none
- **Knowledge:** none (create empty `ravel/knowledge/`)
- **Extra:** has a non-standard `plan.md` file in the plan directory —
  preserve as-is.

#### RacketPro

- **Root plan:** `LLM_STATE/racketpro/` → `ravel/project-root/`
- **Children:** none
- **Knowledge:** none (create empty `ravel/knowledge/`)

## §4. Script structure and error handling

### §4.1 Interface

```
./scripts/migrate-project.sh <project-path>
```

The script auto-detects which project it's operating on by matching the
basename of `<project-path>` against the known set: `Ravel`,
`APIAnyware-MacOS`, `TestAnyware`, `Modaliser-Racket`,
`RacketPro`. Unknown project names are a hard error.

### §4.2 Precondition checks (fail-fast)

Before any filesystem mutation, the script validates:

1. `<project-path>` is a git repo (`.git/` exists)
2. `<project-path>/LLM_STATE/` exists
3. `<project-path>/ravel/` does NOT exist (migration hasn't already
   run)
4. Working tree is clean (`git status --porcelain` is empty) — ensures
   the migration commit is isolated and `git revert` is safe
5. The expected root plan directory exists (e.g.,
   `LLM_STATE/ravel-orchestrator/` for Ravel)

Any failure prints a diagnostic and exits non-zero. No partial
mutations.

### §4.3 Execution order

1. Run precondition checks
2. Print the plan (dry-run summary of every move/delete)
3. Create `ravel/` and `ravel/knowledge/`
4. Move root plan to `ravel/project-root/`
5. Move child plans under `ravel/project-root/`
6. Run per-project specializations (knowledge move, doc relocation,
   non-plan file cleanup)
7. Rename `phase.md` → `plan-state.md` in all plan directories
   (recursive find)
8. Delete `related-plans.md` in all plan directories (recursive find)
9. Verify `LLM_STATE/` is empty, then delete it
10. `git add` specific paths + commit

### §4.4 Error model

- `set -euo pipefail` throughout
- Every `mv`, `rm`, `mkdir` is preceded by an existence check
- If any operation fails, the script stops immediately. Since no commit
  has been made yet, the user can `git checkout .` to restore the
  pre-mutation state
- No confirmation prompt — the dry-run summary is printed, then the
  script executes

### §4.5 Git commit

Single commit per project using `git add` on specific paths:

- `git add ravel/`
- `git rm -r LLM_STATE/`
- Per-project removals where applicable (e.g., `git rm -r knowledge/`
  for APIAnyware-MacOS after the move to `ravel/knowledge/`)

Commit message: `migrate: LLM_STATE/ → ravel/project-root/`

No push. The user pushes when ready.

### §4.6 Reversibility

Git-native. Each project gets a single migration commit. Reversal is
`git revert <commit>`. No custom undo logic.

## §5. Ravel artifact retirement (Phase 0)

Phase 0 is independent of the `LLM_STATE/` → `ravel/` rename and
can execute immediately.

### §5.1 Deleted artifacts

| Path | What it is | Why delete |
|---|---|---|
| `src/` | Rust v0.1.0 CLI (commands, config, context, evolution, knowledge, lib, main) | Subsumed by BEAM daemon + Rust TUI split |
| `Cargo.toml` | Root Rust workspace manifest | CLI retired |
| `Cargo.lock` | Root lockfile | Goes with `Cargo.toml` |
| `adapters/claude-code/` | Legacy Claude Code plugin (7 skills, 4 reference docs, `plugin.json`) | Sub-H designs replacements as TUI actions. Git history preserves content. |
| `defaults/` | v0.1.0 config format (`config.yml`, `gitignore`) | Replaced by `ravel.toml` + `daemon.toml` (Sub-A) |
| `eval/` | Evaluation framework (Rust harness + YAML corpus + Python quality) | Rust code depends on deleted root crate. Daemon builds its own test corpus. |
| `tests/` | Rust integration tests + fixtures for v0.1.0 CLI | Tests retired code. Daemon will have ExUnit tests. |

### §5.2 Preserved artifacts

| Path | Why keep |
|---|---|
| `docs/` | Design docs, architecture, research sources. Active reference material. |
| `spikes/` | BEAM PTY spike and other technical spikes. Reference material. |
| `README.md`, `LICENSE`, `TODO.md` | Standard project files. |

## §6. Dependent project details

### §6.1 APIAnyware-MacOS

See §3.2 for the full restructure. Key specifics:

- Only project with an existing `knowledge/` directory (moves to
  `ravel/knowledge/`)
- Has standalone docs at `LLM_STATE/` root that move to project `docs/`
- Has `targets/template.md` (a plan template, not a plan) that moves to
  `docs/`
- **Uncommitted WIP on `main`** — the script must `git add` specific
  paths only. Never `git add -A`, `git add .`, or `git commit -a`.

### §6.2 TestAnyware (formerly GUIVisionVMDriver)

- `vision-pipeline/` is the root plan; `ocr-accuracy/` nests under it
- **Uncommitted WIP on `main`** — same staging caveat as
  APIAnyware-MacOS

### §6.3 Modaliser-Racket

- Single plan, no children
- Has a non-standard `plan.md` in the plan directory — preserved as-is

### §6.4 RacketPro

- Single plan, no children
- Simplest migration — pure standard operations

## §7. What the migration does NOT do

Clear boundary between the script's responsibility and the daemon's:

1. **No `plan-state.md` schema enforcement** — the script renames
   `phase.md` → `plan-state.md` but does not add YAML frontmatter,
   `description:` fields, or `daemon-pid`. The daemon validates and
   enforces schema at first boot.

2. **No vault creation** — Sub-A's `ravel init` creates the vault
   directory with the A4 layout (`ravel.toml`, `daemon.toml`,
   `routing.ex`, `plan-catalog.md`, `experts/`, `runtime/`,
   `.obsidian/`, symlinks). The script doesn't touch the vault.

3. **No `ravel adopt-project`** — vault symlink creation
   (`<vault>/projects/<name>` → `<project>/ravel/`) is a daemon
   operation, run after the script.

4. **No LLM_CONTEXT changes** — `run-plan.sh`, `phases/`,
   `fixed-memory/`, `create-plan.md` are all untouched. Dependent
   projects keep using them until they individually migrate.

5. **No `~/.ravel/` cleanup** — v0.1.0 has no real users. The
   hardcoded path was only referenced in the deleted Rust CLI code.
   Nothing to clean up on the filesystem.

6. **No `.gitignore` updates** — the new `ravel/` directory contains
   only tracked plan files and an empty `knowledge/` directory. No
   generated files, build artifacts, or secrets are introduced. The
   vault's `.gitignore` (for `runtime/`, `cache/`, `projects/`) is
   Sub-A's responsibility at `ravel init` time.

## §8. Cross-sub-project requirements

### §8.1 Sub-A (global store)

`ravel adopt-project` must work against the post-migration
`<project>/ravel/` layout. A's adoption check
(`<project>/ravel/project-root/plan-state.md` exists) is already
aligned with G's output. No design change needed.

### §8.2 Sub-B (phase cycle)

`PhaseRunner` must read `plan-state.md` (not `phase.md`). B's design
already committed to this rename but implementation must respect it.
No design change needed.

### §8.3 Sub-F (hierarchy)

Plan discovery walks `<project>/ravel/project-root/` and its
descendants looking for `plan-state.md`. G's output conforms to this.
Vault catalog generation handles the `project-root/` convention and
nested children at arbitrary depth, as specified in F's design.
No design change needed.

### §8.4 Sub-E (ingestion)

Ingestion reads from `<project>/ravel/knowledge/` (Tier 1). G
ensures this directory exists (even if empty) for every migrated
project. No design change needed.

### §8.5 Sub-N (domain experts)

Expert scope globs reference `<project>/ravel/knowledge/` for
Tier 1. G's layout matches. No design change needed.

### §8.6 Sub-C (harness adapters)

No direct impact. The adapter spawns sessions with paths the daemon
provides; the daemon resolves paths from the post-migration layout.

### §8.7 Sub-M (observability)

No direct impact. M instruments daemon operations, not the one-shot
migration script.

**Summary:** G's output conforms to what A, B, F, E, and N already
expect. No sibling needs a design change — only implementation
awareness that `plan-state.md` is the file name and
`ravel/project-root/` is the root.

## Appendix A — Decision Trail

### Q1. When does the rename happen? (Timing model)

**Decision:** per-project staggering (option D). Ravel migrates
first at daemon switch-over. Dependents migrate later, one at a time,
at the user's discretion.

**Rejected:**
- Late migration (A) — blocks rename until daemon ships, but there's no
  benefit to waiting for all projects at once
- Early migration with LLM_CONTEXT compatibility (B) — unnecessary
  LLM_CONTEXT changes
- Symlink bridge (C) — intermediate state complexity for no gain

### Q2. Ravel's own transition window?

**Decision:** rename waits for daemon readiness (option A). The rename
IS the switch-over event. No intermediate states.

**Rejected:**
- Symlink bridge (B) — adds complexity
- `run-plan.sh` update (C) — changes LLM_CONTEXT unnecessarily

### Q3. Two tools or one?

**Decision:** two separate tools (option C). Shell script for
per-project restructure, `ravel adopt-project` for vault
integration. Different lifecycles, different error models.

**Rejected:**
- Single Mix task (A) — couples restructure to daemon existence
- Standalone shell script for both (B) — vault integration needs daemon
  logic

### Q4. Reversibility?

**Decision:** git-native only (option A). One commit per project,
`git revert` if needed.

**Rejected:**
- Undo script (B) — over-engineering for git-tracked repos
- Dry-run + confirm (C) — the script prints its plan but doesn't pause

### Q5. `plan-state.md` schema at migration time?

**Decision:** phase-only (option C). Rename the file, don't touch
content. Schema enforcement is the daemon's job at first boot.

**Rejected:**
- Minimal with TODO placeholder (A) — still puts schema work in the
  script
- Fully populated (B) — requires inference logic in a shell script

### Q6. `adapters/claude-code/` fate?

**Decision:** delete outright (option A). Sub-H designs replacements.
Git history preserves content.

**Rejected:**
- Archive (B) — sentimental; git history is sufficient
- Leave until Sub-H (C) — dead code lingering

### Q7. Script handles all known cases vs generic?

**Decision:** handles all known cases (option A). Five projects, all
user-owned. No abstraction needed for a one-shot tool.

**Rejected:**
- Standard case only with manual flags (B) — leaves work for the user
- Parameterized with config (C) — over-engineering

### Q8. APIAnyware-MacOS root-level docs?

**Decision:** move to project `docs/` (option B).

**Rejected:**
- `ravel/` root (A) — they're project docs, not plan-system docs
- Leave for manual decision (C) — script can handle it

### Q9. `eval/` fate?

**Decision:** delete entirely (option A). Rust code depends on deleted
root crate. Daemon builds its own test corpus.

**Rejected:**
- Keep corpus/quality only (B) — speculative reuse

### Q10. `tests/` fate?

**Decision:** delete entirely (option A). Tests retired code. Daemon
will have ExUnit tests.

**Rejected:**
- Keep fixtures only (B) — speculative reuse

### Q11. Artifact retirement timing?

**Decision:** immediate (Phase 0). Dead artifacts with no dependents
can be deleted without waiting for the `LLM_STATE/` rename.

### Q12. TestAnyware root plan?

**Decision:** `vision-pipeline/` is the root plan. `ocr-accuracy/`
nests under it as a child.
