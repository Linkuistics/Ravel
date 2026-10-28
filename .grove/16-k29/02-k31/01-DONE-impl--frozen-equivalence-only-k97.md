# frozen-equivalence-only-k97


## Goal

Remove live TypeScript execution from the example-equivalence suite. Independent Rhai runs keep comparing with the frozen TypeScript observations.



## Context

- `crates/ravel/tests/rhai_examples.rs`, its fixtures under `tests/equivalence/fixtures/`, and `Taskfile.yml`.
- `rhai-equivalence-fixtures-k30` records the single capture, its digests and the comparison exclusions. Never edit or regenerate those JSON fixtures.

## Done when

- No TypeScript source constants, live dual-language branches, one-shot capture test or `equivalence:freeze` task remain in this suite.
- The three policy fixture comparisons, all existing field-mutation controls, the real secondary jj workspace comparison and its skip message, the moved-store grant control and the LLM checks remain.
- The control that changes a Rhai translation compares it against the frozen fixture and still fires.
- The fixture digests match the capture. Focused example tests, `task check` and `task test` pass.

## Notes

The TypeScript runtime and all other fixture suites stay available for the following migrations. No product code or book excerpts move in this child.

## Decisions (running log)

2026-10-28: This is the first child of the mandated `typescript-worker-cutover-k31` decomposition and is the only child implemented in this session, as Grove requires. The approved parent spec supplies the design; no new design approval is needed. Use the existing sandbox seam and immutable fixtures rather than introducing another comparison abstraction.

2026-10-28: Implementation plan: first verify the existing Rhai fixture and mutation controls; remove TypeScript source fields and language dispatch; require a fixture in the comparison helpers; port the changed-translation and moved-store controls to Rhai only; remove the three live policy comparisons, the duplicate live jj comparison and the one-shot generator/task; verify unchanged fixture digests and all retained tests. This test-infrastructure refactor adds no production behavior or framework API decision.

2026-10-28: Baseline: the existing frozen-observation test, fixture-mutation control and live changed-translation control all passed before the refactor. The comparison now requires a fixture, and the sandbox installer has only the Rhai setup. The changed-translation control uses the frozen `route bugfix` and `unlisted "nope"` cases; it still changes a real policy's effort and refusal remedy, requires the comparison to fail and confirms the original policy compares equal. The field-mutation control continues to exercise effort, remedy and context summary independently.

2026-10-28: Removed `the_static_route_table_gives_the_same_results_in_both_languages`, `the_sample_gives_the_same_results_in_both_languages` and `the_harness_table_gives_the_same_results_in_both_languages`: the retained frozen-observation test executes each identical Rhai request table against the immutable TypeScript baseline. Removed `the_harness_table_grants_the_main_repository_for_every_kind_in_a_secondary_jj_workspace`: the retained frozen-workspace test executes the same real jj workspace loop against its captured baseline and retains the absent-jj skip message. Removed the ignored `freeze_typescript_observations` generator and `equivalence:freeze` Taskfile task as required; the capture must never run again. Removed the unused `tests/equivalence/harness-table.ts` original after checking its references: its only executable consumer was the deleted source constant. The fixtures directory and all three JSON observations stay unchanged. The moved-store grant control now executes only Rhai. The LLM tests are unchanged.

2026-10-28: No in-session reviewer is materialised for this mechanical contraction. The existing executable comparisons and their deliberate fixture/policy mutations directly test the central behavioral claim; another read of this same deletion would be a weaker instrument. No architectural or framework decision changed. Source discovery uses the graph fallback recorded in the parent brief.

2026-10-28: The focused suite passed all nine retained tests, including the real jj workspaces. With `TMPDIR=/tmp`, all three frozen-fixture and mutation tests passed again. The first full `task check` passed its formatting, shell lint, worker build/probes/types, runtime integrity, fake harness checks, Rust lint and terminal recovery, then stopped at the unchanged `rhai_limits::a_native_call_that_does_not_yield_stops_at_the_deadline`: the successful pad control exceeded its separate one-second elapsed-time assertion. It had returned a valid selection, rather than hitting the worker deadline. The test passed alone without a code change. The existing nextest configuration already reserves all test slots for other one-second timing cases. This observation is consistent with contention, not proof of its precise source. Re-run the unchanged suite with four concurrent tests using the installed nextest help's documented `NEXTEST_TEST_THREADS` setting; do not widen deadlines, add retries or change product code. The source inventory was unchanged across the failed run.

2026-10-28: `task docs:excerpts` passed all nine excerpts. The first `task docs:examples` stopped before running examples: its conservative freshness scan includes Rust integration tests, and Cargo correctly reused the product binary after this test-only edit. Clear only this package's cached build artifacts with `mise exec -- cargo clean -p ravel`, then rerun the normal task to obtain a fresh product binary. No source timestamps or binary contents are patched to bypass the checker. The already scheduled example-checker child retains responsibility for the worker-pair removal.

2026-10-28: With the fresh build, `mise exec -- task docs:examples` passed all 22 complete examples with no failures or skips (57 labelled fragments are outside execution). `NEXTEST_TEST_THREADS=4 mise exec -- task check` then passed every prerequisite check and all 614 selected tests, with 48 existing skips. The previously failing deadline test passed in this complete run. The separate `task test` is required before retirement and uses the same concurrency setting; this is a documented scheduler setting, not a retry policy or an assertion change.

2026-10-28: The first separate four-thread test run stopped after 593 cases at the unchanged `worker_reap_admission_success_gate_remove_choice_missing`. Its successful Bun evaluation crossed the fixture's 1000 ms selection deadline, returning `selection_timeout` where the injected teardown-removal failure expected `worker_failed`. The preceding full check had passed this exact case. This case already reserves every nextest slot, so reducing concurrency does not establish an explanation for its timing. Inspect the original fixture and rerun the individual case before another complete run. This fixture is among those the next child ports to Rhai; preserve its cleanup assertions and explicit deadline while migrating it.

2026-10-28: The admission case passed alone without edits, and the complete `NEXTEST_TEST_THREADS=4 mise exec -- task test` rerun passed all 614 tests with 48 existing skips. The sorted per-test PASS identities match the successful `task check` run item by item. The two earlier timing failures remain observations, not repaired defects or silently enabled retries. No deadline, test assertion, scheduler configuration or production source was changed.

2026-10-28: Final source inventory: every tracked file outside `.grove/`, including the Rust and TypeScript sources, all fixtures, scripts, manifests, lockfiles, `.config/nextest.toml` and book sources. The before/after SHA-256 records are identical, with inventory digest `38c0bbaddc63dc6abb0787616b028d5984763b1937769877470f199cec96b597`. Each historical fixture also retains its exact capture digest from `rhai-equivalence-fixtures-k30`: static `ef53987c43a387e2d2a9bac84bd36a92d9dacc9c7001ef60fbcba2c7b277aac7`, sample `ca6ae3a2889e2c8947cafdc33774d22fd5869b1119562b38e441a31cd41e2101`, harness table `0885d0c3bff543a0996673e228e2e5a1122a02e1964db50442bb485466049e9f`. The fixture tables and comparison exclusions are unchanged. Source enumeration shows only Rhai policy constants and installation calls in the changed suite; the old source still supplies the positive control for the removed TypeScript dispatch. The normal Taskfile list parses and exposes no capture task.

2026-10-28: This child's acceptance conditions hold. No quoted product source moved, the nine excerpts and 22 complete examples passed, and no ADR decision changed. Retire only `frozen-equivalence-only-k97`. The parent `typescript-worker-cutover-k31` retains four live migration/contraction children, so neither it nor `typescript-removal-k29` closes in this session. Next is `rhai-runtime-fixtures-k98`.
