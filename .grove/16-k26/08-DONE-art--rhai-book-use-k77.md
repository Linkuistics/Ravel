# rhai-book-use-k77

## Goal

Draw the two figures that the owner asked for in Read 1.
Make the other figures of chapters 1–6 agree with the redrafted text.

## Context

- F2, in the owner's words: “This could do with a diagram and concrete example.”
  The question: what does the policy return, and how does Ravel tell the caller which harness, model and effort the policy chose?
  The figure shows the selection, the launch of `program` with `args`, and the routing notice on the caller's standard error.
  Its labels use the real values of an example in the text.
- F5, in the owner's words: “inherited how? what is the mechanism. We should have a diagram to explain how this works.”
  The question: how does `ravel end-session`, which runs inside the harness, find the session that launched the harness?
  The figure shows how Ravel sets `RAVEL_EXIT_FILE` in the environment of the harness.
  Each child process inherits it, and `end-session` sends the result through that path. A nested session gives its own child a new path.
- “Figure needs” in `docs/book/contract.md` has a row for each figure.
  The “Handed forward” list of the node brief gives the place that the draft chose for each figure.
- `docs/book/figures.md` holds the conventions: one `.typ` source for each figure, the layout modules, the crisp style of `_draw.typ` and the layout records.
- The text beside a figure does not repeat its colour key. The owner decided this as finding R8 of 2026-10-17.
  The restatement of F7 concerns explanations, not keys.

## Done when

- Both figures are in their chapters, each with a caption, alt text and a link to the full-size image. Their labels agree with the text.
- Each other figure agrees with the redrafted text.
- `task docs:figures` reproduces every figure and layout record. `task docs:book:build` passes.
- Each figure loads in the local book at the 750-pixel column.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Proof `rhai-book-use-k78` already follows this leaf. Cut no next stage.
- This run is the first correction run against this document. Another send-back against it goes to the owner.

## Decisions (running log)

- The evaluation harness path needs its own sequence: it returns data to the policy before the final managed launch, a relation the selection figure does not carry.
- The routing sequence, nested-path tree and evaluation-harness sequence use the shared Typst modules. The latter answers a distinct question, so the figure-needs table now names it.
- Chapter 6 repeats the three figures where its direct entries need the relations. Chapter 5 reuses the tree and identifies its chapter 1 paths. Additional embedded routing copies in chapters 3 and 5 were declined as an editorial choice: those examples select different program names. The medium and contract permit them.
- `--json` needed escaped hyphens in Typst: the first render converted them to an en dash. The final export shows the literal option. This extends lesson L29.
- The leaf remains live: the whole-document art read of chapters 1–6 is still required before the handed-forward entries can be cleared and this leaf retired. Check the final figures and their placements in that read, and use the existing proof sibling rather than adding another.
- On 2026-10-28, `task docs:figures` matched 17 sources and layout records, `task docs:book:build` passed, and strict `task docs:ste` reported no problems in chapters 1–6. A headless Chrome decode check on the rebuilt local book loaded every figure at a 750-pixel column, with deliberately missing-image controls failing as expected. Safari in an isolated macOS guest showed the routing sequence and part of the process tree; the final three exports were inspected at 750 pixels. The guest was stopped. Details and limits are in `docs/book/figures.md`.
- Continuation: checked the new figures' placements and adjacent explanations, and inspected existing figure placements in chapters 1–6. The whole-document art read is not complete; resume it before clearing the handed-forward entries or retiring this leaf. The earlier figure and build checks remain evidence for the earlier subject state, not a substitute for that read.

- The art read covered all of chapters 1–6 and the sources of their twelve distinct figures. The existing tables carry the other sequences, comparisons and mappings; no further figure was needed. The three art entries in the node brief are closed. The shared installed-file view still awaits the delivery-removal reconciliation already assigned to Parts III and IV.
- Fresh checks after the read: `task docs:figures` matched all 17 sources and layout records; `task docs:book:build` passed; strict `task docs:ste` reported zero problems in chapters 1–6. The earlier local 750-pixel image-load and visual evidence remains recorded in `docs/book/figures.md`.
