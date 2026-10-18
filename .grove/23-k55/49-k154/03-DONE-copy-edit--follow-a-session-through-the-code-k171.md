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

## Decisions (running log)

- Inputs read: the brief chain, `docs/book/contract.md` (sections 1 and 2),
  `docs/book/ste.md`, `docs/book/excerpts.md`, `CONTEXT.md`, both draft logs
  (k155, k170) and the whole chapter. Chapter 3 is the precedent for “the
  fake” as the short noun after “fake agent”, so the chapter keeps it after a
  first “fake agent”.
- Before any edit, strict STE and strict prose breaks on chapter 8 already
  passed. The defects are those a script cannot find: first-use links,
  unqualified “context”, an undefined term, demonstratives, a hedge, an
  italic definition and the phrasal verb in a heading.
- First-use links (the node handoff). “Managed process” is now linked at its
  first use in “Trace the example”, and the later link in “Prepare the managed
  launch” is removed. The same read found four more glossary terms whose first
  use had no link: policy context and confinement (the scope paragraph), kind
  (“Prepare the request”) and refusal (“Follow a refusal or launch failure”).
  Each now links to its chapter 2 section. The later policy-context and
  chapter 5 confinement links stay as depth.
- Heading. “Clean up and write the result” is a phrasal verb (Rule 9.3). It
  is now “Complete cleanup and write the result”, which agrees with the path
  table's row “Complete cleanup and decide”. The section keeps its function.
  No link in the book used the old anchor (searched `docs/book/src`, `docs/`
  and `README.md`).
- “Context” is never unqualified. The worker table, the bound row and the
  context exchange now say “policy context”. “Old result-context variables”
  is now “the inherited variables that name a result destination”, because
  `run.rs` removes both `RAVEL_EXIT_FILE` and `HARNESS_DISPATCH_EXIT_FILE`.
  The storage table's “fresh nested context” is now “a fresh session-result
  context for each launch”.
- “Transport” was an undefined term. The table header and two sentences now
  name `result_after` and the result channel. “Rust coordinator” was a second
  name for “worker coordinator”. The chapter now defines the worker
  coordinator as the code in `worker.rs`, so the refusal table and the
  cancellation section use one defined term.
- Hedge with no condition: “A group that survives normally makes
  `result_after` fail first”. Source (`run.rs` 220–228, `completion.rs`
  96–99) shows that each `Ended` reaches `result_after`, and a present group
  is always `session_result_invalid`. The sentence now states that condition.
  This is a wording fix inside the draft's claim, not a new technical claim.
- Smaller fixes: “This is an orientation” and “This is an order of sources”
  get nouns. “Begins cancellation handlers” names `handlers.begin()`.
  “Reports its operation reports” is now “writes the runner's operation
  reports”. *latch* is bold at its definition. “Here” is now “In the
  example”. “Wins” is now “Ravel keeps” (`ste.md`). “Since” is now “because”.
- No excerpt, code block, alt text, caption or figure changed. Art owns the
  figure text and its adjacent prose. Outcome order, teaching and structure
  did not need a correction run.

## Verification and limits

- After the last chapter edit, all exited 0: `task docs:book:build`,
  `task docs:ste -- --strict` on chapter 8, `SUMMARY.md` and `CONTEXT.md`
  (0 problems), `task docs:breaks -- --strict` on chapter 8 (no long run),
  `task docs:excerpts` (8 passed), `task docs:examples` (13 passed, no
  failures, skips or page errors) and `task docs:figures` (10 match).
- Controls: a scratch copy with a planted semicolon made strict STE exit 1.
  All 15 chapter link targets exist, with anchors checked against the built
  HTML, and a planted absent anchor was reported missing.
- No reader comprehension, rendering view or complete-source claim follows.
- The node's copy-edit handoff is cleared from its brief. The art handoffs
  stay. No live later sibling holds art, so this leaf cuts it.
