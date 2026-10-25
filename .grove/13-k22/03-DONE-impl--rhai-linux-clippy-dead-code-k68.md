# rhai-linux-clippy-dead-code-k68

## Goal

Make the `rust-clippy` step of `task test:native-linux` pass on Linux again. Three test helpers in `crates/ravel/src/rhai_worker/heap.rs` have no caller, and clippy runs with `-D warnings` there.

## Context

- `rhai-stop-order-k24` ran `task test:native-linux PLATFORM=arm64` and saw `rust-clippy exited 101`. The `rhai-worker` step passed 240 of 240 tests, so only the lint step fails.
- The errors are `function live is never used`, `function reset_peak is never used` and `function peak is never used`, all `#[cfg(test)]` helpers in `heap.rs`. The same three warnings show in `task check` on macOS, where they do not fail.
- No test in the tree calls them. The likely cause is `rhai-llm-removal-k63`, which removed the tests that measured the heap. Check that with `jj log` or `git log -S`.

## Done when

- Either a test uses the helpers, or the helpers and the statics that only they read are gone. Prefer removal, unless an engine-limit test should measure the peak.
- `task check` has no dead-code warning, and the `rust-clippy` step of `task test:native-linux` passes.

## Notes

- Do not change the heap bound itself.

## Decisions (running log)

- **Removed the helpers instead of testing them.** `git log -S reset_peak` shows `rhai-llm-bounds-k17` added `live`, `reset_peak` and `peak` to measure the heap of the LLM body bounds, and `rhai-llm-removal-k63` (`82766a0`) removed the only tests that called them. No engine-limit test needs a peak, so `live`, `reset_peak`, `peak`, the `PEAK` static and its `fetch_max` in `reserve` are gone from `rhai_worker/heap.rs`. `LIVE` stays, because the bound check reads it. The heap bound is unchanged.
- **Evidence.** `task check` passes with no warning. `task test:native-linux PLATFORM=arm64` ends with `failed=0`, and its `rust-clippy` step passes. Only arm64 ran.
