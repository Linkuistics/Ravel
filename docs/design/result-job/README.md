# Result-job completion views

Run `task docs:result-job`, then open
<http://127.0.0.1:8774/#discussion>. The document fills the browser window.
If that port is occupied, use `task docs:result-job PORT=8879` and open
<http://127.0.0.1:8879/#discussion> (this design's presentation).
Graph views start at the dark node labelled **Start here**; sequences start
at their first message. Sources remain editable beside the viewer.

- [Native entry and lifetime → One-session launcher owns entry through final exit](index.html#diagram-native-entry) — Added checked watching, selection-failure ownership, fallible Started and common finalisation.
- [Native entry and lifetime → Failed Started transition retains owned teardown](index.html#diagram-entry-failure) — Added real bounded cleanup after a failed parent-mask transition with a spawned child.
- [Checked native-entry transitions](../../specs/result-job-completion.md#checked-native-entry-transitions) — Stated required failure, shielding and settlement guarantees.
- [Ownership and transition protocol](../../native-entry-transition-design.md#transition-protocol) — Assigned every checked transition and its failure owner.
- [Frozen discriminator manifest](../../native-entry-transition-design.md#frozen-discriminator-manifest) — Defined exact entry, signal, failure and protected-case parameters with physical receipts and broken controls.
- [Complete repair price](../../native-entry-transition-design.md#complete-repair-price-and-handoff) — Priced the full incumbent repair and recorded the focused review's scope.

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
