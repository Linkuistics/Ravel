# rhai-measured-reads-k53

**Integrates:** rhai-measured-reads-k52

## Goal

Triage the implementation review and integrate supported corrections to the
Rhai measured-read bridge before later host capabilities consume it.

## Context

- Find the review commit by the stable handle `rhai-measured-reads-k52` and read
  its findings, source evidence, question dispositions and verification limits.
- The reviewed producer is `rhai-measured-reads-k12`, commit
  `05208e3d9dcd7fff7b18e0d60bf6fbde493bc4e3`.
- The contract is `docs/specs/policy-evaluation.md`, with signal ownership and
  finalisation governed by `docs/specs/result-job-completion.md`.
- A root `Taskfile.yml` defines reusable checks. Use Grove 22.6.0 explicitly if
  the PATH shim still selects 18.2.0; this review used
  `/opt/homebrew/Cellar/grove/22.6.0/bin/grove-llm` for tree operations.

## Done when

- Every finding in `rhai-measured-reads-k52` has a reasoned disposition.
- Supported corrections are integrated, their controls discriminate the
  behaviour at issue, and appropriate verification evidence and its limits
  are recorded.
- The code, tests and binding specifications agree at the resulting boundary.

## Notes

The review supplies evidence to triage. Its conclusions are not requirements or
a prescribed fix list. Read the findings from the review's committed artifact.

## Decisions (running log)

Findings were read from the review leaf `rhai-measured-reads-k52` (commit 5f8b54e). Each was reproduced from the source before any change. All four were real and none needed a redesign, so no new review chain was cut.

- **F1 (P1), real issue, fixed.** `Reader::start` now blocks the handled signals through `cancellation::Shield` before it spawns the thread, and restores only the creator, as the diagnostic readers do. A failed block, creation or restore is an `io::Error`, which `front::converse` already maps to a broken-worker refusal before any host call. No join of a stalled reader was added. Controls: a unit test reads the reader thread's own mask, with a plain `thread::spawn` as the firing control, and checks the creator's mask is unchanged. Two more use the existing `native_tests` fault injection (`block-omitted`, `restore-performed`), so `configure` is now `pub(crate)`. Removing the shield fails all three. The spec `result-job-completion.md` (*Checked native-entry transitions*) and `policy-evaluation.md` (*The bridge*) now name this thread.
- **F2 (P2), real issue and a contract stated unclearly, fixed.** The spec said "every read" and also "a read that failed is no measured read". These are two ledgers. Settled: the source *budget* counts a read once its bytes arrived, whether or not they decode (not UTF-8, not JSON, too deep, reply too long). The *measured list* holds only reads the script got content from. A read that finds no file (missing, not regular, unreadable) is no source, and the test pins that choice. `Fetched::Undecoded` carries the new case and `Host::undecoded` counts it. The old data-failure test stays valid, because it asserts the measured list and not the budget. Controls: six rows at the 256 boundary, with and without the caller's document, for invalid JSON and for a binary file. Not counting them fails the test.
- **F3 (P2), real issue, fixed.** Rhai's `set_max_modules` counts import statements, cached ones too, and a block scope resets it. The review's `for` loop example does not reproduce on 1.27.0, but 70 flat statements for one file do. The engine counter is now `usize::MAX`, and the front's count of distinct files is the only bound, as the spec says (the 65th module). The dead `ErrorTooManyModules` wording in `import_failure` is removed. The unit test that pinned `max_modules() == 64` now pins the new rule. Controls: a graph of 8 modules with 7 leaves each (64 files, no file above 8 imports) loads, and one more file refuses with the host's message. Deleting the host check fails it and the old flat 64/65 test. A 70-statement policy for one cached module, at the top level and in `select`, loads once. Putting the engine limit back to 64 fails it. The old flat 64/65 test alone could not tell the two limits apart.
- **F4 (P2), real issue, fixed.** `convert::check_depth` applies the 100-level rule to a parsed file before the reply is built. A deeper file is a catchable `source_invalid_json` that names the first level past the bound, and it counts as a source (F2). The 127- and 128-level files used to break the reply parser, and now they are data failures. Controls: levels 100, 101, 127 and 128, plus a unit test on both container kinds. Removing the check fails the integration test. The worker still converts with the unchecked `from_json`, because the front is the only sender and now guarantees the bound. The spec states the rule under *Measured reads*.
- **Review questions with no finding.** The stop order, the framing, the uncatchable ends and the file rules were accepted as the review reported them. The seam disclaimer in the spec stays correct: the deadline unit test proves the mechanism, not a kernel-held read. The entry as first element of `policy.modules` matches the spec and stays.

## Verification and its limits

- macOS: `cargo nextest run --locked --workspace --no-fail-fast` 490 of 490 (48 skipped by the suite's own filters). `cargo fmt --all --check` and `cargo clippy --locked --workspace --all-targets` are clean. The scripted `task check` pieces that need Bun (worker build, typecheck) did not run.
- Linux arm64, Docker image `ravel-native-linux:arm64`, the `rhai-worker` selection of `measure.sh`: 150 of 151. The one failure is `rhai_policy a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path`, which needs the Bun worker that this ad-hoc run did not build (`worker_missing`). The review recorded the same failure.
- Not run: the full `native-linux` campaign. The campaign observes the two diagnostic readers and not this thread, so the F1 control is the unit test on the thread's own mask. Real signal delivery to an abandoned reader during finalisation was not exercised.
- Mutations run (each fixed behaviour undone, new control fails, file restored): no shield, uncounted decode failures, no host module check, no depth check, engine limit back to 64.

## For adjacent work

- `rhai-data-helpers-k13`: `ravel::json_parse` crosses the same boundary and should call `convert::check_depth` for the 100-level rule, so it needs no new rule.
