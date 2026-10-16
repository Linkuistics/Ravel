# writing-an-owner-policy-k73

## Goal

Draft **part A** of chapter 4, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/04-writing-an-owner-policy.md`,
linked from `docs/book/src/SUMMARY.md`: the anatomy of a policy, its
typechecking, and validating every route and refusal with fake agents. The
chapter's headings for part B are not written here.

## Context

- Chapters 1–3 are proved before this runs; the chapter reuses chapter 3's
  example as its starting point and extends it.
- Settle the chapter's heading structure for the whole of chapter 4 now, so
  part B fills sections rather than reshaping the file, and write down in
  this leaf's log which headings part B owns.

## Done when

- Part A's sections exist, build and pass `task docs:examples`; each has a
  worked complete example or an honest fragment.
- Last act: `leaf-add` a `draft` leaf (bare stem `writing-an-owner-policy`)
  whose body states part B's scope from the node brief, the headings reserved
  for it and anything part A learnt that changes it; then `leaf-add` the
  `copy-edit` leaf if no live later sibling already holds it.
