# a-complete-session-k84


## Goal

Proof chapter 3, `docs/book/src/03-a-complete-session.md`, as a whole under
`docs/book/contract.md`, including its first complete example and both new
figures. Check the learning route from chapters 1–2 and direct figure entries.



## Context

- Art k83 adds `successful-session` and `session-outcomes`, each editable
  PlantUML beside its generated SVG. Shared `_uml-style.puml` extends the
  opening unit's vocabulary; `docs/book/figures.md` records conventions and
  actual viewing coverage. Both art handoffs in the node brief are closed.
- Preserve the successful sequence's admission boundaries: the worker changes
  to `/` before its identity greeting; front judgement, confirmed worker reap
  and private-directory removal precede executable resolution and launch.
  Created lifelines begin at creation. The runner is inside the native front.
  Publication goes to temporary files; group stops precede direct-child reap;
  terminal recovery, group absence, result validation/read, temporary cleanup
  and settlement precede the final write and flush.
- The outcome machine groups cleanup operations, without claiming that an
  attempted cleanup succeeded. Its exits name the primary settled outcome;
  cancellation wins, original failures remain primary, and additional cleanup
  reports survive. Empty publication differs from absence, a later nonzero
  managed exit does not alone invalidate a value, and the final write can
  fail after cleanup or be interrupted after the settlement decision. The
  full algorithm and detailed timing remain chapters 8 and 9.
- Art's final checks passed: `task docs:figures`, `task docs:book:build` and
  `task docs:examples` (one complete example, one installation fragment not
  run). Source/tool digests were unchanged across that measurement. Art used
  no in-session reviewer.
- Native Chrome inspection covered both final figures in the locally served
  book on Auto's light page and Navy, with surrounding notation, captions and
  alternatives. Chapter 3's editor preview, narrow viewports, GitHub and the
  published site were not inspected. The TestAnyware guest supplied no visual
  evidence because its screenshots failed; it was stopped. No comprehension
  result follows from the visual checks.

## Done when

- The whole chapter meets its contract and the node's Done when, with source
  verification of restated behaviour and all three chapter gates passing.
- Inspect the declared viewing environments that are reachable and name the
  remaining limits; retain the distinction from the owner's later stand-in
  read. Check the examples, figures and their local explanations as entry
  points, not just image/source agreement.
- Route any correction under the editorial family procedure, or retire and
  close `a-complete-session-k63` if its contract is met. Promote only current
  context that later units and the first-half set proof still need.

## Notes

No known defect is handed forward. Chapter 3's original prose and runnable
example were unchanged by art apart from figure teaching and alternatives.

## Decisions (running log)

- Whole-chapter proof found no correction to make. The example, sequence,
  outcome machine and surrounding prose agree on worker admission, publication,
  reap/group/terminal cleanup, cancellation precedence and final output. The
  learning route from chapters 1–2 supplies the terms; both figures have local
  notation, captions, full-size links and prose alternatives. No hand-forward
  entry remains and no correction run is needed.
- Verified the restated rules against both session/completion specifications,
  the three cited ADRs, `run::run`/`settle`, `choice::evaluate_and_resolve`,
  worker teardown admission, `publish_result`, the runner's `watch` and
  `supervise_reported`, terminal lending, worker startup and SDK `definePolicy`.
  The installation task, build wrapper and formula template retain the paired
  layout and installed smoke check. Tier 2 graph discovery/trace/snippets used
  generation `2026-10-16T04:09:35Z`; coverage recorded no gaps in the core Rust
  files. PlantUML sources lacked tracked freshness and were read directly.
- Fresh checks: `task docs:book:build`; `task docs:figures` (four matching
  figures); `task docs:examples` (one passed, zero failed/skipped/page errors,
  one installation fragment not run). SHA-256 checks before/after the figure
  and example run agreed for the chapters, SUMMARY, figure sources/SVGs,
  contract/convention notes, Taskfile, book/figure/example scripts and top-level
  Rust source files in the front and runner. No source or checker was edited
  during that run. These checks establish the declared example claims, not
  foreground-terminal behaviour or reader comprehension.
- Inspected the local mdBook in native Chrome on a light page: the runnable
  example and transcript, both figures at chapter width, their notation,
  captions and prose alternatives. Read the chapter and editable figures raw.
  VS Code's empty window did not respond to file-open/quick-access attempts;
  no editor-preview evidence is claimed. GitHub and the published site remain
  unavailable for this unpublished book; no new theme, narrow-viewport,
  terminal-interaction or human comprehension coverage is claimed.
- `a-complete-session-k63` meets its Done when: chapter 3 is in SUMMARY, all
  four editorial stages have run, its example passes, distinct outcomes point
  to chapter 5 and the success guarantee is explicit. Close that node and
  promote only the forward-link and viewing/evidence limits to the book brief.
