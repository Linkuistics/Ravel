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

### Task 1 — `Ravel.Coordination.SingletonLock` `[impl]`
- **Status:** done
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
- **Results:**
  Implemented at `lib/ravel/coordination/singleton_lock.ex` (~130 lines
  incl. `@moduledoc` and diagnostics boilerplate; the business logic
  is under the spec's 40-line estimate). Key deviations from the
  literal spec:
  - **Spec assumed `flock(1)` is on macOS's default PATH — it is not.**
    `flock(1)` is util-linux; macOS requires `brew install flock`. The
    module hard-errors at `acquire/2` with `{:error, {:flock_missing, _}}`
    when the binary is absent, and the README Runtime section now lists
    `flock(1)` as a required external tool.
  - **Child command is `cat`, not `sleep infinity`.** `sleep infinity`
    is a GNU-ism (macOS BSD `sleep` rejects it) and does not react to
    stdin-pipe closure. `flock -xn <path> cat` holds the lock via cat
    reading its inherited stdin; when BEAM closes the port's stdin
    pipe on daemon exit, cat sees EOF, exits cleanly, flock exits, lock
    releases. Verified under both graceful `release/1` and the
    `pkill -9 -P <flock_pid>` crash-simulation test.
  - **`:spawn_executable` instead of `:spawn`.** Avoids the intermediate
    shell and its known orphan-child reaping issues with Erlang ports.
  - **Optional `:lock_dir` opt for testability.** The public API is
    `acquire(vault_path, opts \\ [])` and `lock_path(opts \\ [])`. Sub-F
    calls `acquire/1` unchanged; tests pass an isolated `:lock_dir` to
    avoid clashing with a real running daemon or with each other.
  - **`acquire_probe_timeout_ms = 200`**. The timing-based
    acquired-vs-contended distinction (no exit within 200ms → acquired)
    is inherent to this mechanism. flock's fail-fast is microseconds;
    200ms is a generous upper bound.

  Three diagnostic events emitted via `Logger`:
  `ravel.lock.lock_acquired` (info), `ravel.lock.lock_released` (info),
  `ravel.lock.lock_contended` (warn). Sub-M will swap these to
  `:telemetry.execute/3` when it lands.

### Task 2 — SingletonLock tests `[test]`
- **Status:** done
- **Dependencies:** Task 1
- **Description:** Per spec §7.1:
  - Two daemons, same vault: second hard-errors with correct PID + path.
  - Stale lock after crash: `kill -9` first daemon, second acquires ok.
  - Graceful shutdown: clean exit, lock released, second starts ok.
  Pure Elixir + filesystem fixtures.
- **Results:**
  Implemented at `test/ravel/coordination/singleton_lock_test.exs`
  (~190 lines, 11 tests, all passing). Coverage:
  - **Success path:** acquire succeeds; lock-file content is exactly
    `"<pid>\n<vault_path>\n"`; `lock_acquired` diagnostic emitted; lock
    directory created if absent.
  - **Contention path:** second acquire returns
    `{:error, {:daemon_running, incumbent_pid, incumbent_vault}}` with
    correct values; `lock_contended` warn diagnostic emitted.
  - **Graceful shutdown:** after `release/1` + `wait_for_lock_release`,
    a second acquire succeeds; `lock_released` info diagnostic emitted;
    double-release is idempotent (guards against `ArgumentError` from
    `Port.close` on closed port).
  - **Crash recovery:** spec §7.1 "stale lock after kill -9". Get the
    port's OS PID via `Port.info(port, :os_pid)`, SIGKILL the process
    tree via `pkill -9 -P <flock_pid>` + `kill -9 <flock_pid>`, wait
    for kernel release, second acquire succeeds. This is the
    load-bearing test for the `flock(2)` chosen-over-`O_CREAT|O_EXCL`
    design decision (spec Appendix A Q1).
  - **flock-missing path:** clobber PATH, assert
    `{:error, {:flock_missing, _}}`.
  - **`lock_path/1` pure function:** trivial but seals the API contract
    for bootstrap subcommands.

  Helper `wait_for_lock_release/1` polls via `flock -xn ... -c true` at
  25ms intervals up to 1s; needed because `Port.close` is async and
  kernel release is not instantaneous. Every test wraps its test body
  in `capture_log/1` to keep output clean.

### Task 5 — FreshnessTracker + FileIO tests `[test]`
- **Status:** done
- **Dependencies:** none (Task 4 / FileIO complete — fully unblocked)
- **Description:** Per spec §7.2 and §7.3:
  - Phase-owned external modification: write succeeds, warn emitted.
  - Human-editable external modification: write discarded, info emitted.
  - Exclusive external modification: write proceeds, no diagnostic.
  - No external modification: write proceeds, no diagnostic.
  - File deleted externally (phase-owned): recreated, warn emitted.
  - File deleted externally (human-editable): skipped, warn emitted.
  - Atomic write safety: kill mid-write, no partial file at target path.
  Pure Elixir + filesystem fixtures. The smoke-test script from the
  FileIO impl cycle covers all 9 branches and can be lifted nearly
  verbatim into `test/ravel/file_io_test.exs`; expand with the
  atomic-write kill scenario from spec §7.3.
- **Results:**
  Implemented at `test/ravel/file_io_test.exs` (~260 lines, 16 tests,
  all passing). The smoke-test script memory referred to was not
  actually checked in to the repo; tests were written fresh from the
  spec. Coverage:
  - **`read_tracked/1`:** records hash on success; returns
    `{:error, :enoent}` on missing file without recording.
  - **`:exclusive` category:** overwrites regardless of external
    modification; creates parent directories.
  - **`:phase_owned` category:** clean write when unchanged; overwrite
    + warn diagnostic on external modification; recreate + warn on
    external deletion; proceeds when path is untracked.
  - **`:human_editable` category:** clean write when unchanged;
    discard + info diagnostic + `{:conflict, :external_modification}`
    on external modification; skip + warn + conflict on external
    deletion; proceeds when untracked (first-ingestion write). Plus a
    rebaseline-after-conflict test verifying that a follow-up write
    with the now-current baseline proceeds cleanly — this is the
    load-bearing assertion for `rebaseline_hash/1`'s loop-prevention
    role.
  - **Atomic write safety** (spec §7.3): simulates kill-mid-write via
    `File.chmod!(dir, 0o555)` to force `File.write!(tmp_path)` to raise
    `File.Error`. Asserts target file retains prior content and no
    `.ravel.*.tmp.*` stub is left in the directory. A 50-iteration
    write loop further asserts the target is always readable as a
    complete version (never a partial rewrite).

  Type-check warning on `hd/1` fixed by destructuring the comprehension
  result: `[first | _] = versions = for i <- 1..50, do: ...`.

### Task 6 — `Ravel.VaultGit` `[impl]`
- **Status:** done
- **Dependencies:** none
- **Description:** Implement the vault git interface per spec §4.2.
  `commit/3` takes vault path, list of file paths, commit message.
  Runs `git add` + `git commit` via `System.cmd/3`. On `index.lock`
  contention, emits `%Diagnostic{level: :error}` and returns
  `{:error, :index_lock_held}`. No retry. ~30 lines.
- **Results:**
  Implemented at `lib/ravel/vault_git.ex` (~55 lines incl. moduledoc
  and error classifier; business logic well under the spec's 30-line
  estimate). `commit/3` uses a `with` chain over two `run_git/2` calls
  (`git add` then `git commit`). Failure classification:
  - `index.lock` substring in stderr → `{:error, :index_lock_held}`
    with an `ravel.coordination` error-level log.
  - Any other non-zero exit → `{:error, {:git_failed, output}}` with
    a log including the trimmed git output.
  - No retry logic — spec §4.2 is explicit: the user retries via the TUI.
  - No success-path diagnostic — every commit is user-initiated, so
    the human already knows it happened.

### Task 7 — VaultGit tests `[test]`
- **Status:** done
- **Dependencies:** Task 6
- **Description:** Per spec §7.4:
  - Index.lock contention: hold lock, trigger commit, assert error + no retry.
  - Successful commit: trigger commit, assert in `git log`.
  Pure Elixir + filesystem fixtures (temp git repos).
- **Results:**
  Implemented at `test/ravel/vault_git_test.exs` (~95 lines, 4 tests,
  all passing). Each test creates a fresh temp repo via
  `git init --initial-branch=main` + seed commit + `commit.gpgsign=false`
  config (so CI without gpg keys doesn't hang). Coverage:
  - **Success:** file staged and committed; `git log --format=%s -1`
    returns the message; working tree is clean.
  - **Index.lock contention:** pre-create `.git/index.lock`, assert
    `{:error, :index_lock_held}` + "index.lock held" in log + no
    commit added.
  - **Generic git failure:** non-existent path → `{:error, {:git_failed, _}}`.
  - **No retry:** after a contention failure, the lock file is still
    present (no retry attempt).

## Cross-task notes

- **Full suite green:** `mix test` reports 36 tests, 0 failures across
  FreshnessTracker (5), FileIO (16), VaultGit (4), SingletonLock (11).
- **`mix format --check-formatted`** passes on all new files.
- **README updated:** new "External tools" subsection under Runtime
  documents the `flock(1)` and `git` dependencies.
- **No new `.gitignore` entries needed** — existing Mix patterns
  (`/_build/`, `/deps/`, `/cover/`) cover everything.
- **Sub-D is now feature-complete for v1.** All four modules
  (`FreshnessTracker`, `FileIO`, `SingletonLock`, `VaultGit`) are
  implemented with spec-branch test coverage. Next natural step is
  sibling-plan adoption (sub-B, sub-E, sub-F, sub-N) — those live in
  the siblings' backlogs.
