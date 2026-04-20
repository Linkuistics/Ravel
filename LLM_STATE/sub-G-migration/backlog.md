# Backlog — Sub-G Migration

Tasks for migrating five projects from `LLM_STATE/` to `ravel/project-root/`
and retiring Ravel v0.1.0 artifacts. Design doc:
`docs/superpowers/specs/2026-04-16-sub-G-migration-design.md`.

## Phase 0 — Artifact retirement (immediate)

### Task 1: Delete Ravel v0.1.0 artifacts
- **Status:** done
- **Dependencies:** none
- **Description:** Delete `src/`, `Cargo.toml`, `Cargo.lock`,
  `adapters/claude-code/`, `defaults/`, `eval/`, `tests/` from the
  Ravel repo. Single git commit. Per design doc §5.
- **Results:** Deleted all 7 targets plus empty `adapters/` parent directory. Preserved: `docs/`, `spikes/`, `README.md`, `LICENSE`, `TODO.md`, `LLM_STATE/`. `.gitignore` has stale `/target` and `/eval/target` entries — harmless, left in place.

## Phase 1 — Ravel restructure (at daemon switch-over)

### Task 2: Write `scripts/migrate-project.sh` — common operations
- **Status:** not_started
- **Dependencies:** none (can be written before the daemon is ready)
- **Description:** Implement the common operations from §3.1: create
  `ravel/`, `ravel/knowledge/`, move root plan to
  `project-root/`, move child plans, rename `phase.md` →
  `plan-state.md` (recursive), delete `related-plans.md` (recursive),
  verify and delete `LLM_STATE/`, git commit with specific adds.
  Include precondition checks (§4.2) and error model (§4.4).
- **Results:** _pending_

### Task 3: Write `scripts/migrate-project.sh` — per-project specializations
- **Status:** not_started
- **Dependencies:** Task 2
- **Description:** Add the per-project dispatch table and specialization
  functions from §3.2: Ravel (8 sub-* children), APIAnyware-MacOS
  (knowledge move, docs relocation, targets nesting), GUIVisionVMDriver
  (ocr-accuracy nests under vision-pipeline), Modaliser-Racket (preserve
  plan.md), RacketPro (pure standard). Respect the uncommitted-WIP
  caveat for APIAnyware-MacOS and GUIVisionVMDriver (specific git adds
  only).
- **Results:** _pending_

### Task 4: Run migration on Ravel
- **Status:** not_started
- **Dependencies:** Task 2, Task 3, daemon phase runner ready (Sub-B
  inside Sub-F)
- **Description:** Execute `migrate-project.sh` on the Ravel repo.
  Verify: `ravel/project-root/` exists with all 8 sub-* children,
  `ravel/knowledge/` exists, all `plan-state.md` files present,
  no `related-plans.md` files remain, `LLM_STATE/` gone. This is the
  daemon switch-over event per §2 Phase 1.
- **Atomic cut-over constraint (sub-H dependency):** Sub-H's 17-command
  palette becomes active the moment Phase 1 completes and is entirely
  inoperative before it. The legacy skills are dropped with no shim.
  `migrate-project.sh` must therefore be structured as a single atomic
  operation: all moves and renames succeed or nothing is committed. No
  intermediate state (e.g., `LLM_STATE/` still present alongside partial
  `ravel/` structure) should ever be left in place. A single git
  commit that atomically records the new structure and removes the old is
  the required delivery form.
- **Results:** _pending_

## Phase 2 — Dependent project migration (staggered)

### Task 5: Run migration on APIAnyware-MacOS
- **Status:** not_started
- **Dependencies:** Task 4
- **Description:** Execute `migrate-project.sh` on APIAnyware-MacOS.
  Verify: `ravel/project-root/` with `targets/racket-oo/` child,
  `ravel/knowledge/` populated from old `knowledge/`, docs moved
  to `docs/`, no `LLM_STATE/`. Then run `ravel adopt-project`.
  Respect the uncommitted-WIP caveat.
- **Results:** _pending_

### Task 6: Run migration on GUIVisionVMDriver
- **Status:** not_started
- **Dependencies:** Task 4
- **Description:** Execute `migrate-project.sh` on GUIVisionVMDriver.
  Verify: `ravel/project-root/` (from vision-pipeline) with
  `ocr-accuracy/` child, `ravel/knowledge/` empty, no `LLM_STATE/`.
  Then run `ravel adopt-project`. Respect the uncommitted-WIP
  caveat.
- **Results:** _pending_

### Task 7: Run migration on Modaliser-Racket
- **Status:** not_started
- **Dependencies:** Task 4
- **Description:** Execute `migrate-project.sh` on Modaliser-Racket.
  Verify: `ravel/project-root/` (from modaliser), `plan.md`
  preserved, `ravel/knowledge/` empty, no `LLM_STATE/`. Then run
  `ravel adopt-project`.
- **Results:** _pending_

### Task 8: Run migration on RacketPro
- **Status:** not_started
- **Dependencies:** Task 4
- **Description:** Execute `migrate-project.sh` on RacketPro. Verify:
  `ravel/project-root/` (from racketpro), `ravel/knowledge/`
  empty, no `LLM_STATE/`. Then run `ravel adopt-project`.
- **Results:** _pending_
