# Rhai configuration evaluation views

Run `task docs:rhai-policy`, then open <http://127.0.0.1:8776/#discussion>.
If that port is in use, run `task docs:rhai-policy PORT=8882` and open <http://127.0.0.1:8882/#discussion>.
Read each graph from its dark node, labelled `Start here`.
Read each sequence from its first message. The sources remain editable beside the viewer.

The [configuration evaluation specification](../../specs/policy-evaluation.md) is the authority for these views:

- [Processes and authority](index.html#diagram-processes) shows the Ravel CLI, the Rhai worker, the reader thread and harness jobs.
- [One LLM-assisted selection](index.html#diagram-evaluation-sequence) shows one request map sent to `ravel::run_harness`, followed by answer checks and selection.
- [Data or the end of the evaluation](index.html#diagram-outcomes) distinguishes recoverable host outcomes from evaluation bounds.
- [Evaluation states](index.html#diagram-evaluation-states) shows admission, host calls, stopping, judgement and launch.
- [Stopping an evaluation](index.html#diagram-stopping-sequence) shows the host stop, worker teardown and final cancellation decision.
- [Entry precedence](index.html#diagram-entry-precedence) shows the re-entry refusal, Rhai discovery and retired-format remedy.
- [Rejected nested session](index.html#diagram-rejected-nested-session) shows the separate group that an outer evaluation cannot stop.

The viewer loads Mermaid 12.0.0, so these views need network access.
`task docs:rhai-policy` serves the viewer. No generated SVG files are part of these views.
A rendered view proves diagram delivery. It does not prove executable behaviour.
The directory and task names retain their existing identifiers until the coordinated configuration rename.
