# follow-a-session-through-the-code-k172

## Goal

Read the whole of chapter 8,
`docs/book/src/08-follow-a-session-through-the-code.md`, after copy-edit k171,
and own its figures and their adjacent prose under `docs/book/contract.md`,
`docs/book/figures.md` and `docs/book/ste.md`. Draw the full successful
sequence, and reconcile the three reused figures with the chapter's code path.

## Context

- The node brief's `## Handed forward` holds the two art entries in full. Read
  them first. In short:
  - **New figure: the full successful sequence.** Put it after the table in
    “Trace the example”, which is its low-resolution map. Participants:
    caller, Ravel CLI (the runner calls are inside this process, so do not
    draw a separate runner process), policy worker, managed process, the
    second CLI invocation for `end-session`, and the result channel. The
    precise messages and their order are k155's “Art sequence contract”
    (eleven steps). Keep the child-side terminal handover before exec separate
    from the parent's `lend` callback inside `watch`, after the start check.
    Keep an optional grace/escalation region, because the example can exit
    before the runner observes the result. The final channel appearance check
    in `supervise_reported` still finds it.
  - **`outcome-precedence`** (embedded in “Choose the final outcome”). Its
    signal step omits the second CLI latch read of `Handlers::finalise`
    (`front`, `outcome`, `late`, `received()`, `pending`). Reconcile the
    figure and the prose after it without a suggestion of arrival-time order.
    Assess its report-before-decision abstraction: `settle` reports an
    original failure under cancellation before the cleanup and group reports.
    Keep every diagnostic and the difference between attempts and confirmed
    cleanup. Its success label “exact result” uses the shared word
    “unchanged” (`ste.md`, R12).
  - **`selection-cancellation` and `supervision-cancellation`** are chapter
    5's figures, reused beside their code paths in “Follow cancellation”.
    Chapter 9 reuses them too. Keep chapter 5's direct-signal examples and the
    limits of cleanup attempts. A change to a shared figure changes chapter 5
    as well, so check both pages.
- Copy-edit changed these terms near the figures. Keep the figure text in
  agreement: “worker coordinator” (the code in `worker.rs`), “policy context”
  (never bare “context”), “Complete cleanup and write the result” (the
  heading, formerly “Clean up …”), and **latch** as the saved signal number.
- The chapter uses the crisp style and the activity module from
  `book-figure-activity-k153`. Each figure has a `.typ` source, a committed
  `.layout.json` record and an SVG. Its labels, key, caption and alt text
  follow the STE writing rules. `task docs:ste` examines the layout records.
- Readers know the diagram types. Explain only conventions that are specific
  to the book. Dense small text across the book stays with whole-book art
  `ravel-book-k101`.

## Done when

- The full successful sequence exists after “Trace the example”, with alt
  text, a full-size link and adjacent prose that agree with the chapter's
  table and k155's sequence contract.
- `outcome-precedence` agrees with the `finalise` expression and with
  `settle`'s report order, or the adjacent prose states the abstraction
  exactly. The two cancellation sequences agree with the chapter 8 text and
  still agree with chapter 5.
- Book build, `task docs:figures`, strict STE (chapter 8 and changed layout
  records), strict prose breaks, excerpts and examples pass. The figures load
  in the served book at chapter width, and an absent-image control fails.
- The node brief's art entries are cleared. Proof is cut under the editorial
  family rule if no live later sibling holds it.

## Notes

A structural or teaching defect found here goes back to draft by a correction
run under the editorial procedure. A wording defect in the prose away from the
figures goes to proof through the node brief's `## Handed forward`.
