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

## Decisions (running log)

- **The examples live in `crates/ravel/examples/`.** The files are `static.rhai`, `sample.rhai`, `llm.rhai` and `harness-table.rhai`. The last one is the owner-shaped policy. The spec installs only the first three under `share/ravel/examples/`. `harness-table.rhai` is for the migration guide and the tests, so `typescript-delivery-removal-k32` does not install it.
- **The TypeScript originals.** The static and sample originals are the shipped files `worker/examples/static.ts` and `worker/sample/policy.ts`, read with `include_str!`. The owner-shaped original is new: `crates/ravel/tests/equivalence/harness-table.ts`. It has the shape of `~/.config/ravel/policy.ts` with invented names and no personal data. `typescript-worker-cutover-k31` deletes that directory with the rest of the TypeScript side.
- **The owner-shaped policy.** It has two invented programs, `lead-agent` and `review-agent`. They take their arguments in two layouts, as the owner's two harnesses do. It lists 14 kinds, and it reads `.jj/repo` to grant the main repository of a secondary workspace. I did not read the owner's policy into the repository. I only took its shape.
- **The test is `crates/ravel/tests/rhai_examples.rs`.** The name starts with `rhai_`, so the `rhai-worker` step of `scripts/native-linux/measure.sh` runs it. That step builds the Bun worker first. Each case runs the real `ravel start-session` twice, once per language, in two sandboxes with a fake agent that records its argument vector.
- **The table of requests is `cases_of(policy)`, and `observe` is the freeze point.** A `Case` has a kind, a prompt, extra arguments, setup steps and an `Expect`. `observe(language, policy, case)` returns one JSON value: the exit code, the program and arguments that the fake agent saw, the compared part of the notice, and the refusal. `rhai-equivalence-fixtures-k30` stores that value for each case. The tables hold 24 cases for the static policy, 51 for the sample and 44 for the harness table.
- **The comparison skips these fields, and the head of the test says why.** `schemaVersion` of the notice (1 and 2), `policy` (path, version string, and the module list that only Rhai has), `worker`, `selectionMs`, `operationReports`, `reads`, `harnessRuns`, `bounds.harnessRuns` and `context.sha256`. The last one hashes the delivered context, which holds the sandbox path, and the two languages use two sandboxes. The test also asserts that each notice comes from the path under test: `bunVersion` for TypeScript and `rhaiVersion` for Rhai.
- **Three outcomes for a case.** `Same` compares every kept field. `SameExceptMessage` compares the refusal without its message, where the message holds the text of a system error. `Differs` records a difference and a reason, and the test fails if the results are equal. So the list of differences below cannot go stale unnoticed. Five cases are `Differs`.
- **The jj test uses a real secondary workspace.** It runs `jj git init --colocate` and `jj workspace add` with a home, a user and an email of its own, so no machine configuration reaches jj. It runs every one of the 14 kinds in the secondary workspace, and two kinds in the primary one. For both languages it asserts that the argument vector holds `--add-dir` once, with the main repository, and that the primary workspace has none. Then it compares the two languages. It prints `SKIPPED: jj is not installed …` and returns when `jj` is missing. The Linux image has no `jj`, so there the test skips.
- **`llm.rhai` asks Claude Code which kind of work the prompt describes.** It sends the arguments that `rhai-harness-smoke-k66` proved: `-p --model haiku --tools "" --no-session-persistence --output-format json --json-schema <schema>`. The question goes on standard input. The schema allows one property, `kind`, with the keys of the table as its `enum`. It checks `structured_output` with `ravel::json_check`. The answer can only pick a key of the table, so it never reaches `program` or `args`. On any failure the caller's own kind decides, and the reason of the selection says which one decided.
- **What `llm.rhai` counts as a failure.** Five results count:
  - a status other than `exited`
  - an exit code other than 0
  - text that is not JSON
  - a document with no `structured_output`
  - an answer that breaks the schema

  Each one writes one line with `debug`. The line names the status or the error code and never a part of the answer. A missing `claude` program is `unavailable`, and it falls back too. A budget of 0 is not a failure. It ends the evaluation as `harness_budget_exhausted`, and the file says that the owner must raise `budgets.harnessRuns`.
- **Tests of `llm.rhai`.** The success path checks the exact arguments, the exact standard input, one run in the notice and no prompt in the notice. Its firing control is another answer that gives another route. The failure path uses a fake `claude` that exits 1 with a rate-limit message. Its control is a kind that the table does not list, which refuses with `incomplete_mapping`. Seven other bad answers give the fallback with their own diagnostic. The budget test has a control with one run of budget.
- **Controls for the comparison.** `the_comparison_fails_on_a_changed_translation` changes one effort and one remedy in a copy of `static.rhai`, and the comparison fails for each. I also changed each shipped example by hand. I changed one effort in `static.rhai`. I changed the reason and the `one_choice` test in `sample.rhai`. I dropped the `--add-dir` of the lead layout in `harness-table.rhai`. I changed the answer path in `llm.rhai`. Each change failed its test. I restored the files after each run.
- **The Linux check.** I ran the new binary in the `ravel-native-linux:arm64` container after `task setup` and `task worker`. All 10 tests passed as the unprivileged user. The jj test skipped, because the image has no `jj`. I did not run the whole campaign, because this leaf changes no process-group code.
- **The in-session review was not spent.** Every claim is an executable comparison with a firing control. No decision rests on a framework API that I remember.
- **`task check` passes** (615 tests run, 48 skipped). `task test` runs the same nextest command, and it passed inside `task check`. No quoted code changed, so `task docs:excerpts` and `task docs:examples` do not apply.

### Differences between the two languages (for the migration guide)

The translations met these differences. Each one is either proved by a case above or met while I wrote the Rhai.

**Shape of the policy**

- `export const policy = definePolicy({ … })` becomes `SCHEMA_VERSION`, `VERSION` and `fn select(request, context)` at the top level. `VERSION` strings in the examples move with the language: `ravel/examples/static 3`, `owner-choice-1` (unchanged), `harness-table 3` and `llm-triage-1`.
- A function does not see a top-level constant by its name. It reads `global::NAME`. The examples do this for `ROUTES`, `CHOICE_FILE` and `OFFERED`.
- `import … from "ravel/examples/static"` becomes `import "static" as starter;`. The alias cannot be `static`, because `static` is a reserved word in Rhai.
- A constant of an imported module is `starter::ROUTES`, and a function is `starter::select_route(…)`.

**Values and strings**

- A route table of arrow functions becomes a map of closures. Call a closure in a variable or a map with `.call(…)`. A call such as `lead("high")` on a variable fails with "Function not found". Top-level functions avoid this.
- `Object.hasOwn(routes, kind)` becomes `routes.contains(kind)`. A Rhai map has no prototype. The kinds `constructor`, `__proto__` and `toString` refuse in both languages, and cases prove it.
- `"status" in choice` becomes `type_of(choice) == "map"`. `undefined` becomes `()`, and a test is `x == ()`.
- `JSON.stringify(text)` becomes `ravel::json_encode(text)`. The cases with a quote, a backslash, accents, a control character, DEL and U+2028 give the same bytes.
- A backtick string interpolates `${…}` and has no `\n` escape. Use a normal string and `+` for a newline. A Unicode escape is ` ` with four digits. `\u{…}` is a syntax error.
- An array has no `join`. The sample writes a loop (`offered_text`).
- `text.trim()` edits the string in place and returns `()`. Write `store.trim();` on its own line.
- `text.split()` with no argument splits on white space. It returns `[""]` for an empty text, so the sample filters out empty names.
- White space differs at two characters. U+FEFF is white space for a JavaScript `\s` and `trim()`, and it is not for Rhai. U+0085 is the other way round. Two cases prove each direction.
- `switch` takes `_ => ()` for the default arm. A `case` with `return` becomes an arm that returns a value.

**Reads and errors**

- `statSync` and `readFileSync` become one `ravel::read_text(path, max_bytes)` under `try`. `source_missing` means no file, and a dangling link counts as missing, as before. `source_not_regular` covers a directory and a FIFO.
- A read is a measured source. The notice lists it in `reads` with its phase, where `readChoice` was an ordinary read that the notice did not list. A read also counts against the 256 sources.
- A choice file of more than 4096 bytes refused as `choice_unreadable` in TypeScript. In Rhai the read ends the evaluation as `source_too_large` at stage `context`, because a read bound cannot be caught. Both exit with 3. The remedy differs.
- A file that is not UTF-8 refuses in both languages with the same code in the sample. In the harness table, TypeScript decodes the bytes with replacement characters and goes on. `read_text` refuses, and the policy gives `jj_store_unreadable`.
- The message of a refusal that holds the text of a system error differs by runtime. This affects an unreadable file, and a `.jj` that is a file (`ENOTDIR` against "Not a directory"). The codes, the exit codes and the remedies are equal.
- A caught host error is a map with `code`, `message` and `name`. A caught runtime error of the script, such as an absent property, is a map with no `code`. A policy that reads `error.code` must first test the shape of the value, or avoid the runtime error as `llm.rhai` does.
- `host.diagnostic(text)` becomes `debug(text)`. The line in the diagnostics starts with the file, line and position of the call.

**The notice**

- The Rhai notice has schema version 2. It adds `policy.modules`, `reads` and `harnessRuns`, and `bounds.harnessRuns`. Everything else that the policies set is equal: `selection`, `resolution`, `context`, `promptSource`, `policyEnvironment`, `diagnostics` and the rest of `bounds`.

**Not translated**

- `llm.rhai` has no TypeScript original. A TypeScript policy could only use `fetch`, and the spec retired that.
- The example `static.ts` also exports `Route` and `Command` types and an `agent` function for other policies. `static.rhai` exports `ROUTES`, `agent` and `select_route`, and its entry `select`. A wrapper imports it by path.

### Handed forward

- **`rhai-equivalence-fixtures-k30`:** call `observe(language, policy, case)` for `cases_of(policy)` with `Language::TypeScript`. Store each value, once, with a command. The jj test needs its own fixture: the 14 secondary and 2 primary observations.
- **`rhai-book-use-k27` and `k28`:** the recipe of `llm.rhai` is the Claude Code recipe of the smoke log. The Codex recipe is not in an example. Only the smoke script holds it. The difference list above is the input of the migration chapter.
- **`typescript-delivery-removal-k32`:** install `static.rhai`, `sample.rhai` and `llm.rhai` from `crates/ravel/examples/`. `sample.rhai` and `llm.rhai` import `static` by path, so the three files must sit in one directory.
- **`owner-policy-conversion-k39`:** `harness-table.rhai` is the pattern for the conversion. Its `main_repository` function is the translation that the owner's policy needs.
