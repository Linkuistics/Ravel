# follow-a-session-through-the-code-k170

## Goal

Complete part B of chapter 8 in
`docs/book/src/08-follow-a-session-through-the-code.md`, under the named
structure brief and contract `docs/book/contract.md`. Start from part A's
successful-path map. Teach the refusal and cancellation forks, error/signal
precedence, and where an independent change belongs.

## Context

- Load `linkuistics:writing-code-walkthroughs` at orientation depth. Source
  remains authoritative; the guides can be stale. No full-source fragment graph.
- Append these reserved level-two headings in order: “Follow a refusal or
  launch failure”, “Follow cancellation”, “Choose the final outcome”, “Find
  where a change belongs”. Part A's eight sections stand; fill the reserved
  scope without restructuring them or adding empty headings.
- Refusal/launch forks: policy refusal; refusal of the policy file, behaviour
  or data; selection timeout; missing/unexecutable selected program. Keep their
  classifications distinct, including timeout versus policy refusal. Locate
  the decision and the point where managed launch stops.
- Cancellation forks: during selection, during supervision and after a result
  was sent. Trace `run`, `settle`, `supervision_failure` in
  `crates/ravel/src/run.rs`, and `Handlers::finalise`/propagation in
  `crates/ravel/src/cancellation.rs`. **`join_cancellation` is test-only in
  this checkout**; do not teach it as the production join. Preserve the final
  decision versus later output-write window and no-atomic-output guarantee.
- Use the outcome-precedence activity figure from k153, checking its claims
  against the source. Reuse chapter 5's `selection-cancellation` and
  `supervision-cancellation` figures unless the code path needs another.
  Hand new figure work to art through the node brief.
- Verify “An independent-change scenario” in `docs/components.md` and teach
  where the change belongs. Do not present an undelivered component or public
  reuse contract. Chapter 10 belongs to future component work.
- K155's log holds the complete O1–O10 destination table, exact guide statement
  groups and successful-sequence art contract. Finish O3, O4, O5, O7 and O9
  where it marks part B. O6/O8 detail belongs to chapter 9. Enumerate the ten
  obligations again, with final section destinations and explicit limits.
- Part A teaches `ravel: <report>` on stderr in its final section. The
  second-half set proof still decides chapter 6's entry.
- Part A's narrow reviewer corrected worker frame ownership (`converse`
  sends frames returned by `load`/`select`) and terminal order (child pre-exec
  handover, parent `lend` inside `watch` after the start check). Preserve both.
  Its real chapter 3 run was nonterminal, and it claimed no runtime trace of
  every internal event. Guide native-entry status is historical, not current.

## Done when

- The reader can name the production function and rule that choose the final
  cancellation outcome, and can locate a concrete independent change.
- The whole chapter passes `task docs:book:build`, strict STE,
  `task docs:breaks -- --strict`, `task docs:examples` and `task docs:excerpts`.
  Every source excerpt names its file under `docs/book/excerpts.md`.
- Guide statements absorbed by part B and final obligation destinations are
  in the log; art needs are in the node brief, with participants and order.
- The successful path remains intact, scope exclusions remain explicit and
  no complete-source or reader-comprehension claim appears.

## Notes

Copy-edit k171 is already the later live sibling. Do not add another copy-edit
leaf. The ordinary one-reviewer allowance is this leaf's, with any escalation
through the editorial family. The node remains live through copy-edit/art/proof.
