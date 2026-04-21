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
`SingletonLock` and `VaultGit` are independent. `FreshnessTracker`
must land before `FileIO`. Tests follow their respective modules.

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

### Task 3 — `Ravel.Coordination.FreshnessTracker` `[impl]`
- **Status:** done
- **Dependencies:** none
- **Description:** Implement the freshness tracker per spec §3.2.
  GenServer owning a `:named_table` + `:public` ETS table keyed by
  absolute path. `record/2` stores `{path, :crypto.hash(:sha256, content)}`.
  `check/1` re-reads file from disk, computes SHA-256, compares against
  stored hash. Returns `:unchanged` or `{:changed, current_hash}`.
  Started early in the supervision tree (before actors). ~50 lines.
- **Results:** Landed at `lib/ravel/coordination/freshness_tracker.ex`
  (~75 lines including module doc + specs). GenServer creates the
  `:ravel_freshness` ETS table in `init/1` with
  `[:named_table, :public, read_concurrency: true]`; the owning
  GenServer's lifecycle bounds the table (crash drops it, which is
  the correct semantics — lost hashes just mean "assume changed on
  next check").

  **Divergence from spec §3.2's two-variant return:** `check/1`'s
  return type was extended to `:unchanged | {:changed, hash} |
  :untracked | {:error, File.posix()}`. The extra variants are
  forced by real-call-site needs: (1) `:untracked` distinguishes
  "never read" (legitimate for first-time writes) from "recorded but
  now differs" — without this, callers would need a second ETS
  preflight; (2) `{:error, :enoent}` is what spec §3.3 step 6 ("file
  deleted externally") needs to pattern-match on inside
  `FileIO.write_safe/3`. Spec §3.2's narrower type was written
  against the happy path; §3.3 tacitly assumes the wider one. Task 4
  should consume all four variants.

  **No `forget/1` or other speculative API** — FileIO's §3.3
  conflict paths all end in an updated hash, so `record/2` covers
  both insert and overwrite.

  **Prerequisite work folded in (not on the backlog):** this was the
  first Elixir implementation on the repo, so the top-level mix
  project had to be scaffolded. Added `mix.exs` (`app: :ravel`,
  `elixir: "~> 1.19"`, no deps yet), `.formatter.exs`,
  `test/test_helper.exs`, and an Elixir/Erlang pin in
  `.tool-versions` (`elixir 1.19.5-otp-28`, `erlang 28.0`).
  `.gitignore` gained the standard Elixir block (`/_build/`,
  `/cover/`, `/deps/`, `/doc/`, `erl_crash.dump`, `*.ez`,
  `ravel-*.tar`). Sub-F's §11.1 scaffolding tasks (`Ravel.Application`,
  supervision tree, `Ravel.VaultDirectory`, actor behaviour) remain
  unbuilt — this is a pure library shell, not a runnable daemon.

  **Verification:** 5 smoke tests in
  `test/ravel/coordination/freshness_tracker_test.exs` cover
  `:untracked`, `:unchanged`, `{:changed, _}`, `{:error, :enoent}`,
  and the "second record replaces prior hash" case. All pass
  (`mix test` → 5 tests, 0 failures). `mix format --check-formatted`
  clean. These are smoke tests for Task 3's own correctness; the
  comprehensive cross-module tests the spec §7.2 specifies live in
  Task 5 and depend on FileIO existing.

  **Suggests next:** Task 4 (`Ravel.FileIO`) is now unblocked. When
  it lands, it consumes all four `check/1` variants and feeds each
  into the category-dispatch logic (`:exclusive` skips the call
  entirely; `:phase_owned` treats `{:changed, _}` and `{:error,
  :enoent}` as "warn + write anyway"; `:human_editable` treats them
  as "discard + info"; `:untracked` is "fresh write, record
  afterward"). Task 5 then exercises the full matrix end-to-end.
  Task 1 (`SingletonLock`) and Task 6 (`VaultGit`) remain fully
  independent and can run in either order.

### Task 4 — `Ravel.FileIO` `[impl]`
- **Status:** not_started
- **Dependencies:** Task 3
- **Description:** Implement the mandatory file I/O layer per spec §3.3.
  `read_tracked/1` reads file + records hash via FreshnessTracker.
  `write_safe/3` implements the full write discipline: category-based
  freshness check, atomic temp-then-rename (same-directory temp file),
  per-category conflict strategies (`:exclusive` skips check,
  `:phase_owned` writes anyway + warn, `:human_editable` discards + info),
  hash update, telemetry emission. Handles file-deleted-externally case
  per spec §3.3 step 6. ~80 lines.
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
