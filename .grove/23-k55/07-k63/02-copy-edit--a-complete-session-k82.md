# a-complete-session-k82


## Goal

Copy-edit the whole of `docs/book/src/03-a-complete-session.md` under the
prose contract in `docs/book/contract.md`, preserving the chapter's technical
teaching and runnable example.



## Context

Read chapters 1–2 for established terms and the document brief's handoffs.
The draft introduces the first complete example, then explains admission,
evaluation and judgement, worker teardown before launch, process groups and
terminal lending, publication, reaping and final settlement. Its source and
validation record is in `a-complete-session-k72`.

The example uses the checker convention in `docs/book/examples.md`: exact
stdout and status, with the explicitly labelled stable stderr excerpt. The
chapter explains the additional process-dependent operation reports observed
on macOS. Keep that distinction. The installation fragment uses the current
checkout in a private prefix, with Java 11+ and tar's bzip2 support named
because the shared setup prepares the book's figure tools too.

The document brief assigns the sequence and outcome state-machine figures
to art; do not pre-empt that stage. Forward chapter mentions remain plain
text until their targets exist, under the book node's established convention.

## Done when

- The whole chapter satisfies the prose contract, with consistent terms and
  first-use links, and no change to the example's behaviour or the source
  sequence.
- `task docs:examples` and `task docs:book:build` pass; any correction outside
  this stage's charter is routed under the editorial family procedure.
- Cut the ordinary art successor with the document's bare stem, unless a live
  later sibling already holds that stage, then retire and commit this leaf.

## Notes
