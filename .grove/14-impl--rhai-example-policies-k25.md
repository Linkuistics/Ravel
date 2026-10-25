# rhai-example-policies-k25

## Goal

Write the Rhai versions of the shipped example policies. Prove that each one gives the same results as its TypeScript original. The examples are the base of the migration guide. The equivalence tests guard the cutover.

## Context

- The input is "Migration" in `docs/specs/policy-evaluation.md`. The requirement "Migration equivalence" and its scenario are the test.
- The TypeScript originals are the shipped static route table `crates/ravel/worker/examples/static.ts` and the shipped sample policy `crates/ravel/worker/sample/policy.ts`. The sample uses `readChoice` and `CHOICE_FILE`.
- The third policy has the shape of the owner's personal policy. It has a harness table, and it grants the main repository of a jj workspace to the harness. It reads a file of the workspace with `node:fs`. Read `~/.config/ravel/policy.ts` to learn the shape, and leave it unchanged.
- Do not copy the personal policy into the repository. Write a new policy of the same shape, with invented names and no personal data.
- The fourth policy is new. It shows an LLM-assisted selection. The policy asks a model through a harness run of Claude Code, and it falls back to a route table. Take the arguments of the call from the running log of `rhai-harness-smoke-k66`, which proved them with a real call.
- All host capabilities exist after the earlier increments. Read the running logs of their leaves for any difference from the spec.

## Done when

- The repository holds `static.rhai`, `sample.rhai`, the owner-shaped policy and `llm.rhai`. They live in a source directory that the removal increment can install under `share/ravel/examples/`.
- The sample replaces `readChoice` with a few lines of Rhai that use `ravel::read_text` and `split()`.
- A test runs each TypeScript policy and its translation through the real `ravel start-session` over one table of requests. It compares the routing notices, the argument vectors that a fake harness receives, the delivered contexts and the refusals.
- The owner-shaped test runs in a secondary jj workspace for every kind that the policy lists. Each argument vector equals that of the TypeScript policy, including the `--add-dir` grant of the main repository. The test skips with a clear message when `jj` is missing.
- `llm.rhai` has a test with a fake harness for the success path and for the failure path with the fallback. It checks the answer with `ravel::json_check`.
- A refusal of the TypeScript policy has the same code and remedy class in its translation, or the leaf records the difference and its reason.
- A short list of every difference between the two languages that the translations met goes in the running log. The migration guide uses it.
- `task check` and `task test` pass.

## Notes

- The comparison skips fields that must differ, such as the worker path and the build identity. State each skipped field in the test.
- The removal increment first freezes the results of this test as fixtures, and then it deletes the TypeScript side. Keep the table of requests in a form that the freeze can reuse.
