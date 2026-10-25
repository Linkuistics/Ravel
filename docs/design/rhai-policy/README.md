# Rhai policy evaluation views

Run `task docs:rhai-policy`, then open <http://127.0.0.1:8776/#discussion>. If
that port is in use, run `task docs:rhai-policy PORT=8882` and open
<http://127.0.0.1:8882/#discussion>. Read each graph view from its dark node,
labelled `Start here`. Read each sequence from its first message. The sources
stay editable beside the viewer.

The [policy evaluation specification](../../specs/policy-evaluation.md) is the
authority for each rule that these views draw. This design added every view:

- [Processes and authority → The CLI acts and the worker computes](index.html#diagram-processes) — Added the two processes and the authority of each. The CLI also has a reader thread.
- [Evaluation → One LLM-assisted selection](index.html#diagram-evaluation-sequence) — Added the sequence from `start-session` through one LLM call to the launch.
- [Evaluation → Evaluation states](index.html#diagram-evaluation-states) — Added every path to a refusal, and the reap before the launch.
- [Evaluation → Data or the end of the evaluation](index.html#diagram-outcomes) — Added the split between outcomes as data and breaches of a bound.
- [Stopping an evaluation → A deadline or a signal stops everything](index.html#diagram-stopping-sequence) — Added the order of the stop and the final read of the cancellation latch.
- [Configuration and migration → Which policy file Ravel evaluates](index.html#diagram-entry-precedence) — Added the precedence of `policy.rhai` and the refusals.

The reused static viewer loads Mermaid 12.0.0, so the views need network access.
`task docs:rhai-policy` is the presentation recipe, and there are no generated
SVG files. A rendered view proves delivery of the diagram, not the behaviour of
the executable.
