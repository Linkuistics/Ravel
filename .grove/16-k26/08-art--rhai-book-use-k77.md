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
