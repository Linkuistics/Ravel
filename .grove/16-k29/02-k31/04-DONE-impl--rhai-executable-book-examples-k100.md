# rhai-executable-book-examples-k100


## Goal

Make executable book examples and their freshness checker independent of the TypeScript worker before removing the runtime.



## Context

- `scripts/examples.sh` (`require_fresh_build`), `scripts/examples.test.sh`, and complete examples in book chapters 7–11. Chapter 9 is a known TypeScript-policy example.
- The parent brief preserves the exact example-edit restriction and checker requirements. Book-tool Bun is unrelated and stays.

## Done when

- Every complete example in chapters 7–11 that runs a TypeScript policy runs an equivalent Rhai script with the same behavior, exit code and output. Edit only its policy file and `--config` argument; the later book node owns prose.
- Record each example port for `rhai-book-inside-k35` and `rhai-book-inside-k36`.
- Freshness checks no longer need the worker digest or `libexec/ravel/ravel-policy`; an older-than-Rust-sources binary still refuses. Retain a firing stale-binary control and remove old pair controls.
- Re-anchor any moved excerpt. `task docs:examples`, `task docs:excerpts` and both checkers' tests pass, including the example mutants where affected. `task check` passes after script edits.

## Notes

Both runtimes still exist in this child. Keep changes mechanical, so the final child can delete the runtime without breaking examples.

## Implementation plan

1. Replace worker-pair controls in `scripts/examples.test.sh` with a standalone-binary pass and timestamp controls over production Rust inputs in a scratch checkout. Observe the new control fail against the old checker before changing it.
2. Make `scripts/examples.sh` compare the binary with production Rust sources, build scripts, Cargo manifests/lockfiles, toolchain and Cargo configuration. Exclude integration-test sources and worker artifacts; retain stale-source mutation coverage.
3. Port chapter 9's TERM-ignoring fake, the checker's launch/refusal/cancellation fixtures and its cross-checker excerpt fixture. Keep the chapter's prose and expected output/status unchanged. Update checker documentation to match.
4. Run `task docs:examples`, `task docs:excerpts`, both checker test tasks, example mutants and `task check` through mise. Retire and seal the task only after the checks pass.

## Decisions (running log)

2026-10-28: The graph CLI refuses `list_projects` because a pre-coordination or unverified generation is active. Project/generation and coverage cannot be established; use direct reads and targeted source searches without negative graph claims. The mandated jj working copy starts clean.

2026-10-28: The initial book check passes all nine excerpts unchanged. Source inspection of chapters 7–11 finds one complete TypeScript-policy example: chapter 9's fake that ignores TERM after sending `finished`. Port only its script/file label and `--config` argument. Leave chapter 8's narrative TypeScript references to the later book leaves.

2026-10-28: Scope freshness to production Rust inputs rather than every `.rs` beneath `crates`: Cargo can rebuild integration tests while reusing an unchanged production binary. Keep Rust sources under `src/`, build scripts, Cargo manifests/locks, workspace Cargo configuration and the pinned toolchain as freshness inputs. Exercise stale and fresh cases in an isolated scratch checkout, so the controls never touch real source timestamps. The checker fixture policies and source excerpt also move off the retiring worker; selection cancellation uses an owner-configured fake harness run that signals the Ravel CLI.

2026-10-28: The new controls first failed against the unchanged checker: the standalone binary was refused for its missing `libexec` worker, and the scratch checkout was refused because the checker tried to compute a worker digest. After the repair and fixture ports, every example-checker control passes, including each stale production input and five consecutive cancellation runs. Add mutations that remove Rust-source checks or wrongly include integration tests, beside the retained stale-build-refusal mutant.

2026-10-28: Ported chapter 9's complete example at “Run a fake that ignores TERM” for the later `rhai-book-inside-k35`/`rhai-book-inside-k36` drafts: it now exports Rhai constants and defines `select(request, context)` with the same selection fields. Only the policy block/file label and `--config` argument changed; `finished`, status 0, fake-agent code and prose stay as before. No quoted Rust source moved, so the nine existing excerpt anchors need no re-anchoring. Checker fixtures now evaluate Rhai; its shared excerpt control reads tracked Rust instead of the retiring SDK. Label-grammar controls retain inert TypeScript snippets deliberately.

2026-10-28: Spent the leaf's single fresh-context reviewer on the checker boundary and mechanical port. One finding was valid/actionable: find's absolute `*/src/*.rs` pattern also matched test paths when a checkout ancestor was named `src`. Moving the scratch fixture under a `src` ancestor reproduced exactly that false stale refusal. The repair selects immediate crate `src` directories with shell globs before traversing them, keeping tests outside the traversal regardless of the checkout path. The existing stale-source mutation controls cover this repair; no second reviewer is required for a fix conclusively checked through that executable seam.

2026-10-28: The full excerpt-checker test task exposed a pre-existing stale assertion: `chapters_one_to_six_pass` demanded zero excerpts, but chapter 4 now quotes `crates/ravel/examples/static.rhai`. The failure is “1 passed, 0 failed” against the expected zero, with the real nine-excerpt book check passing. Let this corpus control accept any passed count while still requiring zero failures/errors; retain the empty-input behavior as a separate fixed prose-page fixture. Port both SDK-reading excerpt controls to the tracked Rhai example so k32 can remove the SDK. The first full workspace check passed before these final test-script edits; repeat the required check against the settled files, with a fresh subject digest.

2026-10-28: Final verification passed after all artifact edits: `mise exec -- task check` (formatting, shellcheck, worker build/probes/integrity/types, fake harness smoke, Clippy, terminal recovery, then 604 passing tests and 48 skipped); `task docs:examples` (22 passed, zero failed/skipped, 57 labelled fragments); `task docs:excerpts` (nine passed); both checker test tasks; and `task docs:examples:mutants` (30/30 killed). The excerpt test task also killed all 24 mutants. Each required task ran through mise. The final example locations and unchanged chapter 9 stdout/status claims agree with the baseline, rather than relying only on matching totals. The source-root regression and both source-scope mutants fired at the intended boundary. No Linux campaign was needed for this script/example-only leaf; k101 still owns the native lookup contraction and Linux rollup.

2026-10-28: The book extractor's full file/command inventory has no executable `policy.ts`, SDK import or `definePolicy`. The same search found those constructs in the parent revision of chapter 9 as a positive control, and the current extracted chapter 9 has the Rhai constants and `--config policy.rhai`. The remaining chapter 8 TypeScript mentions are narrative and deliberately await the later book drafts. `docs/documentation-lessons.md` L40 now records the freshness scope, checkout-path regression and changing-document count lessons.

2026-10-28: The final run hashed every tracked non-`.grove/` project file plus `target/debug/ravel` before and after the complete verification sequence. All 3,607 per-file records match; both manifests have SHA-256 `96ecf2f87f96afd4b789c527769dbe597d025aad8c8035373f0a6a79e6e2602b`. Local logs are `target/k100-final-*.log`, the subject path list is `target/k100-subjects.paths`, and the before/after manifests are `target/k100-final.{before,after}.sha256`. These measure the project inputs and binary, not external toolchain contents. The parent remains open with k101 live; no review leaf names this producer and no node closes at this retirement.
