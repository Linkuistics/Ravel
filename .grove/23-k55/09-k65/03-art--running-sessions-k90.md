# running-sessions-k90

## Goal

Produce chapter 5's figures (`docs/book/src/05-running-sessions.md`) under the
figure contract in `docs/book/contract.md` and the layout and regeneration
procedure in `docs/book/figures.md`, reusing the visual vocabulary chapters
1–4 fixed.

## Context

- The node brief's `Handed forward` entry for art is this leaf's charter: two
  UML sequences at "Cancel during selection or supervision" (selection:
  worker teardown, no managed launch; supervision: cleanup of the managed
  process's group and result suppression after publication), and chapter 3's
  outcome state machine extended or reused beside "Interpret the exit
  status".
- Copy-edit (`running-sessions-k89`) fixed the chapter's spellings. Use them in
  captions and alt text: "the managed process's group" (never "managed group"),
  `end-session` (never "the publisher"), "lend the terminal", "routing notice",
  "result channel", "native front" then "the front". The state machine's
  outcome labels should match the status table's rows: 0, 1, 2, 3, 5, 124,
  126, 127 and the three cancellation statuses.
- "Interpret the exit status" now opens with a paragraph of local context and
  a paragraph linking each status to where it is taught; place the state machine so that
  the table stays the direct entry a reader arrives at.
- Distinguish failed cleanup attempts from confirmed cleanup, original-failure
  precedence, final-write failure, and publication from delivery, as the
  handed-forward entry asks. The surrounding prose already supplies the
  technical sequence; art owns notation, captions, alt text and the rendered
  views.

## Done when

- The figures exist with committed sources and SVGs, `task docs:figures` (or
  the figure check `figures.md` names) regenerates and matches them, and the
  chapter references them with alt text.
- `task docs:examples` and `task docs:book:build` pass.
- The node brief's art entry is cleared, and the proof leaf is queued with bare
  stem `running-sessions` unless a live later sibling already holds it.

## Notes

Do not mechanically duplicate chapter 3's asset or silently change its
teaching when extending it. Record viewing coverage and limits in
`docs/book/figures.md` as earlier units did.
