# rhai-equivalence-fixtures-k30

## Goal

Freeze the results of the TypeScript policies as fixtures, while the TypeScript path still works. The Rhai translations then keep a reference after the removal.

## Context

- The input is the equivalence test of `rhai-example-policies`. Read its running log for the table of requests and for the skipped fields.
- The spec says that the increment which removes the TypeScript path first freezes these results as fixtures. See "Migration" in `docs/specs/policy-evaluation.md`.
- A fixture holds, for each request, the routing notice, the argument vector that the fake harness received, the delivered context and the refusal. It omits the fields that must differ between languages.

## Done when

- A fixture file exists for each translated policy: the static route table, the sample, the owner-shaped policy and, where it applies, the LLM example.
- The Rhai side of the equivalence test compares against the fixtures and needs no TypeScript policy.
- A test shows that a change to a fixture makes the comparison fail. This is the firing control.
- The test of the owner-shaped policy keeps its secondary jj workspace and its skip message.
- The old TypeScript side of the test stays in place until the cutover leaf. A comment names the leaf that removes it.
- `task check` and `task test` pass.

## Notes

- Generate the fixtures with a command, and record the command in the running log. Do not write them by hand.
- Run the generator once, review the result, and commit it. A later session must never regenerate a fixture to make a test pass.

## Decisions (running log)

- **The fixture is the existing observation, keyed by case name.** Keep the current exclusions and `Expect` rules, including the documented language differences. Each of `static.json`, `sample.json` and `harness-table.json` will hold the TypeScript observations. The harness-table file also holds the 14 secondary and two primary jj workspace observations. `llm.rhai` has no TypeScript original, so its existing fake-harness assertions remain its reference; no invented TypeScript baseline is applicable.
- **Implementation and validation.** Add independent Rhai-to-fixture checks and a control that changes an effort and a refusal in an in-memory fixture. Keep the live TypeScript checks, marked for removal by `typescript-worker-cutover-k31`. Extract the existing jj observation loop so the generator and both comparison paths use the same real workspaces and grant assertions. Add a one-shot ignored-test generator behind a Taskfile command; it must refuse an existing destination and require jj and an unprivileged user. Generate once, review every case and its coverage, then run `task check` and `task test` before retirement.
- **Graph fallback.** `codebase-memory-mcp cli list_projects` refused with “a pre-coordination or unverified CBM generation is active”. No graph project or generation could be confirmed. Discovery used the earlier leaf's exact test path and targeted source reads; no graph completeness claim is made.
- **The one capture succeeded.** Command: `mise exec -- task equivalence:freeze`. It recorded 24 static cases, 51 sample cases, 44 harness-table cases and all 14 secondary plus two primary jj observations, using the actual TypeScript path. The sorted SHA-256 inventory of the nonignored files under `crates/` and `scripts/`, plus `Taskfile.yml`, both workspace Cargo files and `rust-toolchain.toml`, was unchanged before and after capture (inventory digest `ade2f2c88a3e4d3b7aee6de55975cf5590ec61d5fe6edc1ca05269de68ff0f32`). The capture's fixture digests are `ef53987c43a387e2d2a9bac84bd36a92d9dacc9c7001ef60fbcba2c7b277aac7` (static), `ca6ae3a2889e2c8947cafdc33774d22fd5869b1119562b38e441a31cd41e2101` (sample) and `0885d0c3bff543a0996673e228e2e5a1122a02e1964db50442bb485466049e9f` (harness table). No fixture was edited or regenerated.
- **A second temporary-root length exposed one more volatile field.** All 13 example tests passed after capture. Running the Rhai fixture test with `TMPDIR=/tmp` then failed only on `context.encodedBytes`: the measured path in the delivered context made the value 285 instead of the captured 329. Keep the fixture bytes unchanged. Validate this byte count against the actual context value before path substitution on each live observation, then omit it only from the cross-sandbox comparison. Continue comparing the complete normalized context value, its source-byte count and its measured sources. Add a changed-context firing control.
- **The portability repair passed its disproof attempt.** `TMPDIR=/tmp mise exec -- task test:session -- --test rhai_examples -E 'test(rhai_examples_match_the_frozen_typescript_observations) | test(the_fixture_comparison_fails_on_a_changed_fixture)'` passed both tests. The firing control changes an existing field in an in-memory fixture and requires exactly the named case to fail, separately for effort, refusal remedy and context summary. The untouched cases compare equal first.
- **Fixture review and review allowance.** Reviewed every captured case's name, exit, launched program, effort and refusal code; inspected the complete delivered context and each documented language difference; checked the workspace keys against all 14 kinds and the two primary cases. The fixture digests still equal those from the single capture. The comparison retains the earlier `SameExceptMessage` and `Differs` semantics: a documented language difference proves inequality rather than prescribing an exact Rhai observation. No in-session reviewer was materialised: executable comparisons, field mutations and the different-root experiment are the more direct instrument for these behavioural claims. No framework API or architectural decision changed, and no ADR revision is needed. The cutover leaf now points to the immutable files and the generator it must remove.
- **Final validation passed.** `mise exec -- task check` passed formatting, shell lint, worker build and probes, runtime integrity, fake harness smoke checks, worker type checks, Rust lint, terminal recovery and all 618 tests (49 skipped). The separate `mise exec -- task test` passed the same 618 tests (49 skipped); comparison of the complete sorted per-test PASS records confirmed the same test identities, not just matching totals. The final inventory of nonignored files under `crates/` and `scripts/`, plus `Taskfile.yml`, `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`, had identical SHA-256 records before and after both runs (inventory digest `5c188805d185051f9efe4d426b0190a210c7eb1a0697eac4fe64bb784c2b2109`). All three fixtures still have their capture digests above. No quoted product code moved, so no book excerpt or example needed repair. The parent node retains its three live removal/reconciliation leaves and does not close here.
