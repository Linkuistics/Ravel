# Result-job completion views

Run `task docs:result-job`, then open
<http://127.0.0.1:8774/#discussion>. The document fills the browser window.
If that port is occupied, use `task docs:result-job PORT=8881` and open
<http://127.0.0.1:8881/#discussion> (this design's presentation).
Graph views start at the dark node labelled **Start here**; sequences start
at their first message. Sources remain editable beside the viewer.

- [Native entry and lifetime → Failed Started transition retains owned teardown](index.html#diagram-entry-failure) — Added the two-second deadline, actual-reap versus uncertainty branches and conditional terminal recovery.
- [Checked native-entry transitions](../../specs/result-job-completion.md#checked-native-entry-transitions) — Made expiry, identity loss and unconfirmed terminal recovery explicit requirements.
- [Bounded error settlement](../../native-entry-transition-design.md#bounded-error-settlement-protocol) — Specified deadlines, checked operations and each protected logical trace.
- [Finite cleanup-failure amendment](../../native-entry-transition-design.md#finite-cleanup-failure-amendment) — Proposed exactly 32 additional rows at existing seams, awaiting commissioning.
- [Complete repair price](../../native-entry-transition-design.md#complete-repair-price-and-handoff) — Repriced the full repair and necessary planning/review/closure; funding remains required.

Unchanged context remains in [one-session ownership](index.html#diagram-native-entry),
[current ownership](index.html#diagram-ownership), [nominal settlement](index.html#diagram-settlement),
[acquisition limits](index.html#diagram-acquisition) and the
[source inventory](../../result-job-source-evidence.md#acquisition-and-stopping-state-inventory).

Mermaid 12.0.0 is loaded by the reused static viewer; rendering needs network
access. `task docs:result-job` is the presentation recipe; there are no generated
SVG assets. Existing executable seams and inherited observations are described
by the [component evidence](../../component-evidence.md). Rendering is not a preservation
or comprehension check.

Presentation checks and their viewing limits are recorded in the design task.
They confirm delivery of the diagrams, not the behavior of the executable or
human comprehension. Product source and behavioral checks remain separate.
