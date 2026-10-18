# follow-a-session-through-the-code-k171

## Goal

Read and copy-edit the whole of chapter 8,
`docs/book/src/08-follow-a-session-through-the-code.md`, after both draft
parts, under `docs/book/contract.md` and `docs/book/ste.md`.

## Context

- The node brief carries the later-stage handoffs, including first-use term
  links, the final part A heading and the successful sequence for art.
- The chapter assumes Rust and TypeScript knowledge. Keep the source identifiers
  exact, while the prose remains intelligible without fluent Rust reading.
  Explain the work of `settle` in plain words; it chooses from collected
  cleanup outcomes rather than performing cleanup itself.
- Five exact excerpts stood after part A. Part B can add more. Keep each
  excerpt and its `Excerpt from` label under `docs/book/excerpts.md`; a prose
  edit cannot silently change a quoted source line.
- Part A's technical corrections distinguish worker `converse` from its
  frame-producing helpers and child-side pre-exec terminal handover from the
  parent's `lend` callback after the start check. Preserve those distinctions.
- Outcome order, teaching, missing facts and structure belong to draft. Use
  the editorial correction procedure for such defects, not an inline rewrite.

## Done when

- The whole chapter meets the prose contract, with consistent terms, first-use
  links, clear source introductions and no unexplained specialised wording.
- Book build, strict STE, strict prose-break check, examples and excerpts pass.
  Reconcile changed anchors and clear the handoffs that this stage resolves.
- The art leaf is added under the editorial family rule if not already queued.
  Its body carries the node's figure needs, including the full successful
  sequence and part B's precedence/cancellation figures.

## Notes

This leaf follows part B, not just part A. Art and proof still follow once for
the whole chapter. No claim of complete source coverage or reader comprehension.
