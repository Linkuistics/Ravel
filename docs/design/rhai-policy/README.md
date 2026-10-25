# Rhai policy evaluation views

Run `task docs:rhai-policy`, then open <http://127.0.0.1:8776/#discussion>. If
that port is in use, run `task docs:rhai-policy PORT=8882` and open
<http://127.0.0.1:8882/#discussion>. Read each graph view from its dark node,
labelled `Start here`. Read each sequence from its first message. The sources
stay editable beside the viewer.

The [policy evaluation specification](../../specs/policy-evaluation.md) is the
authority for each rule that these views draw. The design of LLM calls through
a harness run, and the integration of its review, changed these views:

- [Processes and authority → The CLI acts and the worker computes](index.html#diagram-processes) — Removed the LLM adapter and the LLM endpoints. The harness run now reaches the model service with the owner's own sign-in.
- [Evaluation → One LLM-assisted selection](index.html#diagram-evaluation-sequence) — The selection now asks Claude Code through `ravel::run_harness`, then parses and checks the answer in the worker.
- [Evaluation → Data or the end of the evaluation](index.html#diagram-outcomes) — Replaced `llm_budget_exhausted` with `harness_budget_exhausted` among the bounds. The replies no longer name the statuses of `ravel::llm`. A run that Ravel stops at its own deadline or output bound returns as data.
- [Evaluation → Evaluation states](index.html#diagram-evaluation-states) — The stop edge names the whole deadline and a breach of a bound of the evaluation. A run that Ravel stops at its own deadline or output bound does not stop the evaluation.
- [Stopping an evaluation → A deadline or a signal stops the evaluation](index.html#diagram-stopping-sequence) — Removed the branch that drops a Rig request. An LLM call stops as a harness run. The title no longer says that the stop reaches every process. The worker now stops with TERM and KILL after one second, as the code does, and a cleanup failure shows as `harness_cleanup_failed` with exit 5.
- [Rejected alternatives → Rejected: a nested start-session](index.html#diagram-rejected-nested-session) — Added the rejected shape, with the process group that the outer stop cannot reach.

The [entry precedence](index.html#diagram-entry-precedence) did not change.

The reused static viewer loads Mermaid 12.0.0, so the views need network access.
`task docs:rhai-policy` is the presentation recipe, and there are no generated
SVG files. A rendered view proves delivery of the diagram, not the behaviour of
the executable.
