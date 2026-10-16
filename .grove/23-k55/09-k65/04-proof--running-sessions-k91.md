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
