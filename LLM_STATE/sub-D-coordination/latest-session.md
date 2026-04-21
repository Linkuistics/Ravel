### Session 2 (2026-04-21T04:12:45Z) — implement Ravel.FileIO (Task 4)

- **Attempted:** Implement `Ravel.FileIO` per spec §3.3: `read_tracked/1` and `write_safe/3` with category-based freshness dispatch, atomic writes, and diagnostic emission.
- **What worked:** `lib/ravel/file_io.ex` landed at ~165 lines. All 9 spec branches smoke-tested end-to-end against a real tmpdir. All four diagnostic paths fired at correct levels (`:warn`/`:info`). `mix format --check-formatted` clean; `mix compile` clean; 5 existing FreshnessTracker tests still pass (no regressions). Task 4 status flipped to `done` by the work phase.
- **Key design decisions made:**
  - `atomic_write!/2` places the temp file in `Path.dirname(path)` (not `System.tmp_dir!/0`) to guarantee `rename(2)` atomicity across mounts.
  - Diagnostic emission via `Logger` with structured metadata rather than `:telemetry.execute/3` — `:telemetry` dep not yet in `mix.exs` and sub-M (the consumer) is unbuilt; swapping in when sub-M lands is a ~5-line change.
  - `rebaseline_hash/1` private helper satisfies the spec §3.3 step 2c "update stored hash" requirement on `:human_editable` discard without growing `FreshnessTracker`'s API surface.
  - `{:error, reason}` other than `:enoent` from `FreshnessTracker.check/1` raises `File.Error` immediately (hard-error-by-default philosophy).
  - Backlog preamble updated to remove the stale "FreshnessTracker must land before FileIO" note; Task 3's results block pruned (already landed; context preserved in git history).
- **What this suggests next:** Task 5 (FreshnessTracker + FileIO integration tests) is now fully unblocked. The smoke-test script can be lifted nearly verbatim into `test/ravel/file_io_test.exs` and expanded to cover the atomic-write kill-mid-write scenario from spec §7.3. Tasks 1/2 (SingletonLock) and 6/7 (VaultGit) remain independent and can run in any order.
