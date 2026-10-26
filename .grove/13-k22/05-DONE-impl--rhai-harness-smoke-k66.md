# rhai-harness-smoke-k66

## Goal

Add two opt-in Taskfile tasks. Each makes one real LLM call, one through Claude Code and one through Codex. The call runs unconfined, with the owner's sign-in. The tasks prove the recipes that the book gives, and an owner runs them after a harness upgrade.

## Context

- The input is "LLM calls through a harness", "Credentials" and "Upgrades" in `docs/specs/policy-evaluation.md`. The table of facts lists the flags of each harness. Check them against the installed `--help` before you rely on them.
- `ravel::run_harness` exists after `rhai-harness-run-k23`. `ravel::json_check` exists after `rhai-json-check-k62`.
- A real call spends subscription usage, and the owner's sign-in lives in the harness store under the owner's `HOME`. Never read, print or record a credential, a token or a sign-in file.
- The default harness-run budget is 0. The sandbox settings of the task raise `budgets.harnessRuns`.
- The previous version of this duty sat in `rhai-stop-order-k24`. That leaf no longer owns it.
- Your running log feeds three later leaves: `rhai-example-policies-k25` for `llm.rhai`, `rhai-book-use-k27` for the recipes of the book, and `owner-policy-conversion-k39`.

## Done when

- One task makes a real call through Claude Code, and one makes a real call through Codex. Each runs the real `ravel start-session` with a Rhai policy in a sandbox configuration. The harness runs unconfined and uses the owner's `HOME`.
- Each call asks a small question with a fixed answer shape. The policy parses the answer with `ravel::json_parse`, checks it with `ravel::json_check`, and selects a harmless fake program as the route. The task checks the route and the notice.
- Each call uses the structured-output flags of its harness: `--json-schema` for Claude Code, and `--output-schema` with a schema file for Codex. The file for Codex sits in the sandbox, where an unconfined run can read it.
- The default `task test` never runs these tasks. A task that finds no program, or no sign-in, stops with a message that names the missing piece.
- The running log records the exact argument vectors that worked, the installed versions, and any difference from the facts table of the spec. It records no credential and no answer text beyond the fixed shape.
- Fix any difference from the spec in this leaf, or cut a leaf that fixes it.
- `task check` and `task test` pass.

## Notes

- Ask the owner before the first real call. Say that it spends subscription usage. If the owner declines, stop and record that.
- The task may run inside a harness session of the driver. Record what a nested run needs, such as inherited variables.
- Name the tasks so that they cannot collide with `smoke` or `release:smoke`.
- `task test:native-linux` does not run these tasks.

## Decisions (running log)

- **The owner agreed to the real calls.** I asked before the first call and named the cost: one small call per harness, which spends subscription usage. The answer was "Yes, one call per harness". Each task ran once, and both passed on the first run.
- **The tasks are `task harness:smoke:claude` and `task harness:smoke:codex`.** Both run `scripts/harness-smoke.sh claude|codex`. `task harness:smoke:test` runs `scripts/harness-smoke.test.sh`, which tests the script against fake harnesses. `scripts/check.sh` runs that test as the step `harness-smoke-fakes`, so `task check` proves the checks without a real call. `task test` runs no smoke task (605 tests run, 48 skipped, and no smoke line in the output).
- **The sandbox home is not the owner's `HOME`, and that differs from the spec.** The spec said that the smoke tasks run unconfined with the owner's `HOME`. Ravel reads settings only from `$HOME/.config/ravel/settings.json` and has no override. A harness run gets the caller's `HOME`. A run in the owner's `HOME` would therefore need the owner's own settings, with `budgets.harnessRuns` raised. The task uses a sandbox home that holds `settings.json` and `policy.rhai`, and it links the sign-in store of the harness into it. The spec "Credentials" section now says this.
- **What the sandbox home links.** I measured with `claude auth status` and `codex login status`, which spend nothing and print no account detail. A sandbox `HOME` with no link: Claude Code and Codex both report signed out. `CODEX_HOME` pointing to the owner's `.codex`, with the sandbox `HOME`: Codex signed in, Claude Code not. `CLAUDE_CONFIG_DIR` pointing to the owner's `.claude`: Claude Code not signed in. A link of `Library/Keychains` alone: Claude Code signed in on macOS. A link of `.codex`: Codex signed in. A link of `.claude` and `.claude.json` without the keychain: Claude Code not signed in. The script links `Library/Keychains` on Darwin and `.claude` elsewhere for Claude Code, and `.codex` for Codex. The `.claude` link on Linux is not measured. Nobody ran the task on Linux, so that link rests on the documented place of the credential file.
- **The environment of the call.** The script removes `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `OPENAI_API_KEY` and `CODEX_API_KEY`, so the call proves the sign-in and an API key cannot pay for it. It also removes `RAVEL_EXIT_FILE` and `RAVEL_POLICY_RUN`.
- **What a nested run needs.** This session ran inside a harness session of the driver. With `RAVEL_POLICY_RUN` in the environment, `start-session` refuses as `policy_reentry_refused`, exit 3. With `RAVEL_EXIT_FILE` inherited and `RAVEL_POLICY_RUN` unset, the nested session does not end with `done` on stdout, so the checks fail. The script removes both, and it comments why. Claude Code inside Claude Code needed nothing else: `CLAUDECODE` and `CLAUDE_CODE_*` stayed inherited and the call worked.
- **Exact argument vectors that worked** (installed: Claude Code 2.1.310, Codex CLI 0.164.0, the versions of the spec; Rhai 1.27.0). The policy sends the question on standard input.
  - Claude Code: `["-p", "--model", "haiku", "--tools", "", "--no-session-persistence", "--output-format", "json", "--json-schema", <the schema as compact JSON text from ravel::json_encode>]`. The answer is `ravel::json_parse(run.stdout).structured_output`. Elapsed time 5,432 ms, status `exited`, exit code 0.
  - Codex: `["exec", "--ephemeral", "--skip-git-repo-check", "--sandbox", "read-only", "-c", "model_reasoning_effort=\"low\"", "--output-schema", <ravel::path_resolve(request.cwd, "answer-schema.json")>, "-"]`. The schema file `answer-schema.json` sits in the working directory of the sandbox, and the policy reads the same file with `ravel::read_json("answer-schema.json").value`. The answer is `ravel::json_parse(run.stdout)`, because the final message is all of standard output. Elapsed time 5,262 ms, status `exited`, exit code 0.
  - The schema, for both: `{"type":"object","properties":{"prime":{"type":"string","enum":["yes","no"]}},"required":["prime"],"additionalProperties":false}`. The question is "Is 17 a prime number? Answer with the JSON object only." Both calls returned the fixed shape with the value `yes`, and the policy selected `smoke-route` with `["routed", "yes"]`.
- **The facts table of the spec held.** `-p` and `--output-format json` with `structured_output` for Claude Code, `--output-schema` with a file and the final message on stdout for Codex, and no `--bare` for a subscription. I found no difference in those rows. The one difference is the `HOME` of the smoke tasks, above, which is fixed in the spec text and needs no code change.
- **Choices for the recipes of `llm.rhai` and the book.** `--tools ""` gives Claude Code no tool, so a call cannot ask for a permission. `--no-session-persistence` and `--ephemeral` keep the call out of the session history of the owner. `--sandbox read-only` and `--skip-git-repo-check` let Codex run in a directory that is no repository. I did not use `--max-budget-usd`, `--effort` or `--json` for either harness. Neither call needed them.
- **The notice names the route in `resolution`, not in `selection`.** `selection` holds `provider`, `model`, `effort` and `reason`. The task reads the route from the argument vector that the route program records and from `resolution`. The notice listed one run with its harness, status, exit code and elapsed time. It listed no prompt and no answer.
- **The notice reported `operationReports` with "stop attempt 1 target -PGID signal 9: Operation not permitted" twice** in the fake-harness runs on macOS, for a managed process that had already ended. The sessions still ended with `done`. I did not chase it here. A later leaf may check whether the code should skip a stop of a group that has no live member.
- **Every check has a control.** `harness-smoke.test.sh` runs the script against fakes. A fake that answers `no`, a Claude Code fake with no `structured_output`, a Claude Code fake that exits 1 with a rate-limit message, a Codex fake with an extra property, a missing program and an unknown harness each make the script fail with a message that names the cause. The two passing fakes check that the arguments hold `--json-schema` and `--output-format`, or `--output-schema` with a path that ends in `answer-schema.json`, and that the question reached standard input. A task with no sign-in, run with an empty `HOME`, stops with "claude has no sign-in" and "codex has no sign-in".
- **The in-session review was not spent.** The claims rest on executable checks with controls, and on two real calls. No framework decision rests on memory: the flags come from the installed `--help` and the facts table of the spec.
- **`task check` and `task test` pass** (605 tests run, 48 skipped, both). The STE lint of the spec reports 0 hard findings after my edit. `task docs:ste` reports one problem in `CONTEXT.md:61`, which this leaf did not touch.
- **`task test:native-linux PLATFORM=arm64` ends with `failed=0`.** It ran once at the end of the increment, as the node brief asks, and it includes the `rust-clippy` and `rhai-worker` steps. It does not run the smoke tasks. Only arm64 ran.
