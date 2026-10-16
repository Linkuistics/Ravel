# running-sessions-k89

## Goal

Copy-edit the whole of `docs/book/src/05-running-sessions.md` under the prose
contract in `docs/book/contract.md`, preserving its structure and technical
teaching. It is linked from `docs/book/src/SUMMARY.md`.

## Context

- The preceding draft is `running-sessions-k74`. Its task records the
  source-derived status/outcome inventory and the one narrow read-only review.
- Read the document brief's `Handed forward` section. Its cancellation
  sequences and outcome state-machine work belong to art, not this stage.
- The chapter has six complete examples: prompt-file bytes/caller data;
  published/empty/missing outcomes; nesting; confinement; cancellation during
  selection; cancellation after publication. Preserve exact output labels,
  blank blocks for zero bytes, `Standard error includes` for stable excerpts,
  and the confinement requirement marker. `<scratch>` is the checker's sole
  normalisation, explained beside the selection-cancellation transcript.
- The direct-entry status table carries local context and actions. Keep the
  distinction between publisher success and settled session success, real
  signal termination versus its shell number, and terminal Ctrl-C reaching
  the managed foreground group versus a signal delivered to the front.
- `--json` changes notices, not the whole stderr stream; the routing notice
  precedes final settlement and can carry `operationReports`. Preserve those
  qualifiers and the possibility of a partial final write on failure.
- Draft validation: 13 complete book examples passed, no failures/skips or
  page errors; mdBook built without warnings; all 27 local anchor links in the
  built chapter resolved. No visual or human reader result is implied.

## Done when

- The complete chapter meets the prose contract within copy-edit's charter;
  any earlier-stage defect is routed through the editorial correction rule.
- `task docs:examples` and `task docs:book:build` pass on the final document.
- Last act: queue the art stage with bare stem `running-sessions` unless a
  live later sibling already holds it, under the editorial family rule.

## Notes

Chapters 1–4 are already proved. Preserve their terminology and backward
anchors. Chapter 6 is still unwritten, so its mention stays plain text until
its page exists. The node's coverage/migration notes serve the later guide
fold-in; this stage removes no guide. The prescribed art, proof and owner's
stand-in read are still outstanding.
