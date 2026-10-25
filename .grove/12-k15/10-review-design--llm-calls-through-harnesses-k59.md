# llm-calls-through-harnesses-k59

**Reviews:** llm-calls-through-harnesses-k58

## Goal

Read the design change of `llm-calls-through-harnesses-k58` adversarially, and report findings without fixes. The change moves every LLM call to a harness run of Claude Code or Codex. It removes `ravel::llm`, the `endpoints` settings key, the LLM budgets and Rig, and it keeps the schema check as `ravel::json_check`.

## Context

- The producer's commit holds the change. Find it with `jj log` and the handle. It touches `docs/specs/policy-evaluation.md`, the ADR set, `CONTEXT.md`, `docs/design/rhai-policy/` and the briefs.
- The running log of `llm-calls-through-harnesses-k58` holds the owner's decisions. Do not reopen a settled decision. Report a consequence that the decision did not see.
- The owner's amendment to GitHub issue 2 is a comment on the issue. Read it with `gh issue view 2 --comments`.
- `task docs:rhai-policy` serves the changed views.

## Done when

- The findings answer each doubt below, with evidence.
  - Does the spec meet each acceptance criterion of issue 2, as the owner amended it?
  - Is every rule that named `ravel::llm`, an LLM endpoint, Rig or an LLM budget gone or rewritten? The summary layers count too. They are the Solution, the tables of host capabilities, budgets and errors, and the notice. They are also the requirements, "Out of scope", the glossary and the design views.
  - A Codex call with structured output needs a schema file that the owner keeps. Is that workable, and does the spec say enough about it?
  - The harness-run budget defaults to 0 and allows 16. Do those values still fit, now that every LLM call is a harness run?
  - Can a harness use its sign-in inside confinement? The spec says that it depends on the harness and the platform. Is that statement honest enough, and does a test need it?
  - `ravel::json_check` throws `schema_invalid` and does not end the evaluation. Does that weaken a requirement?
  - The Credentials section now says that Ravel trusts the owner policy. Does another part of the spec contradict that, such as the leak probes or the hostile classes?
  - Does each claim about Claude Code and Codex match the cited documentation and the `--help` output of the installed versions?
- If findings are worth acting on, this session cuts the `integrate-review-design` leaf.

## Notes

- Make no real call to a model, and never read, print or record a credential.
