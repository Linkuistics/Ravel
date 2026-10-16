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

## Decisions (running log)

- One spelling per term, matching chapters 1–4: "the managed process's group"
  replaces "managed group" and "managed foreground group"; `end-session`
  replaces "the publisher"; "fake agent" replaces "the fake"; "lend" (chapter
  3's word) replaces "borrow"; "routing notice" names the text-form starting
  line as well as the JSON notice (`run.rs` prints the text line before
  launch, the JSON notice after supervision); "result channel" replaces bare
  "channel".
- Term-list words linked at first use on the page: native front moved to its
  first use (confinement read grants), result channel, runner, refusal.
  Reaping is linked to chapter 3's definition instead of being re-defined here.
- Figurative or uncheckable wording replaced by mechanism, each checked
  against source: the scrubbed launch variables named from `run.rs` (instead of
  "old launch controls"); "ending" stated as the runner's account of how the
  job ended, with `operationReports` absent exactly when supervision failed
  before it; "unchanged-text promise", "cleanup promise has a defined
  boundary", "debris", "another full grace", "authenticated notice", "session
  protocol", "remains primary" and the worker identity "pairs builds" each
  restated as what happens.
- Status-2 conditions (invalid prompt, unusable backend, overlaps) are no
  longer called refusals, which the term list reserves for status 3.
- Slash pairs in the status table written out; British spelling unchanged.
- The status table gains one sentence linking each row's status to where it
  is taught, which completes the contract's direct-entry rule (local context
  and a link back) for exit codes. No example, label or expected output
  changed.
- Gates: `task docs:examples` 13 passed, 0 failed, 0 skipped, 0 page errors;
  `task docs:book:build` built without warnings; all 33 anchor links in the
  built chapter resolve. No in-session reviewer was spent. Nothing is handed
  back to draft.
