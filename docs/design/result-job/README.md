# Result-job completion views

Run `task docs:result-job`, then open
<http://127.0.0.1:8774/#discussion>. The document fills the browser window.
If that port is occupied, use `task docs:result-job PORT=8874` and open
<http://127.0.0.1:8874/#discussion> (the current session's checked view).
Graph views start at the dark node labelled **Start here**; sequences start
at their first message. Sources remain editable beside the viewer.

- [Ownership → Consumer pairing versus a completion owner](index.html#diagram-ownership) — Named concrete failure/native limits and stacked current and proposed views to fit the desktop document.
- [Completion and failure → Publication, ending and cleanup join before return](index.html#diagram-settlement) — Restricted the sequence to the current nominal front and included output reservation and the final write.
- [Completion and failure → Acquisition and no-ending failure limits](index.html#diagram-acquisition) — Added acquisition states and the distinction between an ending and an error that discards required facts.
- [Native entry and lifetime → Child entry state does not restore the host](index.html#diagram-native-entry) — Added persistent host signal changes and the separate CLI guard lifetime.
- [Acquisition and entry contract](../../specs/result-job-completion.md#acquisition-and-entry-limits) — Recorded supported enabling points and the limits of complete settlement.
- [Source acquisition inventory](../../result-job-source-evidence.md#acquisition-and-stopping-state-inventory) — Added stopping-state ownership, counterexamples and the cheapest discriminating observations still owed.

Mermaid 12.0.0 is loaded by the reused static viewer; rendering needs network
access. `task docs:result-job` is the presentation recipe; there are no generated
SVG assets. Existing executable seams and inherited observations are described
by the [component guide](../../components.md). Rendering is not a preservation
or comprehension check.

The four views were inspected in Chrome's default light desktop view. The
acquisition link also opened correctly in a fresh page after rendering. A
390-pixel native-entry preview kept readable labels but needs scrolling;
the complete narrow-screen document and dark appearance were not qualified.
No mobile usability or human comprehension result is inferred.
