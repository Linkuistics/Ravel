# follow-a-session-through-the-code-k155

## Goal

Draft **part A** of chapter 8, as the document node's brief describes, under
`docs/book/contract.md`, into
`docs/book/src/08-follow-a-session-through-the-code.md`, linked from
`docs/book/src/SUMMARY.md`: the successful session from `main` to the result
write, with the `end-session` side of the channel. The headings for part B are
not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 3 gives the stages
  of a session in plain words; this chapter shows where each stage is in the
  code. Link back instead of re-teaching.
- Load the `linkuistics:writing-code-walkthroughs` skill first. Use it at
  orientation depth, as the node brief says.
- Run the chapter 3 example in a scratch directory with the checkout build and
  read the order from the run and the code together. A debugger, a trace or a
  temporary log in a scratch copy of the source is allowed, and nothing from
  that copy is committed.
- Settle the heading structure for the whole of chapter 8 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- `task docs:excerpts` and the example check exist before this leaf runs. If
  either is missing, stop: `book-source-excerpts-k150` is not retired.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and
  `task docs:excerpts`. Each source excerpt names its file under the
  convention.
- The full successful-session sequence is described in the prose and its figure
  need is in the log for the art stage (the participants, the messages and the
  order). Do not draw it here: art owns the figure.
- The log lists the obligations that part A teaches (some of O3, O4, O5 and
  O9) and which part of them remains for part B.
- Last act: `leaf-add` a `draft` leaf (bare stem
  `follow-a-session-through-the-code`) whose body states part B's scope from the
  node brief, the headings reserved for it and anything part A learned that
  changes it; then `leaf-add` the `copy-edit` leaf if no live later sibling
  already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  order of the steps and on the claim, if the chapter makes it, about which
  function does each step.
