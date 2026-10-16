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

## Decisions (running log)

- Keep chapter 3's overview unchanged. Chapter 5 adds a UML refinement of
  settlement, making cancellation, original-failure and cleanup precedence
  visible with the status numbers; it is a conceptual outcome view, not a
  claim about function-call order. Keep the status table before this view.
- Draw separate selection and supervision cancellation sequences. Lifelines
  have no destruction marks on attempted teardown: an unsuccessful wait or
  removal cannot certify the end of a lifetime. The supervision view shows
  publication first and no result delivery after cancellation.
- Use page-native tables for program-path resolution, nested result authority,
  confined/unconfined launch differences and stdout/stderr routing. These are
  relations otherwise carried only by prose. Decline extra process drawings
  for these relations as an editorial judgement: tables allow direct comparison
  and chapter 2 already supplies the process view. Defer the detailed shutdown
  timing diagram to chapter 9 as an editorial judgement about teaching depth;
  chapter 5 retains its timing prose. Neither decision is a medium limitation
  or a contract prohibition.
- Read the session and completion specifications and the three ADRs against
  the new diagrams. Tier 2 coverage for project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-16T05:11:14Z`, reports matching Markdown metadata and no recorded
  gaps for the chapter and contracts. PlantUML freshness is not tracked;
  those files and the shared styles were read directly. No product-source
  or runtime completeness claim follows from this document check.
- Spent the leaf's single fresh-context review on whether the diagram and
  caption semantics misrepresent lifetime, cleanup, cancellation or status
  precedence. The reviewer reported no semantic contradictions; there were
  no findings to reconcile. Mechanical label improvements after viewing
  separate the cancellation statuses, say that a result channel is allocated,
  and retain the chapter's “result channel” spelling.
- All three diagrams and the four comparison tables were inspected in the
  local mdBook through Chrome on a light page. The final SVGs were inspected
  again after refreshing cached assets; viewing limits are recorded in
  `docs/book/figures.md`. The temporary server and browser tab were closed.
- `task docs:figures docs:examples docs:book:build` exits 0: seven SVGs match,
  all thirteen complete examples pass individually, none fail or skip, six
  fragments are not run, and mdBook builds without warnings. SHA-256
  manifests before and after match for all `rg --files` entries under
  `docs/book/src`, `scripts` and `crates`, plus `Cargo.toml`, `Cargo.lock`,
  `Taskfile.yml` and `docs/book/book.toml`. All 43 fenced blocks, eight example
  introduction lines and 41 file/output labels match the parent chapter.
- The art handoff is closed. No earlier-stage correction or architectural
  decision was earned; no later proof sibling is present. Queue proof with
  the bare document stem as the last work act. The document node and its
  ancestors remain open because proof and later book work remain live.
