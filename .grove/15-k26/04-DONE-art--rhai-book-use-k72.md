# rhai-book-use-k72

## Goal

Make the figures of Parts I and II agree with the copy-edited text of chapters 1–6.
Keep one visual style across the book, as section 3 of `docs/book/contract.md` states.

## Context

- Structure brief and prose contract: `docs/book/contract.md`. Figure mechanics: `docs/book/figures.md`.
- Read the node brief's `## Handed forward` list. Its art entries name the figures to change:
  `selection-cancellation.typ`, `process-authority.typ`, `source-components.typ` and `installed-files.typ`.
- Copy-edit `rhai-book-use-k69` fixed one name for each term in chapters 1–6.
  A figure label uses the same words as the text:
  - "policy worker", not "Bun worker" or "Rhai worker".
  - "harness run" for a run during evaluation, and "final managed process" for the selected program.
  - "host call" for a request of a host capability.
  - "standard output", "standard error" and "standard input" in text. Code font keeps `stdout` where it names a field.
- The alt text of `source-components.svg` and `installed-files.svg` in chapter 6 says "internal Rhai worker role".
  Copy-edit did not change it, because the alt text describes the figure. Update the alt text with each figure.
- Chapter 5 states that the fake evaluation harness sends INT to the Ravel CLI. The policy itself cannot send a signal.
  The current selection-cancellation figure and its alt text show the policy worker as the sender.

## Done when

- Each figure in chapters 1–6 agrees with its alt text, its caption and the prose beside it.
- The handed-forward art entries are closed and cleared from the node brief.
- `task docs:figures` passes. `task docs:ste -- --strict` passes on each changed `.layout.json` and chapter.
- `task docs:book:build` succeeds.
- Work outside this charter follows the editorial family's handoff or correction procedure.
- The proof stage exists under this document node, with the bare stem `rhai-book-use`.
  Cut it only if a live later sibling does not already hold it.

## Notes

Proof cuts the Read 1 requirements leaf. The owner did not waive either reader checkpoint.

## Decisions (running log)

- Keep the existing Typst/CeTZ renderer, shared style and page-native tables.
  The chapter tables already compare evaluation calls with the final managed process.
  A separate model-call diagram would repeat those relations. This is an editorial choice, not a restriction or failure of the medium.
- Update the four handed-forward figures, their alternatives and their captions together.
  The cancellation sequence shows the fake evaluation harness as the signal sender and places its cleanup before worker teardown.
  The installed view names no notice path before the delivery increment settles it.
- The whole-document read found two further figure defects.
  Remove the old worker's change to `/` from the successful sequence.
  Use “standard output” in the outcome figure, as the book's word decisions require.
- The graph CLI cannot start because an active generation is incompatible with its coordinator.
  Source verification uses direct reads of the relevant Rust files, with no graph completeness claim.
- The process-authority view omits evaluation harness runs.
  It separates the policy-response path from the final opaque-result path.
  Its key and chapter alternative state this omission.
  The source-components view and selection-cancellation sequence draw the harness role explicitly.
  This is an editorial choice, not a medium restriction or failure.
- The final-outcome key now states the evaluation exception: failed harness cleanup can replace a deadline with status 5.
  `rhai_worker::Host::settle` verifies that precedence.
  Direct reads also verified the internal worker role, its private starting directory and the synchronous host-call sequence.
- Chapter 8's old example and alternative no longer match the shared cancellation figure.
  Its already scheduled draft owns the rewrite. The Parts III and IV node brief now records this exact dependency.

## Viewing evidence

Chrome on macOS showed the seven revised figures in the local light mdBook on 2026-10-26.
The window screenshot was 1200 by 2085 pixels, with the chapter column about 750 pixels wide.
The inspection covered the diagrams and their keys in chapters 2, 3, 5 and 6.
No label overlap, clipped key or missing figure was visible.
The final-outcome full-size link opened the complete SVG.

The following sizes use ordinary 14-unit text and each SVG's viewBox at a 750-pixel image width.
C4 qualifiers retain their smaller shared size.

| Figure | Body text, px | SVG bytes |
|---|---:|---:|
| `installed-files` | 16.4 | 135,058 |
| `process-authority` | 12.7 | 275,752 |
| `selection-cancellation` | 16.7 | 217,619 |
| `session-outcomes` | 12.5 | 182,780 |
| `settled-exit-status` | 13.1 | 205,828 |
| `source-components` | 13.8 | 254,859 |
| `successful-session` | 12.8 | 218,283 |

The fourteen SVGs total 3,309,375 bytes.
This retains the earlier whole-book decision to accept the set above its old 2 MB benchmark.
No renderer, font, notation or shared-style change was necessary.
Regeneration remains `mise exec -- task docs:figures:update` with the pins in `docs/book/figures.md`.

Raw Markdown supplied the whole-document read of chapters 1–6, with tables, captions and alternatives.
No fresh editor preview, dark theme, narrow viewport, GitHub view or Pages site was inspected.
These checks establish no new platform result, real-harness result or reader comprehension.
Both owner reader checkpoints remain required.

## Verification

- `mise exec -- task setup` completed with the pinned tools.
- `mise exec -- task docs:figures` reproduced all 14 figures and their layout records.
- Strict `docs:ste` passed all six chapters and the seven changed layout records, with no problems.
- Strict `docs:breaks` passed chapters 1–6. The caption addition in chapter 2 stays within the 450-word prose limit.
- `mise exec -- task docs:book:build` passed without a warning.
- `mise exec -- task docs:excerpts` passed all nine excerpts.
- SHA-256 digests stayed unchanged across these final checks for 147 files under `docs/book` and `scripts`, plus `Taskfile.yml` and `CONTEXT.md`.
- The general prose checker passed the changed chapters and task briefs with no hard finding.
  The unpublished figure-mechanics note still fails that broader checker across its older instructions and historical records.
  This art pass updates its current figure meanings only and makes no claim that the whole note passes that checker.
- The runnable example code did not change. This pass did not repeat real-harness smoke calls or the example execution suite.
