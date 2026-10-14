# Result-job completion views

Run `task docs:result-job`, then open
<http://127.0.0.1:8774/#discussion>. The document fills the browser window.
If that port is occupied, use `task docs:result-job PORT=8878` and open
<http://127.0.0.1:8878/#discussion> (this design's presentation).
Graph views start at the dark node labelled **Start here**; sequences start
at their first message. Sources remain editable beside the viewer.

- [Completion and failure → Required failure facts and settlement precedence](index.html#diagram-repair-outcomes) — Added error-carried cancellation and original-failure/cleanup/result order, including no-ending settlement.
- [Completion and failure → Owned terminal handoff followed by failed exec](index.html#diagram-failed-exec) — Added attempted-group recording and checked failed-exec recovery under explicit exclusive-lease premises; separated spawn-reported success from confirmed exec.
- [Native entry and lifetime → One-session launcher owns entry through final exit](index.html#diagram-native-entry) — Applied the agreed CLI lifetime, pre-runtime capture, child-watching prerequisite and final exit boundary.
- [Required failure settlement](../../specs/result-job-completion.md#required-failure-settlement) — Specified resource owners, original failures, cancellation and secondary diagnostics for the four known gaps.
- [One-session native launcher entry](../../specs/result-job-completion.md#one-session-native-launcher-entry) — Separated child transparency from restoring a continuing host and defined signal/terminal authority through exit.
- [Repair evidence and discriminating checks](../../completion-failure-design-evidence.md#discriminating-checks) — Named exact enabling points, fault controls, protected expectations and actual fixture cleanup still owed.

Unchanged context remains in [current ownership](index.html#diagram-ownership),
[nominal settlement](index.html#diagram-settlement),
[acquisition limits](index.html#diagram-acquisition) and the
[source inventory](../../result-job-source-evidence.md#acquisition-and-stopping-state-inventory).

Mermaid 12.0.0 is loaded by the reused static viewer; rendering needs network
access. `task docs:result-job` is the presentation recipe; there are no generated
SVG assets. Existing executable seams and inherited observations are described
by the [component guide](../../components.md). Rendering is not a preservation
or comprehension check.

Presentation checks and their viewing limits are recorded in the design task.
They confirm delivery of the diagrams, not the behavior of the executable or
human comprehension. Product source and behavioral checks remain separate.
