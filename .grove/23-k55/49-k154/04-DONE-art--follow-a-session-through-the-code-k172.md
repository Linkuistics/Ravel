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

## Decisions (running log)

- Read the whole chapter, its contract, STE decisions, figure conventions and
  brief chain. The existing figure plan supplies the design for this AFK stage.
  Use the existing sequence module for `session-code-path`, with the six
  participants and eleven steps in k155. Section dividers and explicit
  conditions mark the optional grace/escalation region. No renderer change.
- Keep the two cancellation sources unchanged. Their direct INT examples,
  attempted cleanup and lack of lifetime-end marks agree with chapters 5 and
  8. Their process-level abstraction does not need a second code-level copy.
  This is an editorial judgement, not a contract prohibition or medium limit.
- Expand the outcome figure's signal action to all five sources. Put the
  original-failure report under cancellation before cleanup/group reports,
  matching `settle`. State source priority rather than signal arrival order.
  Use “unchanged” for the success outcome.
- The combined worker-failure rules were carried only by prose. Convert that
  paragraph to a precedence table with a separate column for retained reports.
  `worker::evaluate` confirms its three rows. The other nine tables and the
  two reused cancellation sequences carry their relations without new diagrams.
- One aimed fresh-context reviewer compared both diagram sources and adjacent
  prose against the production paths. It found no concrete issue in event
  order, lifetime marks, signal priority or diagnostic preservation. No finding
  needs classification or another review. A later wording edit shortened the
  original-failure report label to prevent a lone word on a line.
- The new sequence uses `max-label: 180` and short labels after inspection.
  It retains all eleven steps without a module or pin change. Its SVG is
  299,179 bytes, below the 300 KB per-figure benchmark. The eleven-figure set
  is 2,240,789 bytes, above the first-half 2 MB benchmark. Carry this set-wide
  size question to the already-live whole-book art leaf k101, with its dense
  text work. No medium limit or new rendering contract is inferred.

## Verification

- Final checks passed on 2026-10-18: book build, all eleven figure/source
  comparisons, strict STE for chapter 8 and both changed layout records
  (zero problems), strict breaks for all eight delivered chapters (zero long
  runs), eight source excerpts and thirteen complete examples. Six labelled
  fragments were not run. These are fake-agent examples, not real-agent or
  interactive-terminal qualification.
- The preview server's final log exposed one earlier stale-output removal
  error while the standalone build shared `target/book`. A subsequent preview
  rebuild succeeded. After stopping that server, the standalone book build
  passed again without warnings or errors. No source change was needed.
- The SHA-256 over sorted tracked non-Grove paths and file bytes was unchanged
  before and after the checks:
  `18d7080f229d8c52c6a40073a75a4106739ce31e8c6ff1a9f8eb97c3eec71585`
  (3,476 tracked paths). Only Grove bookkeeping followed this measurement.
- Tier 2 graph verification used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T09:19:35Z`. Material production symbols were found, traced in
  both directions where relevant and read exactly. Coverage checks on the
  production paths, chapter/figure sources, layouts, contract, conventions,
  task context and three ADRs reported no recorded issue and matching metadata.
  This is best-effort coverage, not a completeness proof. Direct source reads
  settled causal order. SVGs are deliberately excluded from the graph and
  were checked by regeneration and browser inspection.
- Native Chrome, locally served mdBook, light theme, 1227 × 919 viewport:
  all four chapter images loaded at about 750 px width. Both edited/new SVGs
  were inspected at full size. No label/arrow/key overlap remained. The new
  worker-failure table was legible. A scratch page's absent-image control
  returned `complete=true`, `naturalWidth=0`, and failed the same predicate
  that passed its present-image control.
- `docs/book/figures.md` records the viewing evidence and limits. No editor,
  other browser/theme, narrower viewport, GitHub, published-site or human-read
  result is claimed. Chapter 5's source agrees with the unchanged shared
  cancellation figures; its served page was not re-inspected.
- Cleared both art handoffs from the node brief. The existing ADR set remains
  accurate: this pass adds illustrations of existing decisions, with no new
  architecture or renderer decision. No out-of-charter correction was found.
- The whole-chapter relation pass retained the existing tables for mappings,
  priorities and sets. No additional architecture or cancellation copy would
  add a relation beyond those views and the new full sequence. This is an
  editorial judgement, not a contract prohibition or a medium limitation.
- Cut the required proof successor k173 as the last work action. No later proof
  sibling existed. No review leaf names this producer, so no Creator receipt
  belongs on a review. The chapter node remains open for proof.
