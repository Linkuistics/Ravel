# what-ravel-is-for-and-its-actors-k80

## Goal

Read chapters 1 and 2 under the art charter after the worker-directory
correction, checking all figures and their text against the corrected teaching
and `docs/book/contract.md`.

## Context

This is the first art correction run. K77 delivered `system-context` and
`process-authority` sources/SVGs, shared `_style.puml`, local notation teaching,
full alt text and full-size links; `docs/book/figures.md` records the vocabulary.
Its container figure requires removal of the worker's private directory before
launch. That cleanup rule survives the correction to where policy executes.
The information and nested-publication tables must keep policy data, caller
location and result publication separate.

K77 visually inspected white-canvas raster previews and verified source/SVG
agreement plus local HTTP asset delivery. Computer use had no browser endpoint;
actual mdBook themes and editor previews remain uninspected. Attempt the
declared viewing environments if available and record limits honestly.

Copy-edit k79 reworded chapter 2's container notation paragraph (the C4
container-level sentence, "gold the result channel" and the closing sentence on
what the figure does not show), the nested-publication role sentence, and the
information table's "starts there". Check these against the figure keys.

Proof k81 already stands after this leaf; do not append another proof leaf.
A second art rerun is escalation under the editorial procedure.

## Done when

- Figures and their role sentences agree with the corrected text, with any
  declined figure classified under the art skill's three reasons.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass.
- Handoffs reflect what remains; retire this leaf with the queued proof next.

## Decisions (running log)

- The two C4 figures, their keys and alt text agree with the corrected prose.
  Private-directory removal remains a launch prerequisite, independently of
  where policy executes. Keep the existing diagrams and information and nested
  publication tables.
- Add an in-page sequence table beside the worker-directory explanation: startup
  in the private directory, the move to `/` before policy import, and removal
  during cleanup before managed launch. Preserve the policy's ability to change
  directory. This relation otherwise remains only in running prose.
- Decline a separate worker-directory diagram as an editorial judgement: the
  small sequence fits a table, and the container view already carries admission
  ordering. No contract prohibition or medium limitation was encountered.
- The browser inventory is empty and `iab` unavailable, but native Chrome is
  accessible through computer use. Inspect the actual local book there rather
  than carrying that previous viewing limit forward without another attempt.
- Native Chrome confirms both figures are legible in local mdBook on Auto's
  light page and Navy. Chapter 2's container figure and nested-publication,
  directory-sequence and information tables also render legibly in VS Code's
  light Markdown preview with Restricted Mode retained. Update the handoff to
  retain only the actual uninspected environments; no second art rerun is needed.
- Proof k81 is the live later sibling for this stage, so add no successor.
  The document node stays open for proof. No architectural decision changed.

## Verification

- Read both complete chapters, every figure's role sentence and key, the
  PlantUML sources and shared style against the contract, glossary, session
  specification and ADRs. The new table carries the corrected sequence without
  changing the cleanup prerequisite or conflating policy data with result paths.
- `task docs:figures` passed: both figures match their sources under the pinned
  renderer and fonts. No SVG or PlantUML change was needed.
- `task docs:book:build` passed. Native Chrome displayed the built chapter pages
  and their figures through `task docs:book` at localhost:8775; the new table
  also appears in the browser and editor accessibility trees as a table.
- `task docs:examples` passed: 0 complete examples, 0 fragments and 0 page
  errors. The opening unit requires no runnable example.
- Tier 2 coverage metadata generation `2026-10-16T03:28:19Z` matched its
  coverage record. Document/specification paths had no recorded gap; PlantUML
  freshness was untracked and SVGs excluded. Read the PlantUML directly and
  inspected the rendered SVGs in the actual local book. No runtime or whole-
  codebase claim is made.
- Remaining viewing limits are recorded in the document brief and figure note:
  Rust/Coal/Ayu, narrower viewports, chapter 1's editor preview, GitHub and the
  published site. Visual inspection is not a human reader checkpoint.
