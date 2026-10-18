# what-ravel-is-for-and-its-actors-k81

## Goal

Proof the whole opening unit, chapters 1 and 2, under `docs/book/contract.md`:
technical truth, teaching, prose, terminology, figures, text alternatives and
available viewing environments. This is the unit's first proof, at the end of
the correction run initiated by art k77.

## Context

Draft k78 corrects the policy worker's working-directory explanation, followed
by copy-edit k79 and art k80. Verify the distinction between private startup
directory, move to `/` before policy import, request's `cwd` and private-directory
cleanup. The figures' worker-cleanup-before-launch ordering remains required.

Read the document brief's current "Handed forward" entries: first-use term
links and the staged conversion of forward chapter references, plus actual
viewing-environment inspection. `docs/book/figures.md` records the shared
notation and the exact limits of prior inspection. No human comprehension
result or GitHub/published-site rendering is established by local checks.

The correction run already covers draft, copy-edit and art once. Apply the
editorial family's escalation rule if another rerun of one is needed.

## Done when

- The whole unit satisfies its contract and the document node's done-when
  clauses; any unavailable viewing environment is named with its evidence limit.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass.
- Clear resolved handoffs. If no live sibling remains, perform the ordinary
  node close check and promotion; proof adds no ordinary successor.

## Decisions (running log)

- Read both chapters as one document against the book contract, glossary,
  session/completion specifications and the two cited decisions. The routing,
  publication, cancellation, cleanup and same-user limits agree across prose,
  tables, figure sources and text alternatives. No chapter correction or
  editorial re-run is needed; no in-session reviewer is used.
- Verified the correction directly: `worker.rs::evaluate` starts the worker
  in its private directory and gates success on confirmed reap and checked
  removal; worker `main.ts` changes to `/` before module registration and
  policy import. The request's `cwd` remains distinct. Graph coverage reports
  no recorded gap and matching metadata for these code files (generation
  `2026-10-16T03:34:14Z`); the figure sources were read directly and the
  excluded SVGs inspected in the rendered local book.
- First-use links lead to the canonical definitions; a definition does not
  link to itself, following k76's settled convention. All glossary terms
  needed by this unit are taught, with launcher CLI/persistent runner in
  chapter 1 and the actor/result/information/confinement definitions in
  chapter 2. No new glossary term or ADR is needed.
- Chapters 3–6 are still mdBook draft entries, so their link conversion stays
  with the already queued first-half set proof and its parent brief. The
  later-half links and guide-fold-in distinctions already have parent owners.
- Native Chrome displayed both local chapters, including both figures and
  their notation, on a light page. Raw chapter and figure sources were read.
  Prior art's Navy/editor checks remain separately attributed. GitHub and
  published-site rendering remain unavailable before push/publication; no
  reader comprehension result follows from these inspections.

## Validation and node close

- `task docs:figures` passed: both figures match their editable sources.
  `task docs:book:build` passed without warnings. `task docs:examples` passed:
  zero examples, fragments and page errors, so this is a page scan rather
  than a behavioural execution. All 3496 tracked files were digested before
  and after this final sequential run; none changed during it.
- Both generated chapter pages' local links and heading anchors resolve;
  every figure target exists and has alt text. Known-heading controls pass
  and deliberately nonexistent anchors fail. An initial scratch assertion
  incorrectly expected one HTML image per page: mdBook emits an inline image
  and a popup copy. The corrected check inspects every image and compares
  unique targets; no chapter or renderer defect was found.
- K62's done-when holds: chapters 1–2 are in SUMMARY and have passed all four
  stages, definitions and glossary agree, both figures have notation teaching
  and text alternatives, and there are no unlabelled fragments. Its handoff
  list is cleared. Forward-link conversion already belongs to the parent
  brief's set proof; local inspection coverage and remaining viewing limits
  are retained in `docs/book/figures.md` and promoted to that parent brief.
- Retirement closes `what-ravel-is-for-and-its-actors-k62`. The book node
  `ravel-book-k55` still has live chapter work, so the cascade stops there.
  No live review names k62 or k81; no Creator line is required.
