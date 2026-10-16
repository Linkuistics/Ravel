# running-sessions-k91

## Goal

Proof the whole of chapter 5, `docs/book/src/05-running-sessions.md`, under
`docs/book/contract.md`, using the proof stage's full charter.

## Context

Read the document-node brief and the contract, not just the preceding stages'
changes. Chapter 5's direct entries include its exit-status table and figures.
Re-enumerate the relevant CLI outcomes from the subject as the contract requires.

Art added the cancellation sequences `selection-cancellation` and
`supervision-cancellation`, and `settled-exit-status`, a conceptual refinement
of chapter 3's unchanged outcome view. The status table remains before the
new view. Four page-native tables carry path resolution, nested result
context, launch-mode comparisons and stdout/stderr routing. Figure layout,
regeneration and the precise viewing coverage/limits are in
`docs/book/figures.md`.

Art's seven SVGs matched their sources, all thirteen complete book examples
passed (none failed or skipped; six labelled fragments not run), and the book
built without warnings. The chapter's fenced examples and labels are unchanged
from copy-edit. One bounded in-session semantic review reported no findings.
These are mechanical/source/visual results, not a human comprehension read or
terminal-interaction evidence. No earlier-stage correction run is queued.

## Done when

- The whole document meets the contract, including source-derived outcome
  coverage, local context, terminology, figures and working links.
- Inspect reachable viewing environments and record the exact scope and
  unavailable surfaces in `docs/book/figures.md` without promoting earlier
  visual checks into this proof's evidence.
- `task docs:figures`, `task docs:examples` and `task docs:book:build` pass.
- Route any corrections according to the editorial-family procedure; retire
  and commit, with the normal node-close check if no live child remains.

## Notes

The owner's stand-in read remains a later requirements leaf. Native x86-64
qualification, remote GitHub rendering and publication keep their declared
limits. Proof reads the entire document; this body does not narrow its charter
to art's changes.

## Decisions (running log)

- Whole-document source read retained the outcome teaching and all three
  figures. The normal statuses re-derived from the front are 0, 1, 2, 3, 5,
  124, 126 and 127, plus handled HUP/INT/TERM shell statuses 129/130/143.
  `main.rs`, `refusal.rs`, `program.rs`, `run.rs` and `cancellation.rs`
  establish their separate parsing, resolution, Started, settlement and
  signal-propagation meanings. Test-only process exits are not CLI outcomes.
- Corrected two local prose-contract details: link the first use of native
  front in the nesting section, and set literal status values in code font
  rather than decorative bold. Neither correction changes example bytes,
  behavioural claims or figure sources; no earlier-stage re-run is needed.
- Independent Chrome inspection covered all three figures at chapter width,
  two full-size SVGs, the chapter's seven tables and their explanations on a
  light page. Raw chapter/PlantUML reading and the unavailable editor preview,
  remote views and remaining theme/viewport limits are recorded separately
  from art in `docs/book/figures.md`. The owner's later read remains unknown.
- Initial required checks passed: seven figures match, thirteen complete
  examples pass with none failed/skipped and six fragments not run, and the
  book builds without warnings. All 22 distinct local link targets/anchors
  resolve in source and built HTML; the external issue link returns HTTP 200.
  A final frozen-input run follows the evidence-note edits. No product source
  changed and no new terminal or platform qualification is claimed.
- Tier 2 graph generation `2026-10-16T05:22:01Z` had no recorded gap in the
  relied-on Rust files; this is best-effort metadata, not completeness proof.
  PlantUML sources had no tracked freshness and SVGs were excluded by suffix:
  direct raw-source reading, actual rendering and regeneration supply that
  evidence instead. No in-session reviewer was used.
- The frozen-input run passed examples again (the same thirteen examples,
  none failed/skipped; six fragments not run) and the warning-free book build,
  but figure comparison failed for `selection-cancellation.svg`. Before/after
  SHA-256 inventories of `docs/book`, `scripts`, `crates` and `.cargo` files
  (excluding ignored dependencies/build outputs), plus Taskfile.yml,
  Cargo.toml and Cargo.lock, were identical. The failed figure check discarded
  its scratch render; its differing bytes are unknown.
- Systematic investigation retained two direct full renders and six subsequent
  full checks, all matching all seven committed SVGs; the two retained
  selection SVGs match every element and byte. No source, SVG, pin or renderer
  command changed. Root cause remains unknown, so green retries do not settle
  reproducibility. Send back one art correction followed by proof, carrying
  this exact case. This supersedes the earlier no-re-run decision; the chapter
  node stays open. The ordinary required task is checked once more before
  retirement, without treating a pass as resolution of the intermittent failure.
- The final ordinary `task docs:figures` passed with all seven sources
  matching. The required tasks are currently green, but the unexplained failed
  run remains owned by the correction run, not waived. There is no handed-
  forward list to clear and no other unresolved chapter defect from this read.
