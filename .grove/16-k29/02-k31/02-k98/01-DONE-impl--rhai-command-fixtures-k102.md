# rhai-command-fixtures-k102

## Goal

Port ordinary CLI fixtures to Rhai while preserving the real command boundary and the existing runtime behavior.

## Plan

1. Port `tests/cli.rs` routing, context, cancellation, confinement, cleanup and PTY fixtures, preserving failure stages and process assertions.
2. Name TypeScript support explicitly in `tests/support/mod.rs` and its remaining hostile/discovery consumers. Port language-neutral unit data in `run.rs`, `settings.rs`, `context.rs` and the request fixture in `choice.rs`.
3. Run affected suites, `task check`, `task test` and both book checkers. Re-anchor quoted source if needed, record results and retire this child only.

## Decisions (running log)

2026-10-28: This is a fixture migration against the approved design, with no production behavior change. Native protocol fixtures belong to sibling `rhai-native-fixtures-k103`; `session_process.rs` contains no executable configuration script and needs no port. The policy snapshot-validator unit suite stays with the runtime contraction. No new implementation design or review context is needed for these established test patterns.

2026-10-28: Remove the TypeScript-only CLI assertions for a missing installed Bun worker and a mismatched Bun handshake from the combined selection-deadline test. Retain the real deadline check using a sleeping Rhai worker. Rhai re-executes the front and has no separately installed worker; its identity checks remain in `rhai_worker/front.rs`, and the pending hostile suite retains its mismatched-worker case. Remove `a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path`: it proves only continued TypeScript acceptance, which k101 intentionally ends. Rhai-precedence tests keep deliberately invalid TypeScript sentinels, so evaluating a fallback still fails visibly.

2026-10-28: Rhai checks environment access through `ravel::env`, reads the choice file as measured text, and uses `print`/`debug` for diagnostics. These replace the TypeScript SDK and ambient process APIs in the combined context case. Cleanup faults now come from the test controller after a configured shell harness has exited and its private temporary path has disappeared; the Rhai selector waits on a regular release file. This keeps the faults at worker/outer cleanup, after harness cleanup. The first focused run exposed the missing explicit harness budget; granting exactly one run fixes setup. Both cleanup cases and 18 affected unit cases pass. The new barrier's ordering warrants one narrow review before retirement.

2026-10-28: Spent the leaf's one narrow fresh-context review on cleanup ordering and preservation of context/cancellation behavior. Classified both findings as valid and actionable after checking `confinement.rs` and the harness cleanup sequence: TMPDIR is private-root/tmp, so the controller now waits for the whole root; worker PID presence alone does not prove selection, so a harness called from select now writes its readiness marker. Also publish the cleanup marker with rename to prevent partial contents being observed (a robustness improvement, not a separately demonstrated failure). Existing expected-failure-stage assertions cover the mechanical barrier correction; no second review is needed. The reviewer found no additional actionable context issue.

2026-10-28: Both cleanup tests fail with permission faults disabled, at their original cleanup-diagnostic/exit assertions, then pass restored. `task check` passed all 613 tests (48 skipped), formatting, shell checks, worker build/probes/types, runtime integrity, fake harness smoke, lint and terminal recovery. The separate `task test` run exposed an unchanged native timeout fixture's dependence on reaching owner diagnostics within 1000 ms; its focused rerun passed once build/book work finished. k103 records the failure and readiness requirement. No production or native-fixture source was changed to suppress it.

2026-10-28: Nine book excerpts pass unchanged; no quoted production lines moved. All 22 complete examples pass (57 fragments not executed). The existing example freshness checker initially rejected `tests/support/mod.rs` because Cargo does not rebuild an unchanged front for integration-test-helper edits. Touching `src/main.rs` without changing its contents, then rerunning `task docs:examples`, performed a real rebuild and passed the guard. The planned k100 checker rewrite remains separate. Chapter 10's description of `Fixture::selecting` now needs to say Rhai and the executable's internal worker role; its prose correction belongs to the existing Parts III/IV drafting work. The example fragment's API remains valid.

2026-10-28: Inspected `session_process.rs`, and the unit fixtures of `limits.rs` and `frame.rs`: these contain process/channel or JSON/bound data rather than executable configuration scripts. The limits suite still tests the legacy notice/request helpers while that production path remains available; k101 owns their contraction. Shared TypeScript helpers now have explicit names, and their hostile consumers retain the same source and behavior. No speculative format or production change was added for the owner's notebook-like tutorial idea; it is recorded in the root brief's horizon.

2026-10-28: Final `task test` passed all 613 cases (48 skipped) without concurrent build work. Compared each named result with the final workspace invocation in `task check`: all 613 PASS records agree; the check task's separate PTY receipt suite also passed. The first comparison included that extra PTY result, so the comparison was corrected to delimit the workspace invocations by their Nextest run IDs. SHA-256 and path-set checks before and after verification agree for all 3,606 tracked non-Grove subjects, including Rust and TypeScript source, scripts, manifests and pins, book source and frozen fixtures. Logs: `target/check.SRI0ou`, `/tmp/ravel-k102-test-final.log`; the earlier failed run is retained at `/tmp/ravel-k102-test.log`. All child work is complete. k103 remains live, so k98 and its ancestors do not close.


## Goal



## Context

## Done when

## Notes
