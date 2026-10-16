# book-read-one-k70

## Goal

Hold read 1 of the book: the owner's labelled stand-in read of Parts I and II,
recorded with its evidence class, its corrections routed to the stages that own
them, and the second half's planning leaf cut under `ravel-book-k55`.

## Context

- Contract, "Reader checkpoints": `docs/book/contract.md`. The reader is the
  owner, reading as a labelled stand-in for the intended reader, and the result
  can establish accuracy, coverage and the owner's own results, not whether a
  developer new to Ravel can find, understand and use the book.
- **Tasks**: write a policy that routes to a fake agent; run a session; interpret
  a refusal and a cancellation; and arrive at one direct entry, such as an
  exit code or an error message, without reading what precedes it.
- **Entry surfaces**: begin at chapter 1 or at the stated entry, in the locally
  built book (`task docs:book`), after confirming the figures load there. The
  README's short entry (`book-readme-and-authority-k69`) is the repository's own
  door; start there if the owner wishes.
- **Viewing environments** to offer, in order: the locally built book; the raw
  Markdown files in an editor preview; GitHub's view, which is unchecked until
  the branch is pushed (this workstream is not on GitHub) and is named as
  unchecked, not omitted; the published site, which comes only with a release.
- Record, as the contract requires: the reader's own words, how far they got,
  the locations they report and what they did not reach. Report separately and
  never as one another: the owner's results, task performance, mechanical
  checks (`task docs:book`, `docs:examples`, `docs:figures`, `check`) and any
  model-proxy result. An early comprehension failure stops the read and routes
  its correction to the stage that owns it before dependent drafting continues.

## Done when

- The read is held with the owner and recorded in this leaf's log with its
  evidence class.
- Each correction is routed to the owning stage: a `leaf-add` of the stage's
  re-run (kind `draft`, `copy-edit`, `art` or `proof`) under the unit's
  document node, in pipeline order ending at `proof`, with the specific defect
  in its body; or recorded as a contract change with the owner. A second
  re-run of one stage against one document is an escalation.
- The second half is planned: a `planning` leaf for Parts III and IV is cut
  under `ravel-book-k55`, carrying chapters 7–9, 11 and 12 (chapter 10 is not
  this node's), the units' handed-forward items from the node brief, the
  examples policy's source-excerpt check (chapter 8), and read 2's tasks from
  the contract. If corrections are still open when the session ends, the
  planning leaf is cut after them in the walk.
- The owner's word on whether the migrated README and removed guides read
  correctly is recorded.

## Notes

- HITL. Stop and ask the owner for the reading; a model's proxy is not read 1.
