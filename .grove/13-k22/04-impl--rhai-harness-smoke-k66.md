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
