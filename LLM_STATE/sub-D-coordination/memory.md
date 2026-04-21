# Memory — Sub-project D: Daemon Coordination

This plan implements sub-project D of the Ravel orchestrator merge.
D's brainstorm is complete; this plan is the implementation work, not
a design phase. The design is fully specified in the spec referenced
below.

## Primary reference

**`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-D-coordination-design.md`**
is the authoritative design document. Every task in this plan's backlog
derives from the spec's §1–§7.

## Parent plan

The orchestrator-level plan lives at
`{{PROJECT}}/LLM_STATE/ravel-orchestrator/` (currently — will move
to `{{PROJECT}}/ravel/project-root/` after sub-project G's
migration lands). It coordinates this sub-plan with its siblings. The
parent plan's `memory.md` holds cross-sub-project architectural state.
This file holds only sub-project-D-specific implementation state.

## Key architectural anchors

### D-1: System-wide singleton lock, not per-vault
Lock file at `<runtime_dir>/ravel/daemon.lock` via `flock(2)`.
Contains PID + vault path. Not inside the vault — invisible to Obsidian.
Enforces v1 single-daemon-per-machine constraint.

### D-2: Content-hash freshness, not mtime
SHA-256 of raw file bytes via `:crypto.hash/2`. Always read-before-write.
Detects actual content changes, not metadata changes.

### D-3: Three file categories with distinct strategies
`:exclusive` (no check, overwrite), `:phase_owned` (daemon-authoritative,
warn on conflict), `:human_editable` (human-authoritative, discard on
conflict). Categories assigned by the caller via `FileIO.write_safe/3`.

### D-4: No autonomous daemon git commits in v1
All git operations are human-mediated. Vault git concurrency is a
non-issue by design constraint.

### D-5: Edge-case safety nets, not high-frequency coordination
External-tool conflicts are rare. The typical workflow has the daemon as
sole writer during phase cycles. Co-equal-actors is a fallback guarantee.

## Implementation state

### Mix project scaffolded at repo root; Elixir 1.19.5-otp-28 / Erlang 28.0
`.tool-versions` pins these versions. Mix project root is the repo root.
`mix format --check-formatted` and `mix test` both pass.

### FreshnessTracker: GenServer owns ETS table `:ravel_freshness`
Table flags: `:named_table, :public, read_concurrency: true`.
Source: `lib/ravel/coordination/freshness_tracker.ex`.

### ETS crash drops all hashes; degradation is safe
On GenServer crash the table is destroyed and all hashes are lost.
Next `check/1` call returns `:untracked`. Callers treat `:untracked` as
"assume changed" — no stale-positive freshness verdicts after crash.

### `check/1` returns four variants, not two
`:untracked | :unchanged | {:changed, hash} | {:error, File.posix()}`.
Spec §3.2 had two variants. `:untracked` distinguishes "never tracked"
without a separate ETS preflight. `{:error, File.posix()}` is consumed
by `FileIO.write_safe/3` per spec §3.3 step 6. The wider type is
tacitly required by the rest of the spec.

## Sibling integration points

- Sub-B: `copy_back/2` → `FileIO.write_safe(path, content, :phase_owned)`
- Sub-E: `SafeFileWriter` → `FileIO.write_safe(path, content, :human_editable)`
- Sub-F: boot step 3 → `SingletonLock.acquire/1`; plan-catalog → `FileIO.write_safe(path, content, :exclusive)`; routing rules → `VaultGit.commit/3`
- Sub-N: expert absorb → `FileIO.write_safe(path, content, :human_editable)`
- Sub-M: subscribes to `%Diagnostic{target: "ravel.coordination"}` and `%Diagnostic{target: "ravel.lock"}` events
