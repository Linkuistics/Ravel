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
`SingletonLock` and `VaultGit` are independent. `FileIO` is now
unblocked (FreshnessTracker landed in the prior cycle). Tests follow
their respective modules.

**Adoption by siblings:** sub-B (`copy_back/2`), sub-E (`SafeFileWriter`),
sub-F (boot sequence, plan-catalog), and sub-N (expert absorb writes)
all integrate with `FileIO.write_safe/3`. Those adoption tasks live in
their respective sibling backlogs, not here. Sub-D delivers the modules;
siblings consume them.

## Task Backlog

### Task 1 — `Ravel.Coordination.SingletonLock` `[impl]`
- **Status:** not_started
- **Dependencies:** none
- **Description:** Implement the singleton lock module per spec §2.
  `acquire/1` takes a vault path, resolves the system-wide lock directory
  (`$XDG_RUNTIME_DIR` or `System.tmp_dir!/0`), creates the directory if
  absent, opens `daemon.lock`, acquires `flock(LOCK_EX | LOCK_NB)` via
  `:erlang.open_port/2` with `flock -xn`, writes PID + vault path into
  the file. On contention, reads incumbent PID + vault path and returns
  `{:error, {:daemon_running, pid, vault_path}}`. On success, returns
  `{:ok, port}` — the caller holds the port for the daemon's lifetime.
  Emit `%Diagnostic{target: "ravel.lock"}` events per spec §2.6.
  ~40 lines.
- **Results:** _pending_

### Task 2 — SingletonLock tests `[test]`
- **Status:** not_started
- **Dependencies:** Task 1
- **Description:** Per spec §7.1:
  - Two daemons, same vault: second hard-errors with correct PID + path.
  - Stale lock after crash: `kill -9` first daemon, second acquires ok.
  - Graceful shutdown: clean exit, lock released, second starts ok.
  Pure Elixir + filesystem fixtures.
- **Results:** _pending_

### Task 4 — `Ravel.FileIO` `[impl]`
- **Status:** not_started
- **Dependencies:** none (FreshnessTracker landed; dependency satisfied)
- **Description:** Implement the mandatory file I/O layer per spec §3.3.
  `read_tracked/1` reads file + records hash via FreshnessTracker.
  `write_safe/3` implements the full write discipline: category-based
  freshness check, atomic temp-then-rename (same-directory temp file),
  per-category conflict strategies (`:exclusive` skips check,
  `:phase_owned` writes anyway + warn, `:human_editable` discards + info),
  hash update, telemetry emission. Handles file-deleted-externally case
  per spec §3.3 step 6. Consume all four `check/1` variants:
  `:untracked` → fresh write, record afterward; `{:error, :enoent}` →
  `:phase_owned` recreates + warn, `:human_editable` skips + warn.
  ~80 lines.
- **Results:** _pending_

### Task 5 — FreshnessTracker + FileIO tests `[test]`
- **Status:** not_started
- **Dependencies:** Task 4
- **Description:** Per spec §7.2 and §7.3:
  - Phase-owned external modification: write succeeds, warn emitted.
  - Human-editable external modification: write discarded, info emitted.
  - Exclusive external modification: write proceeds, no diagnostic.
  - No external modification: write proceeds, no diagnostic.
  - File deleted externally (phase-owned): recreated, warn emitted.
  - File deleted externally (human-editable): skipped, warn emitted.
  - Atomic write safety: kill mid-write, no partial file at target path.
  Pure Elixir + filesystem fixtures.
- **Results:** _pending_

### Task 6 — `Ravel.VaultGit` `[impl]`
- **Status:** not_started
- **Dependencies:** none
- **Description:** Implement the vault git interface per spec §4.2.
  `commit/3` takes vault path, list of file paths, commit message.
  Runs `git add` + `git commit` via `System.cmd/3`. On `index.lock`
  contention, emits `%Diagnostic{level: :error}` and returns
  `{:error, :index_lock_held}`. No retry. ~30 lines.
- **Results:** _pending_

### Task 7 — VaultGit tests `[test]`
- **Status:** not_started
- **Dependencies:** Task 6
- **Description:** Per spec §7.4:
  - Index.lock contention: hold lock, trigger commit, assert error + no retry.
  - Successful commit: trigger commit, assert in `git log`.
  Pure Elixir + filesystem fixtures (temp git repos).
- **Results:** _pending_
