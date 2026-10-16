# what-ravel-is-for-and-its-actors-k77

## Goal

Supply the opening unit's figures, under `docs/book/contract.md` (figure needs
in section 1, figure notations, renderers and delivery in section 3, the layout
in `docs/book/figures.md`), and fix the book's visual vocabulary for later
units: the C4 system-context figure for chapters 1–2 and the C4 container
figure for processes and authority in chapter 2. The node brief's "Handed
forward" entry for art states each figure's reader question and what it must
and must not show.

## Context

- The copy-edited text is complete without figures and has no placeholders.
  A figure is placed where its question arises and carries its own notation
  key; the text it sits in stays as copy-edit left it, apart from the sentence
  that introduces the figure.
- Names in the figures use the chapters' spelling exactly: caller, owner,
  owner policy, policy worker, native front, runner (inside the native front),
  managed process, opaque result, result channel, session-result context.
  Copy-edit settled "managed process" (not "managed job/program") and "native
  front" (not "the front"); "job" is only the cleanup scope, the process group.
- The container figure shows the policy response path (worker to native front)
  apart from the result path (managed process via `end-session` and the result
  channel to the native front, then standard output). The worker ends, is reaped
  and has its private directory removed before the managed launch. Chapter 2's
  "Trust and confinement" states the same-user and moved-out-of-group limits;
  boxes must not suggest authentication or containment beyond the group.
- No renderer has yet produced a committed figure: `task docs:figures` and
  `task docs:figures:update` are verified only against their controls'
  fixtures, so this is their first real use.

## Done when

- Both figures exist as source beside generated SVG under
  `docs/book/src/figures/`, `task docs:figures` reports no difference, and each
  is referenced from its chapter with text alternatives a reader without the
  image can use.
- The notation (C4 levels, element and relation shapes) is explained where it
  first appears, and the visual vocabulary is recorded for later units.
- `task docs:book:build` and `task docs:examples` pass.
- As the last work act, add the `proof` leaf under this node with the same
  bare slug, unless a live later sibling already holds that stage.

## Notes

Copy-edit sent nothing back to draft and added nothing to the node brief's
"Handed forward" list; the proof entry there (forward chapter links, first-use
term links) stands.

## Decisions (running log)

- Use two C4 views: system context for the caller/owner/Ravel/selected-program
  relations, and a container view for the native front, policy worker, managed
  process and temporary result channel. Keep the runner inside the native
  front's description, not as a separately running container. Label the
  worker-before-launch ordering explicitly; a static view does not show
  simultaneous lifetimes. Use the agreed pinned PlantUML/Smetana pipeline.
- Keep the existing prose and introduce figures beside the question they
  answer. Add compact comparisons for result versus exit status versus work,
  and for the three kinds of information; existing fit and responsibility
  tables remain. Decline a detailed lifecycle diagram here as an editorial
  judgement: chapter 3 owns its sequence and state teaching, and these figures
  need only the admission ordering. No medium limitation was encountered.
- The specification says the worker starts in a private directory and moves
  to `/` before policy import; chapter 2 currently says it runs in its private
  directory. This technical correction belongs to draft. Route one correction
  run through draft, copy-edit, art and proof after this art work, retaining
  the current stage's figure deliverables.
- Whole-unit figure reading also found the platform evidence comparison and
  nested-session publication mapping carried only in prose. Give each a small
  in-page table with a takeaway sentence. No additional diagram kind is needed.
- Raw SVG renders were visually inspected on a white canvas, and the shared
  legend colours were fixed explicitly for contrast. Browser inspection was
  attempted through computer use, but `iab` is unavailable and the browser
  inventory is empty; local mdBook theme rendering, editor preview, GitHub and
  published-site appearance are not established by that SVG inspection.

## Verification

- `task docs:figures`: both real figures regenerate byte-identically under the
  pinned PlantUML 1.2026.8/Smetana pipeline, with the shared style applied.
- `task docs:book:build`: passes. Local HTTP reads of both chapter pages carry
  the full image alt text; both served SVGs compare byte-identically with their
  source-tree assets. This establishes delivery, not browser appearance.
- `task docs:examples`: passes with 0 complete examples, 0 fragments and 0 page
  errors in this opening unit; no runnable example is required by its brief.
- Tier 2 graph metadata reported generation `2026-10-16T03:12:18Z`, matching
  recorded coverage, with no recorded gaps in the document/specification paths.
  PlantUML source paths had untracked freshness and were read directly. Figure
  content rests on the normative specification and ADRs, not a new runtime or
  whole-codebase claim. No product source or architectural decision changed.
